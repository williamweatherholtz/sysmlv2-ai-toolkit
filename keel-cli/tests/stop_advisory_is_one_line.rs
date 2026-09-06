//! GH#52 / D0351 (dcStopAdvisoryIsOneLine): the Stop hook's exit-0 advisory is ONE line in the owner's
//! format - the count of decisions outstanding, their id range, how to accept, and their short names -
//! and on a project with nothing outstanding the green turn boundary is silent. Run on the binary
//! against a real scaffold, which grants no standing consent, so recorded Decisions stay proposed.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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

fn stop_hook(dir: &Path) -> (i32, String) {
    let mut child = Command::new(keel_bin())
        .args(["hook", "stop"])
        .current_dir(dir)
        .env("KEEL_ACTOR", "ai")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");
    child.stdin.take().expect("stdin").write_all(br#"{"session_id":"probe","stop_hook_active":false}"#).expect("payload");
    let out = child.wait_with_output().expect("wait");
    (out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stdout).to_string())
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("sa{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    root
}

fn record(root: &Path, slug: &str, title: &str) {
    // substantive context and rationale, so the scaffold stays GREEN (decision-rationale, D0103)
    let (ok, out) = run(root, &["record", "decision", "--slug", slug, "--title", title, "--context", "the fixture needs a proposed decision that waits on the human", "--decision", "one plain decision the human has not yet accepted", "--rationale", "a proposed decision is what the advisory counts, and this one is proposed because the scaffold grants no consent", "--consequences", "the advisory names it", "--author", "ai", "--date", "2026-09-06"]);
    assert!(ok && out.contains("(proposed)"), "{out}");
}

#[test]
fn with_decisions_outstanding_the_green_advisory_is_one_line_naming_them() {
    let root = scaffold("named");
    record(&root, "one", "probeOne: the first thing that waits on the human");
    record(&root, "two", "probeTwo: the second thing that waits on the human");
    let (code, out) = stop_hook(&root);
    assert_eq!(code, 0, "a green tree allows the stop: {out}");
    let lines: Vec<&str> = out.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 1, "exactly one line: {out}");
    let v: serde_json::Value = serde_json::from_str(lines[0]).expect("the hook speaks JSON");
    let msg = v["systemMessage"].as_str().expect("systemMessage");
    assert!(msg.starts_with("2 decisions outstanding (d0001..d0002)"), "count and range first: {msg}");
    assert!(msg.contains("keel accept dNNNN") && msg.contains("quoted word"), "how to accept: {msg}");
    assert!(msg.ends_with("Covering: probeOne, probeTwo"), "the short names, whole: {msg}");
    assert!(!msg.contains("oversight") && !msg.contains("127.0.0.1") && !msg.contains("deck"), "the console/bridge nag is gone: {msg}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn with_nothing_outstanding_the_green_turn_boundary_is_silent() {
    let root = scaffold("silent");
    let (code, out) = stop_hook(&root);
    assert_eq!(code, 0);
    assert!(out.trim().is_empty(), "green and nothing waiting: silence, not a nag: {out}");
    let _ = std::fs::remove_dir_all(&root);
}
