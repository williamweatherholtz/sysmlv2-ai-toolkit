//! D0308 / issue341: an acceptance binds to a commit SHA, and the guard `acceptance-binds-to-text`
//! fails when the Decision's signed fields at HEAD differ from their text at that SHA. The remedy for a
//! legitimate edit is `keel accept <d> --rebind`, which records a new acceptance result against the SHA
//! whose text is current - never an edit of the acceptance. Both directions are exercised, and the
//! re-binding path too: a guard that only ever fails is a wall, not a gate.

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

fn commit_all(root: &Path, msg: &str) -> String {
    git(root, &["add", "-A"]);
    git(root, &["-c", "user.email=p@x", "-c", "user.name=p", "commit", "-q", "-m", msg]);
    git(root, &["rev-parse", "--short", "HEAD"])
}

fn guard(root: &Path) -> (i32, String) {
    let out = Command::new(keel_bin()).args(["guard", "acceptance-binds-to-text", "."]).current_dir(root).env("KEEL_ACTOR", "claudeOpus5").output().expect("guard");
    (out.status.code().unwrap_or(-1), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

#[test]
fn editing_an_accepted_decisions_text_is_red_reverting_is_green_and_rebinding_clears_a_real_edit() {
    let root = std::env::temp_dir().join(format!("keel-bindtext-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir");
    // The test process is an agent session (no TTY, harness markers), so recording the human's
    // acceptance needs the D0192 delegation, as it does in this repository; the note quotes them.
    std::fs::write(root.join(".engine").join("contracts").join("attestation-policy.toml"), "[decisionAcceptance]\ndelegatedRecording = \"d0192\"\n").expect("policy");
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    std::fs::write(root.join(".tracking").join("actors.sysml"), "package Actors {\n    private import EngineElement::*;\n    part hum : Person { :>> id = \"00000000-0000-4000-8000-000000000101\"; :>> title = \"hum\"; }\n}\n").expect("actors");
    let dec = root.join(".engine").join("decisions").join("0001-probe.sysml");
    let body = |decision: &str| format!(
        "package Decision0001 {{\n    private import EngineElement::*;\n    part d0001 : Decision {{\n        :>> id = \"00000000-0000-4000-8000-000000000001\";\n        :>> title = \"probe\";\n        :>> createdAt = \"2026-09-01\";\n        :>> createdBy = \"hum\";\n        :>> status = DecisionStatus::proposed;\n        :>> context = \"c\";\n        :>> decision = \"{decision}\";\n        :>> rationale = \"r\";\n        :>> consequences = \"q\";\n    }}\n}}\n"
    );
    std::fs::write(&dec, body("do exactly this")).expect("decision");
    git(&root, &["init", "-q", "."]);
    let c1 = commit_all(&root, "proposed");
    // Accept it (as the human, at their own terminal: judge == recorder).
    let out = Command::new(keel_bin()).args(["accept", "d0001", "--note", "their words: 'yes, do exactly this'", "--date", "2026-09-02", "--by", "hum"]).current_dir(&root).env("KEEL_ACTOR", "hum").output().expect("accept");
    assert!(out.status.success(), "accept: {}", String::from_utf8_lossy(&out.stderr));
    commit_all(&root, "accepted");
    let (code, out) = guard(&root);
    assert_eq!(code, 0, "an accepted Decision whose text is unchanged is GREEN: {out}");
    assert!(out.contains("1 scanned"), "{out}");

    // Someone edits the signed text after the fact.
    let now = std::fs::read_to_string(&dec).expect("read");
    std::fs::write(&dec, now.replace("do exactly this", "do exactly this, and also that")).expect("edit");
    let (code, out) = guard(&root);
    assert_ne!(code, 0, "an edited signed field must be RED: {out}");
    assert!(out.contains("d0001") && out.contains("decision changed") && out.contains(&c1) && out.contains("--rebind"), "the violation names the Decision, the field, the binding SHA and the remedy: {out}");

    // Revert: green again.
    std::fs::write(&dec, now.clone()).expect("revert");
    assert_eq!(guard(&root).0, 0, "restoring the signed words is green");

    // A real, kept edit is re-bound on a human judgment - the new result is the binding.
    std::fs::write(&dec, now.replace("do exactly this", "do exactly this, and also that")).expect("edit again");
    let c3 = commit_all(&root, "edited");
    assert_ne!(guard(&root).0, 0);
    let out = Command::new(keel_bin()).args(["accept", "d0001", "--rebind", "--note", "their words: 'yes, and also that'", "--date", "2026-09-04", "--by", "hum"]).current_dir(&root).env("KEEL_ACTOR", "hum").output().expect("rebind");
    assert!(out.status.success(), "rebind: {}", String::from_utf8_lossy(&out.stderr));
    let text = std::fs::read_to_string(&dec).expect("read");
    assert!(text.contains("d0001AcceptR2") && text.contains(&c3) && text.contains("REBOUND"), "a second acceptance result bound to the edited text's SHA:\n{text}");
    assert!(text.contains("d0001AcceptR1"), "the first acceptance stands");
    assert_eq!(guard(&root).0, 0, "after re-binding the guard is green");

    // D0329 (found while correcting D0234, issue283): in a gated repository the corrected text and
    // its re-binding must land in ONE commit - the gate will not let the edit through alone - so the
    // re-binding's judgedAgainst names the tree BEFORE the edit. Uncommitted: pending, not drift.
    // Committed together: the binding is the commit that carried both.
    let now = std::fs::read_to_string(&dec).expect("read");
    std::fs::write(&dec, now.replace("and also that", "and also that, corrected")).expect("edit again, uncommitted");
    let out = Command::new(keel_bin()).args(["accept", "d0001", "--rebind", "--note", "their words: 'yes, and also that, corrected'", "--date", "2026-09-05", "--by", "hum"]).current_dir(&root).env("KEEL_ACTOR", "hum").output().expect("rebind");
    assert!(out.status.success(), "rebind: {}", String::from_utf8_lossy(&out.stderr));
    let (code, out) = guard(&root);
    assert_eq!(code, 0, "an uncommitted re-binding is PENDING, not drift: {out}");
    assert!(out.contains("not committed yet"), "and the guard says so: {out}");
    let c4 = commit_all(&root, "corrected text and its re-binding, one commit");
    let (code, out) = guard(&root);
    assert_eq!(code, 0, "the commit that carried text and re-binding together is the binding ({c4}): {out}");
    assert!(!out.contains("not committed yet"), "{out}");
    let _ = std::fs::remove_dir_all(&root);
}
