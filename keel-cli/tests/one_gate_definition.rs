//! ONE gate definition, three callers (issue282).
//!
//! There were three bodies with three different bars: the workspace per-project gate ran validate +
//! guards + declared rules; the scaffolded pre-commit hook ran validate + guard + `rules --enforce`;
//! and the `sync`/`land` gate ran validate + guards and NO rules at all — that file contained zero
//! occurrences of the word, while its own doc comment claimed it was "deliberately the SAME entry
//! point the commit hook uses".
//!
//! The declared-rule layer is precisely how a DOWNSTREAM project adds a blocking control without
//! writing Rust. So such a project had its only control enforced at commit and unenforced on the
//! merged tree — the exact class the merged-tree gate exists to catch.
//!
//! This test authors a downstream rule with NOTHING in Rust behind it, violates it in a way that
//! leaves `validate` clean and every guard clean, and asserts all three callers refuse. The isolation
//! matters: an untriaged-Issue fixture also trips `guard:issues`, so it would have failed the old
//! `sync` gate too and proved nothing.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keel"))
}

fn git(dir: &Path, args: &[&str]) {
    Command::new("git").arg("-C").arg(dir).args(args).output().expect("run git");
}

struct Tmp(PathBuf);
impl Drop for Tmp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

const DOWNSTREAM_RULE: &str = r#"// A downstream project's own blocking control, declared in SysML with no Rust behind it.
package DownstreamRules {
    private import EngineElement::*;
    private import EngineProcess::*;

    part everyStoryHasAnOwnerRule : ElementRule {
        :>> id = "ddddddd1-0001-4001-9001-dddddddddddd";
        :>> title = "Every Story must declare an owner";
        :>> subjectType = "Story";
        :>> predicate = "nonBlank(owner)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
}
"#;

const VIOLATING_STORY: &str = r#"// A Story with NO owner: violates the downstream rule and nothing else.
package OwnerlessFixture {
    private import EngineElement::*;
    private import EngineWork::*;

    part storyNoOwner : Story {
        :>> id = "bbbbbbbb-2222-4222-9222-bbbbbbbbbbbb";
        :>> title = "a story with no owner";
        :>> createdAt = "2026-01-02";
        :>> createdBy = "you";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> estimatedPoints = 1;
    }
}
"#;

#[test]
fn a_declared_rule_violation_fails_the_commit_gate_and_sync_and_land() {
    let base = std::env::temp_dir().join(format!("keel_one_gate_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let _cleanup = Tmp(base.clone());
    let remote = base.join("remote.git");
    let work = base.join("work");
    std::fs::create_dir_all(&work).expect("mkdir");
    Command::new("git").args(["init", "-q", "--bare"]).arg(&remote).output().expect("init bare");
    Command::new("git").args(["init", "-q"]).arg(&work).output().expect("init work");
    git(&work, &["config", "user.email", "t@test.invalid"]);
    git(&work, &["config", "user.name", "t"]);
    git(&work, &["remote", "add", "origin", remote.to_str().unwrap()]);

    let proj = work.join("alpha");
    std::fs::create_dir_all(&proj).expect("mkdir");
    let out = keel()
        .args(["init", proj.to_str().unwrap(), "--profile", "guided"])
        .output()
        .expect("run keel init");
    assert!(out.status.success(), "init failed: {}", String::from_utf8_lossy(&out.stderr));
    git(&work, &["add", "-A"]);
    git(&work, &["commit", "-q", "--no-verify", "-m", "project"]);
    git(&work, &["branch", "-M", "main"]);
    git(&work, &["push", "-q", "-u", "origin", "main"]);

    // The downstream control, and something that violates ONLY it.
    std::fs::write(proj.join(".engine/rules/downstream.sysml"), DOWNSTREAM_RULE).expect("write rule");
    std::fs::write(proj.join(".tracking/ownerless.sysml"), VIOLATING_STORY).expect("write story");

    // ISOLATION, asserted rather than assumed: validate is clean and every guard is clean, so the
    // ONLY thing that can fail a caller is the declared-rule layer.
    let out = keel().args(["validate", proj.to_str().unwrap()]).output().expect("validate");
    assert!(out.status.success(), "fixture must leave validate CLEAN: {}", String::from_utf8_lossy(&out.stdout));
    let out = keel().args(["guard", "all", proj.to_str().unwrap()]).output().expect("guard");
    assert!(out.status.success(), "fixture must leave the guards CLEAN: {}", String::from_utf8_lossy(&out.stdout));
    let out = keel().args(["rules", proj.to_str().unwrap()]).output().expect("rules");
    let rules = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(rules.contains("storyNoOwner"), "the declared rule must actually be violated: {rules}");

    git(&work, &["add", "-A"]);

    // CALLER 1 — the commit gate, which is what the scaffolded pre-commit hook runs.
    let out = keel().args(["gate", "--workspace"]).arg(&work).output().expect("gate");
    let said = String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "the commit gate must fail on a declared-rule violation:\n{said}");
    assert!(said.contains("everyStoryHasAnOwnerRule"), "and must name the rule:\n{said}");

    git(&work, &["commit", "-q", "--no-verify", "-m", "violating commit"]);

    // CALLER 2 — land. Before this fix it PUSHED, exit 0.
    let out = keel().arg("land").current_dir(&proj).output().expect("land");
    let said = String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "land must fail on a declared-rule violation:\n{said}");
    assert!(said.contains("everyStoryHasAnOwnerRule"), "land must name the rule:\n{said}");

    // CALLER 3 — sync, the merged-tree gate.
    let out = keel().arg("sync").current_dir(&proj).output().expect("sync");
    let said = String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "sync must fail on a declared-rule violation:\n{said}");
    assert!(said.contains("everyStoryHasAnOwnerRule"), "sync must name the rule:\n{said}");

    // And nothing reached the remote.
    let out = Command::new("git").arg("-C").arg(&work).args(["ls-remote", "origin", "main"]).output().expect("ls-remote");
    let remote_head = String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string();
    let out = Command::new("git").arg("-C").arg(&work).args(["rev-parse", "HEAD"]).output().expect("rev-parse");
    let local_head = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_ne!(remote_head, local_head, "the violating commit must not have reached the remote");
}
