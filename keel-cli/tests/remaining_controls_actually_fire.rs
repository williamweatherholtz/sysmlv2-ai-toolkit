//! Arming probes for the controls issue303 left unproven (D0253: arming may be proven by TEST where a
//! read-only view cannot establish it). Each probe proves the control ALLOWS on a clean case before it
//! asserts the REFUSAL on a dirty one - a control that refuses everything is stuck, not armed.
//!
//! ctlPostEditGate, ctlTurnBoundaryGate, ctlOverrideObligation, ctlAuditAdherence. ctlAdoptionCheck
//! is proven in `adoption_check_can_fail.rs`; the three infra-resident controls (branch protection,
//! audit-history in CI, the release gate job) keep their stated reasons in control-arming.toml.

use std::io::Write as _;
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

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
}

fn git_out(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn run_hook(root: &Path, event: &str, payload: &str) -> (i32, String, String) {
    let mut child = Command::new(keel_bin())
        .args(["hook", event])
        .current_dir(root)
        .env("KEEL_ACTOR", "claudeOpus5")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn hook");
    child.stdin.as_mut().expect("stdin").write_all(payload.as_bytes()).expect("write payload");
    let out = child.wait_with_output().expect("hook finished");
    (out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stdout).to_string(), String::from_utf8_lossy(&out.stderr).to_string())
}

