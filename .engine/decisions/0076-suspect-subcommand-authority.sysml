// =====================================================================
// D0076 — `sysmlv2 suspect` exposes orient's AUTHORITATIVE suspect (criterion-change + D0050
// deliverable-source drift), a deliberate SUPERSET of query.py's criterion-change-only suspect.
// So suspect is the ONE algorithmic-trio view (M2.2c) that does NOT byte-match query.py — by
// design: Rust is the authority (D0048), D0050 drift is real + Rust-only, query.py suspect is the
// incomplete one (retired at M4). Accepted 2026-06-19 by wweatherholtz (M2.2c suspect-semantics
// question: "suspect = orient's answer (13)").
// =====================================================================
package Decision0076 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // D0076 defines a CLI surface's meaning (a tooling decision); it edits no
    // .engine/processes|workflows file, so no process-change marker is required (D0070).
    part d0076 : Decision {
        :>> id = "63940516-2c7d-4e8f-a076-516273849662";
        :>> title = "D0076: `sysmlv2 suspect` = orient's authoritative suspect (D0050 superset), NOT byte-parity with query.py suspect";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Porting the M2.2 algorithmic-trio views to Rust (D0074/D0075) under a byte-parity-vs-query.py bar. governing-version + reprocess-candidates port cleanly with parity. But `suspect` does NOT: query.py suspect (the kernel-path classify()) only ever implemented criterion-TEXT-change suspicion (D0005 rule 3) and returns []; Rust `orient` already computes a SUPERSET — criterion-change PLUS D0050 deliverable-source drift (re-verify a Rust task when its source moved) — and reports 13. So a `sysmlv2 suspect` subcommand cannot simultaneously byte-match query.py AND stay consistent with Rust's own authoritative orient.";
        :>> decision = "`sysmlv2 suspect` exposes orient's authoritative suspect set (criterion-change + D0050 deliverable-source drift), i.e. `{\"suspect\": orient.suspect}`. It is therefore a deliberate SUPERSET of query.py suspect and is the ONE M2.2c trio view exempt from the byte-parity bar. The parity bar for the trio is amended: governing-version + reprocess-candidates are byte-identical to query.py; suspect is a documented superset (orient is the single source of truth for suspect, never a second divergent implementation).";
        :>> rationale = "D0048 makes the Rust toolchain the authority; D0050 deliverable-source drift is a real, Rust-only suspicion that query.py never gained. Replicating query.py's criterion-change-only output would (a) port a known gap and (b) create two conflicting 'suspect' answers inside Rust (orient=superset, suspect=subset) — a worse outcome than a documented parity exception. ALTERNATIVES: (A) suspect replicates query.py ([], byte-parity) — rejected: ports a gap, self-contradicts orient; (B) no standalone suspect subcommand (orient owns it) — viable but loses CLI symmetry with query.py before M4 retirement; (C CHOSEN) suspect = orient.suspect, parity-exception. CRITERIA: single-source-of-truth x authority-correctness (D0048/D0050) x no-conflicting-answers > strict-byte-parity-with-a-soon-retired-tool.";
        :>> consequences = "M2.2c's DoD parity bar reads: governing-version + reprocess byte-identical to query.py; suspect = orient (superset, this decision). The migration parity_check + any future view-parity harness must EXEMPT suspect from byte-equality (assert superset/consistency-with-orient instead). When query.py is retired (M4, issue012), this exception becomes moot (only orient's definition remains). Pairs with D0048 (Rust authority), D0050 (deliverable suspicion), D0074/D0075 (the port).";
    }

    verification d0076Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a076-516273849663";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0076 (`sysmlv2 suspect` = orient's authoritative suspect, a D0050 superset of query.py, parity-exception) by selecting 'suspect = orient's answer (13)' in the M2.2c suspect-semantics question on 2026-06-19.";
    }
    part d0076AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a076-516273849664";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "967f018";
        :>> judgedAt = "2026-06-19";
        :>> judgedBy = "wweatherholtz";
    }
}
