// =====================================================================
// D0110 — adopt the Decision Authoring process (.engine/processes/decision-authoring.sysml) to make the
// one-source discipline ENFORCEABLE = a carried-out process (issue058; human-directed 2026-07-05).
// The D0070 keystone governing Decision for that process-def addition (#ProspectiveChange).
// =====================================================================
package Decision0110 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0110 : Decision {
        :>> id = "10a1b0c1-4110-42a3-ab5c-516273849710";
        :>> title = "D0110: adopt the Decision Authoring process (one-source discipline as a carried-out process)";
        :>> createdAt = "2026-07-05";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue058's one-source discipline (each Decision fact has ONE canonical home; verdicts/status are discrete TestResults or computed, never restated as prose) was previously only guidance in the business-architecture skill — and the dual-truth defect slipped past it even after D0106. The human directed (2026-07-05): 'enforceable = having a process that is carried out.' Per D0059 + the process-skill guard, a discipline is only carried out when it is a DEFINED process deployed by a skill (not doc guidance). Per D0070, adding such a process-def is a keystone process change requiring this governing Decision.";
        :>> decision = "Adopt .engine/processes/decision-authoring.sysml (a Process with steps: own-facts-in-their-fields / verdict-is-a-discrete-human-judged-TestResult-never-prose / state-is-computed / detecting-rule-backstop), deployed by the decision-authoring skill (registered) so it is NON-INERT (process-skill guard). Enforcement stack: the carried-out process + guard:confirmation-authenticity (acceptance must be human-judged, hard) + decisionNoVerdictProseRule (verdict-prose detector). This is the enforceable resolution of issue058.";
        :>> rationale = "ALTERNATIVES: (i) keep it as skill guidance (the prior 'close as covered') — REJECTED by the human (guidance is a linter, ignorable; the defect already slipped past it). (ii) a new hard guard only — REJECTED as incomplete (a guard checks artifacts; the DISCIPLINE of authoring is a process to be followed, D0059). (iii) CHOSEN — a defined, deployed process + the existing automated backstops: makes the discipline 'carried out' (non-inert per process-skill) AND checked. CRITERIA: enforceable-not-guidance x non-inert (D0059) x reuses-existing-backstops x D0070-governed.";
        :>> consequences = "The decision-authoring process + skill exist and are guarded non-inert (process-skill 13->14). Governs the process-def addition (D0070 keystone; this Decision carries #ProspectiveChange). Resolves issue058 (with oneSourceDecisionAuthoring). doc-sync: skills-registry (the deploying skill). Acceptance verdict/provenance are the discrete d0110Accept TestResult (not prose, D0058/issue058 — dogfooding the very discipline this adopts).";
    }

    verification d0110Accept : Test { :>> id = "10a1b0c1-4110-42a3-bb5c-516273849710"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of D0110 (adopt the decision-authoring process — make the one-source discipline a carried-out, deployed, non-inert process) via the explicit direction 'I want it to be enforceable, which I think means having a process that is carried out, yes?' on 2026-07-05."; }
    part d0110AcceptR1 : TestResult { :>> id = "10a1b0c1-4110-42a3-cb5c-516273849710"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "37931c2"; :>> judgedAt = "2026-07-05"; :>> judgedBy = "wweatherholtz"; }
}
