// =====================================================================
// EngineState — modular workflow states. States are DATA, not a hardcoded
// enum: a project can add a "deployment" or "ai-review" state without
// touching engine code (decision 0009). An element's current state is a
// COMPUTED view that resolves to a WorkflowState by name (never authored).
//
// Gate / GateCheck now live in the process-as-data meta-model
// (.engine/workflows/_meta.sysml), not here.
// =====================================================================
package EngineState {
    private import ScalarValues::*;

    part def WorkflowState {
        attribute name : String;          // display name / key (computed state views resolve to this)
        attribute order : Integer;        // board column ordering
        attribute isTerminal : Boolean;   // true for done-like states
        attribute description : String;
    }

    part def WorkflowDefinition {
        attribute name : String;
        attribute states : String[*];      // names of WorkflowStates
        attribute transitions : String[*]; // "from->to" pairs, as strings
    }
}
