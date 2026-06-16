// =====================================================================
// ProjectIssues — open issues tracked against the engine.
// Convention: author a `part <name> : Issue { ... }` for each open
// issue. The `relatedTask` field names the backlog action that should
// address it. `discoveredInField = true` for issues found in production use.
// Surface with: python .engine/tools/query.py issues
// =====================================================================
package ProjectIssues {
    private import EngineElement::*;
    private import EngineWork::*;

    part issue001 : Issue {
        :>> id = "f26d7e8f-a9b0-4c1d-daaa-3f4a5b6c7d8e";
        :>> title = "authoredBy/judgedBy are untyped strings, not validated actor refs";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "claudeSonnet46";
        :>> description = "The authoredBy and judgedBy fields in Element and TestResult are raw String attributes. They name actors by convention but are not validated against the ProjectActors registry, so typos or stale names are silently accepted.";
        :>> discoveredInField = false;
        :>> relatedTask = "authoredByRefs";
    }
}
