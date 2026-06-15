// =====================================================================
// ProjectDeliveryRustS2 — Sprint 2 delivery run: recursive-descent parser + AST.
// Story: rustS2Parser — parser producing typed AST for the full engine dialect;
//   all .tracking/ and .engine/ files parse without error; BDD + proptest green.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS2 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 2 Story
    // ====================================================================
    part rustS2ParserStory : Story {
        :>> id = "a1b2c3d4-2222-4e5f-b6c7-d8e9f0a1b2c3";
        :>> title = "Sprint 2: SysML v2 engine-dialect recursive-descent parser";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    // ====================================================================
    // Delivery backlog — DoD read by query.py.
    // ====================================================================
    action def DeliveryRunRustS2 {
        action storyRustS2Parser;
        verification storyRustS2ParserDoD : Test { :>> id = "b2c3d4e5-3333-4f6a-c7d8-e9f0a1b2c3d4"; :>> method = VerificationMethod::test; :>> procedureText = "Recursive-descent parser produces typed AST; all .tracking/ and .engine/ files parse without error; 27 unit + 6 BDD + 3 proptest + 2 integration tests green; clippy -D warnings clean; no .unwrap() in src/."; }
        part storyRustS2ParserR1 : TestResult { :>> id = "b2c3d4e5-3333-4f6b-c7d8-e9f0a1b2c3d5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "513580f"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS2RefineGate : Test {
        :>> id = "c3d4e5f6-4444-4a7b-d8e9-f0a1b2c3d4e5";
        :>> title = "Sprint 2 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "BDD parser feature file drafted (6 scenarios: empty package, part-with-attr, string-concat, block-comment, part-def-skip, error-case); AST node types agreed (Package/Item/Value hierarchy); error recovery strategy (skip-to-semicolon / skip-brace-block); DoD accepted.";
    }
    part rustS2RefineGateR1 : TestResult { :>> id = "d4e5f6a7-5555-4b8c-e9f0-a1b2c3d4e5f6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "513580f"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS2StandupGate : Test {
        :>> id = "e5f6a7b8-6666-4c9d-f0a1-b2c3d4e5f6a7";
        :>> title = "Sprint 2 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation approach: private Parser struct owning Vec<Token>; parse_item dispatches to sub-functions for each construct type; skip_item (stop at ; or { block end) for unknown constructs; string concatenation via Plus token eager-fold; block comments in skip_trivia; new tokens Plus/Slash/LBracket/RBracket/Doc added to lexer; no blockers.";
    }
    part rustS2StandupGateR1 : TestResult { :>> id = "f6a7b8c9-7777-4d0e-a1b2-c3d4e5f6a7b8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "513580f"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS2ImplementGate : Test {
        :>> id = "a7b8c9d0-8888-4e1f-b2c3-d4e5f6a7b8c9";
        :>> title = "Sprint 2 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "ast.rs + parser.rs authored; token.rs: Span::Copy added, Plus/Slash/LBracket/RBracket/Doc tokens added; lexer.rs: block-comment skip_trivia, new single-char tokens; parser.rs: 27 unit + 6 BDD + 3 proptest + 2 integration tests all pass; clippy -D warnings clean.";
    }
    part rustS2ImplementGateR1 : TestResult { :>> id = "b8c9d0e1-9999-4f2a-c3d4-e5f6a7b8c9d0"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "513580f"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS2ReviewGate : Test {
        :>> id = "c9d0e1f2-aaaa-4a3b-d4e5-f6a7b8c9d0e1";
        :>> title = "Sprint 2 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: Parser struct private; parse() public entry point; no .unwrap() in src/; skip_item/skip_brace_block/skip_bracket_block cover all unknown-construct shapes; parse_attribute_body tolerant of non-:>> items; Part/Verification followed by Def correctly skip the type definition; integration test walks all 52+ engine+tracking files; BDD feature files match step definitions; proptest covers package name, part attr, string concat.";
    }
    part rustS2ReviewGateR1 : TestResult { :>> id = "d0e1f2a3-bbbb-4b4c-e5f6-a7b8c9d0e1f2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "513580f"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS2CloseOutGate : Test {
        :>> id = "e1f2a3b4-cccc-4c5d-f6a7-b8c9d0e1f2a3";
        :>> title = "Sprint 2 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRustS2ParserR1 recorded (pass); rustS2ParserDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 2 complete.";
    }

    verification rustS2RetroGate : Test {
        :>> id = "f2a3b4c5-dddd-4d6e-a7b8-c9d0e1f2a3b4";
        :>> title = "Sprint 2 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 2 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
}
