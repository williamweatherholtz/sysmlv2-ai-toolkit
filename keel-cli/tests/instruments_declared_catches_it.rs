//! Guard 65 `instruments-declared` is SHOWN catching its defect (D0360, D0361, scenario S-F7).
//!
//! Written WITH the guard rather than after it, which is the whole point of D0360: the guard that
//! preceded this one was probed by hand and its receipt went into a commit message, so the census
//! counted it unproven and was right to.
//!
//! The defect: an instrument that produces a number, verdict or figure while nothing declares it, so
//! it sits outside the computed control structure and no analysis reaches it. That is how six defects
//! in one day landed in channels the STPA self-analysis could not see. The mirror defect matters too -
//! a declaration naming a path that no longer exists claims something is being watched that is not.

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
    let root = base.join(format!("ins{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    std::fs::create_dir_all(root.join("scripts")).expect("scripts dir");
    root
}

fn declare(root: &Path, body: &str) {
    let dir = root.join(".engine").join("contracts");
    std::fs::create_dir_all(&dir).expect("contracts dir");
    std::fs::write(dir.join("instruments.toml"), body).expect("write manifest");
}

#[test]
fn an_undeclared_instrument_in_the_tree_is_a_violation() {
    let root = scaffold("undecl");
    std::fs::write(root.join("scripts").join("coverage_number.py"), "print(42)\n").expect("script");
    declare(&root, "# nothing declared\n");
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("FAIL") && out.contains("coverage_number.py"),
        "the guard must NAME the undeclared instrument: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_declaration_whose_path_is_gone_is_a_violation() {
    // The mirror defect: a stale entry is a claim that something is watched when it is not.
    let root = scaffold("stale");
    declare(
        &root,
        "[gone]\npath = \"scripts/deleted_probe.py\"\nkind = \"instrument\"\n",
    );
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("FAIL") && out.contains("deleted_probe.py"),
        "a declaration must resolve, or the inventory lies in the other direction: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_declared_instrument_is_clean_and_an_argued_exclusion_is_clean() {
    let root = scaffold("clean");
    std::fs::write(root.join("scripts").join("coverage_number.py"), "print(42)\n").expect("script");
    std::fs::write(root.join("scripts").join("cleanup.py"), "pass\n").expect("script");
    declare(
        &root,
        "[coverage]\npath = \"scripts/coverage_number.py\"\nkind = \"instrument\"\n\n\
         [notInstruments]\ncleanup = \"deletes temporary files - not a measure\"\n",
    );
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("0 violation(s)"),
        "a declared instrument and an argued exclusion are both clean: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_project_with_no_manifest_is_never_accused() {
    // The activation convention: a project that never adopted this control has not violated it.
    let root = scaffold("none");
    std::fs::write(root.join("scripts").join("coverage_number.py"), "print(42)\n").expect("script");
    let manifest = root.join(".engine").join("contracts").join("instruments.toml");
    let _ = std::fs::remove_file(&manifest);
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("0 violation(s)") && out.contains("0 scanned"),
        "no manifest means nothing declared and nothing accused: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}
