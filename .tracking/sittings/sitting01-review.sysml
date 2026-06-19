// =====================================================================
// ProjectSittings — per-sitting sprint-review records (D0049 human gate; D0073 mechanism).
// A sitting = one continuous work session containing >=1 sprint. The review is a confirmation
// TestResult naming the sprints + decisions it accepts, written ONLY on the human's explicit
// attestation (D0016/D0051 — never inferred). This is the first such record (D0073's form);
// the form + a coverage guard are the sittingReviewRecord implementation task.
// =====================================================================
package ProjectSittings {
    private import EngineElement::*;
    private import EngineVerification::*;

    verification sitting01Review : Test {
        :>> id = "63940516-2c7d-4e8f-a551-516273849670";
        :>> title = "Sitting 01 review — sprints 38-44 + D0070-D0075 (2026-06-18/19)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Per-sitting sprint review (D0049 single human gate; D0073 mechanism). wweatherholtz reviewed and accepts the content of this sitting: sprints 38 (pglCharterEdge), 39 (pglProcessRef), 40 (pglProcessMarkerCarrier), 41 (pglProcessChangeGuard), 42 (pglCharterEnforcement), 43 (pglViews), 44 (auditAdherenceMetrics); and decisions D0070 (process-change marker on the Decision), D0071 (query-driven viewpoints, since superseded), D0072 (actualHours=AI-time), D0073 (sitting-review record), D0074 (full-Rust direction), D0075 (TOML enriched-select views). Explicit attestation: 'i reviewed and accept sprints 38-44 + d0070-d0075' (2026-06-19).";
    }
    part sitting01ReviewR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a551-516273849671";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "0bd3ca0";
        :>> judgedAt = "2026-06-19";
        :>> judgedBy = "wweatherholtz";
    }
}
