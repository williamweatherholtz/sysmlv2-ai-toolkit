//! Git-traversal process-governance views ported from `query.py` (D0074/D0075 M2.2c):
//! `governing-version`, `reprocess-candidates`, and `suspect`.
//!
//! `governing-version` + `reprocess-candidates` are byte-identical to query.py (the pglViews
//! resolver, D0068/D0069/D0070): by git ancestry, the process version that governed a work item
//! AS-OF its charter, plus which process-change Decisions were in force then vs. after, plus the
//! safety-change reprocess set. `suspect` exposes orient's AUTHORITATIVE suspect (criterion-change
//! plus D0050 deliverable-source drift) — a deliberate SUPERSET of query.py suspect, NOT
//! byte-parity, per D0076 (orient is the single source of truth for suspect).

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::algo::{is_word, story_names};
use crate::json::Json;

/// Convention (D0069): a sprint `Story` is governed by the Delivery workflow — the FALLBACK when
/// per-item resolution finds nothing more specific.
const GOVERNING_PROCESS_STORY: &str = ".engine/workflows/delivery.sysml";

/// The governing definition for `item`, resolved PER ITEM from its charter edges (K10/D0183):
/// the item's `#CharteredBy` Decision's introduction commit is inspected for the process/workflow
/// definition files it touched — a Decision that landed a process change governs the work it
/// charters through that definition. Falls back to the D0069 kind convention (Story → Delivery)
/// when the charter touched no definition, which is the common EXECUTE case.
fn governing_def_for(repo: &Path, item: &str) -> String {
    // the charter edge: `#CharteredBy dependency from <item> to <decision>` anywhere in .tracking
    let charter_decision = git_lines(repo, &["grep", "-h", &format!("#CharteredBy dependency from {item} to"), "HEAD", "--", ".tracking"])
        .into_iter()
        .next()
        .and_then(|l| l.split_whitespace().last().map(|d| d.trim_end_matches(';').to_string()));
    if let Some(decision) = charter_decision {
        if let Some(commit) = decision_intro_commit(repo, &decision) {
            let touched = git_lines(repo, &["show", "--name-only", "--format=", &commit, "--", ".engine/processes", ".engine/workflows"]);
            if let Some(def) = touched.into_iter().find(|f| std::path::Path::new(f).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml"))) {
                return def;
            }
        }
    }
    GOVERNING_PROCESS_STORY.to_string()
}

// ── git plumbing ──────────────────────────────────────────────────────────────

/// Run `git -C <repo> <args>`; return non-empty trimmed stdout lines, or `[]` on failure.
fn git_lines(repo: &Path, args: &[&str]) -> Vec<String> {
    // The CALL count now happens in gitx::git() at construction; only the rich detail
    // (argv tally, wall time) stays here, so the two layers never double-count.
    crate::perf::note_git(args);
    let output = crate::perf::timed(&crate::perf::GIT_NANOS, || {
        crate::gitx::git().arg("-C").arg(repo).args(args).output()
    });
    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect(),
        _ => Vec::new(),
    }
}

/// The commit that INTRODUCED a named item into `.tracking/delivery` (charter-time anchor).
fn item_intro_commit(repo: &Path, name: &str) -> Option<String> {
    git_lines(repo, &["log", "--format=%H", "--reverse", "-S", name, "--", ".tracking/delivery"]).into_iter().next()
}

// ── the batched index (issue317) ──────────────────────────────────────────────────────────────
//
// The per-item resolver is CORRECT and unusable at corpus scale: each item costs a pickaxe search
// over full history plus one `merge-base` per process-change Decision, so 501 stories cost roughly
// fifty thousand git subprocesses. Measured: `reprocess-candidates` did not finish in 240 seconds
// (exit 124, zero bytes), which means the question it answers — what was authored under a process
// version since superseded — had never been answered on this project.
//
// Nothing about the DERIVATION changes here. The same facts are read from the same history; they
// are read ONCE into memory instead of once per item. The equality of the two paths is asserted by
// test, because a fast lens that quietly answers differently is worse than a slow correct one.

