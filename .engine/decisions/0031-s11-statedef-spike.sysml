package Decision0031 {
    private import EngineWork::*;

    part d0031 : Decision {
        :>> id = "eb166b9c-0d1e-4f2a-bcd3-4e5f6a7b8c9d";
        :>> title = "S11 stateDefSpike: pilot 0.59.0 does not support state def body syntax; transitions stay String[*]";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 11 (stateDefSpike): WorkflowDefinition.transitions is currently `String[*]` (values like 'brief->brief-review'). The stateDefSpike task asked: can native SysML v2 `state def` with `entry state` / `then` transition body syntax replace these strings? The spike was run against pilot kernel 0.59.0.";
        :>> decisionText = "VERDICT NOT GREEN: bare `state def X;` parses (ok), but the body syntax (`entry state`, `then` keyword) produces 'no viable alternative at input entry' errors in pilot 0.59.0. The transitions-in-body grammar is not implemented. WorkflowDefinition.transitions : String[*] stays unchanged. Revisit when a newer toolchain ships (tracked by toolchainWatch). Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "WorkflowDefinition.transitions stays as String[*]. No schema change. State def body syntax added to sysmlv2-syntax-notes.md as a confirmed gap in pilot 0.59.0.";
    }
}
