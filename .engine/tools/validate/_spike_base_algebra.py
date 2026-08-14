"""Settle, against the KERNEL, which base SysML v2 constructs are actually valid — the authoritative
table the base-first programme (D0139) depends on.

WHY THIS EXISTS (issue097): the rust authority (`keel check`) is PERMISSIVE. It accepted
`verify X by Y` and `refine X by Y`, which the kernel REJECTS, and that false positive reached an
accepted Decision as a migration target for 613 edges. `keel validate` being green does not mean a file
is valid SysML v2, so any claim about what base SysML v2 offers MUST be measured here, not there.

One JVM, many cells — kernel startup dominates, so batching is the difference between minutes and hours.
Each case is deliberately MINIMAL so a failure is attributable to the construct under test and not to
scaffolding. Cases are grouped by the question they answer.

Read the output as: `ok` = the kernel accepts it, so it is portable to a standard viewer (SysON).
`FAIL` = it is not valid SysML v2 regardless of what the rust parser says.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

# Each entry: name -> source. Keep every case to ONE package and the fewest declarations that make the
# construct legal, so a FAIL means the construct failed.
CASES = {
    # ── relationships the engine currently expresses with custom markers ──
    "refine_at_package": "package A1 { requirement def R; requirement a : R; requirement b : R; refine a by b; }",
    "trace_at_package": "package A2 { part def P; part a : P; part b : P; trace a to b; }",
    "derive_at_package": "package A3 { requirement def R; requirement a : R; requirement b : R; derive a from b; }",
    "specialize_requirement": "package A4 { requirement def R; requirement a : R; requirement b :> a; }",
    "satisfy_by (baseline, known good)": "package A5 { requirement def R; part def P; requirement r : R; part p : P; satisfy r by p; }",
    "allocate_to (baseline, known good)": "package A6 { part def P; part a : P; part b : P; allocate a to b; }",
    "dependency_bare": "package A7 { part def P; part a : P; part b : P; dependency a to b; }",
    "dependency_from_to": "package A8 { part def P; part a : P; part b : P; dependency from a to b; }",

    # ── constraints: the candidate home for process -> controls (D0139 clause D) ──
    "assert_constraint_in_part": "package B1 { constraint def Ok; part def Proc { assert constraint c : Ok; } }",
    "assert_constraint_at_package": "package B2 { constraint def Ok; assert constraint c : Ok; }",
    "require_constraint_in_requirement": "package B3 { constraint def Ok; requirement def R { require constraint c : Ok; } }",
    "constraint_usage_in_part": "package B4 { constraint def Ok; part def Proc { constraint c : Ok; } }",

    # ── behaviour: does a Process legitimately PERFORM its steps? ──
    "perform_action_in_part": "package C1 { action def Step; part def Proc { perform action s : Step; } }",
    "action_usage_in_part": "package C2 { action def Step; part def Proc { action s : Step; } }",
    "succession_in_action": "package C3 { action def Outer { action a; action b; first a then b; } }",

    # ── verification: the ONLY valid shape, re-confirmed, plus whether a subject can be typed ──
    "objective_in_verification_def": "package D1 { requirement def R; requirement r : R; verification def V { subject s; objective r; } }",
    "objective_typed_subject": "package D2 { requirement def R; part def P; requirement r : R; verification def V { subject s : P; objective r; } }",
    "verification_usage_of_def": "package D3 { requirement def R; requirement r : R; verification def V { subject s; objective r; } verification v : V; }",

    # ── multi-valued references: the gap that pushed the engine toward markers ──
    "ref_single_value": "package E1 { part def C; part def Proc { ref e : C; } part c1 : C; part p : Proc { :>> e = c1; } }",
    "ref_multi_sequence": "package E2 { part def C; part def Proc { ref e : C[*]; } part c1 : C; part c2 : C; part p : Proc { :>> e = (c1, c2); } }",
    "ref_multi_ordered": "package E3 { part def C; part def Proc { ref e : C[*] ordered; } part c1 : C; part p : Proc { :>> e = c1; } }",

    # ── ports / interfaces: the up/downstream connection surface ──
    "port_def_and_usage": "package F1 { port def Pt; part def P { port p : Pt; } }",
    "interface_def": "package F2 { port def Pt; part def P { port p : Pt; } interface def If { end a : Pt; end b : Pt; } }",
    "connect_ports": "package F3 { port def Pt; part def P { port p : Pt; } part x : P; part y : P; connect x.p to y.p; }",

    # ── use cases: upstream intent ──
    "include_use_case": "package G1 { use case def U; use case def V { include use case u : U; } }",
    "use_case_subject_actor": "package G2 { part def Sys; use case def U { subject s : Sys; } }",

    # ── metadata: the form the engine relies on ──
    "metadata_prefix_on_dependency": "package H1 { metadata def M; part def P; part a : P; part b : P; #M dependency from a to b; }",
    "metadata_prefix_on_part": "package H2 { metadata def M; part def P; #M part a : P; }",
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
        first = (text or "").strip().splitlines()
        print("    " + (first[0][:160] if first else "(no message)"), flush=True)
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
