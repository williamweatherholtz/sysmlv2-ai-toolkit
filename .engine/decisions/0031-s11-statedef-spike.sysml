package Decision0031 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0031 : Decision {
        :>> id = "eb166b9c-0d1e-4f2a-bcd3-4e5f6a7b8c9d";
        :>> title = "S11 stateDefSpike: pilot 0.59.0 does not support state def body syntax; transitions stay String[*]";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 11 (stateDefSpike): WorkflowDefinition.transitions is currently `String[*]` (values like 'brief->brief-review'). The stateDefSpike task asked: can native SysML v2 `state def` with `entry state` / `then` transition body syntax replace these strings? The spike was run against pilot kernel 0.59.0.";
        :>> decision = "VERDICT NOT GREEN: keep WorkflowDefinition.transitions : String[*] unchanged; do not adopt native state def body syntax now. Revisit when a newer toolchain ships (tracked by toolchainWatch).";
        :>> rationale = "In pilot 0.59.0 bare `state def X;` parses, but the body syntax (`entry state`, `then` keyword) produces 'no viable alternative at input entry' errors — the transitions-in-body grammar is not implemented. Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "WorkflowDefinition.transitions stays as String[*]. No schema change. State def body syntax added to sysmlv2-syntax-notes.md as a confirmed gap in pilot 0.59.0.";
    }

    verification d0031Accept : Test { :>> id = "23ad8fe5-8e5e-4593-bb71-8e64b6704620"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0031 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0031AcceptR1 : TestResult { :>> id = "609e1615-5866-4d66-94eb-4f612dcdefea"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
