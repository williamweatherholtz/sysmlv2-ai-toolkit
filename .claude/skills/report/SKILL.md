---
name: report
description: |
  Deploys the Computed Report / Scorecard process (D0087): generate human-digestible AGGREGATE
  reports via `keel report <assurance|traceability|quality-debt|flow> [--html]` — totals,
  percentages, and ratios rolled up from the per-element views, split into HEALTH (is it sound now?)
  and OPPORTUNITY (where to improve / leading signals). Use when asked for a status/health report,
  coverage %, a scorecard, metrics/KPIs, debt, velocity, or "how are we doing". A report is a
  computed #View (§2.1/D0015) — never author a metric, commit a snapshot, or hand-maintain a dashboard.
metadata:
  version: 0.1.0
  domain: [report, scorecard, metrics, KPI, coverage, health, MBSE-measurement, computed-view, D0087]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# report

Runs the engine's Computed Report / Scorecard process (`.engine/processes/report.sysml`). Its
defining move: **a metric is a derivation, not a fact** — every number is recomputed from authored
facts on demand (trends recompute from git), so a report can never drift and a snapshot is never
stored (the compute-don't-store guarantee of `coverage`/`assured`/`render`).

## Expert Vocabulary Payload

**`keel report <name> [--html] [--root ROOT]`** → JSON aggregates (or, with `--html`, a
self-contained scorecard of metric cards with good/warn/bad tone). Grounded in the INCOSE Digital
Engineering Measurement Framework + SE Leading Indicators + the model-quality canon
(consistency/completeness/correctness via human rigour + automated tools = our critique + guards).

- **`assurance`** [HEALTH] — verification coverage % (verified+attested of all, D0082), critique
  coverage %, acceptance integrity %, open findings by severity, suspect load, the READY/NOT verdict.
- **`traceability`** [HEALTH] — % needs verified, % requirements verified (3-tier), needs-with-satisfy
  and requirements-with-verify edge completeness (DO-178C/ISO-26262-style end-to-end traceability).
- **`quality-debt`** [OPPORTUNITY] — charter debt (grandfathered elements still uncovered/uncritiqued),
  requirements volatility (supersession churn — the early-warning signal), suspect + stale set.
- **`flow`** [OPPORTUNITY] — ready frontier, WIP, velocity (points/sprint), cycle time (refine→retro),
  time / story point, lead time (created→retro, DORA-style), predictability (point spread), throughput,
  aging WIP, open issues. (`--trend` headline = delivered-points burnup.)
- **`governance`** [HEALTH] — decisions (accepted/superseded), acceptance integrity %, process-change
  decisions (#ProspectiveChange/#SafetyChange), supersession churn.

**The two numbers to watch:** verification coverage % (the headline leading indicator) and
requirements volatility (the documented early-warning sign, ties to the D0054 adoption-friction risk).

**`--trend`** adds a git-derived time-series for each report's headline metric (assurance →
verification coverage %, traceability → requirements verified %, quality-debt → supersede volatility,
flow → throughput), rendered as a sparkline with first→last delta. It recomputes the FULL pipeline at
each of ~12 recent commits via a throwaway git worktree — accurate but **slow (minutes)** and
COMPUTED FROM GIT, never a stored metric history. Use it for "are we getting healthier?", not routine checks.

**Generate, don't commit:** `keel report assurance --html > scorecard.html` then open it.
Generated `*.html` is git-ignored (D0085/D0086/D0087). It's the `report` viewpoint (D0056/D0057).

## Anti-Pattern Watchlist

1. **Committing a metric snapshot / CSV / rendered scorecard** — stores a number that drifts. Fix:
   regenerate; the tool + authored facts are the source.
2. **Hand-maintaining a dashboard** — a parallel store. Fix: add the metric to a computed report.
3. **Ad-hoc python to get a number** — Fix: formalize it as a report card (the formalize-reports rule).
4. **Annotating a report with findings** — Fix: a gap becomes tracked work (items + edges), not a note
   on a disposable artifact.
