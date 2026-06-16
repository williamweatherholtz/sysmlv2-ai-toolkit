#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when, World};
use sysmlv2_cli::orient;

static DIR_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_root() -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("orient_bdd_{n}"))
}

/// Write a valid SysML package to `<root>/.tracking/tasks.sysml`.
///
/// `tasks`: (name, has_pass_result) — names with results get a `{name}DoDR1 : TestResult`.
/// Use empty `judgedAgainst` so git-SHA validation is bypassed in temp dirs.
/// `succs`: (pred, succ) succession edges.
fn write_sysml(root: &Path, tasks: &[(String, bool)], succs: &[(String, String)]) {
    let tracking = root.join(".tracking");
    std::fs::create_dir_all(&tracking).unwrap();
    let mut body = String::new();
    for (i, (name, done)) in tasks.iter().enumerate() {
        body.push_str(&format!("        action {name};\n"));
        if *done {
            let id = format!("{:08x}-0000-4000-8000-{:012x}", i + 1, i + 1);
            body.push_str(&format!(
                "        part {name}DoDR1 : TestResult {{ :>> id = \"{id}\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"\"; :>> judgedAt = \"2026-06-15\"; :>> judgedBy = \"test\"; }}\n"
            ));
        }
    }
    for (pred, succ) in succs {
        body.push_str(&format!("        first {pred} then {succ};\n"));
    }
    let content = format!("package TestOrientBdd {{\n    action def TestRun {{\n{body}    }}\n}}\n");
    std::fs::write(tracking.join("tasks.sysml"), content).unwrap();
}

/// Write a task with a NON-EMPTY invalid SHA so it lands in `invalidEvidence`.
fn write_sysml_invalid_sha(root: &Path, name: &str) {
    let tracking = root.join(".tracking");
    std::fs::create_dir_all(&tracking).unwrap();
    let body = format!(
        "        action {name};\n        part {name}DoDR1 : TestResult {{ :>> id = \"ffffffff-0000-4000-8000-000000000001\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"deadbeef00000000000000000000000000000001\"; :>> judgedAt = \"2026-06-15\"; :>> judgedBy = \"test\"; }}\n"
    );
    let content = format!("package TestOrientBdd {{\n    action def TestRun {{\n{body}    }}\n}}\n");
    std::fs::write(tracking.join("tasks.sysml"), content).unwrap();
}

#[derive(Debug, Default, World)]
pub struct OrientWorld {
    root: Option<PathBuf>,
    tasks: Vec<(String, bool)>,
    succs: Vec<(String, String)>,
    done: usize,
    outstanding: usize,
    ready: Vec<String>,
    invalid_evidence: Vec<String>,
}

impl Drop for OrientWorld {
    fn drop(&mut self) {
        if let Some(root) = &self.root {
            let _ = std::fs::remove_dir_all(root);
        }
    }
}

// ── given steps ───────────────────────────────────────────────────────────────

#[given(regex = r#"^a tracking dir with task "([^"]+)" and a passing result$"#)]
fn given_task_with_result(world: &mut OrientWorld, name: String) {
    if world.root.is_none() {
        world.root = Some(unique_root());
    }
    world.tasks.push((name, true));
}

#[given(regex = r#"^a tracking dir with tasks "([^"]+)" and "([^"]+)" where "([^"]+)" depends on "([^"]+)"$"#)]
fn given_two_tasks_with_dep(
    world: &mut OrientWorld,
    _a: String,
    _b: String,
    succ: String,
    pred: String,
) {
    if world.root.is_none() {
        world.root = Some(unique_root());
    }
    world.tasks.push((pred.clone(), false));
    world.tasks.push((succ.clone(), false));
    world.succs.push((pred, succ));
}

#[given(regex = r#"^task "([^"]+)" depends on "([^"]+)"$"#)]
fn given_task_depends(world: &mut OrientWorld, succ: String, pred: String) {
    world.tasks.push((succ.clone(), false));
    world.succs.push((pred, succ));
}

#[given(regex = r#"^a tracking dir with task "([^"]+)" and an invalid SHA result$"#)]
fn given_task_invalid_sha(world: &mut OrientWorld, name: String) {
    let root = unique_root();
    write_sysml_invalid_sha(&root, &name);
    world.root = Some(root);
}

