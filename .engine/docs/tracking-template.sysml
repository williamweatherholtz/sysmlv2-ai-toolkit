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
    private import EngineVerification::*; // Test (criterion) + TestResult (appended record)

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

    // -- work-item backlog dialect v2 (read by query.py; CR-3) --
    // An `action def` in .tracking IS a backlog; tasks = actions; deps = succession.
    // Each task's DoD = a method-tagged `verification <task>DoD : Test` criterion.
    // Results are APPENDED, immutable `part <task>R<n> : TestResult` records —
    // re-verification appends R2, NEVER edits R1. Done = the latest result is a pass.
    // TEXTUAL CONTRACT: <task>DoD / <task>R<n> naming, authored ON ONE LINE each.
    action def ExampleBacklog {
        action firstTask;
        action secondTask;
        first firstTask then secondTask;

        verification firstTaskDoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "what done means"; }
        verification secondTaskDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "what done means"; }

        // appended when firstTask passed (judgedAgainst = the commit it was judged at):
        part firstTaskR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abc1234"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
    }
}
