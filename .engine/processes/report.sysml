// =====================================================================
// EngineReportView — the COMPUTED REPORT / SCORECARD layer (D0087). Human-digestible AGGREGATE
// reports (totals, percentages, ratios) rolling up the per-element views, split into HEALTH and
// OPPORTUNITY reads. A report is NEVER authored or committed; it is computed on demand (§2.1/D0015
// #View) and trends recompute from git. Deployed by the report skill (D0059).
// =====================================================================
package EngineReportView {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    part reportView : Process {
        :>> id = "63940516-2c7d-4e8f-c087-516273849870";
        :>> title = "Computed Report / Scorecard (computed view)";
        :>> purpose = "Roll up the per-element computed views into human-digestible aggregate scorecards (verification coverage %, critique %, attestation %, findings, suspect load, traceability completeness, debt, flow/velocity) — a HEALTH read (is it sound now?) and an OPPORTUNITY read (where to improve / leading signals) — regenerated on demand from authored facts; never author or commit a report or a metric snapshot.";
        :>> cadence = Cadence::adHoc;
    }

    part rpCompute : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c087-516273849871";
        :>> title = "Compute — never store the number";
        :>> order = 1;
        :>> triggerCondition = "Someone wants a health/opportunity read of the project (coverage totals, %, debt, velocity, trend).";
        :>> actionText = "Run `keel report <assurance|traceability|quality-debt|flow>` (JSON) or `... --html` (a scorecard). Every metric is COMPUTED fresh by aggregating the per-element views (coverage/critique/assured/decisions/orphans/issues); trends recompute scalar metrics at sampled git commits. NEVER author a metric, commit a CSV/snapshot, or hand-maintain a dashboard — a stored number drifts (§2.1/D0018).";
        :>> producedArtifact = "A computed scorecard (JSON, or a self-contained HTML page of metric cards).";
        :>> owner = Owner::ai;
    }
    part rpRead : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c087-516273849872";
        :>> title = "Read — health vs opportunity";
        :>> order = 2;
        :>> triggerCondition = "A report is generated.";
        :>> actionText = "Read HEALTH reports (assurance, traceability) as 'is the model sound right now?' and OPPORTUNITY reports (quality-debt, flow) as 'where to improve / what is trending'. Tone colours flag good/warn/bad. The headline leading indicator is verification coverage %; the headline early-warning is requirements volatility (supersession churn).";
        :>> producedArtifact = "An ephemeral read (no stored artifact).";
        :>> owner = Owner::human;
    }
    part rpAct : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c087-516273849873";
        :>> title = "Act — feed the frontier, do not annotate the report";
        :>> order = 3;
        :>> triggerCondition = "A report surfaces a gap, debt, or downward trend.";
        :>> actionText = "Turn a finding into tracked work through the normal flow: a coverage/critique gap becomes a sprint item; a suspect element gets re-verified; a debt item is planned. Record the action as items + edges — NEVER annotate the report (it is disposable and regenerates). Generated /*.html is git-ignored and MUST NOT be committed.";
        :>> producedArtifact = "Tracked work items addressing the gap; nothing stored from the report itself.";
        :>> owner = Owner::ai;
    }
}
