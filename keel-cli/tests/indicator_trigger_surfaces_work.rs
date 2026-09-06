//! D0333 / dcUngroundedRatioTriggers: the rootedness ungrounded ratio is a declared indicator with a
//! trigger threshold, and `orient`'s burndown surfaces the derivation work when it is crossed - and
//! surfaces nothing when it is not. Both directions on the binary: a fresh scaffold (no delivery
//! Stories, ratio 0) triggers nothing; a fixture with three Stories all chartered by a Decision that
//! reaches no Need (ratio 100) triggers the work the contract names.

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

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("t{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    assert!(root.join(".engine/contracts/indicator-triggers.toml").is_file(), "the trigger contract ships with the engine");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    root
}

fn burndown(root: &Path) -> serde_json::Value {
    let out = Command::new(keel_bin()).args(["orient", "."]).current_dir(root).env("KEEL_ACTOR", "ai").output().expect("keel runs");
    assert!(out.status.success(), "orient: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    // orient's JSON is followed by advisory lines; the JSON object ends at the last closing brace
    let json = stdout.rfind('}').map_or(stdout.as_ref(), |i| &stdout[..=i]);
    let v: serde_json::Value = serde_json::from_str(json).unwrap_or_else(|e| panic!("orient is JSON: {e}: {stdout}"));
    v["burndown"].clone()
}

#[test]
fn below_the_threshold_nothing_is_surfaced_and_above_it_the_derivation_work_is() {
    let root = scaffold("trig");
    let b = burndown(&root);
    assert_eq!(b["ungrounded_ratio_pct"], 0, "a scaffold has no delivery Stories: {b}");
    assert_eq!(b["triggers"].as_array().map(Vec::len), Some(0), "nothing is surfaced below the threshold: {b}");

    // Three Stories chartered by a Decision that reaches no Need: ratio 100, past `above = 50`.
    std::fs::write(
        root.join(".engine/decisions/0001-x.sysml"),
        "package Decision0001 {\n    private import EngineElement::*;\n    part d0001 : Decision { :>> id = \"00000000-0000-4000-8000-000000000001\"; :>> title = \"x\"; :>> createdAt = \"2026-09-05\"; :>> createdBy = \"ai\"; :>> status = DecisionStatus::accepted; :>> context = \"c\"; :>> decision = \"d\"; :>> rationale = \"r\"; :>> consequences = \"q\"; }\n}\n",
    )
    .expect("decision");
    std::fs::write(
        root.join(".tracking/delivery.sysml"),
        "package Delivery {\n    private import EngineElement::*;\n    private import EngineWork::*;\n    part s1 : Story { :>> id = \"00000000-0000-4000-8000-000000000011\"; :>> title = \"one\"; :>> createdAt = \"2026-09-05\"; :>> createdBy = \"ai\"; }\n    part s2 : Story { :>> id = \"00000000-0000-4000-8000-000000000012\"; :>> title = \"two\"; :>> createdAt = \"2026-09-05\"; :>> createdBy = \"ai\"; }\n    part s3 : Story { :>> id = \"00000000-0000-4000-8000-000000000013\"; :>> title = \"three\"; :>> createdAt = \"2026-09-05\"; :>> createdBy = \"ai\"; }\n    #CharteredBy dependency from s1 to d0001;\n    #CharteredBy dependency from s2 to d0001;\n    #CharteredBy dependency from s3 to d0001;\n}\n",
    )
    .expect("stories");
    let b = burndown(&root);
    assert_eq!(b["ungrounded_ratio_pct"], 100, "{b}");
    let t = b["triggers"].as_array().expect("triggers");
    assert_eq!(t.len(), 1, "the crossed trigger is surfaced: {b}");
    assert_eq!(t[0]["indicator"], "ungroundedRatioIndicator");
    assert!(t[0]["surfaces"].as_str().is_some_and(|s| s.contains("latent-need derivation")), "and names the work: {b}");
    let _ = std::fs::remove_dir_all(&root);
}
