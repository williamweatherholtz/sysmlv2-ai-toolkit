"""Spike: which native requirement-traceability relationships does the pilot
support? satisfy/verify are known; test derive/refine/trace + verify-by-case."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    "satisfy_by": """
package S1 { requirement def R; part def P; requirement r : R; part p : P; satisfy r by p; }
""",
    "verify_by_case": """
package S2 {
    requirement def R; requirement r : R;
    verification def V; verification v : V;
    verify r by v;
}
""",
    "verify_in_case": """
package S3 {
    requirement def R; requirement r : R;
    verification def V { subject s : R; }
    verification v : V { verify r; }
}
""",
    "derive_keyword": """
package S4 { requirement def R; requirement hi : R; requirement lo : R; derive lo from hi; }
""",
    "refine_keyword": """
package S5 { requirement def R; use case def U; requirement r : R; use case u : U; refine r by u; }
""",
    "trace_keyword": """
package S6 { requirement def R; requirement a : R; requirement b : R; trace a to b; }
""",
    "require_constraint_block": """
package S7 {
    private import ScalarValues::*;
    requirement def R { attribute m : Real; require constraint { m > 0 } }
}
""",
}

km, kc = _kernel.start()
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    bad = any(w in (text or "").lower() for w in ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved", "extraneous", "wasn't expected"))
    print(f"[{'FAIL' if bad else 'ok  '}] {name}")
    if bad:
        print("    " + (text or "").strip().replace("\n", "\n    ")[:300])
_kernel.teardown_and_exit(km, 0)
