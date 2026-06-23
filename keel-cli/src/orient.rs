//! Orient subcommand: state cursor + ready/suspect/invalidEvidence/done frontier.
//!
//! Pure-Rust equivalent of `query.py orient` — no Jupyter kernel required.
//! Uses [`crate::indexer::extract`] to parse `.tracking/**/*.sysml` via the AST
//! parser and then applies git-based suspect/invalid-evidence classification.

use std::{
    collections::{HashMap, HashSet},
    io::Write,
    path::Path,
    process::{Command, Stdio},
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
    /// OPEN issues (no complete `#Resolves` resolver) — D0077, surfaced so the frontier can't
    /// read "empty" while issues are unresolved.
    pub open_issues: Vec<String>,
    /// Per-suspect-task reason string (criterion-change / transitive / deliverable-drift).
    /// Carried for `suspect --explain`; NOT emitted in the standard orient JSON.
    pub suspect_reasons: HashMap<String, String>,
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
            "{{\n  \"in_progress_sprints\": {},\n  \"ready\": {},\n  \"suspect\": {},\n  \"invalidEvidence\": {},\n  \"open_issues\": {},\n  \"counts\": {{\"done\": {}, \"outstanding\": {}}}\n}}",
            in_progress_block,
            str_array(&self.ready),
            str_array(&self.suspect),
            str_array(&self.invalid_evidence),
            str_array(&self.open_issues),
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

/// Extract a `<task>DoD` verification's `procedureText` from a file's content (no git).
fn extract_dod_criterion(content: &str, task: &str) -> Option<String> {
    let pfx = format!("verification {task}DoD");
    for line in content.lines() {
        if line.trim_start().starts_with(&pfx) {
            let pat = "procedureText = \"";
            let start = line.find(pat)? + pat.len();
            let rest = &line[start..];
            let end = rest.find('"')?;
            return Some(rest[..end].to_owned());
        }
    }
    None
}

/// Map every CURRENT `<task>DoD` verification to its repo-relative file path (one working-tree
/// pass, no git). Lets criterion lookup fetch a single historical blob instead of scanning all
/// files at the commit — the dominant `orient` cost (orientPerf, sr11FastStart).
fn build_dod_files(repo: &Path) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    for path in crate::collect_sysml(&repo.join(".tracking")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Some(rel) = path.strip_prefix(repo).ok().and_then(std::path::Path::to_str).map(|s| s.replace('\\', "/")) else {
            continue;
        };
        for line in text.lines() {
            if let Some(rest) = line.trim_start().strip_prefix("verification ") {
                let name = rest.split([' ', ':']).next().unwrap_or("");
                if let Some(task) = name.strip_suffix("DoD") {
                    out.entry(task.to_owned()).or_insert_with(|| rel.clone());
                }
            }
        }
    }
    out
}

/// Validate many commit SHAs in ONE `git cat-file --batch-check` spawn (orientPerf): returns
/// `sha -> is-commit`. Conservative on git failure (true) — matches `git_sha_valid`.
fn valid_commits(repo: &Path, shas: &[String]) -> HashMap<String, bool> {
    let mut out: HashMap<String, bool> = HashMap::new();
    if shas.is_empty() {
        return out;
    }
    let spawn = Command::new("git")
        .arg("-C").arg(repo)
        .args(["cat-file", "--batch-check"])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null())
        .spawn();
    let Ok(mut child) = spawn else {
        for s in shas { out.insert(s.clone(), true); }
        return out;
    };
    if let Some(mut si) = child.stdin.take() {
        let mut buf = String::new();
        for s in shas {
            buf.push_str(s);
            buf.push('\n');
        }
        let _ = si.write_all(buf.as_bytes());
    }
    let Ok(o) = child.wait_with_output() else {
        for s in shas { out.insert(s.clone(), true); }
        return out;
    };
    let text = String::from_utf8_lossy(&o.stdout);
    // --batch-check emits one line per input, in order: `<oid> <type> <size>` or `<input> missing`.
    for (s, line) in shas.iter().zip(text.lines()) {
        out.insert(s.clone(), line.split_whitespace().nth(1) == Some("commit"));
    }
    out
}