fn keel(root: &Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(keel_bin()).args(args).current_dir(root).env("KEEL_ACTOR", "claudeOpus5").output().expect("keel runs");
    (out.status.code().unwrap_or(-1), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

/// A scaffolded project (`keel init`), which is the only fixture the TURN gate passes clean on: the
/// bare seed used by the pre-write probes has no engine, and `hook stop` runs every guard.
fn scaffolded(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-arming-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let (code, out) = keel(Path::new("."), &["init", &root.to_string_lossy()]);
    assert_eq!(code, 0, "scaffold failed: {out}");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    root
}

fn is_block(out: &str) -> bool {
    out.contains(r#""decision":"block""#) || out.contains(r#""decision": "block""#)
}

fn model_file(root: &Path) -> PathBuf {
    let dir = root.join(".tracking");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect(".tracking exists")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "sysml"))
        .collect();
    files.sort();
    files.into_iter().next().expect("the scaffold has at least one tracking file")
}

// ── ctlPostEditGate ───────────────────────────────────────────────────────────────────────────────

/// An edit leaving the model unparseable is BLOCKED at the edit boundary; a clean edit is silent.
#[test]
fn post_edit_gate_allows_a_clean_edit_and_blocks_a_broken_one() {
    let root = scaffolded("postedit");
    let file = model_file(&root);
    let payload = format!(r#"{{"session_id":"probe","tool_input":{{"file_path":"{}"}}}}"#, file.to_string_lossy().replace('\\', "/"));
    // ALLOW first: the scaffold parses, so the fast tier has nothing to say.
    let (code, out, err) = run_hook(&root, "post-edit", &payload);
    assert_eq!(code, 0, "a clean edit must pass: {out}{err}");
    assert!(!is_block(&out), "a clean edit must not block: {out}");
    // REFUSE: break the file the edit touched.
    let original = std::fs::read_to_string(&file).expect("read");
    std::fs::write(&file, format!("{original}\npackage Broken {{\n")).expect("break");
    let (_, out, _) = run_hook(&root, "post-edit", &payload);
    assert!(is_block(&out), "an edit that leaves the model unparseable must BLOCK at the edit boundary: {out}");
    assert!(out.contains("edit gate"), "the block names the tier: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── ctlTurnBoundaryGate ───────────────────────────────────────────────────────────────────────────

/// A turn cannot END while the model is dishonest; a clean tree ends its turn silently.
#[test]
fn turn_boundary_gate_allows_a_clean_tree_and_blocks_a_dishonest_one() {
    let root = scaffolded("stop");
    let payload = r#"{"session_id":"probe"}"#;
    let (code, out, err) = run_hook(&root, "stop", payload);
    assert!(!is_block(&out), "a fresh scaffold must end its turn (else the gate is stuck, not armed): {out}{err}");
    assert_eq!(code, 0, "{out}{err}");
    let file = model_file(&root);
    let original = std::fs::read_to_string(&file).expect("read");
    std::fs::write(&file, format!("{original}\npackage Broken {{\n")).expect("break");
    let (_, out, _) = run_hook(&root, "stop", payload);
    assert!(is_block(&out), "a turn over an unparseable model must BLOCK: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── ctlOverrideObligation ─────────────────────────────────────────────────────────────────────────

/// An unlock is single-use and path-bound, and consuming it RECORDS an obligation.
#[test]
fn override_unlock_is_single_use_path_bound_and_recorded() {
    let root = std::env::temp_dir().join(format!("keel-arming-override-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir");
    std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
    std::fs::write(root.join(".keel").join("actor"), "claudeOpus5\n").expect("actor");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    std::fs::write(root.join(".engine").join("contracts").join("adoption-profile.toml"), "profile = \"strict\"\ndeclaredAt = \"2026-08-29\"\n").expect("profile");
    let target = root.join(".tracking").join("issues.sysml");
    let path = target.to_string_lossy().replace('\\', "/");
    let payload = format!(r#"{{"session_id":"probe","tool_input":{{"file_path":"{path}"}}}}"#);
    let denies = |out: &str| out.contains(r#""permissionDecision":"deny""#) || out.contains(r#""permissionDecision": "deny""#);

    // Locked: a protected surface under strict denies.
    let (_, out, _) = run_hook(&root, "pre-write", &payload);
    assert!(denies(&out), "the protected surface must deny before any unlock: {out}");
    // A different path is NOT unlocked by this override (path-bound).
    let (code, out) = keel(&root, &["override", ".tracking/issues.sysml", "--reason", "probe: the API cannot express this write, and this test says so"]);
    assert_eq!(code, 0, "override arms: {out}");
    let other = format!(r#"{{"session_id":"probe","tool_input":{{"file_path":"{}"}}}}"#, root.join(".tracking").join("decisions.sysml").to_string_lossy().replace('\\', "/"));
    let (_, out, _) = run_hook(&root, "pre-write", &other);
    assert!(denies(&out) || out.contains("ask"), "an unlock for one path must not open another: {out}");
    // The unlocked path passes ONCE...
    let (_, out, _) = run_hook(&root, "pre-write", &payload);
    assert!(out.contains("override active"), "the armed override lets the write through and says so: {out}");
    assert!(!denies(&out), "{out}");
    // ...and its consumption is RECORDED as an obligation in the tree.
    let obligations = root.join(".tracking").join("obligations");
    let recorded = std::fs::read_dir(&obligations).map(|rd| rd.flatten().count()).unwrap_or(0);
    assert!(recorded >= 1, "consuming an override must record an obligation under .tracking/obligations/");
    // ...then the lock is back.
    let (_, out, _) = run_hook(&root, "pre-write", &payload);
    assert!(denies(&out), "single-use: the second write after one unlock must deny again: {out}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── ctlAuditAdherence ─────────────────────────────────────────────────────────────────────────────

fn commit_all(root: &Path, msg: &str) -> String {
    git(root, &["add", "-A"]);
    git(root, &["-c", "user.email=probe@keel", "-c", "user.name=probe", "commit", "-q", "-m", msg]);
    git_out(root, &["rev-parse", "HEAD"])
}

/// Guard-set / rule-severity monotonicity re-derived from the tree: an UNSIGNED downgrade fails the
/// audit; the same downgrade with a co-committed marked Decision passes; an unchanged range passes.
#[test]
fn audit_adherence_fails_an_unsigned_downgrade_and_passes_a_signed_one() {
    let root = std::env::temp_dir().join(format!("keel-arming-adherence-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".engine").join("rules")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("seed.sysml"), "package Seed {\n}\n").expect("seed");
    let rule = root.join(".engine").join("rules").join("probe.sysml");
    let rule_text = |sev: &str| format!("package ProbeRules {{\n    part probeRule : ElementRule {{ :>> severity = RuleSeverity::{sev}; }}\n}}\n");
    std::fs::write(&rule, rule_text("blocking")).expect("rule");
    git(&root, &["init", "-q", "."]);
    let c1 = commit_all(&root, "a blocking rule");

    // ALLOW: nothing weakened over an unchanged range.
    let (code, out) = keel(&root, &["audit-adherence", "--since", &c1]);
    assert_eq!(code, 0, "an unchanged range passes: {out}");

    // REFUSE: downgrade to warning with no Decision.
    std::fs::write(&rule, rule_text("warning")).expect("downgrade");
    commit_all(&root, "quietly weaken");
    let (code, out) = keel(&root, &["audit-adherence", "--since", &c1]);
    assert_ne!(code, 0, "an unsigned severity downgrade must FAIL the audit: {out}");
    assert!(out.contains("probeRule") || out.to_lowercase().contains("weaken"), "the failure names the rule or the weakening: {out}");

    // ALLOW AGAIN: restore, then downgrade WITH a marked Decision in the same commit.
    std::fs::write(&rule, rule_text("blocking")).expect("restore");
    let c3 = commit_all(&root, "restore");
    std::fs::write(&rule, rule_text("warning")).expect("downgrade signed");
    std::fs::write(
        root.join(".engine").join("decisions").join("0001-signed.sysml"),
        "package Decision0001 {\n    #SafetyChange part d0001 : Decision { :>> id = \"00000000-0000-4000-8000-000000000001\"; :>> title = \"signed downgrade\"; }\n}\n",
    )
    .expect("decision");
    commit_all(&root, "weaken, signed");
    let (code, out) = keel(&root, &["audit-adherence", "--since", &c3]);
    assert_eq!(code, 0, "a downgrade co-committed with a marked Decision is the sanctioned path and must pass: {out}");
    let _ = std::fs::remove_dir_all(&root);
}
