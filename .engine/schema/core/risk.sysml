// =====================================================================
// Engine::Core::Risk
// Risk / Mitigation / Issue. Tracked uncertainty and discovered
// failures. (Enterprise risk register concepts; NOT safety hazards —
// those are STPA, in schema/safety.)
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // Risk — a tracked uncertainty with impact. Reduced by a Mitigation
    // (linked via `dependency`). Residual risk is an attribute, not a
    // separate element. A materialized risk becomes an Issue.
    // -----------------------------------------------------------------
    part def Risk {
        // @Tracked
        attribute statement : String;
        attribute likelihood : String;    // "high" | "medium" | "low"
        attribute impact : String;        // "high" | "medium" | "low"
        attribute residual : String;      // remaining after mitigation
        attribute status : String;        // "open" | "mitigated" | "accepted" | "materialized"
    }

    part def Mitigation {
        // @Tracked
        doc /* An action/requirement that reduces a Risk. Links to its
               Risk via `dependency`; may itself BE a (Safety)Requirement
               satisfied by design + verified by Test. */
        attribute statement : String;
    }

    // -----------------------------------------------------------------
    // Issue — a discovered failure (from a failing TestResult or a
    // field report). Distinct from planned WorkItems: it ARISES rather
    // than being planned. Resolved by linking to a fixing WorkItem.
    // -----------------------------------------------------------------
    part def Issue {
        // @Tracked
        attribute title : String;
        attribute severity : String;      // "critical" | "major" | "minor"
        attribute status : String;        // "open" | "in_progress" | "resolved" | "wontfix"
        attribute discoveredVia : String; // "test" | "review" | "field" | ...
    }

}
}
