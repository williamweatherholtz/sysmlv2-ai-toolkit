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
        :>> title = "Definition of Done";
        :>> purpose = "Gate before setting a Story to a terminal state: every check below passes.";
        :>> cadence = "per-item";
    }

    part dodMerged : ProcessStep {
        :>> title = "Merged";
        :>> order = 1;
        :>> actionText = "Change is merged to main / PR approved.";
        :>> owner = "ai";
    }
    part dodVerified : ProcessStep {
        :>> title = "Satisfaction == verified";
        :>> order = 2;
        :>> actionText = "Computed satisfaction(Story) == verified: every verifying Test "
                       + "has a covering pass (commit ancestry), transitively. No "
                       + "suspect/failing/uncovered.";
        :>> owner = "ai";
    }
    part dodStaleness : ProcessStep {
        :>> title = "Downstream staleness cleared";
        :>> order = 3;
        :>> actionText = "Items this Story's changes put into `suspect` are resolved "
                       + "(re-verified) or have follow-up Stories.";
        :>> owner = "either";
    }
    part dodDocs : ProcessStep {
        :>> title = "Docs in sync";
        :>> order = 4;
        :>> actionText = "If user-facing behavior changed, docs updated in the same PR.";
        :>> owner = "either";
    }
}
