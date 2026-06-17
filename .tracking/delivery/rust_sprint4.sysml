// =====================================================================
// ProjectDeliveryRustS4 — Sprint 4 delivery run: build-time spec-pin check.
// Story: rustS4SpecCompat — build.rs pins SysML v2 grammar SHA; offline mode;
//   version constant baked in at compile time; 3 BDD scenarios green.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS4 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 4 Story
    // ====================================================================
    part rustS4SpecCompatStory : Story {
        :>> id = "01234567-5555-4e5f-b6c7-d8e9f0a1b2c6";
        :>> title = "Sprint 4: SysML v2 spec-compatibility check in build.rs";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 2;
    }

    // ====================================================================
    // Delivery backlog — DoD read by query.py.
    // ====================================================================
    action def DeliveryRunRustS4 {
        action storyRustS4SpecCompat;
        verification storyRustS4SpecCompatDoD : Test { :>> id = "12345678-6666-4f6a-c7d8-e9f0a1b2c3d6"; :>> method = VerificationMethod::test; :>> procedureText = "build.rs defines SYSML_V2_SPEC_URL and SYSML_V2_GRAMMAR_SHA constants; fetches on build + verifies SHA256; fails with actionable message on mismatch; SYSML_V2_SPEC_OFFLINE=1 skips remote check; SYSML_V2_GRAMMAR_VERSION emitted as rustc-env; .cargo/config.toml sets offline default; 3 BDD scenarios + 68 total tests green; clippy -D warnings clean."; }
        part storyRustS4SpecCompatDoDR1 : TestResult { :>> id = "ef012345-cdef-4a6b-c7d8-e9f0a1b2c3d5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6bbf241"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS4RefineGate : Test {
        :>> id = "23456789-7777-4a7b-d8e9-f0a1b2c3d4e7";
        :>> title = "Sprint 4 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "SysML v2 Release repo URL confirmed (github.com/Systems-Modeling/SysML-v2-Release); build.rs approach agreed (5s timeout, non-fatal network error, fatal SHA mismatch); SYSML_V2_SPEC_OFFLINE=1 default via .cargo/config.toml; SYSML_V2_GRAMMAR_VERSION constant agreed; 3 BDD scenarios outlined (offline mode reflected, SHA mismatch error, correct SHA passes); sha2 + hex + ureq build-deps agreed; DoD accepted.";
    }
    part rustS4RefineGateR1 : TestResult { :>> id = "3456789a-8888-4b8c-e9f0-a1b2c3d4e5f8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ebc38f3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS4StandupGate : Test {
        :>> id = "456789ab-9999-4c9d-f0a1-b2c3d4e5f6a9";
        :>> title = "Sprint 4 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation approach: build.rs standalone with sha256_hex using sha2+hex; ureq::get with 5s timeout; process::exit(1) on mismatch (cleaner than panic!); spec_compat.rs library module exposes SYSML_V2_SPEC_URL, SYSML_V2_GRAMMAR_SHA, SYSML_V2_GRAMMAR_VERSION, is_offline(), sha256_hex(), verify_sha(); BDD tests pure library functions (no env var mutation, no network); no blockers.";
    }
    part rustS4StandupGateR1 : TestResult { :>> id = "56789abc-aaaa-4d0e-a1b2-c3d4e5f6a7ba"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ebc38f3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS4ImplementGate : Test {
        :>> id = "6789abcd-bbbb-4e1f-b2c3-d4e5f6a7b8cb";
        :>> title = "Sprint 4 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "build.rs authored with SYSML_V2_SPEC_URL + SYSML_V2_GRAMMAR_SHA constants, offline check, ureq fetch, SHA verify, actionable mismatch message; spec_compat.rs authored with constants + is_offline + sha256_hex + verify_sha + ShaMismatch; .cargo/config.toml SYSML_V2_SPEC_OFFLINE=1 default; 3 spec_compat BDD scenarios green; 68 total tests pass; clippy -D warnings clean.";
    }
    part rustS4ImplementGateR1 : TestResult { :>> id = "789abcde-cccc-4f2a-c3d4-e5f6a7b8c9dc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ebc38f3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS4ReviewGate : Test {
        :>> id = "89abcdef-dddd-4a3b-d4e5-f6a7b8c9d0ed";
        :>> title = "Sprint 4 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: build.rs is standalone (no lib import); spec_compat.rs has no.unwrap() in production code; process::exit(1) used instead of panic! for clean error message; SHA placeholder clearly documented as not-yet-pinned sentinel; .cargo/config.toml force=false allows CI override; BDD tests pure functions (no network, no env var mutation); sha2+hex in both deps and build-deps; version constant is compile-time env!(); is_offline() tests runtime env var for completeness.";
    }
    part rustS4ReviewGateR1 : TestResult { :>> id = "9abcdef0-eeee-4b4c-e5f6-a7b8c9d0e1fe"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ebc38f3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS4CloseOutGate : Test {
        :>> id = "abcdef01-ffff-4c5d-f6a7-b8c9d0e1f2af";
        :>> title = "Sprint 4 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRustS4SpecCompatDoDR1 recorded (pass); rustS4SpecCompatDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 4 complete.";
    }
    part rustS4CloseOutGateR1 : TestResult { :>> id = "afcf84bf-cfce-4464-b3d9-b62dbe698ad8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }

    verification rustS4RetroGate : Test {
        :>> id = "bcdef012-0000-4d6e-a7b8-c9d0e1f2a3b0";
        :>> title = "Sprint 4 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 4 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part rustS4RetroGateR1 : TestResult { :>> id = "eee1cdb9-d5f4-4304-b3d9-b62dc1573038"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
