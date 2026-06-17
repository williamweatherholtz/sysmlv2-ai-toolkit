#!/usr/bin/env python3
"""Acceptance-event guard (D0066/D0047, attrModelGuard): a status=accepted Decision's
acceptance must be a confirmation EVENT (a passing `dNNNNAcceptR1` TestResult verifying the
Decision), never prose/status alone. Flags any accepted Decision lacking the event.

Pure-Python, kernel-free (D0048). Wired into pre-commit on .engine/decisions changes.
All 66 accepted decisions were migrated to events (D0066), so no grandfathering is needed —
the rule applies uniformly. (This is the enforcement twin of `query.py attestation-coverage`.)

  python .engine/tools/validate/validate_acceptance_events.py            # check (exit 1 on violation)
  python .engine/tools/validate/validate_acceptance_events.py --selftest # positive+negative self-test
"""
from __future__ import annotations

import glob
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
DECISIONS = os.path.join(ROOT, ".engine", "decisions")

_PART = re.compile(r'\bpart\s+(d\w+)\s*:\s*Decision\b')


def missing_acceptance(text: str) -> str | None:
    """Return the decision name if it is accepted but lacks a passing acceptance event, else None."""
    m = _PART.search(text)
    if not m or "DecisionStatus::accepted" not in text:
        return None
    dname = m.group(1)
    if re.search(rf'\b{re.escape(dname)}AcceptR1\b[^}}]*VerdictKind::pass', text):
        return None
    return dname


def main() -> int:
    if "--selftest" in sys.argv:
        return selftest()

    violations = []
    total = 0
    for f in sorted(glob.glob(os.path.join(DECISIONS, "*.sysml"))):
        with open(f, encoding="utf-8") as fh:
            text = fh.read()
        if "DecisionStatus::accepted" in text:
            total += 1
        miss = missing_acceptance(text)
        if miss:
            violations.append(miss)

    print("========================================================")
    print(f"  {total} accepted decisions scanned")
    if violations:
        for d in sorted(violations):
            print(f"  ERROR {d}: accepted but no passing acceptance event "
                  "(D0066 — acceptance must be a confirmation TestResult, not prose/status; "
                  "add `verification {d}Accept : Test{{method=confirmation}}` + `part {d}AcceptR1 : "
                  "TestResult{{outcome=pass; judgedBy=<human>}}`; SKIP_VALIDATE=1 to bypass)".format(d=d))
        print(f"  {len(violations)} violation(s)")
        print("FAIL — every accepted Decision needs an acceptance event.")
        return 1
    print("  0 violations (every accepted Decision has a passing acceptance event)")
    print("PASS")
    return 0


def selftest() -> int:
    """Positive: an accepted decision WITH a passing event passes. Negative: one WITHOUT is flagged."""
    ok_doc = (
        'part d9001 : Decision { :>> status = DecisionStatus::accepted; }\n'
        'part d9001AcceptR1 : TestResult { :>> outcome = VerdictKind::pass; }\n'
    )
    bad_doc = 'part d9002 : Decision { :>> status = DecisionStatus::accepted; }\n'
    pos = missing_acceptance(ok_doc) is None
    neg = missing_acceptance(bad_doc) == "d9002"
    ok = pos and neg
    print(f"selftest: accepted+event passes ({pos}) & accepted-no-event flagged ({neg}) = "
          f"{'PASS' if ok else 'FAIL'}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
