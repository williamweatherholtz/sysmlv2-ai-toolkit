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
    doc /* Verification PROCESS vs RESULT: Test/TestPlan are native verification defs (the reusable check); TestResult is the appended immutable record of ONE execution (outcome + judgedAgainst commit). Current status is always a computed view. */

    private import ScalarValues::*;
    private import EngineElement::*;

    // Test — the reusable verification PROCESS (native verification def). What it
    // checks is linked via the engine #Verify edge; `method` implies who runs it
    // (test/analyze -> automatable; inspect/demo/confirmation -> human).
    verification def Test :> TrackedVerification {
        attribute method : VerificationMethod;
        attribute procedureText : String;   // how to carry it out
        attribute location : String[0..1];   // path to an automated test, if any
        // Antagonistic critique (D0080): when method=critique, a Test is a lens-tagged
        // verification. lens = the adversarial angle; critiquedBy = the critic KIND (the critic
        // IDENTITY stays in the result's judgedBy, which must differ from the target's author —
        // independence). severity = the assessed severity (a raised finding becomes an Issue
        // carrying the same Severity; Low is surfaced not gated, >= Medium needs human disposition).
        attribute lens : CritiqueLens[0..1];
        attribute critiquedBy : CriticKind[0..1];
        attribute severity : Severity[0..1];
        // Finding disposition (D0092): when this Test is a method=confirmation DISPOSITION of a
        // >= Medium finding (#Dispositions-linked to the finding Issue), `disposition` carries the
        // human's verdict (act/acceptRisk/dismiss). The rationale lives in procedureText; provenance
        // (who/when/commit) in the result. Reuses the verification substrate so dispositions inherit
        // staleness (suspect -> re-disposition when the finding changes, D0080).
        attribute disposition : DispositionKind[0..1];
    }

    // TestPlan — a suite: a verification that composes member verifications
    // (native grouping idiom; SysML has no dedicated suite type).
    verification def TestPlan :> TrackedVerification;

    // TestResult — the engine's atomic immutable RECORD of ONE execution,
    // appended never overwritten, SEPARATE from the verification process.
    // judgedAgainst is a git commit SHA: happened-before is computed via git
    // ANCESTRY (wall-clock can't order divergent branches). See the contract.
    // Retyped to occurrence def (occurrenceRetype spike, D0032): a test result IS
    // an event — a performance of a verification at a specific moment in time.
    // Existing `part <name> : TestResult { ... }` instances continue to parse.
    occurrence def TestResult :> Element {
        attribute outcome : VerdictKind;
        attribute judgedAt : Timestamp;     // display only
        attribute judgedAgainst : String;   // git commit SHA (causal truth)
        attribute judgedBy : String;        // ci | actor id
        attribute notes : String[0..1];
    }
}
