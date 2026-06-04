// =====================================================================
// Engine::Core::Requirements
// Requirement taxonomy via specialization, plus Constraint & Assumption.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // Requirement base. Native SysMLv2 `requirement def`. All @Tracked.
    // Subtypes are specializations (`:>`). Instance-level derivation
    // ALSO uses `:>` (a child requirement specializes its parent) —
    // this is the idiomatic v2 replacement for v1 deriveReqt.
    // -----------------------------------------------------------------
    requirement def Requirement {
        // @Tracked
        doc /* A required condition or capability. */
        attribute rationale : String;
    }

    // Intent layer ----------------------------------------------------
    requirement def MarketRequirement :> Requirement {
        doc /* Top-level intent: how we differentiate in the market /
               why this project exists. Validated (verify against)
               rather than satisfied. The root of derivation chains. */
    }

    // Behavioral / quality layer --------------------------------------
    requirement def FunctionalRequirement :> Requirement {
        doc /* What the system must DO. */
    }

    requirement def NonFunctionalRequirement :> Requirement {
        doc /* Quality attribute: performance, reliability, usability,
               etc. Often carries a measure (the verifying Test holds
               the threshold). */
        attribute measure : String;       // e.g. "p95 latency < 200ms"
    }

    requirement def SafetyRequirement :> Requirement {
        doc /* Derived from STPA safety constraints (system-level or
               controller-level). NO asil attribute — HARA/ASIL are
               intentionally out of scope (see decision 0008). */
    }

    requirement def SecurityRequirement :> Requirement {
        doc /* Resistance to adversarial compromise. Distinct from
               safety (accident) and from functional. */
    }

    requirement def InterfaceRequirement :> Requirement {
        doc /* How the system talks to another system. Backbone of
               surgical dependency-swap analysis. */
    }

    // -----------------------------------------------------------------
    // Constraint — a restriction on the SOLUTION SPACE, not a behavior.
    // (e.g. "must run on Windows", "must use Rust", budget caps.)
    // Different from a requirement: it bounds HOW you may satisfy one.
    // -----------------------------------------------------------------
    requirement def Constraint :> Requirement {
        attribute category : String;      // "platform" | "technology" | "budget" | "regulatory" | "schedule"
    }

    // -----------------------------------------------------------------
    // Assumption — taken as true without proof. First-class because
    // when an assumption BREAKS, everything derived from it goes
    // suspect. Highest-value input to the suspicion engine.
    // -----------------------------------------------------------------
    part def Assumption {
        // @Tracked
        attribute statement : String;
        attribute confidence : String;    // "high" | "medium" | "low"
        attribute validUntil : String;    // optional review date
    }

}
}
