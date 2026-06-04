// =====================================================================
// Engine::Core::Verification
// Atomic verification: TestPlan -> Test -> TestResult.
// No subjective/objective flag — the verification METHOD carries that.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // TestPlan — a suite. Contains Tests (composition).
    // -----------------------------------------------------------------
    part def TestPlan {
        // @Tracked
        attribute name : String;
    }

    // -----------------------------------------------------------------
    // Test — an ATOMIC verification element. First-class, independently
    // queryable. Linked to what it checks via `verify`. A Test is not a
    // checklist line inside another item (decision 0004).
    //
    // `method` — the four classic V&V methods. Whether a human is
    // required FALLS OUT of the method; there is no separate flag:
    //   "test"          execute & compare       -> automatable
    //   "analysis"      reason / compute / model -> automatable
    //   "inspection"    examine artifact         -> human
    //   "demonstration" observe behavior in use  -> human
    // -----------------------------------------------------------------
    part def Test {
        // @Tracked
        attribute name : String;
        attribute method : String;
        attribute procedure : String;     // how to carry it out
        attribute location : String;      // path to automated test, if any
    }

    // -----------------------------------------------------------------
    // TestResult — an AUTHORED fact: the record of one execution.
    // Appended, never overwritten — the log is the evidence trail the
    // suspicion engine reads.
    //
    // `judgedAgainst` is a GIT COMMIT SHA, not just a timestamp:
    // happened-before is computed via git ANCESTRY, because wall-clock
    // timestamps on divergent branches don't establish causal order
    // (see contracts/computed-state.md). Timestamp is for display.
    // -----------------------------------------------------------------
    part def TestResult {
        // @Tracked
        attribute outcome : String;       // "pass" | "fail" | "skip"
        attribute judgedAt : String;      // ISO 8601 (display)
        attribute judgedAgainst : String; // git commit SHA (causal truth)
        attribute judgedBy : String;      // "ci" | actor id (human/ai)
        attribute notes : String;
    }

}
}
