package Decision0026 {
    private import EngineWork::*;

    part d0026 : Decision {
        :>> id = "be4f5a6b-c7d8-4e9f-a0b1-c2d3e4f5a6b7";
        :>> title = "S8 review doc-sync: #Marker succession syntax confirmed; validate script reference fixed";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 8 (runtimeParser) confirmed two syntax facts and exposed stale doc claims: (1) '#Marker first X then Y;' succession prefix works in both action def bodies and at package level — undocumented in sysmlv2-syntax-notes.md; (2) sysmlv2-syntax-notes.md 'How to validate' section still referenced retired validate_sysml.py (retired 2026-06-11) instead of the four layer validators.";
        :>> decision = "Update sysmlv2-syntax-notes.md: add a '#Marker first X then Y;' bullet to the Works section (Sprint 8 confirmed, refs D0025); replace the 'How to validate' section with the four-layer validator commands, remove the validate_sysml.py reference, and add the NEVER pipe warning. Record as D0026 per the doc-sync process.";
        :>> rationale = "Sprint 8 confirmed the #Marker succession prefix works in both action def bodies and at package level but it was undocumented, and the 'How to validate' section still pointed at validate_sysml.py (retired 2026-06-11) instead of the four layer validators. No schema or process semantics change. Accepted under standing sprint review instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "Syntax-notes correctly documents the #OrderingOnly succession prefix. Validate section no longer points to retired tooling. No schema or process semantics change.";
    }
}
