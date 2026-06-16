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
    doc /* Modular workflow states as data (WorkflowState/WorkflowDefinition). Element state is a computed view, never authored. StateCursor retired by D0045 — orient now computes sprint ceremony status from delivery file TestResults. */

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

    // StateCursor RETIRED by D0045 (2026-06-16). orient now computes sprint ceremony
    // status from delivery file TestResults — no hand-maintained pointer needed.
}
