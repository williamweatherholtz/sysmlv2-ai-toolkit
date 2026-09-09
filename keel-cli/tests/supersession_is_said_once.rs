//! D0384 option A / D0398 (dcSupersessionIsSaidOnce, sprint 622): supersession is said ONCE, by the edge.
//! A Decision with an incoming `#Supersede` edge is RETIRED - absent from the acceptance queue whatever
//! its `status` field kept; a `#SupersedeClause` edge reverses one clause and leaves its target in force;
//! and `DecisionStatus::superseded` is no longer a member, so the engine-instance gate refuses it by name.
//! Constructed on a real scaffold (no standing consent, so every recorded Decision stays proposed) - the
//! pair is BUILT here, never read from this repository's tree (issue396 was found by reading the tree).

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

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("so{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    root
}

const BODY: [&str; 8] = ["--context", "the fixture needs a decision with a substantive context", "--rationale", "a rationale long enough for the decision-rationale guard to accept", "--consequences", "q", "--author", "ai"];

fn record(root: &Path, slug: &str, title: &str, extra: &[&str]) -> (bool, String) {
    let mut args: Vec<&str> = vec!["record", "decision", "--slug", slug, "--title", title, "--decision", "one plain decision", "--date", "2026-09-09"];
    args.extend_from_slice(&BODY);
    args.extend_from_slice(extra);
    let (ok, out) = run(root, &args);
    assert!(ok, "{out}");
    (ok, out)
}

/// The fixture owns its Decisions: flip one's standing in place (what `keel accept` records on a
/// human's word), so the reader under test sees an ACCEPTED target.
fn set_accepted(root: &Path, file: &str) {
    let p = root.join(".engine/decisions").join(file);
    let text = std::fs::read_to_string(&p).expect("decision");
    assert!(text.contains("DecisionStatus::proposed"), "{text}");
    std::fs::write(&p, text.replacen("DecisionStatus::proposed", "DecisionStatus::accepted", 1)).expect("flip");
}

#[test]
fn a_retired_decision_leaves_the_queue_and_a_clause_reversal_leaves_its_target_in_force() {
    let root = scaffold("queue");
    record(&root, "first", "firstRule: the first rule", &[]);
    record(&root, "second", "secondRule: the second rule", &[]);
    set_accepted(&root, "0002-second.sysml");
    // d0003 RETIRES d0001 whole; d0004 reverses ONE clause of d0002 and leaves it in force.
    let (_, out) = record(&root, "third", "thirdRule: replaces the first rule", &["--supersedes", "d0001"]);
    assert!(out.contains("d0001 is RETIRED whole"), "the record says what the edge does: {out}");
    let (_, out) = record(&root, "fourth", "fourthRule: narrows one clause of the second rule", &["--supersedes-clause", "d0002"]);
    assert!(out.contains("#SupersedeClause d0004 -> d0002") && out.contains("stays in force"), "{out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0004-fourth.sysml")).expect("decision");
    assert!(text.contains("#SupersedeClause dependency from d0004 to d0002;"), "the clause edge is in the Decision's own file: {text}");
    let (ok, out) = run(&root, &["check-engine", "."]);
    assert!(ok, "both markers resolve: {out}");

    // The queue: d0001 still READS proposed, and is absent because the edge retired it; its proposed
    // siblings d0003 and d0004 are present. This is the issue396 shape, closed by construction.
    let d1 = std::fs::read_to_string(root.join(".engine/decisions/0001-first.sysml")).expect("d0001");
    assert!(d1.contains("DecisionStatus::proposed"), "the field was never rewritten - the edge is the fact: {d1}");
    let (_, queue) = run(&root, &["show", "authority-queue", "."]);
    assert!(!queue.contains("d0001"), "a retired Decision is not on the human's queue: {queue}");
    assert!(queue.contains("d0003") && queue.contains("d0004"), "its proposed siblings are: {queue}");
    let (_, orient) = run(&root, &["orient", "."]);
    let pending = orient.split("\"pendingAcceptances\"").nth(1).and_then(|s| s.split(']').next()).expect("orient lists pendingAcceptances");
    assert!(!pending.contains("d0001") && pending.contains("d0003"), "orient's pendingAcceptances agrees: {pending}");

    // The scorecard: d0002 stays counted accepted under its clause reversal; d0001 is the one retired.
    let (_, gov) = run(&root, &["report", "governance", "."]);
    assert!(gov.contains("1 accepted / 1 retired of 4 total"), "accepted-and-in-force vs retired, from the edges: {gov}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_retired_status_value_cannot_be_authored() {
    let root = scaffold("member");
    record(&root, "only", "onlyRule: the only rule", &[]);
    let p = root.join(".engine/decisions/0001-only.sysml");
    let text = std::fs::read_to_string(&p).expect("decision");
    std::fs::write(&p, text.replacen("DecisionStatus::proposed", "DecisionStatus::superseded", 1)).expect("write the old shape");
    let (ok, out) = run(&root, &["check-engine", "."]);
    assert!(!ok, "the engine-instance gate refuses the removed member: {out}");
    assert!(out.contains("unknown member `superseded`") && out.contains("DecisionStatus"), "and names it: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_decision_is_retired_whole_or_reversed_in_one_clause_never_both() {
    let root = scaffold("both");
    record(&root, "base", "baseRule: the base rule", &[]);
    let mut args: Vec<&str> = vec!["record", "decision", "--slug", "twice", "--title", "twiceRule: names the base twice", "--decision", "one plain decision", "--date", "2026-09-09"];
    args.extend_from_slice(&BODY);
    args.extend_from_slice(&["--supersedes", "d0001", "--supersedes-clause", "d0001"]);
    let (ok, out) = run(&root, &args);
    assert!(!ok && out.contains("never both"), "{out}");
    assert!(std::fs::read_dir(root.join(".engine/decisions")).expect("dir").flatten().all(|e| !e.file_name().to_string_lossy().contains("twice")), "nothing written");
    let _ = std::fs::remove_dir_all(&root);
}
