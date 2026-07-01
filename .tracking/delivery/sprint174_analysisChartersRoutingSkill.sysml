// ProjectDeliveryS174 — Sprint 174: analysisChartersRoutingSkill (issue055, unblocked).
// Deliver issue055's CORE behavior fix: the engine-triage skill now ROUTES substantial analysis/design
// through a chartered WorkKind::research spike (chartered to an Issue or a status=proposed Decision — the
// charter guard already permits both). Stops sustained design leaking into conversation. The cosmetic
// charter-guard MESSAGE tweak + issue055's final close ride sprint175 (same guards.rs). EXECUTE. 1 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS174 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from analysisRoutingStory to issue055;

    part analysisRoutingStory : Story {
        :>> id = "63940516-2c7d-4e8f-c174-51627384c001";
        :>> title = "Sprint 174: analysisChartersRoutingSkill (1 pt) - engine-triage routes substantial analysis -> chartered research spike (issue055)";
        :>> createdAt = "2026-07-01"; :>> createdBy = "claudeOpus"; :>> kind = WorkKind::documentation; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS174 {
        action storyAnalysisRouting;
        verification storyAnalysisRoutingDoD : Test { :>> id = "63940516-2c7d-4e8f-c174-51627384c002"; :>> method = VerificationMethod::inspect; :>> procedureText = "issue055 core behavior fix. Added engine-triage checklist item 8 (Analysis/design -> a chartered research spike): substantial analysis/diagnosis/architectural design routes to a WorkKind::research spike chartered to an Issue or a status=proposed Decision (the charter guard already permits both — edge-existence, not target type), DoD = design artifact + a recorded proposed Decision; quick VIEW/ORIENT answers stay conversational. Skill version 0.2.0 -> 0.3.0 (doc-sync). This is the routing that stops sustained design leaking into chat (the issue054 defect recursed). PARTIAL: issue055's cosmetic charter-guard MESSAGE tweak + final resolution ride sprint175 (batched with the process-skill->workflows guards.rs change), so issue055 stays honestly OPEN until then. VERIFY: engine-triage SKILL.md item 8 present + version bumped; kernel validate_instances PASS (skill); keel validate + guard all green."; }
        part storyAnalysisRoutingDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c174-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9aa3951"; :>> judgedAt = "2026-07-01"; :>> judgedBy = "claudeOpus"; }
    }

    verification analysisRoutingRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c174-51627384c004"; :>> title = "Sprint 174 refine gate (DoR)"; :>> createdAt = "2026-07-01"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: issue055 unblocked (the guard already permits the charter; only convention + routing missing). Deliver the routing via the engine-triage skill. estimatedPoints=1. #CharteredBy issue055. DoR met."; }
    part analysisRoutingRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c174-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9aa3951"; :>> judgedAt = "2026-07-01"; :>> judgedBy = "claudeOpus"; }
    verification analysisRoutingStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c174-51627384c006"; :>> title = "Sprint 174 standup gate"; :>> createdAt = "2026-07-01"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add a triage routing item (analysis/design -> research spike) to engine-triage SKILL.md; bump version; defer the guards.rs message tweak to sprint175 (batch guards.rs changes). No blockers."; }
    part analysisRoutingStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c174-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9aa3951"; :>> judgedAt = "2026-07-01"; :>> judgedBy = "claudeOpus"; }
    verification analysisRoutingImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c174-51627384c008"; :>> title = "Sprint 174 implement gate"; :>> createdAt = "2026-07-01"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "engine-triage SKILL.md item 8 added (research-spike routing) + version 0.3.0. kernel validate_instances (skill) + keel validate + guard all green."; }
    part analysisRoutingImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c174-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9aa3951"; :>> judgedAt = "2026-07-01"; :>> judgedBy = "claudeOpus"; }
    verification analysisRoutingReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c174-51627384c00a"; :>> title = "Sprint 174 review gate"; :>> createdAt = "2026-07-01"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The routing rule is the real fix for issue055's behavior half — it makes 'sustained design gets a spike' a structural triage step, not vigilance. Correctly distinguishes quick VIEW/ORIENT (stay conversational) from substantial design (spike). Honest partial: issue055 stays open until the message tweak in 175. Clean."; }
    part analysisRoutingReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c174-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9aa3951"; :>> judgedAt = "2026-07-01"; :>> judgedBy = "claudeOpus"; }
    verification analysisRoutingCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c174-51627384c00c"; :>> title = "Sprint 174 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-01"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; engine-triage now routes analysis -> research spike. keel validate + guard all green. Self-evidencing (D0051). issue055 honestly OPEN pending the sprint175 message tweak."; }
    part analysisRoutingCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c174-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9aa3951"; :>> judgedAt = "2026-07-01"; :>> judgedBy = "claudeOpus"; }
    verification analysisRoutingRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c174-51627384c00e"; :>> title = "Sprint 174 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-01"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the routing fix is cheap + structural — one triage checklist item turns 'analysis leaks to chat' into 'analysis gets a spike'; and it was itself discovered BY the leak it fixes (issue055). AVOIDABLE-ISSUE SCAN: no new avoidable defect; the underlying avoidable pattern (design-in-chat) is now guarded by triage item 8. LESSON: batch same-file (guards.rs) changes across sprints to avoid redundant rebuilds — the charter-message tweak waits for sprint175. Estimate 1 pt held; issue055 deliberately left open until its message tweak lands in 175 (honest partial)."; }
    part analysisRoutingRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c174-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9aa3951"; :>> judgedAt = "2026-07-01"; :>> judgedBy = "claudeOpus"; }
}
