//! GH#14: a mistyped flag must not become the ROOT and turn a check green over nothing.
//!
//! `keel guard --read` used to gate a directory literally named `--read`, find no files, and report
//! every guard PASS with 0 scanned. Two defects compounding: silent argument mis-parsing, and
//! pass-at-zero. Together they produce A GREEN RUN OVER NOTHING, which is worse than an error
//! because it is indistinguishable from a clean tree.
//!
//! The same shape recurred while building `github-ingest`, where a trailing `--at` value was read as
//! the root — which is why this is a shared refusal rather than a fix in one command.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root")
}

fn run(args: &[&str]) -> (i32, String) {
    let out = Command::new(keel_bin()).args(args).current_dir(repo()).output().expect("keel");
    (
        out.status.code().unwrap_or(-1),
        format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
    )
}

#[test]
fn an_unknown_flag_is_refused_rather_than_treated_as_a_path() {
    for cmd in ["guard", "validate", "check-engine"] {
        let (code, text) = run(&[cmd, "--read"]);
        assert_eq!(code, 2, "`keel {cmd} --read` must REFUSE (exit 2), not answer: {text}");
        assert!(
            text.contains("looks like a flag"),
            "and it must say WHY, so the operator fixes the command rather than the tree: {text}"
        );
        // Look for an actual VERDICT line, not the bare word: the refusal message itself explains
        // the hazard using "PASS", and the first draft of this assertion tripped on its own text.
        assert!(
            !text.contains("] PASS") && !text.contains("validated clean"),
            "it must emit no verdict at all — a green line here is the whole defect: {text}"
        );
    }
}

#[test]
fn a_real_path_still_works_so_the_refusal_is_not_a_lockout() {
    let (code, text) = run(&["guard", "identity-present", "."]);
    assert_eq!(code, 0, "an ordinary ROOT argument must still be accepted: {text}");
    assert!(
        text.contains("PASS") && !text.contains("0 scanned"),
        "and it must actually scan the corpus — otherwise the refusal traded one vacuous pass for \
         another: {text}"
    );
}
