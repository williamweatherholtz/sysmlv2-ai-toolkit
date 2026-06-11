package Decision0013 {
    private import EngineWork::*;

    part d0013 : Decision {
        :>> title = "The engine clock is git ancestry, not native occurrence modeling";
        :>> status = "accepted";
        :>> context = "SysML v2 offers occurrence/snapshot/timeslice + HappensBefore for temporal structure. The engine needed happened-before for verification staleness.";
        :>> decisionText = "Stay git-only: commits are the timeline; merge-base --is-ancestor orders events even across divergent branches (wall-clock cannot). Native occurrence/snapshot remains available for DELIVERABLE/subject-version modeling - it is not the engine tracking clock; modeling time in-model would duplicate git and go stale (invariant 2.1). Accepted by user 2026-06-11 (commit 0e8b24c).";
        :>> consequences = "No second source of temporal truth; timestamps (verifiedAt) are display-only authored facts of the attestation moment.";
    }
}
