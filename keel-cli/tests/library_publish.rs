//! Gherkin for `dcLibraryPublish` (sprint 493, D0250 clause D) — WRITTEN BEFORE the implementation.
//!
//! Publishing is the LOUD direction: `keel process publish <name>` exports the unit into the
//! machine-local library clone and COMMITS, naming unit and version. It never pushes — the push is
//! the human-visible act. A unit whose content did not move publishes nothing (the issue302
//! no-op-version semantics extended to the library). And after a publish, `library sync` stays
//! quiet: AHEAD is the sanctioned state, not the divergence defect.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn keel_home(home: &Path, cwd: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .env("USERPROFILE", home)
        .env("HOME", home)
        .current_dir(cwd)
        .output()
        .expect("keel runs");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

/// A project with one exportable guard-less process, an isolated HOME, and an initialised library.
fn fixture(tag: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("keel-pub-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let home = base.join("home");
    std::fs::create_dir_all(&home).expect("mkdir");
    // An empty bare library remote.
    let bare = base.join("library.git");
    std::fs::create_dir_all(&bare).expect("mkdir");
    let out = Command::new("git").args(["init", "-q", "--bare"]).arg(&bare).output().expect("bare");
    assert!(out.status.success());
    // git clone of an EMPTY bare works and leaves an empty clone on a default branch.
    keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    let clone = home.join(".keel").join("library");
    git(&clone, &["config", "user.email", "pub@example.invalid"]);
    git(&clone, &["config", "user.name", "pub"]);
    // The project: a declared process + its skill, the minimum an export needs.
    let proj = base.join("proj");
    std::fs::create_dir_all(proj.join(".engine").join("processes")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".engine").join("skills").join("tiny-process")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".tracking")).expect("mkdir");
    std::fs::write(proj.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    std::fs::write(
        proj.join(".engine").join("processes").join("tiny-process.sysml"),
        "// tiny\npackage ProcessTiny {\n}\n",
    )
    .expect("process");
    std::fs::write(proj.join(".engine").join("skills").join("tiny-process").join("SKILL.md"), "# tiny\n").expect("skill");
    std::fs::write(
        proj.join(".engine").join("skills").join("tiny-process").join("registry.sysml"),
        "package SkillsRegistryTiny {\n}\n",
    )
    .expect("registry");
    (base, home, proj, clone)
}

#[test]
fn publish_lands_the_unit_as_one_commit_and_never_pushes() {
    let (base, home, proj, clone) = fixture("lands");
    let before_remote = Command::new("git").arg("-C").arg(base.join("library.git")).args(["rev-list", "--all", "--count"]).output().expect("git");
    let before_remote = String::from_utf8_lossy(&before_remote.stdout).trim().to_string();

    let (ok, text) = keel_home(&home, &proj, &["process", "publish", "tiny-process"]);
    assert!(ok, "publish must land the unit in the clone: {text}");
    assert!(
        clone.join("tiny-process").join("unit.toml").exists(),
        "the exported unit sits in the clone as a directory: {text}"
    );
    let last = git(&clone, &["log", "-1", "--format=%s"]);
    assert!(
        last.contains("tiny-process"),
        "ONE commit naming the unit — publishing is the LOUD direction (D0250 clause D): {last}"
    );
    // Never pushes: the remote's commit count is unchanged.
    let after_remote = Command::new("git").arg("-C").arg(base.join("library.git")).args(["rev-list", "--all", "--count"]).output().expect("git");
    let after_remote = String::from_utf8_lossy(&after_remote.stdout).trim().to_string();
    assert_eq!(before_remote, after_remote, "publish NEVER pushes — the push is the human-visible act");
    // And sync tolerates the sanctioned AHEAD state quietly.
    let (ok, text) = keel_home(&home, &proj, &["library", "sync"]);
    assert!(ok, "AHEAD after publish is the sanctioned state, not the divergence defect: {text}");
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn publishing_an_unchanged_unit_is_a_stated_no_op_with_no_commit() {
    let (base, home, proj, clone) = fixture("noop");
    let (ok, _) = keel_home(&home, &proj, &["process", "publish", "tiny-process"]);
    assert!(ok);
    let head = git(&clone, &["rev-parse", "HEAD"]);
    let (ok, text) = keel_home(&home, &proj, &["process", "publish", "tiny-process"]);
    assert!(ok, "a no-op publish is stated, not a failure: {text}");
    assert!(
        text.to_lowercase().contains("no-op") || text.to_lowercase().contains("unchanged") || text.to_lowercase().contains("nothing to publish"),
        "the no-op is SAID (issue302 semantics: a version that did not move does not publish): {text}"
    );
    assert_eq!(
        head,
        git(&clone, &["rev-parse", "HEAD"]),
        "an unchanged unit must create NO commit — churn makes the library log useless as a review surface"
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn publishing_a_unit_the_project_does_not_declare_refuses() {
    let (base, home, proj, _clone) = fixture("absent");
    let (ok, text) = keel_home(&home, &proj, &["process", "publish", "no-such-process"]);
    assert!(!ok, "an undeclared process cannot publish: {text}");
    assert!(text.contains("no-such-process"), "naming what was asked for: {text}");
    let _ = std::fs::remove_dir_all(&base);
}
