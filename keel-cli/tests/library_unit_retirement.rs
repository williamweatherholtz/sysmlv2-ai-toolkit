//! issue381 / GH#57, GH#58 (D0345): a library unit its upstream has abandoned can be RETIRED in the
//! library, and the retirement reaches every consumer - `import --from-library` refuses it naming the
//! reason and the replacement, `library list` marks it, and `keel status` marks an INSTALLED retired
//! unit. Every scenario runs the binary with an isolated HOME so the machine's real library is never
//! touched.

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

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A bare library remote seeded with one unit.
fn bare_library(base: &Path, unit: &str) -> PathBuf {
    let work = base.join("lib-work");
    std::fs::create_dir_all(work.join(unit)).expect("mkdir");
    std::fs::write(
        work.join(unit).join("unit.toml"),
        format!("unitId = \"lib-{unit}\"\nversion = 6\nprocess = \"{unit}\"\nskills = []\nrules = []\nguards = []\n"),
    )
    .expect("unit.toml");
    std::fs::write(work.join(unit).join(format!("processes__SL__{unit}.sysml")), "// unit payload\n").expect("payload");
    git(&work, &["init", "-q"]);
    git(&work, &["config", "user.email", "lib@example.invalid"]);
    git(&work, &["config", "user.name", "lib"]);
    git(&work, &["add", "-A"]);
    git(&work, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed library"]);
    let bare = base.join("library.git");
    let out = Command::new("git").args(["clone", "-q", "--bare"]).arg(&work).arg(&bare).output().expect("bare");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    bare
}

fn keel_home(home: &Path, cwd: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin()).args(args).env("USERPROFILE", home).env("HOME", home).env("KEEL_ACTOR", "ai").current_dir(cwd).output().expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn fixture(tag: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("keel-retire-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let home = base.join("home");
    std::fs::create_dir_all(&home).expect("mkdir home");
    let bare = bare_library(&base, "old-channel");
    let (ok, text) = keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    assert!(ok, "library init: {text}");
    let clone = home.join(".keel").join("library");
    git(&clone, &["config", "user.email", "t@example.invalid"]);
    git(&clone, &["config", "user.name", "t"]);
    (base, home)
}

const WHY: &str = "its auto-accept path records an acceptance in a hardcoded human's name";

#[test]
fn retiring_a_unit_marks_it_in_the_clone_and_a_fresh_import_refuses_naming_the_why() {
    let (base, home) = fixture("refuse");
    let (ok, text) = keel_home(&home, &base, &["process", "retire", "old-channel", "--why", WHY, "--replaced-by", "record-time standing consent in the binary (D0291)", "--at", "2026-09-06"]);
    assert!(ok, "retire succeeds: {text}");
    assert!(text.contains("marked RETIRED") && text.contains("NOT pushed"), "committed to the clone, push is separate: {text}");
    let toml = std::fs::read_to_string(home.join(".keel/library/old-channel/unit.toml")).expect("unit.toml");
    assert!(toml.contains("retired = true") && toml.contains(WHY) && toml.contains("version = 6"), "the record is in the unit, the version untouched: {toml}");
    let log = Command::new("git").arg("-C").arg(home.join(".keel/library")).args(["log", "-1", "--format=%s"]).output().expect("git log");
    assert!(String::from_utf8_lossy(&log.stdout).starts_with("retire old-channel:"), "one commit in the clone");
    // list marks it
    let (_, list) = keel_home(&home, &base, &["library", "list"]);
    assert!(list.contains("old-channel") && list.contains("RETIRED 2026-09-06") && list.contains(WHY), "{list}");
    // a project trying to import it is refused with the reason and the replacement
    let proj = base.join("proj");
    std::fs::create_dir_all(&proj).expect("mkdir");
    assert!(keel_home(&home, &proj, &["init", "."]).0, "scaffold");
    let (ok, text) = keel_home(&home, &proj, &["process", "import", "--from-library", "old-channel"]);
    assert!(!ok, "a retired unit is not importable: {text}");
    assert!(text.contains("RETIRED") && text.contains(WHY) && text.contains("Replaced by: record-time standing consent"), "the refusal carries why and what replaces it: {text}");
    assert!(!proj.join(".engine/processes/old-channel.sysml").exists(), "nothing landed");
    // a second retirement is refused - the record is written once
    let (ok, text) = keel_home(&home, &base, &["process", "retire", "old-channel", "--why", WHY, "--at", "2026-09-07"]);
    assert!(!ok && text.contains("already"), "{text}");
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_retirement_needs_a_reason() {
    let (base, home) = fixture("why");
    let (ok, text) = keel_home(&home, &base, &["process", "retire", "old-channel", "--why", "meh"]);
    assert!(!ok && text.contains("--why is required"), "a retirement with no real reason is refused: {text}");
    assert!(!std::fs::read_to_string(home.join(".keel/library/old-channel/unit.toml")).expect("toml").contains("retired"), "nothing written");
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn status_marks_an_installed_unit_its_upstream_retired() {
    let (base, home) = fixture("status");
    // A project that installed the unit BEFORE it was retired.
    let proj = base.join("proj");
    std::fs::create_dir_all(&proj).expect("mkdir");
    assert!(keel_home(&home, &proj, &["init", "."]).0, "scaffold");
    let contracts = proj.join(".engine/contracts");
    let installed = std::fs::read_to_string(contracts.join("installed-units.toml")).unwrap_or_default();
    std::fs::write(contracts.join("installed-units.toml"), format!("{installed}\n[lib-old-channel]\nprocess = \"old-channel\"\nversion = 6\n")).expect("install record");
    // Upstream retires it; this machine syncs.
    let (ok, text) = keel_home(&home, &base, &["process", "retire", "old-channel", "--why", WHY, "--at", "2026-09-06"]);
    assert!(ok, "{text}");
    let (_, status) = keel_home(&home, &proj, &["status", "."]);
    assert!(status.contains("old-channel v6 installed here is RETIRED 2026-09-06"), "status names the installed retired unit: {status}");
    assert!(!status.contains("available, not installed: old-channel"), "a retired unit is never offered: {status}");
    let _ = std::fs::remove_dir_all(&base);
}
