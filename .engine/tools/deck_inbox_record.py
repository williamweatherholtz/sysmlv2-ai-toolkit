"""Record deck-inbox rows into keel - the RECEIVE side of the mobile pipeline (issue194).

The pipeline: a phone tap on the deck delivers one ROW to the keel-deck-inbox sheet through the
viewer's Smartsheet connector (the page shows the store's row id as the receipt). Only the AI
session holds connector access, so the session PULLS unprocessed rows, writes them to a JSON file,
and runs this tool. Each row is recorded through the keel write API and then VERIFIED from the
COMPUTED VIEW - never from the endpoint's own reply. The tool prints per-row outcomes; rows that
verified are safe to mark Processed in the sheet (the session does that, again via the connector).

Dedup: latest At per (uid, kind) wins. Duplicate deliveries are EXPECTED - the page invites a
re-tap on an ambiguous outcome - and harmless by construction.

Verdict routing (same routes the deck's own JS uses):
  finding    verdict act|dismiss -> POST /api/disposition; maybe -> reported for triage, not recorded
  acceptance verdict accept|reject -> POST /api/decision/accept|/reject (file resolved from --root);
             maybe -> reported for triage
  sitting    any verdict -> POST /api/deck/sitting (the server refuses a non-Person `by`)

Usage:
  python deck_inbox_record.py rows.json --base http://127.0.0.1:7777 --root <repo>
  python deck_inbox_record.py rows.json --copy-test <repo>     # rehearse on a served COPY first

rows.json: [{"rowId": 123, "uid": "...", "name": "...", "kind": "finding|acceptance|sitting",
             "verdict": "...", "note": "...", "by": "...", "at": "ISO"}]
"""
import json
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

import httpx

COPY_PORT = 8909


def decision_file(root: Path, name: str) -> str | None:
    """Resolve a decision name to its repo-relative file - the accept/reject API requires it."""
    for f in sorted((root / ".engine" / "decisions").glob("*.sysml")):
        if f"part {name} : Decision" in f.read_text(encoding="utf-8", errors="replace"):
            return f.relative_to(root).as_posix()
    return None


def dedup(rows: list[dict]) -> list[dict]:
    latest: dict[tuple, dict] = {}
    for r in rows:
        key = (r.get("uid"), r.get("kind"))
        if key not in latest or str(r.get("at", "")) >= str(latest[key].get("at", "")):
            latest[key] = r
    return list(latest.values())


def poll(read, want, tries: int = 60) -> bool:
    """Poll a computed-view read until `want(value)` - D0167 reads after writes may serve stale.

    60 tries (~30s): a 10s window expired mid-batch on the first real 16-write pull and reported a
    correctly-recorded fact UNVERIFIED. A false UNVERIFIED costs a manual investigation; the label
    stays honest either way, so the window errs long."""
    for _ in range(tries):
        try:
            if want(read()):
                return True
        except httpx.HTTPError:
            pass
        time.sleep(0.5)
    return False


