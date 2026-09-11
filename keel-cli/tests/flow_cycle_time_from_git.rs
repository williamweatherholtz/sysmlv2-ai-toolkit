//! `keel show flow` / `keel report flow` read cycle time from GIT, in minutes, never from the
//! day-granular `judgedAt` (`dcCycleTimeReadsFromGit`; issue483, issue485).
//!
//! The D0388 pair, named before the real tree was read:
//! * KNOWN-POSITIVE - a fixture repository whose two sprints land their retro results 30 and 90
//!   minutes after the commit that created them reads `minutes: 30` and `minutes: 90`, and the
//!   cycle-time card reads in hours because the median is under a day.
//! * KNOWN-NEGATIVE - a sprint whose retro result exists only in the working tree (no landing
//!   commit) is `open` with `minutes: null`, never 0; a project outside any git repository is
//!   reported as unreadable, never as a zero.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keel"))
}

struct Tmp(PathBuf);
impl Drop for Tmp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn fresh(tag: &str) -> Tmp {
    let dir = std::env::temp_dir().join(format!("keel-flow-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".engine")).expect("mkdir .engine");
    std::fs::create_dir_all(dir.join(".tracking").join("delivery")).expect("mkdir delivery");
    Tmp(dir)
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// Commit everything at a FIXED committer time, so the fixture's intervals are exact.
fn commit_at(dir: &Path, epoch: i64, msg: &str) {
    git(dir, &["add", "-A"]);
    let date = format!("@{epoch} +0000");
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["commit", "-q", "-m", msg])
        .env("GIT_AUTHOR_DATE", &date)
        .env("GIT_COMMITTER_DATE", &date)
        .output()
        .expect("git commit");
    assert!(out.status.success(), "commit {msg}: {}", String::from_utf8_lossy(&out.stderr));
}

fn sprint_text(n: u32, points: u32) -> String {
    format!(
        "package ProjectDeliveryS{n} {{\n    part story{n} : Story {{ :>> estimatedPoints = {points}; }}\n    verification s{n}RetroGate : Test {{ :>> title = \"Sprint {n} retro gate\"; }}\n}}\n"
    )
}

fn retro_result(n: u32) -> String {
    format!("    part s{n}RetroGateR1 : TestResult {{ :>> outcome = VerdictKind::pass; :>> judgedAt = \"2026-09-11\"; }}\n")
}

fn write(dir: &Path, rel: &str, text: &str) {
    std::fs::write(dir.join(rel), text).expect("write");
}

fn append_retro(dir: &Path, rel: &str, n: u32) {
    let p = dir.join(rel);
    let mut s = std::fs::read_to_string(&p).expect("read");
    let close = s.rfind('}').expect("closing brace");
    s.insert_str(close, &retro_result(n));
    std::fs::write(&p, s).expect("write");
}

/// The lens, with its pretty-printing whitespace removed so assertions can name `"key":value`.
fn compact(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    let mut in_str = false;
    let mut escaped = false;
    for c in json.chars() {
        if in_str {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
            out.push(c);
        } else if !c.is_whitespace() {
            out.push(c);
        }
    }
    out
}

fn show_flow(dir: &Path) -> String {
    let out = keel().args(["show", "flow"]).arg(dir).arg("--json").output().expect("keel show flow");
    assert!(out.status.success(), "show flow: {}", String::from_utf8_lossy(&out.stderr));
    compact(&String::from_utf8_lossy(&out.stdout))
}

fn report_flow(dir: &Path) -> String {
    let out = keel().args(["report", "flow", "--root"]).arg(dir).output().expect("keel report flow");
    assert!(out.status.success(), "report flow: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The `minutes` value the lens gives sprint `name`, as the raw JSON token (`30`, `null`, ...).
fn minutes_of(json: &str, name: &str) -> String {
    let at = json.find(&format!("\"sprint\":\"{name}\"")).unwrap_or_else(|| panic!("{name} in lens: {json}"));
    let rest = &json[at..];
    let m = rest.find("\"minutes\":").expect("minutes key") + "\"minutes\":".len();
    rest[m..].split([',', '}']).next().expect("token").trim().to_string()
}

fn state_of(json: &str, name: &str) -> String {
    let at = json.find(&format!("\"sprint\":\"{name}\"")).unwrap_or_else(|| panic!("{name} in lens: {json}"));
    let rest = &json[at..];
    let m = rest.find("\"state\":\"").expect("state key") + "\"state\":\"".len();
    rest[m..].split('"').next().expect("token").to_string()
}

const T0: i64 = 1_700_000_000;

/// Two sprints created and closed 30 and 90 minutes apart, and a third whose retro result never
/// landed in a commit.
fn fixture(tag: &str) -> Tmp {
    let t = fresh(tag);
    let d = &t.0;
    git(d, &["init", "-q"]);
    git(d, &["config", "user.email", "p@e.invalid"]);
    git(d, &["config", "user.name", "probe"]);
    write(d, ".engine/README.md", "engine\n");
    commit_at(d, T0, "base");
    write(d, ".tracking/delivery/sprint1_a.sysml", &sprint_text(1, 2));
    commit_at(d, T0 + 600, "sprint 1 opens");
    append_retro(d, ".tracking/delivery/sprint1_a.sysml", 1);
    commit_at(d, T0 + 600 + 30 * 60, "sprint 1 retro lands");
    write(d, ".tracking/delivery/sprint2_b.sysml", &sprint_text(2, 5));
    commit_at(d, T0 + 4000, "sprint 2 opens");
    append_retro(d, ".tracking/delivery/sprint2_b.sysml", 2);
    commit_at(d, T0 + 4000 + 90 * 60, "sprint 2 retro lands");
    write(d, ".tracking/delivery/sprint3_c.sysml", &sprint_text(3, 3));
    commit_at(d, T0 + 10_000, "sprint 3 opens");
    // The retro result is written but NOT committed: the ceremony has no landing commit.
    append_retro(d, ".tracking/delivery/sprint3_c.sysml", 3);
    t
}

/// KNOWN-POSITIVE: 30 and 90 minutes read as 30 and 90; the card reads in hours.
#[test]
fn two_sprints_thirty_and_ninety_minutes_apart_read_thirty_and_ninety() {
    let t = fixture("pos");
    let json = show_flow(&t.0);
    assert!(json.contains("\"lens\":\"flow\""), "{json}");
    assert!(json.contains("\"resolution\":\"minutes, from commit timestamps\""), "{json}");
    assert_eq!(minutes_of(&json, "sprint1_a"), "30", "{json}");
    assert_eq!(minutes_of(&json, "sprint2_b"), "90", "{json}");
    assert_eq!(state_of(&json, "sprint1_a"), "closed");
    assert_eq!(state_of(&json, "sprint2_b"), "closed");
    // Each closed sprint was touched by two commits: its birth and its retro landing.
    let s1 = &json[json.find("\"sprint\":\"sprint1_a\"").expect("s1")..];
    assert!(s1.contains("\"commits\":2"), "{s1}");
    assert!(json.contains("\"points\":2"), "{json}");
    assert!(json.contains("\"points\":5"), "{json}");
    assert!(json.contains("\"closed\":2"), "{json}");

    // The report's card: median of 30 and 90 = 60 minutes, under a day, so it reads in hours.
    let text = report_flow(&t.0);
    assert!(text.contains("Cycle time"), "{text}");
    assert!(text.contains("1.0 h"), "cycle card in hours: {text}");
    assert!(text.contains("Time / story point"), "{text}");
    assert!(text.contains("Inter-commit gap"), "{text}");
    assert!(text.contains("Point calibration"), "{text}");
    assert!(!text.contains("unavailable"), "{text}");
}

/// KNOWN-NEGATIVE: a retro result with no landing commit is `open`, `minutes: null`, never 0.
#[test]
fn a_sprint_whose_retro_has_no_landing_commit_is_open_never_zero() {
    let t = fixture("neg");
    let json = show_flow(&t.0);
    assert_eq!(state_of(&json, "sprint3_c"), "open", "{json}");
    assert_eq!(minutes_of(&json, "sprint3_c"), "null", "{json}");
    assert!(json.contains("\"open\":1"), "{json}");
    let s3 = &json[json.find("\"sprint\":\"sprint3_c\"").expect("s3")..];
    assert!(!s3.starts_with("\"sprint\":\"sprint3_c\",\"points\":3,\"state\":\"closed\""), "{s3}");
    // The report counts it open in the cycle-time detail rather than folding a 0 into the median.
    let text = report_flow(&t.0);
    assert!(text.contains("1 open"), "{text}");
}

/// KNOWN-NEGATIVE: a project git cannot read is reported as such - not as a zero.
#[test]
fn a_project_outside_git_is_reported_unreadable_never_zero() {
    let t = fresh("nogit");
    write(&t.0, ".tracking/delivery/sprint1_a.sysml", &sprint_text(1, 2));
    let out = keel().args(["show", "flow"]).arg(&t.0).arg("--json").output().expect("keel show flow");
    let text = compact(&String::from_utf8_lossy(&out.stdout));
    assert!(text.contains("\"error\""), "{text}");
    assert!(!text.contains("\"minutes\":0"), "{text}");
    let report = report_flow(&t.0);
    assert!(report.contains("unavailable"), "{report}");
    assert!(report.contains("git history could not be read"), "{report}");
}
