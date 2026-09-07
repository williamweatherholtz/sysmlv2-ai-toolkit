//! Guard 64 `gate-environment-parity` is SHOWN catching its defect (D0360, issue385).
//!
//! The guard was probed by hand the day it was written - the token stripped out of release.yml, the
//! guard shown to fail, the file restored - and the receipt for that probe went into a commit message
//! rather than into a test. The census that D0360 rests on then found the guard sitting among the 27
//! with no demonstrated catch, which is exactly right: a sentence in a commit message is not evidence
//! anyone can re-run.
//!
//! So this constructs the defect rather than describing it. Two workflows that run `cargo test`, one
//! supplying a variable its sibling does not - the shape that let the v0.4.0 tag gate red on a test
//! green on every push and publish nothing - and the guard must report it. The clean case is asserted
//! too, because a control that fires on everything is not a control either.

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

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("gep{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    std::fs::create_dir_all(root.join(".github").join("workflows")).expect("workflows dir");
    root
}

/// Write a workflow that runs the suite, with or without the token its sibling has.
fn workflow(root: &Path, name: &str, with_token: bool) {
    let env_block = if with_token {
        "        env:\n          GH_TOKEN: ${{ github.token }}\n"
    } else {
        ""
    };
    let text = format!(
        "name: {name}\non: [push]\njobs:\n  gate:\n    runs-on: ubuntu-latest\n    steps:\n      \
         - uses: actions/checkout@v4\n        with:\n          fetch-depth: 0\n      \
         - name: cargo test\n{env_block}        run: cargo test --workspace\n"
    );
    std::fs::write(root.join(".github").join("workflows").join(format!("{name}.yml")), text)
        .expect("write workflow");
}

#[test]
fn a_gating_workflow_missing_its_siblings_variable_is_a_violation() {
    let root = scaffold("bites");
    workflow(&root, "ci", true);
    workflow(&root, "release", false); // the defect: same command, different world
    let (_ok, out) = run(&root, &["guard", "gate-environment-parity", "."]);
    assert!(
        out.contains("FAIL") && out.contains("release.yml"),
        "the guard must NAME the workflow that runs the suite without its sibling's variable: {out}"
    );
    assert!(
        out.contains("GH_TOKEN"),
        "and name the variable, so the refusal is actionable rather than a verdict: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn two_workflows_that_agree_are_not_a_violation() {
    // A control that fires on everything has no information in it. This is the other half of the
    // proof, not a formality.
    let root = scaffold("clean");
    workflow(&root, "ci", true);
    workflow(&root, "release", true);
    let (_ok, out) = run(&root, &["guard", "gate-environment-parity", "."]);
    assert!(
        out.contains("0 violation(s)"),
        "agreeing workflows are clean: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_project_whose_gate_declares_nothing_is_never_accused() {
    // The expectation is DERIVED from the tree: with no variable supplied anywhere, there is nothing
    // to be missing, and a downstream project whose suite never touches the network must stay green.
    let root = scaffold("silent");
    workflow(&root, "ci", false);
    workflow(&root, "release", false);
    let (_ok, out) = run(&root, &["guard", "gate-environment-parity", "."]);
    assert!(
        out.contains("0 violation(s)"),
        "nothing declared means nothing missing: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}
