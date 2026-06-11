"""Settle it: does the pilot support a native `verify` relationship, and in what
syntax? satisfy is confirmed. Try several verify placements/forms."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    "verify_by_at_package (like satisfy)": """
package V1 { requirement def R; verification def Vc; requirement r : R; verification v : Vc; verify r by v; }
""",
    "verify_inside_verification_usage": """
package V2 { requirement def R; verification def Vc; requirement r : R; verification v : Vc { verify r; } }
""",
    "verify_inside_requirement": """
package V3 { requirement def R; verification def Vc; verification v : Vc; requirement r : R { verify v; } }
""",
    "verify_by_inside_requirement": """
package V4 { requirement def R; verification def Vc; verification v : Vc; requirement r : R { verify by v; } }
""",
    "verify_by_part (verify a req by a part)": """
package V5 { requirement def R; part def P; requirement r : R; part p : P; verify r by p; }
""",
    "objective_in_verification_def": """
package V6 { requirement def R; requirement r : R; verification def Vc { subject s; objective r; } }
""",
}

km, kc = _kernel.start()
ok_forms = []
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    bad = any(w in (text or "").lower() for w in ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved", "extraneous", "wasn't expected"))
    print(f"[{'FAIL' if bad else 'ok  '}] {name}")
    if bad:
        print("    " + (text or "").strip().replace("\n", "\n    ")[:200])
    else:
        ok_forms.append(name)
print("\nWORKING verify forms:", ok_forms or "NONE")
_kernel.teardown_and_exit(km, 0)
