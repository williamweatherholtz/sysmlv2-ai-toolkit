// =====================================================================
// EngineDoR — Definition of Ready: the Standup gate, run against each upcoming item
// before work starts. Antagonistic by design: fail -> resolve or invoke grill-me.
//
// instanceMigration 2026-06-11: Gate/GateCheck are GONE (CLAUDE.md §2.7). DoR is a
// `Process` whose checks are ordered `ProcessStep`s (actionText = the rule). Pass/fail
// is a computed view, never authored. Kind-specific checks are noted in actionText.
// =====================================================================
package EngineDoR {
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;

    action dor : Process {
        :>> id = "5ae3bc44-7af3-42d7-bda1-cc7c4ca3eeef";
        :>> title = "Definition of Ready";
        :>> purpose = "Standup gate, before starting a ~5-point batch: each check below passes.";
        :>> cadence = Cadence::eventDriven;
    }

    // -- Universal checks (all work items) --
    action dorClarity : ProcessStep {
        :>> id = "4661c889-1e3d-4e46-9503-0d458d60addd";
        :>> title = "Goal & boundary clear";
        :>> actionText = "[judgment] Title and intent are unambiguous; scope boundaries stated.";
        :>> owner = Owner::either;
    }
    action dorTests : ProcessStep {
        :>> id = "4aba2383-2c35-47ea-a173-db53762a95b1";
        :>> title = "Acceptance criteria exist as Tests";
        :>> actionText = "[computed: keel item] The task has a method-tagged DoD criterion; no bare text blobs.";
        :>> owner = Owner::ai;
    }
    action dorDeps : ProcessStep {
        :>> id = "f5d1d708-7112-46da-ba9c-36bff889b100";
        :>> title = "Dependencies linked";
        :>> actionText = "[judgment] Upstream successions authored; no unstated prerequisites.";
        :>> owner = Owner::ai;
    }
    // CONSUMER-FIRST (D0144). Added after three instances in one pass where a PRODUCER was built and
    // the consumer turned out to be unable to read it: flow edges emitted into a Model that does not
    // ingest `.engine/workflows` (issue104); the schema retyped before the parser could read
    // `action def X :> Y`; and a 6132-value provenance migration whose reference target lives in a
    // package `.engine` may not import (issue106). Each was cheap to check and expensive to discover.
    action dorConsumer : ProcessStep {
        :>> id = "7f3c1e08-5a24-4bd6-9e17-0c86b2495df3";
        :>> title = "The consumer can actually consume it";
        :>> actionText = "[judgment] If this item PRODUCES anything another component must read — an edge, "
                       + "an attribute, a file, an emitted artifact, a new construct — name that consumer "
                       + "and confirm it can read the thing as specified, BEFORE building. Check the "
                       + "direction of any dependency the change would create: a producer in `.engine` "
                       + "may not depend on `.tracking`. Where a computed proxy exists, use it: "
                       + "`keel guard parser-coverage` says whether the parser reads a construct, and the "
                       + "dead-schema rule (rules.sysml) says whether anything instantiates a type.";
        :>> owner = Owner::either;
    }

    action dorSized : ProcessStep {
        :>> id = "99661c95-ff8f-413d-a367-79d74b1dedd3";
        :>> title = "Sized";
        :>> actionText = "[judgment] Sized (optional for backlog tasks during bootstrap).";
        :>> owner = Owner::human;
    }

    // -- kind-specific (stricter) --
    action dorCodeIO : ProcessStep {
        :>> id = "3875e6ef-7dda-42f6-b5f2-9d6c68b583fc";
        :>> title = "I/O & example defined (kind=code)";
        :>> actionText = "[judgment] For kind=code: inputs/outputs specified; example input->output "
                       + "present; test method per Test set.";
        :>> owner = Owner::either;
    }
    action dorResearchQ : ProcessStep {
        :>> id = "4a173954-cd94-436c-b216-77e972adac81";
        :>> title = "Question & success criterion (kind=research)";
        :>> actionText = "[judgment] For kind=research: the question is crystallized; success = "
                       + "'we know X' / 'we have evidence of Y'; time-box set.";
        :>> owner = Owner::either;
    }
    action dorDecision : ProcessStep {
        :>> id = "0cf86114-a9af-4216-aa48-cf260791de7b";
        :>> title = "Options & criteria (kind=decision)";
        :>> actionText = "[judgment] For kind=decision: options enumerated; selection criteria stated; "
                       + "consequences considered.";
        :>> owner = Owner::either;
    }

    // -- Escalation --
    action dorEscalate : ProcessStep {
        :>> id = "34c92aae-34a1-4ffb-a1b8-596e992238e3";
        :>> title = "Resolve or grill-me";
        :>> actionText = "[judgment] Any failing check above is resolved, OR the grill-me skill is "
                       + "invoked to drive it to resolution before work starts.";
        :>> owner = Owner::either;
    }

    // dor: step order, native succession (D0143 — was `order : Integer`)
    first dorClarity then dorTests;
    first dorTests then dorDeps;
    first dorDeps then dorConsumer;
    first dorConsumer then dorSized;
    first dorSized then dorCodeIO;
    first dorCodeIO then dorResearchQ;
    first dorResearchQ then dorDecision;
    first dorDecision then dorEscalate;
}
