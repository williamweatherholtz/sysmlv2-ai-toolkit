// =====================================================================
// D0059 — process-has-deploying-skill: every process is DEPLOYED by a skill (its own
// or a consuming ceremony skill); no inert processes. Adds the missing doc-sync skill.
// Accepted 2026-06-17 by wweatherholtz ("processes should have downstream AI skills that
// run as a means of deployment ... use skills to cement processes").
// =====================================================================
package Decision0059 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0059 : Decision {
        :>> id = "61728394-0a5b-4c6d-c619-40516273841b";
        :>> title = "D0059: every process has a downstream deploying skill (no inert processes); doc-sync skill added";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Repeated pain: process improvements + doc updates required the human to manually drive them (e.g. hand-updating CLAUDE.md after each decision). Root cause, recurring: a PROCESS defined without a deploying SKILL is inert — applied by human vigilance, inconsistently or not at all. Same failure class as issue007 (critique process had no skill) and issue011 (review invocation not enforced). doc-sync was the live instance: .engine/processes/doc-sync.sysml + a CLAUDE.md §4 rule existed, but no doc-sync skill, so doc updates fell to the human.";
        :>> decision = "PRINCIPLE: every process MUST be deployed by an AI skill — either its OWN deploying skill (doc-sync -> doc-sync skill; architectural-critique -> architectural-critique skill) or a CONSUMING ceremony skill (definition-of-ready -> sprint-planning; definition-of-done -> sprint-closeout/test-result; agile-workflow -> the sprint-* skills). No process is inert: if a process has no deploying skill, that is an orphan (flagged by the orphans/coverage view, generalized to processes). Generalizes D0040 (recurring work -> skill). Applied now: created the doc-sync skill (.engine/skills/doc-sync/SKILL.md) + registered it.";
        :>> rationale = "ALTERNATIVES: (A) a pre-commit GUARD that fails when a change 'should' touch docs but didn't — rejected: 'should touch docs' is not mechanically decidable (which doc? what claim?), so the guard would be either toothless or noisy; (B) keep relying on the CLAUDE.md §4 rule + human vigilance — rejected: that is exactly the failure mode being fixed (drift was a HIGH finding, and it recurred); (C) CHOSEN: a deploying skill per process — the AI runs the process consistently, and process<->skill coverage is auditable as a view. CRITERIA: consistency (skills > prose rules for recurring work) x enforceability (a skill is invoked + auditable) x mechanizability (don't build guards for the non-mechanizable) x friction (D0054).";
        :>> consequences = "doc-sync skill registered + referenced in CLAUDE.md §4. The process<->skill mapping becomes a coverage concern (which processes lack a deploying skill -> orphan). Future processes ship with their deploying skill in the same change. Reduces the human-must-drive-doc-updates friction the principle was created to fix. Reaffirms D0040.";
    }

    verification d0059Accept : Test { :>> id = "b4a1e25f-5800-47ba-aeea-e42f15efe244"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0059 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0059AcceptR1 : TestResult { :>> id = "7a97b218-da6d-46c5-85ed-63f9f1c18cbe"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