/// Commit ancestry plus name-introduction, read in a fixed number of git calls.
pub struct GovernIndex {
    /// For each MARKER commit (a process-def change or a Decision's effective commit), every commit
    /// that has it as an ancestor. `is_ancestor(marker, x)` becomes a set lookup.
    descendants: HashMap<String, HashSet<String>>,
    /// Item name -> the commit that introduced it under `.tracking/delivery`.
    intro: HashMap<String, String>,
    /// Process-def path -> the commits that changed it, newest-first.
    def_commits: HashMap<String, Vec<String>>,
    /// Every commit reachable from HEAD — the validity set `git_sha_valid` would answer one at a time.
    known: HashSet<String>,
    /// Item -> the Decision that charters it, read from the working tree rather than `git grep`.
    charter: HashMap<String, String>,
    /// Decision -> the commit that introduced its file.
    decision_intro: HashMap<String, String>,
    /// Commit -> the process/workflow definition files it touched.
    commit_defs: HashMap<String, Vec<String>>,
}

impl GovernIndex {
    fn is_ancestor(&self, marker: &str, commit: &str) -> bool {
        self.descendants.get(marker).is_some_and(|d| d.contains(commit))
    }
    fn sha_valid(&self, sha: &str) -> bool {
        // A short sha is stored in most records; match by prefix against the known set.
        self.known.contains(sha) || self.known.iter().any(|k| k.starts_with(sha))
    }
}

/// `parent -> children`, from one `git rev-list --parents HEAD`.
fn child_map(repo: &Path) -> (HashMap<String, Vec<String>>, HashSet<String>) {
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut known = HashSet::new();
    for line in git_lines(repo, &["rev-list", "--parents", "HEAD"]) {
        let mut it = line.split_whitespace();
        let Some(commit) = it.next() else { continue };
        known.insert(commit.to_string());
        for parent in it {
            children.entry(parent.to_string()).or_default().push(commit.to_string());
        }
    }
    (children, known)
}

/// Every commit reachable FORWARD from `marker` — i.e. every commit `marker` is an ancestor of.
fn descendants_from(children: &HashMap<String, Vec<String>>, marker: &str) -> HashSet<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack = vec![marker.to_string()];
    while let Some(c) = stack.pop() {
        if !seen.insert(c.clone()) {
            continue;
        }
        if let Some(kids) = children.get(&c) {
            stack.extend(kids.iter().cloned());
        }
    }
    seen
}

/// The commit that introduced each of `names`, from ONE pass over `.tracking/delivery` history.
///
/// Faithful to the pickaxe it replaces: `git log -S <name>` finds the first commit where the string
/// appears, so this scans ADDED lines and tokenises them, recording the OLDEST commit in which a
/// name appears. The log is newest-first, so the last sighting wins.
fn intro_commits(repo: &Path, names: &HashSet<String>) -> HashMap<String, String> {
    const MARK: &str = "__keelcommit__";
    let raw = crate::perf::timed(&crate::perf::GIT_NANOS, || {
        crate::gitx::git()
            .arg("-C")
            .arg(repo)
            .args(["log", &format!("--format={MARK}%H"), "-p", "-U0", "--", ".tracking/delivery"])
            .output()
    });
    let Ok(out) = raw else { return HashMap::new() };
    let text = String::from_utf8_lossy(&out.stdout);
    let mut intro: HashMap<String, String> = HashMap::new();
    let mut commit = String::new();
    for line in text.lines() {
        if let Some(sha) = line.strip_prefix(MARK) {
            commit = sha.trim().to_string();
            continue;
        }
        // Only ADDED content introduces a name; `+++` is the file header, not content.
        let Some(added) = line.strip_prefix('+') else { continue };
        if added.starts_with("++") {
            continue;
        }
        for word in added.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
            if !word.is_empty() && names.contains(word) {
                intro.insert(word.to_string(), commit.clone());
            }
        }
    }
    intro
}

/// `#CharteredBy` edges read from the WORKING TREE, not from `git grep` — the edges are current
/// facts, and the per-item resolver was grepping HEAD once per item to learn the same thing.
fn charter_edges(root: &Path) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for path in crate::collect_sysml(&root.join(".tracking")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        for line in text.lines() {
            if let Some(rest) = line.trim().strip_prefix("#CharteredBy dependency from ") {
                if let Some((item, decision)) = rest.split_once(" to ") {
                    out.insert(item.trim().to_string(), decision.trim_end_matches(';').trim().to_string());
                }
            }
        }
    }
    out
}

