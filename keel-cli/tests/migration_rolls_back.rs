//! Fault-injected proof of migration reversibility (srMigrationIsReversible, D0252's unbuilt clause).
//!
//! The promise existed in an accepted Decision and the mechanism did not: the only recovery was an
//! error message advising the human to run `git checkout -- .`. The failure is not hypothetical —
//! during design, piping migrate's output to `head` closed the pipe, killed the command mid-apply,
//! and left one file written with the pin unstamped: a tree that is neither vintage.
//!
//! A rollback that has never been observed to run is a claim (D0253). These cases run it.
//!
//! TWO FAILURE CLASSES, and the distinction is the whole design:
//!   DETECTED — the process is alive and restores its own writes.
//!   INTERRUPTED — the process is killed and runs no code, so a marker written before the first byte
//!     lets the NEXT run detect and restore. That is the only recovery a killed process can have.

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

fn git(dir: &Path, args: &[&str]) -> bool {
    Command::new("git").arg("-C").arg(dir).args(args).output().is_ok_and(|o| o.status.success())
}

fn run(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin()).args(args).current_dir(dir).output().expect("keel");
    (
        out.status.success(),
        format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
    )
}

/// A committed project whose pin is stale, so a migration has real work to do.
fn stale_project(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-rollback-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let out = Command::new(keel_bin()).args(["init"]).arg(&root).args(["--profile", "guided"]).output().expect("init");
    assert!(out.status.success(), "init: {}", String::from_utf8_lossy(&out.stderr));
    assert!(git(&root, &["init", "-q"]), "git init");
    assert!(git(&root, &["config", "user.email", "p@e.invalid"]), "config");
    assert!(git(&root, &["config", "user.name", "probe"]), "config");
    let pin = root.join(".engine/contracts/engine-version.toml");
    let stale = std::fs::read_to_string(&pin)
        .expect("pin")
        .lines()
        .map(|l| if l.trim_start().starts_with("engine") { "engine = \"0.0.1\"".to_string() } else { l.to_string() })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&pin, stale).expect("write stale pin");
    assert!(git(&root, &["add", "-A"]), "stage");
    assert!(git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "on the old engine"]), "commit");
    root
}

fn dirty(root: &Path) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain", "--", ".engine", ".tracking"])
        .output()
        .expect("git status");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn cleanup(root: &Path) {
    let _ = std::fs::remove_dir_all(root);
}

/// Drop the read-only bit the fault injection set, so the fixture can be deleted. Uses `attrib`
/// rather than `set_permissions`, because clippy rightly flags `set_readonly(false)` as a
/// cross-platform footgun and this is a Windows-only fixture teardown.
#[cfg(windows)]
fn unprotect(path: &Path) {
    let _ = Command::new("cmd").args(["/C", "attrib", "-R"]).arg(path).output();
}

// ── INTERRUPTED: the marker is how a killed process gets recovered ────────────────────────────

#[test]
fn an_interrupted_migration_is_restored_by_the_next_run() {
    let root = stale_project("interrupted");
    let head = {
        let out = Command::new("git").arg("-C").arg(&root).args(["rev-parse", "HEAD"]).output().expect("git");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };
    // FAULT INJECTION: simulate exactly what a killed migration leaves behind — some files written,
    // and the marker still on disk because the process never reached the line that removes it.
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("migrate-in-progress"), &head).expect("marker");
    std::fs::write(root.join(".engine/contracts/unit-ids.toml"), "# half-written by a killed run\n").expect("write");
    std::fs::write(root.join(".engine/processes/orphan-from-crash.sysml"), "// created by a killed run\n").expect("write");
    assert!(!dirty(&root).is_empty(), "precondition: the tree is dirty after the simulated crash");

    let (_, text) = run(&root, &["migrate", "."]);
    assert!(
        text.contains("recovered"),
        "the next run must DETECT the unfinished migration and say so — silent recovery would hide \
         that a run failed at all: {text}"
    );
    // Everything the killed run wrote is gone: modified files restored, created files removed.
    let after = std::fs::read_to_string(root.join(".engine/contracts/unit-ids.toml")).expect("unit-ids");
    assert!(!after.contains("half-written"), "a MODIFIED file is restored from the commit: {after:.80}");
    assert!(
        !root.join(".engine/processes/orphan-from-crash.sysml").exists(),
        "a file the killed run CREATED is removed — checkout alone would have left it behind"
    );
    assert!(
        !root.join(".keel").join("migrate-in-progress").exists(),
        "and the marker is cleared, or every later run would 'recover' from a migration that finished"
    );
    cleanup(&root);
}

#[test]
fn a_completed_migration_leaves_no_marker_for_the_next_run_to_recover_from() {
    let root = stale_project("completed");
    let (ok, text) = run(&root, &["migrate", "."]);
    assert!(ok, "migrate must succeed on a clean stale project: {text}");
    assert!(
        !root.join(".keel").join("migrate-in-progress").exists(),
        "a completed run clears its marker — a marker left behind turns the rollback INTO the defect, \
         restoring a good tree to its pre-migration state on the next invocation"
    );
    // The migration actually did something, or this test proves nothing about the success path.
    let pin = std::fs::read_to_string(root.join(".engine/contracts/engine-version.toml")).expect("pin");
    assert!(!pin.contains("0.0.1"), "precondition: the migration re-stamped the pin: {pin}");

    // And a second run over the completed tree does NOT restore it.
    let (_, again) = run(&root, &["migrate", "."]);
    assert!(!again.contains("recovered"), "a completed tree must not be 'recovered': {again}");
    assert!(!pin.contains("0.0.1"), "and the pin stays advanced");
    cleanup(&root);
}

// ── DETECTED: an unverifiable migration is rolled back rather than left written ────────────────

/// The re-plan check exists to catch a step that did not do what it reported. Before this work it
/// printed "migrated but NOT verified — inspect `git diff`" and LEFT the tree written, which is the
/// exact state reversibility exists to refuse. Injected by making a planned target read-only, so a
/// write fails mid-apply with other files already written.
#[test]
#[cfg(windows)]
fn a_write_failure_mid_apply_restores_every_file_already_written() {
    let root = stale_project("writefail");
    // Make the file the resync plans to touch unwritable, so the apply fails after starting.
    let target = root.join(".engine/contracts/unit-ids.toml");
    let mut perms = std::fs::metadata(&target).expect("meta").permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&target, perms).expect("chmod");

    let (ok, text) = run(&root, &["migrate", "."]);
    // Either the write failed and rolled back, or the platform allowed the write after all — say
    // which, rather than passing vacuously on a fault that was never injected.
    if ok {
        unprotect(&target);
        cleanup(&root);
        panic!("fault was not injected: the read-only file was written anyway, so this case tested nothing");
    }
    assert!(
        text.contains("ROLLED BACK"),
        "a detected write failure must restore, not advise: {text}"
    );
    unprotect(&target);
    assert_eq!(
        dirty(&root),
        "",
        "after the rollback the tree is byte-identical to its pre-migration commit — that is the \
         whole guarantee, and `git status --porcelain` is how it is measured"
    );
    cleanup(&root);
}
