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

    part issue002 : Issue {
        :>> id = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";
        :>> title = "status dashboard lacks process traceability view — no visibility into which processes sprints used, subversions, or evolution";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The dashboard shows workflow diagrams but does not trace which processes each sprint actually followed, when/how processes were subverted, how processes evolved over time via improvement items, or which backlog items link to which decisions and skill changes. User request: full system traceability — sprint-to-process linkage, process evolution timeline, item-to-decision trace, subversion highlighting.";
        :>> discoveredInField = true;
        :>> relatedTask = "baselineView";
    }

    part issue003 : Issue {
        :>> id = "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e";
        :>> title = "EngineWork::Release and EngineBaseline::Release are two conflicting Release types";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeSonnet46";
        :>> description = "EngineWork has Release :> Element with membershipRule : String. EngineBaseline has Release :> Element (occurrence def) with commit + purpose. When both are in scope (both imported), type resolution is ambiguous and the pilot kernel picks EngineWork::Release. tracking-template.sysml omits the EngineBaseline import until these are consolidated. Fix: either merge EngineBaseline::Release into EngineWork (drop membershipRule, add commit+purpose) or rename one to avoid conflict.";
        :>> discoveredInField = false;
        :>> relatedTask = "trackedMetadata";
    }
}