/// Decision id -> introducing commit, from one `git log --diff-filter=A` over `.engine/decisions`.
fn decision_intro_commits(repo: &Path) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut commit = String::new();
    for line in git_lines(
        repo,
        &["log", "--format=__C__%H", "--name-only", "--diff-filter=A", "--", ".engine/decisions"],
    ) {
        if let Some(sha) = line.strip_prefix("__C__") {
            commit = sha.trim().to_string();
        } else if let Some(file) = std::path::Path::new(&line).file_name().and_then(|f| f.to_str()) {
            // `0261-slug.sysml` -> `d0261`
            let digits: String = file.chars().take_while(char::is_ascii_digit).collect();
            if digits.len() == 4 {
                out.insert(format!("d{digits}"), commit.clone());
            }
        }
    }
    out
}

/// Commit -> process/workflow definition files it touched, and the inverted def -> commits map.
fn def_touch_maps(repo: &Path) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
    let (mut by_commit, mut by_def): (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) =
        (HashMap::new(), HashMap::new());
    let mut commit = String::new();
    for line in git_lines(
        repo,
        &["log", "--format=__C__%H", "--name-only", "--", ".engine/processes", ".engine/workflows"],
    ) {
        if let Some(sha) = line.strip_prefix("__C__") {
            commit = sha.trim().to_string();
        } else if std::path::Path::new(&line).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml")) {
            by_commit.entry(commit.clone()).or_default().push(line.clone());
            by_def.entry(line.clone()).or_default().push(commit.clone());
        }
    }
    (by_commit, by_def)
}

/// Build the index once for a whole-corpus run: a fixed number of git calls regardless of how many
/// items are resolved afterwards. Private: the two public entry points are the only callers, so the
/// index cannot be built inconsistently from outside.
fn build_index(root: &Path, names: &HashSet<String>, markers: &HashSet<String>) -> GovernIndex {
    let (children, known) = child_map(root);
    let mut descendants = HashMap::new();
    let (commit_defs, def_commits) = def_touch_maps(root);
    let decision_intro = decision_intro_commits(root);
    // Markers are the commits ancestry is ever tested against: every def-change commit, plus each
    // Decision's effective commit. Expanding them here means no `merge-base` runs per item.
    let mut all_markers: HashSet<String> = markers.clone();
    for commits in def_commits.values() {
        all_markers.extend(commits.iter().cloned());
    }
    for m in &all_markers {
        let full = if known.contains(m) {
            Some(m.clone())
        } else {
            known.iter().find(|k| k.starts_with(m.as_str())).cloned()
        };
        if let Some(full) = full {
            descendants.insert(m.clone(), descendants_from(&children, &full));
        }
    }
    GovernIndex {
        descendants,
        intro: intro_commits(root, names),
        def_commits,
        known,
        charter: charter_edges(root),
        decision_intro,
        commit_defs,
    }
}

/// The indexed twin of [`govern_resolve`] — same derivation, no per-item git.
fn govern_resolve_indexed(idx: &GovernIndex, pcs: &[ProcChange], item: &str) -> GovernData {
    let governing_def = idx
        .charter
        .get(item)
        .and_then(|d| idx.decision_intro.get(d))
        .and_then(|c| idx.commit_defs.get(c))
        .and_then(|defs| defs.first().cloned())
        .unwrap_or_else(|| GOVERNING_PROCESS_STORY.to_string());
    let Some(item_commit) = idx.intro.get(item).cloned() else {
        return GovernData {
            governing_def,
            item: item.to_string(),
            error: Some("no introduction commit found in .tracking/delivery".to_string()),
            item_commit: None,
            governing: None,
            later_count: 0,
            in_force: Vec::new(),
            after: Vec::new(),
            reprocess: Vec::new(),
        };
    };
    let empty = Vec::new();
    let def_commits = idx.def_commits.get(&governing_def).unwrap_or(&empty);
    let governing = def_commits.iter().find(|c| idx.is_ancestor(c, &item_commit)).cloned();
    let later_count = def_commits.iter().filter(|c| !idx.is_ancestor(c, &item_commit)).count();

    let (mut in_force, mut after) = (Vec::new(), Vec::new());
    for d in pcs {
        let Some(ec) = &d.effective_commit else { continue };
        if !idx.sha_valid(ec) {
            continue;
        }
        if idx.is_ancestor(ec, &item_commit) {
            in_force.push(d.decision.clone());
        } else {
            after.push((d.decision.clone(), d.retroactivity.clone()));
        }
    }
    let mut reprocess: Vec<String> = after.iter().filter(|(_, r)| r == "safety").map(|(d, _)| d.clone()).collect();
    reprocess.sort();
    GovernData {
        governing_def,
        item: item.to_string(),
        error: None,
        item_commit: Some(item_commit),
        governing,
        later_count,
        in_force,
        after,
        reprocess,
    }
}

