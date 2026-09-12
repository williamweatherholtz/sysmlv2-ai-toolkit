//! Guard `gating-workflow-history` decides "runs the gate" from what a workflow INVOKES, by a fixed
//! set of needles. D0452 moved the gating verbs under `keel gate` / `keel audit`, so the needles had
//! to move with them - a needle still reading `keel validate` would match nothing in a rewritten
//! workflow and the guard would fall silent over every shallow checkout (the exact class the guard
//! exists for, issue229/issue260). This is the D0388 pair for that rewrite: a shallow workflow that
//! runs `keel gate validate` is caught; the same workflow with a full checkout is clean.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test binary path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn run(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("KEEL_ACTOR", "ai")
        .output()
        .expect("keel runs");
    (
        out.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
    )
}

/// The number immediately before `label` (`2 scanned` -> 2); a substring test on a count breaks at 10.
fn count_before(text: &str, label: &str) -> Option<u64> {
    let at = text.find(label)?;
    let digits: String = text[..at].chars().rev().take_while(char::is_ascii_digit).collect();
    digits.chars().rev().collect::<String>().parse().ok()
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("gwh{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    std::fs::create_dir_all(root.join(".github").join("workflows")).expect("workflows dir");
    root
}

/// A workflow that runs the routed gate, checked out shallow or in full.
fn workflow(root: &Path, name: &str, command: &str, full_history: bool) {
    let depth = if full_history { "        with:\n          fetch-depth: 0\n" } else { "" };
    let text = format!(
        "name: {name}\non: [push]\njobs:\n  gate:\n    runs-on: ubuntu-latest\n    steps:\n      \
         - uses: actions/checkout@v4\n{depth}      \
         - name: gate\n        run: {command}\n"
    );
    std::fs::write(root.join(".github").join("workflows").join(format!("{name}.yml")), text)
        .expect("write workflow");
}

#[test]
fn a_shallow_workflow_running_the_routed_gate_is_a_violation() {
    let root = scaffold("bites");
    workflow(&root, "ci", "keel gate validate . && keel gate guard .", false);
    let (_ok, out) = run(&root, &["gate", "guard", "gating-workflow-history", "."]);
    assert!(
        out.contains("FAIL") && out.contains("ci.yml") && out.contains("fetch-depth: 0"),
        "the guard must recognise `keel gate validate` as running the gate and NAME the shallow workflow: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_shallow_workflow_running_the_routed_audit_is_a_violation() {
    let root = scaffold("audit");
    workflow(&root, "ci", "keel audit history --since origin/main", false);
    let (_ok, out) = run(&root, &["gate", "guard", "gating-workflow-history", "."]);
    assert!(
        out.contains("FAIL") && out.contains("ci.yml"),
        "`keel audit history` is history-derived and must be recognised as gating: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_same_workflow_with_a_full_checkout_is_clean() {
    let root = scaffold("clean");
    workflow(&root, "ci", "keel gate validate . && keel gate guard .", true);
    let (_ok, out) = run(&root, &["gate", "guard", "gating-workflow-history", "."]);
    let scanned = count_before(&out, " scanned").unwrap_or(0);
    assert!(
        scanned >= 1 && out.contains("0 violation(s)"),
        "a full checkout is clean and the workflows (this one and the scaffold's keel-gate.yml) were SCANNED, not skipped: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}
