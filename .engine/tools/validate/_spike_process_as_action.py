"""Kernel-verify the target shape for dcProcessAsBehaviour BEFORE building it (the issue097 rule).

The panel found `Process`/`ProcessStep` modelled as `part def` with behaviour in strings and sequence in
an `order : Integer`. The proposed fix is `action def` + `first..then`. That fix is only worth building
if the SHAPE the instances would take is valid SysML v2 — a typed ACTION USAGE carrying attributes, an
`assert constraint` member (D0141 attached 14 of those to Process parts and they must survive), and
package-level successions between usages.

Every case below is a shape the migration would actually emit. A FAIL here kills or reshapes the
migration; it must not be discovered after 117 instances have been rewritten.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    # the definition side
    "action_def_specializing_abstract":
        "package A1 { abstract action def Base; action def Proc :> Base; }",
    # the usage side — this is what every Process instance becomes
    "typed_action_usage_with_body":
        "package A2 { action def Proc; action deploy : Proc { } }",
    "typed_action_usage_with_attributes":
        "package A3 { attribute def T; action def Proc { attribute title : T; } action deploy : Proc { :>> title = t; } }",
    # D0141's asserts must survive the retype
    "assert_constraint_in_action_usage":
        "package A4 { constraint def Ok; action def Proc; action deploy : Proc { assert constraint c : Ok; } }",
    "assert_constraint_in_action_def":
        "package A5 { constraint def Ok; action def Proc { assert constraint c : Ok; } }",
    # sequence replaces `order : Integer`
    "succession_between_package_level_usages":
        "package A6 { action def Step; action a : Step; action b : Step; first a then b; }",
    "succession_inside_action_def":
        "package A7 { action def Step; action def Proc { action a : Step; action b : Step; first a then b; } }",
    # can an action def carry the tracked-identity base the engine needs?
    "abstract_action_def_with_attributes":
        "package A8 { attribute def T; abstract action def TrackedAction { attribute id : T; } action def Proc :> TrackedAction; }",
    # CONTROL: an undeclared type must still fail, proving the kernel is resolving at all
    "control_undeclared_action_type":
        "package A9 { action deploy : NoSuchActionDefAnywhere { } }",
}

km, kc = _kernel.start()
ok, bad = [], []
BAD = ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved",
       "extraneous", "wasn't expected", "must be", "must have")
for name, src in CASES.items():
    _, text = _kernel.run_cell(kc, src)
    low = (text or "").lower()
    failed = any(w in low for w in BAD)
    print(f"[{'FAIL' if failed else 'ok  '}] {name}", flush=True)
    if failed:
        first = (text or "").strip().splitlines()
        print("    " + (first[0][:150] if first else "(no message)"), flush=True)
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
