//! The CLI surface as the binary KNOWS it (D0252 clause A, D0271).
//!
//! # Why a list exists at all
//!
//! D0252 clause A says a unit declares the CAPABILITIES it invokes — guards, schema types, and
//! COMMANDS — and that an install whose target engine lacks one REFUSES naming the missing
//! capability rather than a version number. The guard half shipped with D0183/K8, diffing a unit's
//! declared guards against `GUARD_NAMES`. The command half needs the same thing: an inventory the
//! handshake can diff against. This is it.
//!
//! # Why this is a hand-held const and not derived at runtime
//!
//! Rust cannot introspect a `match` at runtime, so the dispatch table is not readable from inside the
//! program. A const is therefore the only option, and a const that drifts from the dispatch is worse
//! than none — it would make the handshake refuse a command the binary HAS, or accept one it lacks.
//! `cli_surface_matches_the_dispatch` holds the two sides equal by parsing `main.rs` at test time,
//! which is the same shape as D0271's two-way ICD guard and the reason it can be trusted.
//!
//! D0273 will collapse the lens family into one router, and this list changes WITH it — that is the
//! point. A unit declaring a name the post-collapse engine no longer has is exactly what must refuse.

/// Every command this binary dispatches, sorted. Kept equal to `main.rs`'s dispatch by test.
pub const COMMAND_NAMES: [&str; 99] = [
    "accept", "activate", "activation", "actor", "actor-trace", "add-task",
    "adoption-check", "advance", "append-gate-result", "append-result", "apply-review", "arch",
    "assumptions", "assured", "attestation", "attestation-coverage", "audit", "audit-adherence",
    "audit-history", "authority-queue", "boundary", "boundary-sweep", "business", "check",
    "check-engine", "claim", "concern-coverage", "contentions", "controls", "coverage",
    "critique-coverage", "critique-policy", "deactivate", "decision-card", "decision-follow-through", "decisions",
    "deck", "diagram", "dispositions", "enforcement-report", "enroll", "gate",
    "github-decider", "github-decision-id", "github-gesture", "github-ingest", "github-pull", "governing-version",
    "guard", "hardening", "hook", "indicators", "init", "intake",
    "item", "knowledge", "land", "launchables", "library", "ls",
    "marker-census", "migrate", "mint", "new", "onboard", "open-issues",
    "orient", "orphans", "outstanding", "override", "process", "projects",
    "recall", "recent", "record", "record-measurement", "render", "report",
    "reprocess-candidates", "reverify", "rootedness", "rules", "serve", "sitting-coverage",
    "snapshot-indicators", "status", "suspect", "sync", "sync-claude", "tier-satisfaction",
    "trace", "trace-need", "validate", "verification", "version", "view",
    "whats-next", "why", "workflows",
];

/// Does this binary dispatch `name`?
#[must_use]
pub fn has_command(name: &str) -> bool {
    COMMAND_NAMES.contains(&name)
}
