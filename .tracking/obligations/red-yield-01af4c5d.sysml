// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded - an act that happened, so discoveredInField is true (dcControlCensusByActorKind); a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligation01af4c5d {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligation01af4c5d : Issue {
        :>> id = "01af4c5d-31d1-4dd7-9e1d-1bc73c887405";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [parser-coverage] skipped-statement count 9 EXCEEDS the committed baseline 7 — a new statement class became invisible to every guard and view (issue231 ratchet). Either teach keel-parser the construct, or raise the baseline IN THE SAME COMMIT with the reason (a visible diff, never silent growth).. Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = true;
        :>> severity = Severity::Low;
    }
    // The 9 were the committed 7 plus two malformed `#Resolves d0437 part ...` statements the verifier
    // subagent hand-wrote into two other obligations (issue471); corrected to dependency edges, the
    // count read 7 again. Resolves with the verifier's mandate becoming a skill.
    #Resolves dependency from dcVerifierProcedureIsASkill to obligation01af4c5d;
}
