//! D0391 / issue408: on a SELF-BUILD tree the hooks run a stable copy at `.keel/bin/keel(.exe)` that
//! they refresh from `target/release/keel(.exe)` themselves - so cargo never has to unlink the file a
//! hook is executing, and no human remembers to copy anything.
//!
//! The defect: the pin probe fell through to PATH and resolved into the build output; `keel suite`
//! then read `fail - 0 passed, 0 failed` three runs running over `failed to remove file
//! target/release/keel.exe`, hiding three real regressions. These tests construct a self-build-shaped
//! tree (a `keel-cli/Cargo.toml` and a build output) and fire a hook against it.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test binary path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

const BIN: &str = if cfg!(windows) { "keel.exe" } else { "keel" };

fn run_hook(root: &Path, event: &str, payload: &str) -> (String, String) {
    let mut child = Command::new(keel_bin())
        .args(["hook", event])
        .current_dir(root)
        .env("KEEL_ACTOR", "ai")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn hook");
    child.stdin.as_mut().expect("stdin").write_all(payload.as_bytes()).expect("write payload");
    let out = child.wait_with_output().expect("hook finished");
    (String::from_utf8_lossy(&out.stdout).to_string(), String::from_utf8_lossy(&out.stderr).to_string())
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("hb{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    let out = Command::new(keel_bin()).args(["init", "."]).current_dir(&root).env("KEEL_ACTOR", "ai").output().expect("init");
    assert!(root.join(".tracking").is_dir(), "scaffold: {}", String::from_utf8_lossy(&out.stderr));
    // init seeds .keel/bin/<version>/<asset>; the stable copy itself is what this sprint adds, so start without one
    let _ = std::fs::remove_file(root.join(".keel").join("bin").join(BIN));
    root
}

/// Make the tree a self-build and give it a build output whose mtime is settled (older than the grace).
fn make_self_build(root: &Path, content: &[u8]) -> PathBuf {
    std::fs::create_dir_all(root.join("keel-cli")).expect("keel-cli dir");
    std::fs::write(root.join("keel-cli").join("Cargo.toml"), "[package]\nname = \"keel-cli\"\n").expect("Cargo.toml");
    let out_dir = root.join("target").join("release");
    std::fs::create_dir_all(&out_dir).expect("target dir");
    let out = out_dir.join(BIN);
    std::fs::write(&out, content).expect("build output");
    let settled = std::time::SystemTime::now() - std::time::Duration::from_secs(10);
    std::fs::File::options().write(true).open(&out).expect("open").set_modified(settled).expect("set mtime");
    out
}

fn ledger_events(root: &Path, event: &str) -> usize {
    std::fs::read_to_string(root.join(".keel").join("metrics").join("hooks.jsonl"))
        .unwrap_or_default()
        .lines()
        .filter(|l| l.contains(&format!("\"event\":\"{event}\"")))
        .count()
}

const PAYLOAD: &str = r#"{"session_id":"hb-session","tool_input":{"command":"echo hi"}}"#;

#[test]
fn the_first_hook_fire_on_a_self_build_places_the_stable_copy_and_says_so_in_the_ledger() {
    let root = scaffold("place");
    let out = make_self_build(&root, b"build-one");
    let copy = root.join(".keel").join("bin").join(BIN);
    assert!(!copy.exists(), "no stable copy before the fire");
    let (_o, err) = run_hook(&root, "pre-bash", PAYLOAD);
    assert!(copy.is_file(), "the hook placed the stable copy: {err}");
    assert_eq!(std::fs::read(&copy).expect("copy"), std::fs::read(&out).expect("out"), "byte-for-byte the build output");
    assert_eq!(ledger_events(&root, "hook-binary-refreshed"), 1, "the refresh is a ledger fact, not only a stderr line");
    assert!(err.contains("hook binary refreshed"), "and the fire says so: {err}");
    // a second fire with the same output does nothing
    let _ = run_hook(&root, "pre-bash", PAYLOAD);
    assert_eq!(ledger_events(&root, "hook-binary-refreshed"), 1, "an unchanged output is not re-copied");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_new_build_output_is_picked_up_by_the_next_hook_fire() {
    let root = scaffold("refresh");
    make_self_build(&root, b"build-one");
    let _ = run_hook(&root, "pre-bash", PAYLOAD);
    let copy = root.join(".keel").join("bin").join(BIN);
    assert_eq!(std::fs::read(&copy).expect("copy"), b"build-one");
    // the build path produces a new binary; nobody copies anything
    make_self_build(&root, b"build-two-longer");
    let _ = run_hook(&root, "post-edit", r#"{"session_id":"hb-session","tool_input":{"file_path":"nothing.txt"}}"#);
    assert_eq!(std::fs::read(&copy).expect("copy"), b"build-two-longer", "the next fire refreshed the copy from the new output");
    assert_eq!(ledger_events(&root, "hook-binary-refreshed"), 2);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_output_still_linking_is_not_copied() {
    let root = scaffold("young");
    make_self_build(&root, b"build-one");
    let out = root.join("target").join("release").join(BIN);
    // fresh mtime: a link may still be in progress
    std::fs::File::options().write(true).open(&out).expect("open").set_modified(std::time::SystemTime::now()).expect("mtime");
    let _ = run_hook(&root, "pre-bash", PAYLOAD);
    assert!(!root.join(".keel").join("bin").join(BIN).exists(), "an output younger than the grace is left alone");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_project_that_is_not_a_self_build_is_untouched() {
    let root = scaffold("plain");
    let out_dir = root.join("target").join("release");
    std::fs::create_dir_all(&out_dir).expect("target dir");
    std::fs::write(out_dir.join(BIN), b"someone else's build").expect("write");
    let _ = run_hook(&root, "pre-bash", PAYLOAD);
    assert!(!root.join(".keel").join("bin").join(BIN).exists(), "no keel-cli/Cargo.toml: nothing is placed");
    assert_eq!(ledger_events(&root, "hook-binary-refreshed"), 0);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn sync_claude_names_the_hook_binary_on_a_self_build_and_places_it() {
    let root = scaffold("sync");
    make_self_build(&root, b"build-one");
    let out = Command::new(keel_bin()).args(["sync-claude", "."]).current_dir(&root).env("KEEL_ACTOR", "ai").output().expect("sync");
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(text.contains("hook binary: placed"), "sync-claude places the copy on a self-build: {text}");
    assert!(text.contains("self-build stable copy"), "and says which binary the hooks will run: {text}");
    assert!(root.join(".keel").join("bin").join(BIN).is_file());
    let _ = std::fs::remove_dir_all(&root);
}
