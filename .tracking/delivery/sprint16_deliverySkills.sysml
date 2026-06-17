// =====================================================================
// ProjectDeliveryS16 — Sprint 16: deliverySkills (3 pts), W3 of the D0046 audit program.
// Story: build + test/verify delivery skills (cargo build/test, validators, parity);
//        reconcile repo-push to the main-only policy (issue009).
// Total: 3 pts  Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS16 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 16 Story
    // ====================================================================
    part deliverySkillsStory : Story {
        :>> id = "2e1f4b07-68ba-4f5d-aceb-1d3e5f7a9b0c";
        :>> title = "Sprint 16: deliverySkills (3 pts) — build + test/verify skills; reconcile repo-push to main-only";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient (mirrors backlog deliverySkillsDoD).
    // ====================================================================
    action def DeliveryRunS16 {
        action storyDeliverySkills;
        verification storyDeliverySkillsDoD : Test { :>> id = "3f2a5c18-79cb-4a6e-bdfc-2e4f6a8b0c1d"; :>> method = VerificationMethod::inspect; :>> procedureText = "deliverySkillsDoR in backlog (pass): build skill + test/verify skill created under .engine/skills/ and registered in skills-registry.sysml; repo-push reconciled to main-only (issue009); doc-sync complete; validate_all green."; }
        part storyDeliverySkillsDoDR1 : TestResult { :>> id = "6c5d8042-a153-4f91-b264-5b7c9d1e3f51"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "14b7050"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification deliverySkillsRefineGate : Test {
        :>> id = "4a3b6d29-8adc-4b7f-ce0d-3f5a7b9c1d2e";
        :>> title = "Sprint 16 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: create two AI skills under .engine/skills/ — (1) build (cargo build --release + run validators / sysmlv2 self-check; report compile + validation status) and (2) test-verify (cargo test + clippy -D warnings + BDD + Python<->Rust orient parity check). Register both in skills-registry.sysml with purpose/triggerCondition/writePolicy=readOnly. Reconcile repo-push SKILL.md to the main-only direct-commit policy (issue009): drop branch-off-main/PR mandates, reflect auto-push hook, writePolicy stays direct. doc-sync any doc claims touched. estimatedPoints=3 (medium: 2 skill docs + 1 reconcile + registry edit). Unblocked (ready in orient, parallel with rustToolchainFix). DoD accepted.";
    }
    part deliverySkillsRefineGateR1 : TestResult { :>> id = "5b4c7e30-9bed-4c80-df1e-4a6b8c0d2e3f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "14b7050"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification deliverySkillsStandupGate : Test {
        :>> id = "6c5d8f41-acfe-4d91-e02f-5b7c9d1e3f40";
        :>> title = "Sprint 16 standup gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Sprint 16 refine gate passed (14b7050). deliverySkills: 2 new skills (build, test-verify) + repo-push reconcile + registry edits. No blockers; parallel with rustToolchainFix.";
    }
    part deliverySkillsStandupGateR1 : TestResult { :>> id = "7d6e9153-b264-40a2-c375-6c8d0e2f4f99"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "14b7050"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification deliverySkillsImplementGate : Test {
        :>> id = "7d6e9052-bd0f-4ea2-f130-6c8d0e2f4051";
        :>> title = "Sprint 16 implement gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "build/SKILL.md + test-verify/SKILL.md authored; repo-push/SKILL.md reconciled to main-only; skills-registry.sysml registers both new skills; validate_instances green (registry parses); storyDeliverySkillsDoR/deliverySkillsDoD recorded.";
    }
    part deliverySkillsImplementGateR1 : TestResult { :>> id = "7d6e9153-b264-40a2-c375-6c8d0e2f4062"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "14b7050"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification deliverySkillsReviewGate : Test {
        :>> id = "8e7f0163-ce10-4fb3-0241-7d9e1f305162";
        :>> title = "Sprint 16 review gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: skills follow the established SKILL.md shape (frontmatter + vocabulary + behavioral instructions + anti-patterns); registry entries correct; repo-push no longer contradicts main-only; doc-sync complete; no regressions.";
    }
    part deliverySkillsReviewGateR1 : TestResult { :>> id = "8e7f0264-c375-41b3-d486-7d9e1f305173"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "14b7050"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification deliverySkillsCloseOutGate : Test {
        :>> id = "9f801274-df21-40c4-1352-8e0f20416273";
        :>> title = "Sprint 16 closeOut gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyDeliverySkillsDoD recorded (pass); deliverySkillsDoD in backlog (pass); validate_all green; user accepts Sprint 16 complete.";
    }
    part deliverySkillsCloseOutGateR1 : TestResult { :>> id = "9f801385-e032-41d5-2463-9f1031527385"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0f326b5"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }

    verification deliverySkillsRetroGate : Test {
        :>> id = "a0912385-e032-41d5-2463-9f1031527384";
        :>> title = "Sprint 16 retro gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 16 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part deliverySkillsRetroGateR1 : TestResult { :>> id = "a0912496-f143-41d5-2463-9f1031527396"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0f326b5"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }
}
