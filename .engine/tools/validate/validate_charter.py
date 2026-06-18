#!/usr/bin/env python3
"""Charter-edge guard (D0068/D0069; D0047 control).

Every delivery Story must declare its CHARTER — a #CharteredBy edge from the Story to the
originating Decision / Need / Requirement that chartered it (D0068). The charter lineage is what
the governing-process VERSION is computed from (pglViews); a Story with no charter has no resolvable
provenance.

FORWARD-ONLY: gates each NEWLY ADDED delivery file (git diff --cached --diff-filter=A). Editing an
existing (pre-charter-edge) sprint file never triggers it, so NO grandfather set is needed — old
sprints (pre-sprint38) are simply never re-checked. New sprints must charter their Story.

Pure-Python, kernel-free (D0048). Wired into pre-commit on added .tracking/delivery files.

Usage:
  python .engine/tools/validate/validate_charter.py            # check staged-added (exit 1 on violation)
  python .engine/tools/validate/validate_charter.py --selftest # positive+negative self-test
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
DELIVERY_PREFIX = ".tracking/delivery/"

_STORY = re.compile(r'^[ \t]*part\s+(\w+)\s*:\s*Story\b', re.MULTILINE)
_CHARTER = re.compile(r'#CharteredBy\s+dependency\s+from\s+(\w+)\s+to\s+(\w+)')


def stories(text: str) -> set[str]:
    return set(_STORY.findall(text))


def chartered(text: str) -> set[str]:
    return {m.group(1) for m in _CHARTER.finditer(text)}


def evaluate(added_delivery_texts: dict[str, str]) -> list[str]:
    """Pure core (so --selftest needs no git index).
      added_delivery_texts = {path: staged content} for NEWLY ADDED delivery files.
    Returns violation messages (empty = pass)."""
    violations: list[str] = []
    for path, text in sorted(added_delivery_texts.items()):
        uncharted = stories(text) - chartered(text)
        for s in sorted(uncharted):
            violations.append(
                f"{path}: Story '{s}' has no #CharteredBy edge — a delivery Story must charter "
                f"to its originating Decision/Need/Requirement (D0068): "
                f"`#CharteredBy dependency from {s} to <dNNNN>;`."
            )
    return violations


def _added_delivery_files() -> list[str]:
    out = subprocess.run(
        ["git", "diff", "--cached", "--name-only", "--diff-filter=A"],
        cwd=ROOT, capture_output=True, text=True,
    ).stdout
    return [
        ln.strip().replace("\\", "/")
        for ln in out.splitlines()
        if ln.strip().replace("\\", "/").startswith(DELIVERY_PREFIX) and ln.strip().endswith(".sysml")
    ]


def _staged_text(path: str) -> str:
    r = subprocess.run(["git", "show", f":{path}"], cwd=ROOT, capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else ""


def main() -> int:
    if "--selftest" in sys.argv:
        return selftest()

    added = _added_delivery_files()
    texts = {p: _staged_text(p) for p in added}
    violations = evaluate(texts)

    print("========================================================")
    print(f"  {len(added)} newly-added delivery file(s) staged")
    if violations:
        for v in violations:
            print(f"  ERROR {v}")
        print("FAIL — add the #CharteredBy edge to the sprint's Story (or SKIP_VALIDATE=1 to bypass).")
        return 1
    print("  0 violations (every newly-added delivery Story declares its charter)")
    print("PASS")
    return 0


def selftest() -> int:
    """positive: an added delivery file whose Story is chartered -> pass.
       negative: an added delivery file whose Story has no charter -> fail.
       neutral: no added delivery file -> pass."""
    good = ('package S {\n'
            '    part s42 : Story { :>> id = "x"; }\n'
            '    #CharteredBy dependency from s42 to d0070;\n'
            '}')
    bad = ('package S {\n'
           '    part s99 : Story { :>> id = "y"; }\n'
           '}')

    pos = evaluate({".tracking/delivery/sprintGood.sysml": good})
    neg = evaluate({".tracking/delivery/sprintBad.sysml": bad})
    neutral = evaluate({})

    checks = {
        "chartered-passes": pos == [],
        "uncharted-flagged": len(neg) == 1 and "s99" in neg[0],
        "no-added-passes": neutral == [],
    }
    ok = all(checks.values())
    print("selftest: " + "; ".join(f"{k}={v}" for k, v in checks.items()))
    print("PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
