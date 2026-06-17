// =====================================================================
// ProjectDeliveryRustS9 — Sprint 9 delivery run: writeApi.
// Story: writeApi — Rust CLI write commands enforcing writePolicy/ids/append-only;
//   sysmlv2 append-result + add-task subcommands; CLAUDE.md §4 updated.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS9 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 9 Story
    // ====================================================================
    part writeApiStory : Story {
        :>> id = "ca1b2c3d-e4f5-4a6b-bc7d-8e9f0a1b2c3d";
        :>> title = "Sprint 9: writeApi — append-result + add-task enforce UUID/append-only; direct editing retired";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 2;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunRustS9 {
        action storyWriteApi;
        verification storyWriteApiDoD : Test { :>> id = "db2c3d4e-f5a6-4b7c-cd8e-9f0a1b2c3d4e"; :>> method = VerificationMethod::test; :>> procedureText = "write.rs module: append_result enforces append-only + auto-UUID; add_task enforces no-dup + auto-UUID; sysmlv2 append-result + add-task subcommands; 7 write BDD scenarios + all existing tests green; CLAUDE.md §4 updated to retire direct editing; clippy clean."; }
        part storyWriteApiDoDR1 : TestResult { :>> id = "ab5c6d7e-f8a9-4b0c-ef1d-2e3f4a5b6c7d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "58ba2c3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification writeApiRefineGate : Test {
        :>> id = "ec3d4e5f-a6b7-4c8d-de9f-0a1b2c3d4e5f";
        :>> title = "Sprint 9 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: write.rs uses parser for validation + text insertion for write path; append_result: parse→find N→gen UUID→text insert after last result or DoD line; add_task: find action def→check dup→gen UUID→insert before closing brace; subcommands parse --file/--task/--sha/--verdict/--judged-by flags; 7 BDD scenarios agreed; CLAUDE.md §4 updates scoped; DoD accepted.";
    }
    part writeApiRefineGateR1 : TestResult { :>> id = "fd4e5f6a-b7c8-4d9e-efa0-1b2c3d4e5f6a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6d9fdb5"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification writeApiStandupGate : Test {
        :>> id = "ae5f6a7b-c8d9-4e0f-f0b1-2c3d4e5f6a7b";
        :>> title = "Sprint 9 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation: write.rs with WriteError enum + gen_uuid (SystemTime+counter) + append_result + add_task; lib.rs: pub mod write; main.rs: append-result + add-task subcommands with flag parsing; write_bdd.rs with 7 scenarios; write.feature; Cargo.toml: write_bdd test entry; CLAUDE.md §4 updated; no blocking issues.";
    }
    part writeApiStandupGateR1 : TestResult { :>> id = "bf6a7b8c-d9e0-4f1a-a1c2-3d4e5f6a7b8c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6d9fdb5"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification writeApiImplementGate : Test {
        :>> id = "ca7b8c9d-e0f1-4a2b-b2d3-4e5f6a7b8c9d";
        :>> title = "Sprint 9 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "All 7 write BDD scenarios pass; 8 orient BDD scenarios pass; all other existing tests pass; clippy -D warnings clean.";
    }
    part writeApiImplementGateR1 : TestResult { :>> id = "bc6d7e8f-a9b0-4c1d-f02e-3f4a5b6c7d8e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "58ba2c3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification writeApiReviewGate : Test {
        :>> id = "db8c9d0e-f1a2-4b3c-c3e4-5f6a7b8c9d0e";
        :>> title = "Sprint 9 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: write.rs clean (no panics, no unwrap in src/); append_result correctly enforces N+1 and append-only; add_task correctly enforces no-dup; UUID generation deterministic-free; CLAUDE.md §4 updated; D0027 recorded.";
    }
    part writeApiReviewGateR1 : TestResult { :>> id = "cd7e8f9a-b0c1-4d2e-a13f-4a5b6c7d8e9f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "58ba2c3"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification writeApiCloseOutGate : Test {
        :>> id = "ec9d0e1f-a2b3-4c4d-d4f5-6a7b8c9d0e1f";
        :>> title = "Sprint 9 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyWriteApiDoDR1 recorded (pass); writeApiDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 9 complete.";
    }
    part writeApiCloseOutGateR1 : TestResult { :>> id = "4b5c6d7e-f8a9-4b0c-a9b6-3e4f5a6b7c8d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c431705"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }

    verification writeApiRetroGate : Test {
        :>> id = "fd0e1f2a-b3c4-4d5e-e5a6-7b8c9d0e1f2a";
        :>> title = "Sprint 9 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 9 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part writeApiRetroGateR1 : TestResult { :>> id = "5c6d7e8f-a9b0-4c1d-bac7-4f5a6b7c8d9e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c431705"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }
}
