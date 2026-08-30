//! `dcActivateDoesNotLoseFacts` (issue293, High) — activate/deactivate must not rewrite the file.
//!
//! The defect: `keel deactivate X` followed by `keel activate X` regenerated `activation.toml` from
//! a header template plus two computed lists, and one round trip DELETED `charteredBy = "d0226"`
//! with its explanatory comment (the D0225 charter reference) and DROPPED `decision-authoring` from
//! the active list because it is `[always]` and asserts no guard.
//!
//! The second loss is semantic, not cosmetic: an ABSENT section means everything is active
//! (D0138/D0164), so a PRESENT list naming 10 of 11 processes marks the 11th INACTIVE. Silently
//! omitting a process from a list that exists can deactivate it.
//!
//! Nothing detected either loss — the KEYSTONE LOCK caught the file changing. A write path that can
//! only be audited by the control that guards its output is not a write path anyone can trust.

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

fn run(root: &Path, args: &[&str]) -> String {
    let out = Command::new(keel_bin()).args(args).current_dir(root).output().expect("keel");
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

/// A project carrying THIS repository's real `activation.toml` — the file the defect was found on.
fn project_with_real_activation(tag: &str) -> (PathBuf, String) {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root");
    let real = repo.join(".engine").join("contracts").join("activation.toml");
    let original = std::fs::read_to_string(&real).expect("this repo's activation.toml");
    let root = std::env::temp_dir().join(format!("keel-activation-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    // Copy the whole engine so process/viewpoint names resolve as they do here.
    copy_dir(&repo.join(".engine"), &root.join(".engine"));
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    (root, original)
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

fn activation(root: &Path) -> String {
    std::fs::read_to_string(root.join(".engine").join("contracts").join("activation.toml")).expect("read")
}

// ── THE CORE CLAIM: a round trip is a no-op, byte for byte ────────────────────────────────────

#[test]
fn a_deactivate_activate_round_trip_leaves_the_file_byte_identical() {
    let (root, original) = project_with_real_activation("roundtrip");
    run(&root, &["deactivate", "knowledge-graph-memory"]);
    run(&root, &["activate", "knowledge-graph-memory"]);
    let after = activation(&root);
    assert_eq!(
        after, original,
        "one deactivate+activate round trip must leave activation.toml BYTE-IDENTICAL — this is the \
         issue293 reproduction, where the round trip silently deleted charteredBy and dropped an \
         [always] process from a present list (which marks it INACTIVE, D0138/D0164)"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE NAMED LOSSES, asserted individually so a partial fix cannot pass ──────────────────────

#[test]
fn the_charter_reference_and_its_comment_survive_a_write() {
    let (root, _) = project_with_real_activation("charter");
    run(&root, &["deactivate", "render"]);
    let after = activation(&root);
    assert!(
        after.contains("charteredBy = \"d0226\""),
        "charteredBy is the D0225 charter for the whole set — a set without a charter is a \
         collection of switches nobody can account for: {after}"
    );
    assert!(
        after.contains("collection of switches nobody can account for"),
        "the comment EXPLAINING charteredBy must survive too; a fact whose reason is deleted is the \
         next thing to be deleted: {after}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_non_switchable_always_process_is_never_dropped_from_a_present_list() {
    let (root, _) = project_with_real_activation("always");
    // `decision-authoring` is reported [always] — it asserts no guard, so the regenerating writer
    // omitted it. Omission from a PRESENT list is a semantic change: it reads as INACTIVE.
    run(&root, &["deactivate", "render"]);
    let after = activation(&root);
    assert!(
        after.contains("\"decision-authoring\""),
        "an [always] process must stay in a list that exists — omitting it from a PRESENT list marks \
         it INACTIVE (D0138/D0164), so the writer would deactivate it by silence: {after}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE COMMAND STILL WORKS: preservation must not become paralysis ───────────────────────────

#[test]
fn the_one_named_process_is_the_only_thing_that_changes() {
    let (root, original) = project_with_real_activation("effective");
    run(&root, &["deactivate", "render"]);
    let after = activation(&root);
    assert!(!after.contains("\"render\""), "deactivate must actually remove the target: {after}");
    // Everything else is untouched: the two texts differ only where `render` was.
    let strip = |s: &str| s.replace("\"render\", ", "").replace(", \"render\"", "").replace("\"render\"", "");
    assert_eq!(
        strip(&after),
        strip(&original),
        "apart from the one process named, the file must be unchanged — a writer that preserves by \
         doing nothing is as broken as one that rewrites everything"
    );
    let _ = std::fs::remove_dir_all(&root);
}
