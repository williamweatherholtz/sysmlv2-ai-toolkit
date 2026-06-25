//! `keel reverify` (D0101) — auto-re-verify drift-suspect REPRODUCIBLE `method=test` verifications by
//! actually re-running the configured gate at HEAD and appending a fresh `TestResult` on green.
//!
//! Honest by construction: a fresh result is stamped ONLY after the real command passed at HEAD — it
//! never fabricates a pass (the honest-state invariant, D0098). Judgment-method verifications
//! (confirmation/inspect/analyze/demo) are out of scope; only reproducible deliverable-drift tasks are
//! refreshed. The reverify gate is declared in `.engine/contracts/reverify.toml` (downstream-overridable).

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(serde::Deserialize, Default)]
struct ReverifyConfig {
    #[serde(default)]
    commands: Vec<String>,
}

/// Parse the reverify command list from `reverify.toml` text.
///
/// # Errors
/// Returns the TOML error string on malformed input.
pub fn parse_commands(toml_str: &str) -> Result<Vec<String>, String> {
    toml::from_str::<ReverifyConfig>(toml_str).map(|c| c.commands).map_err(|e| e.to_string())
}

/// The deliverable-manifest task names (`task: NAME | paths`) — the reproducible-reverify-eligible set.
#[must_use]
pub fn manifest_task_names(manifest_text: &str) -> HashSet<String> {
    manifest_text
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.starts_with('#') {
                return None;
            }
            let rest = l.strip_prefix("task:")?;
            let name = rest.split('|').next()?.trim();
            (!name.is_empty()).then(|| name.to_string())
        })
        .collect()
}

/// The drift-suspect deliverable tasks: the orient suspect set ∩ the manifest task names (source-drift,
/// reproducibly re-verifiable — excludes transitive/criterion suspects, which need judgment).
fn drift_tasks(root: &Path) -> Vec<String> {
    let manifest = std::fs::read_to_string(root.join(".engine").join("deliverable-manifest.txt")).unwrap_or_default();
    let names = manifest_task_names(&manifest);
    let mut out: Vec<String> = crate::orient::compute(root).suspect.into_iter().filter(|t| names.contains(t)).collect();
    out.sort();
    out
}

/// The `.tracking` file declaring `action <task>;` (where the task's `DoD` + results live).
fn find_task_file(root: &Path, task: &str) -> Option<PathBuf> {
    let needle = format!("action {task};");
    crate::collect_sysml(&root.join(".tracking")).into_iter().find(|f| std::fs::read_to_string(f).is_ok_and(|t| t.contains(&needle)))
}

fn git_capture(root: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Run one reverify command as a shell command in `root`; returns true on exit 0.
///
/// Builds into an ISOLATED `target/reverify` dir (`CARGO_TARGET_DIR`) so a `cargo` gate never tries to
/// overwrite the running `keel` binary (self-replacement lock — "Access is denied" on Windows). Harmless
/// for non-cargo commands.
fn run_shell(root: &Path, cmd: &str) -> bool {
    let mut c = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(cmd);
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c").arg(cmd);
        c
    };
    c.current_dir(root).env("CARGO_TARGET_DIR", root.join("target").join("reverify")).status().is_ok_and(|s| s.success())
}

/// Run `keel reverify`: re-run the configured gate at HEAD and stamp a fresh result on green.
///
/// On all commands exiting 0, appends a fresh `TestResult` to each drift-suspect task. Returns a process
/// exit code (0 = ok/no-op, 1 = gate failed, 2 = config error).
#[must_use]
pub fn run(root: &Path, task_filter: Option<&str>, by: &str) -> i32 {
    let drift: Vec<String> = drift_tasks(root).into_iter().filter(|t| task_filter.is_none_or(|f| f == t)).collect();
    if drift.is_empty() {
        println!("reverify: no drift-suspect deliverable task(s) to re-verify");
        return 0;
    }
    let cfg_path = root.join(".engine").join("contracts").join("reverify.toml");
    let Ok(cfg_text) = std::fs::read_to_string(&cfg_path) else {
        println!("reverify: no reverify command configured (.engine/contracts/reverify.toml absent) — nothing re-run; {} task(s) stay suspect", drift.len());
        return 0;
    };
    let commands = match parse_commands(&cfg_text) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("reverify: reverify.toml parse error: {e}");
            return 2;
        }
    };
    if commands.is_empty() {
        println!("reverify: reverify.toml declares no commands — nothing re-run");
        return 0;
    }
    println!("reverify: re-verifying {} drift task(s) via {} command(s)…", drift.len(), commands.len());
    for cmd in &commands {
        println!("reverify: running `{cmd}`");
        if !run_shell(root, cmd) {
            eprintln!("reverify: `{cmd}` FAILED at HEAD — no fresh result stamped; {} task(s) stay suspect (honest)", drift.len());
            return 1;
        }
    }
    let head = git_capture(root, &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let date = git_capture(root, &["show", "-s", "--format=%cs", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let mut stamped = 0;
    for task in &drift {
        match find_task_file(root, task) {
            Some(file) => match crate::write::append_result(&file, task, &head, "pass", &date, by) {
                Ok(id) => {
                    stamped += 1;
                    println!("reverify: {task} re-verified pass @ {head} ({id})");
                }
                Err(e) => eprintln!("reverify: could not stamp {task}: {e}"),
            },
            None => eprintln!("reverify: could not locate the .tracking file declaring `action {task};`"),
        }
    }
    println!("reverify: gate green — stamped {stamped}/{} drift task(s) fresh at HEAD {head} ({date})", drift.len());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_commands_reads_the_list() {
        let toml = "commands = [\"cargo test --workspace\", \"cargo clippy -- -D warnings\"]\n";
        assert_eq!(parse_commands(toml).unwrap(), vec!["cargo test --workspace", "cargo clippy -- -D warnings"]);
    }

    #[test]
    fn parse_commands_absent_key_is_empty() {
        // A reverify.toml with no `commands` key => no-op (empty), not an error.
        assert!(parse_commands("# just a comment\n").unwrap().is_empty());
    }

    #[test]
    fn manifest_task_names_extracts_tasks_skipping_comments() {
        let m = "# header comment\ntask: rustS1Lexer | a.rs b.rs\ntask: rustS9writeApi | c.rs\n# task: ignoredComment | x\n";
        let got = manifest_task_names(m);
        assert_eq!(got.len(), 2);
        assert!(got.contains("rustS1Lexer"));
        assert!(got.contains("rustS9writeApi"));
        assert!(!got.contains("ignoredComment"));
    }
}
