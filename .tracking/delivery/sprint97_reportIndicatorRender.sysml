// =====================================================================
// ProjectDeliveryS97 — Sprint 97: reportIndicatorRender (D0090 refinement). The canonical indicator
// layer + shared metric_value + report --trend sourcing were built (D0090), but the report SCALAR
// cards still computed their numbers INLINE — a metric written twice (once in metric_value, once in
// the card), which can drift. Refactor: extract pure shared formula helpers (coverage_pct_of /
// verified_pct_of / collect_flows / velocity_of) and route BOTH metric_value AND the report scalar
// cards through them, so each metric is computed in exactly one place AND the card shows the same
// number the Indicator does. Structural cards (per-tier breakdowns, edge completeness, debt, lists)
// keep their local computations for the detail text. Verified: report headlines == indicators
// (coverage 100=100, critique 24=24, attestation 100=100, velocity 3.06=3.06).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS97 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from reportIndicatorRenderStory to d0090;

    part reportIndicatorRenderStory : Story {
        :>> id = "63940516-2c7d-4e8f-b097-51627384c001";
        :>> title = "Sprint 97: reportIndicatorRender (2 pts) — report scalar cards source from shared metric helpers (single-source with the Indicators); full D0090 de-dup";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::refactor;
        :>> priority = WorkPriority::p3;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS97 {
        action storyReportIndicatorRender;
        verification storyReportIndicatorRenderDoD : Test { :>> id = "63940516-2c7d-4e8f-b097-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0090 incremental refinement (reportIndicatorRender): the report SCALAR cards computed their numbers inline, duplicating metric_value. FIX: extracted pure shared formula helpers in view.rs — coverage_pct_of(cov, ty) + verified_pct_of(cov, ty) (the gate-covered / verified tier ratios) and collect_flows(root) + velocity_of(flows) (the sprint-flow set + canonical mean velocity). metric_value now calls these for coverage_pct/req_verified_pct/needs_verified_pct/velocity/throughput/burnup; the assurance, traceability, and flow scalar cards call the SAME helpers for their headline value (keeping verified/attested/tier breakdowns + edge completeness local for the detail text). Each metric is now computed in exactly one place and the card shows the same number its Indicator does. VERIFY (live): report assurance coverage 100% == coverageIndicator 100; critique 24% == critiqueCoverageIndicator 24; attestation 100% == attestationIndicator 100; report flow Velocity 3.06 == velocityIndicator 3.06 (the card now shows the canonical float, not a truncated int). clippy --workspace --all-targets -D warnings + cargo test (44 lib) + rust validate (110) + guard all (13) green."; }
        part storyReportIndicatorRenderDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b097-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9919a24"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }
    }

    verification reportIndicatorRenderRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b097-51627384c004"; :>> title = "Sprint 97 refine gate (DoR)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0090 left a tracked refinement — the report scalar cards still computed inline (a metric written twice). Resolver: reportIndicatorRender. estimatedPoints=2. #CharteredBy d0090. DoD accepted: extract shared formula helpers; route both metric_value and the scalar cards through them; values must stay identical (verified against the indicators)."; }
    part reportIndicatorRenderRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b097-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9919a24"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification reportIndicatorRenderStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b097-51627384c006"; :>> title = "Sprint 97 standup gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add coverage_pct_of/verified_pct_of (near is_covered_tier) + collect_flows/velocity_of (near sprint_flow); simplify metric_value's coverage + flow arms to call them; swap assurance covered_pct, traceability n_pct/r_pct, and flow velocity to the helpers; format velocity as float. No semantic change (numbers identical, velocity gains precision). No blockers."; }
    part reportIndicatorRenderStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b097-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9919a24"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification reportIndicatorRenderImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b097-51627384c008"; :>> title = "Sprint 97 implement gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the 4 shared helpers + routed metric_value and the assurance/traceability/flow scalar cards through them. VERIFY (live): report headlines == indicators (coverage 100=100, critique 24=24, attestation 100=100, velocity 3.06=3.06); clippy --workspace --all-targets -D warnings green; cargo test 44 lib pass; rust validate 110 clean; guard all (13) PASS."; }
    part reportIndicatorRenderImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b097-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9919a24"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification reportIndicatorRenderReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b097-51627384c00a"; :>> title = "Sprint 97 review gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The coverage/V&V/velocity metrics now live in one formula each, called by both metric_value (which the Indicators + --trend already source) and the report scalar cards — closing D0090's 'a metric computed once AND declared once as an indicator' intent. Numbers verified identical to the indicators; the only display change is the flow Velocity card now showing the canonical float (3.06) instead of a truncated int (3), which is more accurate and matches velocityIndicator. Structural cards untouched. Scoped, low-risk. Clean."; }
    part reportIndicatorRenderReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b097-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9919a24"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification reportIndicatorRenderCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b097-51627384c00c"; :>> title = "Sprint 97 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyReportIndicatorRender DoD recorded pass; shared formula helpers live; report scalar cards == indicators; clippy + cargo test (44) + rust validate (110) + guard all (13) green. Self-evidencing (D0051). D0090 (indicators canonical / reports render them) is now fully realized — no inline scalar duplication remains in the de-dupped metrics."; }
    part reportIndicatorRenderCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b097-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9919a24"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification reportIndicatorRenderRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b097-51627384c00e"; :>> title = "Sprint 97 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the de-dup unit was chosen as the pure FORMULA helper (coverage_pct_of/verified_pct_of/velocity_of) rather than routing cards through metric_value(root,key) — this kept native u32/f64 return types (no f64->u32 cast lint), avoided redundant Model rebuilds (cards reuse their already-built cov/flows), and still gave one source of truth; cross-checking the report headlines against `indicators` made 'single-source' a VERIFIED claim, not an assertion. CAUGHT (not avoidable-rework, a deliberate scope line): critique%/attestation%/volatility/accepted-decisions are counts already computed via the shared compute_* + pct() helpers, so they were left as-is (their formula is already single-source); only the coverage-tier and flow scalars were genuinely written twice. Estimate 2 pts held. D0090 fully landed; the queued report<->indicator refinement is closed. The remaining frontier is the critique campaign batch 4 (d0062/d0047/d0049)."; }
    part reportIndicatorRenderRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b097-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9919a24"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }
}
