//! dcProcessRemoveCommand: `keel process remove <name>` takes an installed unit out as ONE act, and
//! refuses while any live surface still points at it.
//!
//! Twice a unit was removed by hand - exec-summary (sprint 524) and decision-channel (sprint 527) -
//! and both times the process file and skill went while a contract entry stayed, because removal
//! existed nowhere as a command. What a hand removal misses is never the obvious file; it is the
//! install record or the minted id, which nothing reads until the next import. So the scenarios here
//! check the CONTRACTS as carefully as the files, and check that a removal blocked by a reference
//! wrote nothing at all - a half-removed unit is worse than an un-removed one.

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

fn keel(home: &Path, cwd: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .env("USERPROFILE", home)
        .env("HOME", home)
        .env("KEEL_ACTOR", "ai")
        .current_dir(cwd)
        .output()
        .expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

/// A scaffolded project carrying one installed unit `probe-unit`: its process file, its skill and
/// registry, and an entry in each of the four contracts an install writes.
fn project(tag: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("keel-remove-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let home = base.join("home");
    let proj = base.join("proj");
    std::fs::create_dir_all(&home).expect("mkdir home");
    std::fs::create_dir_all(&proj).expect("mkdir proj");
    assert!(keel(&home, &proj, &["init", "."]).0, "scaffold");

    let e = proj.join(".engine");
    std::fs::write(
        e.join("processes").join("probe-unit.sysml"),
        "package ProbeUnit {\n    // a process definition, standing in for a real unit\n}\n",
    )
    .expect("process file");
    let skill = e.join("skills").join("probe-unit");
    std::fs::create_dir_all(&skill).expect("mkdir skill");
    std::fs::write(skill.join("SKILL.md"), "---\nname: probe-unit\n---\nbody\n").expect("skill");
    // D0220: the skill declares its own deployment beside itself, naming the process it deploys and
    // its own location - which is how `deploying_skills` binds skill to process. The fixture uses the
    // real shape rather than a simplified one, so the removal is exercised through the same
    // resolution a live unit goes through.
    std::fs::write(
        skill.join("registry.sysml"),
        "package ProbeReg {\n    part probeUnitSkill : AISkill {\n        :>> purpose = \"deploys .engine/processes/probe-unit.sysml\";\n        :>> location = \".engine/skills/probe-unit/SKILL.md\";\n    }\n}\n",
    )
    .expect("registry");

    let c = e.join("contracts");
    std::fs::create_dir_all(&c).expect("mkdir contracts");
    let append = |f: &str, s: &str| {
        let p = c.join(f);
        let mut text = std::fs::read_to_string(&p).unwrap_or_default();
        text.push_str(s);
        std::fs::write(&p, text).expect("contract");
    };
    append("unit-ids.toml", "probe-unit = \"lib-probe\"\n");
    append("installed-units.toml", "[lib-probe]\nprocess = \"probe-unit\"\nversion = 3\n");
    append("unit-extras.toml", "[probe-unit]\nfiles = []\n");
    append("activation.toml", "probe-unit = true\n");
    (base, proj)
}

fn contracts_text(proj: &Path) -> String {
    let c = proj.join(".engine").join("contracts");
    ["unit-ids.toml", "installed-units.toml", "unit-extras.toml", "activation.toml"]
        .iter()
        .map(|f| std::fs::read_to_string(c.join(f)).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The whole point of the command: files AND every contract entry go together, and the report says
/// which contracts it actually touched rather than claiming a clean sweep it did not check.
#[test]
fn removing_a_unit_takes_its_files_and_every_contract_entry() {
    let (_base, proj) = project("clean");
    let home = _base.join("home");

    let (ok, dry) = keel(&home, &proj, &["process", "remove", "probe-unit", "--dry-run"]);
    assert!(ok, "dry run succeeds: {dry}");
    assert!(dry.contains("Nothing written"), "a dry run says so: {dry}");
    assert!(proj.join(".engine/processes/probe-unit.sysml").exists(), "dry run wrote nothing: {dry}");

    let (ok, text) = keel(&home, &proj, &["process", "remove", "probe-unit"]);
    assert!(ok, "remove succeeds: {text}");
    assert!(!proj.join(".engine/processes/probe-unit.sysml").exists(), "the process file is gone: {text}");
    assert!(!proj.join(".engine/skills/probe-unit/SKILL.md").exists(), "the skill is gone: {text}");
    assert!(!proj.join(".engine/skills/probe-unit/registry.sysml").exists(), "the registry is gone: {text}");
    assert!(!proj.join(".engine/skills/probe-unit").exists(), "no empty skill shell is left behind: {text}");

    let after = contracts_text(&proj);
    for leftover in ["probe-unit = \"lib-probe\"", "[lib-probe]", "[probe-unit]", "probe-unit = true"] {
        assert!(!after.contains(leftover), "contract entry `{leftover}` survived the removal:\n{after}");
    }
    assert!(text.contains("LIBRARY copy is untouched"), "the report says the library is untouched: {text}");
    assert!(text.contains("keystone lock"), "the report names the Decision the commit will need: {text}");
}

/// The refusal, and the part that matters most: NOTHING is written when a live surface still points
/// at the unit. A removal that half-succeeds turns four guards red at commit time with no indication
/// of which removal caused it.
#[test]
fn a_referenced_unit_is_refused_and_nothing_is_removed() {
    let (_base, proj) = project("referenced");
    let home = _base.join("home");
    std::fs::write(
        proj.join(".engine").join("docs").join("keeps-a-pointer.md"),
        "The unit is defined in .engine/processes/probe-unit.sysml and is still cited here.\n",
    )
    .expect("doc");

    let (ok, text) = keel(&home, &proj, &["process", "remove", "probe-unit"]);
    assert!(!ok, "a referenced unit is refused: {text}");
    assert!(text.contains("still referenced"), "the refusal says why: {text}");
    assert!(text.contains("keeps-a-pointer.md"), "and names the file: {text}");
    assert!(text.contains("Nothing was removed"), "and says nothing happened: {text}");

    assert!(proj.join(".engine/processes/probe-unit.sysml").exists(), "the process file survives a refusal");
    assert!(proj.join(".engine/skills/probe-unit/SKILL.md").exists(), "the skill survives a refusal");
    let after = contracts_text(&proj);
    assert!(after.contains("[lib-probe]") && after.contains("probe-unit = true"), "no contract was touched:\n{after}");
}

/// A name this project does not hold is a refusal with a pointer, not a silent success - the shape
/// that made hand-removal plausible in the first place.
#[test]
fn removing_a_unit_the_project_does_not_have_says_so() {
    let (_base, proj) = project("absent");
    let home = _base.join("home");
    let (ok, text) = keel(&home, &proj, &["process", "remove", "no-such-unit"]);
    assert!(!ok, "refused: {text}");
    assert!(text.contains("no process `no-such-unit`") && text.contains("process list"), "names it and points somewhere: {text}");
}
