//! The `.claude/` enforcement surface — engine-derived, keel-scope-owned, shipped by `keel init`
//! (D0174/D-P0a, D0175/D-P0b; proposal §5-P0).
//!
//! WHY THIS EXISTS: the in-loop half of keel's discipline — hooks, the output style, permission
//! rules — lived only in the self-build's `.claude/`, so a downstream `keel init` project got the
//! commit gate and nothing in the loop (proposal §1.1: shipped-but-undiscoverable skills, a
//! pre-commit that skipped silently without the binary). This module is the ONE generator for the
//! keel-owned subset of that surface, used by `init` (fresh scaffold), by `sync-claude`
//! (regenerate/merge in place), and by the `claude-surface-drift` guard (`sync-claude --check`) —
//! one implementation, one surface (D-P0a).
//!
//! OWNERSHIP MODEL, not byte-equality: `settings.json` is mixed-ownership by construction — users
//! add their own hooks and permissions. Merge owns ONLY keel-identified entries (hook commands that
//! invoke `keel hook`, the keel output style); everything foreign survives byte-for-byte. The
//! generator version is stamped in a sidecar (`.claude/.keel-surface`), and version skew reports as
//! a REGENERATE obligation, never as a violation.
//!
//! FAIL-LOUD (K2/P0.3): every scaffolded hook resolves the binary via `KEEL_BIN` (absolute,
//! injected) then PATH — never a cwd-relative `target/` probe; a missing binary emits a visible
//! warning naming the install path, never `|| true` silence. The protected-path `PreToolUse` test is
//! PURE SHELL (no binary needed): with the binary absent it denies by default with a loud message.

use std::fmt::Write as _;
use std::path::Path;

