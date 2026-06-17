// =====================================================================
// ProjectDeliveryS37 — Sprint 37: attrModelGuard (2 pts).
// Story: a guard (validate_acceptance_events.py) that fails if a status=accepted
//        Decision lacks a passing acceptance event (acceptance is a TestResult, not
//        prose/status; D0066/D0047). The enforcement twin of attestation-coverage.
//        Completes the attribution model (D0060-D0067).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049). Gate results via append-gate-result.
// =====================================================================
package ProjectDeliveryS37 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part attrModelGuardStory : Story {
        :>> id = "67940516-2c7d-4e8f-b037-516273849501";
        :>> title = "Sprint 37: attrModelGuard (2 pts) — acceptance-event guard (D0066/D0047)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS37 {
        action storyAttrModelGuard;
        verification storyAttrModelGuardDoD : Test { :>> id = "67940516-2c7d-4e8f-b037-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "attrModelGuardDoD (pass): validate_acceptance_events.py flags a status=accepted Decision lacking a passing acceptance event; --selftest passes (positive+negative); 0 violations at HEAD; wired into pre-commit on .engine/decisions; CLAUDE.md §5 lists it; validate_all green."; }
        part storyAttrModelGuardDoDR1 : TestResult { :>> id = "0d6cb118-74a3-48cb-a576-401582ec7575"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0149606"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification attrModelGuardRefineGate : Test {
        :>> id = "67940516-2c7d-4e8f-b037-516273849503";
        :>> title = "Sprint 37 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = attrModelGuard (D0061) — the D0047 enforcement of D0066 (acceptance is a TestResult, not prose). Enforcement twin of attrModelCoverageView. Pattern: validate_sprint_coverage (kernel-free guard + --selftest + pre-commit). No grandfather needed (all 66 accepted decisions migrated to events). No blocking dependency (attrModelCoverageView done). estimatedPoints=2. DoD accepted.";
    }
    part attrModelGuardRefineGateR1 : TestResult { :>> id = "87cb99d9-5aa0-4dbe-a7b8-22a9d5a8807e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0ef8f28"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelGuardStandupGate : Test {
        :>> id = "67940516-2c7d-4e8f-b037-516273849504";
        :>> title = "Sprint 37 standup gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: validate_acceptance_events.py (glob decisions; flag status=accepted lacking a passing dNNNNAcceptR1) + --selftest; wire into .githooks/pre-commit on .engine/decisions; CLAUDE.md §5 row. No blockers.";
    }
    part attrModelGuardStandupGateR1 : TestResult { :>> id = "01b8746a-41f4-4612-a7b8-22a9d8d32c86"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0ef8f28"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelGuardImplementGate : Test {
        :>> id = "67940516-2c7d-4e8f-b037-516273849505";
        :>> title = "Sprint 37 implement gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "validate_acceptance_events.py built (kernel-free). --selftest PASS (positive: accepted+event passes; negative: accepted-no-event flagged d9002). Live: 66 accepted, 0 violations. Wired into .githooks/pre-commit on .engine/decisions/; CLAUDE.md §5 lists it. attrModelGuard backlog DoD recorded pass. Completes the attribution model (D0060-D0067).";
    }
    part attrModelGuardImplementGateR1 : TestResult { :>> id = "3f33ae13-7be5-4c62-b0e3-89a5ecc9deda"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0149606"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelGuardReviewGate : Test {
        :>> id = "67940516-2c7d-4e8f-b037-516273849506";
        :>> title = "Sprint 37 review gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: guard mirrors validate_sprint_coverage shape; shares the acceptance-event regex with the coverage view (view computes, guard blocks); no grandfather (all migrated). Phase-3 scan: D0047 loop on acceptance is now closed — a future accepted Decision with no event fails pre-commit. The full attribution model (data + acceptance) is now both migrated AND guarded.";
    }
    part attrModelGuardReviewGateR1 : TestResult { :>> id = "9babd94b-532f-467e-b0e3-89a5f1e764be"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0149606"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelGuardCloseOutGate : Test {
        :>> id = "67940516-2c7d-4e8f-b037-516273849507";
        :>> title = "Sprint 37 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + attrModelGuard DoD recorded pass; validate_all green; the guard runs clean in pre-commit. No human gate (D0051: a tested guard, self-evidencing).";
    }
    part attrModelGuardCloseOutGateR1 : TestResult { :>> id = "e9245b62-d784-488f-a576-40157a6f03e5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0149606"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelGuardRetroGate : Test {
        :>> id = "67940516-2c7d-4e8f-b037-516273849508";
        :>> title = "Sprint 37 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) -- avoidable-issue scan. No avoidable issue. Process improvement: acceptance-is-an-event is now an automated guard (D0047 — a control, not vigilance), paired with the coverage view. The attribution-model epic (D0060->D0067: data migration + acceptance events + view + guard) is COMPLETE. Remaining frontier is the two design gates (processGovernanceLineage, queryDrivenViewpoints) that need human acceptance. Estimate 2 pts held.";
    }
    part attrModelGuardRetroGateR1 : TestResult { :>> id = "c58a3f85-18a1-47df-a576-40157e4bca75"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0149606"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
