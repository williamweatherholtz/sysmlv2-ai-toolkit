// =====================================================================
// EngineArtifacts — the item types that flow through the workflows.
//
// RE-GROUNDED on native SysML v2 (2026-06-09): a workflow IS a behavior, so a
// workflow is a native `action def`; phases are nested `action`s; order is
// `succession` (first/then); artifact handoff is item `flow`; parallelism is
// convergent successions. There is NO custom Workflow/Phase/produces/consumes
// meta-model — that reinvented SysML's native activity model (which is more
// comprehensive and `%show`-readable: succession renders as a discrete
// earlier/laterOccurrence element, not an opaque tuple).
//
// These `item def`s are the artifacts produced/consumed. They are intermediate:
// they will later align with the real schema types (Need -> requirement def,
// UseCase -> use case def, TestResult -> verification, ...). `#View` marks a
// computed (never-authored) artifact.
// =====================================================================
package EngineArtifacts {
    private import ScalarValues::*;

    item def Brief;
    item def Persona;
    item def Need;
    item def UseCase;
    item def SystemRequirement;
    item def SubsystemRequirement;
    item def ComponentRequirement;
    item def Component;
    item def DesignElement;
    item def Decision;
    item def ICD;                 // view
    item def AllocationMatrix;    // view
    item def Story;
    item def TestResult;
    item def Release;
    item def Baseline;            // view
    item def SafetyCase;          // view
    item def ReleaseNotes;        // view
    item def Issue;
    item def ChangeRequest;

    metadata def View;            // marks an artifact whose nature is computed, not authored

    // Acceptance criterion / Definition of Done for a task (verification model, option A).
    // `method`: test | analysis | inspection | demonstration | confirmation
    // (confirmation = a human sign-off — the common, lightweight path). A task
    // `satisfy`s its AcceptanceCriterion; "done" = a recorded pass result (commit SHA).
    requirement def AcceptanceCriterion {
        attribute method : String;
        attribute statement : String;
        // The recorded pass RESULT. Present => done; the commit it passed against
        // (enables git-ancestry suspicion). Absent => outstanding.
        attribute verifiedAtCommit : String[0..1];
        attribute verifiedBy : String[0..1];
        attribute verifiedAt : String[0..1];        // ISO-8601 attestation time (own fact, not the commit date)
    }
}
