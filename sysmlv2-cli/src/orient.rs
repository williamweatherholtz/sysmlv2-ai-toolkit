//! Orient subcommand: state cursor + ready/suspect/invalidEvidence/done frontier.
//!
//! Pure-Rust equivalent of `query.py orient` — no Jupyter kernel required.
//! Uses [`crate::indexer::extract`] to parse `.tracking/**/*.sysml` via the AST
//! parser and then applies git-based suspect/invalid-evidence classification.

use std::{
    collections::{HashMap, HashSet},
    path::Path,
    process::Command,
};

use crate::indexer::{ExtractedIndex, TaskData};

// ── public types ──────────────────────────────────────────────────────────────

/// Output of the `orient` subcommand.
#[derive(Debug)]
pub struct Output {
    /// Parsed state cursor (defaults to empty strings if state.sysml is absent).
    pub cursor: crate::Cursor,
    /// Tasks that are outstanding and whose every dependency is done.
    pub ready: Vec<String>,
    /// Done tasks whose `DoD` criterion text changed since they were verified.
    pub suspect: Vec<String>,
    /// Done tasks whose `judgedAgainst` SHA cannot be resolved in git.
    pub invalid_evidence: Vec<String>,
    /// Number of done tasks.
    pub done: usize,
    /// Number of outstanding (not-done) tasks.
    pub outstanding: usize,
}

impl Output {
    /// Render as a JSON string matching the `query.py orient` output format.
    #[must_use]
    pub fn to_json(&self) -> String {
        let c = &self.cursor;
        let cursor_block = format!(
            "{{\n    \"activeWorkflow\": \"{}\",\n    \"activePhase\": \"{}\",\n    \"enteredAt\": \"{}\",\n    \"enteredBy\": \"{}\"\n  }}",
            json_esc(&c.active_workflow),
            json_esc(&c.active_phase),
            json_esc(&c.entered_at),
            json_esc(&c.entered_by),
        );
        format!(
            "{{\n  \"cursor\": {},\n  \"ready\": {},\n  \"suspect\": {},\n  \"invalidEvidence\": {},\n  \"counts\": {{\"done\": {}, \"outstanding\": {}}}\n}}",
            cursor_block,
            str_array(&self.ready),
            str_array(&self.suspect),
            str_array(&self.invalid_evidence),
            self.done,
            self.outstanding,
        )
    }
}

fn json_esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn str_array(items: &[String]) -> String {
    if items.is_empty() {
        return "[]".to_owned();
    }
    let inner: Vec<String> = items.iter().map(|s| format!("\"{}\"", json_esc(s))).collect();
    format!("[{}]", inner.join(", "))
}

// ── top-level entry point ─────────────────────────────────────────────────────

/// Compute the orientation view for a repository root.
///
/// Parses `.tracking/**/*.sysml` via the AST parser (see [`crate::indexer::extract`])
/// and applies git-based suspect/invalid-evidence classification.
#[must_use]
pub fn compute(root: &Path) -> Output {
    let tracking = root.join(".tracking");
    let idx = crate::indexer::extract(&tracking);
    compute_orient(root, idx)
}

// ── git helpers ───────────────────────────────────────────────────────────────

fn git_sha_valid(sha: &str, repo: &Path) -> bool {
    if sha.is_empty() {
        return false;
    }
    Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["cat-file", "-t", sha])
        .output()
        .map(|o| o.status.success() && o.stdout.starts_with(b"commit"))
        .unwrap_or(true) // conservative: if git is unavailable, don't invalidate
}

/// Read criterion text for `task` at git commit `sha`.
fn git_criterion_at(sha: &str, task: &str, repo: &Path) -> Option<String> {
    let ls = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["ls-tree", "-r", "--name-only", sha, ".tracking"])
        .output()
        .ok()?;
    if !ls.status.success() {
        return None;
    }
    let file_list = String::from_utf8(ls.stdout).ok()?;
    let dod_pfx = format!("verification {task}DoD");

    for rel in file_list.lines() {
        if !std::path::Path::new(rel)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("sysml"))
        {
            continue;
        }
        let show = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["show", &format!("{sha}:{rel}")])
            .output()
            .ok()?;
        if !show.status.success() {
            continue;
        }
        let content = String::from_utf8(show.stdout).ok()?;
        for line in content.lines() {
            if line.trim().starts_with(&dod_pfx) {
                // Extract procedureText from the line.
                let pat = "procedureText = \"";
                let start = line.find(pat)? + pat.len();
                let rest = &line[start..];
                let end = rest.find('"')?;
                return Some(rest[..end].to_owned());
            }
        }
    }
    None
}

