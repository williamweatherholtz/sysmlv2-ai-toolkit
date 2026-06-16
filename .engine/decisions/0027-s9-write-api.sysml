package Decision0027 {
    private import EngineWork::*;

    part d0027 : Decision {
        :>> id = "cf5a6b7c-d8e9-4f0a-bc1d-2e3f4a5b6c7d";
        :>> title = "S9 writeApi: Rust CLI write API enforcing UUID/append-only; direct editing retired as primary path";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 9 (writeApi): The engine has had 'direct editing of .sysml files as the sanctioned bootstrap write path' since bootstrap began. The runtimeParser (S8) completed the AST-based read layer. The next step is a write layer that enforces the three write-policy invariants: (1) ids — every new record gets an auto-generated UUID; (2) append-only — TestResults can only be appended, never overwritten; (3) writePolicy — append_result requires task existence, add_task rejects duplicates.";
        :>> decisionText = "Implement write.rs module in sysmlv2-cli with append_result and add_task functions. append_result: parse file, verify task exists, determine next N = max(existing)+1, generate UUID via SystemTime+counter mix, find insertion point via text search (last DoDR line or DoD verification line), write updated content. add_task: parse file, verify def exists, reject duplicate, gen UUID, find def close brace, insert action+DoD before it. Expose as sysmlv2 append-result and sysmlv2 add-task subcommands with --file/--task/--sha/--def/--dod/--method flags. Update CLAUDE.md §4 to retire direct editing as the primary path. 7 write BDD scenarios. Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "write.rs 340 lines; 2 new subcommands; CLAUDE.md §4 updated; direct editing still possible for schema/decisions/cursor but no longer primary. gen_uuid uses SystemTime + AtomicU64 counter — not cryptographically random but sufficient for dev tooling.";
    }
}
