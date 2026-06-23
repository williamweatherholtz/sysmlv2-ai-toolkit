// =====================================================================
// ProjectDeliveryS115 — Sprint 115: initSmokeTest (D0093 spin-up regression guard). The cold-start path
// (`keel init`) had NO automated test — a scaffold regression (embedded-engine drift, engine/instance
// remap break) would only surface when a human tried it. FIX: keel-cli/tests/init_smoke.rs drives the
// REAL `keel` binary (CARGO_BIN_EXE_keel) end-to-end against a temp dir: init -> assert the scaffold
// (.engine/ + CLAUDE.md + .tracking/ + read-only reference/decisions/ + fresh empty decisions/), then
// `keel validate` clean + `keel guard all` ALL PASS (the D0093 'spin up GREEN' promise) + `keel orient`
// works + re-init REFUSES to overwrite. Hermetic (owns + cleans its temp dir). Total: 2 pts. Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS115 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from initSmokeTestStory to d0093;

    part initSmokeTestStory : Story {
        :>> id = "63940516-2c7d-4e8f-c115-51627384c001";
        :>> title = "Sprint 115: initSmokeTest (2 pts) — end-to-end test that `keel init` scaffolds a working green project (D0093)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS115 {
        action storyInitSmokeTest;
        verification storyInitSmokeTestDoD : Test { :>> id = "63940516-2c7d-4e8f-c115-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0093 cold-start regression guard. keel-cli/tests/init_smoke.rs (a cargo integration test driving env!(CARGO_BIN_EXE_keel) — the real binary, exercising the embedded include_dir scaffold + the engine/instance remap as a newcomer would): create a unique hermetic temp dir (atomic counter + pid, cleaned via a Drop guard), run `keel init`, assert exit 0 + .engine/ + CLAUDE.md + .tracking/ + .engine/reference/decisions/ (read-only arch) + .engine/decisions/ (fresh empty, where the new project authors its own); then `keel validate <proj>` clean, `keel guard all <proj>` ALL PASS (the D0093 spin-up-green promise), `keel orient <proj>` succeeds + emits ready[], and a SECOND `keel init <proj>` exits non-zero (refuses to clobber). VERIFY (this sprint): cargo test --test init_smoke = 1 passed; full cargo test 46 lib + init_smoke + all BDD pass; clippy --workspace --all-targets -D warnings + keel validate + guard all (13) green."; }
        part storyInitSmokeTestDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c115-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a1588d7"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification initSmokeTestRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c115-51627384c004"; :>> title = "Sprint 115 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the console-arc retros flagged the lack of an init smoke test. D0093 makes `keel init` the spin-up path; a regression in the embedded scaffold should fail a test, not a human. Resolver: initSmokeTest. estimatedPoints=2. #CharteredBy d0093. DoD accepted."; }
    part initSmokeTestRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c115-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a1588d7"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification initSmokeTestStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c115-51627384c006"; :>> title = "Sprint 115 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: a cargo integration test (tests/init_smoke.rs) invoking the real binary via CARGO_BIN_EXE_keel — init then validate/guard-all/orient/re-init-refuse against a hermetic temp dir. Confirmed manually first that a fresh scaffold is green (212 files, validate clean, guard ALL PASS, orient ok, re-init exit 2) so the test asserts true behavior. No .engine/schema/process-def change -> no kernel/keystone. No blockers."; }
    part initSmokeTestStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c115-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a1588d7"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification initSmokeTestImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c115-51627384c008"; :>> title = "Sprint 115 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented tests/init_smoke.rs (binary-driven, hermetic). VERIFY: cargo test --test init_smoke = 1 passed (0.69s); full suite 46 lib + init_smoke + BDD all pass; clippy --workspace --all-targets -D warnings green (the test allows unwrap/expect — test-only); keel validate + guard all (13) PASS. Manually pre-confirmed the scaffold is green before asserting it."; }
    part initSmokeTestImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c115-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a1588d7"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification initSmokeTestReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c115-51627384c00a"; :>> title = "Sprint 115 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The test drives the SHIPPED binary (not the lib API), so it catches exactly what a newcomer hits: embedded-scaffold drift, the engine/instance remap (reference/decisions read-only + fresh decisions empty), and the spin-up-green promise (validate + guard all). It is hermetic (temp dir owned + Drop-cleaned, atomic+pid naming — no ambient state, lesson carried from issue041) and asserts the refuse-to-overwrite safety. Cheap (0.7s). A real cold-start regression guard. Clean."; }
    part initSmokeTestReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c115-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a1588d7"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification initSmokeTestCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c115-51627384c00c"; :>> title = "Sprint 115 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyInitSmokeTest DoD recorded pass; the binary-driven init smoke test is live + green. clippy + full cargo test (46 lib + init_smoke + BDD) + keel validate + guard all (13) green. Self-evidencing (D0051). Third of four post-D0094 forward items; the deliverable-manifest serve.rs coverage is last."; }
    part initSmokeTestCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c115-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a1588d7"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification initSmokeTestRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c115-51627384c00e"; :>> title = "Sprint 115 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: testing the SHIPPED binary end-to-end (CARGO_BIN_EXE_keel) — not the lib — means the test covers the embedded include_dir scaffold + the engine/instance remap, the exact surface a newcomer touches and the one most likely to silently drift; manually confirming the scaffold was green BEFORE writing assertions avoided baking a false expectation. Applied the issue041 lesson proactively: the test is hermetic from the start (owns its temp dir + Drop cleanup + atomic/pid naming), not dependent on ambient state. AVOIDABLE-ISSUE SCAN: no NEW avoidable defect. Re-confirmed: stop keel.exe before rebuild. NON-blocking forward note (NOT debt): no GitHub Actions workflow exists yet, so this runs via `cargo test` (the real local/pre-merge gate); a .github/workflows CI runner could run it on push later, deferred. Estimate 2 pts held. initSmokeTest done; the last forward item (serve.rs in the deliverable-manifest) next."; }
    part initSmokeTestRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c115-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a1588d7"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
