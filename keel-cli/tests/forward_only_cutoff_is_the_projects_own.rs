//! D0319 / issue352 (GH#47): a forward-only rule's cutoff is the ADOPTING project's date, never only the
//! engine's. The shipped `delegatedAcceptanceSubstanceRule` carries `acceptQuotesDelegatedWords(
//! 2026-08-22)` - this repository's own adoption of D0192 - and a downstream project scaffolded today
//! used to be retro-failed on acceptances it recorded before it ever had the rule. On a fresh scaffold
//! (declared today): an unquoted delegated acceptance judged BEFORE today passes; one judged today
//! fails. Run on the binary, against `keel init`'s real rules.

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

/// A delegated acceptance record (recorder `ai`, judge `you`) whose note is a bare sentence - the shape
/// the substance rule refuses after its cutoff.
fn decision(judged_at: &str) -> String {
    format!(
        "package Decision0001 {{\n    private import EngineElement::*;\n    part d0001 : Decision {{\n        :>> id = \"00000000-0000-4000-8000-000000000001\";\n        :>> title = \"probe\";\n        :>> createdAt = \"{judged_at}\";\n        :>> createdBy = \"ai\";\n        :>> status = DecisionStatus::accepted;\n        :>> context = \"c\";\n        :>> decision = \"d\";\n        :>> rationale = \"r\";\n        :>> consequences = \"q\";\n    }}\n    verification d0001Accept : Test {{ :>> id = \"00000000-0000-4000-8000-000000000002\"; :>> method = VerificationMethod::confirmation; :>> procedureText = \"they approved it in the meeting\"; }}\n    part d0001AcceptR1 : TestResult {{ :>> id = \"00000000-0000-4000-8000-000000000003\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"{judged_at}\"; :>> judgedBy = \"you\"; :>> createdBy = \"ai\"; }}\n}}\n"
    )
}

/// issue376: a fresh scaffold ships the policy with its GRANT lines commented out. This fixture's human
/// delegates the RECORDING of their acceptance (D0192) - the grant the tests below exercise.
fn grant_recording_delegation(root: &Path) {
    let p = root.join(".engine/contracts/attestation-policy.toml");
    let text = std::fs::read_to_string(&p).expect("policy");
    let granted = text.replace("# delegatedRecording = \"d0192\"", "delegatedRecording = \"d0192\"");
    assert_ne!(text, granted, "the commented delegation line ships");
    std::fs::write(&p, granted).expect("grant");
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("c{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    grant_recording_delegation(&root);
    let declared = std::fs::read_to_string(root.join(".engine/contracts/adoption-profile.toml")).expect("profile");
    assert!(declared.contains("declaredAt"), "init declares the adoption date: {declared}");
    root
}

#[test]
fn an_acceptance_recorded_before_the_project_adopted_the_rule_is_not_retro_failed() {
    let root = scaffold("before");
    std::fs::write(root.join(".engine/decisions/0001-probe.sysml"), decision("2026-08-30")).expect("decision");
    let (ok, out) = run(&root, &["guard", "confirmation-authenticity", "."]);
    assert!(
        ok && out.contains("0 violation(s)"),
        "judged 2026-08-30, after the ENGINE's 2026-08-22 cutoff but before THIS project adopted the rule today - the cutoff is the project's, so this is history, not a violation:\n{out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_unquoted_acceptance_after_adoption_still_fails() {
    let root = scaffold("after");
    let today = std::fs::read_to_string(root.join(".engine/contracts/adoption-profile.toml")).expect("profile").lines().find_map(|l| l.trim().strip_prefix("declaredAt")).and_then(|r| r.split('"').nth(1).map(str::to_string)).expect("declaredAt");
    std::fs::write(root.join(".engine/decisions/0001-probe.sysml"), decision(&today)).expect("decision");
    let (ok, out) = run(&root, &["guard", "confirmation-authenticity", "."]);
    assert!(!ok && out.contains("d0001") && out.contains("quote"), "judged on the adoption day with no quote receipt: the rule bites as before:\n{out}");
    let _ = std::fs::remove_dir_all(&root);
}
