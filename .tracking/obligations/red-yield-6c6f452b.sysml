// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded - an act that happened, so discoveredInField is true (dcControlCensusByActorKind); a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligation6c6f452b {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligation6c6f452b : Issue {
        :>> id = "6c6f452b-34da-4901-b445-5893dde8d4fa";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-11"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [sprint-closure] sprint674_controlProofStateIsComputed.sysml: task `storyControlProofStateIsComputed` has no TestResult, but work has moved on to a later sprint — an unstamped task is served as READY forever, so finished work is indistinguishable from open work (D0260/sprint483) [sprint-closure] sprint675_workingTreeEolMatchesTheAttribute.sysml: task `storyWorkingTreeEolMatchesTheAttribute` has no TestResult, but work has moved on to a later sprint — an unstamped task is served as READY forever, so finished work is indistinguishable from open work (D0260/sprint483). Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = true;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcSprintCeremonyIsOneWrite to obligation6c6f452b;
}
