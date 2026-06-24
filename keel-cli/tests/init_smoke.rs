//! Smoke test for `keel init` (D0093 spin-up): a fresh scaffold must be a WORKING project — it
//! validates clean, passes every guard, orients, and refuses to overwrite itself. Drives the REAL
//! `keel` binary end-to-end (via `CARGO_BIN_EXE_keel`), so it exercises the embedded scaffold + the
//! engine/instance remap exactly as a newcomer would. This is the cold-start regression guard the
//! console-arc retros flagged as missing (initSmokeTest).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static DIR_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_dir() -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    std::env::temp_dir().join(format!("keel_init_smoke_{pid}_{n}"))
}

fn keel() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keel"))
}

struct TmpProject(PathBuf);
impl Drop for TmpProject {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Count files under `dir` (recursively) matching `pred`.
fn walk_count(dir: &Path, pred: &dyn Fn(&Path) -> bool) -> usize {
    let mut n = 0;
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                n += walk_count(&p, pred);
            } else if pred(&p) {
                n += 1;
            }
        }
    }
    n
}

#[test]
fn init_scaffolds_a_working_project() {
    let dir = unique_dir();
    let _cleanup = TmpProject(dir.clone());
    let proj = dir.to_str().unwrap();

    // 1. init succeeds and lays down the scaffold.
    let out = keel().args(["init", proj]).output().expect("run keel init");
    assert!(out.status.success(), "init failed: {}", String::from_utf8_lossy(&out.stderr));
    assert!(dir.join(".engine").is_dir(), ".engine/ not scaffolded");
    assert!(dir.join("CLAUDE.md").is_file(), "CLAUDE.md not written");
    assert!(dir.join(".tracking").is_dir(), ".tracking/ not created");
    // engine/instance boundary (D0093): architecture decisions ship read-only under reference/, and
    // the new project's OWN decisions dir is created fresh + empty.
    assert!(dir.join(".engine").join("reference").join("decisions").is_dir(), "reference/decisions/ missing");
    assert!(dir.join(".engine").join("decisions").is_dir(), "fresh decisions/ missing");
    // scaffoldEngineDevExclude: the engine-DEV-only kernel/Python toolchain must NOT ship downstream.
    assert!(!dir.join(".engine").join("tools").exists(), ".engine/tools/ leaked into the scaffold (engine-dev only)");
    let py = walk_count(&dir.join(".engine"), &|p| p.extension().is_some_and(|e| e == "py" || e == "pyc"));
    assert_eq!(py, 0, "{py} python file(s) leaked into the scaffold (engine-dev only)");
    // scaffoldCommitGate: a Rust-only pre-commit gate is scaffolded (keel validate/guard; NO conda/kernel).
    let hook = std::fs::read_to_string(dir.join(".githooks").join("pre-commit")).expect(".githooks/pre-commit not scaffolded");
    assert!(hook.contains("keel") && hook.contains("validate") && hook.contains("guard"), "pre-commit gate missing keel validate/guard");
    assert!(!hook.contains("conda") && !hook.contains("python") && !hook.contains(".py"), "pre-commit gate must be kernel-free (no conda/python)");

    // 2. the fresh scaffold validates clean.
    let out = keel().args(["validate", proj]).output().expect("run keel validate");
    assert!(out.status.success(), "fresh scaffold failed validate: {}", String::from_utf8_lossy(&out.stdout));

    // 3. the fresh scaffold passes EVERY guard (the D0093 promise: spin up green).
    let out = keel().args(["guard", "all", proj]).output().expect("run keel guard");
    assert!(out.status.success(), "fresh scaffold failed guard: {}", String::from_utf8_lossy(&out.stdout));

    // 4. it orients (computable state, no crash).
    let out = keel().args(["orient", proj]).output().expect("run keel orient");
    assert!(out.status.success(), "fresh scaffold failed orient");
    assert!(String::from_utf8_lossy(&out.stdout).contains("\"ready\""), "orient output missing ready[]");

    // 5. re-init refuses to overwrite (exit 2, non-success) — never clobbers existing work.
    let out = keel().args(["init", proj]).output().expect("run keel init again");
    assert!(!out.status.success(), "re-init should refuse to overwrite an existing .engine/");
}
