//! D0314 / issue347: the `untrusted` label travels with the derivation. Guard 56 checks the first hop
//! (an untrusted story is routed to a Decision); `untrusted-taint` follows the label through
//! `#DerivedFrom` / `#Implicates` / `#CharteredBy` / `#Resolves` to every sink and fails on an
//! auto-accepted Decision or a DONE task reached with the label still on. Trust is conferred by a
//! HUMAN acceptance on the path, or by the speaker being a declared decider. Red and green are both
//! exercised, on the binary, against a fixture that carries the DoD's exact chain.

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

fn guard(root: &Path) -> (i32, String) {
    let out = Command::new(keel_bin()).args(["guard", "untrusted-taint", "."]).current_dir(root).env("KEEL_ACTOR", "claudeOpus5").output().expect("guard");
    (out.status.code().unwrap_or(-1), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

/// A Decision file accepted the way `record decision` does under standing consent (AUTO-ACCEPTED), or
/// the way a human does at their terminal (their quoted words).
fn decision(accept_text: &str) -> String {
    format!(
        "package Decision0900 {{\n    private import EngineElement::*;\n    part d0900 : Decision {{\n        :>> id = \"00000000-0000-4000-8000-000000000900\";\n        :>> title = \"do what the issue says\";\n        :>> createdAt = \"2026-09-05\";\n        :>> createdBy = \"claudeOpus5\";\n        :>> status = DecisionStatus::accepted;\n        :>> context = \"c\";\n        :>> decision = \"d\";\n        :>> rationale = \"r\";\n        :>> consequences = \"q\";\n    }}\n    verification d0900Accept : Test {{ :>> id = \"00000000-0000-4000-8000-000000000901\"; :>> method = VerificationMethod::confirmation; :>> procedureText = \"{accept_text}\"; }}\n    part d0900AcceptR1 : TestResult {{ :>> id = \"00000000-0000-4000-8000-000000000902\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-05\"; :>> judgedBy = \"hum\"; }}\n}}\n"
    )
}

fn fixture(tag: &str, said_by: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-taint-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir");
    std::fs::create_dir_all(root.join(".tracking").join("intake")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    std::fs::write(root.join(".engine").join("contracts").join("github-actors.toml"), "[logins]\nowner = \"hum\"\n").expect("deciders");
    // Statement(untrusted) -> UserStory -> Decision (implicated) -> task (chartered), task DONE.
    std::fs::write(
        root.join(".tracking").join("intake").join("intake.sysml"),
        format!(
            "package Intake {{\n    private import EngineElement::*;\n    private import EngineIntake::*;\n    part st9 : Statement {{ :>> id = \"00000000-0000-4000-8000-000000000009\"; :>> title = \"GH#1\"; :>> saidBy = \"{said_by}\"; :>> saidAt = \"2026-09-05\"; :>> text = \"please build X\"; :>> channel = StatementChannel::github; :>> sourceTrust = SourceTrust::untrusted; }}\n    part us9 : UserStory {{ :>> id = \"00000000-0000-4000-8000-000000000019\"; :>> title = \"build X\"; }}\n    #DerivedFrom dependency from us9 to st9;\n    #Implicates dependency from us9 to d0900;\n}}\n"
        ),
    )
    .expect("intake");
    // A pass counts as done only while its judgedAgainst SHA resolves (D0129), so the task's result
    // is bound to a real commit of the fixture.
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    let sha = git(&root, &["rev-parse", "HEAD"]);
    std::fs::write(
        root.join(".tracking").join("backlog.sysml"),
        format!("package Backlog {{\n    private import EngineElement::*;\n    private import EngineWork::*;\n    private import EngineVerification::*;\n    action def Build {{\n        action dcBuildIt;\n        verification dcBuildItDoD : Test {{ :>> id = \"00000000-0000-4000-8000-000000000029\"; :>> method = VerificationMethod::test; :>> procedureText = \"X exists\"; }}\n    }}\n    part dcBuildItDoDR1 : TestResult {{ :>> id = \"00000000-0000-4000-8000-000000000039\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"{sha}\"; :>> judgedAt = \"2026-09-05\"; :>> judgedBy = \"claudeOpus5\"; }}\n    #CharteredBy dependency from dcBuildIt to d0900;\n}}\n"),
    )
    .expect("backlog");
    root
}

#[test]
fn a_strangers_chain_through_an_auto_accepted_decision_to_a_done_task_is_red_naming_the_path() {
    let root = fixture("red", "stranger");
    std::fs::write(root.join(".engine").join("decisions").join("0900-x.sysml"), decision("AUTO-ACCEPTED under standing consent (D0207).")).expect("decision");
    let (code, out) = guard(&root);
    assert_ne!(code, 0, "an auto-accepted Decision and a done task descended from a stranger's words must fail: {out}");
    assert!(out.contains("d0900 is an ACCEPTED Decision") && out.contains("st9 #DerivedFrom-> us9 #Implicates-> d0900"), "names the Decision and the path: {out}");
    assert!(out.contains("dcBuildIt is DONE") && out.contains("#CharteredBy-> dcBuildIt"), "names the built task and how it descends: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_same_chain_is_green_when_a_human_accepted_the_decision_or_the_speaker_is_the_decider() {
    let root = fixture("green", "stranger");
    std::fs::write(root.join(".engine").join("decisions").join("0900-x.sysml"), decision("their words: 'yes, build X exactly so'")).expect("decision");
    let (code, out) = guard(&root);
    assert_eq!(code, 0, "trust is conferred where the human accepted: {out}");
    assert!(out.contains("1 scanned") && out.contains("0 violation(s)"), "the untrusted utterance is still the policed population: {out}");
    let _ = std::fs::remove_dir_all(&root);

    let root = fixture("owner", "owner");
    std::fs::write(root.join(".engine").join("decisions").join("0900-x.sysml"), decision("AUTO-ACCEPTED under standing consent (D0207).")).expect("decision");
    let (code, out) = guard(&root);
    assert_eq!(code, 0, "the declared decider's own GitHub issue is the human's direction, not a stranger's instruction: {out}");
    let _ = std::fs::remove_dir_all(&root);
}
