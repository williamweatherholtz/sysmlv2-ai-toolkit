//! `keel status` — one command that checks all bases (D0270, from st062).
//!
//! The human asked for "a full status output for state of everything including versioning, library
//! update new processes, etc. a command I can run to check all bases". Before this, answering that
//! meant running `version`, `library list`, `process show` per unit, `orient`, `open-issues` and
//! `gh run list`, then comparing their output by hand — which is the friction, not the information.
//!
//! # The rule every line here obeys
//!
//! **"Nothing to report" and "I cannot tell" must never render the same.** A library that cannot be
//! reached reports UNKNOWN, not zero drift; a repo with no CI configured says so rather than showing
//! a blank verdict. This is the pass-at-zero hazard in report form, and it has already been caught
//! three times elsewhere in this codebase — a status screen is exactly where it would do the most
//! damage, because its whole purpose is to be trusted at a glance.
//!
//! # It reads, it never writes
//!
//! No line here mutates the tree, syncs the library, or reaches for the network except to ask GitHub
//! for a run verdict. Status that changes what it reports is not status.

use std::path::Path;

/// One row of the readout: a section, its verdict, and the detail lines under it.
struct Section {
    label: &'static str,
    state: State,
    lines: Vec<String>,
}

/// A section's verdict. `Unknown` is a first-class outcome, not a fallback for `Ok`.
#[derive(PartialEq)]
enum State {
    Ok,
    Attention,
    Unknown,
}

impl State {
    const fn tag(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Attention => "ATTENTION",
            Self::Unknown => "UNKNOWN",
        }
    }

    /// The tag coloured for a terminal (D0287): OK green, ATTENTION red, UNKNOWN yellow - the
    /// three verdicts must never look alike at a glance. Padded on the bare text so colour codes do
    /// not shift the columns.
    fn painted(&self) -> String {
        let padded = format!("{:<10}", self.tag());
        match self {
            Self::Ok => crate::color::pass(&padded),
            Self::Attention => crate::color::fail(&padded),
            Self::Unknown => crate::color::warn(&padded),
        }
    }
}

/// Where the platform keeps managed (admin-deployed) Claude Code settings - the one hook host the
/// agent's shell cannot write. Read for presence only, never parsed for policy (that is the admin's).
const fn managed_settings_path() -> &'static str {
    if cfg!(windows) {
        "C:/Program Files/ClaudeCode/managed-settings.json"
    } else if cfg!(target_os = "macos") {
        "/Library/Application Support/ClaudeCode/managed-settings.json"
    } else {
        "/etc/claude-code/managed-settings.json"
    }
}

/// HOOKS — which hosts carry the keel hook set for this project, and whether anything silences them
/// (D0296). An out-of-hook check on purpose: a silenced hook cannot report itself (D0296 run 5), so
/// the one place that can say "your hooks are off" is a command the human runs by hand.
fn hooks_section(root: &Path) -> Section {
    use crate::claude_surface::{hooks_silenced, merge_settings, PLUGIN_DIR, REPO_SCOPE_SETTINGS};
    let mut lines = Vec::new();
    let mut state = State::Ok;
    // the kill switch first: it decides the verdict regardless of how many hosts exist
    for rel in REPO_SCOPE_SETTINGS {
        let text = std::fs::read_to_string(root.join(rel)).unwrap_or_default();
        let doc: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
        if hooks_silenced(&doc) {
            state = State::Attention;
            lines.push(format!("{rel} sets disableAllHooks - a plain `claude` launch runs NO hook; `keel sync-claude` removes it, `keel claude` overrides it"));
        }
    }
    // host 1: the repo-scope declaration
    let settings = root.join(".claude").join("settings.json");
    let declared = std::fs::read_to_string(&settings).is_ok_and(|text| {
        if let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) {
            let merged = merge_settings(&doc);
            let events = merged.get("hooks").and_then(serde_json::Value::as_object).map_or(0, serde_json::Map::len);
            if merged.get("hooks") == doc.get("hooks") {
                lines.push(format!("repo settings: {events} keel events declared"));
            } else {
                state = State::Attention;
                lines.push("repo settings: keel entries DRIFTED from this binary - `keel sync-claude`".to_string());
            }
        } else {
            state = State::Attention;
            lines.push("repo settings: .claude/settings.json is not JSON".to_string());
        }
        true
    });
    // host 2: the plugin rendering
    let plugin = root.join(PLUGIN_DIR).join("hooks").join("hooks.json").is_file();
    lines.push(if plugin {
        format!("plugin rendering: {PLUGIN_DIR} present (a settings edit cannot remove it)")
    } else {
        format!("plugin rendering: none at {PLUGIN_DIR} - `keel sync-claude` writes it")
    });
    // host 3: the launch pin
    lines.push(if root.join(".keel").join("launch-settings.json").is_file() {
        "launch pin: .keel/launch-settings.json written by a `keel claude` / console launch".to_string()
    } else {
        "launch pin: no keel launch on this machine yet (`keel claude` writes it)".to_string()
    });
    // host 4: managed settings - presence only
    lines.push(if Path::new(managed_settings_path()).is_file() {
        format!("managed settings: present at {} (admin-deployed; the one host the agent cannot write)", managed_settings_path())
    } else {
        "managed settings: none (optional, admin-deployed)".to_string()
    });
    if !declared && !plugin {
        state = State::Attention;
        lines.insert(0, "NO hook host: neither repo settings nor a plugin rendering - only the commit gate and CI gate this project; `keel sync-claude` creates both".to_string());
    }
    if state == State::Ok {
        let hosts = usize::from(declared) + usize::from(plugin);
        lines.insert(0, format!("{hosts} host(s) carry the hook set; nothing silences them"));
    }
    Section { label: "hooks", state, lines }
}

