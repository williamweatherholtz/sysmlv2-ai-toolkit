//! D0315 / issue359 (GH#53): `keel accept` typed at the human's own terminal records a note that
//! satisfies the delegated-acceptance substance rule by construction - the TTY is the gesture, and
//! the command cites it. The agent-session path (no TTY) is unchanged: it still needs the human's
//! quoted words (D0289). `KEEL_TTY_GESTURE=1` is the test's stand-in for an interactive stdin.

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

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
}

fn fixture(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-ttyaccept-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir");
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    // The machine is bound to the AGENT, as this repository's is: the human at their terminal on
    // this machine passes --by, so the recorder is the agent binding and the record reads as
    // delegated - the exact GH#53 shape, where the rule then wanted a quote.
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    std::fs::write(root.join(".engine").join("contracts").join("attestation-policy.toml"), "[decisionAcceptance]\ndelegatedRecording = \"d0192\"\n").expect("policy");
    std::fs::write(root.join(".tracking").join("actors.sysml"), "package Actors {\n    private import EngineElement::*;\n    part hum : Person { :>> id = \"00000000-0000-4000-8000-000000000101\"; :>> title = \"hum\"; }\n    part claudeOpus5 : Actor { :>> id = \"00000000-0000-4000-8000-000000000102\"; :>> title = \"claudeOpus5\"; :>> kind = ActorKind::ai; }\n}\n").expect("actors");
    std::fs::write(
        root.join(".engine").join("decisions").join("0001-probe.sysml"),
        "package Decision0001 {\n    private import EngineElement::*;\n    part d0001 : Decision {\n        :>> id = \"00000000-0000-4000-8000-000000000001\";\n        :>> title = \"probe\";\n        :>> createdAt = \"2026-09-01\";\n        :>> createdBy = \"claudeOpus5\";\n        :>> status = DecisionStatus::proposed;\n        :>> context = \"c\";\n        :>> decision = \"do exactly this\";\n        :>> rationale = \"r\";\n        :>> consequences = \"q\";\n    }\n}\n",
    )
    .expect("decision");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

fn accept(root: &Path, tty: bool) -> (bool, String) {
    let mut cmd = Command::new(keel_bin());
    cmd.args(["accept", "d0001", "--note", "looks right, go ahead", "--date", "2026-09-05", "--by", "hum"]).current_dir(root).env("KEEL_ACTOR", "claudeOpus5").env("CLAUDECODE", "1");
    if tty {
        cmd.env("KEEL_TTY_GESTURE", "1");
    }
    let out = cmd.output().expect("accept");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

#[test]
fn a_plain_sentence_typed_at_a_terminal_records_a_note_that_cites_the_gesture() {
    let root = fixture("tty");
    let (ok, out) = accept(&root, true);
    assert!(ok, "the human's own terminal is the gesture; the plain sentence must be accepted: {out}");
    let text = std::fs::read_to_string(root.join(".engine").join("decisions").join("0001-probe.sysml")).expect("read");
    assert!(text.contains("looks right, go ahead - TTY gesture (asserted by KEEL_TTY_GESTURE, not observed): typed at a terminal by hum, 2026-09-05"), "the command cites the gesture in the record: {text}");
    // The predicate the six GH#53 acceptances failed (acceptQuotesDelegatedWords reads the Accept
    // Test's procedureText through this same function): the recorded text now passes it by construction.
    let recorded = text.split("verification d0001Accept : Test").nth(1).and_then(|r| r.split("procedureText = \"").nth(1)).and_then(|r| r.split('"').next()).expect("the acceptance Test's procedureText");
    assert!(keel_cli::view::note_quotes_human(recorded), "the substance rule accepts the cited gesture: {recorded}");
    assert!(!keel_cli::view::note_quotes_human("looks right, go ahead"), "and would have refused the bare sentence - the GH#53 shape");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_agent_session_path_is_unchanged_and_still_wants_the_humans_quoted_words() {
    let root = fixture("agent");
    let (ok, out) = accept(&root, false);
    assert!(!ok, "no TTY and no quote: the agent session must be refused as before (D0289): {out}");
    assert!(out.contains("QUOTE their words verbatim"), "the refusal names the remedy: {out}");
    let _ = std::fs::remove_dir_all(&root);
}
