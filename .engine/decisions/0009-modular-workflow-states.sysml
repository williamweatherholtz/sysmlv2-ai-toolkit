package Decision0009 {
    private import EngineWork::*;

    part d0009 : Decision {
        :>> id = "9e678ed3-de32-4af2-9cfb-a4c3d25e3e16";
        :>> title = "Workflow states are data, not a hardcoded enum";
        :>> status = DecisionStatus::accepted;
        :>> context = "Hardcoding backlog/ready/in_progress prevents adding a "
            + "deployment or ai-review state without code changes.";
        :>> decision = "WorkflowState is an instance type; a tracked element references its currentState BY NAME; a project defines the states and transitions it needs (a WorkflowDefinition); processes drive transitions while the engine only records state; DoD/staleness reference states by name, never by literal.";
        :>> rationale = "Hardcoding backlog/ready/in_progress prevents adding a deployment or ai-review state without code changes; making states data keeps the workflow extensible per project without engine changes, at the cost that state validity must be checked against the WorkflowDefinition by the validator.";
        :>> consequences = "Pros: workflow is extensible per project without "
            + "engine changes. Cons: state validity must be checked against the "
            + "WorkflowDefinition by the validator.";
    }
}
