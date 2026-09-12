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
pub const COMMAND_NAMES: [&str; 38] = [
    "accept", "activate", "activation", "actor", "advance", "audit",
    "claim", "claude", "currency", "deactivate", "deck", "enroll",
    "gate", "github-decider", "github-decision-id", "github-gesture", "github-ingest", "github-pull",
    "hook", "init", "judge-set", "land", "library", "migrate",
    "onboard", "override", "process", "projects", "recall", "record",
    "reject", "render", "serve", "show", "suite", "sync",
    "sync-claude", "version",
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
pub const LENS_NAMES: [&str; 51] = [
    "actor-trace", "arch", "assumptions", "attestation", "attestation-coverage", "authority-queue",
    "boundary", "boundary-sweep", "business", "commit-delta", "concern-coverage", "contentions",
    "control-census", "control-structure", "controls", "coverage", "critique-coverage", "critique-policy",
    "decision-follow-through", "decisions", "dispositions", "enforcement-report", "flow", "governing-version",
    "hardening", "indicators", "intake", "item", "knowledge", "launchables",
    "ls", "marker-census", "open-issues", "orient", "orphans", "outstanding",
    "priority", "recent", "reprocess-candidates", "rootedness", "sitting-coverage", "status",
    "suspect", "tier-satisfaction", "trace", "trace-need", "verification", "view",
    "whats-next", "why", "workflows",
];

/// Is `name` a lens reachable through `keel show`?
#[must_use]
pub fn has_lens(name: &str) -> bool {
    LENS_NAMES.contains(&name)
}

/// The words `keel render` resolves BEFORE it looks for a declared `.view.toml` (D0449).
///
/// In the order `cmd_render` tries them: the whole-model graph (`model` | `all` | `whole`, once
/// `keel diagram`), `report` (once `keel report`), `decision-card` (once `keel decision-card`), then
/// the computed `control-structure` lens. A declared view carrying one of these names would never be
/// reached — the silent shadowing sprint 512 met in the console's view binder — so the test at the
/// foot of this file refuses it.
pub const RENDER_RESERVED: [&str; 6] = ["all", "control-structure", "decision-card", "model", "report", "whole"];

/// Is `name` a word `keel render` resolves before any declared view?
#[must_use]
pub fn is_render_reserved(name: &str) -> bool {
    RENDER_RESERVED.contains(&name)
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

#[cfg(test)]
mod render_reserved_tests {
    use super::*;

    /// D0449: no declared view may be named a render sub-verb. Read from the SOURCE of the declared
    /// views, so a `.view.toml` added later under a reserved name fails here, not in a user's shell.
    #[test]
    fn no_declared_view_is_named_a_render_reserved_word() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(".engine").join("views");
        let mut declared = Vec::new();
        for e in std::fs::read_dir(&dir).expect(".engine/views is readable").flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if let Some(view) = name.strip_suffix(".view.toml") {
                declared.push(view.to_owned());
            }
        }
        assert!(!declared.is_empty(), "the engine declares views under {}", dir.display());
        let shadowed: Vec<&String> = declared.iter().filter(|v| is_render_reserved(v)).collect();
        assert!(shadowed.is_empty(), "a declared view is named a `keel render` reserved word and could never be rendered: {shadowed:?}");
    }

    /// The list is sorted, so a reader can find a word and a diff shows one line per change.
    #[test]
    fn render_reserved_is_sorted_and_names_no_lens_but_control_structure() {
        let mut sorted = RENDER_RESERVED.to_vec();
        sorted.sort_unstable();
        assert_eq!(sorted, RENDER_RESERVED.to_vec());
        for w in RENDER_RESERVED {
            assert!(w == "control-structure" || !has_lens(w), "a render reserved word doubling as a show lens is two spellings of one answer: {w}");
        }
    }
}
