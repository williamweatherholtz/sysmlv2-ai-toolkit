//! `keel accept` in an agent session honours the DECLARED recording delegation (D0289 on D0192).
//!
//! attestation-policy.toml has delegated the RECORDING of a human's acceptance to the agent since
//! 2026-08-22 (D0192 option A: the record must quote the human's words verbatim). The command's channel
//! layer nevertheless refused every agent session outright, so the human had to go to a terminal to
//! type acceptances the policy already allowed. Three properties:
//!   1. delegation declared + the note quotes the human  -> the acceptance is recorded;
//!   2. delegation declared + no quote                    -> refused, and the refusal says QUOTE;
//!   3. delegation withdrawn (line deleted)               -> refused as before, quote or not.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

/// Run as an AGENT session would: the marker set, stdin not a terminal.
fn agent(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("CLAUDE_CODE_SESSION_ID", "00000000-0000-4000-8000-000000000001")
        .env("KEEL_ACTOR", "ai")
        .stdin(Stdio::null())
        .output()
        .expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn shallow_root(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("acc{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    root
}

fn project_with_a_proposed_decision(tag: &str) -> PathBuf {
    let root = shallow_root(tag);
    assert!(agent(&root, &["init", "."]).0, "scaffold");
    let (ok, text) = agent(
        &root,
        &["record", "decision", "--slug", "probe", "--title", "t", "--context", "c", "--decision", "d", "--rationale", "r", "--consequences", "q", "--author", "ai", "--date", "2026-09-03"],
    );
    assert!(ok, "a proposed decision exists: {text}");
    root
}

fn decision_text(root: &Path) -> String {
    let dir = root.join(".engine").join("decisions");
    let f = std::fs::read_dir(&dir).expect("decisions").flatten().map(|e| e.path()).find(|p| p.to_string_lossy().contains("probe")).expect("the probe decision file");
    std::fs::read_to_string(f).expect("read")
}

#[test]
fn a_quoted_note_records_the_acceptance_under_the_declared_delegation() {
    let root = project_with_a_proposed_decision("q");
    // D0335 (D0201 B read-back): a quote that names NO decision is a generic yes an agent could attach
    // to anything - refused, nothing written, the remedy named.
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "their words in chat: 'yes, accept it and keep going'", "--by", "you", "--date", "2026-09-03"]);
    assert!(!ok, "a quote naming no decision is refused under read-back ratification: {text}");
    assert!(text.contains("read-back") && text.contains("d0001"), "the refusal names the remedy: {text}");
    assert!(!decision_text(&root).contains("AcceptR1"), "nothing written");
    // The human's words naming the decision: recorded under the delegation, carrying the quote.
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "their words in chat: 'yes, accept d0001 and keep going'", "--by", "you", "--date", "2026-09-03"]);
    assert!(ok, "delegation declared + a quote naming the decision must record: {text}");
    assert!(text.contains("delegation d0192"), "the record cites the delegation it acts under: {text}");
    let d = decision_text(&root);
    assert!(d.contains("DecisionStatus::accepted") && d.contains("yes, accept d0001 and keep going"), "the acceptance event carries the quote:\n{d}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_unquoted_note_is_refused_and_told_to_quote() {
    let root = project_with_a_proposed_decision("nq");
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "the human agreed", "--by", "you", "--date", "2026-09-03"]);
    assert!(!ok, "no quote, no record: {text}");
    assert!(text.contains("QUOTE"), "the refusal names the missing receipt: {text}");
    assert!(!decision_text(&root).contains("DecisionStatus::accepted"), "nothing was written");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn withdrawing_the_delegation_restores_the_refusal() {
    let root = project_with_a_proposed_decision("wd");
    let policy = root.join(".engine").join("contracts").join("attestation-policy.toml");
    let text = std::fs::read_to_string(&policy).expect("policy");
    let withdrawn: String = text.lines().filter(|l| !l.trim_start().starts_with("delegatedRecording = \"d0192\"")).map(|l| format!("{l}\n")).collect();
    assert_ne!(text, withdrawn, "the fixture ships the delegation line to withdraw");
    std::fs::write(&policy, withdrawn).expect("write");
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "their words: 'yes, accept it and keep going'", "--by", "you", "--date", "2026-09-03"]);
    assert!(!ok, "with the delegation withdrawn even a quoted note is refused: {text}");
    assert!(text.contains("no recording delegation"), "and the refusal says why: {text}");
    let _ = std::fs::remove_dir_all(&root);
}
