// ProjectDeliveryS197 — Sprint 197: serveTier1LaunchableRegistry — implement Tier 1 (srServeModelDrivenRegistry)
// of the D0109 serve-discipline tooling: the model-driven launchable set + a launchability guardrail.
// First code increment; build order = highest tier first (the launcher/validator read this). EXECUTE. 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS197 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveTier1Story to d0109;

    // #Capability (user-facing feature) -> must derive from a Need (requirement-rootedness guard, D0099).
    #Capability part serveTier1Story : Story {
        :>> id = "63940516-2c7d-4e8f-c197-51627384c001";
        :>> title = "Sprint 197: serveTier1LaunchableRegistry (2 pt) - model-driven launchable set (srServeModelDrivenRegistry, Tier 1)";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    #DerivedFrom dependency from serveTier1Story to nServeReuseModel;

    action def DeliveryRunS197 {
        action storyServeTier1;
        verification storyServeTier1DoD : Test { :>> id = "63940516-2c7d-4e8f-c197-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Implements Tier 1 (srServeModelDrivenRegistry) — the foundation the launcher + validator read (build-order: highest tier first). view::launchables computes the launchable set (processes + AISkills) from the DECLARED model (nServeReuseModel — no separate list); view::is_launchable is the guardrail (a Process/AISkill is launchable; anything else / unknown is not — the srServeLauncherDefinedOnly 'no freeform target' basis). `keel launchables [ROOT]` CLI + /api/launchables serve endpoint (cached). Output-contract note: for model-producing launches the 'expected structure' = the result keel validate+guard passes (existing authority, run pre-commit = generation-time); finer per-launchable schemas are a deferred should. VERIFIED LIVE: `keel launchables .` -> total 46 (from the model). D0047 control: unit test launchable_set_is_processes_and_skills_only. clippy -D warnings clean; cargo test green; keel validate 212 + guard all (15) green."; }
        part storyServeTier1DoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c197-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dea0f7"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    }

    // #Verify: this delivery verifies the registry (set-from-model) core of srServeModelDrivenRegistry.
    #Verify dependency from storyServeTier1DoD to srServeModelDrivenRegistry;

    verification serveTier1RefineGate : Test { :>> id = "63940516-2c7d-4e8f-c197-51627384c004"; :>> title = "Sprint 197 refine gate (DoR)"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: design complete (Business+Architecture); Tier 1 srServeModelDrivenRegistry is the highest-tier dependency (#DependsOn), build it first. estimatedPoints=2. #CharteredBy d0109. DoR met."; }
    part serveTier1RefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c197-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dea0f7"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier1StandupGate : Test { :>> id = "63940516-2c7d-4e8f-c197-51627384c006"; :>> title = "Sprint 197 standup gate"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: view::launchables (set from model) + view::is_launchable (guardrail) + keel launchables CLI + /api/launchables; unit-test the guardrail. Output-contract = keel-validate-as-schema for now (no frozen-core change). No blockers."; }
    part serveTier1StandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c197-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dea0f7"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier1ImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c197-51627384c008"; :>> title = "Sprint 197 implement gate"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "view::launchables + is_launchable + keel launchables + /api/launchables landed. LIVE: keel launchables -> 46. clippy -D clean; cargo test launchable_set green; keel validate + guard green."; }
    part serveTier1ImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c197-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dea0f7"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier1ReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c197-51627384c00a"; :>> title = "Sprint 197 review gate"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. First code increment built the RIGHT thing first (Tier 1 foundation, per the #DependsOn DAG + the human's no-churn rule): the launchable set is model-derived (nServeReuseModel, no drift-prone list) and the guardrail is the srServeLauncherDefinedOnly basis. Honest scope: this is the registry/set core of the SR; the launcher UI (Tier 2) + finer output schemas are next. #Capability #DerivedFrom nServeReuseModel keeps rootedness clean. Clean."; }
    part serveTier1ReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c197-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dea0f7"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier1CloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c197-51627384c00c"; :>> title = "Sprint 197 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; Tier 1 (the launchable registry) is built + verifies srServeModelDrivenRegistry's core. clippy + cargo test + keel validate + guard green. Self-evidencing (D0051). Tier 2 (launcher + approve-gate + output-validator) is next."; }
    part serveTier1CloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c197-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dea0f7"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
    verification serveTier1RetroGate : Test { :>> id = "63940516-2c7d-4e8f-c197-51627384c00e"; :>> title = "Sprint 197 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-07"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: implementation started at the correct tier (the #DependsOn DAG made 'what to build first' unambiguous — no guessing, no downstream churn), and the launchable set is model-derived (reuses the one source of truth). The output-contract question resolved without a frozen-core change (keel-validate-as-schema for model-producing launches), avoiding premature schema work. AVOIDABLE-ISSUE SCAN: the usual clippy lints (needless mut, long doc paragraph) on new code — the standard tax. LESSON: encoding build order as #DependsOn paid off immediately — the first code increment was chosen by the model, not debated. Estimate 2 pt held. NEXT: Tier 2 — srServeLauncherDefinedOnly (launch only via is_launchable) + srServeApproveGate (approve-before-execute — closes issue059's residual) + srServeOutputSchemaValidate (keel-validate output + repair loop)."; }
    part serveTier1RetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c197-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dea0f7"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "claudeOpus"; }
}