def record(rows: list[dict], base: str, root: Path) -> list[dict]:
    c = httpx.Client(base_url=base, timeout=30.0)
    today = time.strftime("%Y-%m-%d")
    out = []
    for r in dedup(rows):
        rid, name, kind = r.get("rowId"), r.get("name", ""), r.get("kind", "")
        verdict, note, by = r.get("verdict", ""), r.get("note", ""), r.get("by", "")
        res = {"rowId": rid, "name": name, "kind": kind, "verdict": verdict}
        # The attestation text carries the SUBSTANCE of the event, not a bare row pointer: who tapped
        # what, delivered as which row, when, against which deck. Their note, when present, is quoted
        # verbatim ahead of it (issue197's sibling defect: "via deck inbox row N" attested nothing).
        event = (
            f"{by} tapped {verdict} on the {name} card in the obligations deck"
            f" (deck HEAD {r.get('head', '?')}), delivered through the connector inbox as row {rid}"
            f" at {r.get('at', '?')}"
        )
        attest = f"Their note, verbatim: '{note}'. {event}." if note else f"{event} with no per-item note."

        if kind == "finding":
            if verdict == "maybe":
                res["outcome"] = "triage"
                res["detail"] = f"needs-work note for the AI, nothing recorded: {note}"
            else:
                v = "act" if verdict == "accept" else "dismiss"
                # judged_by is the row's By — the verdict is THEIRS. Omitting it lets the server
                # attribute their tap to the session actor (issue197: a High disposition landed as
                # judgedBy=claudeOpus5, legal under D0165's delegation and therefore invisible to
                # every guard, but false).
                p = c.post("/api/disposition", json={
                    "finding": name, "verdict": v,
                    "rationale": attest, "judged_at": today, "judged_by": by})
                if p.status_code != 200:
                    res["outcome"] = "REFUSED"
                    res["detail"] = p.text[:200]
                else:
                    ok = poll(
                        lambda: c.get("/api/dispositions").json(),
                        lambda d: any(f.get("finding") == name and f.get("dispositioned")
                                      for f in d.get("findings", [])))
                    res["outcome"] = "verified" if ok else "UNVERIFIED"
        elif kind == "acceptance":
            if verdict == "maybe":
                res["outcome"] = "triage"
                res["detail"] = f"needs-work note for the AI, nothing recorded: {note}"
            else:
                f = decision_file(root, name)
                if not f:
                    res["outcome"] = "REFUSED"
                    res["detail"] = "decision file not found under .engine/decisions"
                else:
                    url = "/api/decision/accept" if verdict == "accept" else "/api/decision/reject"
                    body = {"decision": name, "file": f, "judged_at": today, "judged_by": by}
                    body["note" if verdict == "accept" else "rationale"] = attest
                    p = c.post(url, json=body)
                    if p.status_code != 200:
                        res["outcome"] = "REFUSED"
                        res["detail"] = p.text[:200]
                    else:
                        ok = poll(
                            lambda: c.get("/api/orient").json(),
                            lambda d: name not in d.get("pendingAcceptances", []))
                        res["outcome"] = "verified" if ok else "UNVERIFIED"
        elif kind == "sitting":
            p = c.post("/api/deck/sitting", json={
                "story": name, "verdict": verdict, "note": attest, "by": by, "judged_at": today})
            if p.status_code != 200:
                res["outcome"] = "REFUSED"
                res["detail"] = p.text[:200]
            else:
                ok = poll(
                    lambda: c.get("/api/computed/sitting-coverage").json(),
                    lambda d: name not in d.get("due_sprints", []))
                res["outcome"] = "verified" if ok else "UNVERIFIED"
        else:
            res["outcome"] = "REFUSED"
            res["detail"] = f"unknown kind {kind!r}"
        out.append(res)
    return out


def copy_test(rows: list[dict], repo: Path) -> int:
    """Rehearse the whole receive side against a served COPY - the real tree is never touched."""
    exe = repo / "target" / "release" / "keel.exe"
    work = Path(tempfile.mkdtemp(prefix="keel-inbox-rehearsal-"))
    for d in (".tracking", ".engine"):
        shutil.copytree(repo / d, work / d)
    exe_copy = work / "keel.exe"
    shutil.copy2(exe, exe_copy)
    import os
    srv = subprocess.Popen(
        [str(exe_copy), "serve", str(work), "--port", str(COPY_PORT)],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        env={**os.environ, "KEEL_ACTOR": "claudeOpus5"})
    try:
        c = httpx.Client(base_url=f"http://127.0.0.1:{COPY_PORT}", timeout=30.0)
        for _ in range(40):
            try:
                if c.get("/api/version").status_code == 200:
                    break
            except httpx.HTTPError:
                time.sleep(0.5)
        else:
            print("FAIL: copy server never answered")
            return 1
        results = record(rows, f"http://127.0.0.1:{COPY_PORT}", work)
        v = subprocess.run([str(exe_copy), "validate", str(work)],
                           capture_output=True, text=True, check=False)
        print(json.dumps({"rehearsal": results, "validate_clean": v.returncode == 0}, indent=2))
        bad = [r for r in results if r["outcome"] in ("REFUSED", "UNVERIFIED")]
        return 1 if (bad or v.returncode != 0) else 0
    finally:
        srv.kill()
        srv.wait(timeout=10)
        shutil.rmtree(work, ignore_errors=True)


def main() -> int:
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        return 2
    rows = json.loads(Path(args[0]).read_text(encoding="utf-8"))
    if "--copy-test" in args:
        return copy_test(rows, Path(args[args.index("--copy-test") + 1]).resolve())
    base = args[args.index("--base") + 1] if "--base" in args else "http://127.0.0.1:7777"
    root = Path(args[args.index("--root") + 1]).resolve() if "--root" in args else Path.cwd()
    results = record(rows, base, root)
    print(json.dumps(results, indent=2))
    return 1 if any(r["outcome"] in ("REFUSED", "UNVERIFIED") for r in results) else 0


if __name__ == "__main__":
    sys.exit(main())
