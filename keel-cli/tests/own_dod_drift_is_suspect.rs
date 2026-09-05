//! dcOwnDoDDriftIsSuspect: a done task whose OWN criterion text changed since its passing result
//! computes as SUSPECT, exactly as it already did when a dependency's criterion changed. Before this,
//! the owner of a task could rewrite the criterion after the pass (D0108) with the pass still standing -
//! the thing verified was no longer the thing agreed, and nothing computed carried it.
//!
//! The fixture is a real git repository: the criterion at the result's `judgedAgainst` SHA is what the
//! comparison reads, so the DoD must exist in a commit BEFORE the result cites it - which is also the
//! stated limit: a criterion recorded in the same commit as its result has no historical text to compare.

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

fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn suspects(root: &Path) -> Vec<String> {
    let out = Command::new(keel_bin()).args(["orient", "."]).current_dir(root).env("KEEL_OFFLINE", "1").env("KEEL_ACTOR", "claudeOpus5").output().expect("orient");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("orient JSON: {e}\n{text}"));
    v["suspect"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

#[test]
fn editing_a_done_tasks_own_criterion_makes_it_suspect_and_restoring_clears_it() {
    let root = std::env::temp_dir().join(format!("keel-owndod-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    let backlog = root.join(".tracking").join("backlog.sysml");
    let dod = |text: &str| format!(
        "package Fx {{\n    private import EngineElement::*;\n    private import EngineWork::*;\n    private import EngineVerification::*;\n    private import EngineRelationships::*;\n\n    action def Build {{\n        action dcThing;\n        verification dcThingDoD : Test {{ :>> id = \"00000000-0000-4000-8000-000000000001\"; :>> method = VerificationMethod::test; :>> procedureText = \"{text}\"; }}\n    }}\n}}\n"
    );
    std::fs::write(&backlog, dod("the agreed criterion")).expect("seed");
    git(&root, &["init", "-q", "."]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "commit", "-q", "-m", "the criterion, agreed"]);
    let agreed_at = git(&root, &["rev-parse", "--short", "HEAD"]);

    // The pass, judged against the commit that holds the agreed criterion.
    let out = Command::new(keel_bin())
        .args(["append-result", "--file", ".tracking/backlog.sysml", "--task", "dcThing", "--sha", &agreed_at, "--verdict", "pass", "--judged-by", "claudeOpus5", "--judged-at", "2026-09-04", "--evidence", "probe"])
        .current_dir(&root)
        .env("KEEL_ACTOR", "claudeOpus5")
        .output()
        .expect("append-result");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "commit", "-q", "-m", "the pass"]);
    assert!(!suspects(&root).contains(&"dcThing".to_string()), "a done task whose criterion is unchanged is not suspect");

    // The owner widens the criterion AFTER the pass - the pass stands, the words it judged do not.
    let now = std::fs::read_to_string(&backlog).expect("read");
    std::fs::write(&backlog, now.replace("the agreed criterion", "the agreed criterion, plus a claim nobody verified")).expect("widen");
    assert!(suspects(&root).contains(&"dcThing".to_string()), "a done task whose OWN criterion changed since its pass must be SUSPECT");

    // Restored: clean again.
    std::fs::write(&backlog, now).expect("restore");
    assert!(!suspects(&root).contains(&"dcThing".to_string()), "restoring the agreed words clears the suspicion");
    let _ = std::fs::remove_dir_all(&root);
}
