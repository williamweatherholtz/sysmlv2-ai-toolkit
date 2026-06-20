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

/// Ceremony status of one in-progress sprint (D0045: replaces the retired cursor).
#[derive(Debug)]
pub struct SprintCeremony {
    /// Delivery file stem, e.g. `sprint17_rustToolchainFix`.
    pub sprint: String,
    /// Gate names with a passing `TestResult`, in canonical order.
    pub passed: Vec<String>,
    /// First canonical gate not yet passed (`None` only if all are passed).
    pub pending: Option<String>,
}

/// Output of the `orient` subcommand.
#[derive(Debug)]
pub struct Output {
    /// In-progress sprints with per-gate ceremony status (computed from delivery files).
    pub in_progress_sprints: Vec<SprintCeremony>,
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
        let sprints: Vec<String> = self.in_progress_sprints.iter().map(|s| {
            let pending = s.pending.as_ref()
                .map_or_else(|| "null".to_owned(), |p| format!("\"{}\"", json_esc(p)));
            format!(
                "{{\"sprint\": \"{}\", \"passed\": {}, \"pending\": {}}}",
                json_esc(&s.sprint),
                str_array(&s.passed),
                pending,
            )
        }).collect();
        let in_progress_block = if sprints.is_empty() {
            "[]".to_owned()
        } else {
            format!("[{}]", sprints.join(", "))
        };
        format!(
            "{{\n  \"in_progress_sprints\": {},\n  \"ready\": {},\n  \"suspect\": {},\n  \"invalidEvidence\": {},\n  \"counts\": {{\"done\": {}, \"outstanding\": {}}}\n}}",
            in_progress_block,
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

pub(crate) fn git_sha_valid(sha: &str, repo: &Path) -> bool {
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

/// Canonical ceremony gate order (mirrors `query.py` `read_sprint_ceremony_status` + D0047).
const GATE_ORDER: [&str; 6] = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];

/// Compute in-progress sprint ceremony status from `.tracking/delivery/*.sysml`
/// (D0045: replaces the `StateCursor`). A sprint is in-progress if at least one gate
/// passed and Retro has not; `pending` is the first canonical gate not yet passed.
fn in_progress_sprints(repo: &Path) -> Vec<SprintCeremony> {
    let delivery = repo.join(".tracking").join("delivery");
    let mut out = Vec::new();
    for path in crate::collect_sysml(&delivery) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let passed: Vec<String> = GATE_ORDER.iter()
            .filter(|g| gate_passed(&text, g))
            .map(|g| (*g).to_owned())
            .collect();
        if passed.is_empty() || passed.iter().any(|g| g == "Retro") {
            continue; // not started, or ceremony complete
        }
        let pending = GATE_ORDER.iter()
            .find(|g| !passed.iter().any(|p| p == *g))
            .map(|g| (*g).to_owned());
        let sprint = path.file_stem().map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        out.push(SprintCeremony { sprint, passed, pending });
    }
    out.sort_by(|a, b| a.sprint.cmp(&b.sprint));
    out
}

/// True if a `part <...><Gate>Gate<...>R<n> : TestResult` with `outcome = pass`
/// exists for the given canonical gate name in `text`.
pub(crate) fn gate_passed(text: &str, gate: &str) -> bool {
    let needle = format!("{gate}Gate");
    for (idx, _) in text.match_indices(&needle) {
        // Must be a `part ...R<n> : TestResult ... outcome = ...::pass` declaration.
        let line_start = text[..idx].rfind('\n').map_or(0, |n| n + 1);
        let stmt_end = text[idx..].find('}').map_or(text.len(), |e| idx + e);
        let stmt = &text[line_start..stmt_end];
        if stmt.contains("part ") && stmt.contains(": TestResult")
            && stmt.contains("VerdictKind::pass")
        {
            return true;
        }
    }
    false
}

/// Load the deliverable-suspicion manifest (D0050): `(source_paths, task_names)`.
/// Missing manifest → empty (feature inert). Lines: `path: <p>` / `task: <t>`; `#` = comment.
fn load_deliverable_manifest(repo: &Path) -> (Vec<String>, HashSet<String>) {
    let mut paths = Vec::new();
    let mut tasks = HashSet::new();
    let Ok(text) = std::fs::read_to_string(repo.join(".engine").join("deliverable-manifest.txt"))
    else {
        return (paths, tasks);
    };
    for line in text.lines() {
        let l = line.trim();
        if let Some(p) = l.strip_prefix("path:") {
            paths.push(p.trim().to_owned());
        } else if let Some(t) = l.strip_prefix("task:") {
            tasks.insert(t.trim().to_owned());
        }
    }
    (paths, tasks)
}

/// True if any commit touching `paths` exists strictly after `sha` (deliverable drifted
/// since it was verified). Conservative: on git failure, returns false (don't flag).
fn deliverable_drifted(repo: &Path, sha: &str, paths: &[String]) -> bool {
    if sha.is_empty() || paths.is_empty() {
        return false;
    }
    let mut args: Vec<String> = vec![
        "-C".into(), repo.display().to_string(),
        "log".into(), "--oneline".into(), format!("{sha}..HEAD"), "--".into(),
    ];
    args.extend(paths.iter().cloned());
    Command::new("git")
        .args(&args)
        .output()
        .map(|o| o.status.success() && !o.stdout.is_empty())
        .unwrap_or(false)
}

/// Mark manifest deliverable tasks suspect when the source drifted since they were
/// verified (D0050). Mutates `suspect_set` in place.
fn apply_deliverable_suspicion(
    repo: &Path,
    done_map: &HashMap<String, bool>,
    verified_at: &HashMap<String, String>,
    suspect_set: &mut HashSet<String>,
) {
    let (dpaths, dtasks) = load_deliverable_manifest(repo);
    if dpaths.is_empty() {
        return;
    }
    for name in &dtasks {
        if done_map.get(name.as_str()).copied().unwrap_or(false) {
            if let Some(ct) = verified_at.get(name.as_str()) {
                if deliverable_drifted(repo, ct, &dpaths) {
                    suspect_set.insert(name.clone());
                }
            }
        }
    }
}

fn compute_orient(repo: &Path, idx: ExtractedIndex) -> Output {
    let ExtractedIndex { tasks, ordering_only, .. } = idx;

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

    // Step 5: deliverable suspicion (D0050) — manifest tasks whose source drifted since verified.
    apply_deliverable_suspicion(repo, &done_map, &verified_at, &mut suspect_set);

    let done = done_map.values().filter(|&&v| v).count();
    let outstanding = done_map.values().filter(|&&v| !v).count();

    // Rank ready by backlog declaration order (D0052) — priority, not alphabetical.
    let mut ready_sorted = ready;
    ready_sorted.sort_by_key(|name| tasks.get(name).map_or(u32::MAX, |t| t.order));
    let mut suspect: Vec<String> = suspect_set.into_iter().collect();
    suspect.sort();
    invalid_evidence.sort();

    Output {
        in_progress_sprints: in_progress_sprints(repo),
        ready: ready_sorted,
        suspect,
        invalid_evidence,
        done,
        outstanding,
    }
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
