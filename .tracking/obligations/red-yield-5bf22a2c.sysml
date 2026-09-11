// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded - an act that happened, so discoveredInField is true (dcControlCensusByActorKind); a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligation5bf22a2c {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligation5bf22a2c : Issue {
        :>> id = "5bf22a2c-5a24-4f9e-83f5-d3710be2b44b";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [ceremony] sprint663_proposedOutcomeTier: Implement passed but Refine (earlier) unpassed; Implement passed but Standup (earlier) unpassed. Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = true;
        :>> severity = Severity::Low;
    }
    // Discharged by D0437: the ceremony guard's order check now reads the RECORDED set, and the
    // rebuilt guard is green over sprint663 (issue470).
    #Resolves dependency from d0437 to obligation5bf22a2c;
}
