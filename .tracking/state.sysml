// =====================================================================
// ProjectState — THE state cursor (CR-6). Where this project stands.
// An authored pointer (like git HEAD): updated in place; history = git log.
// query.py `orient` merges this with the computed ready/suspect frontier.
// =====================================================================
package ProjectState {
    private import EngineState::*;

    part cursor : StateCursor {
        :>> activeWorkflow = "ArchitectureWorkflow";
        :>> activePhase = "Architecture/allocIface";
        :>> enteredAt = "2026-06-11";
        :>> enteredBy = "claudeOpus";
    }
}
