//! Orient subcommand: state cursor + ready/suspect/done frontier.
//!
//! Pure-Rust equivalent of `query.py orient` — no Jupyter kernel required.
//! Reads `.tracking/state.sysml` for the cursor and scans all `.tracking/**/*.sysml`
//! files line-by-line for tasks, succession edges, `DoDs`, and `TestResults`.

use std::{
    collections::{HashMap, HashSet},
    path::Path,
    process::Command,
};

use crate::collect_sysml;

// ── public types ──────────────────────────────────────────────────────────────

/// The project state cursor from `.tracking/state.sysml`.
#[derive(Debug, Default, Clone)]
pub struct Cursor {
    /// Active workflow name (e.g. `"DeliveryWorkflow"`).
    pub active_workflow: String,
    /// Active phase within the workflow (e.g. `"Delivery/implement"`).
    pub active_phase: String,
    /// ISO-8601 date the cursor was set.
    pub entered_at: String,
    /// Actor who set the cursor.
    pub entered_by: String,
}

/// Output of the `orient` subcommand.
#[derive(Debug)]
pub struct Output {
    /// Parsed state cursor (defaults to empty strings if state.sysml is absent).
    pub cursor: Cursor,
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
/// Reads `.tracking/state.sysml` for the cursor and scans all
/// `.tracking/**/*.sysml` files for tasks, `DoDs`, `TestResults`, and succession
/// edges.  Uses `git` (via [`std::process::Command`]) for evidence validation
/// and suspect detection; git failures are treated conservatively (no false
/// positives).
#[must_use]
pub fn compute(root: &Path) -> Output {
    let tracking = root.join(".tracking");
    let cursor = read_cursor(&tracking);
    let (tasks, ordering_only) = read_backlog(&tracking);
    compute_orient(root, cursor, &tasks, &ordering_only)
}

// ── internal data model ───────────────────────────────────────────────────────

#[derive(Default)]
struct TaskData {
    deps: Vec<String>,
    dod_text: Option<String>,
    /// Results sorted ascending by sequence number after `read_backlog`.
    results: Vec<ResultEntry>,
}

struct ResultEntry {
    n: u32,
    outcome: String,
    judged_against: String,
}

// ── text-scan helpers ─────────────────────────────────────────────────────────

/// Extract `:>> attr = "value"` → `value`.
fn str_attr(line: &str, attr: &str) -> Option<String> {
    let pat = format!("{attr} = \"");
    let start = line.find(&pat)? + pat.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

/// Extract `attr = Ns::variant` → `variant` (or `attr = variant` → `variant`).
fn enum_attr(line: &str, attr: &str) -> Option<String> {
    let pat = format!("{attr} = ");
    let start = line.find(&pat)? + pat.len();
    let rest = &line[start..];
    let after = rest.find("::").map_or(rest, |p| &rest[p + 2..]);
    let end = after.find([';', ' ', '}', '\n']).unwrap_or(after.len());
    if end == 0 {
        return None;
    }
    Some(after[..end].to_owned())
}

/// Parse `part <name>DoDR<n> : TestResult` or `part <name>R<n> : TestResult` → `(name, n)`.
fn parse_result_header(line: &str) -> Option<(String, u32)> {
    let rest = line.trim().strip_prefix("part ")?;
    let close = rest.find(" : TestResult")?;
    let name_n = &rest[..close];
    // Try DoDR<n> suffix first (DoD-linked TestResult).
    if let Some(dod_pos) = name_n.rfind("DoDR") {
        let digits = &name_n[dod_pos + 4..];
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            if let Ok(n) = digits.parse::<u32>() {
                return Some((name_n[..dod_pos].to_owned(), n));
            }
        }
    }
    // Fallback: walk backwards to find the last R<digits> suffix.
    for i in (0..name_n.len()).rev() {
        if name_n.as_bytes().get(i) == Some(&b'R') {
            let digits = &name_n[i + 1..];
            if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
                let n: u32 = digits.parse().ok()?;
                return Some((name_n[..i].to_owned(), n));
            }
        }
    }
    None
}

/// Parse `verification <name>DoD : Test` → `name`.
fn parse_dod_header(line: &str) -> Option<String> {
    let rest = line.trim().strip_prefix("verification ")?;
    // Only search for "DoD" in the name portion (before " : Test"), so we don't
    // match "DoD" appearing in procedureText strings on the same line.
    let test_sep = rest.find(" : Test")?;
    let name_part = &rest[..test_sep];
    let pos = name_part.rfind("DoD")?;
    if pos + 3 != name_part.len() {
        return None;
    }
    Some(name_part[..pos].to_owned())
}

/// Parse `first X then Y;` → `(X, Y)` (non-ordering-only).
fn parse_succession(line: &str) -> Option<(String, String)> {
    let t = line.trim();
    if t.starts_with('#') {
        return None; // ordering-only handled separately
    }
    let rest = t.strip_prefix("first ")?;
    let then = rest.find(" then ")?;
    let pred = rest[..then].trim();
    let succ = rest[then + 6..].trim_end_matches(';').trim();
    if pred.is_empty() || succ.is_empty() || pred.contains(' ') || succ.contains(' ') {
        return None;
    }
    Some((pred.to_owned(), succ.to_owned()))
}

/// Parse `#OrderingOnly first X then Y;` → `(X, Y)`.
fn parse_ordering_only(line: &str) -> Option<(String, String)> {
    let rest = line.trim().strip_prefix("#OrderingOnly ")?;
    let rest2 = rest.strip_prefix("first ")?;
    let then = rest2.find(" then ")?;
    let pred = rest2[..then].trim();
    let succ = rest2[then + 6..].trim_end_matches(';').trim();
    if pred.is_empty() || succ.is_empty() {
        return None;
    }
    Some((pred.to_owned(), succ.to_owned()))
}

