// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded; a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligation0953c012 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligation0953c012 : Issue {
        :>> id = "0953c012-3359-41e0-bdd9-4f4523b876e1";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeFable5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: 60405c11-f525-4aed-b42f-e87923d904f2. First problem at yield: keel guard: [viewpoint-renderer] attestation-coverage: renderer names no known keel command — 'keel attestation-coverage' [viewpoint-renderer] authority-queue: renderer names no known keel command — 'keel authority-queue' [viewpoint-renderer] concern-coverage: renderer names no known keel command — 'keel concern-coverage' [viewpoint-renderer] critique-policy: renderer names no known keel command — 'keel critique-policy' [viewpoint-renderer] decisions: renderer names no known keel comma. Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }

    // TRIAGED (D0077). The red that caused the yield was TRANSIENT and is already gone: for a few
    // minutes the built binary was post-collapse while the viewpoint registry was pre-collapse, so
    // every renderer named a verb the binary no longer had. Stashing the D0273 work and rebuilding
    // restored the pair to agreement and the renderer guard passes.
    //
    // It resolves with dcCliSurfaceConsolidation rather than being closed as noise, because the same
    // window reopens the moment that work is unstashed - and the real blocker behind it, issue330, is
    // a question only the owner of ten Viewpoints can answer.
    #Resolves dependency from dcCliSurfaceConsolidation to obligation0953c012;
}
