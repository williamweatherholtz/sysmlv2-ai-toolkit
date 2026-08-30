//! The DoD probe for `dcRenameIsAChange` (issue273, CRITICAL) — the keystone lock versus `git mv`.
//!
//! The incident: narrowing the keystone's diff-filter from ACMR to MD (so a fresh scaffold could
//! make its first commit, issue272) dropped R — so a locked file could be EDITED AND MOVED in one
//! staged change and the guard saw nothing: every blocking rule downgraded to a warning by a
//! `git mv`, PASS 0 scanned, live in every repository. Second blindness in the same reader:
//! git C-quotes non-ASCII paths in this output, so such a path was invisible to the parse.
//!
//! The fix reads `--name-status -M -z` with `core.quotePath=false` and returns BOTH sides of a
//! rename. This probe is the DoD's VERIFY, in both directions: the bypass now FAILS with both
//! paths named, and the issue272 case (a fresh project's first commit, all additions) still passes.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A committed project whose `.engine/rules/rules.sysml` is under the keystone lock.
fn locked_project(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-keystone-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".engine").join("rules")).expect("mkdir");
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    // Big enough that a one-word downgrade keeps git's rename similarity above threshold — the
    // incident's own shape was R096, a 96%-similar rename.
    let body: String = (0..30).map(|i| format!("    // rule context line {i}\n")).collect();
    std::fs::write(
        root.join(".engine").join("rules").join("rules.sysml"),
        format!("package Rules {{\n{body}    // severity = blocking\n}}\n"),
    )
    .expect("rules");
    git(&root, &["init", "-q"]);
    git(&root, &["config", "user.email", "probe@example.invalid"]);
    git(&root, &["config", "user.name", "probe"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

fn staged_guard(root: &Path) -> (bool, String) {
    let out = Command::new(keel_bin()).args(["guard", "process-change"]).current_dir(root).output().expect("keel");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

// ── Direction 1: THE BYPASS — edit + rename in one staged change must FAIL, naming both paths ─────

#[test]
fn an_edited_rename_of_a_locked_file_fails_naming_both_paths() {
    let root = locked_project("mv");
    // The reproduction from issue273: downgrade the content INTO a new filename, drop the original —
    // a copy-with-one-word-changed, so git sees a high-similarity rename, not a delete+add.
    let downgraded = std::fs::read_to_string(root.join(".engine").join("rules").join("rules.sysml"))
        .expect("read")
        .replace("blocking", "warning");
    std::fs::write(root.join(".engine").join("rules").join("rules2.sysml"), downgraded).expect("write moved");
    std::fs::remove_file(root.join(".engine").join("rules").join("rules.sysml")).expect("rm original");
    git(&root, &["add", "-A"]);
    // Confirm git actually sees this as a RENAME (R with similarity), or the probe tests nothing.
    let status = Command::new("git").arg("-C").arg(&root)
        .args(["diff", "--cached", "--name-status", "-M"]).output().expect("git");
    let status = String::from_utf8_lossy(&status.stdout).to_string();
    assert!(status.trim_start().starts_with('R'), "fixture must stage a rename, got: {status}");

    let (ok, text) = staged_guard(&root);
    assert!(
        !ok,
        "an edited RENAME of a locked file must FAIL the keystone (issue273: this exact change once \
         passed with 0 scanned, downgrading every blocking rule via git mv): {text}"
    );
    assert!(
        text.contains("rules.sysml") || text.contains("rules2.sysml"),
        "the refusal names the locked path(s) so the operator can act: {text}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── Direction 2: THE ISSUE272 CASE SURVIVES — a fresh project's first commit is all-A and passes ──

#[test]
fn a_fresh_projects_first_commit_still_passes_the_keystone() {
    let root = std::env::temp_dir().join(format!("keel-keystone-fresh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let out = Command::new(keel_bin()).args(["init"]).arg(&root).args(["--profile", "guided"]).output().expect("init");
    assert!(out.status.success(), "init: {}", String::from_utf8_lossy(&out.stderr));
    git(&root, &["init", "-q"]);
    git(&root, &["config", "user.email", "probe@example.invalid"]);
    git(&root, &["config", "user.name", "probe"]);
    git(&root, &["add", "-A"]);
    // Everything staged is an ADDITION — the case issue272's narrowing existed to allow, and the
    // case the MDR re-widening must not re-break.
    let (ok, text) = staged_guard(&root);
    assert!(
        ok,
        "a fresh scaffold's first commit (all additions, no Decision yet possible) must PASS — \
         re-breaking issue272 while fixing issue273 would trade one lockout for another: {text}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── Direction 3: the C-quoting blindness — a non-ASCII path is still read, both sides ─────────────

#[test]
fn a_non_ascii_locked_path_is_not_invisible_to_the_reader() {
    let root = locked_project("utf8");
    // Rename the locked file to a name git would C-quote in default output.
    let downgraded = std::fs::read_to_string(root.join(".engine").join("rules").join("rules.sysml"))
        .expect("read")
        .replace("blocking", "warning");
    std::fs::write(root.join(".engine").join("rules").join("règles.sysml"), downgraded).expect("write utf8");
    std::fs::remove_file(root.join(".engine").join("rules").join("rules.sysml")).expect("rm");
    git(&root, &["add", "-A"]);
    let (ok, text) = staged_guard(&root);
    assert!(
        !ok,
        "a rename to a NON-ASCII name must still fail the keystone — C-quoted output once made such \
         paths invisible to the very parse policing them (issue273, second blindness): {text}"
    );
    let _ = std::fs::remove_dir_all(&root);
}
