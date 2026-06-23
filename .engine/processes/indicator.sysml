// =====================================================================
// EngineIndicatorMonitoring — declare + collect + interpret MONITORED MEASURES (D0089;
// ISO-15939/PSM/SMM). An Indicator informs by DIRECTION (D0088), not a threshold. Data arrives by a
// measurement METHOD: computed (repo-derived, no stored datapoints), pulled (external, recorded),
// manual (subjective, recorded). Deployed by the indicator skill (D0059).
// =====================================================================
package EngineIndicatorMonitoring {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    part indicatorMonitoring : Process {
        :>> id = "63940516-2c7d-4e8f-c089-516273849710";
        :>> title = "Indicator Monitoring (computed / pulled / manual)";
        :>> purpose = "Declare monitored measures (Indicators) that inform needs/decisions by direction, and collect their data by the right method — computed (repo-derived), pulled (external programmatic), or manual (subjective) — so the engine can show each measure's value + direction-aware trend without committing to a premature threshold (D0088).";
        :>> cadence = Cadence::adHoc;
    }

    part imDeclare : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c089-516273849711";
        :>> title = "Declare the indicator (not a requirement)";
        :>> order = 1;
        :>> triggerCondition = "A measure worth WATCHING is identified, but its 'good enough' threshold is not defensibly settable (else it is a requirement, D0088).";
        :>> actionText = "Author an `Indicator` (.tracking/indicators.sysml): measures + goal (minimize/maximize/observe) + unit + method (computed/pulled/manual) + collectionRef; #Informs dependency to the need/decision it informs. Do NOT add verify/satisfy edges — an Indicator is monitored by direction, not gated by a threshold; it is excluded from the assurance/orphan required-edge sets.";
        :>> producedArtifact = "An authored Indicator item.";
        :>> owner = Owner::ai;
    }
    part imCollect : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c089-516273849712";
        :>> title = "Collect data by the method";
        :>> order = 2;
        :>> triggerCondition = "An indicator exists and a fresh datapoint is due (cadence) or first needed.";
        :>> actionText = "computed: nothing to record — the series is computed on demand from the repo (the report/trend engine). pulled: run the collection skill/command (an API/scraper, e.g. via web/MCP tools) and `keel record-measurement --indicator I --value V --at DATE --source ...` (a recorded observation, because external values can't be recomputed). manual: gather the value (survey/assessment) and record it the same way. NEVER fabricate or back-date an external/manual observation — record what was actually observed, with provenance.";
        :>> producedArtifact = "For pulled/manual: a recorded Measurement datapoint (value @ measuredAt + source). For computed: nothing stored.";
        :>> owner = Owner::ai;
    }
    part imInterpret : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c089-516273849713";
        :>> title = "Interpret — direction, not a gate";
        :>> order = 3;
        :>> triggerCondition = "Someone wants to read the indicators.";
        :>> actionText = "Run `keel indicators [--trend]` (computed series need --trend). Read each indicator's value + baseline->latest + direction-aware status (improving/degrading/flat by its goal). An indicator never PASSES or FAILS — it informs. If a real, defensible boundary emerges (a baseline + a stakeholder commitment), THEN promote it to a requirement/guard (D0088); until then, watch the direction.";
        :>> producedArtifact = "An ephemeral read (no stored artifact); possibly a decision to act on a degrading signal.";
        :>> owner = Owner::human;
    }
}
