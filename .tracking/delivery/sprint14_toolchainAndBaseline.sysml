// =====================================================================
// ProjectDeliveryS14 — Sprint 14: toolchainWatch + baselineView.
// Stories: toolchainWatch (2 pts) — evaluate newer SysML v2 toolchain for
//          expose/verify-by/derive adoption; record Decision.
//          baselineView (3 pts) — Release type + computed baseline view demo.
// Total: 5 pts  Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS14 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 14 Story
    // ====================================================================
    part toolchainAndBaselineStory : Story {
        :>> id = "5b2c3d4e-6f7a-4b8c-9d0e-1f2a3b4c5d6e";
        :>> title = "Sprint 14: toolchainWatch (2 pts) + baselineView (3 pts) — toolchain adoption decision + computed baseline view";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 5;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunS14 {
        action storyToolchainAndBaseline;
        verification storyToolchainAndBaselineDoD : Test { :>> id = "6c3d4e5f-7a8b-4c9d-0e1f-2a3b4c5d6e7f"; :>> method = VerificationMethod::demo; :>> procedureText = "toolchainWatchDoDR1 in backlog (pass): newer SysML v2 toolchain evaluated; expose/verify-by/derive adoption Decision recorded. baselineViewDoDR1 in backlog (pass): Release (commit + member refs) renders a computed baseline view via query.py or report.py."; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification toolchainAndBaselineRefineGate : Test {
        :>> id = "8e5f6a7b-9c0d-4e1f-2a3b-4c5d6e7f8a9b";
        :>> title = "Sprint 14 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: toolchainWatch = research SysML v2 toolchain (expose/verify-by/derive status in current + upcoming releases) + record adoption Decision; baselineView = define Release occurrence def in schema/core + author a baseline instance in .tracking + extend query.py or report.py with baseline tab/subcommand + demo. estimatedPoints = 5 (2+3). Both unblocked. DoD accepted.";
    }
    part toolchainAndBaselineRefineGateR1 : TestResult { :>> id = "9f6a7b8c-0d1e-4f2a-3b4c-5d6e7f8a9b0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f2ec68"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification toolchainAndBaselineStandupGate : Test {
        :>> id = "0a7b8c9d-1e2f-4a3b-4c5d-6e7f8a9b0c1d";
        :>> title = "Sprint 14 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Sprint 14 refine gate passed (aba9e6f); cursor at Delivery/refine. toolchainWatch: research SysML v2 toolchain expose/verify-by/derive support status; record Decision. baselineView: Release occurrence def + query.py baseline subcommand + demo. No blockers.";
    }
    part toolchainAndBaselineStandupGateR1 : TestResult { :>> id = "b0c1d2e3-f4a5-4b6c-7d8e-9f0a1b2c3d4e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "aba9e6f"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification toolchainAndBaselineImplementGate : Test {
        :>> id = "1b8c9d0e-2f3a-4b4c-5d6e-7f8a9b0c1d2e";
        :>> title = "Sprint 14 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "D0041 (toolchainWatch: all deferred, D0031 confirmed); D0042 (baselineView: EngineBaseline::Release occurrence def + baselines.sysml + report.py tab 5); schema 14/14 green; instances 49/49 green; tracking 23+/23+ green. toolchainWatchDoDR1 + baselineViewDoDR1 to be added to backlog.";
    }

    verification toolchainAndBaselineReviewGate : Test {
        :>> id = "2c9d0e1f-3a4b-4c5d-6e7f-8a9b0c1d2e3f";
        :>> title = "Sprint 14 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: toolchainWatch Decision consistent with D0040 view rule; baselineView Release type in schema + baseline query clean; doc-sync complete.";
    }

    verification toolchainAndBaselineCloseOutGate : Test {
        :>> id = "3d0e1f2a-4b5c-4d6e-7f8a-9b0c1d2e3f4a";
        :>> title = "Sprint 14 closeOut gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyToolchainAndBaselineDoDR1 recorded (pass); toolchainWatchDoDR1 + baselineViewDoDR1 in backlog (pass); validate_all green; user accepts Sprint 14 complete.";
    }

    verification toolchainAndBaselineRetroGate : Test {
        :>> id = "4e1f2a3b-5c6d-4e7f-8a9b-0c1d2e3f4a5b";
        :>> title = "Sprint 14 retro gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 14 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
}
