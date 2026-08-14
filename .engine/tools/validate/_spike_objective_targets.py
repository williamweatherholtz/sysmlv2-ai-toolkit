"""Settle the D0139 clause (E) blocker: base SysML v2 verification puts the verified thing in the
`objective` of a verification case. The engine's #Verify marker (453 edges) targets Stories, Decisions,
actions and gates — NOT only requirements. So the question the base-first migration actually turns on is:

    can `objective` reference something that is NOT a requirement usage?

If it cannot, then #Verify is not a relationship that base `verify`/`objective` can absorb for the
majority of its uses, and clause (E) is under-scoped in a second, independent way beyond issue097.

KERNEL ONLY — `keel check` is permissive and must not be used to answer this (issue097).
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    # baseline: the shape the repo's notes call "the only valid verification shape"
    "objective_requirement (baseline)":
        "package O1 { requirement def R; requirement r : R; verification def V { subject s; objective r; } }",
    # can the objective be a PART usage? (Story is a part def in work.sysml)
    "objective_part_usage":
        "package O2 { part def P; part p : P; verification def V { subject s; objective p; } }",
    # can the objective be an ACTION usage? (sprint DoD gates verify actions)
    "objective_action_usage":
        "package O3 { action def A; action a : A; verification def V { subject s; objective a; } }",
    # can the objective be a requirement DEF rather than a usage?
    "objective_requirement_def":
        "package O4 { requirement def R; verification def V { subject s; objective R; } }",
    # does an objective declared as a nested requirement usage work (no external ref)?
    "objective_nested_requirement":
        "package O5 { verification def V { subject s; objective { doc /* inline */ } } }",
    # the repo's actual Test shape: a verification def specializing an abstract verification def
    "verification_def_specializing_abstract":
        "package O6 { abstract verification def TV; verification def T :> TV; }",
    # can a verification USAGE carry the objective (per-instance verification)?
    "objective_in_verification_usage":
        "package O7 { requirement def R; requirement r : R; verification def V; verification v : V { objective r; } }",
    # cross-package: objective referencing an imported requirement (the repo's real topology)
    "objective_cross_package":
        "package O8a { requirement def R; requirement r : R; } package O8b { private import O8a::*; verification def V { subject s; objective r; } }",
}

km, kc = _kernel.start()
ok, bad = [], []
BAD_WORDS = ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved",
             "extraneous", "wasn't expected", "missing", "must be in the objective")
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
_kernel.teardown_and_exit(km, 0)
