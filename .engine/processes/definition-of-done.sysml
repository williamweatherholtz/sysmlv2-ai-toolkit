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
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;

    part dod : Process {
        :>> id = "88f837a6-3bb7-4dc7-a903-3017b9e62747";
        :>> title = "Definition of Done";
        :>> purpose = "Gate before setting a Story to a terminal state: every check below passes.";
        :>> cadence = Cadence::perItem;
    }

    part dodMerged : ProcessStep {
        :>> id = "5d88008f-fb04-450f-a71b-cdf50041fe01";
        :>> title = "Merged";
        :>> order = 1;
        :>> actionText = "[computed: git] Change is committed to main (main-only policy; auto-pushed).";
        :>> owner = Owner::ai;
    }
    part dodVerified : ProcessStep {
        :>> id = "46ec4593-4d12-42c2-9248-492db952f02e";
        :>> title = "Satisfaction == verified";
        :>> order = 2;
        :>> actionText = "[computed: query.py] The item's LATEST appended TestResult is a "
                       + "pass and it is not suspect (D0005 material-change suspicion).";
        :>> owner = Owner::ai;
    }
    part dodStaleness : ProcessStep {
        :>> id = "a6b3bdc1-0454-4b66-bffa-fbbc9be27e7a";
        :>> title = "Downstream staleness cleared";
        :>> order = 3;
        :>> actionText = "[computed: query.py suspect] Items this change put into suspect "
                       + "are re-verified (appended results) or have follow-up tasks.";
        :>> owner = Owner::either;
    }
    part dodDocs : ProcessStep {
        :>> id = "1741a1c7-6b24-4823-975c-92be30ecdbad";
        :>> title = "Docs in sync";
        :>> order = 4;
        :>> actionText = "[judgment] Run the Documentation Sync process (EngineDocSync): every "
                       + "doc claim the change invalidates is fixed in the same commit.";
        :>> owner = Owner::either;
    }
}
