//! Guard 65 `instruments-declared` is SHOWN catching its defect (D0360, D0361, D0363, scenario S-F7).
//!
//! Rewritten when the measures moved from a contract file into the model (D0363): the defect is the
//! same, the declaration is now a `Sensor` item, and the exclusion is argued in the file itself.
//!
//! The defect: something that produces a number, verdict or figure while nothing declares it, so it
//! sits outside the computed control structure, no analysis reaches it, and - since D0363 - no
//! verification case can verify it. That is how six defects in one day landed in channels the STPA
//! self-analysis could not see. The mirror defect matters too: a Sensor naming a mechanism that does
//! not exist claims something is watched when it is not.

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

/// Author one `Sensor` declaring `mechanism`, the way a project would.
fn declare_sensor(root: &Path, name: &str, mechanism: &str) {
    let body = format!(
        "package ProbeInstruments {{\n    private import EngineElement::*;\n    private import EngineSafety::*;\n\
         \n    part {name} : Sensor {{\n        :>> id = \"1f0c9a2b-4d5e-4a6f-8b70-{:012x}\";\n        \
         :>> title = \"{name}\";\n        :>> createdAt = \"2026-09-07\"; :>> createdBy = \"ai\";\n        \
         :>> mechanism = \"{mechanism}\";\n        :>> measures = \"a probe\";\n        \
         :>> determinism = Determinism::tree;\n        :>> reading = SensorReading::number;\n    }}\n}}\n",
        name.len() * 7717
    );
    let dir = root.join(".tracking");
    std::fs::create_dir_all(&dir).expect("tracking dir");
    std::fs::write(dir.join(format!("{name}.sysml")), body).expect("write sensor");
}

#[test]
fn an_undeclared_instrument_in_the_tree_is_a_violation() {
    let root = scaffold("undecl");
    std::fs::write(root.join("scripts").join("coverage_number.py"), "print(42)\n").expect("script");
    // one declared Sensor, so the guard is active, but it names something else
    declare_sensor(&root, "snOther", "scripts/other_measure.py");
    std::fs::write(root.join("scripts").join("other_measure.py"), "print(1)\n").expect("script");
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("FAIL") && out.contains("coverage_number.py"),
        "the guard must NAME the undeclared measure: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_sensor_whose_mechanism_is_gone_is_a_violation() {
    // The mirror defect: a stale measure claims something is watched when it is not.
    let root = scaffold("stale");
    declare_sensor(&root, "snGone", "scripts/deleted_probe.py");
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("FAIL") && out.contains("deleted_probe.py"),
        "a Sensor's mechanism must exist, or the inventory lies in the other direction: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_declared_measure_is_clean_and_an_exclusion_argued_in_the_file_is_clean() {
    let root = scaffold("clean");
    std::fs::write(root.join("scripts").join("coverage_number.py"), "print(42)\n").expect("script");
    std::fs::write(
        root.join("scripts").join("cleanup.py"),
        "# not-an-instrument: deletes temporary files - it measures nothing.\npass\n",
    )
    .expect("script");
    declare_sensor(&root, "snCoverage", "scripts/coverage_number.py");
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("0 violation(s)"),
        "a declared measure and an exclusion argued in the file are both clean: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_project_with_no_sensors_is_never_accused() {
    // The activation convention: a project that never adopted this control has not violated it.
    let root = scaffold("none");
    std::fs::write(root.join("scripts").join("coverage_number.py"), "print(42)\n").expect("script");
    let (_ok, out) = run(&root, &["guard", "instruments-declared", "."]);
    assert!(
        out.contains("0 violation(s)") && out.contains("0 scanned"),
        "no Sensor items means nothing declared and nothing accused: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}
