// =====================================================================
// ProjectDeliveryRustS8 — Sprint 8 delivery run: runtimeParser.
// Story: runtimeParser — AST-based indexer replacing orient.rs text-scan;
//   Succession.is_ordering_only field; parity with query.py orient output.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS8 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 8 Story
    // ====================================================================
    part runtimeParserStory : Story {
        :>> id = "d0e1f2a3-b4c5-4d6e-9f0a-1b2c3d4e5f6a";
        :>> title = "Sprint 8: runtimeParser — AST-driven indexer replacing text-scan; is_ordering_only; parity";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 2;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunRustS8 {
        action storyRuntimeParser;
        verification storyRuntimeParserDoD : Test { :>> id = "e1f2a3b4-c5d6-4e7f-a08b-2c3d4e5f6a7b"; :>> method = VerificationMethod::test; :>> procedureText = "Succession.is_ordering_only: bool added to AST (D0025); parse_action_def_body + parse_hash_item updated; indexer.rs extracts tasks/succs/DoDs/results from AST; orient::compute uses indexer; ordering-only edges handled; package-level items indexed; 8 orient BDD scenarios + all existing tests green; sysmlv2 orient 60 done / same ready list (parity); clippy clean."; }
        part storyRuntimeParserDoDR1 : TestResult { :>> id = "7ab0c1d2-e3f4-4b5c-de6f-0e1f2a3b4c5d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ec2c08"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification runtimeParserRefineGate : Test {
        :>> id = "f2a3b4c5-d6e7-4f8a-b19c-3d4e5f6a7b8c";
        :>> title = "Sprint 8 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: Succession.is_ordering_only added (D0025) to preserve #OrderingOnly marker in AST; indexer.rs module built with extract(tracking) → ExtractedIndex; orient.rs text-scan functions removed, replaced by indexer; git helpers + compute_orient unchanged; new BDD: ordering-only succession + package-level items; parity on real project required; DoD accepted.";
    }
    part runtimeParserRefineGateR1 : TestResult { :>> id = "03b4c5d6-e7f8-4a9b-bc0d-4e5f6a7b8c9d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "276c833"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification runtimeParserStandupGate : Test {
        :>> id = "14c5d6e7-f8a9-4b0c-cd1e-5f6a7b8c9d0e";
        :>> title = "Sprint 8 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation: Succession.is_ordering_only set in parse_action_def_body for #OrderingOnly + in parse_hash_item for pkg-level; indexer.rs 200 lines; orient.rs rewritten to use indexer (text-scan functions gone); lib.rs: pub mod indexer + skip is_ordering_only in compute_orient_state; no blocking issues.";
    }
    part runtimeParserStandupGateR1 : TestResult { :>> id = "25d6e7f8-a9b0-4c1d-de2f-6a7b8c9d0e1f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "276c833"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification runtimeParserImplementGate : Test {
        :>> id = "36e7f8a9-b0c1-4d2e-ef3a-7b8c9d0e1f2a";
        :>> title = "Sprint 8 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "All 8 orient BDD scenarios pass (6 existing + ordering-only + package-level); 27 parser unit tests pass; 10 lexer BDD scenarios pass; 6 semantic BDD scenarios pass; 3 compat BDD scenarios pass; sysmlv2 orient = 60 done / 12 outstanding (parity with pre-sprint); clippy -D warnings clean.";
    }
    part runtimeParserImplementGateR1 : TestResult { :>> id = "8bc1d2e3-f405-4c6d-ef7a-1f2a3b4c5d6e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ec2c08"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification runtimeParserReviewGate : Test {
        :>> id = "47f8a9b0-c1d2-4e3f-f04b-8c9d0e1f2a3b";
        :>> title = "Sprint 8 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: indexer.rs clean (no text-scan, AST-only); orient.rs all text-scan functions removed; Succession.is_ordering_only correctly propagated through parser and indexer; compute_orient unchanged (same git logic); parity confirmed on real project; D0025 recorded.";
    }
    part runtimeParserReviewGateR1 : TestResult { :>> id = "9cd2e3f4-a5b6-4d7e-f08b-2a3b4c5d6e7f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ec2c08"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification runtimeParserCloseOutGate : Test {
        :>> id = "58a9b0c1-d2e3-4f4a-a15c-9d0e1f2a3b4c";
        :>> title = "Sprint 8 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRuntimeParserDoDR1 recorded (pass); runtimeParserDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 8 complete.";
    }
    part runtimeParserCloseOutGateR1 : TestResult { :>> id = "2f3a4b5c-d6e7-4f8a-e794-1c2d3e4f5a6b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c431705"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }

    verification runtimeParserRetroGate : Test {
        :>> id = "69b0c1d2-e3f4-4a5b-b26d-0e1f2a3b4c5d";
        :>> title = "Sprint 8 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 8 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part runtimeParserRetroGateR1 : TestResult { :>> id = "3a4b5c6d-e7f8-4a9b-f8a5-2d3e4f5a6b7c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c431705"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }
}
