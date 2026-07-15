// ProjectDeliveryS212 — Sprint 212: keelViewerScopeRevise — apply the human's scope feedback (D0115):
// supersede N-7 (single-user unauth), N-11 + N-14 (deferred), N-9 (refocused onto AI critique); remove
// the 3 now-moot SRs + backlog; add 2 new Needs (N-15 iterative AI critique, N-16 in-program modify/
// critique) PENDING a third acceptance gate. EXECUTE. 1 pt.
package ProjectDeliveryS212 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from keelViewerScopeStory to d0115;

    part keelViewerScopeStory : Story {
        :>> id = "63940516-2c7d-4e8f-c212-51627384c001";
        :>> title = "Sprint 212: keelViewerScopeRevise (1 pt) - apply D0115 scope cuts + add N-15/N-16 (pending acceptance)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS212 {
        action storyKeelViewerScopeRevise;
        verification storyKeelViewerScopeReviseDoD : Test { :>> id = "63940516-2c7d-4e8f-c212-51627384c002"; :>> method = VerificationMethod::inspect; :>> procedureText = "Applies the human's scope feedback (2026-07-14) as D0115 (accepted, human-judged). AUTHORED: (1) D0115 supersedes N-7 (nViewerMultiStakeholder — single-user, unauthenticated; saveable viewpoints = N-8), N-11 (nViewerNotifications) + N-14 (nViewerAuditProvenance — deferred), N-9 (nViewerCoverageGap — refocused onto AI critique) via #Supersede edges. (2) REMOVED the 3 now-moot SystemRequirements (srViewerCoverageGap/Notifications/AuditProvenance) + their satisfy edges + 3 backlog actions. (3) ADDED 2 new Needs — N-15 nViewerIterativeCritique (iterative AI critique over a constrained query, axis-iterated; reuses element-critique + agent bridge) + N-16 nViewerInProgramEdit (modify AND critique elements in-program, write-back via keel) — PENDING a third human acceptance gate (no architecture derived until accepted). Net: 10 live Needs (N-1..N-6, N-8, N-10, N-12, N-13) + 2 pending. VERIFY: keel validate clean (229); guard all + check-engine green (D0115 validated); the 4 Needs carry #Supersede edges; new Needs carry no acceptance yet."; }
        part storyKeelViewerScopeReviseDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c212-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    }

    verification ksrRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c212-51627384c004"; :>> title = "Sprint 212 refine gate (DoR)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: human scope feedback is explicit direction -> record D0115 + supersede the 4 Needs + add the 2 new. Confirm the new Needs before their architecture. estimatedPoints=1. #CharteredBy d0115. DoR met."; }
    part ksrRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c212-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification ksrStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c212-51627384c006"; :>> title = "Sprint 212 standup gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: D0115 + #Supersede edges; remove moot SRs/backlog; author N-15/N-16 unaccepted. No blockers."; }
    part ksrStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c212-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification ksrImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c212-51627384c008"; :>> title = "Sprint 212 implement gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "D0115 + 4 #Supersede edges + 3 SR removals + 3 backlog removals + 2 new Needs authored. keel validate clean (229); guard all + check-engine green."; }
    part ksrImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c212-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification ksrReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c212-51627384c00a"; :>> title = "Sprint 212 review gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Scope reduction handled the honest way: an accepted Need is not mutated in place — a superseding Decision (D0115) with #Supersede edges records the cut + the why, and the moot downstream (SRs + backlog) is removed so the burndown reflects the smaller true scope. The refocus (N-9 dashboard -> AI critique) captures the human's ACTUAL goal, which conveniently reuses keel's existing critique substrate. New Needs correctly held pending the acceptance gate. Clean."; }
    part ksrReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c212-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification ksrCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c212-51627384c00c"; :>> title = "Sprint 212 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; scope revised (4 Needs superseded, moot downstream removed, 2 new Needs added pending). keel validate + guard + check-engine green. Self-evidencing (D0051). GATE FOR THE HUMAN: confirm/accept N-15 + N-16 -> then derive their SRs + backlog (critique reuses element-critique/agent-bridge; in-program edit reuses apply-review)."; }
    part ksrCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c212-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification ksrRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c212-51627384c00e"; :>> title = "Sprint 212 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: human feedback that CUT scope was handled as first-class modeling — a superseding Decision + #Supersede edges + removal of the moot downstream — rather than silent deletion or in-place mutation; the model shrank honestly. Refocusing N-9 onto the human's real intent (AI critique) surfaced that the goal reuses existing keel substrate (element-critique/agent-bridge/apply-review) — cheaper than the dashboard would have been. AVOIDABLE-ISSUE SCAN: none. LESSON: 'won't do / defer / refocus' are Decisions, not edits — supersede + prune keeps the burndown truthful. Estimate 1 pt held. NEXT: human accepts N-15/N-16 -> their architecture + backlog."; }
    part ksrRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c212-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6dfa921"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
}
