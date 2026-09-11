//! The DoD probe for `dcGuardReadsTheWorkingTreeOutsideTheHook` (issue464, D0440) — which
//! population the keystone guards judge.
//!
//! The incident: on 2026-09-10 the D0425 verifier ran `keel guard --no-receipt` over a tree whose
//! sprint edits were not yet staged and reported ALL PASS; the pre-commit hook then refused the
//! same edits on process-change once `git add` had run. The guards read `git diff --cached`, so
//! outside a hook they judged an empty index and called it a clean tree.
//!
//! The fix: inside a git hook (`GIT_INDEX_FILE`, which git sets for every hook, or `KEEL_HOOK`) the
//! staged read is unchanged; anywhere else the guards read the working tree against HEAD, with
//! untracked files as additions, and the summary line names which read was made.
//!
//! The D0388 pair, chosen before any tree was read: known-positive = an UNSTAGED edit to a locked
//! file with no Decision fails before staging (today it passes); known-negative = the same tree
//! with an unstaged marked Decision beside it passes. A third run pins the hook shape: with
//! `KEEL_HOOK` set the same unstaged edit is invisible, as the commit's population says it should be.
//!
//! Every child pins its own environment: the land's touched run executes INSIDE a post-commit hook,
//! where `GIT_INDEX_FILE` is inherited, so a test that did not remove it would read the index there
//! and the working tree at a terminal.

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

/// A committed project whose `.engine/processes/x.sysml` is under the keystone lock.
fn locked_project(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-wtread-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".engine").join("processes")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    std::fs::write(root.join(".engine").join("processes").join("x.sysml"), "package X {\n    // step one\n}\n")
        .expect("process");
    git(&root, &["init", "-q"]);
    git(&root, &["config", "user.email", "probe@example.invalid"]);
    git(&root, &["config", "user.name", "probe"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

fn edit_process_unstaged(root: &Path) {
    std::fs::write(root.join(".engine").join("processes").join("x.sysml"), "package X {\n    // step one, reworded\n}\n")
        .expect("edit");
}

fn marked_decision_unstaged(root: &Path) {
    std::fs::write(
        root.join(".engine").join("decisions").join("0001-x.sysml"),
        "package D0001 {\n    part d0001 : Decision {\n        :>> title = \"x\";\n    }\n    part d0001Marker : ProspectiveChange;\n    #ProspectiveChange dependency from d0001 to d0001;\n}\n",
    )
    .expect("decision");
}

/// `keel guard process-change` as a process OUTSIDE any hook: neither marker variable set.
fn guard_at_terminal(root: &Path) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(["guard", "process-change"])
        .current_dir(root)
        .env_remove("GIT_INDEX_FILE")
        .env_remove("KEEL_HOOK")
        .output()
        .expect("keel");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

/// `keel guard process-change` as the commit gate runs it.
fn guard_in_hook(root: &Path) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(["guard", "process-change"])
        .current_dir(root)
        .env_remove("GIT_INDEX_FILE")
        .env("KEEL_HOOK", "pre-commit")
        .output()
        .expect("keel");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

// ── Known-positive: an unstaged locked edit with no Decision FAILS at the terminal ────────────────

#[test]
fn an_unstaged_locked_edit_with_no_decision_fails_before_staging() {
    let root = locked_project("positive");
    edit_process_unstaged(&root);
    let (ok, text) = guard_at_terminal(&root);
    assert!(!ok, "the unstaged edit to a locked file must fail process-change at the terminal:\n{text}");
    assert!(text.contains(".engine/processes/x.sysml"), "the violation names the locked file:\n{text}");
    assert!(text.contains("(read: working tree)"), "the summary line names the read it made:\n{text}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── Known-negative: the same tree with an unstaged marked Decision beside it PASSES ──────────────

#[test]
fn an_unstaged_locked_edit_with_an_unstaged_marked_decision_passes() {
    let root = locked_project("negative");
    edit_process_unstaged(&root);
    marked_decision_unstaged(&root);
    let (ok, text) = guard_at_terminal(&root);
    assert!(ok, "the untracked marked Decision authorises the unstaged edit:\n{text}");
    assert!(text.contains("(read: working tree)"), "the summary line names the read it made:\n{text}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── The hook shape is unchanged: the staged population is what the commit carries ────────────────

#[test]
fn inside_a_hook_the_staged_read_is_unchanged() {
    let root = locked_project("hook");
    edit_process_unstaged(&root);
    // Unstaged: not in the commit's population, so the commit gate has nothing to refuse.
    let (ok, text) = guard_in_hook(&root);
    assert!(ok, "an unstaged edit is not in the index the hook judges:\n{text}");
    assert!(text.contains("(read: index)"), "the summary line names the read it made:\n{text}");
    // Staged: now it is, and the refusal is the existing one.
    git(&root, &["add", "-A"]);
    let (ok, text) = guard_in_hook(&root);
    assert!(!ok, "the staged edit with no Decision fails in the hook as before:\n{text}");
    assert!(text.contains("(read: index)"), "the summary line names the read it made:\n{text}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── A clean tree reads clean under either mode ───────────────────────────────────────────────────

#[test]
fn a_clean_tree_passes_under_both_reads() {
    let root = locked_project("clean");
    let (ok, text) = guard_at_terminal(&root);
    assert!(ok, "clean tree, working-tree read:\n{text}");
    let (ok, text) = guard_in_hook(&root);
    assert!(ok, "clean tree, index read:\n{text}");
    let _ = std::fs::remove_dir_all(&root);
}
