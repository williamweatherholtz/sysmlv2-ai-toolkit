// =====================================================================
// ProjectBaselines — Release instances marking project milestones (D0042).
//
// Each Release records a git commit SHA + purpose. The computed baseline view
// (report.py "Baselines" tab) derives "items done at this release" from
// git ancestry — never authored explicitly (text-is-truth principle).
//
// Commit convention: Release.commit = the short SHA of the commit that
// closed (or immediately followed) the milestone.
// =====================================================================
package ProjectBaselines {
    private import EngineElement::*;
    private import EngineBaseline::*;

    // v0.2.0 — the first real run of the Deploy workflow (D0135). Contents stay COMPUTED from git
    // ancestry; this records only the irreducible facts (version, commit, purpose).
    part release020 : Release {
        :>> id = "0d750001-0814-5501-9b3e-000000250001";
        :>> title = "v0.2.0 — distributed collaboration, in-loop gates, 23 guards";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> commit = "425145d";
        :>> purpose = "First release since v0.1.0 (six weeks stale) and the first real run of the DEFINED-but-never-run Deploy workflow. Adds: the D0129 distributed-collaboration model (ancestry-preserving integration, per-machine actor identity that REFUSES rather than defaulting, CSPRNG ids, a portable gate); in-loop governance (the response contract in the system prompt, plus turn-boundary and per-edit gates in the binary, D0130/D0134); and the guard set 17 -> 23 (18 hard, 5 warning-only) including duplicate-identity, marker-vocabulary, attestation-substance, verification-trace, priority-inversion and retro-backlog. BREAKING for callers: write paths now refuse when the acting actor is unstated (D0129) — bind with `keel actor set`, KEEL_ACTOR, or an explicit flag. NOT included: process import/export/activation (nProcessPortability unbuilt — `keel init` remains all-or-nothing).";
    }

    // Schema green (34 items done): all initial backlog items complete.
    part schemaCompleteBaseline : Release {
        :>> id = "7b4c5d6e-8f9a-4b0c-1d2e-3f4a5b6c7d8e";
        :>> title = "Schema Complete — all 34 EngineBuild items done";
        :>> createdAt = "2026-06-11";
        :>> createdBy = "wweatherholtz";
        :>> commit = "61850f4";
        :>> purpose = "Initial engine schema, workflows, validators, and query layer all green. Starting NextWork queue.";
    }

    // Rust parser complete (Sprint 9 write-API done): full CLI parser + write API.
    part rustParserBaseline : Release {
        :>> id = "8c5d6e7f-9a0b-4c1d-2e3f-4a5b6c7d8e9f";
        :>> title = "Rust Parser + Write API — Sprints 0-9 complete";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
        :>> commit = "58ba2c3";
        :>> purpose = "sysmlv2 CLI: parse, validate, orient, whats-next, append-result, add-task. Write API enforces UUID + append-only semantics.";
    }

    // Sprint 14 baseline: toolchainWatch + baselineView delivered.
    part sprint14Baseline : Release {
        :>> id = "9d6e7f8a-0b1c-4d2e-3f4a-5b6c7d8e9f0a";
        :>> title = "Sprint 14 — toolchainWatch deferred; baselineView demo live";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> commit = "aba9e6f";
        :>> purpose = "toolchainWatch: all three feature areas deferred (D0041). baselineView: Release schema + baselines.sysml + report.py Baselines tab (D0042).";
    }
}
