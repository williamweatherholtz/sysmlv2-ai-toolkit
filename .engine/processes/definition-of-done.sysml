// =====================================================================
// Engine::Processes::DefinitionOfDone
// The gate a Story passes before currentState -> done.
// GateChecks are queryable completeness rules (engine-evaluated).
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Processes {
    private import Engine::Core::*;

    part dod : Gate {
        :>> name = "Definition of Done";
        :>> appliesWhen = "Before setting a Story to a terminal state.";
    }

    part dodMerged : GateCheck {
        :>> name = "Merged";
        :>> rule = "Change is merged to main / PR approved.";
        :>> order = 1;
        :>> owner = "ai";
    }
    part dodVerified : GateCheck {
        :>> name = "Satisfaction == verified";
        :>> rule = "Computed satisfaction(Story) == verified: every "
                + "verifying Test has a covering pass (commit ancestry), "
                + "transitively. No suspect/failing/uncovered.";
        :>> order = 2;
        :>> owner = "ai";
    }
    part dodStaleness : GateCheck {
        :>> name = "Downstream staleness cleared";
        :>> rule = "Items this Story's changes put into `suspect` are "
                + "resolved (re-verified) or have follow-up Stories.";
        :>> order = 3;
        :>> owner = "either";
    }
    part dodDocs : GateCheck {
        :>> name = "Docs in sync";
        :>> rule = "If user-facing behavior changed, docs updated in the "
                + "same PR.";
        :>> order = 4;
        :>> owner = "either";
    }

}
}