#[given(regex = r#"^a tracking dir with an ordering-only edge from "([^"]+)" to "([^"]+)"$"#)]
fn given_ordering_only_edge(world: &mut OrientWorld, pred: String, succ: String) {
    let root = unique_root();
    let tracking = root.join(".tracking");
    std::fs::create_dir_all(&tracking).unwrap();
    let content = format!(
        "package OOBdd {{\n    action def OORun {{\n        action {pred};\n        action {succ};\n        #OrderingOnly first {pred} then {succ};\n    }}\n}}\n"
    );
    std::fs::write(tracking.join("oo.sysml"), content).unwrap();
    world.root = Some(root);
}

#[given(regex = r#"^a tracking dir with package-level task "([^"]+)" and result$"#)]
fn given_package_level_task(world: &mut OrientWorld, name: String) {
    let root = unique_root();
    let tracking = root.join(".tracking");
    std::fs::create_dir_all(&tracking).unwrap();
    let content = format!(
        "package PkgLevelBdd {{\n    action {name};\n    part {name}DoDR1 : TestResult {{ :>> id = \"bb000001-0000-4000-8000-000000000001\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"\"; :>> judgedAt = \"2026-06-15\"; :>> judgedBy = \"test\"; }}\n}}\n"
    );
    std::fs::write(tracking.join("pkglevel.sysml"), content).unwrap();
    world.root = Some(root);
}

#[given(regex = r#"^a tracking dir with a legacy-named result for "([^"]+)"$"#)]
fn given_legacy_result(world: &mut OrientWorld, name: String) {
    let root = unique_root();
    let tracking = root.join(".tracking");
    std::fs::create_dir_all(&tracking).unwrap();
    let body = format!(
        "        action {name};\n        part {name}R1 : TestResult {{ :>> id = \"aa000001-0000-4000-8000-000000000001\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"\"; :>> judgedAt = \"2026-06-15\"; :>> judgedBy = \"test\"; }}\n"
    );
    let content = format!("package LegacyOrientBdd {{\n    action def LegacyRun {{\n{body}    }}\n}}\n");
    std::fs::write(tracking.join("legacy.sysml"), content).unwrap();
    world.root = Some(root);
}

// ── when steps ────────────────────────────────────────────────────────────────

#[when("I run orient")]
fn when_run_orient(world: &mut OrientWorld) {
    let root = world.root.as_ref().unwrap();
    write_sysml(root, &world.tasks, &world.succs);
    let output = orient::compute(root);
    world.done = output.done;
    world.outstanding = output.outstanding;
    world.ready = output.ready;
    world.invalid_evidence = output.invalid_evidence;
}

#[when("I run orient on the prepared dir")]
fn when_run_orient_prepared(world: &mut OrientWorld) {
    let root = world.root.as_ref().unwrap();
    let output = orient::compute(root);
    world.done = output.done;
    world.outstanding = output.outstanding;
    world.ready = output.ready;
    world.invalid_evidence = output.invalid_evidence;
}

#[when("I run whats-next")]
fn when_run_whats_next(world: &mut OrientWorld) {
    let root = world.root.as_ref().unwrap();
    write_sysml(root, &world.tasks, &world.succs);
    let output = orient::compute(root);
    world.done = output.done;
    world.outstanding = output.outstanding;
    world.ready = output.ready;
    world.invalid_evidence = output.invalid_evidence;
}

// ── then steps ────────────────────────────────────────────────────────────────

#[then(regex = r"^done count is (\d+)$")]
fn then_done(world: &mut OrientWorld, n: usize) {
    assert_eq!(world.done, n, "done count");
}

#[then(regex = r"^outstanding count is (\d+)$")]
fn then_outstanding(world: &mut OrientWorld, n: usize) {
    assert_eq!(world.outstanding, n, "outstanding count");
}

#[then("ready is empty")]
fn then_ready_empty(world: &mut OrientWorld) {
    assert!(world.ready.is_empty(), "expected empty ready, got {:?}", world.ready);
}

#[then(regex = r#"^ready contains "([^"]+)"$"#)]
fn then_ready_contains(world: &mut OrientWorld, name: String) {
    assert!(
        world.ready.contains(&name),
        "expected {:?} in ready {:?}",
        name,
        world.ready
    );
}

#[then(regex = r#"^invalidEvidence contains "([^"]+)"$"#)]
fn then_invalid_evidence_contains(world: &mut OrientWorld, name: String) {
    assert!(
        world.invalid_evidence.contains(&name),
        "expected {:?} in invalidEvidence {:?}",
        name,
        world.invalid_evidence
    );
}

// ── runner ────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    OrientWorld::run("tests/features/orient.feature").await;
}