/// Commits that changed a process-def file, newest-first.
fn def_change_commits(repo: &Path, path: &str) -> Vec<String> {
    git_lines(repo, &["log", "--format=%H", "--", path])
}

/// True if commit `a` is an ancestor of `b`.
fn is_ancestor(repo: &Path, a: &str, b: &str) -> bool {
    crate::gitx::git()
        .arg("-C")
        .arg(repo)
        .args(["merge-base", "--is-ancestor", a, b])
        .output()
        .is_ok_and(|o| o.status.success())
}

// ── process-change Decisions ──────────────────────────────────────────────────

struct ProcChange {
    decision: String,
    retroactivity: String,
    effective_commit: Option<String>,
}

/// Process-change Decisions across `.engine/decisions` (sorted) with their effective commit
/// (the acceptance event's `judgedAgainst`). Mirrors query.py's `process_change_decisions_full`,
/// minus the `governed_defs` field (unused by the governing-version resolver).
fn proc_change_decisions(root: &Path) -> Vec<ProcChange> {
    let mut out = Vec::new();
    for path in crate::collect_sysml(&root.join(".engine").join("decisions")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        for (retro, dec) in scan_proc_change_markers(&text) {
            let effective_commit = acceptance_judged_against(&text, &dec);
            out.push(ProcChange { decision: dec, retroactivity: retro, effective_commit });
        }
    }
    out
}

/// `^[ \t]*#(ProspectiveChange|SafetyChange)\s+part\s+(\w+)\s*:\s*Decision\b` per line →
/// `(retroactivity, decision)`. Prose/comment mentions don't match (they don't start with `#`).
fn scan_proc_change_markers(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim_start_matches(crate::algo::is_space);
        let (retro, rest) = if let Some(r) = t.strip_prefix("#ProspectiveChange") {
            ("prospective", r)
        } else if let Some(r) = t.strip_prefix("#SafetyChange") {
            ("safety", r)
        } else {
            continue;
        };
        let rest_ws = rest.trim_start_matches(crate::algo::is_space);
        if rest_ws.len() == rest.len() {
            continue; // require whitespace after the marker
        }
        let Some(after_part) = rest_ws.strip_prefix("part") else { continue };
        let after_part_ws = after_part.trim_start_matches(crate::algo::is_space);
        if after_part_ws.len() == after_part.len() {
            continue; // require whitespace after `part`
        }
        let ident: String = after_part_ws.chars().take_while(|c| is_word(*c)).collect();
        if ident.is_empty() {
            continue;
        }
        let Some(r) = after_part_ws.strip_prefix(ident.as_str()) else { continue };
        let r = r.trim_start_matches(crate::algo::is_space);
        let Some(r) = r.strip_prefix(':') else { continue };
        let r = r.trim_start_matches(crate::algo::is_space);
        let Some(tail) = r.strip_prefix("Decision") else { continue };
        if tail.chars().next().is_none_or(|c| !is_word(c)) {
            out.push((retro.to_string(), ident));
        }
    }
    out
}

/// `\b{dec}AcceptR1\b.*?judgedAgainst\s*=\s*"(\w+)"` (DOTALL) — the acceptance event's commit.
fn acceptance_judged_against(text: &str, dec: &str) -> Option<String> {
    let needle = format!("{dec}AcceptR1");
    let pos = text.find(&needle)?;
    let after = &text[pos..];
    let ja = after.find("judgedAgainst")?;
    let after_ja = &after[ja..];
    let eq = after_ja.find('=')?;
    let after_eq = &after_ja[eq + 1..];
    let q1 = after_eq.find('"')?;
    let after_q1 = &after_eq[q1 + 1..];
    let val: String = after_q1.chars().take_while(|c| is_word(*c)).collect();
    if val.is_empty() {
        None
    } else {
        Some(val)
    }
}

// ── charter-time scoping for the assurance gates (D0068 freeze, D0081) ─────────────────────────

