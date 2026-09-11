// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded - an act that happened, so discoveredInField is true (dcControlCensusByActorKind); a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligation0a8b234f {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligation0a8b234f : Issue {
        :>> id = "0a8b234f-a9e5-4213-8c02-667a440ab3e2";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [cli-surface-declared] `controls` synopsis differs: facts say `the two-way hazard/control diff: uncovered failure conditions and unanchored controls; per control DECLARED, ARMED and PROOF (PROVEN / UNPROVEN / UNDETERMINED, D0360)`, cli_facts.rs says `the two-way hazard/control diff: uncovered failure conditions and unanchored controls` - one home, the .sysml; regenerate the mirror. Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = true;
        :>> severity = Severity::Low;
    }

    #Resolves dependency from dcCliSurfaceGuardNamesAStaleBinary to obligation0a8b234f;
}
