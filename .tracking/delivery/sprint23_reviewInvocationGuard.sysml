// =====================================================================
// ProjectDeliveryS23 — Sprint 23: reviewInvocationGuard (2 pts).
// Story: extend validate_ceremony.py so a passing Retro gate must record avoidable-issue
//        SCAN EVIDENCE, not an empty rubber-stamp (issue011 — the S16 "scan skipped" gap).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049); test-backed, nothing to confirm (D0051).
// =====================================================================
package ProjectDeliveryS23 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part reviewInvocationGuardStory : Story {
        :>> id = "9f861375-df21-4f6e-bd47-8e0f20417121";
        :>> title = "Sprint 23: reviewInvocationGuard (2 pts) — guard that the retro avoidable-issue scan actually ran";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS23 {
        action storyReviewInvocationGuard;
        verification storyReviewInvocationGuardDoD : Test { :>> id = "a0972486-e032-4f6e-ce48-9f1031528121"; :>> method = VerificationMethod::test; :>> procedureText = "reviewInvocationGuardDoD in backlog (pass): validate_ceremony.py flags a passing Retro gate whose verification text records no scan evidence; positive + negative tested; wired in pre-commit."; }
        part storyReviewInvocationGuardDoDR1 : TestResult { :>> id = "b1083597-f143-4f6e-fe49-0a2142639121"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b193c26"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    verification reviewInvocationGuardRefineGate : Test {
        :>> id = "c2194608-0254-4f6e-3574-1b3253740121";
        :>> title = "Sprint 23 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach: D0049 already makes the retro autonomous (the scan always runs at close), so the residual issue011 risk is an EMPTY/rubber-stamp retro. Guard it: validate_ceremony.py flags a delivery file whose Retro gate passed but whose retro verification procedureText records no avoidable-issue scan evidence (keywords: avoidable/improvement/retro held/no avoidable/process improvement). Pure-Python, already in pre-commit. estimatedPoints=2. DoD accepted.";
    }
    part reviewInvocationGuardRefineGateR1 : TestResult { :>> id = "d32a5719-1365-4f6e-4685-2c4364850121"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b193c26"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification reviewInvocationGuardStandupGate : Test {
        :>> id = "e43b682a-2476-4f6e-5796-3d5475960121";
        :>> title = "Sprint 23 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. reviewInvocationGuard: retro-scan-evidence check in validate_ceremony.py. No blockers.";
    }
    part reviewInvocationGuardStandupGateR1 : TestResult { :>> id = "f54c793b-3587-4f6e-689a-4e6586a70121"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b193c26"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification reviewInvocationGuardImplementGate : Test {
        :>> id = "065d804c-4698-4f6e-79ab-5f7697b80121";
        :>> title = "Sprint 23 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "validate_ceremony.py gains retro_scan_missing() + a per-file check: Retro passed but no scan evidence -> ERROR. Positive test: all 24 delivery files PASS (existing retros carry evidence; sprint11 grandfathered). Negative test: empty rubber-stamp retro -> missing=True (caught), evidence retro -> False. Pure-Python; already wired in pre-commit.";
    }
    part reviewInvocationGuardImplementGateR1 : TestResult { :>> id = "176e915d-57a9-4f6e-8abc-6a87a8c90121"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b193c26"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification reviewInvocationGuardReviewGate : Test {
        :>> id = "287a026e-68ba-4f6e-9bcd-7b98b9da0121";
        :>> title = "Sprint 23 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: guard is keyword-evidence (lightweight but a real backstop against empty retros); D0049 already prevents skip, this prevents rubber-stamp. Phase-3 scan (this very sprint, via the guard's own discipline): no avoidable issues; pure-python so no deliverable re-suspect.";
    }
    part reviewInvocationGuardReviewGateR1 : TestResult { :>> id = "398b137f-79cb-4f6e-acde-8ca9caeb0121"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b193c26"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification reviewInvocationGuardCloseOutGate : Test {
        :>> id = "4a9c248a-8adc-4f6e-bdef-9dba0bdc0121";
        :>> title = "Sprint 23 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + backlog DoD recorded; .tracking validates green. No human gate (D0051: test-backed).";
    }
    part reviewInvocationGuardCloseOutGateR1 : TestResult { :>> id = "5a9d359b-9bed-4f6e-cdef-aecb1ced0121"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b193c26"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification reviewInvocationGuardRetroGate : Test {
        :>> id = "6a9e469c-9bed-4f6e-defa-bedc2ede0121";
        :>> title = "Sprint 23 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049): no avoidable issues — the guard self-applies (this sprint's retro carries scan evidence). Note: the keyword-based evidence check is lightweight; a future enrichment could tie retro findings to actual tracked issueNNN refs. Estimate 2 pts held.";
    }
    part reviewInvocationGuardRetroGateR1 : TestResult { :>> id = "7aaf57ad-9bed-4f6e-efab-cfed3fef0121"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b193c26"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
}
