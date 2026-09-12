//! The DoD probe for `dcResyncIsNotASelfModification` (issue475, D0441) — process-change tells the
//! engine's own published text arriving on a project from a hand edit to a locked file.
//!
//! The incident: `keel migrate` resyncs `.engine/` from the engine embedded in the binary, and
//! `.engine/skills/**/SKILL.md` is a locked path. Under the staged read the refusal fell at a
//! downstream project's post-migrate commit; under D0440's working-tree read it fell inside
//! migrate's own D0336 gate, and six `migration_fixture_is_realistic` tests reverted the update on
//! 2026-09-10 with `locked file(s) changed (.engine/skills/actor-enrollment/SKILL.md) with NO
//! co-committed process-change Decision`.
//!
//! The D0388 pair, chosen before any tree was read: known-positive = a committed project whose
//! locked skill file differs from the embedded text is overwritten with the embedded text and
//! passes process-change with no Decision, the warning line naming the path; known-negative = the
//! same overwrite with one line appended fails naming the file. A third case pins the self-build
//! exclusion: the positive tree plus a `keel-cli/Cargo.toml` fails the same way, because there the
//! embedded engine IS the tree.
//!
//! Every child removes `GIT_INDEX_FILE` / `KEEL_HOOK` so the read is the working tree's whatever
//! shell runs the test (the land's touched run executes inside a post-commit hook).

use std::path::{Path, PathBuf};
use std::process::Command;

const SKILL: &str = ".engine/skills/actor-enrollment/SKILL.md";

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

/// The text the binary under test embeds - the crate's own `.engine/` at build time.
fn embedded_skill() -> String {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root").join(SKILL);
    std::fs::read_to_string(&src).expect("the engine ships actor-enrollment/SKILL.md")
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A committed project whose locked skill file holds text the engine does NOT ship.
fn stale_project(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-resync-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(SKILL).parent().expect("skill dir")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    std::fs::write(root.join(SKILL), "# actor-enrollment\n\nan older release's text\n").expect("stale skill");
    git(&root, &["init", "-q"]);
    git(&root, &["config", "user.email", "probe@example.invalid"]);
    git(&root, &["config", "user.name", "probe"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

fn guard_at_terminal(root: &Path) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(["gate", "guard", "process-change", "--no-receipt"])
        .arg(root)
        .env_remove("GIT_INDEX_FILE")
        .env_remove("KEEL_HOOK")
        .output()
        .expect("run keel gate guard");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

fn cleanup(root: &Path) {
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn known_positive_the_embedded_text_passes_the_lock_with_no_decision() {
    let root = stale_project("pos");
    std::fs::write(root.join(SKILL), embedded_skill()).expect("resync");
    let (ok, text) = guard_at_terminal(&root);
    assert!(ok, "a locked file carrying the engine's own text is the engine arriving, not an edit:\n{text}");
    assert!(
        text.contains("engine resync") && text.contains(SKILL),
        "the exemption is announced, naming the path, so the suspension is visible:\n{text}"
    );
    assert!(text.contains("(read: working tree)"), "the read is named:\n{text}");
    cleanup(&root);
}

#[test]
fn known_negative_one_appended_line_is_back_under_the_lock() {
    let root = stale_project("neg");
    std::fs::write(root.join(SKILL), format!("{}\none more line the engine did not ship\n", embedded_skill())).expect("edit");
    let (ok, text) = guard_at_terminal(&root);
    assert!(!ok, "an edited locked file is refused whatever it started as:\n{text}");
    assert!(text.contains(SKILL) && text.contains("NO co-committed process-change Decision"), "names the file:\n{text}");
    assert!(!text.contains("engine resync"), "no exemption is claimed for an edited file:\n{text}");
    cleanup(&root);
}

#[test]
fn the_self_build_gets_no_exemption() {
    let root = stale_project("self");
    std::fs::create_dir_all(root.join("keel-cli")).expect("mkdir");
    std::fs::write(root.join("keel-cli").join("Cargo.toml"), "[package]\nname = \"keel-cli\"\n").expect("manifest");
    std::fs::write(root.join(SKILL), embedded_skill()).expect("same text");
    let (ok, text) = guard_at_terminal(&root);
    assert!(!ok, "in the self-build the embedded engine IS the tree, so equality proves nothing:\n{text}");
    assert!(text.contains(SKILL), "names the file:\n{text}");
    cleanup(&root);
}
