// =====================================================================
// EngineArtifacts — workflow-local artifact types ONLY.
//
// CR-2 (2026-06-11): the shadow item defs that duplicated schema/core types
// (Brief/Need/UseCase/Decision/Story/TestResult/...) are RETIRED — schema/core
// is the canonical vocabulary and workflow pins/flows are typed by it directly
// (spiked: part-def and requirement-def pins + flows parse).
//
// What remains here: COMPUTED-VIEW artifacts (#View) — outputs a workflow names
// but no one authors; regenerated from authored facts + git (CLAUDE.md §2.1/§2.6).
// (AcceptanceCriterion retired by CR-3: criteria are `verification ... : Test`
// usages; results are appended TestResult records — EngineVerification.)
// =====================================================================
package EngineArtifacts {
    metadata def View;            // marks an artifact whose nature is computed, not authored

    #View item def ICD;               // computed from ports/connections
    #View item def AllocationMatrix;  // computed from allocate edges
    #View item def Baseline;          // computed from Release + commit match
    #View item def SafetyCase;        // computed (curated view; see parked critique findings)
    #View item def ReleaseNotes;      // computed from Release membership
}
