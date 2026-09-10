//! D0421 / issue416: `keel land` runs the TOUCHED test set - the integration tests whose text names a
//! module the push changes - before the first push, once the human has accepted D0421; an empty set
//! runs nothing and says so; and while D0421 is proposed the set is printed and nothing is run (D0337).
//!
//! This is NOT the D0356 gate returning: `push_gate_is_withdrawn.rs` still pins that a push waits on
//! no suite receipt, and its fixture (a `lib.rs`-only crate, which names no module) lands through
//! this path with an empty set.

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

fn commit(root: &Path, msg: &str) {
    git(root, &["add", "-A"]);
    git(root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", msg]);
}

fn write(root: &Path, rel: &str, text: &str) {
    let p = root.join(rel);
    std::fs::create_dir_all(p.parent().expect("parent")).expect("mkdir");
    std::fs::write(p, text).expect("write");
}

/// A self-build-shaped crate: `widget` is a real module, `widget_check` a real integration test that
/// names it, committed and PUSHED so `origin/main` is the base the next push is measured from.
/// `accepted` plants the D0421 acceptance token the refusal is armed by.
fn fixture(tag: &str, accepted: bool) -> PathBuf {
    fixture_with(tag, accepted, false)
}

/// `prose_names_token`: the Decision file (proposed, no acceptance part) whose decision TEXT names
/// `d0421AcceptR1` - the live shape of D0421 at record time, which must NOT arm the gate (issue435).
fn fixture_with(tag: &str, accepted: bool, prose_names_token: bool) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let base = base.join(format!("tt{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&base);
    let root = base.join("proj");
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    write(&root, "keel-cli/Cargo.toml", "[package]\nname = \"fake\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"\n");
    write(&root, "keel-cli/src/lib.rs", "pub mod widget;\n");
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 1 }\n");
    write(&root, "keel-cli/tests/widget_check.rs", "#[test]\nfn widget_answers() { assert_eq!(fake::widget::answer(), 1); }\n");
    // A parseable package either way: every decisions-reading guard walks this directory, so an
    // unparseable file would make the tree gate red for a reason that is not the one under test.
    if prose_names_token {
        write(&root, ".engine/decisions/0421-touchedTestsRunBeforeLand.sysml", "package Decision0421Fixture {\n    private import EngineWork::*;\n    part d0421 : Decision { :>> id = \"00000000-0000-4000-8000-000000000421\"; :>> title = \"fixture\"; :>> context = \"a fixture Decision whose prose names the acceptance token (issue435)\"; :>> decision = \"once this Decision carries d0421AcceptR1 in its file, land runs the set\"; :>> rationale = \"the token in prose must not read as the acceptance itself\"; :>> consequences = \"none; a fixture\"; :>> status = DecisionStatus::proposed; :>> createdBy = \"t\"; :>> createdAt = \"2026-09-09\"; }\n}\n");
    }
    if accepted {
        // The acceptance is the DECLARED part with a passing outcome - the shape `keel accept` writes -
        // never the token named in prose, which is what armed the gate at record time (issue435).
        write(&root, ".engine/decisions/0421-touchedTestsRunBeforeLand.sysml", "package Decision0421Fixture {\n    private import EngineWork::*;\n    part d0421AcceptR1 : TestResult { :>> id = \"00000000-0000-4000-8000-000000000422\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"seed\"; :>> judgedAt = \"2026-09-09\"; :>> judgedBy = \"t\"; :>> createdBy = \"t\"; }\n}\n");
    }
    git(&root, &["init", "-q", "-b", "main", "."]);
    git(&root, &["config", "user.email", "t@example.invalid"]);
    git(&root, &["config", "user.name", "t"]);
    commit(&root, "seed");
    let bare = base.join("origin.git");
    assert!(Command::new("git").args(["init", "-q", "--bare", "-b", "main"]).arg(&bare).output().expect("bare").status.success());
    git(&root, &["remote", "add", "origin", bare.to_str().expect("utf8")]);
    git(&root, &["push", "-q", "origin", "main"]);
    root
}

#[test]
fn a_stale_test_naming_a_changed_module_refuses_the_push_and_the_fixed_one_lands() {
    let root = fixture("gate", true);
    // The module moves; the test that names it is now stale.
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 2 }\n");
    commit(&root, "widget moved");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(!ok, "a failing touched test must refuse the push: {out}");
    assert!(out.contains("REFUSING") && out.contains("widget_check"), "the refusal names the failing test: {out}");
    assert!(out.contains("changed modules [widget]"), "and the module that touched it: {out}");
    let receipt = std::fs::read_to_string(root.join(keel_cli::touched::RECEIPT)).expect("receipt written");
    assert!(receipt.contains("outcome = \"fail\"") && receipt.contains("failing = [\"widget_check\"]"), "the receipt records the failure: {receipt}");

    // The test is brought back to what the module now says; the same tree lands.
    write(&root, "keel-cli/tests/widget_check.rs", "#[test]\nfn widget_answers() { assert_eq!(fake::widget::answer(), 2); }\n");
    commit(&root, "test follows the module");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "the fixed tree lands: {out}");
    assert!(out.contains("touched tests pass"), "and says the set passed: {out}");
    let receipt = std::fs::read_to_string(root.join(keel_cli::touched::RECEIPT)).expect("receipt written");
    assert!(receipt.contains("outcome = \"pass\""), "the receipt records the pass: {receipt}");

    // A change touching no module any test names runs nothing, and the receipt records the empty set.
    write(&root, "keel-cli/src/other.rs", "pub fn other() -> u8 { 3 }\n");
    write(&root, "keel-cli/src/lib.rs", "pub mod widget;\npub mod other;\n");
    commit(&root, "an unnamed module");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "an untouched set lands: {out}");
    assert!(out.contains("set EMPTY"), "and says the set is empty: {out}");
    assert!(out.contains("unattributed (name no module): [keel-cli/src/lib.rs]"), "lib.rs is reported, not matched: {out}");
    let receipt = std::fs::read_to_string(root.join(keel_cli::touched::RECEIPT)).expect("receipt written");
    assert!(receipt.contains("outcome = \"empty\"") && receipt.contains("tests = []") && receipt.contains("stems = [\"other\"]"), "the empty set is a receipt too: {receipt}");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}

