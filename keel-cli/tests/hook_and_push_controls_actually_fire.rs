//! Arming probes for four more controls (dcProveTheRemainingControls / issue303):
//! `ctlPreWriteTiers`, `ctlFireLedger`, `ctlPrePushBehind`, `ctlReverify`.
//!
//! Same discipline as the write-lock and launcher probes: each control is shown to ALLOW on the
//! clean case before it is shown to REFUSE on the dirty one, because a control that refuses
//! everything is stuck rather than armed — and each probe exercises the REAL mechanism (the real
//! binary via its hook entry point, the real shell hook via a real `git push`), never a
//! reimplementation of it, since a probe of a copy proves the copy.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
}

/// A keel-shaped fixture project with a declared STRICT profile — the tier where pre-write DENIES.
fn strict_project(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-hookprobe-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    std::fs::write(
        root.join(".engine").join("contracts").join("adoption-profile.toml"),
        "profile = \"strict\"\ndeclaredAt = \"2026-08-29\"\n",
    )
    .expect("profile");
    root
}

/// Invoke the REAL hook entry point the harness invokes, with `cwd` inside the fixture so
/// `find_repo_root` resolves the fixture and not this repository.
fn run_hook(root: &Path, event: &str, payload: &str) -> (String, String) {
    let mut child = Command::new(keel_bin())
        .args(["hook", event])
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn hook");
    child.stdin.as_mut().expect("stdin").write_all(payload.as_bytes()).expect("write payload");
    let out = child.wait_with_output().expect("hook finished");
    (String::from_utf8_lossy(&out.stdout).to_string(), String::from_utf8_lossy(&out.stderr).to_string())
}