// ── cursor reader ─────────────────────────────────────────────────────────────

fn read_cursor(tracking: &Path) -> Cursor {
    let path = tracking.join("state.sysml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Cursor::default();
    };
    let mut c = Cursor::default();
    for line in text.lines() {
        if let Some(v) = str_attr(line, "activeWorkflow") {
            c.active_workflow = v;
        }
        if let Some(v) = str_attr(line, "activePhase") {
            c.active_phase = v;
        }
        if let Some(v) = str_attr(line, "enteredAt") {
            c.entered_at = v;
        }
        if let Some(v) = str_attr(line, "enteredBy") {
            c.entered_by = v;
        }
    }
    c
}

// ── backlog reader ────────────────────────────────────────────────────────────

/// Scan all `.tracking/**/*.sysml` files for tasks, edges, `DoDs`, and results.
fn read_backlog(tracking: &Path) -> (HashMap<String, TaskData>, HashSet<(String, String)>) {
    let mut tasks: HashMap<String, TaskData> = HashMap::new();
    let mut all_edges: Vec<(String, String)> = Vec::new();
    let mut ordering_only: HashSet<(String, String)> = HashSet::new();
    let mut raw_results: HashMap<String, Vec<ResultEntry>> = HashMap::new();

    for path in collect_sysml(tracking) {
        scan_file(&path, &mut tasks, &mut all_edges, &mut ordering_only, &mut raw_results);
    }

    // Attach sorted results to known tasks only — don't create phantom tasks from result names.
    for (name, mut rs) in raw_results {
        rs.sort_by_key(|r| r.n);
        if let Some(task) = tasks.get_mut(&name) {
            task.results = rs;
        }
    }

    // Build deps from succession edges.
    for (pred, succ) in &all_edges {
        if let Some(task) = tasks.get_mut(succ.as_str()) {
            if !task.deps.contains(pred) {
                task.deps.push(pred.clone());
            }
        }
    }

    (tasks, ordering_only)
}

fn scan_file(
    path: &Path,
    tasks: &mut HashMap<String, TaskData>,
    edges: &mut Vec<(String, String)>,
    ordering_only: &mut HashSet<(String, String)>,
    results: &mut HashMap<String, Vec<ResultEntry>>,
) {
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    for line in text.lines() {
        let t = line.trim();

        // Action item: `action name;` (not `action def`, not with type annotation).
        if let Some(rest) = t.strip_prefix("action ") {
            if !rest.starts_with("def ") && !rest.contains(':') {
                let name = rest.trim_end_matches(';').trim();
                if !name.is_empty() && !name.contains(' ') {
                    tasks.entry(name.to_owned()).or_default();
                }
            }
        }

        // Succession edges.
        if let Some((p, s)) = parse_succession(t) {
            edges.push((p, s));
        }
        if let Some((p, s)) = parse_ordering_only(t) {
            ordering_only.insert((p, s));
        }

        // DoD line.
        if let Some(name) = parse_dod_header(t) {
            let text_val = str_attr(line, "procedureText").unwrap_or_default();
            tasks.entry(name).or_default().dod_text = Some(text_val);
        }

        // TestResult line.
        if let Some((name, n)) = parse_result_header(t) {
            let outcome = enum_attr(line, "outcome").unwrap_or_default();
            let sha = str_attr(line, "judgedAgainst").unwrap_or_default();
            results
                .entry(name)
                .or_default()
                .push(ResultEntry { n, outcome, judged_against: sha });
        }
    }
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
/// Returns `None` if git is unavailable or the task wasn't in the repo at that commit.
fn git_criterion_at(sha: &str, task: &str, repo: &Path) -> Option<String> {
    // List .tracking/**/*.sysml at the given commit.
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
                return str_attr(line, "procedureText");
            }
        }
    }
    None
}

// ── classification ────────────────────────────────────────────────────────────

fn compute_orient(
    repo: &Path,
    cursor: Cursor,
    tasks: &HashMap<String, TaskData>,
    ordering_only: &HashSet<(String, String)>,
) -> Output {
    // Step 1: compute done/invalid-evidence/verified-at.
    let mut done_map: HashMap<String, bool> = HashMap::new();
    let mut verified_at: HashMap<String, String> = HashMap::new();
    let mut invalid_evidence: Vec<String> = Vec::new();

    for (name, data) in tasks {
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
    for (name, data) in tasks {
        let is_done = done_map.get(name.as_str()).copied().unwrap_or(false);
        let is_invalid = invalid_evidence.contains(name);
        if !is_done && !is_invalid {
            let all_deps_done = data.deps.iter().all(|dep| {
                done_map.get(dep.as_str()).copied().unwrap_or(false)
            });
            if all_deps_done {
                ready.push(name.clone());
            }
        }
    }

    // Step 3: compute suspect (criterion text changed since verified).
    let mut suspect_set: HashSet<String> = HashSet::new();
    for (name, data) in tasks {
        let is_done = done_map.get(name.as_str()).copied().unwrap_or(false);
        if !is_done {
            continue;
        }
        let Some(ct) = verified_at.get(name.as_str()) else {
            continue;
        };
        'dep_loop: for dep in &data.deps {
            if ordering_only.contains(&(dep.clone(), name.clone())) {
                continue;
            }
            let Some(dep_data) = tasks.get(dep.as_str()) else {
                continue;
            };
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
        for (name, data) in tasks {
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
