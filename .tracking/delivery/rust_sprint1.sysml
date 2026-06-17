// =====================================================================
// ProjectDeliveryRustS1 — Sprint 1 delivery run: lexer tokenizer.
// Story: rustS1Lexer — tokenizer for the engine dialect; proptest-covered.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS1 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 1 Story
    // ====================================================================
    part rustS1LexerStory : Story {
        :>> id = "b2c3d4e5-1111-4f6a-b7c8-d9e0f1a2b3c4";
        :>> title = "Sprint 1: SysML v2 engine-dialect lexer (tokenizer)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    // ====================================================================
    // Delivery backlog — DoD read by query.py.
    // ====================================================================
    action def DeliveryRunRustS1 {
        action storyRustS1Lexer;
        verification storyRustS1LexerDoD : Test { :>> id = "c3d4e5f6-2222-4a7b-c8d9-e0f1a2b3c4d5"; :>> method = VerificationMethod::test; :>> procedureText = "Lexer tokenizes all token types in the engine dialect; proptest 3 strategies ≥256 cases each; 10 BDD scenarios green; 15 unit tests green; zero panics; no .unwrap() in src/; cargo test + cargo clippy -D warnings green."; }
        part storyRustS1LexerR1 : TestResult { :>> id = "d4e5f6a7-3333-4b8c-d9e0-f1a2b3c4d5e6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fc2e44e"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS1RefineGate : Test {
        :>> id = "e5f6a7b8-4444-4c9d-e0f1-a2b3c4d5e6f7";
        :>> title = "Sprint 1 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Token type list complete (21 keywords + 11 operators + 3 literal kinds + Eof); proptest strategies identified (arb_identifier, arb_simple_string, arb_enum_literal); lexer API agreed (tokenize(source, filename) -> Result<Vec<Token>, LexError>); DoD accepted.";
    }
    part rustS1RefineGateR1 : TestResult { :>> id = "f6a7b8c9-5555-4d0e-f1a2-b3c4d5e6f7a8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fc2e44e"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS1StandupGate : Test {
        :>> id = "a7b8c9d0-6666-4e1f-a2b3-c4d5e6f7a8b9";
        :>> title = "Sprint 1 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation approach: Lexer<'src> struct with CharIndices peekable; next_char returns (pos, ch, line, col); skip_trivia handles whitespace and // comments; scan_colon dispatches ::/:>>/:>/: ; keyword_or_ident maps keywords; three proptest strategies written TDD-first; no blockers.";
    }
    part rustS1StandupGateR1 : TestResult { :>> id = "b8c9d0e1-7777-4f2a-b3c4-d5e6f7a8b9c0"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fc2e44e"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS1ImplementGate : Test {
        :>> id = "c9d0e1f2-8888-4a3b-c4d5-e6f7a8b9c0d1";
        :>> title = "Sprint 1 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "error.rs + token.rs + lexer.rs authored; cargo test --workspace: 15 unit + 10 BDD + 3 proptest suites all pass; cargo clippy --workspace --all-targets -D warnings: clean.";
    }
    part rustS1ImplementGateR1 : TestResult { :>> id = "d0e1f2a3-9999-4b4c-d5e6-f7a8b9c0d1e2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fc2e44e"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS1ReviewGate : Test {
        :>> id = "e1f2a3b4-aaaa-4c5d-e6f7-a8b9c0d1e2f3";
        :>> title = "Sprint 1 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: LexError uses thiserror; Token/TokenKind/Span are pub with doc comments; Lexer struct is private; tokenize() is the single public entry point; no .unwrap() in src/; BDD feature file matches step definitions; proptest strategies cover ident/string/enum-literal.";
    }
    part rustS1ReviewGateR1 : TestResult { :>> id = "f2a3b4c5-bbbb-4d6e-f7a8-b9c0d1e2f3a4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fc2e44e"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeOpus"; }

    verification rustS1CloseOutGate : Test {
        :>> id = "a3b4c5d6-cccc-4e7f-a8b9-c0d1e2f3a4b5";
        :>> title = "Sprint 1 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRustS1LexerR1 recorded (pass); rustS1LexerDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 1 complete.";
    }
    part rustS1CloseOutGateR1 : TestResult { :>> id = "a3a9dbb2-0b6b-496c-b3d9-b62da0372bd8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }

    verification rustS1RetroGate : Test {
        :>> id = "b4c5d6e7-dddd-4f8a-b9c0-d1e2f3a4b5c6";
        :>> title = "Sprint 1 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 1 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part rustS1RetroGateR1 : TestResult { :>> id = "3c93a7d1-9017-459c-b3d9-b62da53e1b50"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
