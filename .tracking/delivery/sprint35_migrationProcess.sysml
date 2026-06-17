// =====================================================================
// ProjectDeliveryS35 — Sprint 35: migrationProcess (3 pts).
// Story: codify the safe bulk-migration process (D0067) — a process + deploying
//        skill encoding expand/migrate/contract + the edge-case checklist +
//        the historical-data rule, grounded in research.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049). Gate results via append-gate-result.
// =====================================================================
package ProjectDeliveryS35 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part migrationProcessStory : Story {
        :>> id = "67940516-2c7d-4e8f-b035-516273849501";
        :>> title = "Sprint 35: migrationProcess (3 pts) — safe bulk-migration process + deploying skill (D0067)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS35 {
        action storyMigrationProcess;
        verification storyMigrationProcessDoD : Test { :>> id = "67940516-2c7d-4e8f-b035-516273849502"; :>> method = VerificationMethod::inspect; :>> procedureText = "migrationProcessDoD (pass): D0067 recorded (+ acceptance event); .engine/processes/migration.sysml (Process + 7 ProcessSteps) + .engine/skills/migration/SKILL.md (gates + edge-case checklist + historical-data rule) created + registered; engine-triage routes bulk migrations to it; CLAUDE.md §4 references it; validate_instances green."; }
    }

    verification migrationProcessRefineGate : Test {
        :>> id = "67940516-2c7d-4e8f-b035-516273849503";
        :>> title = "Sprint 35 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = codify the safe-migration process (D0040: the 3 bulk migrations this session recur -> deploying-skill-backed process). Research-grounded (two agents: Fowler ParallelChange, Ambler Refactoring Databases, Google LSC, event upcasting, Flyway/Liquibase, ETL control totals, provenance integrity). Accepted by the human ('Process + deploying skill + Decision'). D0059-conformant (process + skill). No blocking dependency. estimatedPoints=3. DoD accepted.";
    }
    part migrationProcessRefineGateR1 : TestResult { :>> id = "57023c2c-4acc-4b15-8439-783309821f9f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "70f9484"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification migrationProcessStandupGate : Test {
        :>> id = "67940516-2c7d-4e8f-b035-516273849504";
        :>> title = "Sprint 35 standup gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: D0067 (decision + dogfood acceptance event); migration.sysml (Process + 7 ProcessSteps = the gates); migration/SKILL.md (gates + anti-patterns + edge-case checklist); register in skills-registry; doc-sync (engine-triage routing + CLAUDE.md §4). No blockers.";
    }
    part migrationProcessStandupGateR1 : TestResult { :>> id = "7d75b64c-2b4c-4479-8439-78330c8e960f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "70f9484"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification migrationProcessImplementGate : Test {
        :>> id = "67940516-2c7d-4e8f-b035-516273849505";
        :>> title = "Sprint 35 implement gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "D0067 recorded (+ d0067Accept event). .engine/processes/migration.sysml (Process migration + 7 ProcessSteps mig0Justify..mig6Contract). .engine/skills/migration/SKILL.md (7 gates + anti-patterns + edge-case checklist + output format). Registered (migration : AISkill). engine-triage gains a bulk-migration routing bullet; CLAUDE.md §4 gains the migration-process rule. validate_instances green. migrationProcess backlog DoD recorded pass.";
    }

    verification migrationProcessReviewGate : Test {
        :>> id = "67940516-2c7d-4e8f-b035-516273849506";
        :>> title = "Sprint 35 review gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the process/skill encode exactly the gates the research converged on + the edge cases we actually hit this session (regex-miss -> AST/reconcile; createdBy [1..1] -> don't-hoist-unsatisfiable; fabricate -> grandfather/backfill-with-basis). D0059-conformant (process has a deploying skill). Phase-3 scan: the reconcile gate makes the silent-miss catch structural, not luck — the lesson from the contract step's regex bug.";
    }

    verification migrationProcessCloseOutGate : Test {
        :>> id = "67940516-2c7d-4e8f-b035-516273849507";
        :>> title = "Sprint 35 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + migrationProcess DoD recorded pass; validate_all green; D0067 sign-off recorded (+ dogfooded). No human gate beyond the design acceptance already given (D0051).";
    }

    verification migrationProcessRetroGate : Test {
        :>> id = "67940516-2c7d-4e8f-b035-516273849508";
        :>> title = "Sprint 35 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) -- avoidable-issue scan. No new avoidable issue. Process improvement: the edge-case class that bit us (silent regex miss, unsafe tighten, fabricate-risk) is now a codified, deploying-skill-backed, research-grounded gated process -- caught structurally (reconcile gate) rather than by luck. Candidate follow-on (not blocking): a reusable reconciliation helper (control-totals) tool so the dry-run gate is one command. Estimate 3 pts held.";
    }
}