/// Every commit's author date, keyed by full SHA — ONE spawn for the whole history (issue148).
///
/// This replaces a `git show -s` PER SHA. That version cost 480 spawns and ~15 SECONDS inside `keel
/// assured`, because `assured` runs every guard and `impossible-evidence-date` asks for the date of
/// every distinct SHA in the corpus. A per-item git call is fine at ten items and catastrophic at five
/// hundred; the whole history is one `git log`, so there is no reason to ever pay per item.
///
/// Memoized for the process. Safe because D0129 forbids rewriting history, so a commit's author date is
/// immutable — and any commit created DURING a run postdates every judgment a run could be reading,
/// which is exactly the case the caller treats as a violation rather than a lookup.
fn commit_dates(repo: &Path) -> &'static std::collections::HashMap<String, String> {
    static CACHE: std::sync::OnceLock<std::collections::HashMap<String, String>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| {
        git_lines(repo, &["log", "--all", "--format=%H %ad", "--date=short"])
            .into_iter()
            .filter_map(|l| {
                let (sha, date) = l.split_once(' ')?;
                Some((sha.to_string(), date.to_string()))
            })
            .collect()
    })
}

/// The author date (YYYY-MM-DD) of `sha`, or `None` if it does not resolve.
///
/// Accepts an ABBREVIATED sha, because that is what `judgedAgainst` records: an exact hit is tried
/// first, then a unique prefix match. An AMBIGUOUS prefix returns `None` rather than an arbitrary
/// winner — a date attached to the wrong commit is worse than no date, since callers use it to decide
/// whether a judgment was possible.
///
/// Author date rather than commit date: a merge or a re-application should not move when a judgment
/// was possible, and D0129 forbids the history rewriting that would make the two diverge anyway.
#[must_use]
pub fn commit_date(repo: &Path, sha: &str) -> Option<String> {
    let map = commit_dates(repo);
    if let Some(d) = map.get(sha) {
        return Some(d.clone());
    }
    let mut hit = None;
    for (full, date) in map {
        if full.starts_with(sha) {
            if hit.is_some() {
                return None; // ambiguous prefix
            }
            hit = Some(date.clone());
        }
    }
    hit
}

/// The INTRODUCTION commit of a Decision — the first commit that added `part <decision> :` under
/// `.engine/decisions` (when the rule landed). `None` if not yet committed (e.g. staged-only).
#[must_use]
pub fn decision_intro_commit(root: &Path, decision: &str) -> Option<String> {
    git_lines(root, &["log", "--format=%H", "--reverse", "-S", &format!("part {decision} :"), "--", ".engine/decisions"])
        .into_iter()
        .next()
}

/// Assurance-element names (`part`/`requirement` defs in `.tracking` + `.engine/decisions`) present
/// AS-OF `commit` — the grandfather set for charter-time scoping. Empty on git failure.
fn names_present_at(repo: &Path, commit: &str) -> std::collections::HashSet<String> {
    git_lines(repo, &["grep", "-hoE", "(part|requirement) [A-Za-z0-9_]+ :", commit, "--", ".tracking", ".engine/decisions"])
        .iter()
        .filter_map(|l| l.split_whitespace().nth(1).map(String::from))
        .collect()
}

/// The set of element names GRANDFATHERED under `decision`.
///
/// Those present when the rule landed (at the decision's introduction commit), hence out of scope
/// for its prospective requirement (charter-time freeze, D0068/D0081). New elements (created after)
/// are NOT grandfathered.
///
/// `None` if the decision's introduction commit can't be resolved (not yet committed / git
/// unavailable) — the caller then treats EVERYTHING as grandfathered so the gate never spuriously
/// blocks (conservative; matches the D0050 git-failure stance).
#[must_use]
pub fn grandfathered_under(root: &Path, decision: &str) -> Option<std::collections::HashSet<String>>{
    crate::perf::add(&crate::perf::GF_CALLS, 1);
    let commit = decision_intro_commit(root, decision)?;
    Some(names_present_at(root, &commit))
}

// ── the resolver ──────────────────────────────────────────────────────────────

struct GovernData {
    governing_def: String,
    item: String,
    error: Option<String>,
    item_commit: Option<String>,
    governing: Option<String>,
    later_count: usize,
    in_force: Vec<String>,
    after: Vec<(String, String)>,
    reprocess: Vec<String>,
}

