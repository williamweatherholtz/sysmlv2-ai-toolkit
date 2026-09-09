//! `keel reject` (D0393 / issue414): a human's REJECTION of a proposed Decision, recorded from an agent
//! session through the write API under the same channel rules as `keel accept`.
//!
//! Before this the first rejection in the project's history was a hand edit mirroring the writer's
//! output - `write::reject_decision` existed only behind the console's POST - which no guard can tell
//! from a fabricated one. Three properties, mirroring accept_honours_delegation.rs:
//!   1. a quote naming the decision -> status rejected, a Reject confirmation Test, a FAIL result judged
//!      by the human and CREATED BY the session actor (D0299);
//!   2. a bare 'no' that names no decision -> refused with the read-back message, nothing written;
//!   3. `--by` given while the session actor is unbound -> refused, nothing written.

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

/// Run as an AGENT session would: the marker set, stdin not a terminal, the session actor bound.
fn agent(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("CLAUDE_CODE_SESSION_ID", "00000000-0000-4000-8000-000000000002")
        .env("KEEL_ACTOR", "ai")
        .stdin(Stdio::null())
        .output()
        .expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

/// The same session with NO actor bound - the recorder is unknown.
fn unbound_agent(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("CLAUDE_CODE_SESSION_ID", "00000000-0000-4000-8000-000000000003")
        .env_remove("KEEL_ACTOR")
        .stdin(Stdio::null())
        .output()
        .expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn project_with_a_proposed_decision(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("rej{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(agent(&root, &["init", "."]).0, "scaffold");
    // issue376: the scaffold ships the grant commented out; this human delegates the RECORDING (D0192)
    let p = root.join(".engine/contracts/attestation-policy.toml");
    let text = std::fs::read_to_string(&p).expect("policy");
    let granted = text.replace("# delegatedRecording = \"d0192\"", "delegatedRecording = \"d0192\"");
    assert_ne!(text, granted, "the commented delegation line ships");
    std::fs::write(&p, granted).expect("grant");
    let (ok, text) = agent(
        &root,
        &["record", "decision", "--slug", "probe", "--title", "t", "--context", "c", "--decision", "d", "--rationale", "r", "--consequences", "q", "--author", "ai", "--date", "2026-09-08"],
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
fn a_quoted_rejection_naming_the_decision_is_recorded_with_judge_and_recorder_apart() {
    let root = project_with_a_proposed_decision("q");
    let (ok, text) = agent(&root, &["reject", "d0001", "--words", "no, reject d0001 - we are not doing this", "--by", "you", "--date", "2026-09-08"]);
    assert!(ok, "delegation declared + a quote naming the decision must record the rejection: {text}");
    assert!(text.contains("rejected d0001") && text.contains("delegation d0192"), "the record says what it did and under what: {text}");
    let d = decision_text(&root);
    assert!(d.contains("DecisionStatus::rejected"), "status flipped:\n{d}");
    assert!(d.contains("verification d0001Reject : Test") && d.contains("VerificationMethod::confirmation"), "a confirmation Test carries the verdict:\n{d}");
    assert!(d.contains("we are not doing this"), "the human's words travel verbatim:\n{d}");
    let result = d.lines().find(|l| l.contains("d0001RejectR1 : TestResult")).expect("a RejectR1 result");
    assert!(result.contains("VerdictKind::fail"), "a rejection is a FAIL result: {result}");
    assert!(result.contains("judgedBy = \"you\""), "judged by the human: {result}");
    assert!(result.contains("createdBy = \"ai\""), "created by the session actor - who recorded is a separate fact (D0299): {result}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_bare_no_that_names_nothing_is_refused_with_the_read_back_message() {
    let root = project_with_a_proposed_decision("bare");
    let (ok, text) = agent(&root, &["reject", "d0001", "--words", "no thanks, not that", "--by", "you", "--date", "2026-09-08"]);
    assert!(!ok, "a quote naming no decision is refused: {text}");
    assert!(text.contains("read-back") && text.contains("rejecting"), "the same read-back message accept uses, in this verb: {text}");
    assert!(!decision_text(&root).contains("DecisionStatus::rejected"), "nothing written");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_unbound_recorder_is_refused() {
    let root = project_with_a_proposed_decision("unb");
    // the session binding init may have left behind must not stand in for a bound actor
    let _ = std::fs::remove_file(root.join(".keel").join("actor"));
    let (ok, text) = unbound_agent(&root, &["reject", "d0001", "--words", "reject d0001, no", "--by", "you", "--date", "2026-09-08"]);
    assert!(!ok, "with --by naming the judge and no session actor, who is recording is unknown - refused: {text}");
    assert!(!decision_text(&root).contains("DecisionStatus::rejected"), "nothing written");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn help_and_surface_carry_the_command() {
    let root = project_with_a_proposed_decision("help");
    let (_ok, text) = agent(&root, &["--help"]);
    assert!(text.contains("reject"), "keel --help renders the fact: {text}");
    let (ok, text) = agent(&root, &["guard", "cli-surface-declared", "."]);
    assert!(ok && text.contains("0 violation(s)"), "facts, help and dispatch agree: {text}");
    let _ = std::fs::remove_dir_all(&root);
}
