package Decision0030 {
    private import EngineWork::*;

    part d0030 : Decision {
        :>> id = "f37e8f9a-b0c1-4d2e-dabc-4a5b6c7d8e9f";
        :>> title = "S10 review doc-sync: query.py issues + Issue.relatedTask added to usage.md and tracking template";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 10 (issueLoop) added the `query.py issues` subcommand and `Issue.relatedTask : String[0..1]` field. The usage.md day-to-day table had no entry for issue tracking. The tracking-template.sysml had no Issue authoring example.";
        :>> decision = "Update usage.md day-to-day table: add a 'Track an issue' row documenting the Issue authoring convention and `query.py issues` command; add an Issue example to tracking-template.sysml showing all fields including relatedTask. Recorded as D0030 per doc-sync process.";
        :>> rationale = "After Sprint 10 issueLoop, usage.md had no issue-tracking entry and the tracking template had no Issue example, so the docs had to catch up to the new feature. Accepted under standing sprint review instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "usage.md and tracking-template.sysml now reflect Sprint 10 issueLoop feature. No schema or process semantics change.";
    }
}
