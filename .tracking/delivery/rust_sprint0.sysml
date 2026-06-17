// =====================================================================
// ProjectDeliveryRustS0 — Sprint 0 delivery run: Cargo workspace + quality gates.
// Story: rustS0Workspace — Cargo workspace, #![deny(...)], BDD harness.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS0 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 0 Story
    // ====================================================================
    part rustS0WorkspaceStory : Story {
        :>> id = "a1b2c3d4-1111-4e5f-a6b7-c8d9e0f1a2b3";
        :>> title = "Sprint 0: Cargo workspace + deny(warnings/clippy) + BDD harness";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 1;
    }

    // ====================================================================
    // Delivery backlog — DoD read by query.py.
    // ====================================================================
    action def DeliveryRunRustS0 {
        action storyRustS0Workspace;
        verification storyRustS0WorkspaceDoD : Test { :>> id = "b2c3d4e5-2222-4f6a-b7c8-d9e0f1a2b3c4"; :>> method = VerificationMethod::test; :>> procedureText = "Cargo.toml workspace at repo root; sysmlv2-parser and sysmlv2-cli crates compile with #![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]; cargo test --workspace passes with zero warnings; pre-commit hook runs cargo clippy -D warnings on staged .rs/Cargo files; validate_all.py still green."; }
        part storyRustS0WorkspaceR1 : TestResult { :>> id = "c3d4e5f6-3333-4a7b-c8d9-e0f1a2b3c4d5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3abbe58"; :>> judgedAt = "2026-06-12"; :>> judgedBy = "claudeOpus"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS0RefineGate : Test {
        :>> id = "d4e5f6a7-4444-4b8c-d9e0-f1a2b3c4d5e6";
        :>> title = "Sprint 0 refine gate (DoR)";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Workspace layout agreed (sysmlv2-parser lib + sysmlv2-cli bin); deny attributes decided; BDD framework (cucumber-rs) selected; no blocking dependencies.";
    }
    part rustS0RefineGateR1 : TestResult { :>> id = "e5f6a7b8-5555-4c9d-e0f1-a2b3c4d5e6f7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3abbe58"; :>> judgedAt = "2026-06-12"; :>> judgedBy = "claudeOpus"; }

    verification rustS0StandupGate : Test {
        :>> id = "f6a7b8c9-6666-4d0e-f1a2-b3c4d5e6f7a8";
        :>> title = "Sprint 0 standup gate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation approach stated: workspace at repo root, two crates, deny macros in lib.rs and main.rs, hook updated; no new blockers.";
    }
    part rustS0StandupGateR1 : TestResult { :>> id = "a7b8c9d0-7777-4e1f-a2b3-c4d5e6f7a8b9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3abbe58"; :>> judgedAt = "2026-06-12"; :>> judgedBy = "claudeOpus"; }

    verification rustS0ImplementGate : Test {
        :>> id = "b8c9d0e1-8888-4f2a-b3c4-d5e6f7a8b9c0";
        :>> title = "Sprint 0 implement gate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "Cargo.toml workspace exists; both crates compile; cargo clippy --workspace --all-targets -D warnings passes; cargo test --workspace passes; pre-commit hook updated.";
    }
    part rustS0ImplementGateR1 : TestResult { :>> id = "e2f3a4b5-cccc-4d6e-f7a8-b9c0d1e2f3a4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdffd53"; :>> judgedAt = "2026-06-12"; :>> judgedBy = "claudeOpus"; }

    verification rustS0ReviewGate : Test {
        :>> id = "c9d0e1f2-9999-4a3b-c4d5-e6f7a8b9c0d1";
        :>> title = "Sprint 0 review gate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: workspace layout correct; deny attributes present in both crates; hook addition is surgical (no regressions to SysML path); Cargo.lock committed.";
    }
    part rustS0ReviewGateR1 : TestResult { :>> id = "f3a4b5c6-dddd-4e7f-a8b9-c0d1e2f3a4b5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdffd53"; :>> judgedAt = "2026-06-12"; :>> judgedBy = "claudeOpus"; }

    verification rustS0CloseOutGate : Test {
        :>> id = "d0e1f2a3-aaaa-4b4c-d5e6-f7a8b9c0d1e2";
        :>> title = "Sprint 0 closeOut gate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "rustS0WorkspaceR1 recorded (pass); storyRustS0WorkspaceR1 recorded (pass); rustS0WorkspaceDoRR1 recorded in backlog (pass); validate_all green; user accepts Sprint 0 complete.";
    }
    part rustS0CloseOutGateR1 : TestResult { :>> id = "a4b5c6d7-eeee-4f8a-b9c0-d1e2f3a4b5c6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdffd53"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "wweatherholtz"; }

    verification rustS0RetroGate : Test {
        :>> id = "e1f2a3b4-bbbb-4c5d-e6f7-a8b9c0d1e2f3";
        :>> title = "Sprint 0 retro gate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 0 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part rustS0RetroGateR1 : TestResult { :>> id = "b5c6d7e8-ffff-4a9b-c0d1-e2f3a4b5c6d7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdffd53"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "wweatherholtz"; }
}
