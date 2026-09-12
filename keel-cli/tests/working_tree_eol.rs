//! issue478 / dcWorkingTreeEolMatchesTheAttribute: a tracked path whose working-copy line ending
//! differs from the ending its `.gitattributes` declares is a NAMED REFUSAL - guard `working-tree-eol`
//! fails naming it, `keel suite --touched` writes `outcome = "eol-mismatch"` in place of a verdict, and
//! `keel land` refuses the push with the changed paths first and the count of the rest.
//!
//! D0388 pair, chosen before any real tree is read: the known-negative is a fixture whose declared
//! paths hold their ending (an `eol=crlf` file that IS CRLF is never named); the known-positive is the
//! same fixture with one `eol=lf` file rewritten CRLF. `core.autocrlf` is pinned `false` so the
//! fixture reads the same on Windows and on Linux CI.

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

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn commit(root: &Path, msg: &str) {
    git(root, &["add", "-A"]);
    git(root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", msg]);
}

/// Bytes exactly as given - `std::fs::write` translates nothing on any host.
fn write(root: &Path, rel: &str, text: &str) {
    let p = root.join(rel);
    std::fs::create_dir_all(p.parent().expect("parent")).expect("mkdir");
    std::fs::write(p, text).expect("write");
}

const ATTRIBUTES: &str = "* text=auto\n*.rs text eol=lf\n*.sysml text eol=lf\n*.bat text eol=crlf\n";

/// A self-build-shaped project under a repo whose `.gitattributes` declares `eol=lf` for `.rs` and
/// `.sysml` and `eol=crlf` for `.bat`; `run.bat` IS CRLF, so the negative half of the pair is in the
/// tree from the start. Committed and pushed so `origin/main` is the base `land` measures from.
fn fixture(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let base = base.join(format!("eol{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&base);
    let root = base.join("proj");
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    git(&root, &["init", "-q", "-b", "main", "."]);
    git(&root, &["config", "core.autocrlf", "false"]);
    git(&root, &["config", "user.email", "t@example.invalid"]);
    git(&root, &["config", "user.name", "t"]);
    write(&root, ".gitattributes", ATTRIBUTES);
    write(&root, "keel-cli/Cargo.toml", "[package]\nname = \"fake\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"\n");
    write(&root, "keel-cli/src/lib.rs", "pub mod widget;\n");
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 1 }\n");
    write(&root, "keel-cli/tests/widget_check.rs", "#[test]\nfn widget_answers() { assert_eq!(fake::widget::answer(), 1); }\n");
    write(&root, "run.bat", "@echo off\r\necho crlf is what a batch file is\r\n");
    commit(&root, "seed");
    let bare = base.join("origin.git");
    assert!(Command::new("git").args(["init", "-q", "--bare", "-b", "main"]).arg(&bare).output().expect("bare").status.success());
    git(&root, &["remote", "add", "origin", bare.to_str().expect("utf8")]);
    git(&root, &["push", "-q", "origin", "main"]);
    root
}

/// The guard, both halves of the pair on one fixture: clean first, then one `eol=lf` path rewritten
/// CRLF, then restored.
#[test]
fn the_guard_passes_a_conforming_tree_and_names_the_one_path_rewritten_crlf() {
    let root = fixture("guard");
    let declared = git(&root, &["ls-files", "--eol", "--", "run.bat"]);
    assert!(declared.contains("w/crlf") && declared.contains("eol=crlf"), "the negative half is in the tree: {declared}");

    // Known-negative FIRST: every declared path holds its ending; the CRLF `.bat` is declared CRLF.
    let (ok, out) = run(&root, &["gate", "guard", "working-tree-eol", "."]);
    assert!(ok, "a conforming tree passes: {out}");
    assert!(out.contains("[guard:working-tree-eol] PASS"), "{out}");
    // Parse the count: `!contains("0 scanned")` is the substring form unknown_flag_is_refused.rs forbids
    // (it breaks at 10, 20, ... - CI run 34610841088 caught this line by that test, not by the touched set).
    let scanned: u64 = out
        .split("] PASS")
        .nth(1)
        .and_then(|rest| rest.split_whitespace().find_map(|w| w.trim_matches(|c: char| !c.is_ascii_digit()).parse().ok()))
        .expect("the PASS line carries a scanned count");
    assert!(scanned > 0, "the declared population is counted, not empty: {out}");
    assert!(!out.contains("run.bat"), "a CRLF file declared eol=crlf is never named: {out}");

    // Known-positive: the same bytes with CRLF endings under `eol=lf`. `git status` calls it clean.
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 1 }\r\n");
    // The shape issue478 found: after the `git add` every commit does, the index holds the normalised
    // LF blob, status is clean, and the working copy is still CRLF.
    git(&root, &["add", "-A"]);
    assert_eq!(git(&root, &["status", "--porcelain"]), "", "git normalises at the add, so status is clean");
    let (ok, out) = run(&root, &["gate", "guard", "working-tree-eol", "."]);
    assert!(!ok, "a CRLF copy of an eol=lf file fails: {out}");
    assert!(out.contains("keel-cli/src/widget.rs: the working copy is crlf while .gitattributes declares eol=lf"), "the path and both endings are named: {out}");
    assert_eq!(out.matches("ERROR").count(), 1, "exactly that one path, not the CRLF .bat and not the scaffold: {out}");
    assert!(out.contains("1 violation(s)"), "{out}");

    // Restored, the tree passes again.
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 1 }\n");
    let (ok, out) = run(&root, &["gate", "guard", "working-tree-eol", "."]);
    assert!(ok, "the restored tree passes: {out}");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}

/// `land` and `suite --touched` refuse on the same census, name the changed path first, count the
/// rest, and write `eol-mismatch` as the receipt's outcome; nothing is pushed. Restored, `land` lands.
#[test]
fn land_and_the_touched_run_refuse_naming_the_changed_path_first_and_the_fixed_tree_lands() {
    let root = fixture("land");
    let before = git(&root, &["rev-parse", "origin/main"]);
    // The push changes `widget.rs`, written CRLF; a second, unchanged `.sysml` is ALSO CRLF so the
    // line has a rest to count.
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 2 }\r\n");
    write(&root, "keel-cli/tests/widget_check.rs", "#[test]\r\nfn widget_answers() { assert_eq!(fake::widget::answer(), 2); }\r\n");
    commit(&root, "widget moves, written by a translating tool");
    let sysml = std::fs::read_dir(root.join(".tracking"))
        .expect(".tracking")
        .flatten()
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|x| x == "sysml") && p.is_file())
        .expect("the scaffold wrote a .sysml under .tracking");
    let lf = std::fs::read_to_string(&sysml).expect("read");
    std::fs::write(&sysml, lf.replace('\n', "\r\n")).expect("rewrite");
    git(&root, &["add", "-A"]);
    assert_eq!(git(&root, &["status", "--porcelain"]), "", "every rewrite is clean to git status once added");
    let rel = sysml.strip_prefix(&root).expect("under root").to_string_lossy().replace('\\', "/");

    let (ok, out) = run(&root, &["suite", "--touched", "."]);
    assert!(!ok, "the touched run refuses: {out}");
    assert!(out.contains("working-tree eol MISMATCH: 3 tracked paths hold a line ending .gitattributes does not declare (issue478)"), "{out}");
    assert!(out.contains("changed by this push: [keel-cli/src/widget.rs (w/crlf where eol=lf), keel-cli/tests/widget_check.rs (w/crlf where eol=lf)]"), "the changed paths come first: {out}");
    assert!(out.contains(&format!("and 1 unchanged path (first: {rel})")), "and the rest is counted: {out}");
    assert!(out.contains("REFUSING to run"), "{out}");
    let receipt = std::fs::read_to_string(root.join(keel_cli::touched::RECEIPT)).expect("receipt written");
    assert!(receipt.contains("outcome = \"eol-mismatch\""), "the receipt's outcome is the mismatch, not a verdict: {receipt}");
    // Receipt order is `git ls-files` order - the tree's, not the push's.
    assert!(receipt.contains(&format!("eol_mismatch = [\"{rel}\", \"keel-cli/src/widget.rs\", \"keel-cli/tests/widget_check.rs\"]")), "{receipt}");
    assert!(receipt.contains("eol_scanned = "), "the census population is in the receipt: {receipt}");
    assert!(!receipt.contains("outcome = \"pass\"") && !receipt.contains("outcome = \"fail\""), "{receipt}");

    let (ok, out) = run(&root, &["land", "."]);
    assert!(!ok, "land refuses the push: {out}");
    assert!(out.contains("keel land: working-tree eol MISMATCH: 3 tracked paths"), "the same line, from land: {out}");
    assert!(out.contains("REFUSING to push") && out.contains("Nothing was pushed"), "{out}");
    assert!(!out.contains("does not pass the gate"), "the eol line is the refusal, not one gate problem among many: {out}");
    assert_eq!(git(&root, &["rev-parse", "origin/main"]), before, "origin did not move");

    // Rewritten with the declared ending - the bytes git would have produced - the same tree lands.
    for p in [root.join("keel-cli/src/widget.rs"), root.join("keel-cli/tests/widget_check.rs"), sysml.clone()] {
        let crlf = std::fs::read_to_string(&p).expect("read");
        std::fs::write(&p, crlf.replace("\r\n", "\n")).expect("rewrite");
    }
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "the conforming tree lands: {out}");
    assert!(out.contains("working-tree eol: ") && out.contains("declared paths hold their ending"), "and says the census was clean: {out}");
    assert_ne!(git(&root, &["rev-parse", "origin/main"]), before, "origin moved");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}
