// =====================================================================
// EngineWorkflow — the process-as-data meta-model (simplified).
//
// Workflows and phases are DATA the engine reads to drive itself (CLAUDE.md;
// design spec §14). Concrete workflows (Business, Architecture, ...) are
// INSTANCES of these types under .engine/workflows/.
//
// DELIBERATELY MINIMAL (see the 2026-06-09 design pass):
//   * A Phase declares only what it PRODUCES and CONSUMES, plus a reference
//     to its inner procedure. Sequence and parallelism are NOT authored —
//     they are COMPUTED from the consumes/produces flow + the standard typed
//     edges (satisfy/verify/allocate/:>/dependency): the dependency DAG.
//   * There is no Gate / GateCheck / CriticBinding type. A phase's "gate"
//     (its Definition of Done) is COMPUTED: the verify-linked Tests on its
//     output must pass. Critics are just Tests with method = "inspection".
//     (Test is the universal verifiable condition — EngineVerification +
//     decisions D0004/D0006.)
//   * ready / blocked / done / needs-review are COMPUTED views, never authored.
//
// Self-contained during bootstrap (local MetaElement base; procedureRef as a
// String). SYNTAX confirmed via .engine/tools/validate/validate_workflows.py.
// =====================================================================
package EngineWorkflow {
    private import ScalarValues::*;

    abstract part def MetaElement {
        attribute id : String;        // UUID — immutable identity (names never collide)
        attribute title : String;     // authored human label
    }

    // The kind of item a phase produces/consumes, and whether it is authored
    // or computed.
    part def ArtifactType :> MetaElement {
        attribute schemaType : String;            // schema type, e.g. "Need", "Persona"
        attribute nature : String;                // "real" | "view"
        attribute derivation : String[0..1];      // set iff nature == "view"
    }

    // One step of a workflow. Order/parallelism and DoD are computed (see header).
    part def Phase :> MetaElement {
        ref produces : ArtifactType[*];
        ref consumes : ArtifactType[*];            // drives the dependency DAG
        attribute procedureRef : String[0..1];     // qualified name of an Engine Process
    }

    // A named workflow: a set of phases with a cadence. Phases are an
    // unordered membership set — execution order is computed from the DAG.
    part def Workflow :> MetaElement {
        attribute purpose : String;
        attribute cadence : String;                // "once"|"continuous"|"event-driven"|"per-item"|"per-release"
        ref phases : Phase[*];
    }
}
