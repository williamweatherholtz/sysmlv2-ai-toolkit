"""Follow-up to _spike_objective_binding.py, which established that `objective X;` DECLARES a member
(an undeclared name passes, while the `satisfy ... by <undeclared>` control fails to resolve).

Remaining question: do the REDEFINITION / SUBSETTING forms `objective :>> r;` and `objective :> r;`
carry a genuine reference to the outer element — i.e. is there any base construct inside a verification
case that actually LINKS to the verified element? Same discriminator: point them at a name that does
not exist. A real reference must fail to resolve.

KERNEL ONLY (issue097).
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    # if these FAIL to resolve, the form is a genuine reference (and thus a usable edge)
    "objective_redefines_UNDECLARED":
        "package R1 { verification def V { subject s; objective :>> zzzNope; } }",
    "objective_subsets_UNDECLARED":
        "package R2 { verification def V { subject s; objective :> zzzNope2; } }",
    "subject_redefines_UNDECLARED":
        "package R3 { verification def V { subject :>> zzzNope3; } }",
    # positive counterparts (must pass) so a FAIL above is attributable to the missing name
    "objective_redefines_DECLARED":
        "package R4 { requirement def R; requirement r : R; verification def V { subject s; objective :>> r; } }",
    "objective_subsets_DECLARED":
        "package R5 { requirement def R; requirement r : R; verification def V { subject s; objective :> r; } }",
    # can a subject reference the verified element by TYPE (the repo's Story is a part def)?
    "subject_typed_by_part_def":
        "package R6 { part def Story; verification def V { subject s : Story; } }",
    # and can a redefining objective point at a PART usage rather than a requirement usage?
    "objective_redefines_part_usage":
        "package R7 { part def P; part p : P; verification def V { subject s; objective :>> p; } }",
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
print("\nINTERPRETATION: an UNDECLARED name that FAILS proves that form is a real reference,")
print("i.e. a base construct that genuinely links a verification case to the verified element.")
_kernel.teardown_and_exit(km, 0)
