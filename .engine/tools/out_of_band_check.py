#!/usr/bin/env python3
"""UserPromptSubmit hook — surface OUT-OF-BAND writes to the working tree.

While `keel serve` is live, the human authors facts directly into the repo through the API — accepting/
rejecting a Decision in the review tab, editing or creating items, authoring edges. These land as
UNCOMMITTED changes in `.tracking/`/`.engine/` with no signal to the agent (2026-07-17: an accepted
D0126/D0127 got swept into a sprint commit by a blanket `git add -A`, unnoticed).

This prints a reminder at the START of each turn when those dirs have uncommitted changes, so the agent
`git diff`s before committing and stages DELIBERATELY instead of blanket-`git add -A`ing over the human's
attested writes. Silent when the tree is clean. Never fails the hook (always exits 0).
"""
import subprocess
import sys


def sh(args):
    try:
        return subprocess.run(args, capture_output=True, text=True, timeout=10).stdout
    except Exception:
        return ""


def main():
    status = sh(["git", "status", "--porcelain", "--", ".tracking", ".engine"]).strip()
    if not status:
        return  # clean — stay silent

    print("[out-of-band-writes] Uncommitted changes in .tracking/.engine at turn start - possibly authored")
    print("via keel serve (human accepts/edits/creates), NOT by me. Run `git diff` and stage DELIBERATELY;")
    print("do NOT blanket `git add -A` over human-attested writes (accepted Decisions, edits). Changed:")
    for line in status.splitlines()[:40]:
        print("  " + line)

    # Specifically surface Decision acceptance / status changes — these are the human's sign-off and must
    # be attributed to them, not folded silently into my commit.
    diff = sh(["git", "diff", "--", ".engine/decisions"])
    accepts = [
        l for l in diff.splitlines()
        if l.startswith("+") and (
            "DecisionStatus::accepted" in l or "DecisionStatus::rejected" in l
            or "Accept :" in l or "Reject :" in l or "judgedBy" in l
        )
    ]
    if accepts:
        print("DECISION ACCEPTANCE / STATUS CHANGES (human sign-off - verify + keep as THEIR attributed record):")
        for l in accepts[:20]:
            print("  " + l)


if __name__ == "__main__":
    try:
        main()
    except Exception:
        pass
    sys.exit(0)
