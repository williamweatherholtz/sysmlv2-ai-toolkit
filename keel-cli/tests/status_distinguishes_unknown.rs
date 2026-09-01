//! `keel status` must never render "I cannot tell" as "nothing to report" (D0270).
//!
//! A status screen exists to be trusted at a glance, which makes it the worst possible place for the
//! pass-at-zero hazard this codebase has already caught three times. A library that cannot be
//! reached is not a library with no drift; a commit with no CI run is not a commit that passed.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

/// A scaffolded project with an isolated HOME, so the machine's real library is invisible to it.
fn project(tag: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("keel-status-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let (root, home) = (base.join("proj"), base.join("home"));
    std::fs::create_dir_all(&home).expect("mkdir");
    let out = Command::new(keel_bin()).args(["init"]).arg(&root).args(["--profile", "guided"]).output().expect("init");
    assert!(out.status.success(), "init: {}", String::from_utf8_lossy(&out.stderr));
    (root, home)
}

fn status(root: &Path, home: &Path) -> String {
    let out = Command::new(keel_bin())
        .args(["status", "."])
        .current_dir(root)
        .env("USERPROFILE", home)
        .env("HOME", home)
        .output()
        .expect("keel");
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

#[test]
fn an_unreachable_library_reports_unknown_and_never_zero_drift() {
    let (root, home) = project("nolib");
    let text = status(&root, &home);
    let lib_line = text
        .lines()
        .find(|l| l.trim_start().starts_with("library"))
        .unwrap_or_default()
        .to_string();
    assert!(
        lib_line.contains("UNKNOWN"),
        "a library that is not initialised must report UNKNOWN — reporting OK would say 'nothing is \
         behind' when the truth is 'I have no idea': {lib_line}"
    );
    assert!(
        text.contains("could not be determined"),
        "and the summary line must COUNT it, so an unknown cannot hide in a clean-looking screen: {text}"
    );
    let _ = std::fs::remove_dir_all(root.parent().unwrap_or(&root));
}

#[test]
fn a_commit_with_no_run_is_not_reported_as_passing() {
    let (root, home) = project("noci");
    // No git repo at all: the CI line cannot resolve a HEAD, let alone a verdict.
    let text = status(&root, &home);
    let ci_line = text.lines().find(|l| l.trim_start().starts_with("ci")).unwrap_or_default();
    assert!(
        ci_line.contains("UNKNOWN"),
        "with no repository there is no verdict to report — 'passed' here would be an invention: {ci_line}"
    );
    assert!(!ci_line.contains("passed"), "and it must not say passed: {ci_line}");
    let _ = std::fs::remove_dir_all(root.parent().unwrap_or(&root));
}

#[test]
fn the_summary_counts_attention_and_unknown_separately() {
    let (root, home) = project("counts");
    let text = status(&root, &home);
    let last = text.lines().rfind(|l| l.contains("need attention")).unwrap_or_default();
    assert!(
        last.contains("need attention") && last.contains("could not be determined"),
        "the closing line must report BOTH counts — folding unknown into clean is how a status screen \
         becomes a comfort blanket: {last}"
    );
    let _ = std::fs::remove_dir_all(root.parent().unwrap_or(&root));
}

#[test]
fn this_repository_reports_its_real_state() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root");
    let out = Command::new(keel_bin()).args(["status", "."]).current_dir(repo).output().expect("keel");
    let text = String::from_utf8_lossy(&out.stdout);
    for section in ["engine", "library", "model", "work", "ci"] {
        assert!(text.contains(section), "every base must appear; missing `{section}`: {text}");
    }
    assert!(text.contains("guards"), "the model section names its guard count: {text}");
    let tracked = count_before(&text, " tracked file(s)").expect("a tracked-file count is printed");
    assert!(
        tracked > 0,
        "and it must read the real corpus rather than an empty one — {tracked} tracked file(s):
{text}"
    );
}

/// The number immediately preceding `label` in `text`, if any.
///
/// # Why a parse and not `!contains("0 <label>")`
///
/// That substring form is wrong for every count ending in zero, because "580 tracked file(s)"
/// CONTAINS "0 tracked file(s)". It has now broken this suite twice — once at 370 scanned, once at
/// 580 tracked files — and the first fix was applied to one site while an identical one stayed live.
/// Reading the number is the only form that says what was meant.
fn count_before(text: &str, label: &str) -> Option<u64> {
    let at = text.find(label)?;
    let digits: String = text[..at].chars().rev().take_while(char::is_ascii_digit).collect();
    digits.chars().rev().collect::<String>().parse().ok()
}
