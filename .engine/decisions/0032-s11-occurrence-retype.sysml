package Decision0032 {
    private import EngineWork::*;

    part d0032 : Decision {
        :>> id = "fc277cad-1e2f-4a3b-cde4-5f6a7b8c9d0e";
        :>> title = "S11 occurrenceRetype: TestResult retyped to occurrence def; Decision stays part def";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 11 (occurrenceRetype): TestResult is currently `part def TestResult :> Element`. A test result is semantically an event (a performance of a verification at a specific moment), making `occurrence def` more correct. The spike verified that: (1) `occurrence def X :> partDef` parses; (2) attributes work (avoid reserved names); (3) `part instance : OccurrenceDef { ... }` continues to parse — no instance migration needed.";
        :>> decision = "VERDICT GREEN: retype TestResult to `occurrence def TestResult :> Element`; Decision stays `part def`.";
        :>> rationale = "A test result is semantically an event (a verification performed at a moment), so occurrence def is more correct, while Decision is a document-like record, not an event, so it stays part def. The spike confirmed the retype parses and existing `part <name> : TestResult { ... }` instances continue to parse unchanged — no instance migration needed. Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "TestResult is now `occurrence def` in EngineVerification. validate_schema passes. No instance migration needed. occurrence def with attributes (avoiding reserved names) documented in sysmlv2-syntax-notes.md.";
    }
}
