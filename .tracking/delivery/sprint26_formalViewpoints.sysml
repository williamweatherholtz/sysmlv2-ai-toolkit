// =====================================================================
// ProjectDeliveryS26 — Sprint 26: formalViewpoints increment 2 (2 pts).
// Story: tooling reads the viewpoint registry — query.py viewpoints (declaration-driven
//        coverage/render-status view) + query.py orphans (orphansVP renderer). Python
//        only (no Rust source -> no deliverable re-suspect). Fixed read_issues brace bug.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049); test-backed (D0051).
// =====================================================================
package ProjectDeliveryS26 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part formalViewpointsStory : Story {
        :>> id = "61728394-0a5b-4c6d-bd61-40516273841b";
        :>> title = "Sprint 26: formalViewpoints increment 2 (2 pts) — declaration-driven query views + orphans + read_issues fix";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS26 {
        action storyFormalViewpoints;
        verification storyFormalViewpointsDoD : Test { :>> id = "61728394-0a5b-4c6d-ce62-40516273841c"; :>> method = VerificationMethod::test; :>> procedureText = "formalViewpointsDoD in backlog (pass): query.py viewpoints + orphans read the registry; read_issues brace-truncation fixed (issue019); validate green."; }
        part storyFormalViewpointsDoDR1 : TestResult { :>> id = "61728394-0a5b-4c6d-fe63-40516273841d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b6b4144"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification formalViewpointsRefineGate : Test {
        :>> id = "61728394-0a5b-4c6d-1164-40516273841e";
        :>> title = "Sprint 26 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach: make views declaration-driven at the query layer (Python, not Rust -> no deliverable re-suspect): query.py viewpoints reads .engine/views/ registry + render status (coverage); query.py orphans renders orphansVP (tasks w/o DoD, issues w/o or dangling relatedTask). report.py HTML tab = optional follow-on. estimatedPoints=2. DoD accepted.";
    }
    part formalViewpointsRefineGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-2264-40516273841f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b6b4144"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsStandupGate : Test {
        :>> id = "61728394-0a5b-4c6d-3364-405162738420";
        :>> title = "Sprint 26 standup gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. formalViewpoints inc2: query.py viewpoints + orphans, kernel-free. No blockers.";
    }
    part formalViewpointsStandupGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-4464-405162738421"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b6b4144"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsImplementGate : Test {
        :>> id = "61728394-0a5b-4c6d-5564-405162738422";
        :>> title = "Sprint 26 implement gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "query.py viewpoints (9 declared, 6 rendered) + orphans (clean) implemented kernel-free; read_issues + _VIEWPOINT_BLOCK brace-truncation fixed (issue019) — the orphans view caught it on first run; CLAUDE.md §1 doc-synced; sysmlv2 validate + validate_instances green. No Rust change (no deliverable re-suspect).";
    }
    part formalViewpointsImplementGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-6664-405162738423"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b6b4144"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsReviewGate : Test {
        :>> id = "61728394-0a5b-4c6d-7764-405162738424";
        :>> title = "Sprint 26 review gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: tooling now reads the registry (declaration-driven); orphans + viewpoints kernel-free. Phase-3 scan: the orphans view exposed + fixed a latent read_issues parser bug (issue019) — logged per D0047, the regex fix is the guard. report.py dashboard tab deferred (cosmetic).";
    }
    part formalViewpointsReviewGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-8864-405162738425"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b6b4144"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsCloseOutGate : Test {
        :>> id = "61728394-0a5b-4c6d-9964-405162738426";
        :>> title = "Sprint 26 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + backlog DoD recorded; validate_all green. No human gate (D0051: test-backed, no schema).";
    }
    part formalViewpointsCloseOutGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-aa64-405162738427"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b6b4144"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsRetroGate : Test {
        :>> id = "61728394-0a5b-4c6d-bb64-405162738428";
        :>> title = "Sprint 26 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049): AVOIDABLE issue found + fixed — read_issues brace truncation (issue019), caught by the new orphans view dogfooding itself (D0047: logged + regex fix is the guard). Otherwise clean. Estimate 2 pts held. Remainder: report.py HTML viewpoints/orphans tab (cosmetic, queued under nothing-blocking).";
    }
    part formalViewpointsRetroGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-cc64-405162738429"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b6b4144"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
