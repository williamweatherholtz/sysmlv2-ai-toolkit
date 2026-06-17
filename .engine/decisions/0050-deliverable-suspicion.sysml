// =====================================================================
// D0050 — deliverable-suspicion: tie the Rust source tree into the suspicion
// model so code drift marks the deliverable's verification TestResults suspect.
// Closes issue006 (the hole that let the toolchain rot silently). W4 of D0046.
// Accepted 2026-06-16 by wweatherholtz (W4 accepted in D0046; "yes proceed").
// =====================================================================
package Decision0050 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0050 : Decision {
        :>> id = "12d3e4f5-0617-4238-376f-1324a5b6c7e0";
        :>> title = "D0050: deliverable-suspicion — Rust source drift marks deliverable TestResults suspect";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Until now the suspicion model watched material change to .sysml authored facts over satisfy/:> edges, but nothing invalidated a TestResult when the Rust SOURCE it verifies changed (issue006). The rustS8/S9 'parity verified' results stayed green while the binary regressed (issue005) — a real traceability hole between the engine model and the deliverable code. With Rust promoted to authority (D0048), guarding the deliverable's verification is now obligatory.";
        :>> decision = "Added a deliverable-suspicion mechanism: a manifest (.engine/deliverable-manifest.txt) declares the deliverable source paths (sysmlv2-parser, sysmlv2-cli, Cargo.toml, Cargo.lock) and the verification task names whose evidence depends on that source; `sysmlv2 orient` (orient.rs, the authority) reads it and marks a listed task SUSPECT when git shows a commit touching a source path AFTER the task's latest passing TestResult's judgedAgainst commit. Re-verifying (appending a TestResult judged at HEAD) clears it.";
        :>> rationale = "Until now the suspicion model watched material change to .sysml authored facts over satisfy/:> edges, but nothing invalidated a TestResult when the Rust SOURCE it verifies changed (issue006) — the rustS8/S9 'parity verified' results stayed green while the binary regressed (issue005), a real traceability hole between the engine model and the deliverable code. With Rust promoted to authority (D0048), guarding the deliverable's verification is now obligatory. Demonstrated live: the Sprint 17 parser/orient changes post-date the rustS1..S9 judgedAgainst commits, so those tasks now surface as suspect.";
        :>> consequences = "orient.suspect now reflects deliverable drift, not just .sysml drift — code changes can no longer leave 'verified' green silently (issue006 closed). The manifest is the explicit, auditable list of deliverable verification tasks + paths (kept in sync as the deliverable grows). The mechanism is git-based and kernel-free (consistent with D0048). query.py (secondary) is not updated for this — the Rust authority carries it. Re-verification discipline: when the deliverable source changes, re-run cargo test + parity_check and append fresh TestResults to clear suspicion.";
    }

    verification d0050Accept : Test { :>> id = "b9d3a60b-981d-4d9c-a8f9-fc09891b109f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0050 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0050AcceptR1 : TestResult { :>> id = "05e4b8ee-874e-486b-87e2-eb1ed65ee073"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
