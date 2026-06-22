// =====================================================================
// ProjectIndicators — monitored measures for this project (D0089). Demonstrates all three
// measurement methods: COMPUTED (objective, repo-derived via the report/trend engine), PULLED
// (objective, external — a Measurement is recorded per observation), MANUAL (subjective, e.g. a
// periodic assessment — recorded). An Indicator informs a need/decision by DIRECTION (goal), not a
// threshold (that would be a requirement, D0088). View: `sysmlv2 indicators [--trend]`.
// Edges: #Measures (Measurement -> its Indicator); #Informs (Indicator -> the decision/need it informs).
// =====================================================================
package ProjectIndicators {
    private import EngineElement::*;
    private import EngineIndicator::*;
    private import EngineRelationships::*;

    // ── COMPUTED (objective, repo-derived; value/series from the shared metric_value, D0090; no stored datapoints) ──
    // collectionRef is a metric KEY (the single shared computation both reports and indicators source from).
    part coverageIndicator : Indicator {
        :>> id = "1d1c0001-0001-4001-9001-516273840001";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Verification coverage % (gate-covered assurance elements)";
        :>> goal = IndicatorGoal::maximize; :>> unit = "%";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "coverage_pct";
    }
    #Informs dependency from coverageIndicator to d0079;

    part critiqueCoverageIndicator : Indicator {
        :>> id = "1d1c0006-0006-4006-9006-516273840006";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Critique coverage % (elements with required-lens critiques)";
        :>> goal = IndicatorGoal::maximize; :>> unit = "%";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "critique_pct";
    }
    #Informs dependency from critiqueCoverageIndicator to d0080;

    part attestationIndicator : Indicator {
        :>> id = "1d1c0007-0007-4007-9007-516273840007";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Acceptance integrity % (accepted decisions with an attestation event)";
        :>> goal = IndicatorGoal::maximize; :>> unit = "%";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "attestation_pct";
    }
    #Informs dependency from attestationIndicator to d0066;

    part reqVerifiedIndicator : Indicator {
        :>> id = "1d1c0008-0008-4008-9008-516273840008";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Requirements verified % (3-tier, verified)";
        :>> goal = IndicatorGoal::maximize; :>> unit = "%";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "req_verified_pct";
    }
    #Informs dependency from reqVerifiedIndicator to d0079;

    part velocityIndicator : Indicator {
        :>> id = "1d1c0002-0002-4002-9002-516273840002";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Velocity (delivered story points per sprint)";
        :>> goal = IndicatorGoal::maximize; :>> unit = "points/sprint";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "velocity";
    }
    #Informs dependency from velocityIndicator to d0064;

    part throughputIndicator : Indicator {
        :>> id = "1d1c0009-0009-4009-9009-516273840009";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Throughput (delivery sprints recorded)";
        :>> goal = IndicatorGoal::maximize; :>> unit = "sprints";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "throughput";
    }
    #Informs dependency from throughputIndicator to d0064;

    part volatilityIndicator : Indicator {
        :>> id = "1d1c0003-0003-4003-9003-516273840003";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Requirements volatility (supersede-edge churn)";
        :>> goal = IndicatorGoal::minimize; :>> unit = "edges";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "volatility";
    }
    #Informs dependency from volatilityIndicator to d0054;

    part openFindingsIndicator : Indicator {
        :>> id = "1d1c000a-000a-400a-900a-51627384000a";
        :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus";
        :>> measures = "Open >= Medium findings (undispositioned + open Critical)";
        :>> goal = IndicatorGoal::minimize; :>> unit = "findings";
        :>> method = MeasurementMethod::computed; :>> collectionRef = "open_findings";
    }
    #Informs dependency from openFindingsIndicator to d0079;

    // ── PULLED (objective, external; each observation recorded as a Measurement with provenance) ──
    part avRegulationCount : Indicator {
        :>> id = "1d1c0004-0004-4004-9004-516273840004";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> measures = "Published autonomous-vehicle regulation documents (US federal)";
        :>> goal = IndicatorGoal::observe;
        :>> unit = "documents";
        :>> method = MeasurementMethod::pulled;
        :>> collectionRef = "a collection skill that queries regulations.gov / NHTSA and records a Measurement";
    }
    #Informs dependency from avRegulationCount to d0089;
    part avRegulationCountM1 : Measurement { :>> id = "1d1c0004-0004-4004-9004-5162738400a1"; :>> value = "42"; :>> measuredAt = "2026-03-01"; :>> source = "regulations.gov AV docket (manual seed)"; :>> createdBy = "claudeOpus"; }
    #Measures dependency from avRegulationCountM1 to avRegulationCount;
    part avRegulationCountM2 : Measurement { :>> id = "1d1c0004-0004-4004-9004-5162738400a2"; :>> value = "47"; :>> measuredAt = "2026-06-01"; :>> source = "regulations.gov AV docket (manual seed)"; :>> createdBy = "claudeOpus"; }
    #Measures dependency from avRegulationCountM2 to avRegulationCount;

    // ── MANUAL (subjective; recorded periodic assessment — the original friction case, D0054/D0088) ──
    part authoringFriction : Indicator {
        :>> id = "1d1c0005-0005-4005-9005-516273840005";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> measures = "Authoring-friction assessment (effort to record a fact vs a spreadsheet; subjective periodic rating)";
        :>> goal = IndicatorGoal::minimize;
        :>> unit = "effort (1=trivial..5=painful)";
        :>> method = MeasurementMethod::manual;
        :>> collectionRef = "periodic maintainer assessment (no defensible threshold yet, D0088 — watched by direction)";
    }
    #Informs dependency from authoringFriction to d0054;
    part authoringFrictionM1 : Measurement { :>> id = "1d1c0005-0005-4005-9005-5162738400b1"; :>> value = "4"; :>> measuredAt = "2026-04-01"; :>> source = "maintainer assessment (pre write-API maturity)"; :>> createdBy = "wweatherholtz"; }
    #Measures dependency from authoringFrictionM1 to authoringFriction;
    part authoringFrictionM2 : Measurement { :>> id = "1d1c0005-0005-4005-9005-5162738400b2"; :>> value = "2"; :>> measuredAt = "2026-06-01"; :>> source = "maintainer assessment (post append-result/add-task/apply-review)"; :>> createdBy = "wweatherholtz"; }
    #Measures dependency from authoringFrictionM2 to authoringFriction;
}
