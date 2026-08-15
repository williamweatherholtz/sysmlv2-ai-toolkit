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
    // #View now lives in the declared marker home (D0142); import it rather than re-declaring,
    // which is the whole point of having one canonical home.
    private import EngineRelationships::*;

    // ICD: the Architecture workflow's declared interface-control output (architecture.sysml:19).
    // Its COMPUTATION is not implemented — the repo has no ports and no `connect` statements — but
    // the workflow legitimately declares the artifact it produces, so this is an unbuilt capability,
    // not dead schema. The old comment claimed it was 'computed from ports/connections', which was
    // a promise rather than a fact (D0142).
    #View item def ICD;
    #View item def AllocationMatrix;  // computed from allocate edges
    #View item def Baseline;          // computed from Release + commit match
    #View item def SafetyCase;        // computed (curated view; see parked critique findings)
    #View item def ReleaseNotes;      // computed from Release membership
}
