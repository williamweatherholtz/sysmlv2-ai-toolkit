//! D0411 / issue426: a gesture citation is written by the command that observed the gesture, never
//! typed into a note.
//!
//! Under the recording delegation (D0192/D0289) an agent session could record an acceptance whose note
//! merely SAID `approved at the console`, and the substance rule accepted the word as the channel. The
//! console appends a device receipt of its own; a terminal cites its own TTY; a session with agent
//! markers and no terminal observed neither. Three properties:
//!   1. an agent session's `--note "approved at the console"` is refused before the write, naming the remedy;
//!   2. a typed TTY citation is refused the same way;
//!   3. `--words` - the human's quoted words naming the decision - still records.

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

/// Run as an AGENT session would: the marker set, stdin not a terminal, no TTY stand-in.
fn agent(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("CLAUDE_CODE_SESSION_ID", "00000000-0000-4000-8000-000000000002")
        .env("KEEL_ACTOR", "ai")
        .env_remove("KEEL_TTY_GESTURE")
        .stdin(Stdio::null())
        .output()
        .expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn shallow_root(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("ges{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    root
}

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
        &["record", "decision", "--slug", "probe", "--title", "t", "--context", "c", "--decision", "d", "--rationale", "r", "--consequences", "q", "--author", "ai", "--date", "2026-09-09"],
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
fn a_typed_gesture_word_is_refused_and_the_humans_quoted_words_record() {
    let root = project_with_a_proposed_decision("c");

    // 1. the console named in text: refused before the write, the remedy is --words
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "approved at the console", "--by", "you", "--date", "2026-09-09"]);
    assert!(!ok, "a typed console citation is refused: {text}");
    assert!(text.contains("written by the surface that observed it") && text.contains("--words"), "the refusal names the mechanism and the remedy: {text}");
    assert!(!decision_text(&root).contains("AcceptR1"), "nothing written");

    // 2. the TTY named in text, with no terminal on stdin: refused the same way
    let (ok, text) = agent(&root, &["accept", "d0001", "--note", "go ahead - TTY gesture: typed at an interactive terminal by you, 2026-09-09", "--by", "you", "--date", "2026-09-09"]);
    assert!(!ok, "a typed TTY citation is refused: {text}");
    assert!(text.contains("D0411"), "{text}");
    assert!(!decision_text(&root).contains("AcceptR1"), "nothing written");

    // 2b. the deck and a GitHub URL, likewise
    for note in ["tapped on the deck", "see https://github.com/x/y/issues/1#issuecomment-9"] {
        let (ok, text) = agent(&root, &["accept", "d0001", "--note", note, "--by", "you", "--date", "2026-09-09"]);
        assert!(!ok, "`{note}` is refused: {text}");
        assert!(!decision_text(&root).contains("AcceptR1"), "nothing written");
    }

    // 3. the human's words, verbatim, naming the decision: recorded, and the record quotes them
    let (ok, text) = agent(&root, &["accept", "d0001", "--words", "yes, accept d0001 as written", "--by", "you", "--date", "2026-09-09"]);
    assert!(ok, "--words records under the delegation: {text}");
    let recorded = decision_text(&root);
    assert!(recorded.contains("AcceptR1") && recorded.contains("\u{201C}yes, accept d0001 as written\u{201D}"), "the record carries the declared pair:\n{recorded}");
    let _ = std::fs::remove_dir_all(&root);
}

/// The reject path shares the channel layer (D0393): a typed gesture is refused there too.
#[test]
fn a_typed_gesture_word_is_refused_on_reject_too() {
    let root = project_with_a_proposed_decision("r");
    let (ok, text) = agent(&root, &["reject", "d0001", "--note", "declined at the console", "--by", "you", "--date", "2026-09-09"]);
    assert!(!ok, "a typed console citation is refused on reject: {text}");
    assert!(text.contains("D0411"), "{text}");
    assert!(!decision_text(&root).contains("RejectR1"), "nothing written");
    let _ = std::fs::remove_dir_all(&root);
}
