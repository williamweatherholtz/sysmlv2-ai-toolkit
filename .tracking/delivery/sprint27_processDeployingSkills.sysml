// =====================================================================
// ProjectDeliveryS27 — Sprint 27: processDeployingSkills (2 pts).
// Story: every process has a downstream deploying skill (D0059); add the missing
//        doc-sync skill so documentation sync runs as the AI's job, not human vigilance.
//        Also records D0060 (the settled attribution/decision model) for ingestion.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049); process/skill + decisions, no schema.
// =====================================================================
package ProjectDeliveryS27 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part processDeployingSkillsStory : Story {
        :>> id = "61728394-0a5b-4c6d-bd71-405162738430";
        :>> title = "Sprint 27: processDeployingSkills (2 pts) — doc-sync skill + process->skill principle (D0059); record D0060";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS27 {
        action storyProcessDeployingSkills;
        verification storyProcessDeployingSkillsDoD : Test { :>> id = "61728394-0a5b-4c6d-ce72-405162738431"; :>> method = VerificationMethod::inspect; :>> procedureText = "processDeployingSkillsDoD in backlog (pass): doc-sync skill + D0059 + CLAUDE.md §4; D0060 (attribution model) recorded; validate_instances green."; }
        part storyProcessDeployingSkillsDoDR1 : TestResult { :>> id = "61728394-0a5b-4c6d-fe73-405162738432"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c844af"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification processDeployingSkillsRefineGate : Test {
        :>> id = "61728394-0a5b-4c6d-1174-405162738433";
        :>> title = "Sprint 27 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach: record D0059 (every process has a deploying skill; no inert processes) + build the missing doc-sync skill (deploys doc-sync process) + register + CLAUDE.md §4 (dogfood: doc-sync this very change). Separately record D0060 (settled data/metadata/attribution model) + queue the attributionModel migration. No schema change this sprint. estimatedPoints=2. DoD accepted.";
    }
    part processDeployingSkillsRefineGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-2274-405162738434"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c844af"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processDeployingSkillsStandupGate : Test {
        :>> id = "61728394-0a5b-4c6d-3374-405162738435";
        :>> title = "Sprint 27 standup gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. doc-sync skill + D0059 + D0060 + CLAUDE.md §4. No blockers.";
    }
    part processDeployingSkillsStandupGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-4474-405162738436"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c844af"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processDeployingSkillsImplementGate : Test {
        :>> id = "61728394-0a5b-4c6d-5574-405162738437";
        :>> title = "Sprint 27 implement gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = ".engine/skills/doc-sync/SKILL.md created + registered (docSync); D0059 (process->skill) + D0060 (attribution model) recorded; CLAUDE.md §4 references the doc-sync skill + the principle (doc-sync dogfooded on this change); attributionModel migration queued; validate_instances green; no schema change.";
    }
    part processDeployingSkillsImplementGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-6674-405162738438"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c844af"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processDeployingSkillsReviewGate : Test {
        :>> id = "61728394-0a5b-4c6d-7774-405162738439";
        :>> title = "Sprint 27 review gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: doc-sync skill follows SKILL.md shape; D0059/D0060 use the alternatives/criteria convention (D0058); CLAUDE.md doc-synced. Phase-3 scan: no avoidable issues; attribution-model EXECUTION correctly deferred to its own migration sprint (pre-authorized).";
    }
    part processDeployingSkillsReviewGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-8874-40516273843a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c844af"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processDeployingSkillsCloseOutGate : Test {
        :>> id = "61728394-0a5b-4c6d-9974-40516273843b";
        :>> title = "Sprint 27 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + backlog DoD recorded; validate_all green. No human gate (D0051: process/skill + recorded decisions, no schema).";
    }
    part processDeployingSkillsCloseOutGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-aa74-40516273843c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c844af"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processDeployingSkillsRetroGate : Test {
        :>> id = "61728394-0a5b-4c6d-bb74-40516273843d";
        :>> title = "Sprint 27 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049): no avoidable issues. The doc-sync skill closes the 'human must drive doc updates' friction (root of this sprint). Note: process<->skill coverage view (orphan-process) is a candidate for the formalViewpoints concern-coverage work. Estimate 2 pts held.";
    }
    part processDeployingSkillsRetroGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-cc74-40516273843e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c844af"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
