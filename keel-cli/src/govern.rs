//! Git-traversal process-governance views ported from `query.py` (D0074/D0075 M2.2c):
//! `governing-version`, `reprocess-candidates`, and `suspect`.
//!
//! `governing-version` + `reprocess-candidates` are byte-identical to query.py (the pglViews
//! resolver, D0068/D0069/D0070): by git ancestry, the process version that governed a work item
//! AS-OF its charter, plus which process-change Decisions were in force then vs. after, plus the
//! safety-change reprocess set. `suspect` exposes orient's AUTHORITATIVE suspect (criterion-change
//! plus D0050 deliverable-source drift) — a deliberate SUPERSET of query.py suspect, NOT
//! byte-parity, per D0076 (orient is the single source of truth for suspect).

use std::path::Path;
use std::process::Command;

use crate::algo::{is_word, story_names};
use crate::json::Json;

/// Convention (D0069): a sprint `Story` is governed by the Delivery workflow.
const GOVERNING_PROCESS_STORY: &str = ".engine/workflows/delivery.sysml";

// ── git plumbing ──────────────────────────────────────────────────────────────

/// Run `git -C <repo> <args>`; return non-empty trimmed stdout lines, or `[]` on failure.
fn git_lines(repo: &Path, args: &[&str]) -> Vec<String> {
    let output = Command::new("git").arg("-C").arg(repo).args(args).output();
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

/// Commits that changed a process-def file, newest-first.
fn def_change_commits(repo: &Path, path: &str) -> Vec<String> {
    git_lines(repo, &["log", "--format=%H", "--", path])
}

/// True if commit `a` is an ancestor of `b`.
fn is_ancestor(repo: &Path, a: &str, b: &str) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["merge-base", "--is-ancestor", a, b])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
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
pub fn grandfathered_under(root: &Path, decision: &str) -> Option<std::collections::HashSet<String>> {
    let commit = decision_intro_commit(root, decision)?;
    Some(names_present_at(root, &commit))
}

// ── the resolver ──────────────────────────────────────────────────────────────

struct GovernData {
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
    let Some(item_commit) = item_intro_commit(repo, item) else {
        return GovernData {
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
    let def_commits = def_change_commits(repo, GOVERNING_PROCESS_STORY);
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
        .map_or(Json::Null, |g| Json::s(format!("git show {g}:{GOVERNING_PROCESS_STORY}")));
    let after_json: Vec<Json> = d
        .after
        .iter()
        .map(|(dec, retro)| Json::Obj(vec![("decision".to_string(), Json::s(dec.clone())), ("retroactivity".to_string(), Json::s(retro.clone()))]))
        .collect();

    Json::Obj(vec![
        ("item".to_string(), Json::s(d.item.clone())),
        ("process".to_string(), Json::s("Delivery")),
        ("process_def".to_string(), Json::s(GOVERNING_PROCESS_STORY)),
        ("convention".to_string(), Json::s("a sprint Story is governed by Delivery (D0069 work->process by kind)")),
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
    let mut items: Vec<Json> = Vec::new();
    for story in all_delivery_stories(root) {
        let d = govern_resolve(root, &pcs, &story);
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
