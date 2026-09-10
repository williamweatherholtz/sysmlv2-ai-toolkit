//! `keel show commit-delta` — what a git range did to the model, before anyone reads a hunk
//! (dcCommitDeltaView, D0282).
//!
//! keel commits to `main` without pull requests, so a change reaches the human as `SysML` hunks at
//! `GitHub`. The landscape finding this answers (`OpenSpec`'s team workflow): a reviewer handed the delta
//! SPEC ahead of the diff sees WHAT the change is supposed to do.
//!
//! This lens is that spec, computed:
//! over `from..to` it lists the Needs, Requirements, Decisions, Issues and tasks the range ADDED, the
//! items it RETIRED (`#Supersede` edges new in the range, D0398) and the Issues it RESOLVED
//! (`#Resolves` edges new in the range), each with its type and title. Nothing is stored; the two
//! models are built from the two commits and compared.
//!
//! The view carries its own reconciliation: the `+part <name> : <Type>` / `+action <name>;` lines
//! `git diff from..to -- .tracking .engine` adds are counted per type beside the view's own counts,
//! and `matches` says whether they agree. They disagree honestly when an item MOVED between files
//! (the diff adds a line, the model gains no item) - the view names the diff's count and its own
//! rather than choosing one. A range with no model change renders its lists empty and
//! `empty = true`; it never invents prose about what the range meant.
//!
//! The decision-channel comment the item's `DoD` named as a second surface is moot: D0291 disconnected
//! the channel, so the surface is `scripts/exec_brief/facts.py` (the `commitDelta` fact) and the
//! published decision brief's section built from it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::json::Json;

use super::{display_label, Model, ViewError};

/// The item types the delta reports: what the human's authority already covers.
pub const DELTA_TYPES: [&str; 6] = ["Need", "SystemRequirement", "Requirement", "Decision", "Issue", "action"];

/// One item the range touched, as the view renders it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeltaItem {
    pub name: String,
    pub type_name: String,
    pub title: String,
    /// The item at the other end of the edge for a retirement or resolution; empty for an addition.
    pub by: String,
}

/// The delta between two models, restricted to [`DELTA_TYPES`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Delta {
    pub added: Vec<DeltaItem>,
    pub superseded: Vec<DeltaItem>,
    pub resolved: Vec<DeltaItem>,
}

impl Delta {
    /// `true` when the range changed nothing the view reports.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.added.is_empty() && self.superseded.is_empty() && self.resolved.is_empty()
    }

    /// Additions per type, the number the diff's count is reconciled against.
    #[must_use]
    pub fn added_by_type(&self) -> BTreeMap<String, usize> {
        let mut out = BTreeMap::new();
        for a in &self.added {
            *out.entry(a.type_name.clone()).or_insert(0) += 1;
        }
        out
    }
}

/// Pure core: the items in `to` but not in `from`, and the retirement / resolution edges `to`
/// carries that `from` did not. Sorted by type then name so the rendering is stable.
pub(super) fn delta(from: &Model, to: &Model) -> Delta {
    let mut d = Delta::default();
    for (name, info) in &to.items {
        if DELTA_TYPES.contains(&info.type_name.as_str()) && !from.items.contains_key(name) {
            d.added.push(DeltaItem { name: name.clone(), type_name: info.type_name.clone(), title: display_label(name, info), by: String::new() });
        }
    }
    let before: BTreeSet<(String, String, String)> = from.edges.iter().map(|e| (e.kind.clone(), e.from.clone(), e.to.clone())).collect();
    for e in &to.edges {
        if before.contains(&(e.kind.clone(), e.from.clone(), e.to.clone())) {
            continue;
        }
        let target = to.items.get(&e.to);
        let item = |type_fallback: &str| DeltaItem {
            name: e.to.clone(),
            type_name: target.map_or_else(|| type_fallback.to_string(), |t| t.type_name.clone()),
            title: target.map_or_else(|| e.to.clone(), |t| display_label(&e.to, t)),
            by: e.from.clone(),
        };
        match e.kind.as_str() {
            "supersede" => d.superseded.push(item("")),
            "resolves" if target.is_some_and(|t| t.type_name == "Issue") => d.resolved.push(item("Issue")),
            _ => {}
        }
    }
    let key = |i: &DeltaItem| (i.type_name.clone(), i.name.clone());
    d.added.sort_by_key(key);
    d.superseded.sort_by_key(key);
    d.resolved.sort_by_key(key);
    d
}