fn govern_resolve(repo: &Path, pcs: &[ProcChange], item: &str) -> GovernData {
    let governing_def = governing_def_for(repo, item);
    let Some(item_commit) = item_intro_commit(repo, item) else {
        return GovernData {
            governing_def,
            item: item.to_string(),
            error: Some("no introduction commit found in .tracking/delivery".to_string()),
            item_commit: None,
            governing: None,
            later_count: 0,
            in_force: Vec::new(),
            after: Vec::new(),
            reprocess: Vec::new(),
        };
    };
    let def_commits = def_change_commits(repo, &governing_def);
    let governing = def_commits.iter().find(|c| is_ancestor(repo, c, &item_commit)).cloned();
    let later_count = def_commits.iter().filter(|c| !is_ancestor(repo, c, &item_commit)).count();

    let mut in_force: Vec<String> = Vec::new();
    let mut after: Vec<(String, String)> = Vec::new();
    for d in pcs {
        let Some(ec) = &d.effective_commit else { continue };
        if !crate::orient::git_sha_valid(ec, repo) {
            continue;
        }
        if is_ancestor(repo, ec, &item_commit) {
            in_force.push(d.decision.clone());
        } else {
            after.push((d.decision.clone(), d.retroactivity.clone()));
        }
    }
    let mut reprocess: Vec<String> = after.iter().filter(|(_, r)| r == "safety").map(|(d, _)| d.clone()).collect();
    reprocess.sort();

    GovernData {
        governing_def,
        item: item.to_string(),
        error: None,
        item_commit: Some(item_commit),
        governing,
        later_count,
        in_force,
        after,
        reprocess,
    }
}

fn governing_version_json(d: &GovernData) -> Json {
    if let Some(err) = &d.error {
        return Json::Obj(vec![("item".to_string(), Json::s(d.item.clone())), ("error".to_string(), Json::s(err.clone()))]);
    }
    let mut in_force = d.in_force.clone();
    in_force.sort();
    let item_commit = d.item_commit.clone().unwrap_or_default();
    let governing_commit = d.governing.as_ref().map_or(Json::Null, |g| Json::s(g.clone()));
    let process_as_it_was = d
        .governing
        .as_ref()
        .map_or(Json::Null, |g| Json::s(format!("git show {g}:{}", d.governing_def)));
    let after_json: Vec<Json> = d
        .after
        .iter()
        .map(|(dec, retro)| Json::Obj(vec![("decision".to_string(), Json::s(dec.clone())), ("retroactivity".to_string(), Json::s(retro.clone()))]))
        .collect();

    Json::Obj(vec![
        ("item".to_string(), Json::s(d.item.clone())),
        ("process".to_string(), Json::s("Delivery")),
        ("process_def".to_string(), Json::s(d.governing_def.clone())),
        ("convention".to_string(), Json::s("resolved per item from its charter edges (K10/D0183); Story->Delivery is the fallback (D0069)")),
        ("item_commit".to_string(), Json::s(item_commit)),
        ("governing_version_commit".to_string(), governing_commit),
        ("process_as_it_was".to_string(), process_as_it_was),
        ("later_version_count".to_string(), Json::Int(i64::try_from(d.later_count).unwrap_or(i64::MAX))),
        ("decisions_in_force_at_charter".to_string(), Json::Arr(in_force.into_iter().map(Json::s).collect())),
        ("process_changes_after_charter".to_string(), Json::Arr(after_json)),
        ("reprocess_required".to_string(), Json::Bool(!d.reprocess.is_empty())),
        ("reprocess_due_to".to_string(), Json::Arr(d.reprocess.iter().map(|x| Json::s(x.clone())).collect())),
        ("valid_then".to_string(), Json::s("asserted by the item's own ceremony gates (they encode the process it followed)")),
    ])
}

// ── public subcommands ────────────────────────────────────────────────────────

/// The INDEXED twin of [`governing_version`], returning the identical JSON.
///
/// Not the default for a single item: building the index costs one full `git log -p` over delivery
/// history, which is more than the ~15s a single per-item resolve takes. It exists so the two paths
/// can be compared for EQUALITY by test — a fast lens that quietly answers differently is worse
/// than a slow correct one, and without this the batched path would be trusted on its speed alone.
#[must_use]
pub fn governing_version_via_index(root: &Path, item: &str) -> String {
    let pcs = proc_change_decisions(root);
    let names: HashSet<String> = std::iter::once(item.to_string()).collect();
    let markers: HashSet<String> = pcs.iter().filter_map(|p| p.effective_commit.clone()).collect();
    let idx = build_index(root, &names, &markers);
    governing_version_json(&govern_resolve_indexed(&idx, &pcs, item)).dump()
}

