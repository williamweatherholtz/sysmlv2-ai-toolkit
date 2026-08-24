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

    // issue244 TRIAGE: release020 records commit 425145d, but tag `v0.2.0` points at e2c812f - the
    // release-recorded guard has WARNED about this disagreement, shipped and unread, for days. The
    // facts, established from git: 425145d IS an ancestor of the tag, two commits back, so the record
    // was TRUE WHEN WRITTEN and the tag was cut later. The tag is what shipped and is published, so it
    // must not move (D0129: never rewrite history). release020 is owned by another actor, so a
    // non-owner corrects it by SUPERSEDING rather than editing its fields (D0108/D0129). This records
    // the SHIPPED commit; release020 stays readable as what was intended at authoring time.
    part release020Shipped : Release {
        :>> id = "bc938157-77ef-40c3-9aa8-37561b0e31a6";
        :>> title = "v0.2.0 (as shipped) — the commit the published tag actually points at";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> commit = "e2c812f";
        :>> purpose = "Supersedes release020's commit field ONLY. Same release, same contents; the recorded commit was authored two commits before the tag was cut, so it named the intended commit rather than the shipped one. Recorded because the release-recorded guard correctly flagged the disagreement and a warning that is merely true is not resolved - it is unread. NOTE for a reader who follows the tag: e2c812f carries a garbage commit MESSAGE (a captured pre-commit hook log was used as the message), which is a separate cosmetic defect in history that must not be rewritten.";
    }
    #Supersede dependency from release020Shipped to release020;

    // v0.2.1 — patch: unblock downstream projects blocked by the D0133 regression (issue089/D0136).
    part release021 : Release {
        :>> id = "0d790001-0814-5901-9b3e-000000290001";
        :>> title = "v0.2.1 — engine markers builtin to the binary (downstream unblock)";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> commit = "c4464de";
        :>> purpose = "Fixes a v0.2.0 regression reported from the field: marker-vocabulary was a HARD guard in the BINARY whose passing condition was `metadata def` lines in the PROJECT's schema, and upgrading the binary never updates an existing project's on-disk .engine/schema/. Reproduced at 566 violations with every commit blocked, including on the engine's own shipped files, with the only apparent remedy being an edit to FROZEN schema/core. The engine's 17-marker algebra is now BUILTIN to the binary and always valid undeclared; project markers may be declared in any .engine or .tracking file. Typo protection unchanged — an undeclared or misspelled marker still hard-fails. Upgrading requires NO action in a downstream project.";
    }

    // v0.2.2 — patch: second downstream unblock (issue090/D0136).
    part release022 : Release {
        :>> id = "0d7b0001-0814-5b01-9b3e-0000002b0001";
        :>> title = "v0.2.2 — an absent declared rule is not a violation";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> commit = "b98f526";
        :>> purpose = "Six HARD guards are rule-sourced and reported a MISSING rule as a VIOLATION, so any project whose .engine/rules/ is absent, older, or authored fresh hit six hard failures and could not commit — with messages naming internal rule names that suggest no action. A project that never ADOPTED a control has not VIOLATED it: an absent rule now yields a visible NOT-ADOPTED warning and a pass, while a malformed rule and a declared-but-violated rule both still hard-fail. Also confirmed this release: a fresh `keel init` project passes its own full gate. Upgrading requires NO action in a downstream project.";
    }

    // v0.2.3 — the first release VERIFIED by obtaining the published asset and asking it what it is.
    part release023 : Release {
        :>> id = "0d7f0001-0814-5f01-9b3e-0000002f0001";
        :>> title = "v0.2.3 — keel version: a released artifact that can identify itself";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> commit = "b006f10";
        :>> purpose = "Adds `keel version` (also --version / -V / --json): the release version, the commit the binary was built from, and the guard inventory it carries. The three prior releases shipped with NO version identification — `--version` printed usage — so a downstream project told to upgrade could not confirm which binary it was running, and neither could the maintainer; a project still on the blocked version was indistinguishable from one that had upgraded. Since a guard's behaviour is a property of the BINARY (the mechanism behind issue089 and issue090), that is the first diagnostic step for any blocked-downstream report, and it was unavailable to both sides. The BUILD COMMIT is the load-bearing part: the version string alone cannot separate the published artifact from a stale local build claiming the same number; it degrades honestly (`unknown` off-git, `+dirty` from a modified tree) and is checkable against these Release facts. The guard inventory is computed from GUARD_NAMES, so it cannot drift from the guards that run. ALSO verified this release, against the DOWNLOADED v0.2.2 asset rather than a local build: a fresh `keel init` project passes its own full gate, and both downstream blocker states pass (.engine/rules/ absent; marker declarations absent from project schema). This is the FIRST release whose published asset was obtained and asked to identify itself — the prior three were verified only by confirming files existed at a URL, which is the process gap issue091 records. Upgrading requires NO action in a downstream project.";
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
    // v0.1.0 backfill (D0191): the tag predates the Release-recording convention; guard 43's
    // first run caught the gap. The commit is the tag's own, read from git - recorded, not invented.
    part release010 : Release {
        :>> id = "4ec95968-eaff-48cd-92b8-610a7c427670";
        :>> title = "v0.1.0 - first tagged release";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> commit = "a0f5afc";
        :>> purpose = "The first tagged release, tagged 2026-06 at the v0.1.0 release-prep commit. Recorded retroactively on guard 43 (release-recorded) catching that the tag had no authored Release item.";
    }

}
