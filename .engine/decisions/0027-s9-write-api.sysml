package Decision0027 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0027 : Decision {
        :>> id = "cf5a6b7c-d8e9-4f0a-bc1d-2e3f4a5b6c7d";
        :>> title = "S9 writeApi: Rust CLI write API enforcing UUID/append-only; direct editing retired as primary path";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 9 (writeApi): The engine has had 'direct editing of .sysml files as the sanctioned bootstrap write path' since bootstrap began. The runtimeParser (S8) completed the AST-based read layer. The next step is a write layer that enforces the three write-policy invariants: (1) ids — every new record gets an auto-generated UUID; (2) append-only — TestResults can only be appended, never overwritten; (3) writePolicy — append_result requires task existence, add_task rejects duplicates.";
        :>> decision = "Implement a write.rs module in sysmlv2-cli with append_result (parse file, verify task exists, compute next N = max(existing)+1, generate UUID via SystemTime+counter mix, find insertion point, write) and add_task (parse file, verify def exists, reject duplicate, gen UUID, insert action+DoD before def close brace), exposed as sysmlv2 append-result and sysmlv2 add-task subcommands with --file/--task/--sha/--def/--dod/--method flags; update CLAUDE.md §4 to retire direct editing as the primary path; add 7 write BDD scenarios.";
        :>> rationale = "With the S8 AST-based read layer complete, the next step is a write layer enforcing the three write-policy invariants (auto-generated UUIDs, append-only TestResults, task-existence/duplicate checks). Direct editing remains possible for schema/decisions/cursor but is no longer primary. gen_uuid uses SystemTime + AtomicU64 counter — not cryptographically random but sufficient for dev tooling. Accepted under standing sprint instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "write.rs 340 lines; 2 new subcommands; CLAUDE.md §4 updated; direct editing still possible for schema/decisions/cursor but no longer primary. gen_uuid uses SystemTime + AtomicU64 counter — not cryptographically random but sufficient for dev tooling.";
    }

    verification d0027Accept : Test { :>> id = "82f58b8b-544c-4dbf-bc62-0f2df0e90692"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0027 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0027AcceptR1 : TestResult { :>> id = "57fbf510-ed03-4290-951f-3297d2233fd2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
