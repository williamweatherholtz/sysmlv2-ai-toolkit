// =====================================================================
// EngineState — modular workflow states. States are DATA, not a hardcoded
// enum: a project can add a "deployment" or "ai-review" state without
// touching engine code (decision 0009). An element's current state is a
// COMPUTED view that resolves to a WorkflowState by name (never authored).
//
// (Gate/GateCheck were REMOVED outright — a gate is its verify-linked Tests
// passing, CLAUDE.md §2.6, not an authored type.)
// =====================================================================
package EngineState {
    doc /* Modular workflow states as data (WorkflowState/WorkflowDefinition) and the StateCursor pointer that orient reads. Element state is a computed view, never authored. */

    private import ScalarValues::*;

    part def WorkflowState {
        attribute name : String;          // display name / key (computed state views resolve to this)
        attribute order : Integer;        // board column ordering
        attribute isTerminal : Boolean;   // true for done-like states
        attribute description : String;
    }

    part def WorkflowDefinition {
        attribute name : String;
        ref states : WorkflowState[*];     // typed refs (M4)
        attribute transitions : String[*]; // "from->to" strings (native state-def spike deferred)
    }

    // The state CURSOR (CR-6): where a project currently stands in the workflows.
    // CLAUDE.md §3b/§3f orient on this. It is an authored POINTER (like git HEAD):
    // updating it in place is sanctioned — its history IS the git history; entering
    // a phase is a recorded judgment (who/when). One instance lives in
    // .tracking/state.sysml. "BOOTSTRAP" is a sanctioned activeWorkflow value (§3e).
    part def StateCursor {
        attribute activeWorkflow : String;   // e.g. "BusinessWorkflow::Business" | "BOOTSTRAP"
        attribute activePhase : String;      // e.g. "brief" | the active backlog during bootstrap
        attribute enteredAt : String;        // ISO-8601
        attribute enteredBy : String;        // actor id
    }
}
