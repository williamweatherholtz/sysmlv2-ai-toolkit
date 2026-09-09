// D0396 (status: the `status` field below is the truth) — planCoversStep: a signed plan covers the steps it names - a marker Decision recorded with plan and step is accepted at record time under the plan's human signature, and a guard holds the cover to the plan
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0396Accept event on sign-off.
package Decision0396 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0396 : Decision {
        :>> id = "3ad2f3e2-701b-4d9f-84db-51fbb3b95b72";
        :>> title = "planCoversStep: a signed plan covers the steps it names - a marker Decision recorded with plan and step is accepted at record time under the plan's human signature, and a guard holds the cover to the plan";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0375 option C, the human's word 2026-09-08: a safety-change or process-change Decision whose item is NAMED in the decision text of a Decision the human accepted THEMSELVES is covered by that acceptance and recorded accepted at record time, so the human signs once, on the plan, and its enumerated steps do not re-ask. Standing consent still does not reach the enforcement surface, because the plan it flows through must carry a human's own signature. This Decision is itself the live case: it is recorded with plan d0375 and step plan-covers-step, and D0375's consequences text names plan-covers-step, so it accepts at record time under D0375's human signature.";
        :>> decision = "NOT A FORK: this records the single mechanism D0375 option C chose. A marker Decision recorded with plan: dNNNN and step: <name> is accepted at record time when its plan is present, carries a human's OWN acceptance - an acceptance note free of the AUTO-ACCEPTED and PLAN-COVERED tokens, judged by a decider named in github-actors.toml - and names the step in its own decision or consequences text matched exactly. The acceptance note carries a PLAN-COVERED token with the plan id and the step, judged by the plan's judge. When any of those conditions is absent the Decision stays proposed and the missing one is named. A plan cannot cover itself or a plan-covered Decision, keeping the cover one level deep so a human signature is never laundered through a chain. Guard plan-covers-step re-runs the same conditions against the tree and fails a covered Decision whose plan no longer holds them, so the cover cannot outlive the plan it cites.";
        :>> rationale = "The human signed the plan, not each step, so re-asking for every enumerated step is friction the plan already resolved (D0054); but the signature is only as good as the plan it cites, so the cover is a re-checkable claim, not a one-time flip. One level keeps the human's word load-bearing: a plan-covered Decision cannot become a plan, so a signature can never be laundered through a chain. The guard is what makes the cover honest after the fact - if the plan is un-accepted, re-accepted under consent, loses its decider, or stops naming the step, the covered Decision is flagged as a signature that no longer exists.";
        :>> consequences = "keel-cli/src/plan_cover.rs assesses the three clauses and formats the note; the record path accepts a marker Decision at record time when a plan covers its step, or leaves it proposed naming the clause; guard plan-covers-step (68) re-checks every PLAN-COVERED acceptance; keel-cli/tests/signed_plan_covers_its_steps.rs covers accept, clause c, an auto-accepted plan, a missing plan, and the guard failing when the plan drops the step. This Decision is accepted at record time as PLAN-COVERED by d0375 step plan-covers-step - the live case the DoD names.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0396Accept : Test { :>> id = "99f07ed2-0cdd-442e-8b95-738043a1a733"; :>> method = VerificationMethod::confirmation; :>> procedureText = "PLAN-COVERED under d0375 step 'plan-covers-step': the human signed the plan itself and this enumerated step does not re-ask (D0375 option C / D0396). The plan's judge: wweatherholtz."; }
    part d0396AcceptR1 : TestResult { :>> id = "798f816b-eeec-4341-99e4-13fe4320be03"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2160187"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
