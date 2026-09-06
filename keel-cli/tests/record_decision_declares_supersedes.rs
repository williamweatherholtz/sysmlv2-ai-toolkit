//! D0352 (dcRecordDecisionDeclaresSupersedes): `keel record decision --supersedes dNNNN[,dNNNN]` and
//! `--derived-from stNNN|usNNN` (or the `supersedes:` / `derived-from:` lines of a `--from` draft)
//! author the `#Supersede` / `#DerivedFrom` edges WITH the Decision, so a reversal cannot land
//! edgeless; a target that does not exist refuses before anything is written. On the binary against
//! a real scaffold, which grants no standing consent.

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
    let root = base.join(format!("sp{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    root
}

const BODY: [&str; 8] = ["--context", "the fixture needs a decision with a substantive context", "--rationale", "a rationale long enough for the decision-rationale guard to accept", "--consequences", "q", "--author", "ai"];

fn record(root: &Path, slug: &str, title: &str, extra: &[&str]) -> (bool, String) {
    let mut args: Vec<&str> = vec!["record", "decision", "--slug", slug, "--title", title, "--decision", "one plain decision", "--date", "2026-09-06"];
    args.extend_from_slice(&BODY);
    args.extend_from_slice(extra);
    run(root, &args)
}

#[test]
fn a_reversal_recorded_with_supersedes_carries_the_edge_and_why_reads_it_back() {
    let root = scaffold("edge");
    assert!(record(&root, "first", "firstRule: the first rule", &[]).0);
    let (ok, out) = record(&root, "second", "secondRule: reverses the first rule", &["--supersedes", "d0001"]);
    assert!(ok, "{out}");
    assert!(out.contains("#Supersede d0002 -> d0001 authored with it"), "the record says what it authored: {out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0002-second.sysml")).expect("decision");
    assert!(text.contains("#Supersede dependency from d0002 to d0001;"), "the edge is in the Decision's own file: {text}");
    let (ok, out) = run(&root, &["validate", "."]);
    assert!(ok, "the edge resolves: {out}");
    let (_, why) = run(&root, &["show", "why", "d0001", "."]);
    assert!(why.contains("d0002"), "keel why reads the supersession back from d0001's side: {why}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn supersedes_naming_a_missing_decision_refuses_and_writes_nothing() {
    let root = scaffold("missing");
    let (ok, out) = record(&root, "orphan", "orphanRule: reverses nothing that exists", &["--supersedes", "d0077"]);
    assert!(!ok, "{out}");
    assert!(out.contains("--supersedes d0077") && out.contains("no Decision"), "the refusal names the target: {out}");
    assert!(std::fs::read_dir(root.join(".engine/decisions")).expect("dir").flatten().all(|e| !e.file_name().to_string_lossy().contains("orphan")), "nothing written");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_from_draft_declares_its_links_and_derived_from_needs_a_recorded_utterance() {
    let root = scaffold("draft");
    assert!(record(&root, "base", "baseRule: the base rule", &[]).0);
    let (ok, out) = run(&root, &["record", "statement", "--text", "please make the base rule stricter", "--said-by", "you", "--said-at", "2026-09-06", "--channel", "chat", "--title", "stricter please", "--by", "ai", "--at", "2026-09-06"]);
    assert!(ok, "a Statement to derive from: {out}");
    let draft = root.join("d.md");
    std::fs::write(&draft, "slug: stricter\ndate: 2026-09-06\nsupersedes: d0001\nderived-from: st001\n--- title\nstricterRule: the base rule, stricter\n--- context\nthe fixture needs a decision with a substantive context\n--- decision\none plain decision\n--- rationale\na rationale long enough for the decision-rationale guard to accept\n--- consequences\nq\n").expect("draft");
    let (ok, out) = run(&root, &["record", "decision", "--from", draft.to_str().expect("utf8"), "--by", "ai", "--at", "2026-09-06"]);
    assert!(ok, "{out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0002-stricter.sysml")).expect("decision");
    assert!(text.contains("#Supersede dependency from d0002 to d0001;") && text.contains("#DerivedFrom dependency from d0002 to st001;"), "{text}");
    assert!(run(&root, &["validate", "."]).0);
    // derived-from must name a recorded utterance or story, not an arbitrary item
    let (ok, out) = record(&root, "loose", "looseRule: derives from a task", &["--derived-from", "d0001"]);
    assert!(!ok && out.contains("--derived-from d0001"), "{out}");
    let _ = std::fs::remove_dir_all(&root);
}
