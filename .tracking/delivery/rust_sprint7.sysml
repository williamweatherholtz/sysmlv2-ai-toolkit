// =====================================================================
// ProjectDeliveryRustS7 — Sprint 7 delivery run: done-detection align + whats-next.
// Story: rustS7QueryAlign — fix DoDR/R naming detection in orient.rs + lib.rs +
//   query.py; add sysmlv2 whats-next subcommand.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS7 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 7 Story
    // ====================================================================
    part rustS7QueryAlignStory : Story {
        :>> id = "e2f3a4b5-c6d7-4e8f-9a0b-1c2d3e4f5a6b";
        :>> title = "Sprint 7: align done-detection for DoDR + legacy R naming; sysmlv2 whats-next subcommand";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 1;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunRustS7 {
        action storyRustS7QueryAlign;
        verification storyRustS7QueryAlignDoD : Test { :>> id = "f3a4b5c6-d7e8-4f9a-ab0c-2d3e4f5a6b7c"; :>> method = VerificationMethod::test; :>> procedureText = "dod_result_is_pass handles DoDR + legacy R naming; parse_result_header handles both; query.py regex fixed; sysmlv2 whats-next outputs ready list; 6 orient + 6 cli BDD scenarios green; clippy clean."; }
        part storyRustS7QueryAlignDoDR1 : TestResult { :>> id = "4de5f6a7-b8c9-4d0e-bf2f-3a4b5c6d7e8f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2811f5c"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS7RefineGate : Test {
        :>> id = "a4b5c6d7-e8f9-4a0b-8c1d-3e4f5a6b7c8d";
        :>> title = "Sprint 7 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "done detection approach agreed: lib.rs dod_result_is_pass extended with or_else fallback for {task}R{n}; query.py _RESULT_LINE regex updated to non-greedy + optional DoD infix; sysmlv2 whats-next calls orient::compute().ready and prints one per line; orient.feature 2 new scenarios; D0023 records CHANGE; DoD accepted.";
    }
    part rustS7RefineGateR1 : TestResult { :>> id = "0ab1c2d3-e4f5-4a6b-8c7d-9e0f1a2b3c4d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "adf4c40"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS7StandupGate : Test {
        :>> id = "b5c6d7e8-f9a0-4b1c-9d2e-4f5a6b7c8d9e";
        :>> title = "Sprint 7 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation: lib.rs dod_result_is_pass uses or_else chain (DoDR prefix first, legacy R prefix fallback); orient.rs parse_result_header rfind(DoDR) + R-scan fallback; query.py non-greedy regex; main.rs cmd_whats_next calls orient::compute(root).ready; no blockers.";
    }
    part rustS7StandupGateR1 : TestResult { :>> id = "1bc2d3e4-f5a6-4b7c-9d8e-0f1a2b3c4d5e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "adf4c40"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS7ImplementGate : Test {
        :>> id = "c6d7e8f9-a0b1-4c2d-ae3f-5a6b7c8d9e0f";
        :>> title = "Sprint 7 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "lib.rs dod_result_is_pass: or_else chain (DoDRprefix first, then legacy R prefix); whats_next_root exported; query.py _RESULT_LINE non-greedy; main.rs cmd_whats_next + dispatch; orient.feature: 6 scenarios (4 original + invalidEvidence + legacy + whats-next); cli.feature: 6 scenarios; 12 scenarios 50 steps green; clippy -D warnings clean.";
    }
    part rustS7ImplementGateR1 : TestResult { :>> id = "2bc3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2811f5c"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS7ReviewGate : Test {
        :>> id = "d7e8f9a0-b1c2-4d3e-bf40-6a7b8c9d0e1f";
        :>> title = "Sprint 7 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: dod_result_is_pass or_else fallback non-ambiguous (DoDR prefix checked first so DoDR1 never matches legacy R prefix); query.py non-greedy regex correctly keys both naming styles; cmd_whats_next clean (no allocation, uses orient::compute); D0023 recorded.";
    }
    part rustS7ReviewGateR1 : TestResult { :>> id = "3cd4e5f6-a7b8-4c9d-ae1f-2a3b4c5d6e7f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2811f5c"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    part rustS7ReviewGateR2 : TestResult { :>> id = "5ef6a7b8-c9d0-4e1f-9a2b-3c4d5e6f7a8b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "56ba800"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS7CloseOutGate : Test {
        :>> id = "e8f9a0b1-c2d3-4e4f-8a51-7b8c9d0e1f2a";
        :>> title = "Sprint 7 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRustS7QueryAlignDoDR1 recorded (pass); rustS7QueryAlignDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 7 complete.";
    }

    verification rustS7RetroGate : Test {
        :>> id = "f9a0b1c2-d3e4-4f5a-9b62-8c9d0e1f2a3b";
        :>> title = "Sprint 7 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 7 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
}
