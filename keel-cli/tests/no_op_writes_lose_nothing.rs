//! The control for the regenerate-from-model class (issue310), generalised from issue293.
//!
//! THE CLASS: any command that rebuilds a human-editable file from the engine's model of it will
//! silently drop whatever that model cannot represent — comments, a charter reference, an entire
//! category of item. `keel activate` did exactly this, and NOTHING DETECTED IT: the keystone lock
//! caught the *file* changing, and only because that file happens to be locked.
//!
//! THE INVARIANT, chosen because it needs no inventory of "human-editable facts" and cannot be
//! defeated by refactoring: **a command told to make a change that is already true must leave the
//! file byte-identical.** Regeneration is lossless only when the model can represent every byte
//! present, so a no-op that alters the file is proof of a fact the writer cannot round-trip. This
//! would have failed on issue293 the day it shipped: `activate` an already-active process rewrote
//! the file and lost `charteredBy`.
//!
//! The pattern is not new here — `sync-claude --check` (the `claude-surface-drift` guard) is this
//! same property, and the `.claude` surface is the one that did NOT have the bug. This file
//! generalises it to every surface and pins the preservation claim the `.claude` writer only
//! ASSERTED (issue310: claimed, not proven).
//!
//! RESIDUAL, stated rather than implied: the case table below is enumerated, not derived, so a
//! NEW regenerating writer is covered only when someone adds a line. `no_new_engine_surface_writer_
//! is_unrepresented` narrows that by failing when the source grows a whole-file write to a tracked
//! surface that no case exercises — a mechanical floor, not a complete fence.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root")
}

fn run(root: &Path, args: &[&str]) -> String {
    let out = Command::new(keel_bin()).args(args).current_dir(root).output().expect("keel");
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("mkdir");
    for e in std::fs::read_dir(from).expect("read_dir").flatten() {
        let (src, dst) = (e.path(), to.join(e.file_name()));
        if src.is_dir() {
            copy_dir(&src, &dst);
        } else {
            let _ = std::fs::copy(&src, &dst);
        }
    }
}

/// A sandbox carrying this repository's real `.engine/`, so names resolve as they do in the field.
fn sandbox(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-noop-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    copy_dir(&repo().join(".engine"), &root.join(".engine"));
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    root
}

fn activation_of(root: &Path) -> String {
    std::fs::read_to_string(root.join(".engine/contracts/activation.toml")).expect("activation.toml")
}

// ── THE INVARIANT, case by case ───────────────────────────────────────────────────────────────

