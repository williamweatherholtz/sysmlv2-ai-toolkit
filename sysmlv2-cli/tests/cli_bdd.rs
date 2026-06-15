#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when, World};
use sysmlv2_cli::{check_files, collect_sysml, Report};

static FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default, World)]
pub struct CliWorld {
    files: Vec<PathBuf>,
    dir: Option<PathBuf>,
    check_report: Option<Report>,
    collected: Vec<PathBuf>,
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

#[tokio::main]
async fn main() {
    CliWorld::run("tests/features/cli.feature").await;
}
