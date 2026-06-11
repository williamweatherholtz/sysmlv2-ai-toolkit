// =====================================================================
// EngineVerification — the verification PROCESS vs. the RESULT RECORD.
//
// schemaAudit 2026-06-10 (research-backed): a test IS a verification, so the
// PROCESS is a native `verification def` (the reusable check/spec — runs 1-to-many).
// The RESULT is a SEPARATE engine record (TestResult) — never folded into the def,
// because a verdict is a property of ONE performance against ONE version, not of
// the reusable spec. "Current pass/fail status" is therefore a COMPUTED VIEW over
// the latest result + git ancestry (what query.py does), never a stored field.
// Native enums: VerdictKind = {pass, fail, inconclusive, error};
// VerificationMethodKind = {inspect, analyze, demo, test} — engine ADDS
// `confirmation` (human sign-off). Closed sets are enum-typed (EngineElement).
// =====================================================================
package EngineVerification {
    private import ScalarValues::*;
    private import EngineElement::*;

    // Test — the reusable verification PROCESS (native verification def). What it
    // checks is linked via the engine #Verify edge; `method` implies who runs it
    // (test/analyze -> automatable; inspect/demo/confirmation -> human).
    verification def Test :> TrackedVerification {
        attribute method : VerificationMethod;
        attribute procedureText : String;   // how to carry it out
        attribute location : String[0..1];   // path to an automated test, if any
    }

    // TestPlan — a suite: a verification that composes member verifications
    // (native grouping idiom; SysML has no dedicated suite type).
    verification def TestPlan :> TrackedVerification;

    // TestResult — the engine's atomic immutable RECORD of ONE execution,
    // appended never overwritten, SEPARATE from the verification process.
    // judgedAgainst is a git commit SHA: happened-before is computed via git
    // ANCESTRY (wall-clock can't order divergent branches). See the contract.
    part def TestResult :> Element {
        attribute outcome : VerdictKind;
        attribute judgedAt : Timestamp;     // display only
        attribute judgedAgainst : String;   // git commit SHA (causal truth)
        attribute judgedBy : String;        // ci | actor id
        attribute notes : String[0..1];
    }
}
