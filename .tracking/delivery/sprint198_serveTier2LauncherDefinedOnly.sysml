// ProjectDeliveryS198 — Sprint 198: serveTier2LauncherDefinedOnly — Tier 2 first slice (srServeLauncherDefinedOnly):
// generalize the serve agent-bridge from critique-only (sr17) to a model-driven LAUNCHER that runs any
// DECLARED process/skill, gated by is_launchable (no freeform launch). Approve-gate + output-validator are
// the remaining Tier-2 slices. EXECUTE. 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS198 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveTier2aStory to d0109;

    #Capability part serveTier2aStory : Story {
        :>> id = "63940516-2c7d-4e8f-c198-51627384c001";
        :>> title = "Sprint 198: serveTier2LauncherDefinedOnly (2 pt) - model-driven launcher, no freeform (srServeLauncherDefinedOnly, Tier 2)";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }
    #DerivedFrom dependency from serveTier2aStory to nServeLaunchDefinedOnly;

    action def DeliveryRunS198 {
        action storyServeTier2a;
        verification storyServeTier2aDoD : Test { :>> id = "63940516-2c7d-4e8f-c198-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Implements srServeLauncherDefinedOnly (Tier 2, on Tier-1 is_launchable). Generalized api_agent_stream: the bridge now accepts action=`launch` (D0109 model-driven launcher) in addition to `critique` (sr17) — supersedes sr17's one-action limit with the declared launchable set, STILL non-freeform. A launch target that is not is_launchable is rejected (launch_undefined early-return: 'not a declared launchable — see keel launchables; no freeform launch path'). build_launch_prompt directs the agent to execute the declared process/skill per its .engine definition, stay strictly within it, and NOT commit. VERIFIED: unit test launch_prompt_directs_a_declared_target_no_commit; is_launchable gate unit-tested in Tier 1 (launchable_set_is_processes_and_skills_only). clippy -D warnings clean; cargo test green; keel validate 213 + guard all (15) green. NOTE: approve-before-execute (srServeApproveGate) + output-validate/repair (srServeOutputSchemaValidate) are the remaining Tier-2 slices."; }
        part storyServeTier2aDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c198-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8598d7c"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    }

    #Verify dependency from storyServeTier2aDoD to srServeLauncherDefinedOnly;

    verification serveTier2aRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c198-51627384c004"; :>> title = "Sprint 198 refine gate (DoR)"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: Tier 1 (is_launchable) done; srServeLauncherDefinedOnly #DependsOn it — next tier. estimatedPoints=2. #CharteredBy d0109. DoR met."; }
    part serveTier2aRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c198-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8598d7c"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2aStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c198-51627384c006"; :>> title = "Sprint 198 standup gate"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add `launch` action + is_launchable gate + build_launch_prompt; reuse the existing spawn/SSE machinery; unit-test the prompt. Approve-gate + validator deferred to later slices. No blockers."; }
    part serveTier2aStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c198-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8598d7c"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2aImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c198-51627384c008"; :>> title = "Sprint 198 implement gate"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "launch action + gate + prompt landed, reusing spawn/SSE. clippy -D clean; cargo test launch_prompt green; keel validate + guard green."; }
    part serveTier2aImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c198-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8598d7c"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2aReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c198-51627384c00a"; :>> title = "Sprint 198 review gate"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The launcher reuses the proven spawn/SSE path (low risk) + the Tier-1 is_launchable gate (defense-in-depth: freeform launch is unrepresentable). Honestly supersedes sr17's one-action limit per accepted D0109, while preserving non-freeform. Scoped to one Tier-2 SR; approve-gate + validator are the next slices (not crammed). Clean."; }
    part serveTier2aReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c198-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8598d7c"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2aCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c198-51627384c00c"; :>> title = "Sprint 198 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; the model-driven launcher (defined-only) is live + verifies srServeLauncherDefinedOnly. clippy + cargo test + keel validate + guard green. Self-evidencing (D0051). NEXT Tier-2 slices: approve-gate, output-validator/repair."; }
    part serveTier2aCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c198-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8598d7c"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2aRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c198-51627384c00e"; :>> title = "Sprint 198 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the launcher was a SMALL change because Tier 1 (is_launchable) already existed + the spawn/SSE machinery was reused — the tiered build order paying off (no churn, minimal new surface). Non-freeform preserved (D0109 supersedes sr17's one-action limit cleanly). AVOIDABLE-ISSUE SCAN: none. LESSON: building the foundation (Tier 1) first made the launcher a thin, low-risk addition. Estimate 2 pt held. NEXT: srServeApproveGate (approve-before-execute — closes issue059's residual; needs a two-phase propose->approve->execute flow) then srServeOutputSchemaValidate (keel-validate output + repair loop) then Tier 3 srServeScopedContext."; }
    part serveTier2aRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c198-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8598d7c"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
}
