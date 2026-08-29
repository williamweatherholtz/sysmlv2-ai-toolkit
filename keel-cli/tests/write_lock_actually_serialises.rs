//! The arming probe for `ctlWriteLock` — the control that guards every model write.
//!
//! # Why this test exists, and why its absence was an assurance gap rather than an oversight
//!
//! `.engine/contracts/control-arming.toml` classifies `ctlWriteLock` as `not-machine-checkable`, on
//! this reasoning: *"In-process and unconditional (write.rs takes the lock or refuses loudly). There
//! is no configuration that could disarm it, so there is nothing to probe: absence of a switch is
//! stronger than a passing check."*
//!
//! The first half is true and the conclusion does not follow. "No switch to flip" rules out being
//! DISARMED BY CONFIGURATION. It says nothing about whether the lock actually serialises — and that
//! is the property the control exists for. issue184/issue185 record the failure directly: four
//! concurrent `record issue` calls once landed TWO issues with ALL FOUR EXITING 0. "There is no
//! configuration that could disarm it" was equally true on that day.
//!
//! The wider error the classification made — and it applies to seven other controls, not just this
//! one — is generalising "not machine-checkable BY A READ-ONLY VIEW" into "not checkable at all".
//! `keel controls` must not spawn processes or write files, so it genuinely cannot establish this.
//! A TEST may do both. The arming question was answered against the constraints of the surface that
//! happened to ask it.
//!
//! # What this asserts
//!
//! Real concurrency against the real lock, not a simulation of it: N OS processes invoking the
//! sanctioned write path at once on one tree. Every write must either LAND or FAIL LOUDLY. The
//! forbidden outcome is the recorded one — a writer that exits 0 having lost its write.

use std::path::{Path, PathBuf};
use std::process::Command;

const WRITERS: usize = 6;

fn keel_bin() -> PathBuf {
    // The integration harness puts the binary beside the test executable's directory.
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

/// A throwaway project the writers can hammer without touching the real tree.
fn fixture(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-locktest-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking").join("intake")).expect("mkdir tracking");
    std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir engine");
    std::fs::write(
        root.join(".tracking").join("issues.sysml"),
        "package Seed {\n}\n",
    )
    .expect("seed file");
    root
}

fn statement_args(root: &Path, i: usize) -> Vec<String> {
    vec![
        "record".into(),
        "statement".into(),
        "--text".into(),
        format!("concurrent writer {i}"),
        "--said-by".into(),
        "wweatherholtz".into(),
        "--said-at".into(),
        "2026-08-29".into(),
        "--title".into(),
        format!("writer {i}"),
        "--by".into(),
        "claudeOpus5".into(),
        "--at".into(),
        "2026-08-29".into(),
        "--root".into(),
        root.to_string_lossy().to_string(),
    ]
}

#[test]
fn concurrent_writers_either_land_or_fail_loudly_but_never_lose_a_write_silently() {
    let bin = keel_bin();
    if !bin.exists() {
        // A missing binary must not look like a passing probe (K2: fail loud, never silently pass).
        panic!("keel binary not found at {} — build before running this probe", bin.display());
    }
    let root = fixture("concurrent");

    // Spawn all writers before waiting on any, so they genuinely contend for the lock.
    let children: Vec<_> = (0..WRITERS)
        .map(|i| {
            Command::new(&bin)
                .args(statement_args(&root, i))
                .spawn()
                .expect("spawn writer")
        })
        .collect();

    let mut exited_zero = 0usize;
    for c in children {
        let out = c.wait_with_output().expect("writer finished");
        if out.status.success() {
            exited_zero += 1;
        }
    }

    // Count what actually landed, from the tree rather than from the writers' own reports —
    // the whole point is that a writer's exit code is not evidence about the tree.
    let landed: usize = std::fs::read_dir(root.join(".tracking").join("intake"))
        .map(|d| {
            d.filter_map(Result::ok)
                .filter_map(|e| std::fs::read_to_string(e.path()).ok())
                .map(|t| t.matches(": Statement").count())
                .sum()
        })
        .unwrap_or(0);

    assert_eq!(
        landed, exited_zero,
        "THE issue184/issue185 CLASS: {exited_zero} writer(s) exited 0 but only {landed} statement(s) \
         are in the tree. A writer that reports success having lost its write is the exact failure \
         `ctlWriteLock` exists to prevent, and the one its arming classification assumed away."
    );
    assert!(
        exited_zero > 0,
        "every one of {WRITERS} writers failed — the lock may be refusing unconditionally, which is \
         safe but not working. A control that never lets anything through is not armed, it is stuck."
    );

    let _ = std::fs::remove_dir_all(&root);
}