fn field(text: &str, key: &str) -> Option<String> {
    text.lines()
        .find_map(|l| l.trim().strip_prefix(key))
        .map(|v| v.trim().trim_start_matches('=').trim().trim_matches('"').to_string())
}

/// ENGINE — the binary's version against the project's binding pin (D0251).
fn engine_section(root: &Path) -> Section {
    let binary = env!("CARGO_PKG_VERSION").to_string();
    let pin_path = root.join(".engine/contracts/engine-version.toml");
    let Ok(text) = std::fs::read_to_string(&pin_path) else {
        return Section {
            label: "engine",
            state: State::Unknown,
            lines: vec![format!("binary {binary}; no pin file — this project declares no engine version")],
        };
    };
    let Some(pinned) = field(&text, "engine") else {
        return Section {
            label: "engine",
            state: State::Unknown,
            lines: vec![format!("binary {binary}; pin file present but declares no `engine =` line")],
        };
    };
    // D0336: the last unattended update attempt on this machine, where the human will see it.
    let attempt_line = crate::migrate::last_attempt(root).map(|a| {
        if a.outcome == "reverted" {
            format!("last update attempt: {} on {} REVERTED - {} failed: {}", a.version, a.at, a.gate, a.output.lines().find(|l| !l.trim().is_empty()).unwrap_or("").chars().take(120).collect::<String>())
        } else {
            format!("last update attempt: {} on {} retained (gate green)", a.version, a.at)
        }
    });
    if pinned == binary {
        let mut lines = vec![format!("{binary} (binary and pin agree)")];
        lines.extend(attempt_line);
        Section { label: "engine", state: State::Ok, lines }
    } else {
        let mut lines = vec![
            format!("binary {binary}, project PINS {pinned} — SKEW"),
            "the pin is binding: writes and gates REFUSE under skew. `keel migrate` re-stamps it".into(),
        ];
        lines.extend(attempt_line);
        Section { label: "engine", state: State::Attention, lines }
    }
}

/// The units this project has installed, name -> version, from its own install record.
fn installed(root: &Path) -> Option<Vec<(String, u32)>> {
    let text = std::fs::read_to_string(root.join(".engine/contracts/installed-units.toml")).ok()?;
    let mut out = Vec::new();
    let (mut name, mut version) = (None, None);
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            name = None;
            version = None;
        } else if let Some(v) = t.strip_prefix("process = ") {
            name = Some(v.trim().trim_matches('"').to_string());
        } else if let Some(v) = t.strip_prefix("version = ") {
            version = v.trim().parse().ok();
        }
        if let (Some(n), Some(v)) = (name.as_ref(), version) {
            out.push((n.clone(), v));
            name = None;
            version = None;
        }
    }
    Some(out)
}

