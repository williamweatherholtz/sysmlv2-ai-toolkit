// =====================================================================
// D0062 — closeout-early-sprints: close out rust_sprint1..rust_sprint6
// (CloseOut + Retro gates) by a RECORDED WAIVER. These six sprints predate the
// close-out/retro rigor; their gates are method=confirmation and were never
// attested. Per the prospective-governance doctrine (an item is governed by the
// process in force when it was created), they are closed under their OWN process
// — the human confirmation their gates call for — authorized here as a bounded,
// attributable waiver rather than uncaptured hand-waving.
// NOTE: authored in the CURRENT schema (authoredBy/context/decisionText/consequences)
// — migrated by attrModelInstances (D0061 task 2).
// Accepted 2026-06-17 by wweatherholtz ("close out the initial 6 sprints ...
// by my superseding directive" + "close out retros by the same directive";
// reframed as a waiver after research+critique this sitting).
// =====================================================================
package Decision0062 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0062 : Decision {
        :>> id = "62839405-1b6c-4d7e-a829-516273849551";
        :>> title = "D0062: waiver — close rust_sprint1..6 (CloseOut + Retro) under their as-authored confirmation gates (predate close-out rigor)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "rust_sprint1..rust_sprint6 (the earliest delivery runs, 2026-06-15) carry CloseOut and Retro gates authored as method=confirmation ('user accepts Sprint N complete') — the pre-D0049 cohort, the same model under which rust_sprint7 closed (confirmation, judgedBy=wweatherholtz). They were never attested, so sysmlv2 orient lists all six as in-progress, pending CloseOut, indefinitely. The work itself shipped and its DoDs are green (lexer/parser/semantic/spec-compat/CLI/orient), and the Rust toolchain it produced is the live authority; only the ceremony attestation is missing. Research this sitting established the governing doctrine: an item is governed by the process in force when it was created (prospective application — NPR 7123.1 baselines, EIA-649, Landgraf prospectivity, PROV/bitemporal), and discretion over historical data is legitimate ONLY when captured as a bounded, attributable instrument (a CM waiver/deviation), never as uncaptured hand-waving.";
        :>> decision = "DECISION (WAIVER): authorize closing these six pre-rigor sprints under their OWN as-authored process. Their gates are method=confirmation, whose evidence IS the human's word (CLAUDE.md §3c); the human (wweatherholtz — the actor the gates name) attests CloseOut and Retro complete for rust_sprint1..rust_sprint6, recorded as judgedBy=wweatherholtz. This is a waiver, not a supersede: it mutates no process and no baseline; it authorizes a bounded, per-item departure (closing on confirmation now rather than reconstructing pre-rigor ceremony) for these six named sprints only.";
        :>> rationale = "ALTERNATIVES: (A) reconstruct each sprint's closeOut/retro evidence and self-record — rejected: the rigor did not exist then, so manufacturing ceremony is a worse, less honest record than an attested waiver. (B) re-author the gates as method=inspect/analysis (the post-D0049 autonomous model) and AI-record them — rejected: that rewrites history by retroactively applying a newer process, the exact anti-pattern the prospective doctrine forbids. (C CHOSEN) close under the as-authored confirmation gates via a recorded waiver. CRITERIA: honesty-of-record x honors-process-in-force-at-creation x captured-discretion.";
        :>> consequences = "Twelve confirmation TestResults appended (CloseOutGateR1 + RetroGateR1 for each of rust_sprint1..6) via the new `sysmlv2 append-gate-result` write API (built this sitting; closes the D0054 friction gap that gate-result recording had no tooling), judgedBy=wweatherholtz, judgedAt=2026-06-17, judgedAgainst=0cbc4d0 (HEAD at the directive). After this, sysmlv2 orient no longer lists the six as in-progress. NO new process and NO precedent for current sprints: close-out/retro rigor going forward is unchanged (D0049, autonomous inspect/analysis). This is a one-time historical reconciliation applying only to runs that predate the rigor. The deeper gap it exposed — the engine has no machine-readable item-to-governing-process link, so 'old item vs new process' is undecidable from the model (issue002) — is tracked as the processGovernanceLineage backlog item (charter-lineage governance: authored charter+process-revision edges, everything else computed).";
    }

    verification d0062Accept : Test { :>> id = "8b9ac6c0-9eed-471f-9535-a4af8f2c3349"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0062 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0062AcceptR1 : TestResult { :>> id = "30811142-02d6-48b8-ae1c-38f93862a725"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
