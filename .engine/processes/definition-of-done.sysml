// =====================================================================
// EngineDoD — Definition of Done: the gate a Story passes before a terminal state.
//
// instanceMigration 2026-06-11: Gate/GateCheck are GONE (CLAUDE.md §2.7 — a gate/DoD
// is its verify-linked Tests passing, not an authored Gate type). DoD is reconceived
// as a `Process` whose checks are ordered `ProcessStep`s; each step's `actionText` is
// the verifiable rule. The pass/fail of each is a COMPUTED view (satisfaction +
// git-ancestry), never authored.
// =====================================================================
package EngineDoD {
    private import EngineProcess::*;

    part dod : Process {
        :>> id = "88f837a6-3bb7-4dc7-a903-3017b9e62747";
        :>> title = "Definition of Done";
        :>> purpose = "Gate before setting a Story to a terminal state: every check below passes.";
        :>> cadence = "per-item";
    }

    part dodMerged : ProcessStep {
        :>> id = "5d88008f-fb04-450f-a71b-cdf50041fe01";
        :>> title = "Merged";
        :>> order = 1;
        :>> actionText = "Change is merged to main / PR approved.";
        :>> owner = "ai";
    }
    part dodVerified : ProcessStep {
        :>> id = "46ec4593-4d12-42c2-9248-492db952f02e";
        :>> title = "Satisfaction == verified";
        :>> order = 2;
        :>> actionText = "Computed satisfaction(Story) == verified: every verifying Test "
                       + "has a covering pass (commit ancestry), transitively. No "
                       + "suspect/failing/uncovered.";
        :>> owner = "ai";
    }
    part dodStaleness : ProcessStep {
        :>> id = "a6b3bdc1-0454-4b66-bffa-fbbc9be27e7a";
        :>> title = "Downstream staleness cleared";
        :>> order = 3;
        :>> actionText = "Items this Story's changes put into `suspect` are resolved "
                       + "(re-verified) or have follow-up Stories.";
        :>> owner = "either";
    }
    part dodDocs : ProcessStep {
        :>> id = "1741a1c7-6b24-4823-975c-92be30ecdbad";
        :>> title = "Docs in sync";
        :>> order = 4;
        :>> actionText = "Run the Documentation Sync process (EngineDocSync): every doc claim "
                       + "the change invalidates is fixed in the same commit.";
        :>> owner = "either";
    }
}
