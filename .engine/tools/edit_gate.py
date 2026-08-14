#!/usr/bin/env python3
"""PostToolUse hook - in-loop TIER-2 governance gate (D0128 / nAiGovernanceInLoop).

After an Edit/Write to a `.sysml` file, run `keel gate --fast` and, if the model is broken, feed the
violations straight back so the agent fixes them at the point of the edit rather than discovering it
at turn end (Tier-3) or commit.

WHY TIER-2 EXISTS, on measured evidence rather than theory. Three of this sitting's recorded avoidable
issues were the same shape - an edit that was WRONG and only discovered much later:
  * a malformed anchored edit inserted a placeholder attribute into a Decision (sprint 251 retro)
  * a doc-sync edit silently did nothing because its anchor did not match (sprint 254 retro)
  * a marker typo would validate clean and blind a hard guard (issue077, proven by negative test)
Each is caught in ~0.35s at the moment of the edit.

WHAT IT DELIBERATELY DOES NOT RUN. Only checks that are FAST and EXACT: validate,
duplicate-identity, marker-vocabulary. Measured warm: ~0.35s, versus 1.9s for the full guard suite,
which therefore stays at the turn boundary and commit. Every heuristic/warning-level guard is
excluded on purpose - a per-edit gate that fires on a prose heuristic would block work mid-thought
and train the agent to disable it, which is the issue076/issue081 dynamic that cost eight bypassed
commits before it was fixed.

Cross-platform binary probing, because the Tier-3 prototype shipped Windows-only and would have
silently no-opped for every other contributor (the same issue076 class inside a governance control).
Never crashes a turn: any error allows the edit.
"""
import json
import os
import subprocess
import sys
from shutil import which


def emit(obj):
    print(json.dumps(obj))
    sys.exit(0)


def find_keel():
    for cand in (
        os.path.join("target", "release", "keel.exe"),
        os.path.join("target", "release", "keel"),
        os.path.join("target", "debug", "keel.exe"),
        os.path.join("target", "debug", "keel"),
    ):
        if os.path.exists(cand):
            return cand
    return which("keel")


def main():
    try:
        data = json.load(sys.stdin)
    except Exception:
        return  # unparseable payload -> allow

    ti = data.get("tool_input") or {}
    path = ti.get("file_path") or (data.get("tool_response") or {}).get("filePath") or ""
    if not str(path).endswith(".sysml"):
        return  # only model files are gated

    if not os.path.isdir(".tracking"):
        return  # not a keel project

    keel = find_keel()
    if not keel:
        # Say so VISIBLY rather than no-opping: a silently-absent gate is worse than no gate.
        emit({"systemMessage": "[edit gate] SKIPPED - no keel binary found. `cargo build --release` "
                              "to re-enable per-edit gating. Turn-end + commit gates still apply."})

    try:
        p = subprocess.run([keel, "gate", "--fast"], capture_output=True, text=True,
                           timeout=120, encoding="utf-8", errors="replace")
    except Exception:
        return  # cannot run -> allow

    if p.returncode == 0:
        return  # clean -> silent, so a passing gate costs the agent nothing

    out = ((p.stdout or "") + (p.stderr or "")).strip()
    if len(out) > 2000:
        out = out[:2000] + "\n... (truncated)"
    emit({
        "decision": "block",
        "reason": "[edit gate] That edit left the model broken - fix it now, at the point of the edit:\n\n"
                  + out
                  + "\n\nThis is the FAST tier (validate + duplicate-identity + marker-vocabulary, all exact). "
                    "Author through the keel write API where one exists.",
    })


if __name__ == "__main__":
    try:
        main()
    except Exception:
        pass
    sys.exit(0)
