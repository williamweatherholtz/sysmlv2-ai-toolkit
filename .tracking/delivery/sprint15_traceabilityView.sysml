// =====================================================================
// ProjectDeliveryS15 — Sprint 15: traceabilityView (5 pts).
// Story: report.py Traceability tab — per-sprint process linkage, process
//        evolution timeline, item-to-decision trace, subversion highlighting.
// Total: 5 pts  Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS15 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 15 Story
    // ====================================================================
    part traceabilityViewStory : Story {
        :>> id = "1d103ea6-57a9-48dc-9adc-d2006c657d4e";
        :>> title = "Sprint 15: traceabilityView (5 pts) — process traceability tab in report.py";
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
    action def DeliveryRunS15 {
        action storyTraceabilityView;
        verification storyTraceabilityViewDoD : Test { :>> id = "74b16e84-d059-47a1-88e7-90da0fc7b8b2"; :>> method = VerificationMethod::demo; :>> procedureText = "traceabilityViewDoDR1 in backlog (pass): report.py Traceability tab renders (1) per-sprint process linkage, (2) process evolution timeline, (3) item-to-decision trace, (4) subversion highlighting."; }
        part storyTraceabilityViewDoDR1 : TestResult { :>> id = "5613c3dc-966d-4b8a-8f3f-ea36ff6eeb81"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7ce9e8d"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification traceabilityViewRefineGate : Test {
        :>> id = "2790b7c6-e3a0-478d-bd65-88ed0403dbc6";
        :>> title = "Sprint 15 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: traceabilityView = new Traceability tab (tab 6 in report.py, Decisions shifts to tab 7) with 4 sections: (1) Process linkage — for each sprint delivery file, parse which phase gates were attempted vs skipped, map to skill names; (2) Evolution timeline — decisions and skills sorted by createdAt showing when process changed; (3) Item-decision trace — backlog items cross-referenced to decisions that govern their scope; (4) Subversion highlight — sprints missing expected gates flagged in orange. All data is parsed from existing .sysml files + git. No schema changes. estimatedPoints = 5. Unblocked. DoD accepted.";
    }
    part traceabilityViewRefineGateR1 : TestResult { :>> id = "0d0033eb-9090-4132-b575-d57ee12d971b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f5f7323"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification traceabilityViewStandupGate : Test {
        :>> id = "ef69435a-8c6d-437c-af65-4fb9c95b391f";
        :>> title = "Sprint 15 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Sprint 15 refine gate passed (f5f7323); cursor at Delivery/refine. traceabilityView: 4-section Traceability tab in report.py; parse sprint delivery files, decisions, skills; no schema changes. No blockers.";
    }
    part traceabilityViewStandupGateR1 : TestResult { :>> id = "07098d0a-b33e-4e1b-95a3-0ddc59ffbf53"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a13ae77"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification traceabilityViewImplementGate : Test {
        :>> id = "de8b40b7-9dbc-44c8-a48d-5c15ead85f95";
        :>> title = "Sprint 15 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "report.py Traceability tab (tab 6) renders all 4 sections; Decisions tab shifts to tab 7; validate_all green; traceabilityViewDoDR1 added to backlog (pass).";
    }
    part traceabilityViewImplementGateR1 : TestResult { :>> id = "a581a8a0-4c21-4cee-93a4-62dc307dcdbe"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7ce9e8d"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification traceabilityViewReviewGate : Test {
        :>> id = "186dcb2c-d5f2-445f-b1b9-2f60acb9b152";
        :>> title = "Sprint 15 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: Traceability tab renders correct data from sprint + decision + skills files; no regressions in tabs 1-5; doc-sync complete.";
    }
    part traceabilityViewReviewGateR1 : TestResult { :>> id = "5d4c1c9b-8c4b-44ad-af59-f0958d7c9893"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7ce9e8d"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification traceabilityViewCloseOutGate : Test {
        :>> id = "93172fbc-c932-47cc-af7f-310c8a30ebff";
        :>> title = "Sprint 15 closeOut gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyTraceabilityViewDoDR1 recorded (pass); traceabilityViewDoDR1 in backlog (pass); validate_all green; user accepts Sprint 15 complete.";
    }

    part traceabilityViewCloseOutGateR1 : TestResult { :>> id = "b3d7a291-e4f8-4c0a-9d12-6e5b8f1a0c23"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c1a7720"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }

    verification traceabilityViewRetroGate : Test {
        :>> id = "ea34680d-ff4e-4553-a163-0aac9e970e78";
        :>> title = "Sprint 15 retro gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 15 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part traceabilityViewRetroGateR1 : TestResult { :>> id = "c6d8b0f4-2a3e-4f5b-9c1d-7e8f0a1b2c3e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0f326b5"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }
}
