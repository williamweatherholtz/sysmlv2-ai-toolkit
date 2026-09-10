//! `keel accept` in an agent session honours the DECLARED recording delegation (D0289 on D0192).
//!
//! attestation-policy.toml has delegated the RECORDING of a human's acceptance to the agent since
//! 2026-08-22 (D0192 option A: the record must quote the human's words verbatim). The command's channel
//! layer nevertheless refused every agent session outright, so the human had to go to a terminal to
//! type acceptances the policy already allowed. Three properties:
//!   1. delegation declared + the note quotes the human  -> the acceptance is recorded - and since D0423
//!      words that do not read the decision back, or are shorter than ten characters, are recorded too,
//!      with a WARN line in the note naming the check (the human's 'A' was refused for its length, issue445);
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

/// issue376: a fresh scaffold ships the policy with its GRANT lines commented out. This fixture's human
/// delegates the RECORDING of their acceptance (D0192) - the grant the tests below exercise.
fn grant_recording_delegation(root: &Path) {
    let p = root.join(".engine/contracts/attestation-policy.toml");
    let text = std::fs::read_to_string(&p).expect("policy");
    let granted = text.replace("# delegatedRecording = \"d0192\"", "delegatedRecording = \"d0192\"");
    assert_ne!(text, granted, "the commented delegation line ships");
    std::fs::write(&p, granted).expect("grant");
}

fn project_with_a_proposed_decision(tag: &str) -> PathBuf {
    let root = shallow_root(tag);
    assert!(agent(&root, &["init", "."]).0, "scaffold");
    grant_recording_delegation(&root);
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
    // The human's words naming the decision: recorded under the delegation, carrying the quote and no WARN.
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "their words in chat: 'yes, accept d0001 and keep going'", "--by", "you", "--date", "2026-09-03"]);
    assert!(ok, "delegation declared + a quote naming the decision must record: {text}");
    assert!(text.contains("delegation d0192"), "the record cites the delegation it acts under: {text}");
    let d = decision_text(&root);
    assert!(d.contains("DecisionStatus::accepted") && d.contains("yes, accept d0001 and keep going"), "the acceptance event carries the quote:\n{d}");
    assert!(!d.contains("WARN:"), "words that read the decision back carry no WARN:\n{d}");
    let _ = std::fs::remove_dir_all(&root);
}

/// D0423 / issue445: the read-back and ten-character checks are computed and WRITTEN, never refused.
#[test]
fn words_that_name_nothing_or_are_short_record_with_a_warn_line() {
    let root = project_with_a_proposed_decision("w");
    // a quote naming no decision: recorded, the note says the read-back did not hold
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "their words in chat: 'yes, accept it and keep going'", "--by", "you", "--date", "2026-09-10"]);
    assert!(ok, "a quote naming no decision is recorded under D0423: {text}");
    assert!(text.contains("WARN") && text.contains("read-back"), "the command says which check would have refused: {text}");
    let d = decision_text(&root);
    assert!(d.contains("AcceptR1") && d.contains("WARN: read-back") && d.contains("D0423"), "the record carries the WARN line naming the check:\n{d}");

    // the human's one-letter answer to a fork: recorded as given, the note says it was short
    let root2 = project_with_a_proposed_decision("s");
    let (ok, text) = agent(&root2, &["accept", "d0001", "--words", "A", "--by", "you", "--date", "2026-09-10"]);
    assert!(ok, "'A' is recorded as given (issue445): {text}");
    assert!(text.contains("WARN") && text.contains("ten characters"), "{text}");
    let d = decision_text(&root2);
    assert!(d.contains("\u{201C}A\u{201D}") && d.contains("WARN: short words"), "the declared pair holds the letter and the record says it was short:\n{d}");
    // and the recorded tree passes the delegated-substance rule: a declared pair is exact at any length
    let (ok, text) = agent(&root2, &["guard", "confirmation-authenticity", "."]);
    assert!(ok, "the short declared quote satisfies the substance rule: {text}");
    let _ = std::fs::remove_dir_all(&root);
    let _ = std::fs::remove_dir_all(&root2);
}

#[test]
fn an_unquoted_note_is_refused_and_told_to_quote() {
    let root = project_with_a_proposed_decision("nq");
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "the human agreed", "--by", "you", "--date", "2026-09-03"]);
    assert!(!ok, "no quote, no record: {text}");
    assert!(text.contains("QUOTE"), "the refusal names the missing receipt: {text}");
    assert!(!decision_text(&root).contains("DecisionStatus::accepted"), "nothing was written");
    // issue445: the refusal itself is a ledger fact - the question "has an agent ever tried this"
    // used to have no record to be read from.
    let ledger = std::fs::read_to_string(root.join(".keel").join("metrics").join("hooks.jsonl")).expect("a refused write leaves a ledger line");
    let refused: Vec<&str> = ledger.lines().filter(|l| l.contains(r#""event":"refused""#)).collect();
    assert_eq!(refused.len(), 1, "one refusal, one line: {ledger}");
    assert!(refused[0].contains(r#""control":"accept:no-quote""#) && refused[0].contains(r#""actorKind":"#), "the line names the check and the actor kind: {}", refused[0]);
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