/// The generator version stamped into the sidecar — the binary's own version.
pub const SURFACE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The documented install path (D-P0b fence: this URL and the docs, nothing else — no package
/// managers, no auto-update, no install scripts).
pub const INSTALL_URL: &str = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/releases";

/// The keel output style — the response contract (D0130), embedded from the self-build's own copy
/// so downstream ships EXACTLY what the self-build runs (K3 parity by construction).
pub const OUTPUT_STYLE: &str = include_str!("../../.claude/output-styles/keel.md");

/// Protected fact surfaces and the sanctioned command each refusal names (K13 deny-and-provide).
///
/// Owned by D-P0a; P1/D-P1a layers ask/override semantics on top. Paths are substring-matched with
/// both separators.
pub const PROTECTED_PATHS: [(&str, &str); 5] = [
    (".tracking/issues", "keel record issue"), // prefix: covers issues.sysml AND per-actor issues-<actor>.sysml (issue210)
    (".tracking/backlog.sysml", "keel add-task"),
    (".tracking/critiques", "the disposition write API (POST /api/disposition) or keel apply-review"), // prefix: covers per-actor files too (issue210)
    (".tracking/delivery/", "keel new sprint / keel append-gate-result / keel append-result"),
    (".engine/decisions/", "keel record decision, accepted only via keel accept (human-only)"),
];

/// Control-plane surfaces (D0179/K7): a write here weakens or redirects enforcement.
///
/// Approval-gated (ask) and recorded — never a quiet config edit. The pure-shell test covers them
/// too, so with the binary absent a control-plane write is denied rather than silently allowed.
pub const CONTROL_PLANE_PATHS: [&str; 3] = [".claude/settings.json", ".githooks/", "output-styles/keel.md"];

/// POSIX-sh binary resolution used by every scaffolded hook: `KEEL_BIN` (absolute, injected by the
/// launcher) then PATH; missing → a VISIBLE warning naming the install path, exit 0 (D0134: a hook
/// never fails a turn on infrastructure absence — the warning is the K2 visibility, and the
/// hardening inventory lists this point's absent-behavior).
fn resolver() -> String {
    format!(
        "K=\"${{KEEL_BIN:-}}\"; {{ [ -n \"$K\" ] && [ -x \"$K\" ]; }} || K=$(command -v keel 2>/dev/null); \
         [ -n \"$K\" ] || {{ echo '[keel] keel binary NOT FOUND - this gate did not run. Set KEEL_BIN or install: {INSTALL_URL}' >&2; exit 0; }}; "
    )
}

/// The resolver for a GATING hook (D0279): a missing binary BLOCKS instead of allowing.
///
/// # Why the Stop hook gets its own resolver
///
/// D0134's rule — a hook never fails a turn on infrastructure absence — is right for the ADVISORY
/// hooks: a routing reminder or a shell tip that cannot run should cost nothing. It was wrong for the
/// turn gate, and the five-critic rigor panel (2026-09-01) named the consequence: of twelve
/// enforcement points, two failed closed, and the one relied on to stop a rogue turn switched itself
/// off on a PATH problem, silently — printing "this gate did not run" and then permitting. That is
/// the exact shape of "we lack the authority to drive our own process". The binary was locked twice
/// that day during ordinary rebuilds; each time the gate did not run and each time the turn ended
/// green.
///
/// The precedent already existed: `pre-write` has been fail-closed since it shipped, in D0134's own
/// enforcement table. The line this draws is the principled one — GATING hooks fail closed, ADVISORY
/// hooks fail open — and the Stop hook is a gate.
///
/// THE COST, stated: a project whose keel binary is genuinely absent cannot end a turn until it is
/// installed. That is loud, immediate, and says exactly what to do — the correct failure mode for a
/// project that has declared it runs under keel. The alternative was an enforcement layer that
/// disabled itself and reported nothing.
fn gating_resolver() -> String {
    format!(
        "K=\"${{KEEL_BIN:-}}\"; {{ [ -n \"$K\" ] && [ -x \"$K\" ]; }} || K=$(command -v keel 2>/dev/null); \
         [ -n \"$K\" ] || {{ printf '%s' '{{\"decision\":\"block\",\"reason\":\"[keel] The turn gate could not run: the keel binary is NOT FOUND. This project runs under keel, so a turn cannot end ungated. Set KEEL_BIN or install: {INSTALL_URL}\"}}'; exit 0; }}; "
    )
}

/// One keel-owned hook entry.
fn hook_entry(cmd: &str, timeout: u64, status: &str) -> serde_json::Value {
    serde_json::json!({
        "type": "command",
        "shell": "bash",
        "command": cmd,
        "timeout": timeout,
        "statusMessage": status
    })
}

/// The PURE-SHELL protected-path `PreToolUse` command (`Write|Edit` matcher): no binary needed to deny.
/// With the binary present it defers to `keel hook pre-write` (profile-aware: strict denies naming
/// the sanctioned command; guided/self-build advises). With the binary ABSENT it denies by default,
/// loudly — a blind write to a fact surface with no gate running is the one thing that must not
/// pass silently (K2).
fn protected_path_command() -> String {
    let mut pats = String::new();
    for p in PROTECTED_PATHS.iter().map(|(p, _)| *p).chain(CONTROL_PLANE_PATHS) {
        let win = p.replace('/', "\\\\");
        let _ = write!(pats, "*'{p}'*|*'{win}'*|");
    }
    let pats = pats.trim_end_matches('|');
    format!(
        "IN=$(cat); case \"$IN\" in {pats}) \
         K=\"${{KEEL_BIN:-}}\"; {{ [ -n \"$K\" ] && [ -x \"$K\" ]; }} || K=$(command -v keel 2>/dev/null); \
         if [ -n \"$K\" ]; then printf '%s' \"$IN\" | \"$K\" hook pre-write; \
         else echo '{{\"hookSpecificOutput\":{{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"[keel] protected fact surface and the keel binary is ABSENT - refusing a blind write. Install: {INSTALL_URL} ; then use the sanctioned write commands (keel --help).\"}}}}'; fi;; \
         *) : ;; esac"
    )
}

/// The keel-owned `hooks` object — FIVE events (D-P0a).
fn keel_hooks() -> serde_json::Value {
    let r = resolver();
    let g = gating_resolver();
    serde_json::json!({
        "UserPromptSubmit": [{ "hooks": [hook_entry(&format!("{r}\"$K\" hook user-prompt"), 30, "route-first checklist")] }],
        "PostToolUse": [{ "matcher": "Write|Edit", "hooks": [hook_entry(&format!("{r}\"$K\" hook post-edit"), 60, "keel fast gate")] }],
        "Stop": [{ "hooks": [hook_entry(&format!("{g}\"$K\" hook stop"), 180, "keel turn gate")] }],
        "PreToolUse": [
            { "matcher": "Bash", "hooks": [hook_entry(&format!("{r}\"$K\" hook pre-bash"), 30, "shell adaptation advisory")] },
            { "matcher": "Write|Edit", "hooks": [hook_entry(&protected_path_command(), 30, "protected-path check")] }
        ],
        "SubagentStop": [{ "hooks": [hook_entry(&format!("{r}\"$K\" hook subagent-stop"), 120, "subagent tree gate")] }]
    })
}

/// Is this hook entry keel-owned? By content, not position: it invokes `keel hook` (through the
/// resolver or directly) or carries the pure-shell protected-path signature.
fn is_keel_hook(h: &serde_json::Value) -> bool {
    h.get("command")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|c| c.contains("hook user-prompt") || c.contains("hook post-edit") || c.contains("hook stop") || c.contains("hook pre-bash") || c.contains("hook subagent-stop") || c.contains("hook pre-write") || c.contains("permissionDecision"))
}

