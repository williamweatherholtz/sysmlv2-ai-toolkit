// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded - an act that happened, so discoveredInField is true (dcControlCensusByActorKind); a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligationf3b226aa {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligationf3b226aa : Issue {
        :>> id = "f3b226aa-65d3-486e-9f72-dca95f92f253";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [issues] obligation5bf22a2c: untriaged — no #Resolves edge (D0077; link a resolving action or Decision). Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = true;
        :>> severity = Severity::Low;
    }
    // Cascade of obligation5bf22a2c (the issues guard on its untriaged state); discharged with it.
    #Resolves dependency from d0437 to obligationf3b226aa;
}
