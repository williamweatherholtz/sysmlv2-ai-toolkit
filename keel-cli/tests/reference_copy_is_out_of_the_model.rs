//! issue292: the engine's reference decisions travel with `keel init` (`.engine/reference/decisions/`,
//! 236 files carrying keel's own acceptances and evidence SHAs) and must be OUT of the downstream
//! project's model: no foreign attestation in its verification surface, no evidence SHA that cannot
//! resolve in its repository - while the `#JustifiedBy` edges from the shipped rules to those decisions
//! still resolve.
//!
//! WHY THIS TEST RECORDS A FACT FIRST. Both earlier gates over init asserted init-then-validate and
//! never recorded anything - the one state in which this whole class is invisible, because an empty
//! project's views are empty whatever they scan. Here a task and a passing result are recorded in the
//! fixture, so the model has exactly one result of its own and any foreign one would stand beside it.
//!
//! The reference copy is kept out by `view::model_dirs` (D0093) and resolved for edge purposes by the
//! edge-endpoints guard's `declared_elsewhere`; this test is what holds both, since issue292's
//! diagnosis of the mechanism was wrong and nothing measured the property until now.

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

fn keel(root: &Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(keel_bin()).args(args).current_dir(root).env("KEEL_ACTOR", "claudeOpus5").env("KEEL_OFFLINE", "1").output().expect("keel runs");
    (out.status.code().unwrap_or(-1), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn a_downstream_model_holds_no_foreign_attestation_after_a_fact_is_recorded() {
    // A SHORT path: the reference decisions have long file names and Windows git refuses paths past
    // 260 chars, which is how the first live probe of this property failed before it started.
    let root = std::env::temp_dir().join(format!("kref{}", std::process::id() % 100_000));
    let _ = std::fs::remove_dir_all(&root);
    let (code, out) = keel(Path::new("."), &["init", &root.to_string_lossy()]);
    assert_eq!(code, 0, "scaffold failed: {out}");
    let reference = std::fs::read_dir(root.join(".engine").join("reference").join("decisions")).map(|rd| rd.count()).unwrap_or(0);
    assert!(reference > 100, "the fixture must actually CARRY the reference copy, else this test proves nothing: {reference} files");

    git(&root, &["init", "-q", "."]);
    git(&root, &["config", "core.longpaths", "true"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=probe@keel", "-c", "user.name=probe", "commit", "-q", "-m", "seed"]);
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");

    // RECORD A FACT - the step every previous gate skipped.
    let backlog = root.join(".tracking").join("backlog.sysml");
    std::fs::write(
        &backlog,
        "package ProbeBacklog {\n    private import EngineElement::*;\n    private import EngineWork::*;\n    private import EngineVerification::*;\n    private import EngineRelationships::*;\n\n    action def ProbeBuild {\n    }\n}\n",
    )
    .expect("backlog");
    let (code, out) = keel(&root, &["add-task", "--file", ".tracking/backlog.sysml", "--def", "ProbeBuild", "--task", "dcProbe", "--method", "test", "--dod", "a probe fact so the model holds one result of its own"]);
    assert_eq!(code, 0, "add-task: {out}");
    let head = String::from_utf8_lossy(&Command::new("git").arg("-C").arg(&root).args(["rev-parse", "--short", "HEAD"]).output().expect("git").stdout).trim().to_string();
    let (code, out) = keel(&root, &["append-result", "--file", ".tracking/backlog.sysml", "--task", "dcProbe", "--sha", &head, "--verdict", "pass", "--judged-by", "claudeOpus5", "--judged-at", "2026-09-04", "--evidence", "probe"]);
    assert_eq!(code, 0, "append-result: {out}");

    // THE MODEL'S ATTESTATIONS: exactly the one recorded here, and none judged by keel's own human.
    let (_, census) = keel(&root, &["attestation", "."]);
    assert!(!census.contains("wweatherholtz"), "keel's own acceptances leaked into the downstream verification surface:\n{census}");
    let judge_rows: Vec<&str> = census.lines().filter(|l| l.trim_start().starts_with("unregistered") || l.trim_start().starts_with("claudeOpus5")).collect();
    assert_eq!(judge_rows.len(), 1, "one judge row, the probe's own:\n{census}");
    assert!(judge_rows[0].split_whitespace().nth(1) == Some("1"), "that judge has exactly one result:\n{census}");

    // THE MODEL'S EVIDENCE: every SHA resolves in THIS repository.
    let (_, orient) = keel(&root, &["orient", "."]);
    assert!(orient.contains("\"invalidEvidence\": []"), "an evidence SHA that does not resolve here means a foreign result is in the model:\n{orient}");

    // THE RULES' EDGES: the shipped `#JustifiedBy` edges to reference decisions still resolve.
    let (code, out) = keel(&root, &["check-engine", "."]);
    assert_eq!(code, 0, "check-engine must stay clean with the reference copy out of the model: {out}");
    let (_, edges) = keel(&root, &["guard", "edge-endpoints", "."]);
    assert!(edges.contains("0 violation"), "the JustifiedBy edges to reference decisions must resolve: {edges}");
    let _ = std::fs::remove_dir_all(&root);
}
