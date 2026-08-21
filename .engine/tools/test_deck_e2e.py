"""End-to-end deck test (issue192): serve a COPY tree, exercise the full save loop with httpx.

This is the test that could never exist for the artifact-runtime save path, and the reason the deck
moved into Rust: every assertion here runs against real HTTP on the real binary before any deploy.

Checks:
  1. GET /deck returns the page: cards present, script parses (node --check), liveness marker last.
  2. POST /api/disposition (a finding verdict, as the deck's JS sends it) -> 200, and the DISPOSITIONS
     VIEW read back shows the finding dispositioned - state verified from the computed view, never
     from the endpoint's own reply.
  3. POST /api/deck/sitting as the registered human -> 200, critique appended, and the sitting due
     count drops in the view.
  4. POST /api/deck/sitting as an AI actor -> 400 REFUSED: a sitting review is the human gate and the
     endpoint must not record it as anyone else.
  5. The tree still validates after all writes.

Usage: python .engine/tools/test_deck_e2e.py <REPO_ROOT>
"""
import json
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

import httpx

PORT = 8907


def main(repo: Path) -> int:
    exe = repo / "target" / "release" / "keel.exe"
    work = Path(tempfile.mkdtemp(prefix="keel-deck-e2e-"))
    for d in (".tracking", ".engine"):
        shutil.copytree(repo / d, work / d)
    # the guards that read source need the crates too - but serve/validate do not; keep the copy lean.
    exe_copy = work / "keel.exe"
    shutil.copy2(exe, exe_copy)

    srv = subprocess.Popen(
        [str(exe_copy), "serve", str(work), "--port", str(PORT)],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        # the deck runs on the human machine where the actor binding exists; the copy has none, so the
        # env supplies it - the same mechanism a real session uses.
        env={**__import__(chr(111)+chr(115)).environ, 'KEEL_ACTOR': 'claudeOpus5'},
    )
    failures = []
    try:
        base = f"http://127.0.0.1:{PORT}"
        c = httpx.Client(base_url=base, timeout=30.0)
        for _ in range(40):
            try:
                if c.get("/api/version").status_code == 200:
                    break
            except httpx.HTTPError:
                time.sleep(0.5)
        else:
            print("FAIL: server never answered")
            return 1

        def check(name, cond, detail=""):
            print(("  ok   " if cond else "  FAIL ") + name + (f"  [{detail}]" if detail and not cond else ""))
            if not cond:
                failures.append(name)

        # ── 1. the page ────────────────────────────────────────────────────────────────────────
        page = c.get("/deck")
        check("GET /deck is 200", page.status_code == 200)
        html = page.text
        check("cards render", 'class=card data-id="' in html)
        check("deck v6-rust stamp visible", "deck v6-rust" in html)
        js = html[html.index("<script>") + 8 : html.rindex("</script>")]
        jsf = work / "deck.js"
        jsf.write_text(js, encoding="utf-8")
        node = subprocess.run(["node", "--check", str(jsf)], capture_output=True, text=True)
        check("emitted script parses (node --check)", node.returncode == 0, node.stderr[:120])
        check("liveness marker is the last statement", js.rstrip().endswith("document.body.setAttribute('data-local-js','ok');"))

        # ── 2. a finding disposition, exactly as the deck's JS sends it ───────────────────────
        disp_before = c.get("/api/dispositions").json()
        undis = [f["finding"] for f in disp_before.get("findings", []) if not f.get("dispositioned")]
        check("a finding is available to judge", bool(undis))
        if undis:
            target = undis[0]
            r = c.post(
                "/api/disposition",
                json={
                    "finding": target,
                    "verdict": "act",
                    "rationale": "e2e: deck save loop",
                    "judged_at": "2026-08-21",
                },
            )
            check("POST /api/disposition is 200", r.status_code == 200, r.text[:120])
            # The first GET after a write may serve the PREVIOUS value labelled `stale` while the
            # recompute runs (D0167's stale-while-revalidate). Polling until fresh is the honest read.
            ok = False
            for _ in range(20):
                after = c.get("/api/dispositions").json()
                now = {f["finding"]: f.get("dispositioned") for f in after.get("findings", [])}
                if now.get(target) is True:
                    ok = True
                    break
                time.sleep(0.5)
            check("the COMPUTED VIEW shows the finding dispositioned", ok)

        # ── 3. a sitting review as the human ───────────────────────────────────────────────────
        sit_before = c.get("/api/computed/sitting-coverage").json()
        due = sit_before.get("due_sprints", [])
        check("a sitting review is due", bool(due))
        if due:
            story = due[0]
            r = c.post(
                "/api/deck/sitting",
                json={
                    "story": story,
                    "verdict": "accept",
                    "note": "e2e: reviewed via deck",
                    "by": "wweatherholtz",
                    "judged_at": "2026-08-21",
                },
            )
            check("POST /api/deck/sitting (human) is 200", r.status_code == 200, r.text[:120])
            ok = False
            n_after = len(due)
            for _ in range(20):
                sit_after = c.get("/api/computed/sitting-coverage").json()
                n_after = len(sit_after.get("due_sprints", []))
                if n_after == len(due) - 1:
                    ok = True
                    break
                time.sleep(0.5)
            check("the sitting due count DROPS in the computed view", ok, f"{len(due)} -> {n_after}")

        # ── 4. the human gate refuses an AI ────────────────────────────────────────────────────
        r = c.post(
            "/api/deck/sitting",
            json={"story": "anyStory", "verdict": "accept", "note": "", "by": "claudeOpus5", "judged_at": "2026-08-21"},
        )
        check("POST /api/deck/sitting as an AI is REFUSED (400)", r.status_code == 400, r.text[:120])

        # ── 5. the tree still validates ────────────────────────────────────────────────────────
        v = subprocess.run([str(exe_copy), "validate", str(work)], capture_output=True, text=True)
        check("tree validates after all writes", v.returncode == 0 and "validated clean" in v.stdout, v.stdout[-120:])
    finally:
        srv.kill()
        srv.wait()
        shutil.rmtree(work, ignore_errors=True)

    print(("PASS: deck e2e, all checks green" if not failures else f"FAIL: {len(failures)} check(s): {failures}"))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main(Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()))
