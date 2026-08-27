//! `keel land` must gate BEFORE the first push (issue280/D0234 clause 4).
//!
//! The loop used to push FIRST and only reach the gate after a REJECTION, so on the success path —
//! the common path — nothing was gated at all and a broken project left the machine. D0234 asserted
//! that `land` gates every project in the workspace before anything is pushed; that was the one
//! clause never implemented, on the path that matters.
//!
//! Driven against a real BARE REMOTE rather than a mocked one, because the defect was entirely in the
//! ORDER of two real operations: a test that stubbed the push could not have seen it.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keel"))
}

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().expect("run git");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

struct Tmp(PathBuf);
impl Drop for Tmp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn land_refuses_to_push_a_workspace_carrying_a_broken_project() {
    let base = std::env::temp_dir().join(format!("keel_land_gate_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let _cleanup = Tmp(base.clone());
    let remote = base.join("remote.git");
    let work = base.join("work");
    std::fs::create_dir_all(&work).expect("mkdir");

    // A real bare remote.
    Command::new("git").args(["init", "-q", "--bare"]).arg(&remote).output().expect("init bare");
    Command::new("git").args(["init", "-q"]).arg(&work).output().expect("init work");
    git(&work, &["config", "user.email", "t@test.invalid"]);
    git(&work, &["config", "user.name", "t"]);
    git(&work, &["remote", "add", "origin", remote.to_str().unwrap()]);

    // Two projects as PEERS — the layout a workspace has (D0238: none at the root).
    for p in ["alpha", "beta"] {
        std::fs::create_dir_all(work.join(p)).expect("mkdir project");
        let out = keel()
            .args(["init", work.join(p).to_str().unwrap(), "--profile", "guided"])
            .output()
            .expect("run keel init");
        assert!(out.status.success(), "init {p} failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    git(&work, &["add", "-A"]);
    git(&work, &["commit", "-q", "--no-verify", "-m", "two projects"]);
    git(&work, &["branch", "-M", "main"]);
    git(&work, &["push", "-q", "-u", "origin", "main"]);
    let baseline = git(&work, &["rev-parse", "HEAD"]);
    assert_eq!(git(&work, &["rev-parse", "origin/main"]), baseline, "baseline must be on the remote");

    // Break BETA and commit past the hook, exactly as a contributor with no configured hook would.
    std::fs::write(work.join("beta/.tracking/broken.sysml"), "package Broken {\n  not sysml (((\n")
        .expect("write broken file");
    git(&work, &["add", "-A"]);
    git(&work, &["commit", "-q", "--no-verify", "-m", "broken beta"]);
    let broken_head = git(&work, &["rev-parse", "HEAD"]);
    assert_ne!(broken_head, baseline, "the broken commit exists locally");

    // Land FROM ALPHA — a DIFFERENT project. A push carries the whole repository, so gating only the
    // project you stand in is precisely the hole.
    let out = keel().arg("land").current_dir(work.join("alpha")).output().expect("run keel land");
    let said = String::from_utf8_lossy(&out.stderr).to_string() + &String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success(), "land must REFUSE a broken workspace, but exited 0:\n{said}");
    assert!(said.contains("REFUSING to push"), "the refusal must say nothing was pushed:\n{said}");

    // The evidence that matters: the broken commit never left the machine.
    git(&work, &["fetch", "-q", "origin"]);
    assert_eq!(
        git(&work, &["rev-parse", "origin/main"]),
        baseline,
        "the remote must still be at the baseline — a broken project reached it"
    );

    // And a CLEAN workspace still lands, or the gate is just a refusal.
    std::fs::remove_file(work.join("beta/.tracking/broken.sysml")).expect("rm");
    git(&work, &["add", "-A"]);
    git(&work, &["commit", "-q", "--no-verify", "-m", "remove the broken file"]);
    let out = keel().arg("land").current_dir(work.join("alpha")).output().expect("run keel land");
    let said = String::from_utf8_lossy(&out.stderr).to_string() + &String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "a clean workspace must still land:\n{said}");
    git(&work, &["fetch", "-q", "origin"]);
    assert_eq!(
        git(&work, &["rev-parse", "origin/main"]),
        git(&work, &["rev-parse", "HEAD"]),
        "the clean tree must actually have been pushed"
    );
}
