//! issue380 / GH#56: an activation charter names a Decision the project holds, or the surfaces say so
//! instead of asserting it. A v0.3.1 `migrate` wrote the engine's own `activation.toml` over a
//! project's and `keel onboard` then printed `CHARTERED by d0226` - a Decision that project did not
//! contain. Now the `activation-manifest` guard fails a dangling `charteredBy` by name and `onboard`
//! prints UNRESOLVED CHARTER; a charter that resolves still reads CHARTERED. Run on the binary.

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
    let out = Command::new(keel_bin()).args(args).current_dir(dir).env("KEEL_ACTOR", "ai").output().expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("ch{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    root
}

fn charter(root: &Path, decision: &str) {
    std::fs::write(
        root.join(".engine/contracts/activation.toml"),
        format!("[processes]\ncharteredBy = \"{decision}\"\nactive = [\"agile-workflow\", \"doc-sync\"]\n"),
    )
    .expect("activation");
}

#[test]
fn a_charter_naming_a_decision_the_project_does_not_hold_is_a_violation_and_onboard_says_so() {
    let root = scaffold("dangling");
    charter(&root, "d0226");
    let (ok, out) = run(&root, &["guard", "activation-manifest", "."]);
    assert!(!ok, "a dangling charteredBy fails the guard: {out}");
    assert!(out.contains("charteredBy = \"d0226\" does not resolve") && out.contains("0226-*.sysml"), "the violation names the Decision and where it was looked for: {out}");
    let (_, onboard) = run(&root, &["onboard", "."]);
    assert!(onboard.contains("UNRESOLVED CHARTER d0226"), "onboard states the claim it cannot back: {onboard}");
    assert!(!onboard.contains("CHARTERED by d0226"), "and never prints it as fact: {onboard}");
    let (_, json) = run(&root, &["onboard", ".", "--json"]);
    assert!(json.contains("\"chartered\":false") && json.contains("\"charterResolves\":false"), "{json}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_charter_the_project_holds_still_reads_chartered() {
    let root = scaffold("held");
    let (ok, out) = run(
        &root,
        &["record", "decision", "--slug", "charter", "--title", "charter: the process set for this project", "--context", "c", "--decision", "these two processes, for the stated reasons", "--rationale", "r", "--consequences", "q", "--author", "ai", "--date", "2026-09-06"],
    );
    assert!(ok, "D0001 exists: {out}");
    charter(&root, "d0001");
    let (ok, out) = run(&root, &["guard", "activation-manifest", "."]);
    assert!(ok && out.contains("0 violation(s)"), "a resolving charter is not a violation: {out}");
    let (_, onboard) = run(&root, &["onboard", "."]);
    assert!(onboard.contains("CHARTERED by d0001"), "{onboard}");
    let _ = std::fs::remove_dir_all(&root);
}
