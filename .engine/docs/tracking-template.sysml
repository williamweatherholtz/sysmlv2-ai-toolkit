// =====================================================================
// TRACKING TEMPLATE — the sanctioned authoring idiom for .tracking/ instances.
// Copy patterns from here into your project's .tracking/*.sysml files.
// This file itself parses green (validated by validate_instances).
//
// RULES (CLAUDE.md §2): schema/core types are the CANONICAL instance vocabulary.
// Author only irreducible facts + recorded judgments; status/coverage/trace are
// computed views. Every item carries an immutable :>> id (UUID).
// =====================================================================
package TrackingTemplate {
    private import EngineElement::*;     // Element bases, Priority, value types
    private import EngineNeeds::*;
    private import EngineWork::*;
    private import EngineArtifacts::*;   // AcceptanceCriterion (backlog dialect)

    // -- a Business-phase artifact: the project brief --
    part exampleBrief : Brief {
        :>> id = "3f1c9a2e-0000-4000-8000-000000000001";
        :>> title = "Example project brief";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> problem = "What hurts today.";
        :>> opportunity = "Why solving it is worth it.";
    }

    // -- a Need, traced to by later requirements via native `satisfy` --
    requirement exampleNeed : Need {
        :>> id = "3f1c9a2e-0000-4000-8000-000000000002";
        :>> title = "Example need";
        :>> source = NeedSource::customer;
        :>> statement = "The solution-free desired outcome.";
        :>> priority = Priority::must;
    }

    // -- a scope-cut Decision (captured even though it produces no action) --
    part exampleDecision : Decision {
        :>> id = "3f1c9a2e-0000-4000-8000-000000000003";
        :>> title = "Example: we will not pursue X";
        :>> status = "accepted";
        :>> context = "Why the question arose.";
        :>> decisionText = "What was decided.";
        :>> consequences = "What follows.";
    }

    // -- work-item backlog dialect (read by query.py whats-next) --
    // An `action def` in .tracking IS a backlog; tasks = actions; deps = succession;
    // each task satisfies a method-tagged DoD. NOTE the textual contract:
    // the DoD usage must be named <task>DoD and authored ON ONE LINE.
    action def ExampleBacklog {
        action firstTask;
        action secondTask;
        first firstTask then secondTask;

        requirement firstTaskDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "what done means"; }
        satisfy firstTaskDoD by firstTask;
        requirement secondTaskDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "what done means"; }
        satisfy secondTaskDoD by secondTask;
    }
}
