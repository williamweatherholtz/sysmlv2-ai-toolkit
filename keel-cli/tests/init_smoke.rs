//! Smoke test for `keel init` (D0093 spin-up): a fresh scaffold must be a WORKING project — it
//! validates clean, passes every guard, orients, and refuses to overwrite itself. Drives the REAL
//! `keel` binary end-to-end (via `CARGO_BIN_EXE_keel`), so it exercises the embedded scaffold + the
//! engine/instance remap exactly as a newcomer would. This is the cold-start regression guard the
//! console-arc retros flagged as missing (initSmokeTest).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static DIR_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_dir() -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    std::env::temp_dir().join(format!("keel_init_smoke_{pid}_{n}"))
}

fn keel() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keel"))
}

struct TmpProject(PathBuf);
impl Drop for TmpProject {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Collect file paths under `dir` (recursively) matching `pred`.
fn walk_paths(dir: &Path, pred: &dyn Fn(&Path) -> bool) -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                out.extend(walk_paths(&p, pred));
            } else if pred(&p) {
                out.push(p.to_string_lossy().to_string());
            }
        }
    }
    out
}

#[test]
fn init_scaffolds_a_working_project() {
    let dir = unique_dir();
    let _cleanup = TmpProject(dir.clone());
    let proj = dir.to_str().unwrap();

    // 1. init succeeds and lays down the scaffold.
    let out = keel().args(["init", proj]).output().expect("run keel init");
    assert!(out.status.success(), "init failed: {}", String::from_utf8_lossy(&out.stderr));
    assert!(dir.join(".engine").is_dir(), ".engine/ not scaffolded");
    assert!(dir.join("CLAUDE.md").is_file(), "CLAUDE.md not written");
    assert!(dir.join(".tracking").is_dir(), ".tracking/ not created");
    // engine/instance boundary (D0093): architecture decisions ship read-only under reference/, and
    // the new project's OWN decisions dir is created fresh + empty.
    assert!(dir.join(".engine").join("reference").join("decisions").is_dir(), "reference/decisions/ missing");
    assert!(dir.join(".engine").join("decisions").is_dir(), "fresh decisions/ missing");
    // scaffoldEngineDevExclude: the engine-DEV-only kernel/Python toolchain must NOT ship downstream —
    // EXCEPT the two tools the portable obligation-review process deploys BY PATH (D0171): guard 39
    // (tool-reference) fails a scaffold whose process references tools it never received, which is
    // exactly what CI caught on the guard's first landing. Exactly those two, nothing else.
    let py: Vec<String> = walk_paths(&dir.join(".engine"), &|p| p.extension().is_some_and(|e| e == "py" || e == "pyc"));
    let mut names: Vec<&str> = py.iter().map(|p| p.rsplit(['/', '\\']).next().unwrap_or(p)).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        vec!["deck_inbox_record.py", "test_deck_e2e.py"],
        "scaffolded python must be exactly the portable deck tools; got {names:?}"
    );
    // D0174/P0: the in-loop enforcement surface ships with init — five hook events, the output
    // style, one skill per registry entry (counts asserted equal), the declared adoption profile,
    // and the optional CI template. Behavioral parity (K3) is what downstream gets.
    let settings: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join(".claude").join("settings.json")).expect(".claude/settings.json not scaffolded"),
    )
    .expect("settings.json must be valid JSON");
    for ev in ["UserPromptSubmit", "PostToolUse", "Stop", "PreToolUse", "SubagentStop"] {
        assert!(settings.pointer(&format!("/hooks/{ev}")).is_some(), "scaffolded settings missing hook event {ev}");
    }
    assert_eq!(settings["outputStyle"], "keel", "the keel output style is the response contract (D0130)");
    assert!(
        !settings.to_string().contains("./target/"),
        "cwd-relative binary probe is the P0.3 forbidden pattern"
    );
    assert!(dir.join(".claude").join("output-styles").join("keel.md").is_file(), "output style not scaffolded");
    let skill_dirs = std::fs::read_dir(dir.join(".claude").join("skills")).expect(".claude/skills missing").count();
    // D0222: a skill declares itself BESIDE itself now, so the registry is every `.sysml` under
    // `.engine/skills/` rather than the central file alone. This was the FIFTH reader of that one
    // fact and the last to be found - the four before it were fixed as each broke, which is why the
    // D0067 migration process should require enumerating a shared fact's readers in the dry run.
    let mut registry = String::new();
    for e in walkdir(&dir.join(".engine").join("skills")) {
        if e.extension().and_then(|x| x.to_str()) == Some("sysml") {
            registry.push_str(&std::fs::read_to_string(&e).unwrap_or_default());
        }
    }
    assert_eq!(skill_dirs, registry.matches(":>> location = ").count(), "skills scaffolded != registry count (P0.1)");
    let profile = std::fs::read_to_string(dir.join(".engine").join("contracts").join("adoption-profile.toml"))
        .expect("adoption profile fact not recorded");
    assert!(profile.contains("profile = \"strict\""), "empty-dir default is strict, DECLARED (P0.4)");
    assert!(dir.join(".github").join("workflows").join("keel-gate.yml").is_file(), "CI template not scaffolded");

    // scaffoldCommitGate: a Rust-only pre-commit gate is scaffolded (keel validate/guard; NO conda/kernel).
    let hook = std::fs::read_to_string(dir.join(".githooks").join("pre-commit")).expect(".githooks/pre-commit not scaffolded");
    assert!(hook.contains("keel") && hook.contains("validate") && hook.contains("guard"), "pre-commit gate missing keel validate/guard");
    assert!(!hook.contains("conda") && !hook.contains("python") && !hook.contains(".py"), "pre-commit gate must be kernel-free (no conda/python)");
    // introductionDryRun: a starter actor registry must ship so the newcomer's first recorded fact passes the actors guard.
    assert!(dir.join(".tracking").join("actors.sysml").is_file(), ".tracking/actors.sysml not scaffolded (newcomer can't record facts)");
    // D0097: the declared critique-policy ships so a downstream project can tune its critique bar (and `keel critique-policy` reads a file, not a built-in fallback).
    let policy = std::fs::read_to_string(dir.join(".engine").join("contracts").join("critique-policy.toml")).expect(".engine/contracts/critique-policy.toml not scaffolded");
    assert!(policy.contains("[lenses]") && policy.contains("Need"), "critique-policy.toml missing the [lenses] default");

    // 2. the fresh scaffold validates clean.
    let out = keel().args(["validate", proj]).output().expect("run keel validate");
    assert!(out.status.success(), "fresh scaffold failed validate: {}", String::from_utf8_lossy(&out.stdout));

    // 3. the fresh scaffold passes EVERY guard (the D0093 promise: spin up green).
    let out = keel().args(["guard", "all", proj]).output().expect("run keel guard");
    assert!(out.status.success(), "fresh scaffold failed guard: {}", String::from_utf8_lossy(&out.stdout));

    // 4. it orients (computable state, no crash).
    let out = keel().args(["orient", proj]).output().expect("run keel orient");
    assert!(out.status.success(), "fresh scaffold failed orient");
    assert!(String::from_utf8_lossy(&out.stdout).contains("\"ready\""), "orient output missing ready[]");

    // 5. re-init refuses to overwrite (exit 2, non-success) — never clobbers existing work.
    let out = keel().args(["init", proj]).output().expect("run keel init again");
    assert!(!out.status.success(), "re-init should refuse to overwrite an existing .engine/");

    // 6. issue291/issue292: the scaffold survives the project RECORDING A FACT.
    //
    // Steps 1-5 assert init-then-read, which is the one state in which an entire defect class is
    // invisible: a collision between the shipped scaffold and a fact the project authors cannot
    // appear until a fact is authored. It shipped, and a field project hit it on its FIRST recorded
    // decision — `.engine/decisions/` starts empty, `next_decision_number` scans only that directory,
    // so the project allocated `package Decision0001`/`part d0001` against the 236 reference decisions
    // remapped in by init. `validate` and `check-engine` both reported clean; only duplicate-identity
    // caught it, naming the read-only reference file as the offender.
    //
    // Sprint 104's own retro NAMED this control ("an init smoke-test in CI that scaffolds + guards a
    // temp project") and declined to track it, judging the risk low. That is the D0047 failure mode
    // exactly: a lesson logged instead of a control built. So the control is the recording itself —
    // not an assertion about decision numbering, which would only catch the one symptom.
    let out = keel()
        .args([
            "record", "decision", "--root", proj,
            "--slug", "how-we-will-track-work",
            "--title", "how this project will track its work",
            "--date", "2026-01-02",
            "--context", "A new project has to choose where its record of decisions lives.",
            "--decision", "Decisions are authored as text files in this repository.",
            "--rationale", "Text diffs, reviews, and survives a change of tooling.",
            "--consequences", "Every decision is a file, and the gate reads it.",
        ])
        .env("KEEL_ACTOR", "smokeTestActor")
        .output()
        .expect("run keel record decision");
    assert!(
        out.status.success(),
        "a fresh project could not record its first decision: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // The gate must still be green with that fact in the tree — this is the assertion that fails on
    // the issue291 shape, and would fail on any future scaffold artifact that collides with an
    // authored one.
    let out = keel().args(["guard", "all", proj]).output().expect("run keel guard after recording");
    assert!(
        out.status.success(),
        "the gate went red once the fresh project recorded a decision: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    let out = keel().args(["validate", proj]).output().expect("run keel validate after recording");
    assert!(
        out.status.success(),
        "validate went red once the fresh project recorded a decision: {}",
        String::from_utf8_lossy(&out.stdout)
    );
}

/// Every file under `dir`, recursively. The registry is a DIRECTORY since D0222, so the count this
/// test asserts has to walk it rather than read one path.
fn walkdir(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walkdir(&p));
        } else {
            out.push(p);
        }
    }
    out
}
