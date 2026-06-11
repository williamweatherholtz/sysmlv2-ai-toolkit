// =====================================================================
// EngineDoR — Definition of Ready: the Standup gate, run against each upcoming item
// before work starts. Antagonistic by design: fail -> resolve or invoke grill-me.
//
// instanceMigration 2026-06-11: Gate/GateCheck are GONE (CLAUDE.md §2.7). DoR is a
// `Process` whose checks are ordered `ProcessStep`s (actionText = the rule). Pass/fail
// is a computed view, never authored. Kind-specific checks are noted in actionText.
// =====================================================================
package EngineDoR {
    private import EngineProcess::*;

    part dor : Process {
        :>> title = "Definition of Ready";
        :>> purpose = "Standup gate, before starting a ~5-point batch: each check below passes.";
        :>> cadence = "event-driven";
    }

    // -- Universal checks (all work items) --
    part dorClarity : ProcessStep {
        :>> title = "Goal & boundary clear";
        :>> order = 1;
        :>> actionText = "Story.title and intent are unambiguous; scope boundaries stated.";
        :>> owner = "either";
    }
    part dorTests : ProcessStep {
        :>> title = "Acceptance criteria exist as Tests";
        :>> order = 2;
        :>> actionText = "Story has >=1 Test linked by `verify`; none are bare text blobs.";
        :>> owner = "ai";
    }
    part dorDeps : ProcessStep {
        :>> title = "Dependencies linked";
        :>> order = 3;
        :>> actionText = "Upstream items linked; no unstated prerequisites.";
        :>> owner = "ai";
    }
    part dorSized : ProcessStep {
        :>> title = "Sized";
        :>> order = 4;
        :>> actionText = "Story.estimatedPoints is set.";
        :>> owner = "human";
    }

    // -- kind-specific (stricter) --
    part dorCodeIO : ProcessStep {
        :>> title = "I/O & example defined (kind=code)";
        :>> order = 5;
        :>> actionText = "For kind=code: inputs/outputs specified; example input->output "
                       + "present; test method per Test set.";
        :>> owner = "either";
    }
    part dorResearchQ : ProcessStep {
        :>> title = "Question & success criterion (kind=research)";
        :>> order = 6;
        :>> actionText = "For kind=research: the question is crystallized; success = "
                       + "'we know X' / 'we have evidence of Y'; time-box set.";
        :>> owner = "either";
    }
    part dorDecision : ProcessStep {
        :>> title = "Options & criteria (kind=decision)";
        :>> order = 7;
        :>> actionText = "For kind=decision: options enumerated; selection criteria stated; "
                       + "consequences considered.";
        :>> owner = "either";
    }

    // -- Escalation --
    part dorEscalate : ProcessStep {
        :>> title = "Resolve or grill-me";
        :>> order = 99;
        :>> actionText = "Any failing check above is resolved, OR the grill-me skill is "
                       + "invoked to drive it to resolution before work starts.";
        :>> owner = "either";
    }
}
