// ProjectDeliveryS217 — Sprint 217: keelViewerTemporalRefocus — apply the Q2 answer (D0116): no
// due-date/overdue STATE (supersede N-3); temporal = time-as-query-filter (N-5). Remove srViewerTemporalState
// + satisfy + backlog. Model-only scope refocus. EXECUTE. 1 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS217 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from keelViewerTemporalStory to d0116;

    part keelViewerTemporalStory : Story {
        :>> id = "63940516-2c7d-4e8f-c217-51627384c001";
        :>> title = "Sprint 217: keelViewerTemporalRefocus (1 pt) - D0116: supersede N-3 (due-state), temporal = time-as-query-filter (N-5)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS217 {
        action storyKeelViewerTemporalRefocus;
        verification storyKeelViewerTemporalRefocusDoD : Test { :>> id = "63940516-2c7d-4e8f-c217-51627384c002"; :>> method = VerificationMethod::inspect; :>> procedureText = "Applies the human's Q2 answer as D0116 (accepted, human-judged): no due-date/overdue STATE machine. AUTHORED: D0116 supersedes N-3 (nViewerTemporalState) via #Supersede; the temporal need is redefined as TIME-AS-QUERY-FILTER, already covered by N-5 (nViewerQueryExpansion / srViewerDatePredicates: date/range predicates over date attributes like judgedAt). REMOVED the moot srViewerTemporalState + its satisfy edge + the viewerTemporalState backlog action. UC-4 (business-process state) stays — re-served by querying ('not reviewed since <date>') not a computed overdue flag. VERIFY: keel validate clean; guard all + check-engine green (D0116 validated); N-3 carries a #Supersede edge; no dangling srViewerTemporalState references."; }
        part storyKeelViewerTemporalRefocusDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c217-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    }

    verification vtrRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c217-51627384c004"; :>> title = "Sprint 217 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: Q2 answered — no due dates; temporal = query-time-filter (N-5). Supersede N-3 + remove its downstream. estimatedPoints=1. #CharteredBy d0116. DoR met."; }
    part vtrRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c217-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vtrStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c217-51627384c006"; :>> title = "Sprint 217 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: D0116 + #Supersede N-3; remove srViewerTemporalState + satisfy + backlog. No blockers."; }
    part vtrStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c217-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vtrImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c217-51627384c008"; :>> title = "Sprint 217 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "D0116 + #Supersede N-3 + removals landed. keel validate clean; guard all + check-engine green."; }
    part vtrImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c217-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vtrReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c217-51627384c00a"; :>> title = "Sprint 217 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The Q2 answer avoided building an unwanted due/overdue state machine — instead it collapsed the temporal need into the already-accepted N-5 (date/range query predicates over authored judgedAt facts). Honest §2.1 outcome: don't compute/store a state nobody asked for; query the facts that already exist. Supersede + prune keeps the model truthful. Clean."; }
    part vtrReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c217-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vtrCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c217-51627384c00c"; :>> title = "Sprint 217 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; N-3 superseded, temporal refocused onto N-5. keel validate + guard + check-engine green. Self-evidencing (D0051). NEXT: BUILD N-5 — date/range query predicates (first increment: a date filter on the explore surface)."; }
    part vtrCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c217-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vtrRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c217-51627384c00e"; :>> title = "Sprint 217 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: asking Q2 BEFORE building temporal saved building the wrong thing — the human didn't want a due/overdue state machine at all, just queryable time; the need collapsed into N-5 (query predicates) with zero new state. That's the whole point of needs-before-architecture + open-questions: surface the assumption, let the human correct it, don't build speculatively. AVOIDABLE-ISSUE SCAN: none. LESSON: a blocked need's open question is worth asking — the answer here DELETED a must-have SR rather than implementing it. Estimate 1 pt held. NEXT: N-5 date/range query filter (the actual want)."; }
    part vtrRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c217-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "713fa14"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
