// ProjectDeliveryS199 — Sprint 199: serveTier2ApproveGate — approve-before-execute (srServeApproveGate).
// The execute path refuses to run without explicit approval of a reviewed PLAN; a /api/agent/plan endpoint
// computes the route+prompt without spawning; the console shows it + requires approval before executing.
// Structurally closes D0106's conversational residual (issue059). EXECUTE. 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS199 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveTier2bStory to d0109;

    #Capability part serveTier2bStory : Story {
        :>> id = "63940516-2c7d-4e8f-c199-51627384c001";
        :>> title = "Sprint 199: serveTier2ApproveGate (2 pt) - approve-before-execute (srServeApproveGate, Tier 2)";
        :>> createdAt = "2026-07-08"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }
    #DerivedFrom dependency from serveTier2bStory to nServeApproveBeforeExecute;

    action def DeliveryRunS199 {
        action storyServeTier2b;
        verification storyServeTier2bDoD : Test { :>> id = "63940516-2c7d-4e8f-c199-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Implements srServeApproveGate (Tier 2, on srServeLauncherDefinedOnly). Two-phase flow: (1) request_plan extracted (shared by plan+stream) computes action_ok/launch_undefined/prompt WITHOUT spawning; (2) NEW GET /api/agent/plan returns {action,target,executable,prompt,requires_approval} — the reviewable route; (3) AgentReq gains `approved` (serde default false); the execute stream now REFUSES unless approved (unapproved early-return: 'approval required — GET /api/agent/plan, review, re-invoke with approved=1'); (4) console runAgentModal rewritten to fetch the plan, show it, require an explicit approve (confirm) before opening the stream with approved=1. This makes approve-before-execute STRUCTURAL — the agent never runs on an unreviewed/unapproved route — closing D0106's conversational residual (issue059) at the interaction boundary, which a commit gate cannot. VERIFY: clippy -D warnings clean; cargo test (78 lib + serve) green; keel validate 214 + guard all (15) green; the existing critique flow now routes through plan->approve->execute."; }
        part storyServeTier2bDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c199-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ced45c"; :>> judgedAt = "2026-07-08"; :>> judgedBy = "claudeOpus"; }
    }

    #Verify dependency from storyServeTier2bDoD to srServeApproveGate;

    verification serveTier2bRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c199-51627384c004"; :>> title = "Sprint 199 refine gate (DoR)"; :>> createdAt = "2026-07-08"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: srServeLauncherDefinedOnly done; srServeApproveGate #DependsOn it — the highest-value Tier-2 slice (closes issue059 residual). estimatedPoints=2. #CharteredBy d0109. DoR met."; }
    part serveTier2bRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c199-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ced45c"; :>> judgedAt = "2026-07-08"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2bStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c199-51627384c006"; :>> title = "Sprint 199 standup gate"; :>> createdAt = "2026-07-08"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: extract request_plan (shared); add /api/agent/plan; add AgentReq.approved + the unapproved stream refusal; rewrite console runAgentModal to plan->show->approve->execute. No blockers."; }
    part serveTier2bStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c199-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ced45c"; :>> judgedAt = "2026-07-08"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2bImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c199-51627384c008"; :>> title = "Sprint 199 implement gate"; :>> createdAt = "2026-07-08"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "request_plan + /api/agent/plan + approved gate + console approve-flow landed. clippy -D clean; cargo test green (78 lib); keel validate + guard green."; }
    part serveTier2bImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c199-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ced45c"; :>> judgedAt = "2026-07-08"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2bReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c199-51627384c00a"; :>> title = "Sprint 199 review gate"; :>> createdAt = "2026-07-08"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. This is the highest-value D0109 slice: approve-before-execute is now structural (the execute stream refuses without approved=1; the plan endpoint gives the human exactly what they approve; the console enforces plan->approve->execute). It closes issue059's residual at the interaction boundary — the class of failure a commit gate can't see. Server gate is the enforceable core (tested); console is the UX. Honest note: approved=1 is a client-set flag on a localhost single-user tool — a plan-token hardening is a possible follow-on. Clean."; }
    part serveTier2bReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c199-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ced45c"; :>> judgedAt = "2026-07-08"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2bCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c199-51627384c00c"; :>> title = "Sprint 199 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-08"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; approve-before-execute is live + verifies srServeApproveGate; issue059's residual is now structurally closed at the boundary. clippy + cargo test + keel validate + guard green. Self-evidencing (D0051). NEXT: srServeOutputSchemaValidate (validate+repair), then Tier 3 srServeScopedContext."; }
    part serveTier2bCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c199-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ced45c"; :>> judgedAt = "2026-07-08"; :>> judgedBy = "claudeOpus"; }
    verification serveTier2bRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c199-51627384c00e"; :>> title = "Sprint 199 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-08"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the exact residual we could NOT gate at commit time (D0106's parse-first, issue059) is now STRUCTURALLY enforced at the interaction boundary — the frontend earned its keep for the one thing only it can do (per the D0109 research). Extracting request_plan made plan+stream share one truth (no drift between what's previewed and what runs). AVOIDABLE-ISSUE SCAN: the redundant-clone clippy lint (fixed). LESSON: a two-phase propose->approve->execute is the boundary-level analogue of the human sign-off gate. Estimate 2 pt held. NEXT: srServeOutputSchemaValidate (keel-validate the agent's change + repair loop) + Tier 3 srServeScopedContext. Possible hardening: a plan-token (vs a client flag) if this ever leaves single-user localhost."; }
    part serveTier2bRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c199-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ced45c"; :>> judgedAt = "2026-07-08"; :>> judgedBy = "claudeOpus"; }
}
