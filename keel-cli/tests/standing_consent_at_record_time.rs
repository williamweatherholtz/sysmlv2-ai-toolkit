//! Standing consent (D0207) is applied at RECORD time by `keel record decision` (D0291) - the GitHub
//! channel that used to apply it at issue creation, notifying the human every time, is disconnected.
//!
//! Three arms:
//!   1. one declared decider + a NON-FORK  -> accepted at record time, AUTO-ACCEPTED note quoting the
//!      standing words, the decider as judge;
//!   2. one declared decider + a FORK (OPTION A / OPTION B in the text) -> stays proposed for the human;
//!   3. no declared decider -> stays proposed and says why (a judge is never guessed).

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
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

fn git(dir: &Path, args: &[&str]) -> bool {
    Command::new("git").arg("-C").arg(dir).args(["-c", "user.email=t@t", "-c", "user.name=t"]).args(args).output().is_ok_and(|o| o.status.success())
}

fn shallow_root(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("sc{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    root
}

fn project(tag: &str, with_decider: bool) -> PathBuf {
    let root = shallow_root(tag);
    assert!(run(&root, &["init", "."]).0, "scaffold");
    if with_decider {
        let p = root.join(".engine/contracts/github-actors.toml");
        let text = std::fs::read_to_string(&p).expect("actors toml");
        std::fs::write(&p, format!("{text}\nsomeone = \"you\"\n")).expect("declare one decider");
    }
    assert!(git(&root, &["init", "-q"]) && git(&root, &["add", "-A"]) && git(&root, &["commit", "-q", "-m", "init"]), "a HEAD to judge against");
    root
}

fn decision_file(root: &Path) -> String {
    let dir = root.join(".engine/decisions");
    let f = std::fs::read_dir(&dir).expect("decisions").flatten().map(|e| e.path()).find(|p| p.to_string_lossy().contains("probe")).expect("probe file");
    std::fs::read_to_string(f).expect("read")
}

const BASE: [&str; 12] = ["record", "decision", "--slug", "probe", "--title", "t", "--context", "c", "--rationale", "r", "--consequences", "q"];

#[test]
fn a_non_fork_is_accepted_at_record_time_under_standing_consent() {
    let root = project("nf", true);
    let mut args: Vec<&str> = BASE.to_vec();
    args.extend(["--decision", "one plain decision", "--author", "ai", "--date", "2026-09-03"]);
    let (ok, text) = run(&root, &args);
    assert!(ok, "{text}");
    assert!(text.contains("accepted D0001 at record time under standing consent d0207"), "{text}");
    let d = decision_file(&root);
    assert!(d.contains("DecisionStatus::accepted"), "flipped:\n{d}");
    assert!(d.contains("AUTO-ACCEPTED under standing consent (D0207)") && d.contains("issues raised are automatically accepted"), "the note quotes the standing words:\n{d}");
    assert!(d.contains("judgedBy = \"you\""), "the declared decider is the judge:\n{d}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_fork_stays_proposed_for_the_human() {
    let root = project("fk", true);
    let mut args: Vec<&str> = BASE.to_vec();
    args.extend(["--decision", "OPTION A (keep it) or OPTION B (drop it) - the human chooses", "--author", "ai", "--date", "2026-09-03"]);
    let (ok, text) = run(&root, &args);
    assert!(ok, "{text}");
    assert!(text.contains("a FORK"), "{text}");
    assert!(decision_file(&root).contains("DecisionStatus::proposed"), "a fork is never auto-accepted");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn with_no_declared_decider_nothing_is_guessed() {
    let root = project("nd", false);
    let mut args: Vec<&str> = BASE.to_vec();
    args.extend(["--decision", "one plain decision", "--author", "ai", "--date", "2026-09-03"]);
    let (ok, text) = run(&root, &args);
    assert!(ok, "{text}");
    assert!(text.contains("names 0 decider(s)"), "{text}");
    assert!(decision_file(&root).contains("DecisionStatus::proposed"), "no judge, no acceptance");
    let _ = std::fs::remove_dir_all(&root);
}
