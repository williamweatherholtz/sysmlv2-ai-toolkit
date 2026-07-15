#![allow(unused, clippy::all, clippy::pedantic, clippy::nursery)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when, World};
use keel_cli::write::{add_task, append_gate_result, append_result, WriteError};

static DIR_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_dir() -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("write_bdd_{n}"))
}

// ── file builders ─────────────────────────────────────────────────────────────

fn write_task_file(dir: &Path, task: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let content = format!(
        "package TestWrite {{\n    action def TestRun {{\n        action {task};\n        verification {task}DoD : Test {{ :>> id = \"00000001-0000-4000-8000-000000000001\"; :>> method = VerificationMethod::test; :>> procedureText = \"test\"; }}\n    }}\n}}\n"
    );
    let path = dir.join("test.sysml");
    std::fs::write(&path, content).unwrap();
    path
}

fn write_task_file_with_result(dir: &Path, task: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let content = format!(
        "package TestWrite {{\n    action def TestRun {{\n        action {task};\n        verification {task}DoD : Test {{ :>> id = \"00000001-0000-4000-8000-000000000001\"; :>> method = VerificationMethod::test; :>> procedureText = \"test\"; }}\n        part {task}DoDR1 : TestResult {{ :>> id = \"00000002-0000-4000-8000-000000000002\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc0001\"; :>> judgedAt = \"2026-06-15\"; :>> judgedBy = \"test\"; }}\n    }}\n}}\n"
    );
    let path = dir.join("test.sysml");
    std::fs::write(&path, content).unwrap();
    path
}

fn write_def_file(dir: &Path, def_name: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let content = format!(
        "package TestWrite {{\n    action def {def_name} {{\n        action placeholder;\n    }}\n}}\n"
    );
    let path = dir.join("test.sysml");
    std::fs::write(&path, content).unwrap();
    path
}

fn write_def_with_task(dir: &Path, def_name: &str, task: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let content = format!(
        "package TestWrite {{\n    action def {def_name} {{\n        action {task};\n        verification {task}DoD : Test {{ :>> id = \"00000001-0000-4000-8000-000000000001\"; :>> method = VerificationMethod::test; :>> procedureText = \"test\"; }}\n    }}\n}}\n"
    );
    let path = dir.join("test.sysml");
    std::fs::write(&path, content).unwrap();
    path
}

fn write_gate_file(dir: &Path, gate: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let content = format!(
        "package TestWrite {{\n    verification {gate} : Test {{\n        :>> id = \"00000001-0000-4000-8000-000000000001\";\n        :>> method = VerificationMethod::confirmation;\n        :>> procedureText = \"user accepts complete\";\n    }}\n}}\n"
    );
    let path = dir.join("test.sysml");
    std::fs::write(&path, content).unwrap();
    path
}

fn write_gate_file_with_result(dir: &Path, gate: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let content = format!(
        "package TestWrite {{\n    verification {gate} : Test {{\n        :>> id = \"00000001-0000-4000-8000-000000000001\";\n        :>> method = VerificationMethod::confirmation;\n        :>> procedureText = \"user accepts complete\";\n    }}\n    part {gate}R1 : TestResult {{ :>> id = \"00000002-0000-4000-8000-000000000002\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc0001\"; :>> judgedAt = \"2026-06-15\"; :>> judgedBy = \"wweatherholtz\"; }}\n}}\n"
    );
    let path = dir.join("test.sysml");
    std::fs::write(&path, content).unwrap();
    path
}

// ── world ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Default, World)]
pub struct WriteWorld {
    dir: Option<PathBuf>,
    file: Option<PathBuf>,
    result: Option<Result<String, String>>,
    last_content: Option<String>,
}

