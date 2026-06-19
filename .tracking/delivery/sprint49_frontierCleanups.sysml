// =====================================================================
// ProjectDeliveryS49 — Sprint 49: frontierCleanups — close 3 decided-but-open frontier items.
//   - queryDrivenViewpoints (D0075): umbrella delivered via M1/tomlViewExecutor.
//   - actualHoursDisposition (D0072): sprint-closeout skill records actualHours = AI time, split
//     by owner; never fabricate/backfill.
//   - sittingReviewRecord (D0073): precise sitting-review check re-added to query.py audit
//     (matches the real sittingNNReviewRn artifact, sitting01).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS49 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // chartered by D0046 — the architectural-critique/audit framework drives surfacing + closing
    // these adherence/frontier items.
    #CharteredBy dependency from frontierCleanupsStory to d0046;

    part frontierCleanupsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b049-516273849501";
        :>> title = "Sprint 49: frontierCleanups (3 pts) — close queryDrivenViewpoints (M1) + actualHoursDisposition (D0072) + sittingReviewRecord (D0073)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS49 {
        action storyFrontierCleanups;
        verification storyFrontierCleanupsDoD : Test { :>> id = "63940516-2c7d-4e8f-b049-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "frontierCleanupsDoD (pass): (1) queryDrivenViewpoints recorded done — delivered via tomlViewExecutor/M1 per D0075 (umbrella). (2) actualHoursDisposition (D0072): sprint-closeout SKILL.md now specifies actualHours = AI wall-clock, split AI-vs-human by owner, never fabricate/backfill. (3) sittingReviewRecord (D0073): query.py audit re-adds a PRECISE sitting-review check matching the real `sittingNNReviewRn : TestResult (pass)` artifact (sitting01ReviewR1) — replaces the removed prose-grep; audit reports sitting_review.ok=true. validate_all green; audit runs clean."; }
        part storyFrontierCleanupsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b049-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "29db68a"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
    }

    verification frontierCleanupsRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b049-516273849503"; :>> title = "Sprint 49 refine gate (DoR)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: 3 small decided-but-open frontier items (each already DECIDED — D0075/D0072/D0073 — only impl/closure left). Low risk: a DoD record, a skill-doc refinement, a precise audit check. Bundled into one sprint (each ~1pt, total 3, within D0044 cap). No fabrication/backfill (D0067). DoD accepted."; }
    part frontierCleanupsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b049-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "29db68a"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification frontierCleanupsStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b049-516273849505"; :>> title = "Sprint 49 standup gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: record queryDrivenViewpoints DoD (M1-delivered); refine sprint-closeout SKILL.md actualHours step (D0072); re-add precise sitting-review check to query.py audit_report (match sittingNNReviewRn pass artifact). No blockers."; }
    part frontierCleanupsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b049-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "29db68a"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification frontierCleanupsImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b049-516273849507"; :>> title = "Sprint 49 implement gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "queryDrivenViewpoints + actualHoursDisposition + sittingReviewRecord DoDs recorded pass (the latter two judgedBy=wweatherholtz, citing their D0072/D0073 acceptances). sprint-closeout SKILL.md actualHours step refined (AI time, owner-split, no fabricate/backfill). query.py audit: sitting_review now {records:[sitting01ReviewR1], ok:true}. validate . 58 clean; audit clean. Story DoD recorded."; }
    part frontierCleanupsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b049-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "29db68a"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification frontierCleanupsReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b049-516273849509"; :>> title = "Sprint 49 review gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed: the sitting-review check is now PRECISE (matches the real artifact, not prose — the lesson from sprint44's false-positive applied). actualHours wiring honors D0072 (owner-split) + D0067 (no fabrication/backfill) — values accrue going forward, not retroactively invented. queryDrivenViewpoints closed honestly (delivered via M1, not re-done). Frontier is clear; M2 is next."; }
    part frontierCleanupsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b049-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "29db68a"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification frontierCleanupsCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b049-51627384950b"; :>> title = "Sprint 49 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): story + 3 task DoDs recorded pass; validate_all + audit green. No human gate (D0051: doc/tooling closure of already-accepted decisions, self-evidencing)."; }
    part frontierCleanupsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b049-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "29db68a"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification frontierCleanupsRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b049-51627384950d"; :>> title = "Sprint 49 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: bundling 3 tiny decided-items into one sprint avoided ceremony overhead while keeping coverage (all 3 named) + a charter edge. The sitting-review check came full circle — sprint44 removed the unreliable prose grep; now that D0073/sitting01 created a real artifact, a precise check replaces it. No avoidable issue. Next: M2 (port query.py views to Rust). Estimate 3 pts held."; }
    part frontierCleanupsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b049-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "29db68a"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
}