/// D0337: until the human accepts D0421 the set is computed and shown, and nothing is run or refused.
#[test]
fn while_d0421_is_proposed_the_set_is_printed_and_the_push_is_not_refused() {
    let root = fixture("inert", false);
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 2 }\n");
    commit(&root, "widget moved, test stale");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "an inert gate refuses nothing: {out}");
    assert!(out.contains("set [widget_check]") && out.contains("INERT"), "but the set is named and the state said: {out}");
    let receipt = std::fs::read_to_string(root.join(keel_cli::touched::RECEIPT)).expect("receipt written");
    assert!(receipt.contains("outcome = \"not-run\"") && receipt.contains("tests = [\"widget_check\"]"), "the receipt carries the set nothing ran: {receipt}");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}

/// issue435: the Decision's own prose names the acceptance token; that is not an acceptance. The set is
/// printed, nothing runs, nothing is refused - the same as a Decision that never mentions it.
#[test]
fn a_decision_whose_prose_names_the_token_does_not_arm_the_gate() {
    let root = fixture_with("prose", false, true);
    write(&root, "keel-cli/src/widget.rs", "pub fn answer() -> u8 { 2 }\n");
    commit(&root, "widget moved, test stale, token in prose");
    let (ok, out) = run(&root, &["land", "."]);
    assert!(ok && out.contains("landed"), "prose is not an acceptance; nothing refuses: {out}");
    assert!(out.contains("INERT"), "the state is said: {out}");
    let receipt = std::fs::read_to_string(root.join(keel_cli::touched::RECEIPT)).expect("receipt written");
    assert!(receipt.contains("outcome = \"not-run\""), "nothing ran: {receipt}");
    let _ = std::fs::remove_dir_all(root.parent().expect("base"));
}

/// The pure set: a known positive and a known negative (D0388), pinned outside the module's own tests.
#[test]
fn the_set_is_computed_from_text_by_whole_word() {
    let tests = vec![("land_gate".to_string(), "use keel_cli::sync::cmd_land;".to_string()), ("orient_view".to_string(), "keel orient .".to_string())];
    assert_eq!(keel_cli::touched::touched_tests(&tests, &["sync".to_string()], &[]), vec!["land_gate".to_string()]);
    assert!(keel_cli::touched::touched_tests(&tests, &["synced".to_string()], &[]).is_empty());
    assert_eq!(keel_cli::touched::module_stem("keel-cli/src/main.rs"), None, "main.rs names no module");
}