/// Pure core of the reconciliation: per type, how many item declarations a unified diff adds NET.
///
/// A declaration is `part <name> : <Type>` or `action <name>;`, with any leading `#Marker` tokens
/// skipped (`#ProspectiveChange part d0431 : Decision` is a Decision - the first live range read it
/// as nothing). A name both removed and added in the same diff is a MOVE, not an addition, and is
/// not counted (the backlog re-declared `action dcGuardPoolDispatchesLongestFirst;` one line down
/// and the first cut read that as a second task). `part def` / `action def` are definitions, not
/// items; a `+++` / `---` header is not a line.
#[must_use]
pub fn diff_added_counts(diff: &str) -> BTreeMap<String, usize> {
    let mut added: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut removed: BTreeSet<(String, String)> = BTreeSet::new();
    for line in diff.lines() {
        let (sign, body) = match line.as_bytes().first() {
            Some(b'+') if !line.starts_with("+++") => ('+', &line[1..]),
            Some(b'-') if !line.starts_with("---") => ('-', &line[1..]),
            _ => continue,
        };
        let Some((type_name, name)) = declaration(body) else { continue };
        if !DELTA_TYPES.contains(&type_name) {
            continue;
        }
        let key = (type_name.to_string(), name.to_string());
        if sign == '+' {
            *added.entry(key).or_insert(0) += 1;
        } else {
            removed.insert(key);
        }
    }
    let mut out = BTreeMap::new();
    for ((type_name, _), n) in added.into_iter().filter(|(k, _)| !removed.contains(k)) {
        *out.entry(type_name).or_insert(0) += n;
    }
    out
}

/// `(type, name)` of the item a source line declares, or `None` for anything else (a definition, a
/// field, a marker-only line). Leading `#Marker` tokens are skipped.
fn declaration(body: &str) -> Option<(&str, &str)> {
    let toks: Vec<&str> = body.split_whitespace().skip_while(|t| t.starts_with('#')).collect();
    let kw = *toks.first()?;
    let name = *toks.get(1)?;
    if name == "def" {
        return None;
    }
    let name = name.trim_end_matches([';', '{']);
    let typed = match (toks.get(2), toks.get(3)) {
        (Some(&":"), Some(t)) => Some(t.trim_end_matches(['{', ';'])),
        _ => None,
    };
    match kw {
        // a bare `action x;` is a task; `action x : ProcessStep` is whatever it is typed as
        "action" => Some((typed.unwrap_or("action"), name)),
        "part" | "requirement" | "occurrence" | "item" => typed.map(|t| (t, name)),
        _ => None,
    }
}

fn items_json(v: &[DeltaItem], with_by: &str) -> Json {
    Json::Arr(
        v.iter()
            .map(|i| {
                let mut o = vec![
                    ("name".to_string(), Json::s(i.name.clone())),
                    ("type".to_string(), Json::s(i.type_name.clone())),
                    ("title".to_string(), Json::s(i.title.clone())),
                ];
                if !with_by.is_empty() {
                    o.push((with_by.to_string(), Json::s(i.by.clone())));
                }
                Json::Obj(o)
            })
            .collect(),
    )
}

fn counts_json(m: &BTreeMap<String, usize>) -> Json {
    Json::Obj(m.iter().map(|(k, v)| (k.clone(), Json::Int(i64::try_from(*v).unwrap_or(0)))).collect())
}

/// Split `A..B`; a bare ref is `<ref>~1..<ref>`.
#[must_use]
pub fn parse_range(range: &str) -> (String, String) {
    match range.split_once("..") {
        Some((a, b)) if !a.is_empty() && !b.is_empty() => (a.to_string(), b.to_string()),
        _ => (format!("{range}~1"), range.to_string()),
    }
}

