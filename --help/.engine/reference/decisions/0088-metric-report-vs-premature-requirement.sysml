// =====================================================================
// D0088 — a measurable quality whose "good enough" boundary cannot yet be defensibly set is recorded
// as a MONITORED leading-indicator REPORT (a computed scorecard, D0087), NOT a requirement or a guard/
// constraint. Promote to a requirement (verifiable threshold) or a guard (executable predicate) only
// when a justified boundary emerges. Reframes friction (issue029/D0054) as a first-class CONCERN
// monitored as an indicator, not a premature requirement. Avoids the Goodhart trap. Accepted 2026-06-22.
// =====================================================================
package Decision0088 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0088 : Decision {
        :>> id = "63940516-2c7d-4e8f-a088-5162738496f0";
        :>> title = "D0088: a metric with no defensible threshold is a monitored leading-indicator REPORT, not a requirement; promote to a requirement/guard only when a justified boundary emerges";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Resolving the d0054 friction finding (issue029) produced a `friction` report. The question arose whether friction should ALSO be a tracked requirement (a 'shall' with a verifiable threshold). The human's position: friction's 'good enough' boundary is genuinely nebulous, so a hard requirement would be a premature misstep — a report is more instructive. Research corroborates: SE leading indicators carry thresholds as INVESTIGATION TRIGGERS, not acceptance gates (SEAri SE Leading Indicators Guide Rev2; NASA Common Leading Indicators); a requirement, by contrast, needs a defensible verifiable threshold. Forcing a nebulous metric into a requirement invites the Goodhart trap ('when a measure becomes a target it ceases to be a good measure') + arbitrary gating. SysML v2 frames a requirement as a special kind of CONSTRAINT (an evaluable boolean predicate + stakeholder shall + verification); an indicator that lacks a defensible predicate is therefore neither a requirement nor a constraint/guard — it is a monitored measure.";
        :>> decision = "A measurable quality whose 'good enough' boundary cannot YET be defensibly set is recorded as a MONITORED leading-indicator REPORT (a computed scorecard #View, D0087) — NOT a requirement (a verified stakeholder contract) and NOT a guard/constraint (an enforced executable predicate). Promote it to a requirement (with a verifiable threshold) or a guard (with a crisp true/false predicate) ONLY when a justified boundary emerges — from baseline/trend data, a real observed failure, or a stakeholder commitment (characterize-before-constrain). This applies to AUTHORING FRICTION specifically: it stays the `friction` report (D0087) as a first-class CONCERN monitored as an indicator (correcting D0054's looser 'first-class requirement' phrasing), not a tracked requirement. It is also the general decision rule for future metrics: report (indicator) -> requirement/guard only on a justified boundary.";
        :>> rationale = "Leading-indicator thresholds are 'triggers for investigation' (SEAri/NASA), not pass/fail acceptance criteria; a premature requirement around a nebulous threshold both invites Goodhart distortion (optimizing the proxy) and gates on an arbitrary number. The measure-then-threshold discipline (GQM; PSM; ISO/IEC/IEEE 15939: information-needs -> indicators, not every indicator is an acceptance criterion) says: characterize + baseline + watch the trend, and only constrain when the boundary is real and defensible. A report is MORE instructive than a false binary — it shows the comparison + trend, leaving judgement to the reader. We already practised the disciplined inverse (D0083): sr11FastStart HAD a threshold, we MEASURED it, it failed, and we revised the target rather than faking the gate — characterize-then-adjust. ALTERNATIVES: (A) write the friction requirement now with a guessed threshold — REJECTED (Goodhart + premature, the human's stated misstep); (B) drop the friction metric entirely — REJECTED (friction is a real first-class concern, D0054, the #1 adoption risk); (C CHOSEN) monitored report now, promote-when-justified. CRITERIA: avoid-premature-thresholds x indicator-vs-acceptance-gate x characterize-before-constrain x instructive-over-binary.";
        :>> consequences = "Friction stays the `friction` report (D0087); issue029's resolution is reframed as a concern-monitored-as-indicator, not a requirement (no requirement/verify edge added). A reusable decision rule for the report-vs-requirement-vs-guard choice on any future metric. Establishes the engine's measure spectrum: INDICATOR (monitored report, no enforced threshold) -> REQUIREMENT (verified stakeholder contract with a threshold) -> CONSTRAINT/GUARD (enforced executable predicate). doc-sync: a CLAUDE.md note that the guards are the engine's executable constraint layer (SysML-v2 'requirements-as-evaluable-constraints' realized as Rust predicates), reports are indicators, and parametric constraints are deliverable-domain. Pairs with D0054 (friction concern), D0087 (report layer), D0079/D0080 (assurance/critique), D0083 (characterize-then-revise precedent), D0001/D0018 (compute-don't-store).";
    }

    verification d0088Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a088-5162738496f1";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0088 (a metric with no defensible threshold is a monitored leading-indicator report, not a premature requirement; promote only when a justified boundary emerges) by directing on 2026-06-22: 'I think friction belongs in a report, not a requirement per se. The threshold for good enough isn't very clear and when that's the case, I feel like a report is more instructive and a hard requirement is a misstep ... it's too nebulous to write a premature requirement around', and selecting 'Record it'.";
    }
    part d0088AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a088-5162738496f2";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "d8213b6";
        :>> judgedAt = "2026-06-22";
        :>> judgedBy = "wweatherholtz";
    }
}