/// Read many `<rev>:<path>` blobs in ONE `git cat-file --batch` spawn (orientPerf): returns
/// `key -> content` (None if missing). Parses the size-delimited batch protocol. `pub(crate)` so
/// the coverage/critique element-content staleness check (D0084) can reuse the batched read.
pub(crate) fn batch_cat_blobs(repo: &Path, keys: &[String]) -> HashMap<String, Option<String>> {
    let mut out: HashMap<String, Option<String>> = HashMap::new();
    if keys.is_empty() {
        return out;
    }
    let spawn = Command::new("git")
        .arg("-C").arg(repo)
        .args(["cat-file", "--batch"])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null())
        .spawn();
    let Ok(mut child) = spawn else { return out; };
    if let Some(mut si) = child.stdin.take() {
        let mut buf = String::new();
        for k in keys {
            buf.push_str(k);
            buf.push('\n');
        }
        let _ = si.write_all(buf.as_bytes());
    }
    let Ok(o) = child.wait_with_output() else { return out; };
    let data = o.stdout;
    let mut pos = 0usize;
    for k in keys {
        let Some(rest) = data.get(pos..) else { break };
        let Some(nl) = rest.iter().position(|&b| b == b'\n') else { break };
        let header = String::from_utf8_lossy(rest.get(..nl).unwrap_or(&[])).into_owned();
        let after_header = pos + nl + 1;
        if header.split_whitespace().nth(1) == Some("missing") || header.ends_with("missing") {
            out.insert(k.clone(), None);
            pos = after_header;
            continue;
        }
        let size: usize = header.split_whitespace().nth(2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let content = data.get(after_header..after_header + size).map(|b| String::from_utf8_lossy(b).into_owned());
        out.insert(k.clone(), content);
        pos = after_header + size + 1; // skip the trailing LF after content
    }
    out
}

/// Compute criterion-change suspects (step 3) with a SINGLE batched blob fetch (orientPerf): a done
/// task is suspect if a non-ordering dep's `DoD` criterion text changed since the task's verified
/// commit. Returns `(task, reason)` pairs. Falls back to a per-call scan only for blobs the batch
/// couldn't resolve (rare — a `DoD` file absent at that commit).
fn criterion_suspects(
    repo: &Path,
    tasks: &HashMap<String, TaskData>,
    ordering_only: &HashSet<(String, String)>,
    done_map: &HashMap<String, bool>,
    verified_at: &HashMap<String, String>,
    dod_files: &HashMap<String, String>,
) -> Vec<(String, String)> {
    let is_done = |n: &str| done_map.get(n).copied().unwrap_or(false);
    // Pass 1: gather the distinct `<ct>:<file>` blob keys every (task, dep) comparison will need.
    let mut keys: HashSet<String> = HashSet::new();
    for (name, data) in tasks {
        if !is_done(name) {
            continue;
        }
        let Some(ct) = verified_at.get(name.as_str()) else { continue };
        for dep in &data.deps {
            if !ordering_only.contains(&(dep.clone(), name.clone())) {
                if let Some(file) = dod_files.get(dep) {
                    keys.insert(format!("{ct}:{file}"));
                }
            }
        }
    }
    let key_vec: Vec<String> = keys.into_iter().collect();
    let blobs = batch_cat_blobs(repo, &key_vec);
    // Pass 2: compare each dep's historical criterion (from the batch) to its current text.
    let mut out: Vec<(String, String)> = Vec::new();
    for (name, data) in tasks {
        if !is_done(name) {
            continue;
        }
        let Some(ct) = verified_at.get(name.as_str()) else { continue };
        for dep in &data.deps {
            if ordering_only.contains(&(dep.clone(), name.clone())) {
                continue;
            }
            let Some(dep_data) = tasks.get(dep.as_str()) else { continue };
            let cur = dep_data.dod_text.as_deref().unwrap_or("");
            let old = dod_files
                .get(dep)
                .and_then(|file| blobs.get(&format!("{ct}:{file}")).cloned().flatten())
                .and_then(|content| extract_dod_criterion(&content, dep))
                .or_else(|| git_criterion_at(ct, dep, repo));
            if let Some(old) = old {
                if old != cur {
                    out.push((name.clone(), format!("criterion of dependency '{dep}' changed since verified at {ct}")));
                    break;
                }
            }
        }
    }
    out
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

/// Load the PER-TASK deliverable-suspicion manifest (D0050; suspectDiagnostics): a list of
/// `(task, its source paths)`. Missing manifest → empty (feature inert).
/// Format: `task: <name> | <space-separated paths>`; `#` = comment.
fn load_deliverable_manifest(repo: &Path) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    let Ok(text) = std::fs::read_to_string(repo.join(".engine").join("deliverable-manifest.txt"))
    else {
        return out;
    };
    for line in text.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("task:") {
            let mut parts = rest.splitn(2, '|');
            let task = parts.next().unwrap_or("").trim().to_owned();
            let paths: Vec<String> = parts.next().unwrap_or("").split_whitespace().map(str::to_owned).collect();
            if !task.is_empty() {
                out.push((task, paths));
            }
        }
    }
    out
}

