// ProjectDeliveryS167 — Sprint 167: orphanStoryCharterBackfill (B2) — backfill #CharteredBy on the 37
// pre-D0068 orphan Stories (no charter edge; rootedness orphan burndown). Each is retroactively chartered
// to the backlog item it delivered (<X>Story -> <X> action; D0068 "originating backlog item"); 4 whose
// action name differs are chartered to their lead Decision (recorded basis, not fabricated). Bulk migration
// (D0067). orphan 37 -> 0. EXECUTE. 2 pts. AUTONOMOUS (D0049).
package ProjectDeliveryS167 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from orphanCharterBackfillStory to orphanStoryCharterBackfill;

    // B2 backfill: retroactive #CharteredBy for the 37 pre-D0068 orphan Stories (control total 37 -> 0).
    // 33 -> the backlog action they delivered; 4 (archDecisions/attributionExpandMigrate/nativeSpikes/
    // toolchainAndBaseline) -> their lead Decision (the action name differs). Basis: each sprint's name +
    // content; no fabricated origin (migration grandfather-with-basis, D0067).
    #CharteredBy dependency from archDecisionsStory to d0034;
    #CharteredBy dependency from attrModelAcceptanceEventsStory to attrModelAcceptanceEvents;
    #CharteredBy dependency from attrModelContractStory to attrModelContract;
    #CharteredBy dependency from attrModelCoverageViewStory to attrModelCoverageView;
    #CharteredBy dependency from attrModelGuardStory to attrModelGuard;
    #CharteredBy dependency from attrModelToolingStory to attrModelTooling;
    #CharteredBy dependency from attributionExpandMigrateStory to d0063;
    #CharteredBy dependency from auditSkillStory to auditSkill;
    #CharteredBy dependency from authoredByRefsStory to authoredByRefs;
    #CharteredBy dependency from backlogPrioritizationStory to backlogPrioritization;
    #CharteredBy dependency from decisionAnalysisStory to decisionAnalysis;
    #CharteredBy dependency from deliverableSuspicionStory to deliverableSuspicion;
    #CharteredBy dependency from deliverySkillsStory to deliverySkills;
    #CharteredBy dependency from foldWhatsNextStory to foldWhatsNext;
    #CharteredBy dependency from formalViewpointsSchemaStory to formalViewpointsSchema;
    #CharteredBy dependency from formalViewpointsStory to formalViewpoints;
    #CharteredBy dependency from issueLoopStory to issueLoop;
    #CharteredBy dependency from migrationProcessStory to migrationProcess;
    #CharteredBy dependency from nativeSpikesStory to d0031;
    #CharteredBy dependency from noSprintGuardStory to noSprintGuard;
    #CharteredBy dependency from processDeployingSkillsStory to processDeployingSkills;
    #CharteredBy dependency from reviewInvocationGuardStory to reviewInvocationGuard;
    #CharteredBy dependency from runtimeParserStory to runtimeParser;
    #CharteredBy dependency from rustS0WorkspaceStory to rustS0Workspace;
    #CharteredBy dependency from rustS1LexerStory to rustS1Lexer;
    #CharteredBy dependency from rustS2ParserStory to rustS2Parser;
    #CharteredBy dependency from rustS3SemanticStory to rustS3Semantic;
    #CharteredBy dependency from rustS4SpecCompatStory to rustS4SpecCompat;
    #CharteredBy dependency from rustS5CLIStory to rustS5CLI;
    #CharteredBy dependency from rustS6OrientStory to rustS6Orient;
    #CharteredBy dependency from rustS7QueryAlignStory to rustS7QueryAlign;
    #CharteredBy dependency from rustToolchainFixStory to rustToolchainFix;
    #CharteredBy dependency from specVersionRuntimeCheckStory to specVersionRuntimeCheck;
    #CharteredBy dependency from toolchainAndBaselineStory to d0041;
    #CharteredBy dependency from traceabilityViewStory to traceabilityView;
    #CharteredBy dependency from triageRouterSkillStory to triageRouterSkill;
    #CharteredBy dependency from writeApiStory to writeApi;

    part orphanCharterBackfillStory : Story {
        :>> id = "63940516-2c7d-4e8f-c167-51627384c001";
        :>> title = "Sprint 167: orphanStoryCharterBackfill (2 pts, B2) - backfill #CharteredBy on 37 pre-D0068 stories; orphan 37 -> 0";
        :>> createdAt = "2026-06-25"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::modeling; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS167 {
        action storyOrphanCharterBackfill;
        verification storyOrphanCharterBackfillDoD : Test { :>> id = "63940516-2c7d-4e8f-c167-51627384c002"; :>> method = VerificationMethod::inspect; :>> procedureText = "B2 (pre-existing debt): 37 orphan Stories carried no #CharteredBy edge (pre-D0068 convention; rootedness orphan=37). Non-blocking (the hard gate is #Capability-only, 0 violations), but a real traceability-completeness gap. FIX (migration, D0067): retroactively charter each orphan Story - 33 to the backlog action it delivered (StoryName minus 'Story' -> action, D0068 originating-backlog-item); 4 whose action name differs to their lead Decision (archDecisionsStory->d0034, attributionExpandMigrateStory->d0063, nativeSpikesStory->d0031, toolchainAndBaselineStory->d0041). Basis = each sprint's name + content; no fabricated origin. Control total: orphan 37 -> 0. VERIFIED: keel rootedness orphan -> 0 (all now decision-chartered, the legitimate D0064 class); keel validate + guard all (14) green."; }
        part storyOrphanCharterBackfillDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c167-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b268730"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "claudeOpus"; }
    }

    verification orphanCharterBackfillRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c167-51627384c004"; :>> title = "Sprint 167 refine gate (DoR)"; :>> createdAt = "2026-06-25"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: B2 = backfill #CharteredBy on 37 orphan Stories. Bulk migration; StoryName->action maps 33 mechanically (action existence verified), 4 to lead decisions. estimatedPoints=2. #CharteredBy orphanStoryCharterBackfill. DoR met."; }
    part orphanCharterBackfillRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c167-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b268730"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "claudeOpus"; }
    verification orphanCharterBackfillStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c167-51627384c006"; :>> title = "Sprint 167 standup gate"; :>> createdAt = "2026-06-25"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: derive each orphan's charter target (suffix-stripped action, verified to exist; 4 overrides to lead Decision), author all 37 #CharteredBy edges in one migration file (cross-package edges resolve in the global model). Control total 37->0. No blockers."; }
    part orphanCharterBackfillStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c167-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b268730"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "claudeOpus"; }
    verification orphanCharterBackfillImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c167-51627384c008"; :>> title = "Sprint 167 implement gate"; :>> createdAt = "2026-06-25"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "37 #CharteredBy edges authored (33 -> action, 4 -> lead decision). VERIFIED: keel rootedness orphan 37 -> 0; keel validate + guard all (14) green; charter guard clean (sprint167's own story is chartered to orphanStoryCharterBackfill)."; }
    part orphanCharterBackfillImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c167-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b268730"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "claudeOpus"; }
    verification orphanCharterBackfillReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c167-51627384c00a"; :>> title = "Sprint 167 review gate"; :>> createdAt = "2026-06-25"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Honest retroactive charter: each edge targets the real backlog item the sprint delivered (or its lead decision for the 4) - basis recorded, nothing fabricated. These were genuinely decision-driven engine-build sprints (D0064), so the resulting decision-chartered classification is accurate. Non-blocking debt cleared. Clean."; }
    part orphanCharterBackfillReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c167-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b268730"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "claudeOpus"; }
    verification orphanCharterBackfillCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c167-51627384c00c"; :>> title = "Sprint 167 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-25"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; rootedness orphan 37 -> 0 (all delivery Stories now need-rooted or decision-chartered). keel validate + guard all (14) green. Self-evidencing (D0051). B2 done -> B1+B2 complete; remaining frontier = releaseTag010 (+ 6 prose-only refs surfaced by the new guard, non-urgent)."; }
    part orphanCharterBackfillCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c167-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b268730"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "claudeOpus"; }
    verification orphanCharterBackfillRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c167-51627384c00e"; :>> title = "Sprint 167 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-25"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the StoryName->action naming convention made 33/37 charters mechanical + verifiable (action existence checked), only 4 needing a content-based lead-decision charter - a clean bulk migration with a control total (37->0). AVOIDABLE-ISSUE SCAN: no avoidable defect; the orphan set was honest pre-existing debt (pre-D0068), now cleared with recorded basis (no fabrication). OBSERVATION: a central migration file holding cross-package #CharteredBy edges works because the model aggregates edges globally; the per-sprint-file convention remains the norm, this backfill file is an acceptable one-off. Estimate 2 pts held. NEXT: releaseTag010 (v0.1.0, human sign-off) is the sole remaining outstanding item."; }
    part orphanCharterBackfillRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c167-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b268730"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "claudeOpus"; }
}
