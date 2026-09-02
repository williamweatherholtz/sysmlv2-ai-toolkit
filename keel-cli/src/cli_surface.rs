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
pub const COMMAND_NAMES: [&str; 65] = [
    "accept", "activate", "activation", "actor", "actor-trace", "add-task",
    "adoption-check", "advance", "append-gate-result", "append-result", "apply-review", "arch",
    "assured", "attestation", "audit", "audit-adherence", "audit-history", "check",
    "check-engine", "claim", "deactivate", "decision-card", "deck", "diagram",
    "enforcement-report", "enroll", "gate", "github-decider", "github-decision-id", "github-gesture",
    "github-ingest", "github-pull", "governing-version", "guard", "hook", "init",
    "item", "land", "library", "migrate", "mint", "new",
    "onboard", "orient", "override", "process", "projects", "recall",
    "record", "record-measurement", "render", "report", "reprocess-candidates", "reverify",
    "rules", "serve", "show", "snapshot-indicators", "status", "sync",
    "sync-claude", "validate", "version", "view", "whats-next",
];

/// Does this binary dispatch `name`?
#[must_use]
pub fn has_command(name: &str) -> bool {
    COMMAND_NAMES.contains(&name)
}

/// Every lens reachable as `keel show <lens>` (D0273), sorted. Kept equal to `cmd_show` by test.
///
/// A SECOND list existed before this: `VIEW_SUBCOMMANDS` in guards.rs, hand-maintained, which the
/// viewpoint-renderer guard diffed renderer strings against. Two inventories of the same fact is one
/// too many — the guard's copy had already grown apologetic comments about names "never added here,
/// so the first viewpoint naming it failed". Both now read this module.
pub const LENS_NAMES: [&str; 35] = [
    "assumptions", "attestation-coverage", "authority-queue", "boundary", "boundary-sweep", "business",
    "concern-coverage", "contentions", "controls", "coverage", "critique-coverage", "critique-policy",
    "decision-follow-through", "decisions", "dispositions", "hardening", "indicators", "intake",
    "knowledge", "launchables", "ls", "marker-census", "open-issues", "orphans",
    "outstanding", "recent", "rootedness", "sitting-coverage", "suspect", "tier-satisfaction",
    "trace", "trace-need", "verification", "why", "workflows",
];

/// Is `name` a lens reachable through `keel show`?
#[must_use]
pub fn has_lens(name: &str) -> bool {
    LENS_NAMES.contains(&name)
}

/// The keel command a renderer string names, as `(verb, next token)`.
///
/// `keel show orphans` yields `("show", Some("orphans"))`; `keel audit` yields `("audit", None)`;
/// a string naming no keel invocation at all yields `None`. Callers that care about the ROUTER —
/// the renderer guard and the console's view binder — read the second element, because after D0273
/// the verb is shared by 35 lenses and identifies none of them.
#[must_use]
pub fn renderer_command(r: &str) -> Option<(&str, Option<&str>)> {
    let rest = r.strip_prefix("keel ")?;
    let mut it = rest.split(char::is_whitespace).flat_map(|t| t.split('(')).filter(|t| !t.is_empty());
    let verb = it.next()?;
    Some((verb, it.next()))
}