fn payload_for(root: &Path, rel: &str) -> String {
    let path = root.join(rel).to_string_lossy().replace('\\', "/");
    format!(r#"{{"session_id":"probe-session","tool_input":{{"file_path":"{path}"}}}}"#)
}

/// issue365 / D0296: a Write that sets `disableAllHooks` in a repo-scope settings file is DENIED in
/// EVERY profile - this fixture declares none (guided), where every other pre-write outcome is
/// advisory. The key silences every hook from every scope; the advisory would be the first casualty.
#[test]
fn pre_write_denies_the_hook_kill_switch_in_a_guided_project() {
    let root = std::env::temp_dir().join(format!("keel-killswitch-hook-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".claude")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    let path = root.join(".claude").join("settings.json").to_string_lossy().replace('\\', "/");
    let poison = format!(
        r#"{{"session_id":"probe","tool_name":"Write","tool_input":{{"file_path":"{path}","content":"{{\"hooks\": {{}}, \"disableAllHooks\": true}}"}}}}"#
    );
    let (out, _) = run_hook(&root, "pre-write", &poison);
    assert!(
        out.contains(r#""permissionDecision":"deny""#) || out.contains(r#""permissionDecision": "deny""#),
        "setting the kill switch must DENY even in a guided project, got: {out}"
    );
    assert!(out.contains("disableAllHooks") && out.contains("issue365"), "the refusal names the key and its issue: {out}");
    // the same file WITHOUT the key is not denied - the control is on the key, not the file
    let benign = format!(r#"{{"session_id":"probe","tool_name":"Write","tool_input":{{"file_path":"{path}","content":"{{\"hooks\": {{}}}}"}}}}"#);
    let (out, _) = run_hook(&root, "pre-write", &benign);
    assert!(!out.contains(r#""deny""#), "a settings write without the kill switch is not denied here: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── ctlPreWriteTiers ──────────────────────────────────────────────────────────────────────────────

#[test]
fn pre_write_denies_a_protected_surface_under_strict_and_allows_an_ordinary_file() {
    let root = strict_project("prewrite");

    // TIER 2 first — and this assertion REPLACES a wrong one. The first draft asserted an ordinary
    // .tracking write passes silently under strict; the control is STRICTER than that: any direct
    // .tracking write is ask-tier (D0176 tier 2), and the probe's failure taught the prober. The
    // ALLOW property still matters, in its true form: ask is not deny.
    let (out, _) = run_hook(&root, "pre-write", &payload_for(&root, ".tracking/ordinary.sysml"));
    assert!(
        out.contains(r#""permissionDecision":"ask""#) || out.contains(r#""permissionDecision": "ask""#),
        "an ordinary .tracking write under strict is ASK (tier 2), got: {out}"
    );
    assert!(
        !out.contains(r#""deny""#),
        "tier 2 must never DENY — an over-strict gate trains its actor to disable it: {out}"
    );

    // DENY: a protected fact surface under strict names the sanctioned path.
    let (out, _) = run_hook(&root, "pre-write", &payload_for(&root, ".tracking/issues.sysml"));
    assert!(
        out.contains(r#""permissionDecision":"deny""#) || out.contains(r#""permissionDecision": "deny""#),
        "a protected surface under strict must DENY (D0176 tier 1), got: {out}"
    );
    assert!(
        out.contains("keel record issue"),
        "the denial must name the sanctioned path, or the writer is blocked with no way forward: {out}"
    );

    // ASK: the control plane is approval-gated, not denied — a human may intend to change enforcement.
    let (out, _) = run_hook(&root, "pre-write", &payload_for(&root, ".claude/settings.json"));
    assert!(
        out.contains(r#""permissionDecision":"ask""#) || out.contains(r#""permissionDecision": "ask""#),
        "a control-plane write under strict is ASK-tier (D0179/K7), got: {out}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── ctlFireLedger ─────────────────────────────────────────────────────────────────────────────────

#[test]
fn every_hook_fire_leaves_a_counted_ledger_line() {
    let root = strict_project("ledger");
    let ledger = root.join(".keel").join("metrics").join("hooks.jsonl");
    assert!(!ledger.exists(), "fixture starts with no ledger");

    run_hook(&root, "pre-write", &payload_for(&root, ".tracking/ordinary.sysml"));
    run_hook(&root, "pre-write", &payload_for(&root, ".tracking/issues.sysml"));

    let text = std::fs::read_to_string(&ledger)
        .expect("D0180: every fire leaves a machine-local line — no ledger was written at all");
    let lines: Vec<_> = text.lines().filter(|l| l.contains("pre-write")).collect();
    assert_eq!(
        lines.len(),
        2,
        "two fires must leave exactly two pre-write lines (the single instrumentation path the \
         hooks-actually-fired checks read), got {}: {text}",
        lines.len()
    );
    // The frozen 6-field schema: each line parses and carries the session that fired.
    for l in &lines {
        let v: serde_json::Value = serde_json::from_str(l).expect("ledger line is valid JSON");
        assert_eq!(v.get("session").and_then(|s| s.as_str()), Some("probe-session"));
    }
    let _ = std::fs::remove_dir_all(&root);
}

// ── ctlPrePushBehind ──────────────────────────────────────────────────────────────────────────────

/// Two clones, one bare remote, and a REAL `git push` through the REAL `.githooks/pre-push`.
///
/// This is the probe the arming contract said "can only be established by attempting a push, which a
/// read-only view must not do". A test is not a read-only view.
#[test]
fn pre_push_refuses_a_main_push_that_is_not_a_fast_forward_of_the_trunk() {
    let base = std::env::temp_dir().join(format!("keel-prepush-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let bare = base.join("origin.git");
    std::fs::create_dir_all(&bare).expect("mkdir bare");
    let out = Command::new("git").arg("init").arg("--bare").arg("-q").arg(&bare).output().expect("bare init");
    assert!(out.status.success());

    let hook_src = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("workspace root").join(".githooks");
    let clone = |name: &str| -> PathBuf {
        let dst = base.join(name);
        let out = Command::new("git").arg("clone").arg("-q").arg(&bare).arg(&dst).output().expect("clone");
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        git(&dst, &["config", "user.email", "probe@example.invalid"]);
        git(&dst, &["config", "user.name", "probe"]);
        git(&dst, &["config", "core.hooksPath", &hook_src.to_string_lossy().replace('\\', "/")]);
        dst
    };

    // Clone A seeds main — the BOOTSTRAP push, creating main on the remote. THE PROBE FOUND A REAL
    // DEFECT HERE (issue306): `git fetch origin main` fails when the remote has no main YET, and the
    // hook's fail-loud clause conflated "cannot reach the remote" with "the remote has no main yet" —
    // so every fresh project's first push was refused. The hook now skips the behind-ness check when
    // remote_sha is all zeros: ref CREATION is not a behind-ness question, exactly as the deletion
    // case one line above it already said about local_sha.
    let a = clone("a");
    std::fs::write(a.join("f.txt"), "one\n").expect("write");
    git(&a, &["add", "-A"]);
    git(&a, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "a1"]);
    let push_a = Command::new("git").arg("-C").arg(&a).args(["push", "-q", "origin", "HEAD:main"]).output().expect("push a");
    assert!(
        push_a.status.success(),
        "the BOOTSTRAP push (creating main on an empty remote) must pass — refusing it locks every \
         fresh project out of its own remote (issue306): {}",
        String::from_utf8_lossy(&push_a.stderr)
    );

    // Clone B (taken BEFORE a1): commit without fetching — behind the trunk — and push main.
    let b = clone("b");
    std::fs::write(b.join("g.txt"), "two\n").expect("write");
    git(&b, &["add", "-A"]);
    git(&b, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "b1"]);
    // The probe's second lesson: a PLAIN behind push never reaches the hook — git's own client-side
    // fast-forward check refuses it first, from the ref advertisement. The hook's added value is the
    // push git would ACCEPT: a FORCED one, which bypasses the client check but still runs pre-push.
    // That is exactly the history-rewrite class D0129 forbids, so the probe pushes with --force —
    // testing what the control is actually FOR rather than a case something else already catches.
    let push_b = Command::new("git").arg("-C").arg(&b).args(["push", "--force", "origin", "HEAD:main"]).output().expect("push b");
    assert!(
        !push_b.status.success(),
        "a FORCED push of a non-descendant MUST be refused by the hook (srDcGateOnMergedTree/D0129) — it succeeded"
    );
    let err = String::from_utf8_lossy(&push_b.stderr);
    assert!(
        err.contains("keel land"),
        "the refusal must come from the HOOK, naming the sanctioned path (keel land) — a refusal from \
         anything else means the hook never fired, got: {err}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

// ── ctlReverify ───────────────────────────────────────────────────────────────────────────────────

/// `keel reverify` against a REAL drift-suspect task: a failing gate stamps NOTHING, a passing gate
/// stamps exactly one fresh result. The first draft of this probe had no drift-suspect task, so its
/// no-stamp assertion passed VACUOUSLY — reverify had nothing it could have stamped. A probe that
/// cannot fail is a decoration, so the fixture manufactures real drift: a task verified at commit 1
/// whose declared deliverable changes at commit 2.
#[test]
fn reverify_stamps_nothing_on_a_red_gate_and_exactly_one_result_on_green() {
    let root = strict_project("reverify");
    std::fs::create_dir_all(root.join(".tracking").join("delivery")).expect("mkdir");
    std::fs::write(root.join("deliverable.txt"), "v1\n").expect("deliverable");
    std::fs::write(
        root.join(".engine").join("deliverable-manifest.txt"),
        "task: probeTask | deliverable.txt\n",
    )
    .expect("manifest");
    git(&root, &["init", "-q"]);
    git(&root, &["config", "user.email", "probe@example.invalid"]);
    git(&root, &["config", "user.name", "probe"]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);
    let c1 = String::from_utf8_lossy(
        &Command::new("git").arg("-C").arg(&root).args(["rev-parse", "HEAD"]).output().expect("head").stdout,
    )
    .trim()
    .to_string();
    // The task, verified AT commit 1 — then the deliverable moves, making the pass stale.
    let task = format!(
        "package ProbeDelivery {{\n    private import EngineElement::*;\n    private import EngineWork::*;\n    private import EngineVerification::*;\n\n    action def ProbeRun {{\n        action probeTask;\n        verification probeTaskDoD : Test {{ :>> id = \"aaaaaaaa-1111-4111-9111-aaaaaaaaaaaa\"; :>> method = VerificationMethod::test; :>> procedureText = \"probe\"; }}\n    }}\n    part probeTaskDoDR1 : TestResult {{ :>> id = \"aaaaaaaa-2222-4222-9222-aaaaaaaaaaaa\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"{c1}\"; :>> judgedAt = \"2026-08-29\"; :>> judgedBy = \"claudeOpus5\"; }}\n}}\n"
    );
    std::fs::write(root.join(".tracking").join("delivery").join("probe.sysml"), task).expect("task");
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "task verified at c1"]);
    std::fs::write(root.join("deliverable.txt"), "v2 - drifted\n").expect("drift");
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "commit.gpgsign=false", "commit", "-q", "-m", "deliverable drifts"]);

    // RED: a failing gate stamps nothing — and the task is provably REACHABLE, because the same
    // tree stamps on green below. Without that second half this assertion would be vacuous.
    std::fs::write(
        root.join(".engine").join("contracts").join("reverify.toml"),
        "commands = [\"git nonexistent-subcommand-that-fails\"]\n",
    )
    .expect("red contract");
    let before = count_results(&root);
    let out = Command::new(keel_bin()).args(["reverify", "--all-drift"]).current_dir(&root).output().expect("reverify");
    assert_eq!(
        before,
        count_results(&root),
        "the gate FAILED, so no fresh TestResult may be stamped — a fabricated pass is the exact \
         dishonesty ctlReverify exists to prevent (D0101). Output: {}",
        String::from_utf8_lossy(&out.stdout)
    );

    // GREEN: the real gate passes; exactly one fresh result lands on the drifted task.
    std::fs::write(
        root.join(".engine").join("contracts").join("reverify.toml"),
        "commands = [\"git --version\"]\n",
    )
    .expect("green contract");
    let out = Command::new(keel_bin()).args(["reverify", "--all-drift"]).current_dir(&root).output().expect("reverify");
    assert_eq!(
        before + 1,
        count_results(&root),
        "a green gate must stamp exactly ONE fresh result on the drifted task. Output: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let _ = std::fs::remove_dir_all(&root);
}

fn count_results(root: &Path) -> usize {
    walkdir(root.join(".tracking"))
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .map(|t| t.matches(": TestResult").count())
        .sum()
}

fn walkdir(dir: PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "sysml") {
                out.push(p);
            }
        }
    }
    out
}
