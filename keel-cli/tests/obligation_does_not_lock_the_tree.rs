//! The control for issue324 — the engine must not write a file that locks the project out of migrating.
//!
//! THREE INDIVIDUALLY-CORRECT CONTROLS COMPOSED INTO A LOCK, reported by a downstream session because
//! this repository structurally cannot reach it (`check_preconditions` refuses any tree holding
//! `keel-cli/Cargo.toml` as a self-build, so the engine never migrates itself):
//!
//!   1. `keel migrate` refuses a dirty tree — and its `git status --porcelain` sees UNTRACKED files,
//!      so one brand-new file is enough.
//!   2. Committing that file to clean the tree fails, because the pre-commit gate runs `keel validate`
//!      and validate REFUSES under engine-version skew (D0251).
//!   3. Skew clears by running the pinned binary or by migrating — and migrating is step 1.
//!
//! The file that springs it is written BY THE ENGINE: `record_obligation` (D0176/K7), whose own doc
//! comment reasons about deadlock at the FILE level ("one Issue per file so recording never deadlocks
//! on the file being repaired") and misses it at the TREE level. It fires preferentially when a project
//! is ALREADY unhealthy, because that is exactly when an obligation gets recorded.
//!
//! WHY THE CASES ARE SHAPED THIS WAY. The carve-out is only worth anything if it is NARROW, so the
//! suite spends two of its four cases proving what is still refused. A fix that let migrate run on any
//! dirty tree would pass a single happy-path test and destroy the property the refusal exists for —
//! that a half-migrated tree mixed with uncommitted edits cannot be told apart afterwards.

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

fn run(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin()).args(args).current_dir(dir).output().expect("keel runs");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

fn git(dir: &Path, args: &[&str]) -> bool {
    Command::new("git").arg("-C").arg(dir).args(args).output().is_ok_and(|o| o.status.success())
}

/// A committed project sitting BEHIND its pin — the state in which the deadlock is reachable.
fn stale_project(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-oblig-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold a project");
    assert!(git(&root, &["init", "-q"]), "git init");
    assert!(git(&root, &["config", "user.email", "o@example.invalid"]), "email");
    assert!(git(&root, &["config", "user.name", "o"]), "name");
    // Behind the pin: the precondition for skew, and therefore for the lock.
    let pin = root.join(".engine/contracts/engine-version.toml");
    let text = std::fs::read_to_string(&pin).expect("pin");
    let stale: String = text
        .lines()
        .map(|l| if l.trim_start().starts_with("engine") { "engine = \"0.0.1\"".to_string() } else { l.to_string() })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&pin, format!("{stale}\n")).expect("write stale pin");
    assert!(git(&root, &["add", "-A"]), "stage");
    assert!(git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "behind the pin"]), "commit");
    root
}

/// Exactly what `record_obligation` writes: one Issue per file under `.tracking/obligations/`.
fn write_obligation(root: &Path, slug: &str) -> PathBuf {
    let dir = root.join(".tracking").join("obligations");
    std::fs::create_dir_all(&dir).expect("mkdir obligations");
    let path = dir.join(format!("{slug}.sysml"));
    std::fs::write(
        &path,
        "// OBLIGATION (auto-recorded, D0176/K7)\npackage ObligationTest {\n}\n",
    )
    .expect("write obligation");
    path
}

fn cleanup(root: &Path) {
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn a_project_holding_only_an_engine_written_obligation_can_still_migrate() {
    let root = stale_project("escape");
    write_obligation(&root, "red-yield-1d65ab16");

    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(
        ok,
        "THE WHOLE DEFECT (issue324): a project behind its pin holding ONE engine-written obligation \
         record could neither migrate (dirty tree) nor commit (validate refuses under skew). The file \
         the engine wrote locked the project out of the only command that could unlock it:\n{text}"
    );
    cleanup(&root);
}

#[test]
fn the_tolerated_file_is_named_rather_than_silently_skipped() {
    let root = stale_project("named");
    write_obligation(&root, "red-yield-deadbeef");

    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(ok, "precondition: it migrates: {text}");
    assert!(
        text.contains("red-yield-deadbeef"),
        "the exemption must NAME the file it tolerated — a carve-out nobody is told about is \
         indistinguishable from a check that quietly stopped working:\n{text}"
    );
    assert!(
        text.to_lowercase().contains("tolerat"),
        "and say that it WAS an exemption, not report the file as ordinary output:\n{text}"
    );
    cleanup(&root);
}

#[test]
fn any_other_uncommitted_tracking_file_still_refuses() {
    let root = stale_project("refuses");
    // An ordinary authored file, uncommitted. NOT an obligation record, so the original reasoning
    // applies in full: this edit cannot be told apart from migration state afterwards.
    std::fs::write(root.join(".tracking").join("mine.sysml"), "package Mine {\n}\n").expect("write");

    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(
        !ok,
        "the carve-out must stay NARROW — a fix that let migrate run on any dirty tree would pass the \
         happy-path case and destroy the property the refusal exists for:\n{text}"
    );
    assert!(text.contains("mine.sysml"), "and the refusal still names what blocked it: {text}");
    cleanup(&root);
}

#[test]
fn a_modified_obligation_record_is_not_tolerated() {
    let root = stale_project("modified");
    let path = write_obligation(&root, "red-yield-committed");
    // Commit it, THEN edit it. A modified record is a hand-edit: no longer purely additive, no longer
    // separable from whatever else the tree is doing, so the reasoning behind the carve-out lapses.
    assert!(git(&root, &["add", "-A"]), "stage the obligation");
    assert!(
        git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "record the obligation"]),
        "commit the obligation"
    );
    std::fs::write(&path, "// OBLIGATION (auto-recorded)\npackage ObligationTest {\n  // edited by hand\n}\n")
        .expect("modify");

    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(
        !ok,
        "only NEW obligation records are tolerated. A modified one is a hand-edit, and tolerating it \
         would re-open the ambiguity the refusal exists to prevent:\n{text}"
    );
    cleanup(&root);
}
