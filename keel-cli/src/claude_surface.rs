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

/// The binary-resolution ORDER every keel surface agrees on (D0230; issue348/GH#43).
///
/// `KEEL_BIN`, then the project's PINNED binary at `.keel/bin/keel(.exe)`, then PATH. The scaffolded
/// git hooks probe in this order; until D0316 the Claude hooks skipped the pin and a project pinned to
/// 0.3.0 ran its turns on whatever PATH held - including an uncommitted build. One list, so the two
/// surfaces cannot disagree again; `every_hook_command_resolves_the_pin_before_path_in_the_git_hooks_order`
/// holds them equal.
pub const RESOLUTION_ORDER: [&str; 3] = ["KEEL_BIN", ".keel/bin/keel", "command -v"];

/// The POSIX-sh probe realising `RESOLUTION_ORDER` inside a hook command. The Claude hooks run with
/// the project directory as cwd and `CLAUDE_PROJECT_DIR` set, so the pin is probed under both.
fn pin_first_probe() -> String {
    "K=\"${KEEL_BIN:-}\"; { [ -n \"$K\" ] && [ -x \"$K\" ]; } || { K=; for c in \"${CLAUDE_PROJECT_DIR:-.}/.keel/bin/keel\" \"${CLAUDE_PROJECT_DIR:-.}/.keel/bin/keel.exe\"; do [ -x \"$c\" ] && { K=\"$c\"; break; }; done; }; [ -n \"$K\" ] || K=$(command -v keel 2>/dev/null); ".to_string()
}

