//! A signed plan covers the steps it names (D0396 / D0375 option C).
//!
//! A process-change or safety-change Decision recorded with `plan: dNNNN` and `step: <name>` is
//! accepted AT RECORD TIME when the plan (a) exists, (b) was accepted by a human's OWN word - not
//! standing consent, judge a decider - and (c) names the step in its own text. Any clause failing
//! leaves it proposed, naming the clause. A plan cannot cover itself or a plan-covered Decision.

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

fn agent(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("CLAUDE_CODE_SESSION_ID", "00000000-0000-4000-8000-000000000009")
        .env("KEEL_ACTOR", "ai")
        .stdin(Stdio::null())
        .output()
        .expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("spc{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(agent(&root, &["init", "."]).0, "scaffold");
    // a declared human decider, so a plan's judge can be one
    let ga = root.join(".engine/contracts/github-actors.toml");
    let mut text = std::fs::read_to_string(&ga).unwrap_or_default();
    if !text.contains("[logins]") {
        text.push_str("\n[logins]\n");
    }
    text.push_str("humanlogin = \"human\"\n");
    std::fs::write(&ga, text).expect("deciders");
    // register `human` as a human actor (Person) so refuse_ai_judgment/kind_of accept it as the judge
    let actors = root.join(".tracking").join("actors.sysml");
    let mut at = std::fs::read_to_string(&actors).unwrap_or_else(|_| "package ProjectActors {\n    private import EngineElement::*;\n}\n".to_string());
    let close = at.rfind('}').expect("actors close");
    at.insert_str(close, "    part human : Person { :>> id = \"a1b2c3d4-0000-4000-8000-000000000abc\"; :>> name = \"Human\"; :>> email = \"h@x\"; }\n");
    std::fs::write(&actors, at).expect("actors");
    root
}

fn draft(root: &Path, name: &str, body: &str) -> PathBuf {
    let p = root.join(format!("{name}.md"));
    std::fs::write(&p, body).expect("draft");
    p
}

fn decisions_text(root: &Path) -> String {
    let dir = root.join(".engine").join("decisions");
    std::fs::read_dir(&dir).into_iter().flatten().flatten().map(|e| std::fs::read_to_string(e.path()).unwrap_or_default()).collect::<Vec<_>>().join("\n")
}

/// Record a plan Decision (non-marker so it records proposed), then accept it as the human by their word.
fn signed_plan(root: &Path, slug: &str, decision_text: &str) -> String {
    let d = draft(
        root,
        slug,
        &format!("slug: {slug}\ntitle: a plan\ndate: 2026-09-08\nauthor: ai\n\n--- context\nsome context here for the plan\n--- decision\n{decision_text}\n--- rationale\nbecause it is the plan\n--- consequences\nnothing yet\n"),
    );
    let (ok, out) = agent(root, &["record", "decision", "--from", d.to_str().unwrap()]);
    assert!(ok, "plan recorded: {out}");
    // the plan id is dNNNN in the output
    let id = out.split_whitespace().find(|w| w.starts_with("D0") && w.len() == 5).map(|w| w.to_lowercase()).expect("plan id");
    // the human accepts the plan themselves (their own word, delegated recording)
    let policy = root.join(".engine/contracts/attestation-policy.toml");
    let t = std::fs::read_to_string(&policy).unwrap();
    std::fs::write(&policy, t.replace("# delegatedRecording = \"d0192\"", "delegatedRecording = \"d0192\"")).unwrap();
    let note = format!("their words: 'yes, accept {id} - this is our plan'");
    let (ok, out) = agent(root, &["accept", &id, "--note", &note, "--by", "human", "--date", "2026-09-08"]);
    assert!(ok, "the human signs the plan: {out}");
    id
}

fn record_marker_covered(root: &Path, slug: &str, plan: &str, step: &str) -> (bool, String) {
    let d = draft(
        root,
        slug,
        &format!("slug: {slug}\ntitle: a covered step\nmarker: process-change\nplan: {plan}\nstep: {step}\ndate: 2026-09-08\nauthor: ai\n\n--- context\nthis implements one step of the plan\n--- decision\nNOT A FORK: do the step exactly as the plan enumerated\n--- rationale\nthe plan already weighed this\n--- consequences\none step done\n"),
    );
    agent(root, &["record", "decision", "--from", d.to_str().unwrap()])
}

#[test]
fn a_marker_naming_a_step_the_plan_carries_is_accepted_under_the_plans_judge() {
    let root = scaffold("cover");
    let plan = signed_plan(&root, "theplan", "NOT A FORK: the plan does stepAlpha and stepBeta, both named here");
    let (ok, out) = record_marker_covered(&root, "stepa", &plan, "stepAlpha");
    assert!(ok, "recording succeeds: {out}");
    assert!(out.contains("PLAN-COVERED") && out.contains(&plan) && out.contains("stepAlpha"), "accepted at record time under the plan: {out}");
    let d = decisions_text(&root);
    // the covered Decision is accepted with the plan's judge and no auto-accept token
    assert!(d.contains("PLAN-COVERED under") && d.contains("stepAlpha"), "the note carries the token and step:\n{out}");
    assert!(d.matches("judgedBy = \"human\"").count() >= 2, "both the plan and the covered step are judged by the human");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_step_the_plan_does_not_name_stays_proposed_naming_clause_c() {
    let root = scaffold("nostep");
    let plan = signed_plan(&root, "theplan", "NOT A FORK: the plan does only stepAlpha, named here");
    let (ok, out) = record_marker_covered(&root, "stepz", &plan, "stepZulu");
    assert!(ok, "the command exits 0 (recorded, proposed): {out}");
    assert!(out.contains("clause (c)") && out.contains("stepZulu"), "clause c named with the step: {out}");
    assert!(out.contains("chars"), "the searched text lengths are quoted: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_auto_accepted_plan_cannot_cover_a_step() {
    let root = scaffold("auto");
    // grant standing consent so a non-fork plan auto-accepts under it (not a human's own word)
    let policy = root.join(".engine/contracts/attestation-policy.toml");
    let t = std::fs::read_to_string(&policy).unwrap();
    // a project with standingConsent + standingWords auto-accepts a non-marker non-fork at record time
    if t.contains("# standingConsent") {
        std::fs::write(&policy, t.replace("# standingConsent = \"d0207\"", "standingConsent = \"d0207\"").replace("# standingWords = ", "standingWords = ")).unwrap();
    }
    let d = draft(
        &root,
        "autoplan",
        "slug: autoplan\ntitle: an auto plan\ndate: 2026-09-08\nauthor: ai\n\n--- context\ncontext for the auto plan\n--- decision\nNOT A FORK: the plan does stepAuto, named here\n--- rationale\nr\n--- consequences\nc\n",
    );
    let (_ok, out) = agent(&root, &["record", "decision", "--from", d.to_str().unwrap()]);
    // if it auto-accepted, its note has AUTO-ACCEPTED; a plan covered by it must be refused clause (b)
    if out.contains("AUTO-ACCEPTED") || decisions_text(&root).contains("AUTO-ACCEPTED") {
        let plan = out.split_whitespace().find(|w| w.starts_with("D0") && w.len() == 5).map(|w| w.to_lowercase()).expect("id");
        let (ok, cout) = record_marker_covered(&root, "stepauto", &plan, "stepAuto");
        assert!(ok, "recorded proposed: {cout}");
        assert!(cout.contains("clause (b)") && cout.contains("AUTO-ACCEPTED"), "an auto-accepted plan cannot cover: {cout}");
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_missing_plan_is_clause_a() {
    let root = scaffold("nopl");
    let (ok, out) = record_marker_covered(&root, "orphan", "d9999", "stepAny");
    assert!(ok, "recorded proposed: {out}");
    assert!(out.contains("clause (a)") && out.contains("d9999"), "a missing plan is clause a: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_guard_fails_when_the_plan_stops_naming_the_step() {
    let root = scaffold("guard");
    let plan = signed_plan(&root, "theplan", "NOT A FORK: the plan does stepGamma, named here");
    let (ok, _out) = record_marker_covered(&root, "stepg", &plan, "stepGamma");
    assert!(ok);
    // clean now
    let (ok, out) = agent(&root, &["gate", "guard", "plan-covers-step", "."]);
    assert!(ok && out.contains("0 violation(s)"), "the cover holds: {out}");
    // edit the plan's decision text to drop the step name
    let dir = root.join(".engine").join("decisions");
    let planfile = std::fs::read_dir(&dir).unwrap().flatten().map(|e| e.path()).find(|p| p.to_string_lossy().contains("theplan")).unwrap();
    let t = std::fs::read_to_string(&planfile).unwrap();
    std::fs::write(&planfile, t.replace("stepGamma", "stepDelta")).unwrap();
    let (_ok, out) = agent(&root, &["gate", "guard", "plan-covers-step", "."]);
    assert!(out.contains("FAIL") && out.contains("clause (c)"), "the guard fails when the plan no longer names the step: {out}");
    let _ = std::fs::remove_dir_all(&root);
}
