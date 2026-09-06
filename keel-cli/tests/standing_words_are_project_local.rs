//! issue376 / GH#57: the words a standing-consent acceptance quotes are the ADOPTING project's own
//! data. The note used to be built from a string literal of this repository's human's words while
//! `keel init` shipped `standingConsent = "d0207"` verbatim - so a fresh project with one declared
//! decider auto-accepted every non-fork Decision in that decider's name, quoting consent they never
//! gave. Three arms on the binary: a fresh scaffold carries NO live grant, consent declared without
//! words stays proposed and says so, consent with words quotes exactly them.

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
    let out = Command::new(keel_bin()).args(args).current_dir(dir).env("KEEL_ACTOR", "ai").output().expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("sw{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    std::fs::write(root.join(".engine/contracts/github-actors.toml"), "[logins]\nowner = \"you\"\n").expect("deciders");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

fn policy_path(root: &Path) -> PathBuf {
    root.join(".engine/contracts/attestation-policy.toml")
}

fn live_grant_lines(text: &str) -> Vec<String> {
    text.lines()
        .filter(|l| {
            let key = l.trim_start().split('=').next().map(str::trim).unwrap_or_default();
            ["delegatedRecording", "standingConsent", "standingWords"].contains(&key)
        })
        .map(str::to_owned)
        .collect()
}

fn draft(root: &Path) -> PathBuf {
    let f = root.join("draft.md");
    let rationale = "r ".repeat(120);
    std::fs::write(&f, format!("slug: probe\ndate: 2026-09-06\n--- title\nprobe: a short title before the colon\n--- context\nc\n--- decision\nAdopt the merge for the extras file.\n--- rationale\n{rationale}\n--- consequences\nq\n")).expect("draft");
    f
}

/// A fresh project inherits the POLICY (kinds, roles, thresholds) and none of the GRANTS.
#[test]
fn a_fresh_scaffold_carries_the_policy_and_no_live_grant() {
    let root = scaffold("fresh");
    let text = std::fs::read_to_string(policy_path(&root)).expect("policy ships");
    assert!(text.contains("[decisionAcceptance]") && text.contains("kinds = "), "the policy itself arrives: {text}");
    assert!(live_grant_lines(&text).is_empty(), "no grant may be inherited (issue376): {:?}", live_grant_lines(&text));
    assert!(text.contains("GRANTS ARE NOT INHERITED"), "and the reader is told why the lines are commented: {text}");
    // With no standing consent, a non-fork Decision is NOT auto-accepted: there is no consent to apply.
    let f = draft(&root);
    let (ok, out) = run(&root, &["record", "decision", "--from", f.to_str().expect("utf8"), "--by", "ai", "--at", "2026-09-06"]);
    assert!(ok, "{out}");
    assert!(!out.contains("at record time"), "a fresh project has given no standing consent: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

/// Consent declared, words absent: the Decision stays proposed and the output names what is missing.
#[test]
fn consent_without_standing_words_stays_proposed_and_says_why() {
    let root = scaffold("nowords");
    let p = policy_path(&root);
    let text = std::fs::read_to_string(&p).expect("policy");
    std::fs::write(&p, text.replace("# standingConsent = \"d0207\"", "standingConsent = \"d0001\"")).expect("grant consent only");
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "consent"]);
    let f = draft(&root);
    let (ok, out) = run(&root, &["record", "decision", "--from", f.to_str().expect("utf8"), "--by", "ai", "--at", "2026-09-06"]);
    assert!(ok, "the record itself succeeds: {out}");
    assert!(out.contains("records no standingWords") && out.contains("stays proposed"), "{out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0001-probe.sysml")).expect("decision file");
    assert!(text.contains("DecisionStatus::proposed") && !text.contains("AcceptR1"), "no acceptance was fabricated: {text}");
    let _ = std::fs::remove_dir_all(&root);
}

/// Consent AND words declared: the acceptance quotes exactly the project's words - never the engine's.
#[test]
fn the_acceptance_quotes_the_projects_own_words() {
    let root = scaffold("words");
    let p = policy_path(&root);
    let text = std::fs::read_to_string(&p).expect("policy");
    let text = text.replace("# standingConsent = \"d0207\"", "standingConsent = \"d0001\"");
    // the shipped (commented) words are this repository's human's; the fixture's human says something else
    let start = text.find("# standingWords = ").expect("the commented words line ships");
    let end = start + text[start..].find('\n').expect("line end");
    let text = format!("{}standingWords = \"yes, record them as you go - I read the log\"{}", &text[..start], &text[end..]);
    std::fs::write(&p, text).expect("grant consent and words");
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "consent"]);
    let f = draft(&root);
    let (ok, out) = run(&root, &["record", "decision", "--from", f.to_str().expect("utf8"), "--by", "ai", "--at", "2026-09-06"]);
    assert!(ok && out.contains("accepted D0001 at record time under standing consent d0001"), "{out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0001-probe.sysml")).expect("decision file");
    assert!(text.contains("yes, record them as you go - I read the log"), "the note quotes the project's words: {text}");
    assert!(!text.contains("customizedly"), "and never this repository's human's: {text}");
    let _ = std::fs::remove_dir_all(&root);
}
