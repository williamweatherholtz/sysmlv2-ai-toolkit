"""Discriminator: is `objective X;` a REFERENCE to an existing element, or does it DECLARE a new
nested requirement usage named X?

Why it matters: relationships.sysml:7-9 and sysmlv2-syntax-notes.md:40 record
`verification def V { subject s; objective r; }` as "the only valid verification shape", and D0139(E)
names it as the migration target for 453 `#Verify` edges. But a migration target is only useful if it
actually LINKS the verification to the verified element. If `objective r;` declares a fresh requirement
usage that merely happens to be named `r`, then it carries NO edge to the outer `r`, and the shape is
not a relationship at all — it cannot replace #Verify no matter how invasive the remodelling.

The clean discriminator: give `objective` a name that is NOT declared anywhere.
  - if it FAILS  -> `objective` resolves a reference (the recorded shape really is an edge)
  - if it PASSES -> `objective` DECLARES a member (the recorded shape is not an edge)

`subject` is tested the same way as a control, and the `::>` reference-redefinition forms that WOULD
express a genuine link are probed as candidate real targets.

KERNEL ONLY (issue097).
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    # THE DISCRIMINATOR: nothing named zzz exists anywhere.
    "objective_UNDECLARED_name":
        "package B1 { verification def V { subject s; objective zzz; } }",
    "subject_UNDECLARED_name":
        "package B2 { verification def V { subject zzz2; } }",
    # control: a genuinely unresolvable TYPE reference must fail, proving the kernel does resolve refs
    "control_unresolvable_type":
        "package B3 { part p : NoSuchTypeAnywhere; }",
    # control: an unresolvable satisfy target must fail (satisfy IS a real reference)
    "control_satisfy_undeclared":
        "package B4 { requirement def R; requirement r : R; satisfy r by zzz3; }",
    # candidate REAL link forms: redefinition / subsetting of an outer requirement usage
    "objective_redefines_outer":
        "package B5 { requirement def R; requirement r : R; verification def V { subject s; objective :>> r; } }",
    "objective_subsets_outer":
        "package B6 { requirement def R; requirement r : R; verification def V { subject s; objective :> r; } }",
    "objective_typed_by_requirement_def":
        "package B7 { requirement def R; verification def V { subject s; objective o : R; } }",
    # does a satisfy edge from the verification to the requirement work as the real link?
    "verify_via_satisfy_from_verification":
        "package B8 { requirement def R; requirement r : R; verification def V; verification v : V; satisfy r by v; }",
}

km, kc = _kernel.start()
ok, bad = [], []
BAD_WORDS = ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved",
             "extraneous", "wasn't expected", "missing")
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    low = (text or "").lower()
    failed = any(w in low for w in BAD_WORDS)
    print(f"[{'FAIL' if failed else 'ok  '}] {name}", flush=True)
    if failed:
        lines = (text or "").strip().splitlines()
        print("    " + (lines[0][:200] if lines else "(no message)"), flush=True)
        bad.append(name)
    else:
        ok.append(name)

print(f"\n== VALID ({len(ok)}) ==")
for n in ok:
    print("  " + n)
print(f"\n== INVALID ({len(bad)}) ==")
for n in bad:
    print("  " + n)
print("\nINTERPRETATION: if objective_UNDECLARED_name is ok while control_satisfy_undeclared FAILS,")
print("then `objective X` DECLARES a member and is NOT a reference to an existing element.")
_kernel.teardown_and_exit(km, 0)
