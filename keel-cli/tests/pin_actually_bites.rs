//! Gherkin for `dcPinBites` (sprint 490, D0251 clauses A and C) — WRITTEN BEFORE the implementation.
//!
//! srProjectPinsItsEngine: "A gate invoked in a project shall run the engine version that project
//! declares; where the resolved binary's version differs from the declaration, the invocation shall
//! refuse and name both versions rather than proceeding with an advisory warning." Today the
//! declaration drives a warning and pins nothing — these scenarios fail against today's binary,
//! which is what makes them worth writing first.
//!
//! The refusal surface is D0251 clause C's split: WRITES and GATES refuse; read-only views WARN and
//! proceed; `version` and `migrate` never refuse, because they are how a skew is inspected and
//! repaired. An ABSENT declaration stays warn-tier (D0136: absence is a state — a pre-D0190 tree
//! keeps working).

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

fn binary_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// A keel-shaped project declaring `declared` as its engine pin (None = no declaration).
fn project(tag: &str, declared: Option<&str>) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-pin-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking").join("intake")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    // `keel validate` registers the schema from the PROJECT's disk, so the fixture carries the real
    // one — copied from this workspace, the same schema `keel init` would lay down.
    let schema_src = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("ws").join(".engine").join("schema");
    copy_tree(&schema_src, &root.join(".engine").join("schema"));
    if let Some(v) = declared {
        std::fs::write(
            root.join(".engine").join("contracts").join("engine-version.toml"),
            format!("engine = \"{v}\"\n"),
        )
        .expect("pin");
    }
    root
}

fn copy_tree(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).expect("mkdir dst");
    for e in std::fs::read_dir(src).expect("read src").flatten() {
        let p = e.path();
        let d = dst.join(e.file_name());
        if p.is_dir() {
            copy_tree(&p, &d);
        } else {
            std::fs::copy(&p, &d).expect("copy schema file");
        }
    }
}

fn keel(root: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin()).args(args).current_dir(root).output().expect("keel runs");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

fn write_args<'a>(root: &'a Path, text_file: &'a str) -> Vec<String> {
    vec![
        "record".into(), "statement".into(),
        "--from".into(), root.join(text_file).to_string_lossy().to_string(),
        "--said-by".into(), "wweatherholtz".into(),
        "--said-at".into(), "2026-08-29".into(),
        "--title".into(), "pin probe".into(),
        "--by".into(), "claudeOpus5".into(),
        "--at".into(), "2026-08-29".into(),
        "--root".into(), root.to_string_lossy().to_string(),
    ]
}

// ── Scenario 0 (the ALLOW case, first — a pin that refuses everything is stuck, not armed) ────────

#[test]
fn a_matching_pin_allows_writes_and_gates() {
    let root = project("match", Some(binary_version()));
    std::fs::write(root.join("say.txt"), "matching pin\n").expect("say");
    let args = write_args(&root, "say.txt");
    let (ok, text) = keel(&root, &args.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(ok, "a MATCHING pin must allow the write: {text}");
    let (ok, text) = keel(&root, &["validate", "."]);
    assert!(ok, "a matching pin must allow the gate: {text}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── Scenario 1: a WRITE under skew refuses, naming both versions ─────────────────────────────────

#[test]
fn a_write_under_skew_refuses_naming_both_versions() {
    let root = project("skew-write", Some("0.0.1"));
    std::fs::write(root.join("say.txt"), "skewed write\n").expect("say");
    let args = write_args(&root, "say.txt");
    let (ok, text) = keel(&root, &args.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(!ok, "a write under a skewed pin must REFUSE (srProjectPinsItsEngine): {text}");
    assert!(
        text.contains("0.0.1") && text.contains(binary_version()),
        "the refusal must name BOTH versions so the operator can act: {text}"
    );
    assert!(
        text.contains("migrate"),
        "the refusal must name the repair path (keel migrate): {text}"
    );
    // And nothing was written: a refused write leaves no trace.
    assert!(
        !root.join(".tracking").join("intake").join("intake-2026-08-29.sysml").exists(),
        "a refused write must write NOTHING"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── Scenario 2: a GATE under skew refuses ─────────────────────────────────────────────────────────

#[test]
fn a_gate_under_skew_refuses() {
    let root = project("skew-gate", Some("0.0.1"));
    for cmd in [&["validate", "."][..], &["guard"][..], &["gate", "--fast", "."][..]] {
        let (ok, text) = keel(&root, cmd);
        assert!(
            !ok,
            "gate command {cmd:?} under a skewed pin must REFUSE — a verdict from an undeclared engine is the thing being pinned: {text}"
        );
        assert!(text.contains("0.0.1"), "{cmd:?} refusal names the declared pin: {text}");
    }
    let _ = std::fs::remove_dir_all(&root);
}

// ── Scenario 3: a READ view under skew warns and proceeds ─────────────────────────────────────────

#[test]
fn a_read_view_under_skew_warns_and_proceeds() {
    let root = project("skew-read", Some("0.0.1"));
    let (ok, text) = keel(&root, &["orient", "."]);
    assert!(ok, "orient must PROCEED under skew — diagnosing a skew must not require it gone: {text}");
    assert!(
        text.contains("SKEW") || text.contains("skew"),
        "orient must WARN loudly under skew: {text}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── Scenario 4: version and migrate never refuse — they are the repair path ──────────────────────

#[test]
fn version_and_migrate_never_refuse() {
    let root = project("skew-repair", Some("0.0.1"));
    let (ok, _) = keel(&root, &["version"]);
    assert!(ok, "version must never refuse — it is how the skew is inspected");
    // migrate refuses OUTSIDE git for its own reasons (no way back from a bad run), so the
    // repair-path scenario gives it what it demands rather than testing two refusals at once.
    for args in [&["init", "-q"][..], &["config", "user.email", "p@example.invalid"], &["config", "user.name", "p"], &["add", "-A"], &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]] {
        let out = Command::new("git").arg("-C").arg(&root).args(args).output().expect("git");
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    }
    let (ok, text) = keel(&root, &["migrate", ".", "--dry-run"]);
    assert!(ok, "migrate must never refuse — it is how the skew is repaired: {text}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── Scenario 5: an ABSENT declaration stays warn-tier (absence is a state, D0136) ─────────────────

#[test]
fn an_absent_declaration_keeps_working() {
    let root = project("undeclared", None);
    std::fs::write(root.join("say.txt"), "undeclared tree\n").expect("say");
    let args = write_args(&root, "say.txt");
    let (ok, text) = keel(&root, &args.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(ok, "a pre-D0190 tree with NO declaration must keep working: {text}");
    let (ok, _) = keel(&root, &["validate", "."]);
    assert!(ok, "and its gates must keep running");
    let _ = std::fs::remove_dir_all(&root);
}
