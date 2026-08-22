// LIVE ALLOCATION EDGES (D0194, sprint 416) — SystemRequirement -> CodeElement, the standing-code
// half of the sanctioned allocation mechanism (the other half is the charter chain: SR ->
// #CharteredBy work -> delivered code). AUTHORED JUDGMENTS, edge-per-line so a reviewer can dispute
// any single mapping; only mappings where the named unit genuinely CARRIES the requirement are
// recorded — an SR whose carrier is unregistered (CI workflows, claude_surface) has no edge here
// rather than a dishonest one, and the tier-satisfaction allocation leg reports that gap honestly.
// The June Component allocations remain in requirements.sysml as superseded history (D0194).
package LiveAllocations {
    private import EngineElement::*;

    // kernel invariants -> their carrying units
    allocate srK01ValidatedChannel to ceWriteApi;
    allocate srK02FailLoud to ceHookDispatch;
    allocate srK05ComputedCompletion to ceComputedViews;
    allocate srK06HumanChannel to ceServeJudgment;
    allocate srK06HumanChannel to ceWriteApi;
    allocate srK07RecordedWeakening to ceHookDispatch;
    allocate srK12VerdictTraceability to ceLauncher;
    allocate srK12VerdictTraceability to ceAuditHistory;
    allocate srK15TreeDerivable to ceAuditHistory;

    // distributed-collaboration invariants
    allocate srDcUniqueIdentityGuard to ceGuards;
    allocate srDcCollisionResistantIds to ceWriteApi;
    allocate srDcMergeOnlyIntegration to ceSyncIntegration;
    allocate srDcGateOnMergedTree to ceSyncIntegration;
    allocate srDcGateOnMergedTree to ceLandGate;
    allocate srDcActorIdentityBinding to ceActorProvenance;
    allocate srDcActorKindClassification to ceActorProvenance;
    allocate srDcWorkClaim to ceRecordClaim;
    allocate srDcWorkClaim to ceClaimHolderRule;
    allocate srDcStaleClaimExpiry to ceClaimHolderRule;
    allocate srDcSyncAwareOrient to ceOrient;

    // serve discipline
    allocate srServeApproveGateHuman to ceServeJudgment;
    allocate srServeApproveGateHuman to ceLauncher;
    allocate srServeSingleSpawnPoint to ceServeJudgment;

    // portability
    allocate srDcPortableGate to ceGitSpawn;
}