/// Deep-merge the keel-owned subset into `existing` settings. Foreign entries survive untouched;
/// keel-owned entries are REPLACED (idempotent: merging twice equals merging once).
#[must_use]
pub fn merge_settings(existing: &serde_json::Value) -> serde_json::Value {
    let mut out = if existing.is_object() { existing.clone() } else { serde_json::json!({}) };
    if let Some(obj) = out.as_object_mut() {
        obj.insert("outputStyle".to_string(), serde_json::json!("keel"));
        if !obj.get("hooks").is_some_and(serde_json::Value::is_object) {
            obj.insert("hooks".to_string(), serde_json::json!({}));
        }
    }
    let ours = keel_hooks();
    let Some(hooks) = out.get_mut("hooks").and_then(serde_json::Value::as_object_mut) else {
        return out; // unreachable by construction above; typed rather than panicking
    };
    let Some(our_events) = ours.as_object() else { return out };
    for (event, our_groups) in our_events {
        let mut kept: Vec<serde_json::Value> = hooks
            .get(event)
            .and_then(serde_json::Value::as_array)
            .map(|groups| {
                groups
                    .iter()
                    .filter(|g| {
                        !g.get("hooks")
                            .and_then(serde_json::Value::as_array)
                            .is_some_and(|hs| hs.iter().any(is_keel_hook))
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        kept.extend(our_groups.as_array().into_iter().flatten().cloned());
        hooks.insert(event.clone(), serde_json::Value::Array(kept));
    }
    out
}

/// The scaffolded single-vendor CI workflow (P0.1): layer 3's hook-independent downstream home.
/// The `rules` step's blocking behavior activates with P1.5; until then it reports.
pub const CI_TEMPLATE: &str = r"# keel gate - scaffolded by `keel init` (optional; delete if your CI wires keel itself).
# Layer-3 verification, hook-independent (K15): validate + guards + rules + audit-history.
name: keel-gate
on: [push, pull_request]
jobs:
  keel:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { fetch-depth: 0 }
      - name: install keel
        run: |
          echo 'Install the released keel binary for linux and put it on PATH.'
          echo 'Releases: https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/releases'
          echo 'Then delete this echo block.'
      - name: validate
        run: keel validate .
      - name: guards
        run: keel guard .
      - name: rules
        run: keel rules . --enforce
      - name: audit-history
        run: keel audit-history --since origin/main || true
";

/// Report from [`sync_claude`].
pub struct SyncReport {
    pub wrote_settings: bool,
    pub skills_written: usize,
    pub registry_count: usize,
    pub version_skew: Option<(String, String)>,
    pub drift: Vec<String>,
}

/// Registry entries: `(title, location)` from `skills-registry.sysml` — the count the scaffold must
/// match (P0.1 "counts asserted equal").
fn registry_skills(root: &Path) -> Vec<(String, String)> {
    // D0222: a skill may declare itself BESIDE itself, so every `.sysml` under `.engine/skills/` is
    // a registry — not just the central file. Reading only the central one made `sync-claude` report
    // "0/0 skill(s)" the moment the declarations moved, and `claude-surface-drift` then passed
    // VACUOUSLY on a population of zero: a false green, which is worse than the failure it hid.
    let mut text = String::new();
    for f in crate::collect_sysml(&root.join(".engine").join("skills")) {
        if let Ok(s) = std::fs::read_to_string(&f) {
            text.push('\n');
            text.push_str(&s);
        }
    }
    let mut out = Vec::new();
    let mut title = None;
    for line in text.lines() {
        let l = line.trim_start();
        if let Some(v) = l.strip_prefix(":>> title = \"") {
            title = v.split('"').next().map(str::to_string);
        }
        if let Some(v) = l.strip_prefix(":>> location = \"") {
            if let (Some(t), Some(loc)) = (title.take(), v.split('"').next()) {
                out.push((t, loc.to_string()));
            }
        }
    }
    out
}

/// Write (or merge into) the keel-owned `.claude/` surface at `root`.
///
/// `check_only`: compute drift and report, write nothing — this IS the `claude-surface-drift`
/// guard's implementation (one surface, one check).
///
/// # Errors
/// io/serde failures, and a skills-count mismatch (registry vs written) — the P0.1 assertion.
pub fn sync_claude(root: &Path, check_only: bool) -> Result<SyncReport, String> {
    let claude = root.join(".claude");
    let settings_path = claude.join("settings.json");
    let existing: serde_json::Value = match std::fs::read_to_string(&settings_path) {
        Ok(t) => serde_json::from_str(&t).map_err(|e| format!("{}: not valid JSON: {e}", settings_path.display()))?,
        Err(_) => serde_json::json!({}),
    };
    let merged = merge_settings(&existing);
    let mut drift = Vec::new();
    if existing != merged {
        drift.push("settings.json: keel-owned entries differ from this binary's generator".to_string());
    }
    // sidecar version stamp
    let sidecar = claude.join(".keel-surface");
    let recorded = std::fs::read_to_string(&sidecar).ok().map(|s| s.trim().to_string());
    let version_skew = match recorded {
        Some(v) if v != SURFACE_VERSION => Some((v, SURFACE_VERSION.to_string())),
        None => Some(("unstamped".to_string(), SURFACE_VERSION.to_string())),
        _ => None,
    };
    // output style
    let style_path = claude.join("output-styles").join("keel.md");
    let style_current = std::fs::read_to_string(&style_path).unwrap_or_default();
    if style_current != OUTPUT_STYLE {
        drift.push("output-styles/keel.md differs from the embedded response contract".to_string());
    }
    // skills: one .claude/skills/<name>/SKILL.md per registry entry
    let registry = registry_skills(root);
    let mut missing_skills = 0usize;
    for (title, loc) in &registry {
        let dst = claude.join("skills").join(title).join("SKILL.md");
        let src_text = std::fs::read_to_string(root.join(loc)).unwrap_or_default();
        if std::fs::read_to_string(&dst).unwrap_or_default() != src_text {
            missing_skills += 1;
        }
    }
    if missing_skills > 0 {
        drift.push(format!("{missing_skills} skill(s) missing or stale under .claude/skills/"));
    }

    if check_only {
        return Ok(SyncReport {
            wrote_settings: false,
            skills_written: 0,
            registry_count: registry.len(),
            version_skew,
            drift,
        });
    }

    std::fs::create_dir_all(claude.join("output-styles")).map_err(|e| e.to_string())?;
    crate::write::write_atomic(&settings_path, &serde_json::to_string_pretty(&merged).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    crate::write::write_atomic(&style_path, OUTPUT_STYLE).map_err(|e| e.to_string())?;
    let mut skills_written = 0usize;
    for (title, loc) in &registry {
        let src_text = std::fs::read_to_string(root.join(loc))
            .map_err(|e| format!("registry names {loc}, which cannot be read: {e} - the tool-reference guard should have caught a missing file"))?;
        let dst_dir = claude.join("skills").join(title);
        std::fs::create_dir_all(&dst_dir).map_err(|e| e.to_string())?;
        crate::write::write_atomic(&dst_dir.join("SKILL.md"), &src_text).map_err(|e| e.to_string())?;
        skills_written += 1;
    }
    if skills_written != registry.len() {
        return Err(format!(
            "skills count mismatch: registry declares {} but {skills_written} written (P0.1 asserts these equal)",
            registry.len()
        ));
    }
    crate::write::write_atomic(&sidecar, format!("{SURFACE_VERSION}\n")).map_err(|e| e.to_string())?;
    Ok(SyncReport { wrote_settings: true, skills_written, registry_count: registry.len(), version_skew: None, drift: Vec::new() })
}

#[cfg(test)]
mod tests {
    use super::{keel_hooks, merge_settings, protected_path_command, resolver, PROTECTED_PATHS};

    /// D-P0a: five events, every command KEEL_BIN-then-PATH, never a cwd-relative target/ probe,
    /// and the missing-binary branch is loud and names the install path.
    #[test]
    fn generated_hooks_have_five_events_and_fail_loud_resolution() {
        let h = keel_hooks();
        for ev in ["UserPromptSubmit", "PostToolUse", "Stop", "PreToolUse", "SubagentStop"] {
            assert!(h.get(ev).is_some(), "missing event {ev}");
        }
        let text = h.to_string();
        assert!(!text.contains("./target/"), "cwd-relative binary probe is the P0.3 forbidden pattern");
        assert!(text.contains("KEEL_BIN"), "KEEL_BIN resolution missing");
        assert!(resolver().contains("NOT FOUND"), "missing-binary branch must be loud");
        assert!(resolver().contains("releases"), "the warning must name the install path");
        // D0279: the TURN GATE fails closed on a missing binary; the advisory hooks still fail open.
        let stop_cmd = h["Stop"][0]["hooks"][0]["command"].as_str().expect("stop command");
        assert!(
            stop_cmd.contains("\"decision\":\"block\"") && stop_cmd.contains("NOT FOUND"),
            "a missing binary must BLOCK the turn, not print and allow: {stop_cmd}"
        );
        let prompt_cmd = h["UserPromptSubmit"][0]["hooks"][0]["command"].as_str().expect("prompt command");
        assert!(
            !prompt_cmd.contains("\"decision\":\"block\""),
            "an ADVISORY hook still fails open — D0134 holds for advice: {prompt_cmd}"
        );
    }

    /// The pure-shell protected-path test needs NO binary to deny, matches both separators, and its
    /// deny reason names the install path (K2 + K13).
    #[test]
    fn protected_path_command_denies_without_binary() {
        let c = protected_path_command();
        for (p, _) in PROTECTED_PATHS {
            assert!(c.contains(p), "missing pattern {p}");
        }
        assert!(c.contains("permissionDecision"), "deny JSON missing");
        assert!(c.contains("ABSENT"), "the binary-absent branch must say so");
        assert!(c.contains("pre-write"), "binary-present branch must defer to keel hook pre-write");
    }

    /// Mixed ownership: a foreign hook group and foreign top-level settings survive the merge
    /// byte-for-byte; merging twice equals merging once (idempotent).
    #[test]
    fn merge_preserves_foreign_entries_and_is_idempotent() {
        let existing = serde_json::json!({
            "permissions": { "deny": ["Skill(se)"] },
            "myCustomKey": 42,
            "hooks": {
                "Stop": [
                    { "hooks": [{ "type": "command", "command": "my-own-linter --fix" }] },
                    { "hooks": [{ "type": "command", "command": "old resolver; \"$K\" hook stop" }] }
                ]
            }
        });
        let once = merge_settings(&existing);
        assert_eq!(once["permissions"]["deny"][0], "Skill(se)", "foreign permission must survive");
        assert_eq!(once["myCustomKey"], 42, "foreign top-level key must survive");
        let stops = once["hooks"]["Stop"].as_array().expect("stop groups");
        assert!(
            stops.iter().any(|g| g.to_string().contains("my-own-linter")),
            "foreign hook group must survive"
        );
        assert_eq!(
            stops.iter().filter(|g| g.to_string().contains("hook stop")).count(),
            1,
            "the stale keel entry is REPLACED, not duplicated"
        );
        let twice = merge_settings(&once);
        assert_eq!(once, twice, "merge must be idempotent");
    }
}
