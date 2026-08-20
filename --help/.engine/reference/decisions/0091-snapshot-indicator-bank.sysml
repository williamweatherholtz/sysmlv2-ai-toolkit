// =====================================================================
// D0091 — SNAPSHOT a computed indicator's reading into the Measurement bank (D0089): a recorded
// OBSERVATION ("the metric read X at time T"), not a cache. `sysmlv2 snapshot-indicators` computes
// each computed indicator's current metric_value + records a Measurement; the indicators view is
// BANK-FIRST + emits the full datapoint series. Builds a durable, fast bank for non-instant series
// (burndown velocity, etc.) alongside the pulled/manual observations. A controlled exception to
// compute-don't-store — surfaced + critiqued below. Accepted 2026-06-22.
// =====================================================================
package Decision0091 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0091 : Decision {
        :>> id = "63940516-2c7d-4e8f-a091-516273849730";
        :>> title = "D0091: snapshot computed indicators into the Measurement bank (recorded observation, not cache); indicators view is bank-first + emits the full series";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0089 banks pulled/manual observations as Measurements; computed indicators recompute their series from git on demand (--trend, slow + fragile). The human asked for a durable 'bank of data' to pull complex non-instant series (burndown velocity, capex per quarter) from. capex-per-quarter is already a pulled/manual indicator (banked); the gap is COMPUTED metrics (recompute-only). The human chose to add snapshotting + emit the full series, and explicitly asked to CRITIQUE this direction after implementation.";
        :>> decision = "Add `sysmlv2 snapshot-indicators` — for each `computed` Indicator, compute its current `metric_value` and record it as a `Measurement` (the SAME type as pulled/manual datapoints, #Measures-linked). Framed as a recorded OBSERVATION ('the metric read X at time T') — irreducible, immune to later computation-logic drift; distinct from `--trend` (which recomputes the past under CURRENT logic, a different question). Run periodically (per sprint/quarter) to build the bank. The `indicators` view is BANK-FIRST: the recorded Measurement series is the durable series for ANY method; it emits the full (measuredAt,value) series (for burndown-style charts); a computed indicator with NO bank falls back to `--trend` or its live value.";
        :>> rationale = "Pulled/manual non-instant data is already banked (D0089); snapshotting extends the same Measurement bank to computed metrics for durability, speed (no git replay), and to capture the historical READING (immune to logic drift). The observation-vs-cache framing is what keeps it consistent with compute-don't-store: a snapshot records what we OBSERVED, not a cache of a recomputable value — recomputing later answers a different question (current-logic-over-history). ALTERNATIVES: (A) recompute-only (--trend) — REJECTED (slow worktree replay, fragile to historical parse, no durable bank); (B) cache computed values as a #View — REJECTED (that IS a drift-prone cache); (C CHOSEN) snapshot-as-observation into the Measurement bank. HONEST TENSION (the human asked to critique this): snapshotting a computed metric DOES store a value that could be recomputed — the observation framing justifies it but is interpretive, with no mechanical guardrail; this decision is critiqued (D0080) below and any >= Medium finding is dispositioned by the human.";
        :>> consequences = "view.rs: measurement_series -> (measuredAt,value); indicators() is bank-first + emits the series + a `series` array; computed_indicator_keys (the snapshot worklist). main.rs: cmd_snapshot_indicators (computes metric_value per computed indicator + write::append_measurement). The bank accumulates real readings; `sysmlv2 indicators` shows the full series. doc-sync CLAUDE.md/usage.md. CRITIQUED (D0080, below): completeness + correctness findings (bank-first staleness; the compute-don't-store exception lacks a guardrail) surfaced for human disposition. Pairs with D0089 (Measurement/Indicator), D0090 (canonical indicators + metric_value), D0088 (measure spectrum), D0001/D0018 (compute-don't-store — the tension), D0050/D0084 (suspicion).";
    }

    verification d0091Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a091-516273849731";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0091 (snapshot computed indicators into the Measurement bank as recorded observations; bank-first indicators view + full series) by directing on 2026-06-22: 'is there a way to store metrics when they're taken? ... it would be nice to have a bank of data to pull complex non-instant (burndown velocity, capex per quarter) from', selecting 'add snapshot (reading = observation)' + 'emit the series', and 'after implementation critique this direction (per our process, right?)'.";
    }
    part d0091AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a091-516273849732";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "fdb1427";
        :>> judgedAt = "2026-06-22";
        :>> judgedBy = "wweatherholtz";
    }
}
