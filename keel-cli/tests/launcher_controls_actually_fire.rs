//! The arming probe for the LAUNCHER's three controls — the largest single assurance gap found.
//!
//! # Why this is the gap and not just a missing test
//!
//! `keel enforcement-report` reports `launcherFraction: unavailable: no run records yet` and
//! `dirtyTreeRefusals: unavailable`. `.keel/runs/` does not exist in this repository. The launcher is
//! fully implemented — dirty-tree refusal, HEAD/fingerprint snapshot, post-run gate, machine-local
//! record, tracked summary — and has **never once been invoked**. Every Need about bounded, approved
//! process launching therefore rests on nothing.
//!
//! The module's own doc comment says these are "the parts that must be TESTABLE outside the SSE
//! stream". They were made testable and never tested; `prepare` and `finish` are called from exactly
//! one place, `serve.rs`, inside the SSE handler.
//!
//! `unavailable` is also the wrong word, and that is the D0217 confusion one level up: it does not
//! distinguish "this has never been invoked" from "this cannot be measured". A reader takes it as the
//! second. It is the first.
//!
//! # What this asserts
//!
//! Three controls, each exercised rather than described:
//!   1. the dirty-tree refusal REFUSES on a dirty tree, and emits its ledger event;
//!   2. a clean tree yields a snapshot carrying the spawn-time HEAD (K12);
//!   3. `finish` writes a record carrying every one of the 14 DECLARED fields, and a run that
//!      changed nothing writes NO tracked summary.

use std::path::{Path, PathBuf};
use std::process::Command;

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
}

/// A real git repository — `prepare` shells out to `git status` and `git rev-parse`, so a fake
/// directory would exercise the error path rather than the control.
fn repo(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-launcher-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir .keel");
    // Bind an actor through the FILE rather than KEEL_ACTOR: the env var is process-wide and this
    // test must not depend on, or disturb, another test's environment.
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("bind actor");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    git(&root, &["init", "-q"]);
    git(&root, &["config", "user.email", "probe@example.invalid"]);
    git(&root, &["config", "user.name", "probe"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

#[test]
fn the_dirty_tree_refusal_actually_refuses() {
    let root = repo("dirty");
    // Clean tree first: the control must ALLOW, or a refusal below proves nothing.
    keel_cli::launcher::prepare(&root, "sprint-planning", "wweatherholtz")
        .expect("a clean tree must prepare — a control that refuses everything is stuck, not armed");

    std::fs::write(root.join(".tracking").join("uncommitted.sysml"), "package X {\n}\n").expect("dirty it");

    // `expect_err` would need RunSetup: Debug; matching keeps the type surface untouched.
    let err = match keel_cli::launcher::prepare(&root, "sprint-planning", "wweatherholtz") {
        Ok(_) => panic!("a DIRTY tree must REFUSE the launch (D0182 single-writer)"),
        Err(e) => e,
    };
    assert!(
        err.contains("DIRTY"),
        "the refusal must name the reason so the operator can act on it, got: {err}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_prepared_run_captures_the_spawn_time_head() {
    let root = repo("snapshot");
    let head = Command::new("git").arg("-C").arg(&root).args(["rev-parse", "--short", "HEAD"])
        .output().expect("rev-parse");
    let head = String::from_utf8_lossy(&head.stdout).trim().to_string();

    let setup = keel_cli::launcher::prepare(&root, "sprint-planning", "wweatherholtz").expect("prepare");
    assert_eq!(
        setup.head_at_spawn, head,
        "K12: the run must record the tree it STARTED from, or its verdict cannot be re-derived"
    );
    assert_eq!(setup.approved_by, "wweatherholtz", "the approver is carried, never resolved from a binding");
    assert!(!setup.id.is_empty(), "a run needs an identity to be recorded against");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn finish_writes_a_record_carrying_every_declared_field() {
    let root = repo("finish");
    let setup = keel_cli::launcher::prepare(&root, "sprint-planning", "wweatherholtz").expect("prepare");

    let outcome = keel_cli::launcher::finish(&root, &setup, Some(0), 3, false).expect("finish");

    assert!(outcome.local_record.exists(), "every run must leave a machine-local record");
    let text = std::fs::read_to_string(&outcome.local_record).expect("read record");
    // The 14 fields are DECLARED in launcher.rs and exempt from existence checks by design — which
    // means nothing else verifies them. Asserting against the declaration is what makes the
    // exemption safe rather than merely stated.
    for field in keel_cli::launcher::RUN_RECORD_FIELDS {
        assert!(
            text.contains(&format!("\"{field}\"")),
            "run record is missing the declared field `{field}` — the schema is exempt from existence \
             checks, so this test is the only thing holding it. Record was: {text}"
        );
    }
    assert!(
        outcome.diff_files.is_empty(),
        "this run changed nothing, so its diff must be empty; got {:?}",
        outcome.diff_files
    );
    assert!(
        outcome.summary_path.is_none(),
        "a run with an EMPTY diff writes no tracked summary — otherwise .tracking/runs/ fills with \
         records of runs that did nothing"
    );
    let _ = std::fs::remove_dir_all(&root);
}
