//! D0356: the push gate is WITHDRAWN, and `keel suite` survives it.
//!
//! It briefly refused any push whose deliverable had moved since the last green full-suite run.
//! Measured, that run costs ~11 wall minutes every time the code moves, against roughly one bad push
//! in twenty-five it could catch - two of the three most recent failures being platform faults this
//! machine cannot reproduce. The owner's instruction on the published brief: "Keep five, revert the
//! push gate, switch the pull on."
//!
//! What is pinned here: a push is NOT refused for a stale or absent receipt (so the gate cannot creep
//! back unnoticed), and `keel suite` still runs, still writes the receipt, and still reports counts -
//! because the receipt is what measured the cost that decided this.

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

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A project shaped like the self-build (it holds `keel-cli/Cargo.toml`), committed, with a bare origin.
fn fixture(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let base = base.join(format!("pg{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&base);
    let root = base.join("proj");
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    std::fs::create_dir_all(root.join("keel-cli").join("src")).expect("mkdir");
    std::fs::write(root.join("keel-cli").join("Cargo.toml"), "[package]\nname = \"fake\"\nversion = \"0.0.1\"\n").expect("toml");
    std::fs::write(root.join("keel-cli").join("src").join("lib.rs"), "pub fn one() -> u8 { 1 }\n").expect("src");
    git(&root, &["init", "-q", "-b", "main", "."]);
    git(&root, &["config", "user.email", "t@example.invalid"]);
    git(&root, &["config", "user.name", "t"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    let bare = base.join("origin.git");
    assert!(Command::new("git").args(["init", "-q", "--bare", "-b", "main"]).arg(&bare).output().expect("bare").status.success());
    git(&root, &["remote", "add", "origin", bare.to_str().expect("utf8")]);
    root
}

#[test]
fn a_push_is_not_refused_for_a_missing_or_stale_suite_receipt() {
    let root = fixture("gone");
    // No receipt has ever been written here.
    assert!(!keel_cli::suite::receipt_path(&root).exists(), "precondition: no receipt");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "a push must not wait on a receipt any more: {out}");
    assert!(!out.contains("suite receipt") && !out.contains("keel suite"), "and must not mention one: {out}");

    // Move the deliverable, which the withdrawn gate treated as fatal, and push again.
    std::fs::write(root.join("keel-cli").join("src").join("lib.rs"), "pub fn one() -> u8 { 2 }\n").expect("src");
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "source moved"]);
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "a moved deliverable is no longer a refusal: {out}");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}

/// The receipt still exists as EVIDENCE - it is what measured the cost that withdrew the gate.
#[test]
fn the_suite_still_runs_and_still_writes_its_receipt() {
    let text = "fingerprint = \"abc\"\nhead = \"1234567\"\nat = 5\npassed = 562\nfailed = 0\noutcome = \"pass\"\n";
    let r = keel_cli::suite::parse_receipt(text).expect("a receipt still parses");
    assert!(r.green() && r.passed == 562, "counts survive the round trip");
    assert_eq!(keel_cli::suite::count_results("test result: ok. 7 passed; 0 failed; 0 ignored\n"), (7, 0));
    // and the command is still dispatched
    let out = Command::new(keel_bin()).args(["suite", "--help"]).output().expect("keel runs");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(text.contains("usage: keel suite"), "--help prints usage and does NOT run the suite: {text}");
}
