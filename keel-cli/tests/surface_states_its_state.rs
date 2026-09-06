//! Two GH reports on the generated `.claude/` surface, on the binary against a real scaffold.
//!
//! GH#49 / D0348 (dcDeactivatedSkillSaysSo): a DEACTIVATED process's skill is deployed with its
//! inactive state written first - not removed (a removed file reads as drift, GH#46) and not left
//! unchanged (the guard was off and the skill still told the agent to act).
//!
//! GH#51 / D0349 (dcSurfaceStampNamesTheBuild): the sidecar stamp carries the build commit beside the
//! version, and when the surface drifted the report names the build that stamped it and the build
//! running - two 0.3.1 builds had generated different surfaces and the report named neither.

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

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("ss{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    root
}

#[test]
fn a_deactivated_processes_skill_is_deployed_saying_it_is_inactive_and_reactivating_clears_it() {
    let root = scaffold("inactive");
    let skill = root.join(".claude/skills/render/SKILL.md");
    assert!(skill.exists(), "the scaffold deploys the render skill");
    let (ok, out) = run(&root, &["deactivate", "render"]);
    assert!(ok, "{out}");
    let (ok, out) = run(&root, &["sync-claude", "."]);
    assert!(ok, "{out}");
    let text = std::fs::read_to_string(&skill).expect("skill");
    assert!(text.starts_with("> **INACTIVE in this project.**") && text.contains("`render`") && text.contains("keel activate render"), "the inactive state is written FIRST: {}", text.lines().next().unwrap_or(""));
    assert!(text.contains("# render"), "and the skill's own text follows, unchanged: {text}");
    let (ok, out) = run(&root, &["sync-claude", "--check", "."]);
    assert!(ok, "the deployed text IS the generated text - no drift while inactive: {out}");
    // Reactivate: the banner goes, the surface is still clean.
    let (ok, out) = run(&root, &["activate", "render"]);
    assert!(ok, "{out}");
    assert!(run(&root, &["sync-claude", "."]).0);
    let text = std::fs::read_to_string(&skill).expect("skill");
    assert!(!text.contains("INACTIVE in this project"), "an active process's skill carries no banner: {text}");
    let (ok, out) = run(&root, &["sync-claude", "--check", "."]);
    assert!(ok, "{out}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_stamp_carries_the_build_and_a_drift_report_names_both_builds() {
    let root = scaffold("stamp");
    let sidecar = root.join(".claude/.keel-surface");
    let stamp = std::fs::read_to_string(&sidecar).expect("sidecar");
    let parts: Vec<&str> = stamp.split_whitespace().collect();
    assert_eq!(parts.len(), 2, "version and build: {stamp:?}");
    let (ok, out) = run(&root, &["version"]);
    assert!(ok && out.contains(parts[1]), "the stamped build is this binary's build commit ({}): {out}", parts[1]);
    // Forge the stamp to another build and mutate a skill: the report names the forged build and this one.
    std::fs::write(&sidecar, format!("{} deadbee\n", parts[0])).expect("forge");
    let skill = root.join(".claude/skills/render/SKILL.md");
    std::fs::write(&skill, "# render - edited by hand\n").expect("mutate");
    let (ok, out) = run(&root, &["sync-claude", "--check", "."]);
    assert!(!ok, "a mutated skill is drift: {out}");
    assert!(out.contains("stamped by build deadbee") && out.contains(&format!("this binary is build {}", parts[1])), "the diagnosis names both generators: {out}");
    // Regenerate: the stamp is this build's again, and the check is clean.
    assert!(run(&root, &["sync-claude", "."]).0);
    assert_eq!(std::fs::read_to_string(&sidecar).expect("sidecar").trim(), format!("{} {}", parts[0], parts[1]));
    assert!(run(&root, &["sync-claude", "--check", "."]).0);
    let _ = std::fs::remove_dir_all(&root);
}