/// LIBRARY — what the shared library holds versus what this project installed: units BEHIND, and
/// units AVAILABLE that the project has never imported (the human's "library update new processes").
fn library_section(root: &Path) -> Section {
    let Some(dir) = crate::library::clone_dir().filter(|d| d.join(".git").exists()) else {
        return Section {
            label: "library",
            state: State::Unknown,
            lines: vec!["not initialised on this machine — cannot tell whether anything is behind".into()],
        };
    };
    let mut available: Vec<(String, u32)> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let Ok(t) = std::fs::read_to_string(e.path().join("unit.toml")) else { continue };
            if let Some(v) = field(&t, "version").and_then(|v| v.parse().ok()) {
                available.push((e.file_name().to_string_lossy().to_string(), v));
            }
        }
    }
    let Some(mine) = installed(root) else {
        return Section {
            label: "library",
            state: State::Unknown,
            lines: vec![format!("{} unit(s) available; this project has NO install record, so drift cannot be computed", available.len())],
        };
    };
    let mut lines = vec![format!("{} unit(s) available, {} installed here", available.len(), mine.len())];
    let mut attention = false;
    let mut behind: Vec<String> = Vec::new();
    for (name, up) in &available {
        if let Some((_, have)) = mine.iter().find(|(n, _)| n == name) {
            if up > have {
                behind.push(format!("  behind: {name} v{have} -> v{up}   `keel process import --from-library {name} --update`"));
            }
        }
    }
    let mut new_units: Vec<String> = available
        .iter()
        .filter(|(n, _)| !mine.iter().any(|(m, _)| m == n))
        .map(|(n, v)| format!("  available, not installed: {n} v{v}   `keel process import --from-library {n}`"))
        .collect();
    behind.sort();
    new_units.sort();
    if !behind.is_empty() || !new_units.is_empty() {
        attention = true;
    }
    lines.extend(behind);
    lines.extend(new_units);
    if !attention {
        lines.push("  nothing behind, nothing new".into());
    }
    // The cache's age is part of the answer: everything above is as-of the last sync.
    if let Some(when) = crate::gitx::git()
        .arg("-C")
        .arg(&dir)
        .args(["log", "-1", "--format=%ci"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    {
        lines.push(format!("  cache as of {when} — `keel library sync` for a fresher answer"));
    }
    Section { label: "library", state: if attention { State::Attention } else { State::Ok }, lines }
}

/// MODEL — is the recorded state honest right now.
fn model_section(root: &Path) -> Section {
    let reports = crate::guards::run_all(root);
    let violations: usize = reports.iter().map(|r| r.violations.len()).sum();
    let warnings: usize = reports.iter().map(|r| r.warnings.len()).sum();
    let files = crate::collect_sysml(&root.join(".tracking")).len();
    let mut lines = vec![format!("{files} tracked file(s), {} guards", reports.len())];
    let state = if violations > 0 {
        lines.push(format!("  {violations} VIOLATION(s) — `keel guard` for detail"));
        State::Attention
    } else {
        lines.push(format!("  0 violations, {warnings} warning(s)"));
        State::Ok
    };
    Section { label: "model", state, lines }
}

/// WORK — what is ready and what is open.
fn work_section(root: &Path) -> Section {
    let out = crate::orient::compute(root);
    let ready = out.ready.len();
    let issues = out.open_issues.len();
    let mut lines = vec![format!("{ready} ready, {issues} open issue(s)")];
    if let Some(next) = out.ready.first() {
        lines.push(format!("  next: {next}"));
    }
    Section { label: "work", state: State::Ok, lines }
}

/// CI — the verdict for the commit at HEAD (the async surfacing the human chose: land records the
/// push, and the next command reports what became of it).
fn ci_section(root: &Path) -> Section {
    let head = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let Some(head) = head else {
        return Section { label: "ci", state: State::Unknown, lines: vec!["not a git repository".into()] };
    };
    let out = std::process::Command::new("gh")
        .args(["run", "list", "--commit", &head, "--limit", "5", "--json", "conclusion,status,workflowName"])
        .current_dir(root)
        .output();
    let Ok(o) = out else {
        return Section {
            label: "ci",
            state: State::Unknown,
            lines: vec!["`gh` unavailable — cannot tell whether this commit passed".into()],
        };
    };
    if !o.status.success() {
        return Section {
            label: "ci",
            state: State::Unknown,
            lines: vec!["gh could not reach the runs — cannot tell whether this commit passed".into()],
        };
    }
    let text = String::from_utf8_lossy(&o.stdout);
    if text.trim() == "[]" || text.trim().is_empty() {
        return Section {
            label: "ci",
            state: State::Unknown,
            lines: vec![format!("no run found for {} — pushed yet?", &head[..7.min(head.len())])],
        };
    }
    let failed = text.contains("\"conclusion\":\"failure\"") || text.contains("\"conclusion\":\"timed_out\"");
    let running = text.contains("\"status\":\"in_progress\"") || text.contains("\"status\":\"queued\"");
    if failed {
        Section {
            label: "ci",
            state: State::Attention,
            lines: vec![format!("{} FAILED — `gh run list` for the run", &head[..7.min(head.len())])],
        }
    } else if running {
        Section {
            label: "ci",
            state: State::Unknown,
            lines: vec![format!("{} still running", &head[..7.min(head.len())])],
        }
    } else {
        Section { label: "ci", state: State::Ok, lines: vec![format!("{} passed", &head[..7.min(head.len())])] }
    }
}

/// `keel status [ROOT]` — every base, in one screen.
#[must_use]
pub fn cmd(root: &Path) -> i32 {
    let sections = [
        engine_section(root),
        library_section(root),
        model_section(root),
        work_section(root),
        hooks_section(root),
        ci_section(root),
    ];
    println!("keel status — {}", root.display());
    println!();
    for s in &sections {
        println!("  {:<9} {} {}", s.label, s.state.painted(), s.lines.first().map_or("", String::as_str));
        for extra in s.lines.iter().skip(1) {
            println!("  {:<20}{extra}", "");
        }
    }
    println!();
    let attention = sections.iter().filter(|s| s.state == State::Attention).count();
    let unknown = sections.iter().filter(|s| s.state == State::Unknown).count();
    // UNKNOWN IS REPORTED SEPARATELY FROM CLEAN, always. Folding "I could not tell" into "nothing to
    // report" is how a status screen becomes a comfort blanket.
    println!("  {attention} section(s) need attention, {unknown} could not be determined.");
    i32::from(attention > 0)
}

#[cfg(test)]
mod hooks_section_tests {
    use super::{hooks_section, State};

    fn fixture(tag: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("keel-status-hooks-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".claude")).expect("mkdir");
        root
    }

    /// D0296: the kill switch is ATTENTION even when every host is present - a silenced hook cannot
    /// report itself, so this line is the one place a human sees it.
    #[test]
    fn kill_switch_is_attention_and_named() {
        let root = fixture("kill");
        let clean = crate::claude_surface::merge_settings(&serde_json::json!({}));
        std::fs::write(root.join(".claude").join("settings.json"), clean.to_string()).expect("clean");
        std::fs::write(root.join(".claude").join("settings.local.json"), r#"{"disableAllHooks": true}"#).expect("local");
        let s = hooks_section(&root);
        assert!(s.state == State::Attention);
        assert!(s.lines[0].contains("settings.local.json") && s.lines[0].contains("disableAllHooks"), "{:?}", s.lines);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// No host at all is ATTENTION, not a clean zero (the pass-at-zero rule this module is built on).
    #[test]
    fn no_host_is_attention_and_a_clean_declaration_is_ok() {
        let root = fixture("none");
        let s = hooks_section(&root);
        assert!(s.state == State::Attention && s.lines[0].starts_with("NO hook host"), "{:?}", s.lines);
        let clean = crate::claude_surface::merge_settings(&serde_json::json!({}));
        std::fs::write(root.join(".claude").join("settings.json"), clean.to_string()).expect("clean");
        let s = hooks_section(&root);
        assert!(s.state == State::Ok, "{:?}", s.lines);
        assert!(s.lines[0].starts_with("1 host(s)"), "{:?}", s.lines);
        assert!(s.lines.iter().any(|l| l.starts_with("plugin rendering: none")), "{:?}", s.lines);
        let _ = std::fs::remove_dir_all(&root);
    }
}
