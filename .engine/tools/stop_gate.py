#!/usr/bin/env python3
"""Stop hook - in-loop TIER-3 governance gate (PROTOTYPE, keel).

On turn-end, run `keel validate` + `keel guard`. If the model is RED (parse errors or guard violations),
BLOCK the stop and feed the violations back, so the agent fixes them before handing off instead of ending
dirty. This moves the honest-state gate from commit-time to the TURN BOUNDARY (in-loop governance).

Loop-safe: if we already blocked once this stop-cycle (`stop_hook_active`) and it is STILL red, allow the
stop with a loud warning instead of trapping the agent (e.g. pre-existing / genuinely-unfixable red).

Prototype notes: emits JSON on stdout; never crashes a turn (any error -> allow stop); NO-OPS outside a
built keel repo (missing binary or .tracking). Adds ~1-2s to each turn-end (validate + guard).
"""
import json
import os
import subprocess
import sys


def emit(obj):
    print(json.dumps(obj))
    sys.exit(0)


def run(args):
    # encoding is EXPLICIT: keel emits UTF-8 (em-dashes, arrows). Without this, Windows decodes
    # as cp1252 and the corrective feedback reaches the agent as mojibake — the gate's whole value
    # is the legibility of what it feeds back.
    try:
        p = subprocess.run(args, capture_output=True, text=True, timeout=120,
                           encoding="utf-8", errors="replace")
        return p.returncode, (p.stdout or "") + (p.stderr or "")
    except Exception:
        return 0, ""  # cannot run -> treat as non-blocking


def find_keel():
    """Locate the engine binary across platforms, or None.

    A Windows-only `keel.exe` path would make this gate silently NO-OP on every other machine —
    the same defect as issue076, reproduced inside the governance control itself. A gate that
    quietly does nothing is worse than no gate, because the discipline is believed to be enforced.
    """
    for cand in (os.path.join("target", "release", "keel.exe"),
                 os.path.join("target", "release", "keel"),
                 os.path.join("target", "debug", "keel.exe"),
                 os.path.join("target", "debug", "keel")):
        if os.path.exists(cand):
            return cand
    from shutil import which
    return which("keel")


def tail(s, n=1000):
    s = s.strip()
    return s if len(s) <= n else "..." + s[-n:]


def guard_fails(o):
    lines = [l for l in o.splitlines() if ("FAIL" in l or "ERROR" in l) and "ALL PASS" not in l]
    return "\n".join(lines)[:1500]


def main():
    try:
        data = json.load(sys.stdin)
    except Exception:
        data = {}
    already = bool(data.get("stop_hook_active"))

    if not os.path.isdir(".tracking"):
        return  # not a keel project at all -> silent no-op, correctly

    keel = find_keel()
    if not keel:
        # This IS a keel project but the gate cannot run. Say so VISIBLY rather than no-opping:
        # a silently-absent gate is the issue076 failure. Never block on it — an unbuilt binary is
        # not a model-honesty problem, and blocking the turn would be hostile and unfixable in-loop.
        emit({"systemMessage": "[in-loop gate] SKIPPED - no keel binary found (looked in target/release, "
                               "target/debug, PATH). Run `cargo build --release` to re-enable in-loop "
                               "honest-state gating. Commit-time guards still apply."})

    problems = []
    rc, o = run([keel, "validate", "."])
    if rc != 0 or "ERROR:" in o:
        problems.append("keel validate:\n" + tail(o))
    rc, o = run([keel, "guard"])
    if rc != 0:
        problems.append("keel guard:\n" + (guard_fails(o) or tail(o)))

    if not problems:
        return  # green -> allow stop

    if already:
        emit({"systemMessage": "[in-loop gate] Still red after a correction pass - allowing stop to avoid a loop. Do NOT commit until keel validate + guard are green."})
    else:
        reason = (
            "[in-loop gate] The model is not in honest state - resolve before ending the turn:\n\n"
            + "\n\n".join(problems)
            + "\n\nFix through the keel write API (append-result / add-task / record decision); "
            "run `keel guard <name>` for detail. Then end the turn."
        )
        emit({"decision": "block", "reason": reason})


if __name__ == "__main__":
    try:
        main()
    except Exception:
        pass
    sys.exit(0)