#[test]
fn activating_an_already_active_process_changes_nothing() {
    let root = sandbox("act-noop");
    let before = activation_of(&root);
    run(&root, &["activate", "knowledge-graph-memory"]); // already active
    assert_eq!(
        activation_of(&root),
        before,
        "a no-op activate must not touch the file — this is the issue293 detector: the old writer \
         regenerated from a template here and lost charteredBy plus every [always] process"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn deactivating_an_already_inactive_process_changes_nothing() {
    let root = sandbox("deact-noop");
    run(&root, &["deactivate", "render"]); // the real change
    let before = activation_of(&root);
    run(&root, &["deactivate", "render"]); // the no-op
    assert_eq!(
        activation_of(&root),
        before,
        "repeating a deactivate must be inert; a writer that rewrites on every invocation degrades \
         the file a little each time and no single run looks wrong"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_no_op_viewpoint_switch_changes_nothing() {
    let root = sandbox("vp-noop");
    let before = activation_of(&root);
    run(&root, &["activate", "orientVP"]); // already active
    assert_eq!(
        activation_of(&root),
        before,
        "the viewpoint branch writes the PROCESS section too (D0164), so it can lose process facts \
         through the other door — the same defect, a different command"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE .claude SURFACE: the claim issue310 says was asserted but never proven ────────────────

#[test]
fn a_hand_added_user_entry_survives_a_claude_surface_regeneration() {
    let root = sandbox("claude");
    run(&root, &["sync-claude", "."]);
    let settings_path = root.join(".claude").join("settings.json");
    let mut settings: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("settings")).expect("json");
    // A user's own key, of a kind keel's model has no representation for.
    settings["userOwnedProbeKey"] = serde_json::json!({"mine": true});
    std::fs::write(&settings_path, serde_json::to_string_pretty(&settings).expect("ser")).expect("write");

    run(&root, &["sync-claude", "."]); // regenerate over the hand edit

    let after: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("settings")).expect("json");
    assert_eq!(
        after["userOwnedProbeKey"],
        serde_json::json!({"mine": true}),
        "sync-claude documents an OWNERSHIP model — foreign entries survive untouched. That was \
         asserted in a doc comment and never held by a test (issue310); it is held here."
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_second_claude_sync_changes_nothing() {
    let root = sandbox("claude-idem");
    run(&root, &["sync-claude", "."]);
    let before = std::fs::read_to_string(root.join(".claude").join("settings.json")).expect("settings");
    run(&root, &["sync-claude", "."]);
    let after = std::fs::read_to_string(root.join(".claude").join("settings.json")).expect("settings");
    assert_eq!(after, before, "a second sync over an already-synced tree must be inert");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_hand_added_comment_survives_the_migrate_pin_restamp() {
    let root = sandbox("pin");
    let pin = root.join(".engine/contracts/engine-version.toml");
    let original = std::fs::read_to_string(&pin).expect("pin");
    std::fs::write(&pin, format!("{original}# pinned here deliberately: see d0251\n")).expect("write");
    run(&root, &["migrate", "."]);
    let after = std::fs::read_to_string(&pin).expect("pin");
    assert!(
        after.contains("pinned here deliberately"),
        "migrate re-stamps the pin and must rewrite ONLY the engine line — this instance was found \
         by the coverage floor below, not by a second field incident: {after}"
    );
    assert!(after.contains("engine = "), "and the pin itself must still be there: {after}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE COVERAGE FLOOR: a new surface writer cannot arrive unnoticed ──────────────────────────

/// The surfaces this file exercises. Adding a whole-file writer over a TRACKED surface without
/// adding a case here fails the test below — the mechanical floor under an enumerated table.
const EXERCISED_SURFACES: &[&str] = &["activation.toml", "settings.json", "engine-version.toml"];

/// Written ONLY by `init`, into a project that does not exist yet — there is nothing of a human's
/// to preserve, so no-op invariance does not apply. Listed with the reason, per the rule above:
/// an exemption whose justification is unwritten is indistinguishable from an oversight.
const SCAFFOLD_ONLY: &[&str] = &["adoption-profile.toml", "keel-wrapper.toml"];

#[test]
fn no_new_engine_surface_writer_is_unrepresented() {
    // Whole-file writes to a committed surface are the class. Machine-local state (`.keel/`) and
    // scaffolding of files that did not exist are NOT: there is nothing of a human's to preserve.
    let src = std::fs::read_to_string(repo().join("keel-cli/src/main.rs")).expect("main.rs")
        + &std::fs::read_to_string(repo().join("keel-cli/src/claude_surface.rs")).expect("claude_surface.rs");
    let mut unrepresented = Vec::new();
    for line in src.lines() {
        let t = line.trim();
        if !t.contains("write(") || t.starts_with("//") {
            continue;
        }
        for surface in [".toml\"", ".json\""] {
            if let Some(i) = t.find(surface) {
                let name = t[..i].rsplit(['"', '/']).next().unwrap_or_default().to_string() + &surface[..5];
                if !EXERCISED_SURFACES.iter().any(|s| name.contains(s.split('.').next().unwrap_or(s)))
                    && !SCAFFOLD_ONLY.iter().any(|s| name.contains(s.split('.').next().unwrap_or(s)))
                    && !t.contains(".keel")
                    && !t.contains("installed-units")
                {
                    unrepresented.push(name);
                }
            }
        }
    }
    unrepresented.sort();
    unrepresented.dedup();
    assert!(
        unrepresented.is_empty(),
        "a whole-file write to a tracked surface has no no-op invariance case: {unrepresented:?} — \
         add one, or exempt it here WITH THE REASON. An enumerated table silently outgrown is the \
         failure mode this floor exists to make loud."
    );
}
