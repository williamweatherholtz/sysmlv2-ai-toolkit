//! D0359 (dcStopSaysNothingAboutDecisions): on a GREEN tree the Stop hook is silent - whether or not
//! Decisions are waiting on the human.
//!
//! For one day it emitted one line naming them (GH#52/D0351): the count, the id range, how to accept,
//! the short names. It was accurate every time, which is why it stopped being read. The owner's
//! instruction, 2026-09-06: "I don't want stop says text anymore. remove." The queue is now surfaced
//! as the published decision brief by the decision-surfacing post-analysis, and `keel show
//! authority-queue` remains the machine-readable answer - so this test pins the SILENCE, in the
//! presence of pending Decisions, which is the state the removed line existed to talk about.
//!
//! Run on the binary against a real scaffold, which grants no standing consent, so recorded
//! Decisions stay proposed.

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
    let root = base.join(format!("sn{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    root
}

fn record(root: &Path, slug: &str, title: &str) {
    // substantive context and rationale, so the scaffold stays GREEN (decision-rationale, D0103)
    let (ok, out) = run(root, &["record", "decision", "--slug", slug, "--title", title, "--context", "the fixture needs a proposed decision that waits on the human", "--decision", "one plain decision the human has not yet accepted", "--rationale", "a proposed decision is what the queue carries, and this one is proposed because the scaffold grants no consent", "--consequences", "the published page carries it; the turn boundary does not", "--author", "ai", "--date", "2026-09-06"]);
    assert!(ok && out.contains("(proposed)"), "{out}");
}

#[test]
fn with_decisions_outstanding_the_green_turn_boundary_is_still_silent() {
    let root = scaffold("pending");
    record(&root, "one", "probeOne: the first thing that waits on the human");
    record(&root, "two", "probeTwo: the second thing that waits on the human");
    let (code, out) = stop_hook(&root);
    assert_eq!(code, 0, "a green tree allows the stop: {out}");
    assert!(
        out.trim().is_empty(),
        "green with TWO decisions pending: the hook says nothing about them (D0359). Got: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn what_waits_is_still_computable_from_the_lens() {
    // The removal is only honest while the answer remains available to anyone who asks for it: the
    // line went, the FACT did not. This is what the post-analysis reads.
    let root = scaffold("lens");
    record(&root, "one", "probeOne: the first thing that waits on the human");
    let (ok, out) = run(&root, &["show", "authority-queue", "."]);
    assert!(ok, "the lens computes: {out}");
    let v: serde_json::Value = serde_json::from_str(&out).expect("the lens speaks JSON");
    let awaiting = v["awaiting"].as_array().expect("awaiting rows");
    let row = awaiting.iter().find(|r| r["item"] == "d0001").expect("the proposed decision is in the queue");
    assert_eq!(row["kind"], "decisionAcceptance");
    assert_eq!(row["shortName"], "probeOne", "the row names what it is ABOUT, so a page can be built from this lens alone");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn with_nothing_outstanding_the_green_turn_boundary_is_silent() {
    let root = scaffold("silent");
    let (code, out) = stop_hook(&root);
    assert_eq!(code, 0);
    assert!(out.trim().is_empty(), "green and nothing waiting: silence: {out}");
    let _ = std::fs::remove_dir_all(&root);
}
