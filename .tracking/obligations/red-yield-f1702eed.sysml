// OBLIGATION (auto-recorded, D0176/K7): a control was overridden or yielded; a human review
// discharges it (triage with a #Resolves edge). One file per fact so recording never deadlocks
// on the file being repaired.
package Obligationf1702eed {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineRelationships::*;

    part obligationf1702eed : Issue {
        :>> id = "f1702eed-d2c1-4d64-a40e-a568bab7ef02";
        :>> title = "turn gate yielded while red - the tree needs a correction pass";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> description = "The Stop hook's second red pass yielded (loop avoidance). Session: b43aa4fc-84f7-4c3d-bc35-61eccafbbda8. First problem at yield: keel guard: [marker-vocabulary] .tracking/delivery/sprint647_refusalsAreLedgerFacts.sysml:39: marker `#Addresses` is NOT declared as a `metadata def` — markers are not type-checked, so an undeclared or MISSPELLED marker validates clean and silently removes this item from whatever guard or view depends on it (issue077/D0133). If it is a TYPO, fix the spelling. If it is your project's own marker, declare `metadata def Addresses;` in any of your own .engine or .tracking files — you do NOT need to touch frozen schema/core (D0136). The engine's own markers are always valid without declaration. [marker-vocabulary] .tracking/delivery/sprint647_refusalsAreLedgerFacts.sysml:40: marker `#Addresses` is NOT declared as a `metadata def` — markers are not type-checked, so an undeclared or MISSPELLED marker validates clean and silently removes this item from whatever guard or view depends on it (issue077/D0133). If it is a TYPO, fix the spelling. If it is your project's own marker, declare `metadata def Addresses;` in any of your own .engine or .tracking files — you do NOT need to touch frozen schema/core (D0136). The engine's own markers are always valid without declaration. [marker-vocabulary] .tracking/delivery/sprint647_refusalsAreLedgerFacts.sysml:41: marker `#Addresses` is NOT declared as a `metadata def` — markers are not type-checked, so an undeclared or MISSPELLED marker validates clean and silently removes this item from whatever guard or view depends on it (issue077/D0133). If it is a TYPO, fix the spelling. If it is your project's own marker, declare `metadata def Addresses;` in any of your own .engine or .tracking files — you do NOT need to touch frozen schema/core (D0136). The engine's own markers are always valid without declaration.. Discharge: make the tree green and triage this obligation with a #Resolves edge.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcDelegatedCeremonyIsASkill to obligationf1702eed;
}
