// =====================================================================
// ProjectDeliveryRustS5 — Sprint 5 delivery run: sysmlv2 CLI binary.
// Story: rustS5CLI — sysmlv2-cli binary with check/validate/ls subcommands;
//   lib.rs exports check_files/validate_root/collect_sysml; 3 BDD scenarios.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS5 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 5 Story
    // ====================================================================
    part rustS5CLIStory : Story {
        :>> id = "5a6b7c8d-9e0f-4a1b-2c3d-4e5f6a7b8c9d";
        :>> title = "Sprint 5: sysmlv2 CLI binary — check, validate, ls subcommands";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 2;
    }

    // ====================================================================
    // Delivery backlog — DoD read by query.py.
    // ====================================================================
    action def DeliveryRunRustS5 {
        action storyRustS5CLI;
        verification storyRustS5CLIDoD : Test { :>> id = "6b7c8d9e-0f1a-4b2c-3d4e-5f6a7b8c9d0e"; :>> method = VerificationMethod::test; :>> procedureText = "sysmlv2-cli lib.rs exports check_files/validate_root/collect_sysml; main.rs dispatches check/validate/ls; 3 BDD scenarios green; 71 total tests pass; clippy -D warnings clean."; }
        part storyRustS5CLIDoDR1 : TestResult { :>> id = "7c8d9e0f-1a2b-4c3d-4e5f-6a7b8c9d0e1f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "18e5a51"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS5RefineGate : Test {
        :>> id = "8d9e0f1a-2b3c-4d4e-5f6a-7b8c9d0e1f2a";
        :>> title = "Sprint 5 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "sysmlv2-cli binary approach agreed: lib.rs (check_files/validate_root/collect_sysml) + main.rs dispatching check/validate/ls; BDD via cucumber-rs testing lib functions; 3 scenarios outlined; no clap dep; sha2+hex+ureq already in parser deps; DoD accepted.";
    }
    part rustS5RefineGateR1 : TestResult { :>> id = "9e0f1a2b-3c4d-4e5f-6a7b-8c9d0e1f2a3b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0d379c3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS5StandupGate : Test {
        :>> id = "0f1a2b3c-4d5e-4f6a-7b8c-9d0e1f2a3b4c";
        :>> title = "Sprint 5 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation approach: lib.rs with check_files (lex+parse each file), validate_root (register .engine/ schema then validate .tracking/), collect_sysml (recursive dir walker); main.rs manual arg dispatch; BDD tests exercise lib functions directly with temp files (atomic counter avoids race); no blockers.";
    }
    part rustS5StandupGateR1 : TestResult { :>> id = "1a2b3c4d-5e6f-4a7b-8c9d-0e1f2a3b4c5e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0d379c3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS5ImplementGate : Test {
        :>> id = "2b3c4d5e-6f7a-4b8c-9d0e-1f2a3b4c5d6f";
        :>> title = "Sprint 5 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "sysmlv2-cli/src/lib.rs authored with check_files/validate_root/collect_sysml/Report/CheckError; sysmlv2-cli/src/main.rs dispatches check/validate/ls subcommands; cli_bdd.rs 3 scenarios green; 71 total tests pass; clippy -D warnings clean; Cargo.toml [[test]] cli_bdd harness=false.";
    }
    part rustS5ImplementGateR1 : TestResult { :>> id = "3c4d5e6f-7a8b-4c9d-0e1f-2a3b4c5d6e7a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "18e5a51"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS5ReviewGate : Test {
        :>> id = "4d5e6f7a-8b9c-4d0e-1f2a-3b4c5d6e7f8b";
        :>> title = "Sprint 5 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: lib.rs no .unwrap() in production code; validate_root registers schema without validating it (schema may use ScalarValues::*); collect_sysml sorts output for determinism; BDD uses AtomicUsize counter to avoid temp file races; main.rs uses process::exit() for CI-clean exit codes; report_status human-readable + stderr for non-zero.";
    }
    part rustS5ReviewGateR1 : TestResult { :>> id = "5e6f7a8b-9c0d-4e1f-2a3b-4c5d6e7f8a9c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "18e5a51"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS5CloseOutGate : Test {
        :>> id = "6f7a8b9c-0d1e-4f2a-3b4c-5d6e7f8a9b0d";
        :>> title = "Sprint 5 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRustS5CLIDoDR1 recorded (pass); rustS5CLIDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 5 complete.";
    }
    part rustS5CloseOutGateR1 : TestResult { :>> id = "0c19af9a-9974-4a00-b3d9-b62dc5b48108"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }

    verification rustS5RetroGate : Test {
        :>> id = "7a8b9c0d-1e2f-4a3b-4c5d-6e7f8a9b0c1e";
        :>> title = "Sprint 5 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 5 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part rustS5RetroGateR1 : TestResult { :>> id = "46b63018-542d-43b8-b3d9-b62dc935d124"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
