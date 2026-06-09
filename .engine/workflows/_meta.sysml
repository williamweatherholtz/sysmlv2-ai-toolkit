// =====================================================================
// EngineWorkflow — the process-as-data meta-model.
//
// Workflows, phases, gates, artifact types, and critic bindings are
// DATA the engine reads to drive itself (see CLAUDE.md and the design
// spec docs/superpowers/specs/2026-06-04-process-model-design.md §14).
// Concrete workflows (Business, Architecture, ...) are INSTANCES of
// these types, authored under .engine/workflows/.
//
// Self-contained during bootstrap: carries a local MetaElement base
// (id/title) instead of importing EngineElement, so it validates
// standalone before the core-schema rewrite lands. `procedureRef` is a
// String (a qualified name) for the same reason — it becomes a typed
// ref to an Engine Process once core parses.
//
// SYNTAX confirmed via .engine/tools/validate/spike_metamodel.py.
// =====================================================================
package EngineWorkflow {
    private import ScalarValues::*;

    // Local base for tracked meta-model elements. Unifies with
    // EngineElement::Element (id/title/createdAt/...) after the core rewrite.
    abstract part def MetaElement {
        attribute id : String;        // UUID — immutable identity (names never collide)
        attribute title : String;     // authored human label
    }

    // A single exit/entry condition. Whether it HOLDS is a computed view.
    part def GateCheck :> MetaElement {
        attribute criterion : String;     // the condition that must hold
    }

    // A promotion gate between phases.
    part def Gate :> MetaElement {
        attribute kind : String;          // "entry" | "exit"
        ref checks : GateCheck[*];
    }

    // The kind of item a phase produces, and whether it is authored or computed.
    part def ArtifactType :> MetaElement {
        attribute schemaType : String;            // schema type produced, e.g. "Need", "Persona"
        attribute nature : String;                // "real" | "view"
        attribute derivation : String[0..1];      // set iff nature == "view": how it is computed
    }

    // Binds a critic to a phase, with the severity at/above which it must run.
    part def CriticBinding :> MetaElement {
        attribute critic : String;                // "premortem"|"devils-advocate"|"stakeholder"|"traceability"
        attribute severityThreshold : String;     // skip critics below this severity
    }

    // One ordered step of a workflow.
    part def Phase :> MetaElement {
        attribute order : Integer;
        ref produces : ArtifactType[*];
        ref entryGate : Gate[0..1];
        ref exitGate : Gate[0..1];
        ref critics : CriticBinding[*];
        attribute procedureRef : String[0..1];    // qualified name of an Engine Process (typed ref once core parses)
    }

    // A named workflow: an ordered sequence of phases with a cadence.
    part def Workflow :> MetaElement {
        attribute purpose : String;
        attribute cadence : String;                // "once"|"continuous"|"event-driven"|"per-item"|"per-release"
        ref phases : Phase[*] ordered;
    }
}