impl Drop for WriteWorld {
    fn drop(&mut self) {
        if let Some(dir) = &self.dir {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
}

// ── given steps ───────────────────────────────────────────────────────────────

#[given(regex = r#"^a tracking file with task "([^"]+)" and a DoD verification$"#)]
fn given_task_with_dod(world: &mut WriteWorld, task: String) {
    let dir = unique_dir();
    let path = write_task_file(&dir, &task);
    world.dir = Some(dir);
    world.file = Some(path);
}

#[given(regex = r#"^a tracking file with task "([^"]+)" and an existing DoDR1$"#)]
fn given_task_with_existing_result(world: &mut WriteWorld, task: String) {
    let dir = unique_dir();
    let path = write_task_file_with_result(&dir, &task);
    world.dir = Some(dir);
    world.file = Some(path);
}

#[given(regex = r#"^a tracking file with task "([^"]+)"$"#)]
fn given_task_only(world: &mut WriteWorld, task: String) {
    let dir = unique_dir();
    let path = write_task_file(&dir, &task);
    world.dir = Some(dir);
    world.file = Some(path);
}

#[given(regex = r#"^a tracking file with action def "([^"]+)"$"#)]
fn given_action_def(world: &mut WriteWorld, def_name: String) {
    let dir = unique_dir();
    let path = write_def_file(&dir, &def_name);
    world.dir = Some(dir);
    world.file = Some(path);
}

#[given(regex = r#"^a tracking file with action def "([^"]+)" containing task "([^"]+)"$"#)]
fn given_def_with_task(world: &mut WriteWorld, def_name: String, task: String) {
    let dir = unique_dir();
    let path = write_def_with_task(&dir, &def_name, &task);
    world.dir = Some(dir);
    world.file = Some(path);
}

#[given(regex = r#"^a tracking file with gate "([^"]+)" and an existing R1$"#)]
fn given_gate_with_result(world: &mut WriteWorld, gate: String) {
    let dir = unique_dir();
    let path = write_gate_file_with_result(&dir, &gate);
    world.dir = Some(dir);
    world.file = Some(path);
}

#[given(regex = r#"^a tracking file with gate "([^"]+)"$"#)]
fn given_gate(world: &mut WriteWorld, gate: String) {
    let dir = unique_dir();
    let path = write_gate_file(&dir, &gate);
    world.dir = Some(dir);
    world.file = Some(path);
}

// ── when steps ────────────────────────────────────────────────────────────────

#[when(regex = r#"^I append a passing result for "([^"]+)" at SHA "([^"]+)"$"#)]
fn when_append_pass(world: &mut WriteWorld, task: String, sha: String) {
    let path = world.file.clone().unwrap();
    let res = append_result(&path, &task, &sha, "pass", "2026-06-15", "test");
    world.last_content = std::fs::read_to_string(&path).ok();
    world.result = Some(res.map_err(|e| e.to_string()));
}

#[when(regex = r#"^I append a result for unknown task "([^"]+)" at SHA "([^"]+)"$"#)]
fn when_append_unknown_task(world: &mut WriteWorld, task: String, sha: String) {
    let path = world.file.clone().unwrap();
    let res = append_result(&path, &task, &sha, "pass", "2026-06-15", "test");
    world.result = Some(res.map_err(|e| e.to_string()));
}

#[when(regex = r#"^I append a "([^"]+)" result for "([^"]+)" at SHA "([^"]+)"$"#)]
fn when_append_bad_verdict(world: &mut WriteWorld, verdict: String, task: String, sha: String) {
    let path = world.file.clone().unwrap();
    let res = append_result(&path, &task, &sha, &verdict, "2026-06-15", "test");
    world.result = Some(res.map_err(|e| e.to_string()));
}

#[when(regex = r#"^I add task "([^"]+)" with DoD "([^"]+)" method "([^"]+)" to def "([^"]+)"$"#)]
fn when_add_task(world: &mut WriteWorld, task: String, dod: String, method: String, def_name: String) {
    let path = world.file.clone().unwrap();
    let res = add_task(&path, &def_name, &task, &dod, &method);
    world.last_content = std::fs::read_to_string(&path).ok();
    world.result = Some(res.map_err(|e| e.to_string()));
}

#[when(regex = r#"^I append a passing gate result for "([^"]+)" at SHA "([^"]+)"$"#)]
fn when_append_gate_pass(world: &mut WriteWorld, gate: String, sha: String) {
    let path = world.file.clone().unwrap();
    let res = append_gate_result(&path, &gate, &sha, "pass", "2026-06-15", "wweatherholtz", None);
    world.last_content = std::fs::read_to_string(&path).ok();
    world.result = Some(res.map_err(|e| e.to_string()));
}

#[when(regex = r#"^I append a gate result for unknown gate "([^"]+)" at SHA "([^"]+)"$"#)]
fn when_append_gate_unknown(world: &mut WriteWorld, gate: String, sha: String) {
    let path = world.file.clone().unwrap();
    let res = append_gate_result(&path, &gate, &sha, "pass", "2026-06-15", "wweatherholtz", None);
    world.result = Some(res.map_err(|e| e.to_string()));
}

// ── then steps ────────────────────────────────────────────────────────────────

#[then(regex = r#"^the file contains "([^"]+)"$"#)]
fn then_file_contains(world: &mut WriteWorld, needle: String) {
    let content = world
        .last_content
        .as_deref()
        .or_else(|| world.file.as_ref().map(|_| ""))
        .unwrap_or("");
    let content = if content.is_empty() {
        world.file.as_ref().map(|p| std::fs::read_to_string(p).unwrap()).unwrap_or_default()
    } else {
        content.to_owned()
    };
    assert!(
        content.contains(&needle),
        "expected {:?} in file:\n{}",
        needle,
        content
    );
}

#[then(regex = r#"^outcome is "([^"]+)"$"#)]
fn then_outcome_is(world: &mut WriteWorld, expected: String) {
    let content = world.last_content.clone()
        .or_else(|| world.file.as_ref().map(|p| std::fs::read_to_string(p).unwrap()))
        .unwrap_or_default();
    assert!(
        content.contains(&format!(":>> outcome = {expected}")),
        "expected outcome = {:?} in:\n{}",
        expected,
        content
    );
}

#[then(regex = r#"^judgedAgainst is "([^"]+)"$"#)]
fn then_judged_against(world: &mut WriteWorld, sha: String) {
    let content = world.last_content.clone()
        .or_else(|| world.file.as_ref().map(|p| std::fs::read_to_string(p).unwrap()))
        .unwrap_or_default();
    assert!(
        content.contains(&format!(":>> judgedAgainst = \"{sha}\"")),
        "expected judgedAgainst = {:?} in:\n{}",
        sha,
        content
    );
}

#[then("the write fails with task-not-found")]
fn then_task_not_found(world: &mut WriteWorld) {
    let err = world.result.as_ref().expect("no result").as_ref().unwrap_err();
    assert!(
        err.contains("task not found"),
        "expected task-not-found error, got: {err}"
    );
}

#[then("the write fails with invalid-verdict")]
fn then_invalid_verdict(world: &mut WriteWorld) {
    let err = world.result.as_ref().expect("no result").as_ref().unwrap_err();
    assert!(
        err.contains("invalid verdict"),
        "expected invalid-verdict error, got: {err}"
    );
}

#[then("the write fails with gate-not-found")]
fn then_gate_not_found(world: &mut WriteWorld) {
    let err = world.result.as_ref().expect("no result").as_ref().unwrap_err();
    assert!(
        err.contains("gate not found"),
        "expected gate-not-found error, got: {err}"
    );
}

#[then("the write fails with task-already-exists")]
fn then_task_already_exists(world: &mut WriteWorld) {
    let err = world.result.as_ref().expect("no result").as_ref().unwrap_err();
    assert!(
        err.contains("task already exists"),
        "expected task-already-exists error, got: {err}"
    );
}

#[then("the new result has a non-empty id field")]
fn then_has_uuid(world: &mut WriteWorld) {
    let content = world.last_content.clone()
        .or_else(|| world.file.as_ref().map(|p| std::fs::read_to_string(p).unwrap()))
        .unwrap_or_default();
    assert!(
        content.contains(":>> id = \""),
        "expected :>> id = \"...\" in:\n{}",
        content
    );
    // Verify UUID format: 8-4-4-4-12 hex chars
    let id_value = content
        .lines()
        .filter(|l| l.contains("DoDR") && l.contains(":>> id = \""))
        .last()
        .and_then(|l| l.split(":>> id = \"").nth(1))
        .and_then(|s| s.split('"').next())
        .unwrap_or("");
    assert!(
        id_value.len() == 36 && id_value.contains('-'),
        "expected UUID format, got: {:?}",
        id_value
    );
}

// ── runner ────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    WriteWorld::run("tests/features/write.feature").await;
}