/// POSIX-sh binary resolution used by every scaffolded hook: `KEEL_BIN` (absolute, injected by the
/// launcher), then the pinned `.keel/bin` binary, then PATH (`RESOLUTION_ORDER`); missing → a VISIBLE
/// warning naming the install path, exit 0 (D0134: a hook never fails a turn on infrastructure
/// absence — the warning is the K2 visibility, and the hardening inventory lists this point's
/// absent-behavior).
fn resolver() -> String {
    let pin_first = pin_first_probe();
    format!(
        "{pin_first}\
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
    let pin_first = pin_first_probe();
    format!(
        "{pin_first}\
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
    let pin_first = pin_first_probe();
    format!(
        "IN=$(cat); case \"$IN\" in {pats}) \
         {pin_first}\
         if [ -n \"$K\" ]; then printf '%s' \"$IN\" | \"$K\" hook pre-write; \
         else echo '{{\"hookSpecificOutput\":{{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"[keel] protected fact surface and the keel binary is ABSENT - refusing a blind write. Install: {INSTALL_URL} ; then use the sanctioned write commands (keel --help).\"}}}}'; fi;; \
         *) : ;; esac"
    )
}

/// The keel-owned `hooks` object — SIX events (D-P0a, then `ConfigChange` under D0296).
fn keel_hooks() -> serde_json::Value {
    let r = resolver();
    let g = gating_resolver();
    serde_json::json!({
        "ConfigChange": [{ "matcher": "project_settings|local_settings", "hooks": [hook_entry(&format!("{r}\"$K\" hook config-change"), 30, "hook kill-switch guard")] }],
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
        .is_some_and(|c| c.contains("hook user-prompt") || c.contains("hook post-edit") || c.contains("hook stop") || c.contains("hook pre-bash") || c.contains("hook subagent-stop") || c.contains("hook pre-write") || c.contains("hook config-change") || c.contains("permissionDecision"))
}

/// The one settings key that silences EVERY hook from whichever scope sets it.
///
/// Keel's, any plugin's and the user's alike (issue365 / D0296 runs 2 and 5). Claude Code reads it from any file and
/// hook lists merge across scopes, so a repo-scope `true` beside byte-identical keel entries turns
/// the whole in-loop tier off while an entry-by-entry drift check reads clean.
pub const HOOK_KILL_SWITCH: &str = "disableAllHooks";

/// The repo-scope settings files Claude Code reads for a project - both are the agent's to edit.
pub const REPO_SCOPE_SETTINGS: [&str; 2] = [".claude/settings.json", ".claude/settings.local.json"];

/// Does this settings document silence the hooks? Truthy `disableAllHooks` at top level.
#[must_use]
pub fn hooks_silenced(settings: &serde_json::Value) -> bool {
    settings
        .get(HOOK_KILL_SWITCH)
        .is_some_and(|v| v.as_bool() == Some(true) || v.as_str().is_some_and(|s| s.eq_ignore_ascii_case("true")))
}

/// Does this text - a Write's `content` or an Edit's `new_string` - set the kill switch?
///
/// Textual on purpose: a partial Edit is not a JSON document, and a refusal here is cheap.
#[must_use]
pub fn text_sets_kill_switch(text: &str) -> bool {
    let Some(i) = text.find(HOOK_KILL_SWITCH) else { return false };
    let rest = &text[i + HOOK_KILL_SWITCH.len()..];
    let rest = rest.trim_start_matches(|c: char| c == '"' || c == '\'' || c == '\\' || c == ':' || c.is_whitespace());
    rest.starts_with("true")
}

/// Where the repository carries the keel PLUGIN - the enforcement copy of the hook set (D0296).
///
/// Rendered by the same generator as the repo's `settings.json` entries, so the hook set has ONE
/// source and two renderings; `sync-claude --check` reports drift in either. A launch passes
/// `--plugin-dir` at this path (or a marketplace install of it), which a repo-scope settings file
/// cannot remove: hook lists merge across scopes.
pub const PLUGIN_DIR: &str = ".engine/claude-plugin";
/// The marketplace manifest at the repository root, so a project's settings can carry the plugin in
/// `extraKnownMarketplaces` / `enabledPlugins` (both are settable from any scope).
pub const MARKETPLACE_MANIFEST: &str = ".claude-plugin/marketplace.json";
/// The plugin's name in both manifests.
pub const PLUGIN_NAME: &str = "keel";

/// `.claude-plugin/plugin.json` for the shipped plugin.
#[must_use]
pub fn plugin_manifest() -> serde_json::Value {
    serde_json::json!({
        "name": PLUGIN_NAME,
        "version": SURFACE_VERSION,
        "description": "keel's in-loop tier hosted above the repository: the same hook set the repo's settings.json declares, rendered by the same generator (D0296).",
        "author": {"name": "keel"},
        "homepage": INSTALL_URL,
        "keywords": ["keel", "hooks", "gate", "sysml"]
    })
}

/// `hooks/hooks.json` for the shipped plugin - the keel hook set, unchanged.
#[must_use]
pub fn plugin_hooks() -> serde_json::Value {
    serde_json::json!({"hooks": keel_hooks()})
}

/// `.claude-plugin/marketplace.json` at the repository root, naming the plugin by relative source.
#[must_use]
pub fn marketplace_manifest() -> serde_json::Value {
    serde_json::json!({
        "name": PLUGIN_NAME,
        "owner": {"name": "keel"},
        "metadata": {"description": "The keel engine's Claude Code plugin: the hook set as an enforcement copy a repo-scope settings file cannot remove (D0296)."},
        "plugins": [{
            "name": PLUGIN_NAME,
            "source": format!("./{PLUGIN_DIR}"),
            "description": "keel hooks: post-edit fast gate, turn gate, protected-path check, shell advisory, subagent gate.",
            "version": SURFACE_VERSION
        }]
    })
}

/// What `keel hook config-change` writes back for a repo-scope settings document, if anything.
///
/// `settings.json` is keel-generated: the keel-owned subset is re-merged (which also strips the
/// kill switch). `settings.local.json` is the user's: only the kill switch is removed. `None` means
/// the document needs nothing - the change is allowed. The payload carries only `file_path` (D0296
/// run 3), so the caller reads the file; a blocked `ConfigChange` is NOT reverted by Claude Code, so
/// the hook restores the file itself or the key survives to the next launch (run 5).
#[must_use]
pub fn restored_settings(doc: &serde_json::Value, is_local: bool) -> Option<(serde_json::Value, &'static str)> {
    if is_local {
        if !hooks_silenced(doc) {
            return None;
        }
        let mut out = doc.clone();
        if let Some(obj) = out.as_object_mut() {
            obj.remove(HOOK_KILL_SWITCH);
        }
        return Some((out, "the hook kill switch was set"));
    }
    let merged = merge_settings(doc);
    if hooks_silenced(doc) {
        return Some((merged, "the hook kill switch was set"));
    }
    if merged != *doc {
        return Some((merged, "a keel-owned hook entry was altered or removed"));
    }
    None
}

/// The three plugin files as `(relative path, pretty JSON)` - the single list both the writer and
/// the drift check walk, so neither can forget one.
#[must_use]
pub fn plugin_files() -> Vec<(String, String)> {
    let pretty = |v: &serde_json::Value| serde_json::to_string_pretty(v).unwrap_or_default() + "\n";
    vec![
        (format!("{PLUGIN_DIR}/.claude-plugin/plugin.json"), pretty(&plugin_manifest())),
        (format!("{PLUGIN_DIR}/hooks/hooks.json"), pretty(&plugin_hooks())),
        (MARKETPLACE_MANIFEST.to_string(), pretty(&marketplace_manifest())),
    ]
}

/// Deep-merge the keel-owned subset into `existing` settings.
///
/// Foreign entries survive untouched; keel-owned entries are REPLACED (idempotent: merging twice
/// equals merging once). The kill switch is keel-owned in the negative - it must be ABSENT - so
/// `sync-claude` removes it (issue365).
#[must_use]
pub fn merge_settings(existing: &serde_json::Value) -> serde_json::Value {
    let mut out = if existing.is_object() { existing.clone() } else { serde_json::json!({}) };
    if let Some(obj) = out.as_object_mut() {
        obj.insert("outputStyle".to_string(), serde_json::json!("keel"));
        obj.remove(HOOK_KILL_SWITCH);
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
pub const CI_TEMPLATE: &str = r#"# keel gate - scaffolded by `keel init` (optional; delete if your CI wires keel itself).
# Layer-3 verification, hook-independent (K15): validate + guards + rules + audit-history.
name: keel-gate
on: [push, pull_request]
jobs:
  keel:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { fetch-depth: 0 }
      # Installs the version this project PINS, not the latest (D0281, issue327). The previous step here
      # was three echo lines that installed nothing, so every fresh project's gate failed on its first
      # push with `keel: command not found` — and read as "the placeholder nobody deleted" rather than
      # "your gate is not running". Latest would be wrong too: the pin is BINDING and validate/guard
      # REFUSE under skew (D0251), so the runner must run what the project declared.
      - name: install keel (the pinned release)
        run: |
          set -euo pipefail
          PIN=$(sed -nE 's/^engine *= *"([^"]+)".*/\1/p' .engine/contracts/engine-version.toml)
          [ -n "$PIN" ] || { echo "::error::no engine pin in .engine/contracts/engine-version.toml"; exit 1; }
          curl -fsSL -o keel "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/releases/download/v${PIN}/keel-linux-x86_64"
          chmod +x keel && sudo mv keel /usr/local/bin/keel
          keel version
      - name: validate
        run: keel validate .
      - name: guards
        run: keel guard .
      - name: rules
        run: keel rules . --enforce
      - name: audit-history
        run: keel audit-history --since origin/main || true
"#;

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
    // The kill switch is named on its own line: "entries differ" would be true too and would hide
    // that every hook is off (issue365). settings.local.json is read as well - not keel-generated,
    // but it silences the hooks from the same repo-scope position.
    for rel in REPO_SCOPE_SETTINGS {
        let text = std::fs::read_to_string(root.join(rel)).unwrap_or_default();
        let doc: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
        if hooks_silenced(&doc) {
            drift.push(format!(
                "{rel}: {HOOK_KILL_SWITCH} is set - EVERY keel hook is silenced from this repo-scope file (Claude Code reads the key from any scope; issue365/D0296). Remove the key - `keel sync-claude` removes it."
            ));
        }
    }
    if existing != merged && !hooks_silenced(&existing) {
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

    // the plugin rendering (D0296): every file byte-equal to this binary's generator
    let mut plugin_stale = Vec::new();
    for (rel, want) in plugin_files() {
        if std::fs::read_to_string(root.join(&rel)).unwrap_or_default().replace("\r\n", "\n") != want {
            plugin_stale.push(rel);
        }
    }
    if !plugin_stale.is_empty() {
        drift.push(format!("plugin rendering stale or missing: {} (D0296 - the same hook set, second rendering)", plugin_stale.join(", ")));
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
    for (rel, text) in plugin_files() {
        let dst = root.join(&rel);
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        crate::write::write_atomic(&dst, &text).map_err(|e| e.to_string())?;
    }
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
    use super::{
        hooks_silenced, keel_hooks, marketplace_manifest, merge_settings, plugin_files, plugin_hooks, protected_path_command, resolver, restored_settings,
        sync_claude, text_sets_kill_switch, HOOK_KILL_SWITCH, MARKETPLACE_MANIFEST, PLUGIN_DIR, PROTECTED_PATHS, RESOLUTION_ORDER,
    };

    /// D-P0a: six events (five, then `ConfigChange` under D0296), every command KEEL_BIN-then-PATH, never a cwd-relative target/ probe,
    /// and the missing-binary branch is loud and names the install path.
    /// issue327: the scaffolded gate's install step was three `echo` lines that installed nothing, so
    /// every fresh project's own gate failed on its first push with `keel: command not found` — and read
    /// as "the placeholder nobody deleted" rather than "your gate is not running". The step must now
    /// INSTALL, and it must install the PINNED version: latest would put the runner in D0251 skew and
    /// every gate step would refuse.
    #[test]
    fn the_scaffolded_gate_installs_the_pinned_release_not_an_echo() {
        let t = super::CI_TEMPLATE;
        assert!(!t.contains("Then delete this echo block"), "the placeholder must be gone");
        assert!(t.contains(".engine/contracts/engine-version.toml"), "the install must READ THE PIN, not fetch latest");
        assert!(t.contains("releases/download/v${PIN}/keel-linux-x86_64"), "and fetch that exact release asset");
        assert!(t.contains("keel version"), "and prove the binary runs before any gate step trusts it");
        // The gate steps that follow still name the binary they now actually have.
        for step in ["keel validate .", "keel guard .", "keel rules . --enforce"] {
            assert!(t.contains(step), "gate step present: {step}");
        }
    }

    /// issue348 (GH#43): every generated hook command probes `KEEL_BIN`, then the PINNED `.keel/bin`
    /// binary, then PATH - the order the scaffolded git hooks use - so a project's turns and its gates
    /// run the same engine. Asserted per command, not once for the file, so a new hook cannot regress it.
    #[test]
    fn every_hook_command_resolves_the_pin_before_path_in_the_git_hooks_order() {
        let h = keel_hooks();
        let mut commands = 0usize;
        for (_, groups) in h.as_object().expect("hooks object") {
            for g in groups.as_array().expect("groups") {
                for hk in g["hooks"].as_array().expect("hooks") {
                    let cmd = hk["command"].as_str().expect("command");
                    assert!(probes_in_order(cmd), "resolution order must be {RESOLUTION_ORDER:?} in: {cmd}");
                    commands += 1;
                }
            }
        }
        assert!(commands >= 6, "every event's command was checked: {commands}");
        // The scaffolded git hook probes in the same order (D0230) - the two surfaces agree.
        assert!(probes_in_order(crate::PRECOMMIT_HOOK), "the git hook probes in the same order");
        assert!(!probes_in_order("K=$(command -v keel); [ -x .keel/bin/keel ] && K=.keel/bin/keel; K=${KEEL_BIN:-$K}"), "a PATH-first script is refused by this check");
    }

    /// Does `script` probe the three resolution steps in `RESOLUTION_ORDER`, each AFTER the previous
    /// (so a comment naming a path earlier does not count as the probe)?
    fn probes_in_order(script: &str) -> bool {
        let mut from = 0usize;
        for tok in RESOLUTION_ORDER {
            match script[from..].find(tok) {
                Some(i) => from += i + tok.len(),
                None => return false,
            }
        }
        true
    }

    #[test]
    fn generated_hooks_have_six_events_and_fail_loud_resolution() {
        let h = keel_hooks();
        for ev in ["ConfigChange", "UserPromptSubmit", "PostToolUse", "Stop", "PreToolUse", "SubagentStop"] {
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
    /// D0296: what config-change restores - the kill switch and any altered keel entry in
    /// settings.json; only the kill switch in settings.local.json; nothing when nothing is wrong.
    #[test]
    fn restored_settings_repairs_exactly_what_was_broken() {
        let clean = merge_settings(&serde_json::json!({"theirs": 1}));
        assert!(restored_settings(&clean, false).is_none(), "a clean file needs nothing");
        let mut poisoned = clean.clone();
        poisoned["disableAllHooks"] = serde_json::json!(true);
        let (fixed, why) = restored_settings(&poisoned, false).expect("repair");
        assert!(why.contains("kill switch"));
        assert_eq!(fixed, clean, "settings.json is restored to the clean merge");
        let mut gutted = clean;
        gutted["hooks"].as_object_mut().expect("hooks").remove("Stop");
        let (fixed, why) = restored_settings(&gutted, false).expect("repair");
        assert!(why.contains("altered or removed"));
        assert!(fixed["hooks"].get("Stop").is_some(), "the removed keel entry is back");
        let local = serde_json::json!({"disableAllHooks": true, "permissions": {"allow": ["Bash"]}});
        let (fixed, _) = restored_settings(&local, true).expect("local repair");
        assert!(fixed.get("disableAllHooks").is_none() && fixed["permissions"]["allow"][0] == "Bash", "local: only the key leaves");
        assert!(restored_settings(&serde_json::json!({"permissions": {}}), true).is_none(), "a local file without the key is the user's");
    }

    /// D0296: the plugin is the SAME hook set - one generator, two renderings - and its three files
    /// are walked by one list for both the writer and the check.
    #[test]
    fn plugin_rendering_is_the_same_hook_set_and_checked() {
        assert_eq!(plugin_hooks()["hooks"], keel_hooks(), "plugin hooks.json carries the settings.json hook set unchanged");
        let files = plugin_files();
        assert_eq!(files.len(), 3);
        assert!(files.iter().any(|(p, _)| p.ends_with("plugin.json")) && files.iter().any(|(p, _)| p.ends_with("hooks.json")));
        let mp = marketplace_manifest();
        assert_eq!(mp["plugins"][0]["source"], format!("./{PLUGIN_DIR}"), "the marketplace names the plugin by its in-repo path");

        let root = std::env::temp_dir().join(format!("keel-plugin-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".claude")).expect("mkdir");
        std::fs::write(root.join(".claude").join("settings.json"), merge_settings(&serde_json::json!({})).to_string()).expect("clean");
        let before = sync_claude(&root, true).expect("check");
        assert!(before.drift.iter().any(|d| d.starts_with("plugin rendering stale or missing")), "no plugin yet = drift: {:?}", before.drift);
        sync_claude(&root, false).expect("write");
        let after = sync_claude(&root, true).expect("check again");
        assert!(!after.drift.iter().any(|d| d.starts_with("plugin rendering")), "written plugin = no plugin drift: {:?}", after.drift);
        assert!(root.join(PLUGIN_DIR).join("hooks").join("hooks.json").exists());
        assert!(root.join(MARKETPLACE_MANIFEST).exists());
        let _ = std::fs::remove_dir_all(&root);
    }

    /// issue365: the kill switch is stripped on merge, seen as drift on check, and NAMED - not
    /// folded into the generic "entries differ" line.
    #[test]
    fn kill_switch_is_stripped_named_and_detected() {
        let poisoned = serde_json::json!({"hooks": {}, "disableAllHooks": true, "theirs": 1});
        assert!(hooks_silenced(&poisoned));
        let merged = merge_settings(&poisoned);
        assert!(merged.get(HOOK_KILL_SWITCH).is_none(), "sync removes the kill switch: {merged}");
        assert_eq!(merged["theirs"], 1, "a foreign entry still survives");
        assert!(!hooks_silenced(&serde_json::json!({"disableAllHooks": false})));
        assert!(text_sets_kill_switch(r#"{"hooks": {}, "disableAllHooks": true}"#));
        assert!(text_sets_kill_switch(r#"\"disableAllHooks\":true"#));
        assert!(!text_sets_kill_switch(r#"{"disableAllHooks": false}"#));
        assert!(!text_sets_kill_switch("nothing about hooks"));

        let root = std::env::temp_dir().join(format!("keel-killswitch-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".claude")).expect("mkdir");
        std::fs::write(root.join(".claude").join("settings.json"), merge_settings(&serde_json::json!({})).to_string()).expect("clean");
        std::fs::write(root.join(".claude").join("settings.local.json"), r#"{"disableAllHooks": true}"#).expect("local");
        let report = sync_claude(&root, true).expect("check runs");
        assert!(
            report.drift.iter().any(|d| d.starts_with(".claude/settings.local.json") && d.contains(HOOK_KILL_SWITCH)),
            "the check must NAME the file and the key, got: {:?}",
            report.drift
        );
        let _ = std::fs::remove_dir_all(&root);
    }

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
