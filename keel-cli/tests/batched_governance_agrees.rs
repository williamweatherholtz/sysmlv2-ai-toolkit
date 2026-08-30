//! The equality proof for the batched governance index (issue317).
//!
//! `reprocess-candidates` answers "what was authored under a process version a SAFETY change later
//! superseded" — the antiquated-data question. It did not finish: 240 seconds, exit 124, zero
//! bytes, because it cost a pickaxe search plus a `merge-base` per Decision for each of 501
//! stories. Measured per item, the slow path is ~15 seconds; 501 of those is roughly two hours.
//!
//! The batched index reads the same history ONCE. That makes it fast, and fast is worthless if it
//! is not the SAME ANSWER — so this suite compares the two paths on real corpus items rather than
//! trusting the speed-up. A fast lens that quietly answers differently is worse than a slow correct
//! one, and would be far harder to notice.

use std::path::Path;
use std::process::Command;

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root")
}

/// Real stories from this corpus, deliberately spanning vintages: an early Rust sprint (June), a
/// mid-life sprint, and a recent one. A single item could agree by luck.
const SAMPLE: [&str; 3] = ["storyRustS0Workspace", "storyRustS1Lexer", "needSetRederivationStory"];

#[test]
fn the_batched_path_gives_the_same_answer_as_the_per_item_path() {
    for item in SAMPLE {
        let slow = keel_cli::govern::governing_version(repo(), item);
        let fast = keel_cli::govern::governing_version_via_index(repo(), item);
        assert_eq!(
            fast, slow,
            "batched governance must be BYTE-IDENTICAL to the per-item resolver for `{item}` — the \
             index is a performance change and must not be a semantic one"
        );
    }
}

#[test]
fn the_corpus_lens_finishes_and_answers_something() {
    let mut bin = std::env::current_exe().expect("test exe");
    bin.pop();
    if bin.ends_with("deps") {
        bin.pop();
    }
    let bin = bin.join(if cfg!(windows) { "keel.exe" } else { "keel" });
    let out = Command::new(&bin)
        .args(["reprocess-candidates", "."])
        .current_dir(repo())
        .output()
        .expect("keel");
    assert!(out.status.success(), "reprocess-candidates must EXIT 0: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("reprocess_candidates"),
        "and it must produce its answer, not an empty stream — the defect was zero bytes after 240s"
    );
    // The corpus HAS safety-change Decisions and items predating them, so an empty set here would
    // mean the lens is answering vacuously rather than working.
    assert!(
        text.contains("due_to"),
        "this corpus contains items chartered before safety changes landed, so an EMPTY result is a \
         mis-aimed lens rather than a clean bill of health: {text:.300}"
    );
}