// ── classification ────────────────────────────────────────────────────────────

fn compute_orient(repo: &Path, idx: ExtractedIndex) -> Output {
    let ExtractedIndex { tasks, ordering_only, cursor } = idx;

    // Step 1: compute done/invalid-evidence/verified-at.
    let mut done_map: HashMap<String, bool> = HashMap::new();
    let mut verified_at: HashMap<String, String> = HashMap::new();
    let mut invalid_evidence: Vec<String> = Vec::new();

    for (name, data) in &tasks {
        if let Some(latest) = data.results.last() {
            if latest.outcome == "pass" {
                let sha = &latest.judged_against;
                if !sha.is_empty() && !git_sha_valid(sha, repo) {
                    done_map.insert(name.clone(), false);
                    invalid_evidence.push(name.clone());
                } else {
                    done_map.insert(name.clone(), true);
                    verified_at.insert(name.clone(), sha.clone());
                }
            } else {
                done_map.insert(name.clone(), false);
            }
        } else {
            done_map.insert(name.clone(), false);
        }
    }

    // Step 2: compute ready.
    let mut ready: Vec<String> = Vec::new();
    for (name, data) in &tasks {
        let is_done = done_map.get(name.as_str()).copied().unwrap_or(false);
        let is_invalid = invalid_evidence.contains(name);
        if !is_done && !is_invalid {
            let all_deps_done = all_deps_satisfied(name, data, &done_map);
            if all_deps_done {
                ready.push(name.clone());
            }
        }
    }

    // Step 3: compute suspect (criterion text changed since verified).
    let mut suspect_set: HashSet<String> = HashSet::new();
    for (name, data) in &tasks {
        let is_done = done_map.get(name.as_str()).copied().unwrap_or(false);
        if !is_done {
            continue;
        }
        let Some(ct) = verified_at.get(name.as_str()) else { continue };
        'dep_loop: for dep in &data.deps {
            if ordering_only.contains(&(dep.clone(), name.clone())) {
                continue;
            }
            let Some(dep_data) = tasks.get(dep.as_str()) else { continue };
            let cur = dep_data.dod_text.as_deref().unwrap_or("");
            if let Some(old) = git_criterion_at(ct, dep, repo) {
                if old != cur {
                    suspect_set.insert(name.clone());
                    break 'dep_loop;
                }
            }
        }
    }

    // Step 4: transitive suspect propagation.
    let mut changed = true;
    while changed {
        changed = false;
        for (name, data) in &tasks {
            let is_done = done_map.get(name.as_str()).copied().unwrap_or(false);
            if !is_done || suspect_set.contains(name.as_str()) {
                continue;
            }
            for dep in &data.deps {
                if ordering_only.contains(&(dep.clone(), name.clone())) {
                    continue;
                }
                if suspect_set.contains(dep.as_str()) {
                    suspect_set.insert(name.clone());
                    changed = true;
                    break;
                }
            }
        }
    }

    let done = done_map.values().filter(|&&v| v).count();
    let outstanding = done_map.values().filter(|&&v| !v).count();

    let mut ready_sorted = ready;
    ready_sorted.sort();
    let mut suspect: Vec<String> = suspect_set.into_iter().collect();
    suspect.sort();
    invalid_evidence.sort();

    Output { cursor, ready: ready_sorted, suspect, invalid_evidence, done, outstanding }
}

fn all_deps_satisfied(
    _name: &str,
    data: &TaskData,
    done_map: &HashMap<String, bool>,
) -> bool {
    data.deps
        .iter()
        .all(|dep| done_map.get(dep.as_str()).copied().unwrap_or(false))
}
