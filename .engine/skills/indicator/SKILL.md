---
name: indicator
description: |
  Deploys the Indicator Monitoring process (D0089): declare monitored MEASURES (Indicators) that
  inform a need/decision by DIRECTION (not a threshold — that's a requirement, D0088), and collect
  their data by method — computed (repo-derived, `keel indicators`), pulled (external/programmatic,
  `record-measurement`), or manual (subjective survey/assessment, `record-measurement`). Use when
  asked to track/watch/monitor a metric or KPI over time, add an indicator, record a measurement,
  pull an external signal (market/regulation/social), or see how a measure is trending. An Indicator
  is a first-class item; a Measurement is one recorded datapoint. Grounded in ISO-15939/PSM/SMM.
metadata:
  version: 0.1.0
  domain: [indicator, measurement, metric, KPI, monitoring, ISO-15939, PSM, SMM, D0089]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# indicator

Runs the engine's Indicator Monitoring process (`.engine/processes/indicator.sysml`). Its defining
move: **an indicator is watched by direction, not gated by a threshold** (D0088) — so it never
PASSES/FAILS; it *informs*. Grounded in the ISO/IEC/IEEE 15939 + PSM + OMG SMM measurement
information model (measure vs measurement; objective vs subjective method).

## Expert Vocabulary Payload

An **`Indicator`** (`.tracking/indicators.sysml`) carries `measures`, `goal`
(minimize/maximize/observe), `unit`, `method`, `collectionRef`, and `#Informs` the need/decision it
serves. **No verify/satisfy edges** — it's excluded from the assurance/orphan required-edge sets. A
**`Measurement`** is one datapoint (`value` @ `measuredAt` + `source`), linked to its indicator by a
`#Measures` edge.

**Three measurement methods:**
- **computed** (objective, repo-derived): `collectionRef` = a report name; the series is computed on
  demand via the report/trend engine — **no stored Measurements** (compute-don't-store).
- **pulled** (objective, external): a collection skill/command queries an API/scraper (web/MCP tools),
  then `keel record-measurement --indicator I --value V --at DATE --source ...` records the
  observation (external values can't be recomputed, so they're recorded with provenance).
- **manual** (subjective): a human gathers the value (survey/assessment) and records it the same way.

**View:** `keel indicators [--trend]` — per indicator: value, baseline→latest, and direction-aware
**status** (improving/degrading/flat by goal). Computed series need `--trend` (git replay, slow);
pulled/manual series come from the recorded Measurements (cheap).

## Anti-Pattern Watchlist

1. **Modeling a monitored measure as a Requirement** — forces a verify edge / pass-fail gate on a
   measure with no defensible threshold (the friction misstep, D0088). Fix: it's an Indicator.
2. **Writing a premature threshold** — don't gate on a "good enough" number you can't defend; watch
   the direction, promote to a requirement only when a real boundary emerges (D0088).
3. **Fabricating/back-dating an external or manual observation** — a Measurement is an irreducible
   recorded fact with provenance; record what was actually observed, never invent it.
4. **Storing a computed indicator's datapoints** — computed series recompute from the repo; don't
   freeze them (compute-don't-store).
