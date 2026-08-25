// D0231 (status: the `status` field below is the truth) — adoptionCheck: gate a foreign tree every push, and state plainly the two defect classes it still cannot see
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0231Accept event on sign-off.
package Decision0231 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0231 : Decision {
        :>> id = "68d2a3d9-ff5d-4399-b533-49b183a5fec8";
        :>> title = "adoptionCheck: gate a foreign tree every push, and state plainly the two defect classes it still cannot see";
        :>> createdAt = "2026-08-25";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Asked which of three things was failing - verification not strong enough, the wrong measures, or the reporting from verification being inaccurate - the measurement said the first is the LEAST supported. 51 guards, 42 hard-blocking, 99 percent of live requirements examined and 91 percent exercised, and no recent defect traceable to a lax threshold. But 39 of 51 guards read only the authored model, 11 cross some boundary, and ZERO read another project's tree - while 11 of the last 20 issues were discovered in the field and cluster at exactly that boundary: another project's tree, an external service, a shell layer, a different binary, the human's attention. The measures are not weak and not mis-thresholded. They point INWARD, at the artifact this project authors, and nothing breaks there any more.";
        :>> decision = "`keel adoption-check` scaffolds a fresh project and, per declared unit, gates that FOREIGN tree in BOTH directions: clean WITHOUT the unit, because a control must not fire on a project that never adopted it, and clean WITH it freshly imported, because a unit must carry everything it references. It runs as a CI GATE on every push. It drives the real CLI by re-invoking the binary rather than calling internal functions, and it shares `restore_dst` with `import` rather than reimplementing the path mapping, so the fixture cannot resolve paths differently from what an adopter actually runs. A companion test makes the check FAIL on purpose and fails the build if it ever stops being able to.";
        :>> rationale = "This is the D0047 rule applied to a finding about the controls themselves: adoption had found four consecutive sprints of defects that inspection could not, and it was being done by hand, when someone remembered. Making it a gate is the difference between a habit and a control. Two design choices are load-bearing. FIRST, both directions, because the failures ran both ways - a unit that could not land (issue251, issue252, issue253) and a control that fired on a project which never opted in (issue259) - and a one-directional check would have looked complete while missing half the class. SECOND, the check is PROVEN ABLE TO FAIL. It reported 25 of 25 clean on its first run, which is the exact shape of a control aimed at nothing: this project has already shipped a guard passing over an empty population twice, and printed 'every unit travels whole' while a schema symbol did not travel.";
        :>> consequences = "STATED PLAINLY, because the honest scope is the point: this catches LESS than I first claimed. I wrote that it would have caught five defects; testing the claim showed TWO. The fixture is a CURRENT keel scaffold, so it is keel adopting keel - a weaker foreign-ness than a real adopter. issue263 is invisible to it because `keel init` ships the rules file defining the symbol, so the reference resolves where penumbra's older tree could not resolve it; issue259 is invisible for the same reason, since every process in the fixture already carries the current conventions and a convention-based guard has nothing to fire on. Representing an older VINTAGE needs a second fixture pinned to a prior release, which this is not, and that gap is recorded as issue265 rather than left implied. Two tests keep the scope honest: one proves the check can still go red on the class it does cover, and one FAILS THE BUILD if the stated limits are ever quietly deleted from the source. Runtime is about three minutes for 25 units. Resolves issue264.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0231Accept : Test { :>> id = "bd516784-802d-4147-91e3-3312af04d742"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/29."; }
    part d0231AcceptR1 : TestResult { :>> id = "a121d509-d85e-4d70-835e-36c113dab917"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3b2bb49"; :>> judgedAt = "2026-08-25"; :>> judgedBy = "wweatherholtz"; }
}
