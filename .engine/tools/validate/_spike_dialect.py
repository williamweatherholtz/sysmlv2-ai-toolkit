"""Spike (CR-3): the backlog dialect v2 — verification usages (criteria) and
TestResult part instances (appended results) INSIDE an action def, one-line,
with enum values. This is what query.py will read."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

SCHEMA = ["schema/core/element.sysml", "schema/core/verification.sysml"]
ENGINE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

SRC = """
package DialectSpike {
    private import EngineElement::*;
    private import EngineVerification::*;

    action def Backlog {
        action taskA;
        action taskB;
        first taskA then taskB;

        verification taskADoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "user signs off"; }
        verification taskBDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "validator green"; }

        part taskAR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abc1234"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        part taskAR2 : TestResult { :>> outcome = VerdictKind::fail; :>> judgedAgainst = "def5678"; :>> judgedAt = "2026-06-12"; :>> judgedBy = "ci"; }
    }
}
"""

km, kc = _kernel.start()
for rel in SCHEMA:
    with open(os.path.join(ENGINE, *rel.split("/")), encoding="utf-8") as fh:
        _kernel.run_cell(kc, fh.read())
status, text = _kernel.run_cell(kc, SRC)
bad = any(w in (text or "").lower() for w in ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved", "extraneous", "wasn't expected"))
print(f"[{'FAIL' if bad else 'ok  '}] dialect v2 (verification + TestResult in action def)")
if bad:
    print((text or "").strip()[:800])
_kernel.teardown_and_exit(km, 0)
