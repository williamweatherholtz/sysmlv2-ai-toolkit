// =====================================================================
// ProjectDeliveryS10 — Sprint 10 delivery run: issueLoop.
// Story: issueLoop — add .tracking/issues.sysml convention + query.py
//   issues subcommand; schema relatedTask field (D0029); Issue instances
//   surface with typed backlog routing.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS10 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 10 Story
    // ====================================================================
    part issueLoopStory : Story {
        :>> id = "1a2b3c4d-e5f6-4a7b-bc8c-9d0e1f2a3b4c";
        :>> title = "Sprint 10: issueLoop — .tracking/issues.sysml + query.py issues subcommand";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 1;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunS10 {
        action storyIssueLoop;
        verification storyIssueLoopDoD : Test { :>> id = "2b3c4d5e-f6a7-4b8c-cd9d-0e1f2a3b4c5d"; :>> method = VerificationMethod::demo; :>> procedureText = "query.py issues subcommand: reads Issue instances from all .tracking files; outputs JSON list with name/title/description/discoveredInField/relatedTask; .tracking/issues.sysml has issue001 linked to authoredByRefs; demo run passes; schema relatedTask field added (D0029); validate_schema green."; }
        part storyIssueLoopDoDR1 : TestResult { :>> id = "3c4d5e6f-a7b8-4c9d-de0e-1f2a3b4c5d6e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ed8c55"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification issueLoopRefineGate : Test {
        :>> id = "4d5e6f7a-b8c9-4d0e-ef1f-2a3b4c5d6e7f";
        :>> title = "Sprint 10 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: add relatedTask : String[0..1] to Issue (D0029); multi-line Issue instances in .tracking/issues.sysml; query.py issues subcommand using _ISSUE_BLOCK regex (multiline match on part...Issue{...}); read_issues() extracts name/title/description/discoveredInField/relatedTask; demo: query.py issues shows issue001 linked to authoredByRefs; DoD accepted.";
    }

    part issueLoopRefineGateR1 : TestResult { :>> id = "ad1e2f3a-b4c5-4d6e-e575-8a9b0c1d2e3f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ed8c55"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification issueLoopStandupGate : Test {
        :>> id = "5e6f7a8b-c9d0-4e1f-f020-3b4c5d6e7f8a";
        :>> title = "Sprint 10 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation: work.sysml Issue type has relatedTask field; .tracking/issues.sysml with issue001; query.py: _ISSUE_BLOCK + _BOOL_ASSIGN regexes added; read_issues() function; issues subcommand in main(); all validators green; no blocking issues.";
    }

    part issueLoopStandupGateR1 : TestResult { :>> id = "be2f3a4b-c5d6-4e7f-f686-9b0c1d2e3f4a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ed8c55"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification issueLoopImplementGate : Test {
        :>> id = "6f7a8b9c-d0e1-4f2a-a131-4c5d6e7f8a9b";
        :>> title = "Sprint 10 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::demo;
        :>> procedureText = "validate_schema green; validate_tracking green (issues.sysml parses); query.py issues outputs issue001 with all fields; relatedTask = authoredByRefs surfaced correctly.";
    }

    part issueLoopImplementGateR1 : TestResult { :>> id = "cf3a4b5c-d6e7-4f8a-a797-0c1d2e3f4a5b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ed8c55"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification issueLoopReviewGate : Test {
        :>> id = "7a8b9c0d-e1f2-4a3b-b242-5d6e7f8a9b0c";
        :>> title = "Sprint 10 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: work.sysml relatedTask field added correctly; issues.sysml parses green; query.py issues subcommand clean (no import side effects); D0029 recorded; doc-sync D0030 recorded.";
    }

    part issueLoopReviewGateR1 : TestResult { :>> id = "d04b5c6d-e7f8-4a9b-b8a8-1d2e3f4a5b6c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ed8c55"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification issueLoopCloseOutGate : Test {
        :>> id = "8b9c0d1e-f2a3-4b4c-c353-6e7f8a9b0c1d";
        :>> title = "Sprint 10 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyIssueLoopDoDR1 recorded (pass); issueLoopDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 10 complete.";
    }

    verification issueLoopRetroGate : Test {
        :>> id = "9c0d1e2f-a3b4-4c5d-d464-7f8a9b0c1d2e";
        :>> title = "Sprint 10 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 10 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
}
