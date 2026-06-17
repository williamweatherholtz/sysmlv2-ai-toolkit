// =====================================================================
// ProjectDeliveryS30 — Sprint 30: noSprintGuard (2 pts).
// Story: the D0047 enforcement for issue020 — a kernel-free guard that flags done
//        backlog work not covered by a sprint, wired into pre-commit. Would have
//        caught the attribution migration committed without sprint28.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049). Gate results via append-gate-result.
// =====================================================================
package ProjectDeliveryS30 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part noSprintGuardStory : Story {
        :>> id = "63940516-2c7d-4e8f-b030-516273849501";
        :>> title = "Sprint 30: noSprintGuard (2 pts) — guard that done backlog work is covered by a sprint (D0064/issue020)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS30 {
        action storyNoSprintGuard;
        verification storyNoSprintGuardDoD : Test { :>> id = "63940516-2c7d-4e8f-b030-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "noSprintGuardDoD (pass): validate_sprint_coverage.py flags a done backlog task not referenced by any delivery file; pure-Python kernel-free; --selftest passes (covered-passes + orphan-flagged); 4 pre-D0064 tasks grandfathered; wired into pre-commit on .tracking/(backlog|delivery) changes; CLAUDE.md §5 updated; 0 violations at HEAD."; }
    }

    verification noSprintGuardRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b030-516273849503";
        :>> title = "Sprint 30 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = the no-sprint guard owed by issue020/D0047 (sprint29 retro). Approach: a done backlog task (passing DoD) must be covered by a sprint (name appears in a delivery file); pre-D0064 historical tasks grandfathered; pure-Python kernel-free (D0048); pre-commit wired. No blocking dependency. DoD = guard + selftest + grandfather + pre-commit + doc-sync + 0 violations at HEAD. estimatedPoints=2. DoD accepted.";
    }
    part noSprintGuardRefineGateR1 : TestResult { :>> id = "975775e8-1350-4a11-bb8a-06e5bc0a64cf"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7dbced7"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification noSprintGuardStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b030-516273849504";
        :>> title = "Sprint 30 standup gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: validate_sprint_coverage.py parses backlog done-tasks (<task>DoDR<n> pass) + greps delivery files for coverage; GRANDFATHERED set for pre-D0064 work; --selftest for positive+negative; wire into .githooks/pre-commit next to validate_ceremony. No blockers.";
    }
    part noSprintGuardStandupGateR1 : TestResult { :>> id = "ec79dbd7-460a-49fd-bb8a-06e5bec1b6c3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7dbced7"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification noSprintGuardImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b030-516273849505";
        :>> title = "Sprint 30 implement gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "validate_sprint_coverage.py built (kernel-free); --selftest PASS (fakeCovered passes, fakeOrphan flagged); live run flags 4 pre-D0064 tasks -> grandfathered (ceremonyGateGuard, rustS8runtimeParser, rustS9writeApi, trackedMetadataReplan); 0 violations at HEAD; wired into .githooks/pre-commit on .tracking/(backlog|delivery); CLAUDE.md §5 lists it. noSprintGuard backlog DoD recorded pass.";
    }

    verification noSprintGuardReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b030-516273849506";
        :>> title = "Sprint 30 review gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: guard mirrors validate_ceremony shape (GRANDFATHERED + scan + exit code); substring coverage is lenient (avoids false-positive commit blocks); grandfather set documented as pre-D0064. Phase-3 scan: the guard demonstrably catches issue020's class (attrModelSchema would flag before sprint28 existed).";
    }

    verification noSprintGuardCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b030-516273849507";
        :>> title = "Sprint 30 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + noSprintGuard DoD recorded pass; validate_all green; the guard itself runs clean in pre-commit. No human gate (D0051: a tested guard, self-evidencing).";
    }

    verification noSprintGuardRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b030-516273849508";
        :>> title = "Sprint 30 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. No new avoidable issue this sprint. Process improvement delivered: issue020's enforcement gap is now closed by an automated guard (D0047 satisfied — the correction became a permanent control, not vigilance). Note: the guard's name-substring coverage is approximate; a precise sprint->task edge will come with processGovernanceLineage. Estimate 2 pts held.";
    }
}
