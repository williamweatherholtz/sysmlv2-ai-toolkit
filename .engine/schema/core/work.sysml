// =====================================================================
// Engine::Core::Work
// Work item hierarchy. The "things you do" — distinct from the SE
// artifacts (requirements, tests) they derive from.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // WorkItem. All @Tracked, so id/timestamps/authoredBy/currentState
    // come from the Tracked metadata. `currentState` references a
    // WorkflowState by name (workflow is modular — see workflow.sysml).
    //
    // `kind` classifies HOW the work gets done. It applies ONLY to
    // work items — a requirement has no kind, it is a requirement.
    //   "code"          programmatic implementation
    //   "research"      investigation; output is a finding (subsumes spike)
    //   "documentation" prose deliverable
    //   "refactor"      behavior-preserving change
    //   "decision"      producing a Decision record
    //   "modeling"      producing SysMLv2 model content (a deliverable model)
    // -----------------------------------------------------------------
    part def WorkItem {
        // @Tracked
        attribute title : String;
        attribute kind : String;
        attribute priority : String;      // "p0" | "p1" | "p2" | "p3"
        attribute owner : String;         // Actor id
    }

    part def Epic :> WorkItem {
        doc /* A large body of work decomposed into Stories.
               Aggregate progress is COMPUTED from children, not set. */
        attribute themeArea : String;
    }

    part def Story :> WorkItem {
        doc /* A sized unit of value. Acceptance criteria are NOT a
               text blob here — each criterion is an atomic Test linked
               by `verify` (see decision 0004, verification.sysml). */
        attribute estimatedPoints : Integer;
        attribute actualPoints : Integer;
    }

    part def Task :> WorkItem {
        doc /* A subtask of a Story. */
        attribute estimatedHours : Real;
    }

    // -----------------------------------------------------------------
    // Decision (ADR). Immutable once accepted; replaced via the
    // `supersede` edge, not by editing. The back-reference is computed.
    // -----------------------------------------------------------------
    part def Decision {
        // @Tracked
        attribute title : String;
        attribute status : String;        // "proposed" | "accepted" | "superseded"
        attribute context : String;
        attribute decision : String;
        attribute consequences : String;
    }

}
}