/// The process version governing `item` as-of its charter (D0068), as JSON — byte-identical to
/// `query.py governing-version <item>`.
#[must_use]
pub fn governing_version(root: &Path, item: &str) -> String {
    let pcs = proc_change_decisions(root);
    governing_version_json(&govern_resolve(root, &pcs, item)).dump()
}

/// Items chartered under a process version later superseded by a SAFETY change, as JSON —
/// byte-identical to `query.py reprocess-candidates`.
#[must_use]
pub fn reprocess_candidates(root: &Path) -> String {
    let pcs = proc_change_decisions(root);
    let stories = all_delivery_stories(root);
    // ONE index for the whole corpus (issue317). The per-item path costs a pickaxe search plus a
    // `merge-base` per Decision; at 501 stories that never finished.
    let names: HashSet<String> = stories.iter().cloned().collect();
    let markers: HashSet<String> = pcs.iter().filter_map(|p| p.effective_commit.clone()).collect();
    let idx = build_index(root, &names, &markers);
    let mut items: Vec<Json> = Vec::new();
    for story in stories {
        let d = govern_resolve_indexed(&idx, &pcs, &story);
        if !d.reprocess.is_empty() {
            items.push(Json::Obj(vec![
                ("item".to_string(), Json::s(story)),
                ("due_to".to_string(), Json::Arr(d.reprocess.iter().map(|x| Json::s(x.clone())).collect())),
            ]));
        }
    }
    Json::Obj(vec![("reprocess_candidates".to_string(), Json::Arr(items))]).dump()
}

/// Orient's AUTHORITATIVE suspect set (criterion-change + D0050 deliverable drift) as JSON.
/// A deliberate SUPERSET of `query.py suspect` (NOT byte-parity), per D0076.
#[must_use]
pub fn suspect(root: &Path, explain: bool) -> String {
    let out = crate::orient::compute(root);
    // D0086: elements rendered suspect by an unresolved failing critique (a human review's finding).
    let crit = crate::view::critique_suspect(root).unwrap_or_default();
    let crit_json = Json::Arr(crit.iter().map(|s| Json::s(s.clone())).collect());
    if !explain {
        return Json::Obj(vec![
            ("suspect".to_string(), Json::Arr(out.suspect.iter().map(|s| Json::s(s.clone())).collect())),
            ("critique_suspect".to_string(), crit_json),
        ])
        .dump();
    }
    // --explain (suspectDiagnostics): per suspect task, WHY it is flagged.
    let arr: Vec<Json> = out
        .suspect
        .iter()
        .map(|t| {
            let reason = out.suspect_reasons.get(t).cloned().unwrap_or_else(|| "suspect (no recorded reason)".to_string());
            Json::Obj(vec![("task".to_string(), Json::s(t.clone())), ("reason".to_string(), Json::s(reason))])
        })
        .collect();
    Json::Obj(vec![
        ("suspect".to_string(), Json::Arr(arr)),
        ("critique_suspect".to_string(), crit_json),
    ])
    .dump()
}

fn all_delivery_stories(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for path in crate::collect_sysml(&root.join(".tracking").join("delivery")) {
        if let Ok(text) = std::fs::read_to_string(&path) {
            out.extend(story_names(&text));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markers_scanned_at_line_start_only() {
        let text = "    #ProspectiveChange part d0039 : Decision {\n    :>> title = \"mentions #SafetyChange in prose\";\n    #SafetyChange part d0099 : Decision {\n";
        let got = scan_proc_change_markers(text);
        assert_eq!(got, vec![("prospective".to_string(), "d0039".to_string()), ("safety".to_string(), "d0099".to_string())]);
    }

    #[test]
    fn acceptance_commit_extracted() {
        let text = "part d0070AcceptR1 : TestResult {\n  :>> outcome = VerdictKind::pass;\n  :>> judgedAgainst = \"abc1234\";\n}";
        assert_eq!(acceptance_judged_against(text, "d0070"), Some("abc1234".to_string()));
        assert_eq!(acceptance_judged_against(text, "d9999"), None);
    }
}
