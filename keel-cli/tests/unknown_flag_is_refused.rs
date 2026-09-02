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
        text.contains("PASS") && count_before(&text, " scanned").is_some_and(|n| n > 0),
        "and it must actually scan the corpus — otherwise the refusal traded one vacuous pass for \
         another: {text}"
    );
}

/// The number immediately preceding `label` in `text`, if any.
///
/// # Why a parse and not `!contains("0 <label>")`
///
/// That substring form is wrong for every count ending in zero, because "580 tracked file(s)"
/// CONTAINS "0 tracked file(s)". It has now broken this suite twice — once at 370 scanned, once at
/// 580 tracked files — and the first fix was applied to one site while an identical one stayed live.
/// Reading the number is the only form that says what was meant.
fn count_before(text: &str, label: &str) -> Option<u64> {
    let at = text.find(label)?;
    let digits: String = text[..at].chars().rev().take_while(char::is_ascii_digit).collect();
    digits.chars().rev().collect::<String>().parse().ok()
}

/// No test may assert a non-zero count by SUBSTRING again (D0047).
///
/// The `!contains("0 <label>")` form is wrong for every count ending in zero — "580 tracked file(s)"
/// contains "0 tracked file(s)". It has broken this suite TWICE: at 370 scanned, and at 580 tracked
/// files. The first repair was applied to the one site that had fired while an identical one sat
/// live in this very file, which is the per-command-instead-of-per-class shape that also produced
/// issue322 and issue325 on the same day.
///
/// So the correction becomes a control rather than a third repair: a lesson is not a control (D0047),
/// and the next author reaching for the substring form should be stopped by a failing test rather
/// than by remembering this paragraph.
#[test]
fn no_test_asserts_a_count_by_substring() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut offenders: Vec<String> = Vec::new();
    let entries = std::fs::read_dir(&dir).expect("the tests directory is readable");
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().is_none_or(|x| x != "rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        for (n, line) in text.lines().enumerate() {
            // The defect is the NEGATED substring test against a zero-prefixed count. A line that
            // merely mentions the shape in prose (this doc comment, for one) is not code, so require
            // the `!`-negation and a `contains(` call on the same line.
            let is_code = line.contains("!text.contains(\"0 ") || line.contains("!out.contains(\"0 ");
            if is_code {
                offenders.push(format!("{}:{}", p.file_name().unwrap_or_default().to_string_lossy(), n + 1));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "these assert a non-zero count by SUBSTRING, which silently breaks at 10, 20, ... 580. Parse \
         the number instead (see `count_before`): {offenders:?}"
    );
}

/// issue346: `record decision --from FILE` accepted an unknown `marker:` value SILENTLY and produced an
/// UNMARKED Decision - `marker: prospective-change`, a plausible spelling of the real `process-change`,
/// yielded a Decision the process-change guard could not see, and the locked-file edit it was meant to
/// authorise was refused for want of a marker that had been written. The vocabulary is two words; an
/// unrecognised value is refused by name, and nothing is written.
#[test]
fn an_unknown_decision_marker_is_refused_not_dropped() {
    let dir = std::env::temp_dir().join(format!("keel-marker-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("tmp");
    let f = dir.join("d.md");
    std::fs::write(
        &f,
        "slug: marker-probe
date: 2026-09-02
marker: prospective-change
--- title
t
--- context
c
--- decision
d
--- rationale
r
--- consequences
q
",
    )
    .expect("write");
    let before = std::fs::read_dir(repo().join(".engine/decisions")).map(|d| d.count()).unwrap_or(0);
    let (code, text) = run(&["record", "decision", "--from", f.to_str().expect("utf8"), "--author", "claudeOpus5"]);
    let after = std::fs::read_dir(repo().join(".engine/decisions")).map(|d| d.count()).unwrap_or(0);
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 2, "an unknown marker must refuse: {text}");
    assert!(text.contains("unknown marker") && text.contains("prospective-change"), "the refusal names the value: {text}");
    assert_eq!(before, after, "nothing was written");
}