/// Repo-relative paths changed in `<sha>..HEAD` (one `git diff --name-only` spawn). Empty on git
/// failure (conservative: no drift). Forward slashes (git's native output).
fn changed_paths_since(repo: &Path, sha: &str) -> Vec<String> {
    if sha.is_empty() {
        return Vec::new();
    }
    Command::new("git")
        .arg("-C").arg(repo)
        .args(["diff", "--name-only", &format!("{sha}..HEAD")])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect())
        .unwrap_or_default()
}

/// True if a manifest `path` (file or directory) contains any `changed` repo-relative path.
fn path_drifted(path: &str, changed: &[String]) -> bool {
    let dir_prefix = format!("{path}/");
    changed.iter().any(|c| c == path || c.starts_with(&dir_prefix))
}

/// Mark manifest deliverable tasks suspect when THEIR OWN source drifted since they were verified
/// (D0050, per-task). Perf (orientPerf/sr11): ONE `git diff` per DISTINCT verified-commit (memoized)
/// instead of one `git log` per task — then prefix-match paths in-process.
fn apply_deliverable_suspicion(
    repo: &Path,
    done_map: &HashMap<String, bool>,
    verified_at: &HashMap<String, String>,
    suspect_set: &mut HashSet<String>,
    reasons: &mut HashMap<String, String>,
) {
    let mut changed_by_ct: HashMap<String, Vec<String>> = HashMap::new();
    for (name, paths) in load_deliverable_manifest(repo) {
        if paths.is_empty() || !done_map.get(name.as_str()).copied().unwrap_or(false) {
            continue;
        }
        let Some(ct) = verified_at.get(name.as_str()) else { continue };
        let changed = changed_by_ct.entry(ct.clone()).or_insert_with(|| changed_paths_since(repo, ct));
        if let Some(hit) = paths.iter().find(|p| path_drifted(p, changed)) {
            suspect_set.insert(name.clone());
            let hit = hit.clone();
            reasons.entry(name.clone()).or_insert_with(|| {
                format!("deliverable source [{}] drifted since verified at {ct} (e.g. {hit})", paths.join(", "))
            });
        }
    }
}