/// The lens: the model delta over `range` (`A..B`; default `HEAD~1..HEAD`), reconciled against the
/// diff's added declarations.
///
/// # Errors
/// Returns [`ViewError`] if either end of the range cannot be checked out or a model fails to build.
pub fn commit_delta(root: &Path, range: &str) -> Result<String, ViewError> {
    let (from, to) = parse_range(range);
    let m_from = super::model_at_commit(root, &from, "delta-from")?;
    let m_to = super::model_at_commit(root, &to, "delta-to")?;
    let d = delta(&m_from, &m_to);
    let diff = super::reports::git_out(root, &["diff", &format!("{from}..{to}"), "--", ".tracking", ".engine"]).unwrap_or_default();
    let diff_counts = diff_added_counts(&diff);
    let view_counts = d.added_by_type();
    let matches = diff_counts == view_counts;
    Ok(Json::Obj(vec![
        ("range".to_string(), Json::s(format!("{from}..{to}"))),
        ("empty".to_string(), Json::Bool(d.is_empty())),
        ("added".to_string(), items_json(&d.added, "")),
        ("superseded".to_string(), items_json(&d.superseded, "by")),
        ("resolved".to_string(), items_json(&d.resolved, "by")),
        (
            "reconciliation".to_string(),
            Json::Obj(vec![
                ("viewAdded".to_string(), counts_json(&view_counts)),
                ("gitDiffAdded".to_string(), counts_json(&diff_counts)),
                ("matches".to_string(), Json::Bool(matches)),
                ("how".to_string(), Json::s(format!("viewAdded: items of a DELTA type in the model at {to} and not at {from}; gitDiffAdded: `+part <name> : <Type>` / `+action <name>;` lines in `git diff {from}..{to} -- .tracking .engine`, definitions excluded; they differ when an item moved between files"))),
            ]),
        ),
        ("how".to_string(), Json::s("two models built from the two commits (git worktree), compared; retirements are #Supersede edges new in the range (D0398), resolutions #Resolves edges new in the range whose target is an Issue; nothing stored (dcCommitDeltaView, D0282)")),
    ])
    .dump())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::super::{Edge, ItemInfo, Model};
    use super::{delta, diff_added_counts, parse_range};

    fn item(t: &str, title: &str) -> ItemInfo {
        let mut attrs = HashMap::new();
        if !title.is_empty() {
            attrs.insert("title".to_string(), title.to_string());
        }
        ItemInfo { type_name: t.to_string(), attrs, marker: None, file: String::new() }
    }

    fn model(items: &[(&str, &str, &str)], edges: &[(&str, &str, &str)]) -> Model {
        Model {
            items: items.iter().map(|(n, t, title)| ((*n).to_string(), item(t, title))).collect(),
            edges: edges.iter().map(|(k, f, t)| Edge { kind: (*k).to_string(), from: (*f).to_string(), to: (*t).to_string() }).collect(),
        }
    }

    /// The D0388 pair. KNOWN-POSITIVE: a range that adds a Decision, retires another and resolves an
    /// Issue names all three, each once, and ignores the Test it also added. KNOWN-NEGATIVE: the same
    /// model at both ends is empty - no item, no prose.
    #[test]
    fn a_range_that_adds_retires_and_resolves_names_each_once_and_an_unchanged_range_is_empty() {
        let before = model(&[("d0001", "Decision", "old"), ("issue1", "Issue", "a defect"), ("t1", "Test", "")], &[]);
        let after = model(
            &[("d0001", "Decision", "old"), ("d0002", "Decision", "new rule"), ("issue1", "Issue", "a defect"), ("t1", "Test", ""), ("t2", "Test", "")],
            &[("supersede", "d0002", "d0001"), ("resolves", "d0002", "issue1")],
        );
        let d = delta(&before, &after);
        assert_eq!(d.added.len(), 1, "{d:?}");
        assert_eq!((d.added[0].name.as_str(), d.added[0].title.as_str()), ("d0002", "new rule"));
        assert_eq!(d.superseded.len(), 1);
        assert_eq!((d.superseded[0].name.as_str(), d.superseded[0].by.as_str()), ("d0001", "d0002"));
        assert_eq!(d.resolved.len(), 1);
        assert_eq!(d.resolved[0].name, "issue1");
        assert!(!d.is_empty());
        assert!(delta(&after, &after).is_empty(), "identical ends are an empty delta");
    }

    /// A resolves edge whose target is not an Issue is not a resolution, and an edge that already
    /// existed at the start of the range is not new.
    #[test]
    fn only_new_edges_count_and_only_issues_resolve() {
        let before = model(&[("d1", "Decision", ""), ("d0", "Decision", "")], &[("supersede", "d1", "d0")]);
        let after = model(&[("d1", "Decision", ""), ("d0", "Decision", ""), ("t", "action", "")], &[("supersede", "d1", "d0"), ("resolves", "d1", "t")]);
        let d = delta(&before, &after);
        assert!(d.superseded.is_empty(), "{d:?}");
        assert!(d.resolved.is_empty(), "{d:?}");
        assert_eq!(d.added_by_type().get("action"), Some(&1));
    }

    /// The reconciliation counter reads declarations, not definitions or headers.
    #[test]
    fn diff_counts_added_declarations_by_delta_type() {
        // d0002 added; d0003 removed only; d0004 added under a marker; dcMoved removed and re-added (a
        // move, not an addition); a typed action is its type (ProcessStep, not a task); definitions and a
        // Test never counted
        let diff = "+++ b/x.sysml\n--- a/x.sysml\n+    part d0002 : Decision { :>> id = \"x\"; }\n+    part def Decision;\n+    action dcNew;\n+    action def RunS1 {\n-    part d0003 : Decision;\n+    part t2 : Test { }\n+    requirement r1 : SystemRequirement {\n+    #ProspectiveChange part d0004 : Decision {\n-        action dcMoved;\n+        action dcMoved;\n+    action stpa9 : ProcessStep {\n";
        let c = diff_added_counts(diff);
        assert_eq!(c.get("Decision"), Some(&2), "{c:?}");
        assert_eq!(c.get("action"), Some(&1), "{c:?}");
        assert_eq!(c.get("SystemRequirement"), Some(&1), "{c:?}");
        assert_eq!(c.get("Test"), None);
        assert_eq!(c.get("ProcessStep"), None);
        assert_eq!(parse_range("a..b"), ("a".to_string(), "b".to_string()));
        assert_eq!(parse_range("HEAD"), ("HEAD~1".to_string(), "HEAD".to_string()));
    }
}
