// =====================================================================
// D0090 — INDICATORS are the canonical monitored-measure layer; a single shared computation
// (metric_value) feeds both indicators and reports, so a metric is computed in ONE place; reports
// RENDER the indicators (grouped by concern) + add point-in-time structure. The trended report
// metrics ARE indicators (D0089), never requirements (D0088). Accepted 2026-06-22.
// =====================================================================
package Decision0090 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0090 : Decision {
        :>> id = "63940516-2c7d-4e8f-a090-516273849720";
        :>> title = "D0090: indicators are the canonical monitored-measure layer; one shared metric_value feeds indicators + reports; reports render indicators (not requirements)";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0089 added the Indicator type, but the report metrics (coverage %, velocity, volatility, ...) were DUPLICATED — computed inline in the report cards AND re-declared as computed indicators bound to a report headline. The human asked to 'move all relevant reports into indicators (then to a report?)' and confirmed 'regardless, they shouldn't be requirements, right?' — choosing 'indicators canonical; reports render them'. The distinction that matters: a report holds TRENDED SCALAR MEASURES (which are indicators) AND point-in-time STRUCTURE (verdicts, tier breakdowns, finding/blocker lists) which are not monitored measures.";
        :>> decision = "(1) The trended scalar report metrics are first-class INDICATORS — the canonical watchlist (.tracking/indicators.sysml): coverage_pct, critique_pct, attestation_pct, req_verified_pct, velocity, throughput, volatility, open_findings (computed) + the pulled/manual demonstrators. `sysmlv2 indicators` is the canonical monitored-measure view. (2) A SINGLE shared computation `metric_value(root, key)` computes each canonical scalar in exactly ONE place (replacing the per-report headline_at); both the computed Indicators AND the report `--trend` headline source from it — the metric is no longer defined twice. (3) Reports RENDER: a report groups its concern's metrics as cards + adds the point-in-time STRUCTURE (READY/NOT verdict, coverage-tier breakdown, findings-by-severity, blocker lists) that are NOT trended measures and stay report-computed. (4) NONE are requirements (D0088): an Indicator is watched by DIRECTION (goal), not gated by a threshold; excluded from the assurance/orphan required-edge sets. INCREMENTAL (honest scope): the canonical indicator layer + the shared metric_value + report `--trend` sourcing are BUILT; having each report SCALAR card source its number directly from metric_value (full de-dup of the cards' inline scalar computation, where a card today also computes a richer breakdown) is a tracked refinement (reportIndicatorRender).";
        :>> rationale = "Single source of truth: the indicator SET declares what we monitor; metric_value is the single COMPUTATION; reports are the human-digestible rendering. This removes the metric-defined-in-two-places duplication the human flagged, and confirms the measure spectrum (D0088): a monitored measure with no defensible threshold is an indicator (direction-watched), not a requirement (threshold-gated) — so first-classing them as Indicators (D0089) is exactly right, and a report never turns them into pass/fail gates. Compute-don't-store holds: metric_value recomputes on demand; indicators are authored declarations + (for pulled/manual) recorded observations. ALTERNATIVES: (A) coexist with the duplication (current pre-D0090) — REJECTED (a metric defined in two places drifts); (B) reports become pure indicator groupings, dropping the structural content — REJECTED (loses the READY verdict / tier breakdowns / blocker lists that are genuinely not scalar measures); (C CHOSEN) indicators canonical + one shared metric_value + reports render the indicator group + structure. CRITERIA: single-source-of-truth x one-computation x not-a-requirement(D0088) x keep-the-structural-content x compute-don't-store.";
        :>> consequences = "view.rs: metric_value(root, key) (the keyed scalar registry) REPLACES headline_at; trend_series + indicators() + the report `--trend` (report_headline_key) all source from it. .tracking/indicators.sysml = 10 canonical indicators (8 computed bound to metric-keys: coverage_pct/critique_pct/attestation_pct/req_verified_pct/velocity/throughput/volatility/open_findings + avRegulationCount pulled + authoringFriction manual). `sysmlv2 indicators [--trend]` is the canonical watchlist. Tracked refinement: backlog reportIndicatorRender (report scalar cards source directly from metric_value / render the indicator group, full de-dup). doc-sync: CLAUDE.md measure spectrum + usage.md. Pairs with D0087 (reports), D0088 (measure spectrum, not-requirements), D0089 (Indicator type), D0054 (friction), D0001/D0018 (compute-don't-store).";
    }

    verification d0090Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a090-516273849721";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0090 (indicators are the canonical monitored-measure layer; one shared metric_value computation feeds both indicators and reports; reports render the indicators + structure; none are requirements) by directing on 2026-06-22: 'move all relevant reports into indicators (then to a report?) ... regardless, they shouldn't be requirements, right?' and selecting 'Indicators canonical; reports render them'.";
    }
    part d0090AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a090-516273849722";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "a75f9c5";
        :>> judgedAt = "2026-06-22";
        :>> judgedBy = "wweatherholtz";
    }
}
