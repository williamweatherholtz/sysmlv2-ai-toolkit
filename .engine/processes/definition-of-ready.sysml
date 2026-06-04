// =====================================================================
// Engine::Processes::DefinitionOfReady
// The Standup gate. Run against each upcoming item before work starts.
// Antagonistic by design: fail -> resolve or invoke grill-me.
// GateChecks are queryable completeness rules (engine-evaluated).
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Processes {
    private import Engine::Core::*;

    part dor : Gate {
        :>> name = "Definition of Ready";
        :>> appliesWhen = "Standup, before starting a ~5-point batch.";
    }

    // -- Universal checks (all work items) ----------------------------
    part dorClarity : GateCheck {
        :>> name = "Goal & boundary clear";
        :>> rule = "Story.title and intent are unambiguous; scope "
                + "boundaries stated.";
        :>> order = 1;
        :>> owner = "either";
    }
    part dorTests : GateCheck {
        :>> name = "Acceptance criteria exist as Tests";
        :>> rule = "Story has >=1 Test linked by `verify`; none are bare "
                + "text blobs.";
        :>> order = 2;
        :>> owner = "ai";
    }
    part dorDeps : GateCheck {
        :>> name = "Dependencies linked";
        :>> rule = "Upstream items linked; no unstated prerequisites.";
        :>> order = 3;
        :>> owner = "ai";
    }
    part dorSized : GateCheck {
        :>> name = "Sized";
        :>> rule = "Story.estimatedPoints is set.";
        :>> order = 4;
        :>> owner = "human";
    }

    // -- kind == "code": stricter checks ------------------------------
    part dorCodeIO : GateCheck {
        :>> name = "I/O & example defined";
        :>> rule = "For kind=code: inputs/outputs specified; example "
                + "input->output present; test method per Test set.";
        :>> order = 5;
        :>> owner = "either";
    }

    // -- kind == "research": ------------------------------------------
    part dorResearchQ : GateCheck {
        :>> name = "Question & success criterion";
        :>> rule = "For kind=research: the question is crystallized; "
                + "success = 'we know X' / 'we have evidence of Y'; "
                + "time-box set.";
        :>> order = 6;
        :>> owner = "either";
    }

    // -- kind == "decision": ------------------------------------------
    part dorDecision : GateCheck {
        :>> name = "Options & criteria";
        :>> rule = "For kind=decision: options enumerated; selection "
                + "criteria stated; consequences considered.";
        :>> order = 7;
        :>> owner = "either";
    }

    // -- Escalation ---------------------------------------------------
    part dorEscalate : GateCheck {
        :>> name = "Resolve or grill-me";
        :>> rule = "Any failing check above is resolved, OR the grill-me "
                + "skill is invoked to drive it to resolution before "
                + "work starts.";
        :>> order = 99;
        :>> owner = "either";
    }

}
}
