// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded - an act that happened, so discoveredInField is true (dcControlCensusByActorKind); a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligationf76e7aba {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligationf76e7aba : Issue {
        :>> id = "f76e7aba-d784-4b10-a935-3762fe174774";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [evidence-cited] .tracking/delivery/sprint649_ucaCarriesItsEvidenceClass.sysml:28: ucaCarriesItsEvidenceClassImplementGateR1 is an AI-judged method=test result with no `// RAN:` receipt - a pass nobody else can re-derive is a testimony, not a test. Pass --evidence to append-result (D0232). Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = true;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcReceiptlessResultIsRefusedAtWrite to obligationf76e7aba;
}
