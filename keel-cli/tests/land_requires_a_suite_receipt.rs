//! D0353 (dcLandRequiresASuiteReceipt): `keel land` refuses to push a self-build tree whose
//! DELIVERABLE changed since the last green suite run on this machine, unless the suite ran at this
//! tree; a docs-only change needs no new receipt; `KEEL_LAND_UNTESTED=1` pushes anyway and records
//! an obligation. On the binary against a scaffold shaped like a self-build (it holds
//! `keel-cli/Cargo.toml`) with a bare remote, so `land` has somewhere to push.

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

fn run_env(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (bool, String) {
    let mut c = Command::new(keel_bin());
    c.args(args).current_dir(dir).env("KEEL_ACTOR", "ai");
    for (k, v) in env {
        c.env(k, v);
    }
    let out = c.output().expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn run(dir: &Path, args: &[&str]) -> (bool, String) {
    run_env(dir, args, &[])
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A keel project that is also shaped like the self-build, committed, with a bare origin.
fn fixture(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let base = base.join(format!("lr{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&base);
    let root = base.join("proj");
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    std::fs::create_dir_all(root.join("keel-cli").join("src")).expect("mkdir");
    std::fs::write(root.join("keel-cli").join("Cargo.toml"), "[package]\nname = \"fake\"\nversion = \"0.0.1\"\n").expect("cargo toml");
    std::fs::write(root.join("keel-cli").join("src").join("lib.rs"), "pub fn one() -> u8 { 1 }\n").expect("src");
    std::fs::write(root.join("README.md"), "# fixture\n").expect("readme");
    git(&root, &["init", "-q", "-b", "main", "."]);
    git(&root, &["config", "user.email", "t@example.invalid"]);
    git(&root, &["config", "user.name", "t"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    let bare = base.join("origin.git");
    let out = Command::new("git").args(["init", "-q", "--bare", "-b", "main"]).arg(&bare).output().expect("bare");
    assert!(out.status.success());
    git(&root, &["remote", "add", "origin", bare.to_str().expect("utf8")]);
    root
}

fn plant_receipt(root: &Path, outcome: &str) {
    let fp = keel_cli::suite::fingerprint(root).expect("fingerprint");
    std::fs::create_dir_all(root.join(".keel").join("metrics")).expect("mkdir");
    std::fs::write(keel_cli::suite::receipt_path(root), format!("fingerprint = \"{fp}\"\nhead = \"seed\"\nat = 1\npassed = 5\nfailed = {}\noutcome = \"{outcome}\"\n", u8::from(outcome != "pass"))).expect("receipt");
}

fn commit_all(root: &Path, msg: &str) {
    git(root, &["add", "-A"]);
    git(root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", msg]);
}

#[test]
fn a_deliverable_change_without_a_fresh_green_receipt_is_refused_and_docs_only_is_not() {
    let root = fixture("refuse");
    // No receipt at all: refused, naming the remedy.
    let (ok, out) = run(&root, &["land", "."]);
    assert!(!ok, "{out}");
    assert!(out.contains("no suite receipt") && out.contains("keel suite"), "{out}");
    // A green receipt for THIS tree: lands.
    plant_receipt(&root, "pass");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "{out}");
    // A docs-only change: the deliverable fingerprint is unchanged, the receipt still covers it.
    std::fs::write(root.join("README.md"), "# fixture, documented\n").expect("readme");
    commit_all(&root, "docs");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok, "a docs-only change needs no new receipt: {out}");
    // A source change: refused by name until the suite runs again.
    std::fs::write(root.join("keel-cli").join("src").join("lib.rs"), "pub fn one() -> u8 { 2 }\n").expect("src");
    commit_all(&root, "source moved");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(!ok, "{out}");
    assert!(out.contains("deliverable CHANGED since the last green suite run") && out.contains("keel suite"), "{out}");
    // A red receipt for the new tree is not enough either.
    plant_receipt(&root, "fail");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(!ok && out.contains("was RED"), "{out}");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}

#[test]
fn the_override_pushes_and_records_an_obligation_never_silently() {
    let root = fixture("override");
    let (ok, out) = run_env(&root, &["land", "."], &[("KEEL_LAND_UNTESTED", "1")]);
    assert!(ok && out.contains("landed"), "{out}");
    assert!(out.contains("OVERRIDDEN") && out.contains("obligation recorded"), "{out}");
    let obligations = std::fs::read_dir(root.join(".tracking").join("obligations")).expect("obligations dir");
    let planted = obligations.flatten().filter(|e| e.file_name().to_string_lossy().starts_with("land-untested")).count();
    assert_eq!(planted, 1, "one tracked obligation names the untested push");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}
