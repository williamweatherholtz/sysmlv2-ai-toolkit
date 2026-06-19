// =====================================================================
// ProjectDeliveryS45 — Sprint 45: rustFailLoudLints (D0074 migration M0).
// Story: establish the fail-loud Rust lint foundation BEFORE the view executor (M1) is written,
//        so new authority-bearing code is born under the denies. CLI crate roots deny
//        unwrap/expect/panic/todo/unimplemented (CLI src already clean); both crates forbid
//        unsafe. Parser restriction-lint cleanup + indexing_slicing deferred to M0b (strangler-fig).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS45 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from rustFailLoudLintsStory to d0074;

    part rustFailLoudLintsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b045-516273849501";
        :>> title = "Sprint 45: rustFailLoudLints (2 pts) — fail-loud Rust lint foundation; D0074 M0";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS45 {
        action storyRustFailLoudLints;
        verification storyRustFailLoudLintsDoD : Test { :>> id = "63940516-2c7d-4e8f-b045-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "rustFailLoudLintsDoD (pass): CLI lib.rs + main.rs deny clippy::unwrap_used/expect_used/panic/todo/unimplemented; sysmlv2-parser + sysmlv2-cli both #![forbid(unsafe_code)]. CLI src had 0 unwrap/expect/panic/todo + no unsafe -> green with no code changes. clippy::indexing_slicing + the parser's 32 restriction sites deferred to parserLintCleanup (M0b), recorded as a parser-crate debt note. cargo clippy --workspace --all-targets -- -D warnings GREEN."; }
        part storyRustFailLoudLintsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b045-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ee7a9fb"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
    }

    verification rustFailLoudLintsRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b045-516273849503";
        :>> title = "Sprint 45 refine gate (DoR)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0074 migration M0 (fail-loud lint foundation), sequenced BEFORE M1 so the view executor is born under the denies. Measured the impact first: 32 restriction-lint sites, ALL in the parser (lexer 2, parser 30); CLI src clean. Decision: deny the core restriction set on the CLI crate roots now (immediate green); strangler-fig the parser (load-bearing) + indexing_slicing to M0b rather than refactor the parser in the lint sprint. forbid(unsafe_code) both crates (no unsafe exists). No blocking dependency. estimatedPoints=2. DoD accepted.";
    }
    part rustFailLoudLintsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b045-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ee7a9fb"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification rustFailLoudLintsStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b045-516273849505";
        :>> title = "Sprint 45 standup gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: extend the existing crate-root deny (warnings/all/pedantic/nursery) in CLI lib.rs + main.rs with the restriction set; add forbid(unsafe_code) to both crate roots; parser gets a debt note pointing to parserLintCleanup (M0b). Integration tests are separate crates -> crate-root denies don't touch them, so no test grandfathering needed. Verify cargo clippy --workspace --all-targets -D warnings. No blockers.";
    }
    part rustFailLoudLintsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b045-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ee7a9fb"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification rustFailLoudLintsImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b045-516273849507";
        :>> title = "Sprint 45 implement gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "CLI lib.rs + main.rs: forbid(unsafe_code) + deny(clippy::unwrap_used/expect_used/panic/todo/unimplemented). parser lib.rs: forbid(unsafe_code) + M0b debt note. cargo clippy --workspace --all-targets -- -D warnings GREEN (no code changes needed — CLI src already clean). rustFailLoudLints backlog DoD recorded pass; parserLintCleanup (M0b) + tomlViewExecutor (M1) added to the backlog with succession from this task.";
    }
    part rustFailLoudLintsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b045-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ee7a9fb"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification rustFailLoudLintsReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b045-516273849509";
        :>> title = "Sprint 45 review gate";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the strangler-fig sequencing is honest — fail-loud is enforced where new code lands (CLI) and on unsafe everywhere, while the load-bearing parser's restriction-lint port is a SEPARATE tracked task (M0b) rather than a risky same-sprint refactor (D0074). indexing_slicing deferred for the same reason (it'd touch write.rs/main.rs working code). The green-with-no-code-changes result confirms the CLI was already clean. Phase-3 scan: M1 (view executor) now inherits the CLI denies — born fail-loud, exactly the sequencing intent.";
    }
    part rustFailLoudLintsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b045-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ee7a9fb"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification rustFailLoudLintsCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b045-51627384950b";
        :>> title = "Sprint 45 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + rustFailLoudLints DoD recorded pass; clippy green; validate_all green. No human gate (D0051: a lint-config change verified by cargo clippy, self-evidencing; direction accepted in D0074).";
    }
    part rustFailLoudLintsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b045-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ee7a9fb"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification rustFailLoudLintsRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b045-51627384950d";
        :>> title = "Sprint 45 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. WELL: measuring the violation surface BEFORE writing the lints (32 sites, all in the parser; CLI clean) let the sprint land green with zero code churn and a clear strangler-fig split (M0 now / M0b deferred), instead of an open-ended refactor. No avoidable issue. Next: tomlViewExecutor (M1) — born under these denies — then parserLintCleanup (M0b) can run in parallel. Estimate 2 pts held.";
    }
    part rustFailLoudLintsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b045-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ee7a9fb"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
}
