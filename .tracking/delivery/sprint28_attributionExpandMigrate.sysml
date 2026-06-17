// =====================================================================
// ProjectDeliveryS28 — Sprint 28: attribution-model EXPAND + MIGRATE-instances (5 pts).
// Story: deliver D0063 increment 1 — additively expand the schema (createdBy +
//        decision + rationale, [0..1]) and migrate every instance (createdBy
//        populated 330 sites; decisionText split into decision+rationale across
//        63 decisions). Old fields kept; removal is attrModelContract.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// NOTE (transparency): this sprint was FORMALIZED AFTER the implement work was
// already committed (44e1e83/693b5e6/88b7727/19f057a) — the work was initially
// run as raw backlog tasks without a sprint wrapper (issue020). The gates below
// record the ceremony that genuinely occurred this sitting (refine via D0061+D0063
// + the migration-shape acceptance; implement; validate/review), judged against the
// real commits. The lapse + its preventing changes are the retro output (issue020).
// =====================================================================
package ProjectDeliveryS28 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part attributionExpandMigrateStory : Story {
        :>> id = "62839405-1b6c-4d7e-d028-516273849563";
        :>> title = "Sprint 28: attribution-model expand + migrate-instances (5 pts) — D0063 increment 1 (attrModelSchema + attrModelInstances)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS28 {
        action storyAttributionExpandMigrate;
        verification storyAttributionExpandMigrateDoD : Test { :>> id = "62839405-1b6c-4d7e-e028-516273849564"; :>> method = VerificationMethod::inspect; :>> procedureText = "attrModelSchemaDoD (pass) + attrModelInstancesDoD (pass) recorded in backlog: schema expanded additively (createdBy on Element+3 Tracked* bases; decision+rationale on Decision; all [0..1]); createdBy populated 330 sites; decisionText split into decision+rationale across 63 decisions; old fields kept (removal = attrModelContract); validate_schema 14/14 + validate_instances 71/71 + validate . 36 clean + validate_actors green."; }
        part storyAttributionExpandMigrateDoDR1 : TestResult { :>> id = "62839405-1b6c-4d7e-f028-516273849565"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19f057a"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification attributionExpandMigrateRefineGate : Test {
        :>> id = "62839405-1b6c-4d7e-1128-516273849566";
        :>> title = "Sprint 28 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = attrModelSchema + attrModelInstances (D0063 increment 1). Approach refined across D0061 (decomposition) and D0063 (expand/migrate/contract shape, accepted by wweatherholtz when shown a rename can't commit without migrating 389+85 sites at once). DoDs exist (attrModelSchemaDoD re-scoped to additive expand; attrModelInstancesDoD to migrate). No blocking dependency (attrModelSchema gates attrModelInstances; both in scope). estimatedPoints=5 (schema change + 330-site populate + 63-decision split). DoD accepted.";
    }
    part attributionExpandMigrateRefineGateR1 : TestResult { :>> id = "62839405-1b6c-4d7e-2228-516273849567"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "44e1e83"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attributionExpandMigrateStandupGate : Test {
        :>> id = "62839405-1b6c-4d7e-3328-516273849568";
        :>> title = "Sprint 28 standup gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: expand schema additively first (commit green), then migrate — createdBy via a one-time idempotent script (populate_created_by.py), decisionText split via parallel agents (additive insertion, decisionText kept verbatim). No blockers.";
    }
    part attributionExpandMigrateStandupGateR1 : TestResult { :>> id = "62839405-1b6c-4d7e-4428-516273849569"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "44e1e83"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attributionExpandMigrateImplementGate : Test {
        :>> id = "62839405-1b6c-4d7e-5528-51627384956a";
        :>> title = "Sprint 28 implement gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "EXPAND (44e1e83): schema/core/element.sysml + work.sysml gain createdBy + decision + rationale ([0..1]); D0063 sign-off; validate_schema 14/14. MIGRATE: createdBy populated 330 sites/58 files (693b5e6, populate_created_by.py); decisionText split into decision+rationale across 63 decisions (88b7727, 7 parallel agents, additive — decisionText kept). validate_instances 71/71 + validate . 36 clean + validate_actors PASS. attrModelSchema + attrModelInstances backlog DoDs recorded pass.";
    }
    part attributionExpandMigrateImplementGateR1 : TestResult { :>> id = "62839405-1b6c-4d7e-6628-51627384956b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "88b7727"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attributionExpandMigrateReviewGate : Test {
        :>> id = "62839405-1b6c-4d7e-7728-51627384956c";
        :>> title = "Sprint 28 review gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: expand is additive ([0..1], old fields kept) so every step commits green; createdBy count == authoredBy count (330==330); 63/63 decisions have decision+rationale with decisionText intact; spot-checked 0001/0049/0061 for split fidelity + content preservation. doc-sync rode the schema change (sysmlv2-syntax-notes Decision-fields note). Phase-3 scan surfaced the avoidable issue -> see retro.";
    }
    part attributionExpandMigrateReviewGateR1 : TestResult { :>> id = "62839405-1b6c-4d7e-8828-51627384956d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "88b7727"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attributionExpandMigrateCloseOutGate : Test {
        :>> id = "62839405-1b6c-4d7e-9928-51627384956e";
        :>> title = "Sprint 28 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story DoD + attrModelSchema/attrModelInstances backlog DoDs recorded pass; validate_all green; deliverable re-verified after the append-gate-result source change (78a235c). No human gate (D0051: tested/migrated work, self-evidencing).";
    }
    part attributionExpandMigrateCloseOutGateR1 : TestResult { :>> id = "62839405-1b6c-4d7e-aa28-51627384956f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19f057a"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attributionExpandMigrateRetroGate : Test {
        :>> id = "62839405-1b6c-4d7e-bb28-516273849570";
        :>> title = "Sprint 28 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049). AVOIDABLE ISSUE (issue020): this migration's implement work was committed as raw backlog tasks with NO sprint wrapper — caught by the human, not a guard. validate_ceremony checks gate ORDER within a delivery file but not a MISSING delivery file. Root cause: §3e bootstrap exemption contradicts the sprint-everything practice; bootstrap is now over (tracking engine exists). PREVENTING CHANGES (queued, sprint29 via issue020->bootstrapExitEval): retire §3e (Decision) + CLAUDE.md anti-subversion + no-sprint guard (D0047) + per-interaction triage skill (engine-triage upgrade + UserPromptSubmit hook). WELL: expand/migrate/contract kept every step green and lossless (decisionText preserved); parallel-agent split was efficient + verified centrally. Estimate 5 pts held.";
    }
    part attributionExpandMigrateRetroGateR1 : TestResult { :>> id = "62839405-1b6c-4d7e-cc28-516273849571"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19f057a"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
