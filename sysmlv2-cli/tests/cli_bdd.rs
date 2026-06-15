#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when, World};
use sysmlv2_cli::{
    check_files, collect_sysml, compute_orient_state, parse_cursor, Cursor, OrientReport, Report,
};
use sysmlv2_parser::ast::Package;
use sysmlv2_parser::{parse, tokenize};

static FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default, World)]
pub struct CliWorld {
    files: Vec<PathBuf>,
    dir: Option<PathBuf>,
    check_report: Option<Report>,
    collected: Vec<PathBuf>,
    orient_packages: Vec<Package>,
    orient_cursor: Option<Cursor>,
    orient_ready: Vec<String>,
    orient_done: usize,
    orient_outstanding: usize,
    orient_json: Option<String>,
}

// ── given ────────────────────────────────────────────────────────────────────

#[given(expr = "a SysML file with content {string}")]
fn given_sysml_file(world: &mut CliWorld, content: String) {
    let dir = std::env::temp_dir().join("sysmlv2_cli_bdd");
    std::fs::create_dir_all(&dir).expect("create temp dir");
    let idx = FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = dir.join(format!("test_{idx}.sysml"));
    std::fs::write(&path, &content).expect("write temp sysml");
    world.files.push(path);
}

#[given(expr = "a temporary directory containing {int} SysML files")]
fn given_dir_with_files(world: &mut CliWorld, count: usize) {
    let run = FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("sysmlv2_cli_bdd_collect_{run}"));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    for i in 0..count {
        let path = dir.join(format!("collect_{i}.sysml"));
        std::fs::write(&path, format!("package File{i} {{}}")).expect("write sysml");
    }
    world.dir = Some(dir);
}

#[given(expr = "orient fixtures loaded from {string}")]
fn given_orient_fixtures(world: &mut CliWorld, path: String) {
    let dir = Path::new(&path);
    world.orient_packages = collect_sysml(dir)
        .iter()
        .filter_map(|p| {
            let src = std::fs::read_to_string(p).ok()?;
            let name = p.to_string_lossy().to_string();
            let tokens = tokenize(&src, &name).ok()?;
            parse(tokens, &name).ok()
        })
        .collect();
}

// ── when ─────────────────────────────────────────────────────────────────────

#[when(expr = "I check the files")]
fn when_check(world: &mut CliWorld) {
    world.check_report = Some(check_files(&world.files));
}

#[when(expr = "I collect SysML files from the directory")]
fn when_collect(world: &mut CliWorld) {
    let dir = world.dir.as_deref().expect("no directory set");
    world.collected = collect_sysml(dir);
}

#[when(expr = "I parse the orient cursor")]
fn when_orient_cursor(world: &mut CliWorld) {
    world.orient_cursor = world.orient_packages.iter().find_map(parse_cursor);
}

#[when(expr = "I compute the orient state")]
fn when_orient_state(world: &mut CliWorld) {
    let (ready, done, outstanding) = compute_orient_state(&world.orient_packages);
    let cursor = world.orient_packages.iter().find_map(parse_cursor);
    world.orient_ready = ready.clone();
    world.orient_done = done;
    world.orient_outstanding = outstanding;
    world.orient_json = Some(OrientReport { cursor, ready, done, outstanding }.to_json());
}

// ── then ─────────────────────────────────────────────────────────────────────

#[then(expr = "the check report is clean")]
fn then_clean(world: &mut CliWorld) {
    let r = world.check_report.as_ref().expect("no report");
    assert!(r.is_clean(), "expected clean report but got errors: {:?}", r.errors);
}

#[then(expr = "the check report has errors")]
fn then_has_errors(world: &mut CliWorld) {
    let r = world.check_report.as_ref().expect("no report");
    assert!(!r.errors.is_empty(), "expected errors but report is clean");
}

#[then(expr = "{int} paths are collected")]
fn then_paths_collected(world: &mut CliWorld, expected: usize) {
    assert_eq!(
        world.collected.len(),
        expected,
        "expected {expected} paths but found {}",
        world.collected.len()
    );
}

#[then(expr = "the cursor active workflow is {string}")]
fn then_cursor_workflow(world: &mut CliWorld, expected: String) {
    let c = world.orient_cursor.as_ref().expect("no cursor parsed");
    assert_eq!(c.active_workflow, expected);
}

#[then(expr = "{int} task is done")]
fn then_done(world: &mut CliWorld, expected: usize) {
    assert_eq!(world.orient_done, expected, "expected {expected} done tasks");
}

#[then(expr = "{int} task is outstanding")]
fn then_outstanding(world: &mut CliWorld, expected: usize) {
    assert_eq!(world.orient_outstanding, expected, "expected {expected} outstanding tasks");
}

#[then(expr = "{string} is in the ready list")]
fn then_ready_contains(world: &mut CliWorld, name: String) {
    assert!(
        world.orient_ready.contains(&name),
        "ready list {:?} does not contain {name}",
        world.orient_ready
    );
}

#[then(expr = "the orient JSON contains {string}")]
fn then_json_contains(world: &mut CliWorld, expected: String) {
    let json = world.orient_json.as_ref().expect("no orient JSON computed");
    assert!(json.contains(&expected), "JSON does not contain {expected}: {json}");
}

#[tokio::main]
async fn main() {
    CliWorld::run("tests/features/cli.feature").await;
}
