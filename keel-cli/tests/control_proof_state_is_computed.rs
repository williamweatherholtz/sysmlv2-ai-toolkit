//! A control's PROOF state is computed by the binary and reported beside DECLARED and ARMED
//! (D0360, dcControlProofStateIsComputed).
//!
//! The census used to live in `.engine/tools/guard_proof_census.py`, a script someone had to
//! remember to run. Now `keel show controls` carries `controlProof`: per enforced guard, PROVEN
//! (a test body names it and asserts a failure), UNPROVEN (named only around passing assertions)
//! or UNDETERMINED (named in no test body - the heuristic cannot tell). The script stays as the
//! independent reference: on one tree the two must agree, and this file checks that they do.
//!
//! D0388: the known-positive and known-negative fixtures below are fixed BEFORE the real tree is
//! read, so an agreement with the script cannot be an agreement between two instruments that both
//! see nothing.

use std::path::{Path, PathBuf};
use std::process::Command;

use keel_cli::control_proof::{census_of_bodies, census_over, test_bodies, ProofState};

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root")
}

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test binary path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn run(dir: &Path, args: &[&str]) -> String {
    let out = Command::new(keel_bin()).args(args).current_dir(dir).env("KEEL_ACTOR", "ai").output().expect("keel runs");
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

/// KNOWN-POSITIVE: a body that names the guard and asserts a non-empty violation set.
const POSITIVE: &str = "#[test]\nfn it_catches() {\n    let violations = guard(\"fixture-guard\");\n    assert!(!violations.is_empty());\n}\n";
/// KNOWN-NEGATIVE: a body that names the guard and asserts only that a clean tree passes.
const NEGATIVE: &str = "#[test]\nfn it_passes() {\n    let ok = guard(\"fixture-guard\");\n    assert!(ok);\n}\n";

#[test]
fn the_known_positive_is_proven_and_the_known_negative_is_unproven_before_any_tree_is_read() {
    let pos = test_bodies(POSITIVE).swap_remove(0);
    let neg = test_bodies(NEGATIVE).swap_remove(0);
    let proven = census_of_bodies(&[("p.rs".into(), pos.0, pos.1)], &["fixture-guard"], true);
    let unproven = census_of_bodies(&[("n.rs".into(), neg.0, neg.1)], &["fixture-guard"], true);
    assert_eq!(proven.guards[0].state, ProofState::Proven, "{proven:?}");
    assert_eq!(unproven.guards[0].state, ProofState::Unproven, "{unproven:?}");
    let nowhere = census_of_bodies(&[], &["fixture-guard"], true);
    assert_eq!(nowhere.guards[0].state, ProofState::Undetermined, "{nowhere:?}");
}

/// A guard added with no failure-asserting test appears in the actionable list the moment it
/// exists - here a name no test in this corpus carries, censused against the real corpus.
#[test]
fn a_guard_with_no_test_appears_in_the_undetermined_list_and_a_proven_real_guard_does_not() {
    // Assembled at run time: written as one literal, THIS body would name it, and the census's first
    // run of this very test reported it UNPROVEN - the heuristic reading the instrument that reads it.
    let nobody = format!("a-guard-{}-has-tested", "nobody");
    let c = census_over(repo(), &[nobody.as_str(), "identity-well-formed"]);
    assert!(c.corpus_present);
    let state = |g: &str| c.guards.iter().find(|x| x.guard == g).expect("guard row").clone();
    assert_eq!(state(&nobody).state, ProofState::Undetermined);
    // The REAL guard this change proved: identity-well-formed sat in the census's "named in no test
    // body" list on 2026-09-11 until `a_malformed_id_is_a_violation_the_guard_names` below.
    let real = state("identity-well-formed");
    assert_eq!(real.state, ProofState::Proven, "{real:?}");
    assert!(real.proven_by.iter().any(|t| t.starts_with("control_proof_state_is_computed.rs::")), "{real:?}");
    let dump: String = c.to_json().dump().split_whitespace().collect();
    assert!(dump.contains(&format!("\"undeterminedList\":[\"{nobody}\"]")), "{dump}");
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("cp{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    let out = run(&root, &["init", "."]);
    assert!(root.join(".tracking").is_dir(), "scaffold: {out}");
    root
}

/// Guard `identity-well-formed` SHOWN catching its defect: a malformed id passes `validate` and is
/// unique, so nothing else in the model ever notices (the defect its doc names). A clean scaffold
/// passes first, so the catch is not a guard that fails everything.
#[test]
fn a_malformed_id_is_a_violation_the_guard_names() {
    let root = scaffold("idwf");
    let clean = run(&root, &["gate", "guard", "identity-well-formed", "."]);
    assert!(clean.contains("PASS") && !clean.contains("FAIL"), "a fresh scaffold must pass: {clean}");
    std::fs::write(
        root.join(".tracking").join("probe.sysml"),
        "package ProbeIds {\n    private import EngineElement::*;\n    part probeItem : Decision { :>> id = \"not-a-uuid-at-all\"; :>> title = \"probe\"; :>> createdAt = \"2026-09-11\"; :>> createdBy = \"ai\"; }\n}\n",
    )
    .expect("write probe");
    let dirty = run(&root, &["gate", "guard", "identity-well-formed", "."]);
    assert!(
        dirty.contains("FAIL") && dirty.contains("not-a-uuid-at-all") && dirty.contains("probe.sysml"),
        "the guard must FAIL naming the id and the file: {dirty}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

fn census_script(repo: &Path) -> Option<String> {
    for py in ["python3", "python"] {
        if let Ok(out) = Command::new(py).arg(repo.join(".engine/tools/guard_proof_census.py")).arg(repo).output() {
            if out.status.success() {
                return Some(String::from_utf8_lossy(&out.stdout).to_string());
            }
        }
    }
    None
}

fn count_after(text: &str, label: &str) -> usize {
    text.lines()
        .find(|l| l.contains(label))
        .and_then(|l| l.rsplit(':').next())
        .and_then(|n| n.trim().parse().ok())
        .unwrap_or_else(|| panic!("census line `{label}` missing in:\n{text}"))
}

/// The computed numbers match the census script's on the same tree, state by state, and the three
/// states are distinguishable in `keel show controls`.
#[test]
fn the_binary_agrees_with_the_census_script_on_this_tree_and_shows_three_states() {
    let repo = repo();
    let c = census_over(repo, &keel_cli::guards::GUARD_NAMES);
    let script = census_script(repo).expect("python3 or python runs .engine/tools/guard_proof_census.py - the reference the binary is held against");
    let head = script.lines().next().unwrap_or_default();
    let guards: usize = head.split("guards:").nth(1).and_then(|r| r.split_whitespace().next()).and_then(|n| n.parse().ok()).expect("guards count");
    let scanned: usize = head.rsplit(':').next().and_then(|n| n.trim().parse().ok()).expect("scanned count");
    assert_eq!(c.guards.len(), guards, "guard count: binary vs script\n{script}");
    assert_eq!(c.test_functions, scanned, "test functions scanned: binary vs script\n{script}");
    assert_eq!(c.proven(), count_after(&script, "asserts a FAILURE"), "PROVEN: binary vs script\n{script}");
    assert_eq!(c.undetermined(), count_after(&script, "named nowhere in any test body"), "UNDETERMINED: binary vs script\n{script}");
    assert_eq!(c.proven() + c.unproven(), count_after(&script, "named inside a test body"), "named: binary vs script\n{script}");
    // Name for name, not just count for count.
    for g in &c.guards {
        let listed_unnamed = script.lines().any(|l| l.trim() == g.guard);
        let listed_unproven = script.lines().any(|l| l.trim().starts_with(&format!("{} ", g.guard)) && l.contains('('));
        match g.state {
            ProofState::Undetermined => assert!(listed_unnamed, "{} UNDETERMINED here, not in the script's unnamed list\n{script}", g.guard),
            ProofState::Unproven => assert!(listed_unproven, "{} UNPROVEN here, not in the script's unproven list\n{script}", g.guard),
            ProofState::Proven => assert!(!listed_unnamed && !listed_unproven, "{} PROVEN here, listed by the script\n{script}", g.guard),
        }
    }
    // The surface: three states apart, the lists present, the share as words rather than a percentage.
    let out: String = run(repo, &["show", "controls", "."]).split_whitespace().collect();
    let section = out.split("\"controlProof\":").nth(1).expect("controlProof section in keel show controls");
    for key in ["\"proven\":", "\"unproven\":", "\"undetermined\":", "\"unprovenList\":", "\"undeterminedList\":", "\"controlArming\""] {
        assert!(out.contains(key), "missing {key}: {out}");
    }
    assert!(section.contains("\"proof\":\"PROVEN\"") && section.contains("\"proof\":\"UNDETERMINED\""), "states not distinguishable: {section}");
    assert!(!section.contains('%'), "the share is context, never a bare percentage: {section}");
}
