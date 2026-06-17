#!/usr/bin/env python3
"""No-sprint guard (D0047 control for issue020): substantive work must be wrapped
in a sprint.

A backlog `action <task>;` with a passing `<task>DoDR<n>` result is "done". Under the
sprint discipline (D0064), done work must be COVERED by a sprint — i.e. the task name
appears in some `.tracking/delivery/*.sysml` file. A done task that is neither covered
nor GRANDFATHERED (pre-D0064 historical work) is flagged: it was committed without a
sprint, the exact lapse issue020 recorded (the attribution migration before sprint28).

Pure-Python, kernel-free (D0048). Wired into pre-commit on .tracking changes.

Usage:
  python .engine/tools/validate/validate_sprint_coverage.py            # check (exit 1 on violation)
  python .engine/tools/validate/validate_sprint_coverage.py --selftest # positive+negative self-test
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
BACKLOG = ROOT / ".tracking" / "backlog.sysml"
DELIVERY = ROOT / ".tracking" / "delivery"

# Done tasks whose work predates the sprint discipline (D0064, 2026-06-17). These were
# tracked as raw backlog items during bootstrap, before engine work was sprinted — accepted
# as historical (mirrors validate_ceremony.GRANDFATHERED / validate_actors.LEGACY_ACTORS).
# Nothing may be ADDED here going forward: new done work must be covered by a sprint.
GRANDFATHERED: set[str] = {
    "ceremonyGateGuard",    # issue010 guard, built pre-D0064; not name-matched in a delivery file
    "rustS8runtimeParser",  # rust_sprint8 work; delivery story named differently
    "rustS9writeApi",       # rust_sprint9 work; delivery story named differently
    "trackedMetadataReplan",# planning task, subsumed by D0061; pre-discipline
}

# A passing DoD result: `part <task>DoDR<n> : TestResult { ... outcome = VerdictKind::pass ... }`
_DONE = re.compile(r'part\s+(\w+?)DoDR\d+\s*:\s*TestResult\s*\{[^}]*VerdictKind::pass')


def done_tasks(backlog_text: str) -> set[str]:
    return set(_DONE.findall(backlog_text))


def covered_by_sprint(task: str, delivery_blob: str) -> bool:
    """A task is covered if its name appears anywhere in the delivery (sprint) files."""
    return task in delivery_blob


def find_uncovered(backlog_text: str, delivery_blob: str, grandfathered: set[str]) -> list[str]:
    return sorted(
        t for t in done_tasks(backlog_text)
        if not covered_by_sprint(t, delivery_blob) and t not in grandfathered
    )


def _delivery_blob() -> str:
    return "\n".join(p.read_text(encoding="utf-8") for p in DELIVERY.glob("*.sysml"))


def main() -> int:
    if "--selftest" in sys.argv:
        return selftest()

    backlog_text = BACKLOG.read_text(encoding="utf-8")
    blob = _delivery_blob()
    uncovered = find_uncovered(backlog_text, blob, GRANDFATHERED)

    print("========================================================")
    print(f"  {len(done_tasks(backlog_text))} done backlog tasks; {len(GRANDFATHERED)} grandfathered (pre-D0064)")
    if uncovered:
        for t in uncovered:
            print(f"  ERROR {t}: done but not covered by any sprint (D0064/issue020 — "
                  "substantive work must be wrapped in a sprint; SKIP_VALIDATE=1 to bypass)")
        print(f"  {len(uncovered)} violation(s)")
        print("FAIL — wrap the work in a sprint delivery file (or grandfather if genuinely pre-D0064).")
        return 1
    print("  0 violations (every done task is covered by a sprint or grandfathered)")
    print("PASS")
    return 0


def selftest() -> int:
    """Positive: a done+covered task passes. Negative: a done+uncovered task is flagged."""
    backlog = (
        'action fakeCovered;\n'
        'part fakeCoveredDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }\n'
        'action fakeOrphan;\n'
        'part fakeOrphanDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }\n'
    )
    blob = "package ProjectDeliveryX { part s : Story { :>> title = \"delivers fakeCovered\"; } }"
    uncovered = find_uncovered(backlog, blob, grandfathered=set())
    ok = uncovered == ["fakeOrphan"]
    print(f"selftest: covered-passes + orphan-flagged = {'PASS' if ok else 'FAIL'} (flagged={uncovered})")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
