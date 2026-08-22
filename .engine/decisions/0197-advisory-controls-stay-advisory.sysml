// D0197 (status: the `status` field below is the truth) — D0197: non-promotion - the advisory tiers stay advisory, on 33 sprints of fire-ledger evidence
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0197Accept event on sign-off.
package Decision0197 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0197 : Decision {
        :>> id = "95e5f198-4c76-49a0-9069-9f28d4621800";
        :>> title = "D0197: non-promotion - the advisory tiers stay advisory, on 33 sprints of fire-ledger evidence";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "K14/D0180 require that any promotion OR non-promotion of advisory controls to blocking cite at least the fixed window of fire-ledger evidence - and the panel (R1 automotive finding 1) caught pmMeasureDoD stamped pass before any such review existed, reopening it. THE EVIDENCE, computed by keel enforcement-report over .keel/metrics/hooks.jsonl at sprint 418: 1122 ledger lines spanning sprints 384-417 (33 sprints, 2 harness sessions), per-event: pre-bash 845 fires / 0 blocks, post-edit 160/0, subagent-stop 80/0, stop 12/0, pre-write 12/0, user-prompt 12/0, actor-rebind 1/0. Zero blocks across every advisory event; the two real blocks in the window came from the HARD tiers (the commit gate refusing sprints 395/407 landings; the scaffold-placeholder guard), which are already blocking. Adherence trend: the ProcessDefect-marked issue count grew only by panel-found historical defects, none an advisory-tier escape.";
        :>> decision = "PROPOSED: NON-PROMOTION. Every advisory control (the pre-bash shell advisory, the post-edit fast tier's warning modes, the warning-tier guards 38-45, the pre-write ask tier in the guided profile) STAYS advisory. The evidence supports no promotion: zero advisory fires wanted to block anything in 33 sprints, so promotion would add friction (D0054's #1 risk) while preventing nothing the record shows happening. REVISIT CONDITION, declared: a promotion review re-runs when either (a) any advisory event logs a block-worthy escape (a violation an advisory fire flagged and the turn ignored), or (b) the guided-to-strict profile question is opened for a downstream team - whichever first. This Decision satisfies pmMeasureDoD's outstanding criterion; the item's DoD flips only after this is SIGNED (a proposed review is not a review in force).";
        :>> rationale = "The panel reopened pmMeasure precisely because K14's discharge was claimed without this review existing; recording NON-promotion with the numbers is the honest completion - K14 never demanded promotion, it demanded that the promote-or-not question be answered FROM evidence rather than by default. Zero blocks is a real answer: the in-loop discipline plus hard gates are absorbing violations before advisory tiers would need teeth.";
        :>> consequences = "IF ACCEPTED: pmMeasureDoD's criterion is met (an R3 pass will cite this Decision + the report), pmMeasure leaves the frontier, and the revisit condition is on record. IF REJECTED: the human names which control to promote and the same evidence frames it. COST: none mechanical - this Decision changes no behavior, which is its content.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0197Accept : Test { :>> id = "d967eff2-cfa0-4cd2-ac9d-a0652b62b14e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Accepted via their deck tap (mobile inbox row, uid 95e5f198, At 2026-08-22T06:54:43Z, deck HEAD 3c9645e), confirmed in chat the same morning: 'updated. accept all sitting reviews and proceed'. The non-promotion verdict and its revisit condition are accepted as written."; }
    part d0197AcceptR1 : TestResult { :>> id = "5ff138be-aae4-47f4-8545-70f78ac07467"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3c9645e"; :>> judgedAt = "2026-08-22"; :>> judgedBy = "wweatherholtz"; }
}
