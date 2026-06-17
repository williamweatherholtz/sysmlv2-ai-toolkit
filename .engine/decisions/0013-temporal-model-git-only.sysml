package Decision0013 {
    private import EngineWork::*;

    part d0013 : Decision {
        :>> id = "3b87ce25-4a33-4e86-8832-dfa79c08dea5";
        :>> title = "The engine clock is git ancestry, not native occurrence modeling";
        :>> status = DecisionStatus::accepted;
        :>> context = "SysML v2 offers occurrence/snapshot/timeslice + HappensBefore for temporal structure. The engine needed happened-before for verification staleness.";
        :>> decision = "Stay git-only: commits are the engine timeline; merge-base --is-ancestor orders events even across divergent branches. Native occurrence/snapshot remains available for DELIVERABLE/subject-version modeling but is NOT the engine tracking clock.";
        :>> rationale = "The engine needed happened-before for verification staleness; wall-clock cannot order events across divergent branches, but git ancestry can. Modeling time in-model would duplicate git and go stale (invariant 2.1). Accepted by user 2026-06-11 (commit 0e8b24c).";
        :>> consequences = "No second source of temporal truth; timestamps (verifiedAt) are display-only authored facts of the attestation moment.";
    }
}
