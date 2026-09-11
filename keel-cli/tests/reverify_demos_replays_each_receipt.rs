//! `keel reverify --demos` (D0444 on D0312 B): a `method=demo` pass whose `// RAN:` receipt IS a command
//! under a prefix the project's `reverify.toml` declares replayable is a PASS at the write, not a
//! proposal, and `--demos` re-runs every such receipt at HEAD:
//!   1. a receipt whose replay exits 0 gets a fresh `pass` carrying the SAME receipt;
//!   2. a receipt whose replay exits non-zero gets a `fail` naming the command and the exit code;
//!   3. a demo whose receipt is prose lands `proposed` and is never replayed;
//!   4. with no `[demo]` section the flag is inert (exit 2) and every AI demo pass is a proposal.
//!
//! `keel attestation --json` reports the split as `demoReplayable` / `demoProposed` from the same tree.

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

/// Run as an AGENT session would: the marker set, stdin not a terminal.
fn agent(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("CLAUDE_CODE_SESSION_ID", "00000000-0000-4000-8000-000000000001")
        .env("KEEL_ACTOR", "ai")
        .stdin(Stdio::null())
        .output()
        .expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn shallow_root(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("rd{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    root
}

const FILE: &str = ".tracking/delivery/s669probe.sysml";

/// Three demo gates and one inspect gate, no results yet.
fn demos_file() -> String {
    let mut s = String::from("package S669Probe {\n    private import EngineElement::*;\n    private import EngineVerification::*;\n");
    for (i, (name, method)) in [("dGreen", "demo"), ("dRed", "demo"), ("dTold", "demo"), ("iGate", "inspect")].iter().enumerate() {
        s.push_str(&format!(
            "    verification {name} : Test {{ :>> id = \"e2e0000{i}-0000-4000-8000-00000000000a\"; :>> title = \"{name}\"; :>> createdAt = \"2026-09-11\"; :>> createdBy = \"ai\"; :>> method = VerificationMethod::{method}; :>> procedureText = \"gate {name}\"; }}\n"
        ));
    }
    s.push_str("}\n");
    s
}

/// A scaffolded project with the fixture file. `declare` writes the `[demo]` section: `keel ` (this
/// binary) and `cmd /C exit` / `sh -c exit` so a receipt can fail on purpose.
fn project(tag: &str, declare: bool) -> PathBuf {
    let root = shallow_root(tag);
    assert!(agent(&root, &["init", "."]).0, "scaffold");
    let contract = root.join(".engine/contracts/reverify.toml");
    let text = std::fs::read_to_string(&contract).expect("the scaffold ships reverify.toml");
    assert!(text.contains("[demo]"), "the scaffold ships the D0444 section: {text}");
    let head = text.split("[demo]").next().expect("the part before the section").to_string();
    let with = if declare { format!("{head}[demo]\nreplayable = [\"keel \", \"{}\"]\n", fail_prefix()) } else { head };
    std::fs::write(&contract, with).expect("contract");
    let f = root.join(FILE);
    std::fs::create_dir_all(f.parent().expect("dir")).expect("mkdir");
    std::fs::write(&f, demos_file()).expect("fixture");
    root
}

/// A shell-level `exit` runs as written by `reverify --demos` (no `keel ` prefix) and exits non-zero.
fn fail_prefix() -> &'static str {
    if cfg!(windows) { "cmd /C exit" } else { "sh -c exit" }
}

fn fail_command() -> String {
    format!("{} 3", fail_prefix())
}

fn file_text(root: &Path) -> String {
    std::fs::read_to_string(root.join(FILE)).expect("read")
}

fn outcome_of(text: &str, part: &str) -> String {
    let line = text.lines().find(|l| l.contains(&format!("part {part} : TestResult"))).unwrap_or_else(|| panic!("no {part} in {text}"));
    let start = line.find("VerdictKind::").expect("an outcome") + "VerdictKind::".len();
    line[start..].chars().take_while(char::is_ascii_alphabetic).collect()
}

fn gate(root: &Path, test: &str, evidence: &str) -> (bool, String) {
    agent(root, &["append-gate-result", "--file", FILE, "--gate", test, "--sha", "abc1234", "--verdict", "pass", "--judged-by", "ai", "--judged-at", "2026-09-11", "--evidence", evidence])
}

/// D0388 pair, named before the tree is read. Positive: `keel version` (this binary, exit 0) replays
/// green. Negative: `cmd /C exit 3` replays red and the fail names the command and `exit 3`.
#[test]
fn a_replayable_demo_receipt_stays_a_pass_and_demos_replays_it_recording_each_verdict() {
    let root = project("p", true);
    let (ok, out) = gate(&root, "dGreen", "keel version");
    assert!(ok, "{out}");
    let (ok, out) = gate(&root, "dRed", &fail_command());
    assert!(ok, "{out}");
    let (ok, out) = gate(&root, "dTold", "watched the render and it looked right");
    assert!(ok, "{out}");
    let (ok, out) = gate(&root, "iGate", "keel version");
    assert!(ok, "{out}");
    let t = file_text(&root);
    assert_eq!(outcome_of(&t, "dGreenR1"), "pass", "a replayable receipt keeps the pass: {t}");
    assert_eq!(outcome_of(&t, "dRedR1"), "pass", "replayable is a shape, not a verdict - the replay decides: {t}");
    assert_eq!(outcome_of(&t, "dToldR1"), "proposed", "prose on a demo is testimony: {t}");
    assert_eq!(outcome_of(&t, "iGateR1"), "proposed", "a command on an inspect does not repeat the looking: {t}");

    // the census sees the split before anything is replayed: two replayable passes, one demo proposal
    let (ok, json) = agent(&root, &["attestation", ".", "--json"]);
    assert!(ok, "{json}");
    assert!(json.contains("\"demoReplayable\":2,\"demoProposed\":1"), "{json}");

    // --demos re-runs the two receipts and records each verdict at HEAD
    let (ok, out) = agent(&root, &["reverify", "--demos", "--by", "ai", "."]);
    assert!(!ok, "one replay fails, so the exit is 1: {out}");
    assert!(out.contains("re-running 2 replayable demo receipt(s)"), "{out}");
    assert!(out.contains("dGreen pass - `keel version`"), "{out}");
    assert!(out.contains("dRed fail - `") && out.contains("1 replayed green, 1 failed"), "{out}");
    let t = file_text(&root);
    assert_eq!(outcome_of(&t, "dGreenR2"), "pass", "{t}");
    assert_eq!(t.matches("// RAN: keel version").count(), 3, "the same receipt is written on the fresh pass (two on dGreen, one on iGate): {t}");
    assert_eq!(outcome_of(&t, "dRedR2"), "fail", "{t}");
    let fail_line = t.lines().find(|l| l.contains("// RAN: replay of")).unwrap_or_else(|| panic!("the fail names its command: {t}"));
    assert!(fail_line.contains(&fail_command()) && fail_line.contains("-> exit 3"), "{fail_line}");
    assert!(!t.contains("dToldR2"), "a prose demo is never replayed: {t}");
    assert!(!t.contains("iGateR2"), "an inspect is never replayed: {t}");

    // after the replay the red demo's latest result is a fail, so it leaves the pool; the green one stays
    let (ok, json) = agent(&root, &["attestation", ".", "--json"]);
    assert!(ok, "{json}");
    assert!(json.contains("\"demoReplayable\":1,\"demoProposed\":1"), "{json}");
    let _ = std::fs::remove_dir_all(&root);
}

/// No `[demo]` section: the flag is inert (exit 2, nothing written) and every AI demo pass is a
/// proposal, command receipt or not - a project that never adopted the rule has adopted nothing.
#[test]
fn without_a_demo_section_every_demo_pass_is_a_proposal_and_demos_is_inert() {
    let root = project("n", false);
    let (ok, out) = gate(&root, "dGreen", "keel version");
    assert!(ok, "{out}");
    let t = file_text(&root);
    assert_eq!(outcome_of(&t, "dGreenR1"), "proposed", "{t}");
    let out = Command::new(keel_bin())
        .args(["reverify", "--demos", "--by", "ai", "."])
        .current_dir(&root)
        .env("KEEL_ACTOR", "ai")
        .stdin(Stdio::null())
        .output()
        .expect("keel runs");
    assert_eq!(out.status.code(), Some(2), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(String::from_utf8_lossy(&out.stderr).contains("declares no [demo] replayable prefixes"));
    assert_eq!(file_text(&root), t, "an inert flag writes nothing");
    let (ok, json) = agent(&root, &["attestation", ".", "--json"]);
    assert!(ok, "{json}");
    assert!(json.contains("\"demoReplayable\":0,\"demoProposed\":1"), "{json}");
    let _ = std::fs::remove_dir_all(&root);
}
