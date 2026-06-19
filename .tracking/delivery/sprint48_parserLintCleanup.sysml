// =====================================================================
// ProjectDeliveryS48 — Sprint 48: parserLintCleanup (D0074 migration M0b).
// Story: complete the fail-loud lint foundation — promote the restriction lints to the parser
//        crate + add clippy::indexing_slicing across the workspace, fixing the genuine production
//        indexing sites. (The feared "32 unwraps" were a miscount — `p.expect(&TokenKind,..)?`
//        is the parser's fallible method, not Option::expect; production code was already clean.)
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS48 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from parserLintCleanupStory to d0074;

    part parserLintCleanupStory : Story {
        :>> id = "63940516-2c7d-4e8f-b048-516273849501";
        :>> title = "Sprint 48: parserLintCleanup (3 pts) — complete fail-loud lints (parser restriction set + indexing_slicing); D0074 M0b";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS48 {
        action storyParserLintCleanup;
        verification storyParserLintCleanupDoD : Test { :>> id = "63940516-2c7d-4e8f-b048-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "parserLintCleanupDoD (pass): parser crate now denies clippy::unwrap_used/expect_used/panic/indexing_slicing (test-cfg allow); production parser was already clean (the '32' were p.expect(&TokenKind)? — a fallible method, not Option::expect). clippy::indexing_slicing added to the CLI too; fixed the genuine production sites: parser peek (.get/map_or) + eat (via peek), with documented #[allow] on the 2 invariant-safe accessors peek_token/advance (Eof-sentinel invariant); CLI main args[2..] -> a `rest` slice via .get().unwrap_or(&[]), flag() windows via slice pattern, write.rs lines[insert_after] + detect_indent slice via .get(). cargo clippy --workspace --all-targets -D warnings GREEN; full test suite green incl. write_bdd 10/10 (load-bearing write API intact)."; }
        part storyParserLintCleanupDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b048-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b085bc0"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
    }

    verification parserLintCleanupRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b048-516273849503";
        :>> title = "Sprint 48 refine gate (DoR)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0074 M0b. Re-measured the surface first and found the feared 32 parser unwraps were a MISCOUNT — `p.expect(&TokenKind,..)?` is the parser's own Result-returning method, not Option/Result::expect; production parser/lexer have ZERO real unwrap/expect/panic (all in tests). So the real work is just clippy::indexing_slicing across the workspace + a handful of production indexing sites. Low risk (test-guarded: parser proptest/BDD + write_bdd). estimatedPoints=3 (downgradable from the feared-large refactor). DoD accepted.";
    }
    part parserLintCleanupRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b048-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b085bc0"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserLintCleanupStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b048-516273849505";
        :>> title = "Sprint 48 standup gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: parser lib.rs deny restriction set + test allow (replace M0b debt note); CLI lib.rs/main.rs add indexing_slicing; fix production indexing (peek/eat clean, peek_token/advance documented-allow, args->rest, flag slice-pattern, write.rs .get()). Iterate cargo clippy --workspace --all-targets -D warnings to green; cargo test --workspace green. No blockers.";
    }
    part parserLintCleanupStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b048-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b085bc0"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserLintCleanupImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b048-516273849507";
        :>> title = "Sprint 48 implement gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "Lints enabled both crates. Fixed sites: parser peek (.get/map_or, mirrors existing peek_next), eat (via self.peek()), peek_token+advance (#[allow] + documented Eof-sentinel invariant); CLI main `rest` slice + flag slice-pattern; write.rs two indent blocks + detect_indent range via .get(). cargo clippy --workspace --all-targets -- -D warnings GREEN; cargo test --workspace green (incl. parser proptest + write_bdd 10/10 — write API behavior unchanged). parserLintCleanup backlog DoD recorded pass.";
    }
    part parserLintCleanupImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b048-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b085bc0"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserLintCleanupReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b048-516273849509";
        :>> title = "Sprint 48 review gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: fail-loud restriction lints now hold workspace-wide; the only #[allow]s are the 2 hot-path token accessors with a written, real invariant (Eof sentinel + saturating advance) — documented-safe, not silent. The production indexing fixes use .get()+safe-default (cosmetic indent / empty-slice), never unwrap. write_bdd 10/10 confirms the write API is untouched in behavior. WELL: re-measuring before acting corrected the 'big risky refactor' fear into a small bounded change. Phase-3 scan: the fail-loud foundation (D0074) is now complete across the toolchain; M2 (port query.py views) inherits it.";
    }
    part parserLintCleanupReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b048-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b085bc0"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserLintCleanupCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b048-51627384950b";
        :>> title = "Sprint 48 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + parserLintCleanup DoD recorded pass; clippy --workspace -D warnings + cargo test --workspace green. No human gate (D0051: a lint + safe-access change verified by clippy + the full test suite, self-evidencing).";
    }
    part parserLintCleanupCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b048-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b085bc0"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification parserLintCleanupRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b048-51627384950d";
        :>> title = "Sprint 48 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. KEY LESSON: the M0 deferral of the parser was based on a MISCOUNT (a grep matched `p.expect(` as Option::expect) — re-measuring the actual surface before M0b turned a feared large/risky refactor into a small bounded one. Avoidable-class: trust the compiler/linter's view of the surface, not a regex count. No avoidable rework (the deferral was conservative, not wrong). The fail-loud foundation (D0074) is now complete; the silent-skip parser hardening (parse_item fallback) the sprint47 retro flagged remains a separate small task (skip->error) for a future increment. Next: M2 (port query.py views to Rust, with marker-retention). Estimate 3 pts held.";
    }
    part parserLintCleanupRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b048-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b085bc0"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
}