/// Propagate suspicion up the dependency graph to fixpoint: a done task is suspect if any of its
/// (non-ordering-only) deps is suspect. Extracted from `compute_orient` (step 4).
fn propagate_transitive_suspect(
    tasks: &HashMap<String, TaskData>,
    ordering_only: &HashSet<(String, String)>,
    done_map: &HashMap<String, bool>,
    suspect_set: &mut HashSet<String>,
    suspect_reasons: &mut HashMap<String, String>,
) {
    let mut changed = true;
    while changed {
        changed = false;
        for (name, data) in tasks {
            if !done_map.get(name.as_str()).copied().unwrap_or(false) || suspect_set.contains(name.as_str()) {
                continue;
            }
            for dep in &data.deps {
                if ordering_only.contains(&(dep.clone(), name.clone())) {
                    continue;
                }
                if suspect_set.contains(dep.as_str()) {
                    suspect_set.insert(name.clone());
                    suspect_reasons.insert(name.clone(), format!("transitively suspect: dependency '{dep}' is suspect"));
                    changed = true;
                    break;
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

    // Validate ALL distinct passing-result SHAs in one batched git spawn (orientPerf/sr11).
    let mut shas: Vec<String> = tasks
        .values()
        .filter_map(|d| d.results.last())
        .filter(|r| r.outcome == "pass" && !r.judged_against.is_empty())
        .map(|r| r.judged_against.clone())
        .collect();
    shas.sort();
    shas.dedup();
    let sha_valid = valid_commits(repo, &shas);

    for (name, data) in &tasks {
        if let Some(latest) = data.results.last() {
            if latest.outcome == "pass" {
                let sha = &latest.judged_against;
                let valid = sha.is_empty() || sha_valid.get(sha).copied().unwrap_or(true);
                if !sha.is_empty() && !valid {
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

    // Step 3: compute suspect (criterion text changed since verified). Capture WHY per task.
    // Perf (orientPerf/sr11): one batched `git cat-file` reads ALL needed historical DoD blobs.
    let dod_files = build_dod_files(repo);
    let mut suspect_set: HashSet<String> = HashSet::new();
    let mut suspect_reasons: HashMap<String, String> = HashMap::new();
    for (name, reason) in criterion_suspects(repo, &tasks, &ordering_only, &done_map, &verified_at, &dod_files) {
        suspect_set.insert(name.clone());
        suspect_reasons.entry(name).or_insert(reason);
    }

    // Step 4: transitive suspect propagation.
    propagate_transitive_suspect(&tasks, &ordering_only, &done_map, &mut suspect_set, &mut suspect_reasons);

    // Step 5: deliverable suspicion (D0050) — manifest tasks whose OWN source drifted since verified.
    apply_deliverable_suspicion(repo, &done_map, &verified_at, &mut suspect_set, &mut suspect_reasons);

    let done = done_map.values().filter(|&&v| v).count();
    let outstanding = done_map.values().filter(|&&v| !v).count();

    // Open issues (D0077): an issue with no complete #Resolves resolver. Reuse this orient
    // run's done-set as the resolver-completeness authority; build the view Model for the edges.
    let done_set: HashSet<String> = done_map.iter().filter(|(_, &v)| v).map(|(k, _)| k.clone()).collect();
    let open_issues = crate::view::open_issue_names(repo, &done_set).unwrap_or_default();

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
        open_issues,
        suspect_reasons,
        done,
        outstanding,
    }
}

/// Done action names: latest `DoD` result passes with a resolvable `judgedAgainst`.
///
/// Exposed as the single done-set authority for the issue-resolution view (D0077): a `#Resolves`
/// action resolver is "complete" iff it is in this set.
#[must_use]
pub fn done_names(root: &Path) -> HashSet<String> {
    let idx = crate::indexer::extract(&root.join(".tracking"));
    let mut done = HashSet::new();
    for (name, data) in &idx.tasks {
        if let Some(latest) = data.results.last() {
            if latest.outcome == "pass" && (latest.judged_against.is_empty() || git_sha_valid(&latest.judged_against, root)) {
                done.insert(name.clone());
            }
        }
    }
    done
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
