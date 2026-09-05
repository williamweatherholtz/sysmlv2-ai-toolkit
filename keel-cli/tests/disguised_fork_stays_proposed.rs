//! D0322 / issue373 (stpa-self run 1, UCA-R1): a Decision that WEIGHS alternatives in prose without the
//! `OPTION X (label)` marker is a fork in substance, and record-time standing consent (D0291) must not
//! accept it on the spot. Two distinct fork signals in the decision text hold it proposed and name the
//! words; a genuine decision that mentions the rejected alternative in passing auto-accepts as before.
//! Run on the binary against a real scaffold with one declared decider.

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
    let root = base.join(format!("f{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    // one declared decider, so standing consent has a judge to bind
    std::fs::write(root.join(".engine/contracts/github-actors.toml"), "[logins]\nowner = \"you\"\n").expect("deciders");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

fn draft(root: &Path, decision: &str) -> PathBuf {
    let f = root.join("draft.md");
    let rationale = "r ".repeat(120);
    std::fs::write(
        &f,
        format!("slug: probe\ndate: 2026-09-05\n--- title\nprobe: a short title before the colon\n--- context\nc\n--- decision\n{decision}\n--- rationale\n{rationale}\n--- consequences\nq\n"),
    )
    .expect("draft");
    f
}

#[test]
fn a_decision_that_weighs_alternatives_without_the_marker_is_held_not_auto_accepted() {
    let root = scaffold("held");
    let f = draft(&root, "Route as recommended: for the skill we take option B, deploying it with its state stated, rather than option A.");
    let (ok, out) = run(&root, &["record", "decision", "--from", f.to_str().expect("utf8"), "--by", "ai", "--at", "2026-09-05"]);
    assert!(ok, "recording still succeeds - the Decision exists, proposed: {out}");
    assert!(out.contains("HELD as a fork in substance") && out.contains("option, recommend"), "held, naming the words that weigh: {out}");
    assert!(!out.contains("accepted D0001 at record time"), "and NOT auto-accepted: {out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0001-probe.sysml")).expect("decision file");
    assert!(!text.contains("AcceptR1"), "no acceptance result was written: {text}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_genuine_decision_naming_its_rejected_alternative_in_passing_still_auto_accepts() {
    let root = scaffold("plain");
    let f = draft(&root, "Adopt the merge; the alternative of a never-overwrite list would freeze the engine's own sections at scaffold vintage.");
    let (ok, out) = run(&root, &["record", "decision", "--from", f.to_str().expect("utf8"), "--by", "ai", "--at", "2026-09-05"]);
    assert!(ok, "{out}");
    assert!(out.contains("accepted D0001 at record time under standing consent"), "one signal in passing is a decision, not a fork: {out}");
    let _ = std::fs::remove_dir_all(&root);
}
