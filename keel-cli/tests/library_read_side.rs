//! Gherkin for `dcLibraryReadSide` (sprint 492, D0250 clauses A/B/C/F) — WRITTEN BEFORE the
//! implementation. The library is a git repository the author owns; each machine holds a CLONE
//! under `<home>/.keel/library`, which is a cache and never a source. Consuming is silent;
//! an unreachable remote is a stated staleness; availability is never activation.
//!
//! Every scenario overrides the child process's HOME/USERPROFILE to a fixture directory, so the
//! real machine-local state is never touched — the same isolation discipline as the hook probes.

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

/// A bare library remote seeded with one exported-unit-shaped directory.
fn bare_library(base: &Path, unit: &str, version: u32) -> PathBuf {
    let work = base.join("lib-work");
    std::fs::create_dir_all(work.join(unit)).expect("mkdir");
    std::fs::write(
        work.join(unit).join("unit.toml"),
        format!("unitId = \"lib-{unit}\"\nversion = {version}\nprocess = \"{unit}\"\nskills = []\nrules = []\nguards = []\n"),
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

/// Run keel with an isolated HOME, from `cwd`.
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

fn fixture(tag: &str) -> (PathBuf, PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("keel-lib-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let home = base.join("home");
    std::fs::create_dir_all(&home).expect("mkdir home");
    let bare = bare_library(&base, "shiny-process", 1);
    (base, home, bare)
}

// ── Scenario 1: init + sync clones; list answers ──────────────────────────────────────────────────

#[test]
fn init_clones_and_list_enumerates_the_units() {
    let (base, home, bare) = fixture("init");
    let (ok, text) = keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    assert!(ok, "library init must clone: {text}");
    assert!(home.join(".keel").join("library").join(".git").exists(), "the clone is machine-local state under <home>/.keel/library");
    let (ok, text) = keel_home(&home, &base, &["library", "list"]);
    assert!(ok, "{text}");
    assert!(
        text.contains("shiny-process") && text.contains('1'),
        "list enumerates units with versions from the CACHE: {text}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

// ── Scenario 2: upstream moves; sync fast-forwards silently ───────────────────────────────────────

#[test]
fn sync_fast_forwards_when_upstream_moves() {
    let (base, home, bare) = fixture("ff");
    keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    // Upstream advances: a second unit lands via another clone.
    let other = base.join("other");
    let out = Command::new("git").args(["clone", "-q"]).arg(&bare).arg(&other).output().expect("clone");
    assert!(out.status.success());
    git(&other, &["config", "user.email", "o@example.invalid"]);
    git(&other, &["config", "user.name", "o"]);
    std::fs::create_dir_all(other.join("second-unit")).expect("mkdir");
    std::fs::write(other.join("second-unit").join("unit.toml"), "unitId = \"lib-second\"\nversion = 1\nprocess = \"second-unit\"\nskills = []\nrules = []\nguards = []\n").expect("toml");
    git(&other, &["add", "-A"]);
    git(&other, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "second unit"]);
    git(&other, &["push", "-q", "origin", "HEAD"]);

    let (ok, text) = keel_home(&home, &base, &["library", "sync"]);
    assert!(ok, "a fast-forward sync must succeed: {text}");
    let (_, list) = keel_home(&home, &base, &["library", "list"]);
    assert!(list.contains("second-unit"), "the new unit is visible after sync: {list}");
    let _ = std::fs::remove_dir_all(&base);
}

// ── Scenario 3: a DIVERGED cache is a defect report, never a merge ────────────────────────────────

#[test]
fn a_diverged_cache_refuses_and_names_the_defect() {
    let (base, home, bare) = fixture("diverge");
    keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    // Doctor the clone: a local commit NOT from publish, while upstream also moves.
    let clone = home.join(".keel").join("library");
    git(&clone, &["config", "user.email", "x@example.invalid"]);
    git(&clone, &["config", "user.name", "x"]);
    std::fs::write(clone.join("rogue.txt"), "hand edit\n").expect("rogue");
    git(&clone, &["add", "-A"]);
    git(&clone, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "rogue local edit"]);
    let other = base.join("other2");
    let out = Command::new("git").args(["clone", "-q"]).arg(&bare).arg(&other).output().expect("clone");
    assert!(out.status.success());
    git(&other, &["config", "user.email", "o@example.invalid"]);
    git(&other, &["config", "user.name", "o"]);
    std::fs::write(other.join("upstream.txt"), "upstream\n").expect("up");
    git(&other, &["add", "-A"]);
    git(&other, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "upstream moves"]);
    git(&other, &["push", "-q", "origin", "HEAD"]);

    let (ok, text) = keel_home(&home, &base, &["library", "sync"]);
    assert!(!ok, "a DIVERGED cache must refuse — the cache is read-only downstream, so divergence is a defect to report, never a merge to perform (D0250 clause B): {text}");
    assert!(
        text.to_lowercase().contains("diverg"),
        "the refusal names the divergence: {text}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

// ── Scenario 4: unreachable remote — stated staleness, cache still answers ────────────────────────

#[test]
fn an_unreachable_remote_is_stated_staleness_and_the_cache_still_answers() {
    let (base, home, bare) = fixture("offline");
    keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    std::fs::remove_dir_all(&bare).expect("kill the remote");
    let (ok, text) = keel_home(&home, &base, &["library", "sync"]);
    assert!(ok, "an unreachable remote must NOT block work (D0250 clause C): {text}");
    assert!(
        text.to_lowercase().contains("stale") || text.to_lowercase().contains("unreachable"),
        "the staleness is STATED, never a silent pretence of currency: {text}"
    );
    let (ok, list) = keel_home(&home, &base, &["library", "list"]);
    assert!(ok && list.contains("shiny-process"), "the last-good cache still answers: {list}");
    let _ = std::fs::remove_dir_all(&base);
}

// ── Scenario 5: THE DIFFERENTIAL — availability is never activation ───────────────────────────────

#[test]
fn a_projects_gate_is_byte_identical_with_and_without_the_library() {
    let (base, home, bare) = fixture("differential");
    // A minimal project with a schema, gated BEFORE any library exists on this "machine".
    let proj = base.join("proj");
    std::fs::create_dir_all(proj.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".engine").join("contracts")).expect("mkdir");
    std::fs::write(proj.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    let schema_src = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("ws").join(".engine").join("schema");
    copy_tree(&schema_src, &proj.join(".engine").join("schema"));

    // The differential compares the WHOLE outcome — exit status and every byte of output — and
    // deliberately does NOT require the gate to be green: identical-on-red is the same property,
    // and demanding green would couple this scenario to every guard's fixture appetite.
    let (ok_before, before) = keel_home(&home, &proj, &["guard"]);
    keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    let (ok_after, after) = keel_home(&home, &proj, &["guard"]);
    assert_eq!(ok_before, ok_after, "the gate VERDICT moved when the library appeared");
    assert_eq!(
        before, after,
        "the gate verdict and guard set must be BYTE-IDENTICAL with the library present and absent — availability is never activation (srAvailabilityIsNotActivation), and this differential is the property that makes silent sync safe"
    );
    let _ = std::fs::remove_dir_all(&base);
}

fn copy_tree(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).expect("mkdir");
    for e in std::fs::read_dir(src).expect("read").flatten() {
        let p = e.path();
        let d = dst.join(e.file_name());
        if p.is_dir() {
            copy_tree(&p, &d);
        } else {
            std::fs::copy(&p, &d).expect("copy");
        }
    }
}

// ── Scenario 6: import --from-library lands a unit exactly as a directory import would ────────────

#[test]
fn import_from_library_resolves_the_cache_and_delegates() {
    let (base, home, bare) = fixture("import");
    keel_home(&home, &base, &["library", "init", &bare.to_string_lossy()]);
    let proj = base.join("proj2");
    std::fs::create_dir_all(proj.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(proj.join(".engine").join("contracts")).expect("mkdir");
    std::fs::write(proj.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");

    let (ok, text) = keel_home(&home, &proj, &["process", "import", "--from-library", "shiny-process", "--assume-local-base"]);
    assert!(ok, "import --from-library must resolve the cache and delegate to the one import path: {text}");
    assert!(
        proj.join(".engine").join("processes").join("shiny-process.sysml").exists()
            || text.contains("imported"),
        "the unit landed: {text}"
    );
    // And a unit the library does not hold refuses NAMING what it looked for.
    let (ok, text) = keel_home(&home, &proj, &["process", "import", "--from-library", "no-such-unit"]);
    assert!(!ok, "an absent unit refuses: {text}");
    assert!(text.contains("no-such-unit"), "naming the unit: {text}");
    let _ = std::fs::remove_dir_all(&base);
}
