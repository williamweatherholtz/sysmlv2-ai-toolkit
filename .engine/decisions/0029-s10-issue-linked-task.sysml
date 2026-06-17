package Decision0029 {
    private import EngineWork::*;

    part d0029 : Decision {
        :>> id = "e15c6d7e-f8a9-4b0c-c9b9-2e3f4a5b6c7d";
        :>> title = "S10 issueLoop: add relatedTask attribute to Issue for typed backlog routing";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 10 (issueLoop): the `issueLoop` feature adds a `query.py issues` subcommand and `.tracking/issues.sysml` convention. The Issue type in EngineWork has `description` and `discoveredInField` but no way to point to the backlog task that should address it. Without a typed routing field, the link is only implicit in the description text.";
        :>> decision = "Add `relatedTask : String[0..1]` to `Issue` in EngineWork, naming the backlog task (by its action name) that should address the issue; the `query.py issues` subcommand reads and surfaces this field.";
        :>> rationale = "Without a typed routing field the link from issue to backlog task is only implicit in description text. Optional (0..1) because some issues are standalone observations. Kept as String (not a typed ref) — typed refs are deferred to the `authoredByRefs` task. Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "Issue instances can now carry an explicit backlog pointer. The field is String (not a typed ref) — typed refs are deferred to the `authoredByRefs` task. validate_schema green.";
    }
}
