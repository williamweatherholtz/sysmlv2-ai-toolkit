#!/usr/bin/env python3
"""Process-change keystone guard (D0070 hard lock; D0047 control).

A staged change to a PROCESS-DEF file (.engine/processes/*.sysml or .engine/workflows/*.sysml)
MUST be co-committed with a Decision (.engine/decisions/*.sysml) bearing a #ProspectiveChange or
#SafetyChange PREFIX marker — or the commit fails. Typos included; NO intent interpretation
(D0070, user-accepted 2026-06-18). Making the recording unavoidable is exactly what makes the
governing process version reliably git-COMPUTABLE (D0069/D0070, pglViews) — so no per-item
process-version stamp is ever needed.

FORWARD-ONLY: this enforces on THIS commit's staged changes. It never retroactively audits the
history of existing process-defs, so NO grandfather set is needed — a pre-guard process-def is
simply never flagged unless it is changed again (at which point the hard lock applies, correctly).

Pure-Python, kernel-free (D0048). Wired into pre-commit on .engine/processes|workflows changes.

Usage:
  python .engine/tools/validate/validate_process_change.py            # check staged (exit 1 on violation)
  python .engine/tools/validate/validate_process_change.py --selftest # positive+negative self-test
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]

PROCESS_DEF_DIRS = (".engine/processes/", ".engine/workflows/")
DECISION_DIR = ".engine/decisions/"

# A process-change marker = a #ProspectiveChange / #SafetyChange PREFIX on a part (the Decision).
# Line-anchored (MULTILINE) so a prose EXAMPLE inside a string literal does not count as a marker.
# (The rust parser rejects the `{ @Marker; }` member form, so the prefix form is the only one
#  authored — see .engine/docs/sysmlv2-syntax-notes.md.)
_MARKER = re.compile(r'^[ \t]*#(ProspectiveChange|SafetyChange)\b', re.MULTILINE)


def is_process_def(path: str) -> bool:
    return path.endswith(".sysml") and any(path.startswith(d) for d in PROCESS_DEF_DIRS)


def is_decision(path: str) -> bool:
    return path.endswith(".sysml") and path.startswith(DECISION_DIR)


def has_marker(text: str) -> bool:
    return bool(_MARKER.search(text))


def evaluate(changed: list[str], decision_texts: dict[str, str]) -> list[str]:
    """Pure core (so --selftest needs no git index).
      changed         = staged file paths, repo-relative, forward slashes.
      decision_texts  = {path: staged content} for the staged decision files.
    Returns violation messages (empty list = pass)."""
    procdefs = sorted(p for p in changed if is_process_def(p))
    if not procdefs:
        return []  # no process-def changed — guard is silent
    marked = sorted(p for p, t in decision_texts.items() if is_decision(p) and has_marker(t))
    if marked:
        return []  # a co-committed process-change Decision is present
    return [
        "process-def file(s) changed (%s) with NO co-committed process-change Decision "
        "(a #ProspectiveChange/#SafetyChange-marked .engine/decisions/*.sysml). D0070 hard lock: "
        "every process-def change — typos included — must record a process-change Decision."
        % ", ".join(procdefs)
    ]


def _staged_files() -> list[str]:
    out = subprocess.run(
        ["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"],
        cwd=ROOT, capture_output=True, text=True,
    ).stdout
    return [ln.strip().replace("\\", "/") for ln in out.splitlines() if ln.strip()]


def _staged_text(path: str) -> str:
    r = subprocess.run(["git", "show", f":{path}"], cwd=ROOT, capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else ""


def main() -> int:
    if "--selftest" in sys.argv:
        return selftest()

    changed = _staged_files()
    decision_texts = {p: _staged_text(p) for p in changed if is_decision(p)}
    violations = evaluate(changed, decision_texts)

    print("========================================================")
    procdefs = [p for p in changed if is_process_def(p)]
    print(f"  {len(procdefs)} process-def file(s) staged; "
          f"{sum(1 for t in decision_texts.values() if has_marker(t))} marked Decision(s) co-committed")
    if violations:
        for v in violations:
            print(f"  ERROR {v}")
        print("FAIL — add a #ProspectiveChange/#SafetyChange-marked Decision to this commit "
              "(or SKIP_VALIDATE=1 to bypass).")
        return 1
    print("  0 violations (process-def changes carry a co-committed process-change Decision)")
    print("PASS")
    return 0


def selftest() -> int:
    """positive: process-def + a marked Decision co-committed -> pass.
       negative: process-def + only an UNmarked Decision -> fail.
       negative2: process-def + NO Decision -> fail.
       neutral: no process-def changed -> pass (guard silent).
       anchor: a marker only INSIDE a string literal (prose) does NOT count."""
    marked = 'package D {\n    #ProspectiveChange part d99 : Decision { :>> id = "x"; }\n}'
    plain = 'package D {\n    part d98 : Decision { :>> id = "y"; }\n}'
    prose = ('package D {\n    part d97 : Decision {\n'
             '        :>> decision = "example: #ProspectiveChange part dNNNN : Decision { ... }";\n    }\n}')

    pos = evaluate([".engine/workflows/delivery.sysml", ".engine/decisions/0099-x.sysml"],
                   {".engine/decisions/0099-x.sysml": marked})
    neg = evaluate([".engine/processes/agile-workflow.sysml", ".engine/decisions/0098-y.sysml"],
                   {".engine/decisions/0098-y.sysml": plain})
    neg2 = evaluate([".engine/processes/agile-workflow.sysml"], {})
    neutral = evaluate([".tracking/backlog.sysml", ".engine/tools/query.py"], {})
    prose_only = evaluate([".engine/workflows/delivery.sysml", ".engine/decisions/0097-z.sysml"],
                          {".engine/decisions/0097-z.sysml": prose})

    checks = {
        "pass-with-marked": pos == [],
        "fail-unmarked": len(neg) == 1,
        "fail-no-decision": len(neg2) == 1,
        "neutral-no-procdef": neutral == [],
        "prose-marker-does-not-count": len(prose_only) == 1,
    }
    ok = all(checks.values())
    print("selftest: " + "; ".join(f"{k}={v}" for k, v in checks.items()))
    print("PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
