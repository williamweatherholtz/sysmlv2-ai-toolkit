//! `keel judge-set` (D0443 on D0312 B): a human's verdict on the SAMPLED proposed results of one file,
//! recorded as one `TestResult` and one `<test>Attest<N>` quote receipt PER ITEM - never a count
//! (issue158). The channel rules are `keel accept`'s under the `confirmationRecord` delegation:
//!   1. delegation declared + quoted words -> every sampled item gets its own result and receipt, the
//!      sample is the policy's share in result-uuid order, `--all` judges the rest, `--fail` names failures;
//!   2. delegation declared + no quote     -> refused, nothing written, a `judge-set:no-quote` ledger line;
//!   3. delegation absent                  -> refused as `no recording delegation`.
//!
//! `keel show attestation --json` reports proposed / sampled / judged / awaiting from the same tree.

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
    let root = base.join(format!("js{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    root
}

const FILE: &str = ".tracking/delivery/s668probe.sysml";

/// Four proposals whose result uuids sort t2, t0, t3, t1 (leading hex 1, 4, 7, b), a proposal-free
/// pass on `pDoD`, and nothing a human has judged.
fn proposals_file() -> String {
    let mut s = String::from("package S668Probe {\n    private import EngineElement::*;\n    private import EngineVerification::*;\n");
    for (i, lead) in ["4", "b", "1", "7"].iter().enumerate() {
        s.push_str(&format!(
            "    verification t{i}Gate : Test {{ :>> id = \"e2e0000{i}-0000-4000-8000-00000000000a\"; :>> title = \"t{i}\"; :>> createdAt = \"2026-09-10\"; :>> createdBy = \"ai\"; :>> method = VerificationMethod::inspect; :>> procedureText = \"gate t{i}\"; }}\n    part t{i}GateR1 : TestResult {{ :>> id = \"{lead}2e0000{i}-0000-4000-8000-00000000000b\"; :>> outcome = VerdictKind::proposed; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-10\"; :>> judgedBy = \"ai\"; }}\n"
        ));
    }
    s.push_str("    verification pDoD : Test { :>> id = \"e2e00009-0000-4000-8000-00000000000a\"; :>> title = \"p\"; :>> createdAt = \"2026-09-10\"; :>> createdBy = \"ai\"; :>> method = VerificationMethod::test; :>> procedureText = \"dod\"; }\n");
    s.push_str("    part pDoDR1 : TestResult { :>> id = \"e2e00009-0000-4000-8000-00000000000b\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-10\"; :>> judgedBy = \"ai\"; }\n");
    s.push_str("}\n");
    s
}

/// issue376: a fresh scaffold ships every grant line commented out. This fixture's human delegates the
/// RECORDING of confirmations (D0198) and sets the sampling share (D0443).
fn project_with_proposals(tag: &str, delegate: bool) -> PathBuf {
    let root = shallow_root(tag);
    assert!(agent(&root, &["init", "."]).0, "scaffold");
    let p = root.join(".engine/contracts/attestation-policy.toml");
    let text = std::fs::read_to_string(&p).expect("policy");
    let mut granted = text.replace("# sampling = \"50%\"", "sampling = \"50%\"");
    assert_ne!(text, granted, "the commented sampling line ships with the scaffold");
    if delegate {
        let with = granted.replace("# delegatedRecording = \"d0198\"", "delegatedRecording = \"d0198\"");
        assert_ne!(granted, with, "the commented confirmationRecord delegation ships");
        granted = with;
    }
    std::fs::write(&p, granted).expect("grant");
    let f = root.join(FILE);
    std::fs::create_dir_all(f.parent().expect("dir")).expect("mkdir");
    std::fs::write(&f, proposals_file()).expect("fixture");
    root
}

fn file_text(root: &Path) -> String {
    std::fs::read_to_string(root.join(FILE)).expect("read")
}

#[test]
fn quoted_words_record_one_result_and_one_receipt_per_sampled_item() {
    let root = project_with_proposals("q", true);
    let (ok, text) = agent(&root, &["show", "attestation", ".", "--json"]);
    assert!(ok, "{text}");
    assert!(text.contains("\"proposals\":{\"proposed\":4,\"sampled\":2,\"judged\":0,\"awaiting\":4,\"demoReplayable\":0,\"demoProposed\":0,\"sampling\":\"50%\"}"), "the census computes the set before anyone judges it: {text}");

    // 50% of four = two, in result-uuid order: t2 (lead 1) and t0 (lead 4). t0 fails, t2 passes.
    let (ok, text) = agent(&root, &["judge-set", FILE, "--words", "the sampled set passes, except t0Gate which does not do what its title says", "--fail", "t0Gate", "--by", "you", "--date", "2026-09-11"]);
    assert!(ok, "delegation declared + quoted words must record: {text}");
    assert!(text.contains("delegation d0198"), "the record cites the delegation it acts under: {text}");
    assert!(text.contains("t2GateR2: pass") && text.contains("t0GateR2: fail"), "the two sampled items, each with its verdict: {text}");
    assert!(!text.contains("t1Gate") && !text.contains("t3Gate"), "the unsampled items are untouched: {text}");
    assert!(!text.contains("WARN"), "no Decision read-back applies to a file: {text}");
    let t = file_text(&root);
    assert!(t.contains("part t2GateR2 : TestResult") && t.contains("part t0GateR2 : TestResult"), "{t}");
    assert!(t.contains(":>> outcome = VerdictKind::fail; :>> judgedAgainst = ") && t.contains(":>> judgedBy = \"you\"; :>> createdBy = \"ai\";"), "the human judged, the agent recorded:\n{t}");
    assert_eq!(t.matches("Attest1 : Test").count(), 2, "one quote receipt per item, never one for the set:\n{t}");
    assert_eq!(t.matches("except t0Gate which does not do what its title says").count(), 2, "each receipt quotes the human verbatim:\n{t}");
    assert_eq!(t.matches("VerdictKind::proposed").count(), 4, "the proposals stand; the judgment is appended, never rewritten");

    // the census now reads 2 judged; the sample is the judged two; two await --all
    let (_, text) = agent(&root, &["show", "attestation", ".", "--json"]);
    assert!(text.contains("\"proposals\":{\"proposed\":4,\"sampled\":2,\"judged\":2,\"awaiting\":2,\"demoReplayable\":0,\"demoProposed\":0,\"sampling\":\"50%\"}"), "{text}");
    // the sample is judged: a second plain judge-set finds nothing and writes nothing
    let before = file_text(&root);
    let (ok, text) = agent(&root, &["judge-set", FILE, "--words", "and the rest pass too, the whole probe file", "--by", "you", "--date", "2026-09-11"]);
    assert!(!ok && text.contains("nothing awaits judgment") && text.contains("--all"), "{text}");
    assert_eq!(before, file_text(&root));
    // --all judges the remaining two
    let (ok, text) = agent(&root, &["judge-set", FILE, "--words", "and the rest pass too, the whole probe file", "--by", "you", "--date", "2026-09-11", "--all"]);
    assert!(ok, "{text}");
    assert!(text.contains("t3GateR2: pass") && text.contains("t1GateR2: pass"), "{text}");
    let (_, text) = agent(&root, &["show", "attestation", ".", "--json"]);
    assert!(text.contains("\"proposals\":{\"proposed\":4,\"sampled\":4,\"judged\":4,\"awaiting\":0,\"demoReplayable\":0,\"demoProposed\":0,\"sampling\":\"50%\"}"), "{text}");

    // what was written is a tree the authority and the confirmation rules accept
    let (ok, text) = agent(&root, &["gate", "validate", "."]);
    assert!(ok, "the written lines validate: {text}");
    let (ok, text) = agent(&root, &["gate", "guard", "confirmation-authenticity", "."]);
    assert!(ok, "each item's receipt is a confirmation the substance rule accepts: {text}");
    let (ok, text) = agent(&root, &["gate", "guard", "attestation-substance", "."]);
    assert!(ok, "{text}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_unquoted_note_is_refused_and_leaves_a_ledger_line() {
    let root = project_with_proposals("nq", true);
    let before = file_text(&root);
    let (ok, text) = agent(&root, &["judge-set", FILE, "--note", "the human said the set was fine", "--by", "you", "--date", "2026-09-11"]);
    assert!(!ok, "no quote, no record: {text}");
    assert!(text.contains("QUOTE"), "the refusal names the missing receipt: {text}");
    assert_eq!(before, file_text(&root), "nothing was written");
    let ledger = std::fs::read_to_string(root.join(".keel").join("metrics").join("hooks.jsonl")).expect("a refused write leaves a ledger line");
    let refused: Vec<&str> = ledger.lines().filter(|l| l.contains(r#""event":"refused""#)).collect();
    assert_eq!(refused.len(), 1, "one refusal, one line: {ledger}");
    assert!(refused[0].contains(r#""control":"judge-set:no-quote""#), "the line names the check: {}", refused[0]);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn without_the_delegation_even_quoted_words_are_refused() {
    let root = project_with_proposals("nd", false);
    let before = file_text(&root);
    let (ok, text) = agent(&root, &["judge-set", FILE, "--words", "the sampled set passes, all of the probe file", "--by", "you", "--date", "2026-09-11"]);
    assert!(!ok, "{text}");
    assert!(text.contains("no recording delegation for confirmationRecord"), "the refusal names the class it looked for: {text}");
    assert_eq!(before, file_text(&root));
    let _ = std::fs::remove_dir_all(&root);
}
