// =====================================================================
// EngineArtifacts — workflow-local artifact types ONLY.
//
// CR-2 (2026-06-11): the shadow item defs that duplicated schema/core types
// (Brief/Need/UseCase/Decision/Story/TestResult/...) are RETIRED — schema/core
// is the canonical vocabulary and workflow pins/flows are typed by it directly
// (spiked: part-def and requirement-def pins + flows parse).
//
// What remains here:
//  - COMPUTED-VIEW artifacts (#View): outputs a workflow names but no one
//    authors — regenerated from authored facts + git (CLAUDE.md §2.1/§2.6).
//  - AcceptanceCriterion: the backlog DoD dialect (INTERIM — Decision 0012
//    records it as debt; superseded by appended TestResults when CR-3 lands).
// =====================================================================
package EngineArtifacts {
    private import ScalarValues::*;

    metadata def View;            // marks an artifact whose nature is computed, not authored

    #View item def ICD;               // computed from ports/connections
    #View item def AllocationMatrix;  // computed from allocate edges
    #View item def Baseline;          // computed from Release + commit match
    #View item def SafetyCase;        // computed (curated view; see parked critique findings)
    #View item def ReleaseNotes;      // computed from Release membership

    // INTERIM backlog DoD dialect (verification model: Decision 0012; replaced by CR-3).
    requirement def AcceptanceCriterion {
        attribute method : String;
        attribute statement : String;
        attribute verifiedAtCommit : String[0..1];
        attribute verifiedBy : String[0..1];
        attribute verifiedAt : String[0..1];        // ISO-8601 attestation time (own fact, not the commit date)
    }
}
