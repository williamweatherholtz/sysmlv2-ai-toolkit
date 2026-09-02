//! A check that has never failed is not a check — it is a claim (issue264/D0231).
//!
//! `keel adoption-check` reported 25 of 25 units clean on its first run. That is exactly the shape of
//! a control that is aimed at nothing: this project has shipped a guard passing on an empty
//! population twice (issue250, and claude-surface-drift over zero skills), and printed
//! `process audit: every unit travels whole` while a schema symbol did not travel (issue263).
//!
//! So this test makes the check FAIL on purpose, and fails the build if it stops being able to. It
//! reproduces the real historical defect — issue252/issue253, where 23 of 24 units carried their
//! process and their SKILL.md while the declaration BINDING skill to process stayed home, so every
//! adoption into a project lacking that process landed on a red gate.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keel"))
}

fn run(args: &[&str], cwd: Option<&Path>) -> (bool, String) {
    let mut c = keel();
    c.args(args);
    if let Some(d) = cwd {
        c.current_dir(d);
    }
    let out = c.output().expect("run keel");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

struct Tmp(PathBuf);
impl Drop for Tmp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn a_unit_that_leaves_its_binding_behind_turns_the_foreign_gate_red() {
    let work = std::env::temp_dir().join(format!("keel-adopt-canfail-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    let _cleanup = Tmp(work.clone());
    let fixture = work.join("fx");
    let bundles = work.join("b");

    // A FOREIGN project, and it must start clean or nothing below means anything.
    assert!(run(&["init", &fixture.to_string_lossy()], None).0, "scaffold failed");
    let (ok, out) = run(&["guard", "all", &fixture.to_string_lossy()], None);
    assert!(ok, "a fresh scaffold must gate clean, else this test proves nothing: {out}");

    // Export a unit, then MUTILATE the bundle the way the engine really did before D0222: the
    // process and the SKILL.md travel, the declaration binding them does not.
    // From the FIXTURE, not from wherever cargo is running (issue339). With `None` the export resolved
    // to the LIVE repository root and wrote its install record there — every test run bumped the real
    // project's `intake` version. A test with a write side-effect on the working tree is a test that
    // edits history nobody asked it to.
    assert!(run(&["process", "export", "intake", "--out", &bundles.to_string_lossy()], Some(&fixture)).0, "export failed");
    let bundle = bundles.join("intake");
    let binding = bundle.join("skills").join("intake").join("registry.sysml");
    assert!(binding.is_file(), "the bundle should carry the binding declaration (D0222)");
    std::fs::remove_file(&binding).unwrap();

    // Make the fixture a project that LACKS the unit, then import the mutilated bundle.
    for rel in ["processes/intake.sysml", "skills/intake/registry.sysml", "skills/intake/SKILL.md"] {
        let p = fixture.join(".engine").join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
        let _ = std::fs::remove_file(p);
    }
    let _ = std::fs::remove_dir(fixture.join(".engine").join("skills").join("intake"));
    run(&["process", "import", &bundle.to_string_lossy()], Some(&fixture));

    // THE ASSERTION THAT MATTERS: the foreign gate must go RED.
    let (ok, out) = run(&["guard", "all", &fixture.to_string_lossy()], None);
    assert!(
        !ok,
        "a unit that left its skill->process binding behind was accepted by the foreign gate. \
         adoption-check can no longer detect the defect it was built for (issue252/issue253): {out}"
    );
    assert!(
        out.contains("process-skill"),
        "expected process-skill to be the guard that catches it, so the failure names the real cause: {out}"
    );
}

#[test]
fn the_checks_own_scope_is_stated_where_someone_will_read_it() {
    // The fixture is a CURRENT keel scaffold, so it cannot represent an adopter on an older vintage —
    // which is what penumbra was, and why issue259 and issue263 are NOT covered. That limit was found
    // by testing the claim rather than asserting it, and a limit nobody can see is one the next reader
    // will assume away. This test fails if the honest scope is ever quietly deleted from the source.
    let src = include_str!("../src/adoption_check.rs");
    for needle in ["WHAT THIS CANNOT CATCH", "issue259", "issue263", "vintage"] {
        assert!(src.contains(needle), "the stated scope lost `{needle}` — say what the check does NOT cover");
    }
}
