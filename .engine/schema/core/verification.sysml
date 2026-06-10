// =====================================================================
// EngineVerification — atomic verification: TestPlan -> Test -> TestResult.
// No subjective/objective flag — the verification METHOD carries that.
// =====================================================================
package EngineVerification {
    private import ScalarValues::*;
    private import EngineElement::*;

    part def TestPlan :> Element;          // a suite; contains Tests

    // Test — an ATOMIC verification element, linked to what it checks via
    // `verify`. `method` implies whether a human is required:
    //   test/analysis -> automatable;  inspection/demonstration -> human.
    part def Test :> Element {
        attribute method : String;          // test | analysis | inspection | demonstration
        attribute procedureText : String;   // how to carry it out
        attribute location : String[0..1];   // path to an automated test, if any
    }

    // TestResult — an AUTHORED fact: one execution, appended never overwritten.
    // judgedAgainst is a git commit SHA: happened-before is computed via git
    // ANCESTRY (wall-clock can't order divergent branches). See the contract.
    part def TestResult :> Element {
        attribute outcome : String;         // pass | fail | skip
        attribute judgedAt : String;        // ISO 8601 (display only)
        attribute judgedAgainst : String;   // git commit SHA (causal truth)
        attribute judgedBy : String;        // ci | actor id
        attribute notes : String[0..1];
    }
}
