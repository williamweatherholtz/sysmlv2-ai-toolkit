// =====================================================================
// ProjectDeliveryS21 — Sprint 21: backlogPrioritization (2 pts).
// Story: rank the ready frontier by backlog declaration order (D0052) so the AI
//        auto-follows the top item instead of asking which to do.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049); test-backed, nothing to confirm (D0051).
// =====================================================================
package ProjectDeliveryS21 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part backlogPrioritizationStory : Story {
        :>> id = "7d649153-bd0f-4f6e-bd40-6c8d0e2f3e4b";
        :>> title = "Sprint 21: backlogPrioritization (2 pts) — rank ready by declaration order (D0052)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS21 {
        action storyBacklogPrioritization;
        verification storyBacklogPrioritizationDoD : Test { :>> id = "8e750264-ce10-4f6e-ce41-7d9e1f306141"; :>> method = VerificationMethod::test; :>> procedureText = "backlogPrioritizationDoD in backlog (pass): orient ranks ready by declaration order; cargo test + clippy green; D0052 recorded."; }
        part storyBacklogPrioritizationDoDR1 : TestResult { :>> id = "9f861375-df21-4f6e-fe42-8e0f20417141"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90598cc"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    verification backlogPrioritizationRefineGate : Test {
        :>> id = "a0972486-e032-4f6e-2463-9f1031528241";
        :>> title = "Sprint 21 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach: add TaskData.order (declaration index) in indexer.rs; orient.rs ranks ready by it; query.py aligned (insertion order); D0052 records backlog-order = priority + AI auto-follows top sign-off-free item. estimatedPoints=2. DoD accepted.";
    }
    part backlogPrioritizationRefineGateR1 : TestResult { :>> id = "b1083597-f143-4f6e-3574-0a2142639241"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90598cc"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification backlogPrioritizationStandupGate : Test {
        :>> id = "c2194608-0254-4f6e-4685-1b3253740241";
        :>> title = "Sprint 21 standup gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. backlogPrioritization: indexer order + orient rank + query.py + D0052. No blockers.";
    }
    part backlogPrioritizationStandupGateR1 : TestResult { :>> id = "d32a5719-1365-4f6e-5796-2c4364850241"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90598cc"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification backlogPrioritizationImplementGate : Test {
        :>> id = "e43b682a-2476-4f6e-689a-3d5475960241";
        :>> title = "Sprint 21 implement gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "TaskData.order added; orient ready sorted by order; query.py ready insertion-order; cargo test + clippy -D warnings green; sysmlv2 orient ready = trackedMetadata, specVersionRuntimeCheck, reviewInvocationGuard (backlog order, not alphabetical).";
    }
    part backlogPrioritizationImplementGateR1 : TestResult { :>> id = "f54c793b-3587-4f6e-79ab-4e6586a70241"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90598cc"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification backlogPrioritizationReviewGate : Test {
        :>> id = "065d804c-4698-4f6e-8abc-5f7697b80241";
        :>> title = "Sprint 21 review gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: ranking correct + clippy-clean. Phase-3 scan: this sprint changes deliverable source (orient.rs/indexer.rs), so deliverable-suspicion (D0050) will re-flag the rust tasks — re-verify follows (intended source-change loop), not an avoidable issue.";
    }
    part backlogPrioritizationReviewGateR1 : TestResult { :>> id = "176e915d-57a9-4f6e-9bcd-6a87a8c90241"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90598cc"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification backlogPrioritizationCloseOutGate : Test {
        :>> id = "287a026e-68ba-4f6e-acde-7b98b9da1241";
        :>> title = "Sprint 21 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): storyBacklogPrioritizationDoD + backlog DoD recorded; .tracking validates green. No human gate (D0051: test-backed).";
    }
    part backlogPrioritizationCloseOutGateR1 : TestResult { :>> id = "398b137f-79cb-4f6e-bdef-8ca9caeb1241"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90598cc"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification backlogPrioritizationRetroGate : Test {
        :>> id = "4a9c248a-8adc-4f6e-cdef-9dba0bdc1241";
        :>> title = "Sprint 21 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049): avoidable-issue scan — the AI was asking 'which item?' because the frontier was unranked; fixed at root (D0052 ranking). Source-change re-suspect is expected (D0050). Estimate 2 pts held.";
    }
    part backlogPrioritizationRetroGateR1 : TestResult { :>> id = "5a9d359b-9bed-4f6e-defa-aecb1ced1241"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90598cc"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
}
