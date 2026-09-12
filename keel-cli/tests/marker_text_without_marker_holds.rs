//! D0439 / issue460: a Decision whose TEXT names the marker vocabulary - `process-change`,
//! `#ProspectiveChange` - while its draft carries no `marker:` line is held proposed under standing
//! consent exactly as a marked one is (D0337), and the record's header says why. On 2026-09-10 D0432's
//! consequences read `Process-change (D0337): ... it waits for the human's word` and it AUTO-ACCEPTED,
//! because only the marker line was read. The D0388 pair, end to end on the binary against a scaffold
//! with one declared decider: the unmarked text is held; the same text WITH the marker takes the
//! existing D0337 path; a text naming neither auto-accepts as before.

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
    let root = base.join(format!("m{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    std::fs::write(root.join(".engine/contracts/github-actors.toml"), "[logins]\nowner = \"you\"\n").expect("deciders");
    let p = root.join(".engine/contracts/attestation-policy.toml");
    let text = std::fs::read_to_string(&p).expect("policy").replace("# standingConsent = \"d0207\"", "standingConsent = \"d0207\"");
    let start = text.find("# standingWords = ").expect("the commented words line ships");
    let end = start + text[start..].find('\n').expect("line end");
    let text = format!("{}standingWords = \"fine by me, keep recording\"{}", &text[..start], &text[end..]);
    std::fs::write(&p, text).expect("grant");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

/// The issue460 draft: consequences that classify the Decision as a process change, in words.
const CONSEQUENCES: &str = "CLAUDE.md names the run. Process-change (D0337): it changes what the verifier runs, so it waits for the human's word.";

fn draft(root: &Path, marker_line: &str, consequences: &str) -> PathBuf {
    let f = root.join("draft.md");
    let rationale = "r ".repeat(120);
    std::fs::write(
        &f,
        format!("slug: probe\ndate: 2026-09-10\n{marker_line}--- title\nprobe: a short title before the colon\n--- context\nThe verifier ran a filter the primary chose and the land ran the whole lib.\n--- decision\nThe verifier runs the touched set.\n--- rationale\n{rationale}\n--- consequences\n{consequences}\n"),
    )
    .expect("draft");
    f
}

fn record(root: &Path, f: &Path) -> (bool, String) {
    run(root, &["record", "decision", "--from", f.to_str().expect("utf8"), "--by", "ai", "--at", "2026-09-10"])
}

/// Known-positive: the text says process-change, the draft has no marker line - HELD, header says why,
/// no acceptance result written, and the guard that reads the file agrees with the write path.
#[test]
fn an_unmarked_text_naming_process_change_is_held_and_its_header_says_why() {
    let root = scaffold("held");
    let (ok, out) = record(&root, &draft(&root, "", CONSEQUENCES));
    assert!(ok, "the record itself succeeds: {out}");
    assert!(out.contains("HELD proposed (D0337/issue460)") && out.contains("process-change"), "held, naming the word: {out}");
    assert!(!out.contains("accepted D0001 at record time"), "and NOT auto-accepted: {out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0001-probe.sysml")).expect("decision file");
    assert!(text.contains("DecisionStatus::proposed") && !text.contains("AcceptR1"), "proposed, no acceptance result: {text}");
    let held: Vec<&str> = text.lines().filter(|l| l.starts_with("// HELD (D0337/issue460):")).collect();
    assert_eq!(held.len(), 1, "one hold line under the header: {text}");
    assert!(held[0].contains("process-change") && held[0].contains("NOT A PROCESS CHANGE"), "{}", held[0]);
    assert!(text.lines().nth(1).is_some_and(|l| l.starts_with("// HELD")), "directly under the scaffolded header: {text}");
    // the guard that reads the file: a HELD Decision is proposed, so it is not an auto-acceptance to scan
    let (_, out) = run(&root, &["gate", "guard", "."]);
    assert!(out.contains("[guard:consent-scope] PASS \u{2014} 0 scanned"), "{out}");
    let _ = std::fs::remove_dir_all(&root);
}

/// Known-negative 1: the same text WITH the marker line is D0337's business - OUTSIDE, not HELD.
#[test]
fn the_same_text_with_the_marker_takes_the_existing_d0337_path() {
    let root = scaffold("mark");
    let (ok, out) = record(&root, &draft(&root, "marker: process-change\n", CONSEQUENCES));
    assert!(ok, "{out}");
    assert!(out.contains("OUTSIDE standing consent") && !out.contains("HELD proposed"), "the marker path, not the hold: {out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0001-probe.sysml")).expect("decision file");
    assert!(text.contains("#ProspectiveChange part d0001") && !text.contains("// HELD"), "{text}");
    let _ = std::fs::remove_dir_all(&root);
}

/// Known-negative 2: a text naming neither word auto-accepts as today - the hold reads nothing into it.
#[test]
fn a_text_naming_no_marker_word_still_auto_accepts() {
    let root = scaffold("plain");
    let (ok, out) = record(&root, &draft(&root, "", "CLAUDE.md names the run; the processes changed nothing and no guard moves."));
    assert!(ok, "{out}");
    assert!(out.contains("accepted D0001 at record time under standing consent"), "{out}");
    let text = std::fs::read_to_string(root.join(".engine/decisions/0001-probe.sysml")).expect("decision file");
    assert!(text.contains("AcceptR1") && !text.contains("// HELD"), "{text}");
    let _ = std::fs::remove_dir_all(&root);
}

/// The author's stated way out: a mention in passing with `NOT A PROCESS CHANGE: <why>` auto-accepts.
#[test]
fn a_declared_mention_in_passing_is_not_held() {
    let root = scaffold("decl");
    let (ok, out) = record(&root, &draft(&root, "", "Rank 1 lands with its own #ProspectiveChange Decision. NOT A PROCESS CHANGE: this Decision ranks work, it changes no process."));
    assert!(ok, "{out}");
    assert!(out.contains("accepted D0001 at record time under standing consent") && !out.contains("HELD proposed"), "{out}");
    let _ = std::fs::remove_dir_all(&root);
}
