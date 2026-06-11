"""Consolidated spike: analysisCalc (calc/analysis def) + nativeAlloc (allocate,
interface def, connection) constructs in the pilot. Maps feasibility in one pass."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    # analysisCalc
    "calc_def_return": "package C1 { private import ScalarValues::*; calc def Coverage { return r : Real; } }",
    "calc_def_with_expr": "package C2 { private import ScalarValues::*; calc def Add { in a : Real; in b : Real; return s : Real = a + b; } }",
    "analysis_def": "package C3 { analysis def CoverageAnalysis; }",
    "analysis_case_def": "package C4 { analysis case def CoverageCase; }",
    # nativeAlloc
    "allocate_stmt": "package C5 { part def Fn; part def Comp; part f : Fn; part c : Comp; allocate f to c; }",
    "allocation_def": "package C6 { part def Fn; part def Comp; allocation def FnToComp; }",
    "interface_def": "package C7 { interface def Ifc; }",
    "connection_def": "package C8 { part def A; part def B; connection def Conn; }",
    "connect_stmt": "package C9 { part def A { part x; part y; connect x to y; } }",
}

km, kc = _kernel.start()
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    bad = any(w in (text or "").lower() for w in ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved", "extraneous", "wasn't expected"))
    print(f"[{'FAIL' if bad else 'ok  '}] {name}")
    if bad:
        print("    " + (text or "").strip().replace("\n", "\n    ")[:200])
_kernel.teardown_and_exit(km, 0)
