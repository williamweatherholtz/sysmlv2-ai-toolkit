// =====================================================================
// ProjectDeliveryS132 — Sprint 132: staleUnescapeFix (D0084) — fix the staleness false-positive engine
// bug (issue044) the critique campaign surfaced, + finish the escaped-content critiques. extract_field
// (view.rs) read raw git blobs without the lexer's string-unescaping, so escaped semantic fields
// (d0023 regex, d0076 escaped-quote) false-flagged their critiques stale -> excluded from coverage.
// FIX: decode_string_body unescapes exactly like lex_string + stops at the first UNESCAPED quote
// (blob-extract == model-attr). Guard (D0047): 2 unit tests. d0023 + d0076 now COVERED.
// critique-coverage 117->119/120. Total: 3 pts. Ceremony: AUTONOMOUS (D0049); issue044 closed by this resolver.
// =====================================================================
package ProjectDeliveryS132 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from staleUnescapeFixStory to d0084;

    part staleUnescapeFixStory : Story {
        :>> id = "63940516-2c7d-4e8f-c132-51627384c001";
        :>> title = "Sprint 132: staleUnescapeFix (3 pts) — fix the escaped-field staleness false-positive (issue044) + critique d0076 (D0084)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::refactor;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS132 {
        action storyStaleUnescapeFix;
        verification storyStaleUnescapeFixDoD : Test { :>> id = "63940516-2c7d-4e8f-c132-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0084 staleness-mechanism bug fix (resolves issue044, Medium, found by the critique campaign). ROOT CAUSE: view.rs extract_field read an element's semantic field from the RAW git blob and did not replicate the lexer's string-unescaping (lex_string: backslash-backslash->backslash, backslash-quote->quote, \\n, \\t) nor detect the end quote as UNESCAPED — so escaped fields (d0023 regex \\s/\\w/\\d, d0076 escaped \\\") compared unequal (blob vs model) -> false-stale -> critiques excluded from coverage (D0084 path, view.rs ~1362). FIX: decode_string_body unescapes exactly like lex_string + stops at the first unescaped quote, so blob-extract == model-attr. PERMANENT GUARD (D0047): unit tests extract_field_unescapes_like_the_lexer + extract_field_plain_value. Plus the d0076 Core-3 critiques (now countable). VERIFY (measured): d0023 + d0076 now COVERED; critique-coverage 117->119/120; clippy --workspace --all-targets -D warnings green (fixed: doc backticks + identical-match-arms); cargo test 48 lib + all BDD pass (no regression); keel validate 145; guard all (13) PASS; assured READY (issue044 closed by this resolver)."; }
        part storyStaleUnescapeFixDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c132-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0b94355"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification staleUnescapeFixRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c132-51627384c004"; :>> title = "Sprint 132 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the critique campaign (Sprint 124) surfaced a Medium staleness false-positive (issue044) blocking d0023+d0076 from counting toward 100%. D0084 owns element-content staleness. Resolver: staleUnescapeFix (fix + guard tests + d0076 critique). estimatedPoints=3. #CharteredBy d0084. DoD accepted."; }
    part staleUnescapeFixRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c132-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0b94355"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification staleUnescapeFixStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c132-51627384c006"; :>> title = "Sprint 132 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: replace extract_field's raw end-quote scan with decode_string_body (lexer-matching unescape + unescaped-quote terminator); add 2 unit tests (D0047 guard); critique d0076. Verify d0023+d0076 count, full test suite green (view.rs is the staleness/coverage engine — regression risk), assured READY. No .engine/schema/process-def change -> no kernel/keystone (the fix corrects a false-positive; intended staleness semantics unchanged -> EXECUTE). No blockers."; }
    part staleUnescapeFixStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c132-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0b94355"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification staleUnescapeFixImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c132-51627384c008"; :>> title = "Sprint 132 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented decode_string_body + 2 unit tests + d0076 critiques. VERIFY (live): d0023+d0076 COVERED; critique-coverage 119/120; clippy -D warnings green; cargo test 48 lib (incl the 2 new) + BDD pass; keel validate 145; guard all (13) PASS. assured READY (issue044 closed)."; }
    part staleUnescapeFixImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c132-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0b94355"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification staleUnescapeFixReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c132-51627384c00a"; :>> title = "Sprint 132 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The fix is minimal + correct: decode_string_body mirrors the lexer's escape handling so the staleness comparison is apples-to-apples; it preserves intended semantics (a genuine content change still differs) — a pure false-positive correction (EXECUTE). The 2 unit tests lock the regression (D0047). The critique campaign found AND fixed a real engine bug — the assurance loop working end to end. d0023+d0076 now count. Clean."; }
    part staleUnescapeFixReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c132-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0b94355"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification staleUnescapeFixCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c132-51627384c00c"; :>> title = "Sprint 132 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyStaleUnescapeFix DoD pass; the engine bug is fixed + guard-tested, d0023+d0076 count, critique-coverage 119/120, issue044 closed, assured READY, guard all (13) green. Self-evidencing (D0051). Only d0003 remains (the writePolicy Medium finding) — needs human disposition to reach 120/120."; }
    part staleUnescapeFixCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c132-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0b94355"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification staleUnescapeFixRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c132-51627384c00e"; :>> title = "Sprint 132 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the critique campaign produced its highest-value artifact — a real engine bug found (the 84-vs-85 discrepancy), root-caused (raw-blob vs lexer-unescaped mismatch), fixed (decode_string_body), and locked with a regression guard (D0047 unit tests). Verifying d0023+d0076 actually flip to COVERED proved the fix end to end. AVOIDABLE-ISSUE SCAN: the bug IS the finding (issue044, now closed); the clippy doc-backtick + identical-match-arm nits were caught + fixed pre-commit. LESSON: any raw-text read of a .sysml field must replicate the lexer's unescaping or it diverges from the model. NON-blocking: d0003 (writePolicy enforcement Medium) is the sole remaining item — it needs the human's ACT/ACCEPT-RISK/DISMISS to close out 120/120. Estimate 3 pts held."; }
    part staleUnescapeFixRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c132-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0b94355"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
