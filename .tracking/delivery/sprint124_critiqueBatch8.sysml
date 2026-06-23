// ProjectDeliveryS124 — Sprint 124: critiqueBatch8 (D0080 backfill pass 8) — Decisions d0022-d0026,
// Core-3 each (15 critiques), critic claudeOpus. All authored + survived. DISCOVERY: d0023 shows
// UNCOVERED despite valid critiques — a genuine engine bug (compute_stale_verifications/extract_field
// doesn't replicate the parser's string-unescaping, so d0023's regex-backslash `decision` field
// false-flags its critiques stale -> excluded; view.rs:1362). Deferred to end-of-run signoff as
// issue044 (Medium) + a staleUnescape fix. critique-coverage 80->84/120. 3 pts. AUTONOMOUS (D0049).
package ProjectDeliveryS124 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from critiqueBatch8Story to d0080;

    part critiqueBatch8Story : Story {
        :>> id = "63940516-2c7d-4e8f-c124-51627384c001";
        :>> title = "Sprint 124: critiqueBatch8 (3 pts) — Core-3 critique backfill for Decisions d0022-d0026 (D0080)";
        :>> createdAt = "2026-06-23"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::modeling; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS124 {
        action storyCritiqueBatch8;
        verification storyCritiqueBatch8DoD : Test { :>> id = "63940516-2c7d-4e8f-c124-51627384c002"; :>> method = VerificationMethod::critique; :>> procedureText = "D0080 backfill pass 8: 15 Core-3 critiques (critic claudeOpus/aiModel, #Verify-linked) for d0022-d0026, all authored + survived. DISCOVERY (deferred to signoff as issue044, Medium): d0023's critiques show UNCOVERED due to an engine bug — compute_stale_verifications/extract_field reads the raw git blob and does NOT replicate the parser's string-unescaping, so d0023's `decision` field (containing regex \\s/\\w/\\d escapes) compares unequal (blob \\s vs model \\s-unescaped) -> false-stale -> the critiques are excluded (view.rs:1362). Blocks coverage for ANY element whose semantic field has backslash escapes. Fix (staleUnescape): normalize/unescape both sides before comparing. VERIFY: critique-coverage 80->84/120 (d0023 critiqued but uncounted pending the fix); keel validate 137; guard all (13) PASS."; }
        part storyCritiqueBatch8DoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c124-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1427491"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification critiqueBatch8RefineGate : Test { :>> id = "63940516-2c7d-4e8f-c124-51627384c004"; :>> title = "Sprint 124 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: autonomous run; pass 8 = d0022-d0026. #CharteredBy d0080. DoD accepted."; }
    part critiqueBatch8RefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c124-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1427491"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification critiqueBatch8StandupGate : Test { :>> id = "63940516-2c7d-4e8f-c124-51627384c006"; :>> title = "Sprint 124 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: Core-3 per decision; historical CR records pass; defer findings. No blockers."; }
    part critiqueBatch8StandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c124-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1427491"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification critiqueBatch8ImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c124-51627384c008"; :>> title = "Sprint 124 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::critique; :>> procedureText = "15 critiques (d0022-d0026) authored, all pass. d0023 uncounted (staleness/unescape bug, deferred issue044). coverage 84/120; keel validate 137; guard all (13) PASS."; }
    part critiqueBatch8ImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c124-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1427491"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification critiqueBatch8ReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c124-51627384c00a"; :>> title = "Sprint 124 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The campaign surfaced a real engine bug (staleness false-positive on backslash-escaped semantic fields) — high-value find. d0023's critiques are valid + retained; they will count once the staleUnescape fix lands. Deferred honestly rather than hidden. Clean."; }
    part critiqueBatch8ReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c124-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1427491"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification critiqueBatch8CloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c124-51627384c00c"; :>> title = "Sprint 124 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; coverage 84/120; assured READY; guard all green. Deferred-for-signoff set now: d0003 (writePolicy Medium), issue044 (staleness-unescape Medium, blocks d0023 + escaped-content elements)."; }
    part critiqueBatch8CloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c124-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1427491"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification critiqueBatch8RetroGate : Test { :>> id = "63940516-2c7d-4e8f-c124-51627384c00e"; :>> title = "Sprint 124 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: noticing the 84-vs-expected-85 coverage discrepancy and chasing it (rather than ignoring) uncovered a real engine bug — staleness false-positives on backslash-escaped semantic fields (regex-bearing decisions). This is the critique campaign producing concrete engine value. AVOIDABLE-ISSUE SCAN: the bug itself is the finding (issue044, deferred). LESSON: extract_field (raw git blob) must normalize string escapes to match the parser, or it will false-flag any escaped content stale. NON-blocking: ~34 decisions remain; some with regex/backslash content will also uncount until staleUnescape lands. Estimate 3 pts held."; }
    part critiqueBatch8RetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c124-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1427491"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
