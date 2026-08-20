// =====================================================================
// D0087 — a COMPUTED REPORT/SCORECARD layer over the view layer: human-digestible aggregate reports
// (totals, percentages, ratios) + git-derived TRENDS, rolling up the existing per-element/gap views.
// `sysmlv2 report <name>` (JSON) + a render report scorecard (HTML). Grounded in the INCOSE Digital
// Engineering Measurement Framework + SE Leading Indicators + the model-quality canon. Everything is
// COMPUTED (no stored metric history — trends recompute from git); never authored, never committed.
// #ProspectiveChange: introduces a process-def + new computed views (keystone, D0070). Accepted 2026-06-21.
// =====================================================================
package Decision0087 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0087 : Decision {
        :>> id = "63940516-2c7d-4e8f-a087-5162738496e0";
        :>> title = "D0087: computed report/scorecard layer (`sysmlv2 report <name>` + render report) — aggregate health + opportunity metrics with git-derived trends, rolling up the per-element views";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "We compute rich PER-ELEMENT / gap views (coverage 3-tier, critique-coverage, orphans, suspect, assured, decisions, attestation, open-issues) but have no human-digestible AGGREGATE layer — totals, percentages, ratios, trends. The human asked for report artifacts on a viewpoint basis with aggregated values + useful metrics (test/verification coverage totals + %, and others), and which indicate HEALTH vs OPPORTUNITY. Research (INCOSE Digital Engineering Measurement Framework: program-execution + model-health incl. requirements volatility; INCOSE/SEAri SE Leading Indicators Rev2: % requirements verified, volatility, defect/issue trends, debt burndown; model-quality canon consistency/completeness/correctness via 'human rigour + automated tools' = our critique + guards; DO-178C/ISO-26262 traceability completeness) converges on: report coverage %, churn/volatility, trend direction, and a readiness verdict — not just lists. The human accepted ALL FOUR reports below + trends-now.";
        :>> decision = "Add a COMPUTED report/scorecard layer that AGGREGATES the existing computed views (no new measurement, mostly rollup) — `sysmlv2 report <name>` emits JSON; `sysmlv2 render report --mode report` (or report HTML) renders a human-digestible scorecard. FOUR reports: (1) ASSURANCE SCORECARD [health] — verification coverage % by tier (verified/attested/addressed/suspect/uncovered, D0082), critique-coverage %, attestation %, open findings by severity, suspect count, READY/NOT verdict + blocker counts (rolls up `assured`). (2) TRACEABILITY / V&V COVERAGE [health] — % needs satisfied-through-verified, % requirements verified (3-tier), orphan rate, satisfy/verify/allocate edge completeness (DO-178C-style end-to-end). (3) QUALITY & DEBT [opportunity] — charter debt (grandfathered elements still uncovered/uncritiqued), suspect+stale set, requirements volatility (supersession rate), decision antiquation (load-bearing-but-uncritiqued / references-retired, from `decisions`). (4) FLOW / VELOCITY [opportunity] — ready-frontier size, WIP (in-progress sprints), throughput (sprints/period), velocity (points/sprint from delivery Stories + dates), issue aging. TRENDS: scalar metrics (coverage %, volatility, finding counts, velocity) are recomputed at SAMPLED historical commits via git (a Model-at-commit reader over git blobs) and rendered as sparklines/series — COMPUTED FROM GIT, never a stored metric history (compute-don't-store, §2.1/D0018). Engine-native: a `report` process-def (regenerate-don't-commit) + report skill (D0059) + a 'report' viewpoint (D0056/D0057); generated /*.html git-ignored; doc-sync.";
        :>> rationale = "These are the canonical SE/MBSE measures (INCOSE DE Measurement Framework + SE Leading Indicators) expressed over OUR authored facts; verification coverage % is the headline leading indicator and requirements volatility (supersession churn) is the documented early-warning sign (ties to the D0054 adoption-friction risk). A report is a pure DERIVATION of authored facts — committing a metric snapshot would store a thing that drifts (the §2.1/D0018 failure), so reports REGENERATE and trends recompute from git, exactly like coverage/assured/diagram/render. Splitting HEALTH (point-in-time soundness) from OPPORTUNITY (leading/where-to-improve) matches how the indicators are meant to be read. Reuse over invention: each scorecard aggregates functions we already have (compute_coverage, compute_critique_coverage, attestation, readiness, decisions_report, orphans, open-issues). ALTERNATIVES: (A) stored metric dashboard / committed CSV — REJECTED: a parallel store that drifts; (B) ad-hoc python per metric — REJECTED (the formalize-reports rule); (C) point-in-time only — REJECTED by the human (trends are the leading-indicator value); (D CHOSEN) computed report layer + git-derived trends. CRITERIA: canonical-SE-measures x health-vs-opportunity x aggregate-over-existing-views x trends-from-git x compute-don't-store.";
        :>> consequences = "New `sysmlv2 report <name>` (assurance|traceability|quality-debt|flow) emitting JSON aggregates (view.rs report functions reusing the per-element computations) + a report HTML scorecard via the render layer (a report template + a 'report' render mode or `report --html`). New git Model-at-commit reader (reuse orient.rs batched cat-file helpers) to recompute scalar metrics at sampled commits for trends/sparklines. New `report` process-def + report skill + skills-registry + 'report' viewpoint. .gitignore keeps ignoring /*.html. #ProspectiveChange keystone co-commit (D0070). doc-sync: usage.md + CLAUDE.md. Sequenced: Stage 1 (Sprint 86) = the four point-in-time scorecards; Stage 2 (Sprint 87) = git-derived trends. Pairs with D0082 (3-tier coverage), D0079/D0080 (assured/critique), D0077 (issues), D0086 (render), D0048 (Rust authority), D0001/D0018 (compute-don't-store), D0054 (friction/volatility risk).";
    }

    verification d0087Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a087-5162738496e1";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0087 (a computed report/scorecard layer aggregating the per-element views into health + opportunity metrics with git-derived trends — assurance scorecard, traceability/V&V coverage, quality & debt, flow/velocity) by directing on 2026-06-21: 'let's talk about report artifacts ... make human-digestible reports ... viewpoint basis, with aggregated values or other useful metrics e.g. test coverage totals, % ... which indicate health and opportunities for improvement', and selecting all four reports + 'Include trends now'.";
    }
    part d0087AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a087-5162738496e2";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "9e1aa68";
        :>> judgedAt = "2026-06-21";
        :>> judgedBy = "wweatherholtz";
    }
}
