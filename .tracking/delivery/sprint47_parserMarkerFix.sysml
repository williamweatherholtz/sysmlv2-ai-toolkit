// =====================================================================
// ProjectDeliveryS47 — Sprint 47: parserMarkerFix (issue024; D0070/D0074).
// Story: the Rust parser silently dropped `#Marker part X : T` (marker-on-a-definition), so the
//        D0070 process-change Decisions vanished from every view — surfaced by M1 dogfooding
//        (the decisions view returned 70/75). Fix parse_hash_item to emit the underlying part;
//        guard with a parser unit test (D0047).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS47 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from parserMarkerFixStory to d0070;

    part parserMarkerFixStory : Story {
        :>> id = "63940516-2c7d-4e8f-b047-516273849501";
        :>> title = "Sprint 47: parserMarkerFix (2 pts) — rust parser dropped #Marker-prefixed parts (issue024)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS47 {
        action storyParserMarkerFix;
        verification storyParserMarkerFixDoD : Test { :>> id = "63940516-2c7d-4e8f-b047-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "parserMarkerFixDoD (pass): parse_hash_item delegates to parse_item for the non-edge case, EMITTING the underlying Part (`#ProspectiveChange part dNNNN : Decision`) instead of skip_item-dropping it (issue024). Marker consumed; AST marker-retention deferred to M2. GUARD: parser unit test marker_prefixed_part_is_preserved (D0047). Verified end-to-end: `sysmlv2 view decisions-active-and-superseded` returns 75 (was 70 — d0039/d0049/d0064/d0067/d0070 restored). cargo test --workspace --lib (33) + parse_engine_files (3) + clippy --workspace -D warnings green."; }
        part storyParserMarkerFixDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b047-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cf12cca"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
    }

    verification parserMarkerFixRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b047-516273849503";
        :>> title = "Sprint 47 refine gate (DoR)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: bug surfaced by M1 dogfooding (decisions view 70/75; missing 5 == the #ProspectiveChange-marked decisions). Diagnosed: parse_hash_item skip_item'd any `#Marker` not followed by first/dependency, dropping marker-on-part forms (D0070) — never caught because rust only validated .tracking, and the marked-PART form lives only in .engine/decisions. Minimal correct fix: emit the underlying item. Touches the load-bearing parser but is a 1-line delegate + a regression test. estimatedPoints=2. DoD accepted.";
    }
    part parserMarkerFixRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b047-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cf12cca"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserMarkerFixStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b047-516273849505";
        :>> title = "Sprint 47 standup gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: in parse_hash_item, replace the skip_item/None branch with `return parse_item(p, filename)` (re-dispatch on the now-current `part` token, emitting Item::Part); add parser unit test marker_prefixed_part_is_preserved; rebuild release; confirm the view returns 75. Marker-retention (for a rust process-changes view) noted for M2. No blockers.";
    }
    part parserMarkerFixStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b047-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cf12cca"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserMarkerFixImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b047-516273849507";
        :>> title = "Sprint 47 implement gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "parser.rs parse_hash_item fixed (delegate to parse_item). Parser unit test marker_prefixed_part_is_preserved added + passes. cargo test --workspace --lib 33 pass; parse_engine_files 3 pass (no regression). clippy --workspace --all-targets -D warnings green. Release rebuilt: `view decisions-active-and-superseded` = 75 (d0049 + d0070 present). issue024 logged with the fix + guard. parserMarkerFix backlog DoD recorded pass.";
    }
    part parserMarkerFixImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b047-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cf12cca"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserMarkerFixReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b047-516273849509";
        :>> title = "Sprint 47 review gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the fix is minimal + correct (emit, don't drop) and the regression test is the D0047 guard. The deeper lesson — a SILENT skip is the dangerous failure mode (it passed `check` clean while losing data); dogfooding M1 over .engine surfaced it precisely because the view consumed files rust had never parsed. Honest residue: the marker annotation is still not retained in the AST, so a rust `process-changes` view needs marker-retention (tracked for M2) — but no data is lost now. Phase-3 scan: this hardens the parser for the full-rust direction (M2+ will parse .engine instances routinely).";
    }
    part parserMarkerFixReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b047-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cf12cca"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserMarkerFixCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b047-51627384950b";
        :>> title = "Sprint 47 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + parserMarkerFix DoD recorded pass; cargo test + clippy + validate_all green; view returns 75. No human gate (D0051: a tested parser fix with a regression guard, self-evidencing).";
    }
    part parserMarkerFixCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b047-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cf12cca"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserMarkerFixRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b047-51627384950d";
        :>> title = "Sprint 47 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. AVOIDABLE-CLASS: a parser branch that SKIPS on an unrecognized form (skip_item + None) silently loses data and passes validation — the same silent-failure anti-pattern D0074's fail-loud conventions target. Better long-term: parse_hash_item should ERROR on a truly-unsupported `#Marker <x>` rather than skip (fail loud); the current fix handles the known case (part) but other unknown followers still skip — noted for M0b/M2 parser hardening. WELL: M1 dogfooding caught a latent parser bug that had been invisible (the form only existed in rust-unvalidated .engine files). Next: M0b (parserLintCleanup — fold in the fail-loud skip->error hardening) + M2 (port query.py views, with marker-retention). Estimate 2 pts held.";
    }
    part parserMarkerFixRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b047-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cf12cca"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
}
