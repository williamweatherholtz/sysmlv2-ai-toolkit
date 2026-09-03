//! `keel show control-structure` is COMPUTED from the facts that wire the authorities (D0284, st066).
//!
//! Two properties the DoD names:
//!   1. a fresh `keel init` project computes the same structure - every controller role has actions
//!      derived from its scaffolded hooks, git hooks, workflow and CLI facts - with NO authored
//!      anchors, because it has authored none;
//!   2. this repository, which HAS authored the residue, computes the same actions and decorates the
//!      roles with the anchors and process models.
//!
//! Plus the derivation is live: removing a hook event from the scaffold's settings.json removes its
//! action on the next run, with no model file edited.

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn run(dir: &Path, args: &[&str]) -> (bool, String) {
    // KEEL_OFFLINE: the remote row is fetched live in normal use; a test must not depend on the network.
    let out = Command::new(keel_bin()).args(args).current_dir(dir).env("KEEL_OFFLINE", "1").output().expect("keel runs");
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn shallow_root(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("cs{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    root
}

fn parse(text: &str) -> serde_json::Value {
    serde_json::from_str(text).unwrap_or_else(|e| panic!("the view is JSON: {e}
{text}"))
}
fn names(v: &serde_json::Value, key: &str) -> Vec<String> {
    v[key].as_array().expect(key).iter().map(|x| x["name"].as_str().unwrap_or("").to_string()).collect()
}
fn anchors(v: &serde_json::Value) -> Vec<Option<String>> {
    v["controllers"].as_array().expect("controllers").iter().map(|c| c["anchor"].as_str().map(str::to_string)).collect()
}

#[test]
fn a_fresh_project_computes_its_structure_with_no_authored_anchors() {
    let root = shallow_root("fresh");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    let (ok, text) = run(&root, &["show", "control-structure", "."]);
    assert!(ok, "the view runs on a fresh project: {text}");
    let v = parse(&text);
    // Every role is derived: hooks from settings.json, commit gate from .githooks, ci from the scaffolded
    // workflow, agent from the CLI facts, human from record statement, console from serve. Two roles may
    // read inert on a scaffold, and both readings are TRUE: the remote's refusal is a live fetch (offline
    // here), and the decision channel is a unit a project IMPORTS - a fresh scaffold has no channel.
    let inert: Vec<&str> = v["inertControllers"].as_array().expect("inert").iter().filter_map(|x| x.as_str()).collect();
    assert!(inert.iter().all(|r| *r == "remote" || *r == "channel"), "only remote/channel may be inert on a scaffold: {inert:?}");
    assert!(inert.contains(&"channel"), "a scaffold has not imported the decision channel, and the view must say so: {inert:?}");
    let acts = names(&v, "actions");
    let fbs = names(&v, "feedback");
    assert!(acts.iter().any(|n| n == "hookStop"), "the Stop hook is a derived action: {acts:?}");
    assert!(acts.iter().any(|n| n == "githookPreCommit"), "pre-commit is a derived action: {acts:?}");
    assert!(acts.iter().any(|n| n == "cmdAddTask") && fbs.iter().any(|n| n == "readOrient"), "CLI facts yield actions and feedback");
    // and NOTHING authored: no anchors, no process models.
    assert!(anchors(&v).iter().all(Option::is_none), "a fresh project has no anchors: {:?}", anchors(&v));
    assert!(v["processModels"].as_array().expect("pm").is_empty(), "no process model was invented");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn removing_a_hook_event_removes_its_action_without_any_model_edit() {
    let root = shallow_root("hook");
    assert!(run(&root, &["init", "."]).0, "scaffold");
    let (_, before) = run(&root, &["show", "control-structure", "."]);
    assert!(names(&parse(&before), "actions").iter().any(|n| n == "hookUserPromptSubmit"), "precondition: the event is wired: {before}");
    let settings = root.join(".claude").join("settings.json");
    let mut v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&settings).expect("settings")).expect("json");
    v["hooks"].as_object_mut().expect("hooks").remove("UserPromptSubmit");
    std::fs::write(&settings, serde_json::to_string_pretty(&v).expect("json")).expect("write");
    let (_, after) = run(&root, &["show", "control-structure", "."]);
    let acts = names(&parse(&after), "actions");
    assert!(!acts.iter().any(|n| n == "hookUserPromptSubmit"), "the action follows the fact, not a model file: {acts:?}");
    assert!(acts.iter().any(|n| n == "hookStop"), "the other events remain");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn this_repository_decorates_the_roles_with_its_authored_residue() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root").to_path_buf();
    let (ok, text) = run(&repo, &["show", "control-structure", "."]);
    assert!(ok, "{text}");
    let v = parse(&text);
    let found: Vec<String> = anchors(&v).into_iter().flatten().collect();
    for anchor in ["ctHuman", "ctAgent", "ctHooks", "ctCommitGate", "ctCI", "ctRemote", "ctConsole"] {
        assert!(found.iter().any(|a| a == anchor), "{anchor} decorates its role: {found:?}");
    }
    // D0291: the decision channel is disconnected here, so its role has no anchor and is INERT - the
    // view says so rather than inventing a controller this project no longer has.
    assert!(!found.iter().any(|a| a == "ctChannel"), "no channel anchor after D0291: {found:?}");
    let inert: Vec<&str> = v["inertControllers"].as_array().expect("inert").iter().filter_map(|x| x.as_str()).collect();
    assert!(inert.contains(&"channel"), "the channel role reads inert: {inert:?}");
    let pms = names(&v, "processModels");
    assert!(pms.iter().any(|n| n == "pmHuman"), "process models are read from the residue: {pms:?}");
    assert!(text.contains("issue343"), "a false belief cites the Issue that makes it false");
    assert!(v["hazardsByProcess"].as_array().expect("hz").iter().any(|h| h["hazard"] == "ehz1"), "hazards attach to processes through the authored edges");
    assert!(names(&v, "actions").iter().any(|n| n == "humanDecidesOnChannel"), "the declared deciders yield the human's channel action");
    assert_eq!(v["remote"]["status"], "unverified: KEEL_OFFLINE set", "offline, the remote row says so instead of guessing");
}
