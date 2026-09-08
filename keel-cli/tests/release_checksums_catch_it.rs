//! Guards 66 `release-checksums-published` and 67 `wrapper-pin-checksummed` are SHOWN catching their
//! defects (D0385, issue417, issue418).
//!
//! The defects: a release workflow that attaches a binary and no hash of it, so the only checksum a
//! downstream project can hold is one it computed from the download it is trying to verify; and an
//! engine pin that moved while the wrapper's checksum table did not, so keelw refuses every fresh
//! clone and no gate says so. Both lived in this project - three releases without a hash, two days
//! of a 0.4.1 pin over a 0.3.1 table - which is why the guard tests construct exactly those trees.

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

fn run(dir: &Path, args: &[&str]) -> String {
    let out = Command::new(keel_bin())
        .args(args)
        .current_dir(dir)
        .env("KEEL_ACTOR", "ai")
        .output()
        .expect("keel runs");
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

fn scaffold(tag: &str) -> PathBuf {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("rc{tag}{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    let out = run(&root, &["init", "."]);
    assert!(root.join(".engine").is_dir(), "scaffold: {out}");
    root
}

fn write_workflow(root: &Path, name: &str, body: &str) {
    let dir = root.join(".github").join("workflows");
    std::fs::create_dir_all(&dir).expect("workflows dir");
    std::fs::write(dir.join(name), body).expect("write workflow");
}

const PUBLISHES_NO_HASH: &str = "name: release\non:\n  push:\n    tags: ['v*']\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - run: cargo build --release --bin keel\n      - run: cp target/release/keel keel-linux-x86_64\n      - uses: softprops/action-gh-release@v2\n        with:\n          files: keel-linux-x86_64\n";

const PUBLISHES_WITH_HASH: &str = "name: release\non:\n  push:\n    tags: ['v*']\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - run: cargo build --release --bin keel\n      - run: cp target/release/keel keel-linux-x86_64\n      - run: sha256sum keel-linux-x86_64 > keel-linux-x86_64.sha256\n      - uses: softprops/action-gh-release@v2\n        with:\n          files: |\n            keel-linux-x86_64\n            keel-linux-x86_64.sha256\n";

#[test]
fn a_workflow_that_publishes_a_binary_without_its_hash_is_a_violation() {
    let root = scaffold("nohash");
    write_workflow(&root, "release.yml", PUBLISHES_NO_HASH);
    let out = run(&root, &["guard", "release-checksums-published", "."]);
    assert!(
        out.contains("FAIL") && out.contains("release.yml") && out.contains("computes no SHA-256"),
        "the guard must NAME the workflow and what it lacks: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_workflow_that_hashes_and_publishes_the_hash_is_clean_and_a_non_publishing_one_is_not_scanned() {
    let root = scaffold("hashed");
    write_workflow(&root, "release.yml", PUBLISHES_WITH_HASH);
    write_workflow(
        &root,
        "ci.yml",
        "name: ci\non: [push]\njobs:\n  gate:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - run: cargo test --release\n",
    );
    let out = run(&root, &["guard", "release-checksums-published", "."]);
    assert!(
        out.contains("1 scanned") && out.contains("0 violation(s)"),
        "one publishing workflow with its hash is clean; ci.yml publishes nothing and is not scanned: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_hash_file_named_but_never_computed_is_a_violation() {
    // The half-fix: the upload lists a .sha256 that no step produces.
    let root = scaffold("named");
    let body = PUBLISHES_WITH_HASH.replace("      - run: sha256sum keel-linux-x86_64 > keel-linux-x86_64.sha256\n", "");
    write_workflow(&root, "release.yml", &body);
    let out = run(&root, &["guard", "release-checksums-published", "."]);
    assert!(
        out.contains("FAIL") && out.contains("comes from nowhere"),
        "a hash file with no computation behind it is the defect wearing the fix's name: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

fn write_pin_and_table(root: &Path, pin: &str, table: &str) {
    let contracts = root.join(".engine").join("contracts");
    std::fs::create_dir_all(&contracts).expect("contracts dir");
    std::fs::write(contracts.join("engine-version.toml"), format!("engine = \"{pin}\"\n")).expect("pin");
    std::fs::write(root.join("keel-wrapper.toml"), table).expect("table");
}

#[test]
fn a_pin_with_no_wrapper_entries_is_a_warning_naming_every_missing_asset() {
    let root = scaffold("pinwarn");
    write_pin_and_table(
        &root,
        "0.4.1",
        "[\"0.3.1\"]\n\"keel-linux-x86_64\" = \"aa\"\n\"keel-macos-aarch64\" = \"bb\"\n\"keel-windows-x86_64.exe\" = \"cc\"\n",
    );
    let out = run(&root, &["guard", "wrapper-pin-checksummed", "."]);
    assert!(
        out.contains("WARN")
            && out.contains("0.4.1")
            && out.contains("keel-linux-x86_64, keel-macos-aarch64, keel-windows-x86_64.exe")
            && out.contains("0 violation(s)"),
        "the pre-sprint state of this project: a warning naming the pin and every asset, never a block: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_pin_whose_table_carries_every_asset_is_clean_and_a_partial_table_names_the_gap() {
    let root = scaffold("pinok");
    write_pin_and_table(
        &root,
        "0.4.1",
        "[\"0.3.1\"]\n\"keel-linux-x86_64\" = \"aa\"\n[\"0.4.1\"]\n\"keel-linux-x86_64\" = \"11\"\n\"keel-macos-aarch64\" = \"22\"\n\"keel-windows-x86_64.exe\" = \"33\"\n",
    );
    let out = run(&root, &["guard", "wrapper-pin-checksummed", "."]);
    assert!(out.contains("0 warning(s)"), "every asset present under the pin: {out}");

    write_pin_and_table(&root, "0.4.1", "[\"0.4.1\"]\n\"keel-linux-x86_64\" = \"11\"\n");
    let out = run(&root, &["guard", "wrapper-pin-checksummed", "."]);
    assert!(
        out.contains("WARN") && out.contains("keel-macos-aarch64, keel-windows-x86_64.exe") && !out.contains("for keel-linux"),
        "a partial table names exactly the missing assets: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_project_with_no_wrapper_table_claims_nothing() {
    let root = scaffold("notable");
    let _ = std::fs::remove_file(root.join("keel-wrapper.toml"));
    let out = run(&root, &["guard", "wrapper-pin-checksummed", "."]);
    assert!(
        out.contains("0 scanned") && out.contains("0 warning(s)"),
        "absent keel-wrapper.toml: the activation convention, nothing is accused: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}
