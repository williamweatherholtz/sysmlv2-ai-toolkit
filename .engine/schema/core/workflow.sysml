// =====================================================================
// Engine::Core::Workflow
// Modular workflow states + gates. States are DATA, not a hardcoded
// enum — a project can add a "deployment" or "ai-review" state without
// touching engine code (decision 0009).
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // WorkflowState — an instance, referenced by name from a tracked
    // element's Tracked.currentState. Define the states you need; the
    // engine does not hardcode them.
    // -----------------------------------------------------------------
    part def WorkflowState {
        attribute name : String;          // display name / key
        attribute order : Integer;        // for board column ordering
        attribute isTerminal : Boolean;   // true for done-like states
        attribute doc : String;
    }

    // -----------------------------------------------------------------
    // WorkflowDefinition — an ordered set of states with allowed
    // transitions. A project picks/defines one. Processes (processes/)
    // drive transitions; the engine just records currentState by name.
    // -----------------------------------------------------------------
    part def WorkflowDefinition {
        attribute name : String;
        attribute states : String[*];     // qualified names of WorkflowStates
        attribute transitions : String[*];// "from->to" pairs, as strings
    }

    // -----------------------------------------------------------------
    // Gate — a checkpoint a tracked element must pass (DoR, DoD,
    // Standup are INSTANCES of Gate; see processes/). A Gate is a set
    // of completeness rules evaluated by the query layer.
    //
    // A GateCheck is a rule: a queryable predicate over the model.
    // Examples:
    //   "every acceptance criterion has a linked Test"
    //   "every ControlAction has >=1 UCA assessment or assessed-safe record"
    // The engine provides the evaluation mechanism; the specific rules
    // are content.
    // -----------------------------------------------------------------
    part def Gate {
        // @Tracked
        attribute name : String;
        attribute appliesWhen : String;   // trigger condition
    }

    part def GateCheck {
        attribute name : String;
        attribute rule : String;          // queryable predicate (engine-evaluated)
        attribute order : Integer;
        attribute owner : String;         // "human" | "ai" | "either"
    }

}
}
