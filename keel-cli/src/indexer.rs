//! AST-driven project indexer — replaces orient.rs text-scan for data extraction.
//!
//! Parses all `.sysml` files under `.tracking/` using `keel_parser` and builds
//! a typed in-memory index used by the orient and query commands.  Git validation
//! (suspect, invalid-evidence) is NOT done here; that lives in `orient.rs`.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use keel_parser::ast::{Attribute, Item, Package, Value};

use crate::collect_sysml;

// ── exported data types ───────────────────────────────────────────────────────

/// A single test-result record extracted from a `part <name>DoDR<n> : TestResult`.
pub struct ResultRecord {
    /// Sequence number from the name suffix (e.g. `1` from `DoDR1` or `R1`).
    pub n: u32,
    /// Value of the `outcome` attribute (e.g. `"pass"`).
    pub outcome: String,
    /// Value of the `judgedAgainst` attribute (short git SHA, may be empty).
    pub judged_against: String,
}

/// All data extracted for a single task.
#[derive(Default)]
pub struct TaskData {
    /// Names of predecessor tasks (regular succession edges only — not ordering-only).
    pub deps: Vec<String>,
    /// Current `procedureText` from the task's `verification <name>DoD : Test`.
    pub dod_text: Option<String>,
    /// Test results sorted ascending by sequence number.
    pub results: Vec<ResultRecord>,
    /// Declaration order across the tracking files (backlog priority — D0052).
    /// Lower = higher priority; ready items are ranked by this, not alphabetically.
    pub order: u32,
}

/// Fully extracted project index ready for orient/query computation.
pub struct ExtractedIndex {
    /// All tasks keyed by name, with deps and results attached.
    pub tasks: HashMap<String, TaskData>,
    /// `(pred, succ)` pairs for ordering-only succession edges (used to suppress
    /// false-suspect propagation without blocking ready computation).
    pub ordering_only: HashSet<(String, String)>,
    /// Project state cursor from `state.sysml`.
    pub cursor: crate::Cursor,
}

// ── attribute helpers ─────────────────────────────────────────────────────────

fn str_attr(attrs: &[Attribute], name: &str) -> String {
    attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| match &a.value {
            Value::Str(s) | Value::Ident(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

fn enum_member(attrs: &[Attribute], name: &str) -> String {
    attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| match &a.value {
            Value::EnumLit { member, .. } => Some(member.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

// ── name decomposition ────────────────────────────────────────────────────────

/// Split `<task>DoDR<n>` or `<task>R<n>` → `(task, n)`.
fn decompose_result_name(name: &str) -> Option<(String, u32)> {
    // Try DoDR<n> suffix first (canonical form).
    if let Some(pos) = name.rfind("DoDR") {
        let digits = &name[pos + 4..];
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            if let Ok(n) = digits.parse::<u32>() {
                return Some((name[..pos].to_owned(), n));
            }
        }
    }
    // Fallback: last R<digits> suffix (legacy naming).
    for i in (0..name.len()).rev() {
        if name.as_bytes().get(i) == Some(&b'R') {
            let digits = &name[i + 1..];
            if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
                let n: u32 = digits.parse().ok()?;
                return Some((name[..i].to_owned(), n));
            }
        }
    }
    None
}

/// Strip trailing `DoD` suffix from a verification name → task name.
fn strip_dod_suffix(name: &str) -> Option<String> {
    name.strip_suffix("DoD").map(str::to_owned)
}

// ── extraction ────────────────────────────────────────────────────────────────

fn push_succession(
    first: &str,
    then: &str,
    is_ordering_only: bool,
    edges: &mut Vec<(String, String)>,
    ordering_only: &mut HashSet<(String, String)>,
) {
    if is_ordering_only {
        ordering_only.insert((first.to_owned(), then.to_owned()));
    } else {
        edges.push((first.to_owned(), then.to_owned()));
    }
}

fn try_extract_result(
    name: &str,
    type_name: Option<&str>,
    attrs: &[Attribute],
    raw_results: &mut HashMap<String, Vec<ResultRecord>>,
) {
    if type_name != Some("TestResult") {
        return;
    }
    let Some((task, n)) = decompose_result_name(name) else { return };
    let outcome = enum_member(attrs, "outcome");
    let judged_against = str_attr(attrs, "judgedAgainst");
    raw_results
        .entry(task)
        .or_default()
        .push(ResultRecord { n, outcome, judged_against });
}

/// Ensure `name` exists in `tasks`, assigning it the next declaration-order index
/// (backlog priority, D0052) the first time it is seen.
fn touch_task(tasks: &mut HashMap<String, TaskData>, counter: &mut u32, name: &str) {
    if !tasks.contains_key(name) {
        let order = *counter;
        *counter += 1;
        tasks.insert(name.to_owned(), TaskData { order, ..Default::default() });
    }
}

fn set_dod(tasks: &mut HashMap<String, TaskData>, counter: &mut u32, task: &str, text: String) {
    touch_task(tasks, counter, task);
    if let Some(t) = tasks.get_mut(task) {
        t.dod_text = Some(text);
    }
}

fn extract_items(
    pkg: &Package,
    tasks: &mut HashMap<String, TaskData>,
    counter: &mut u32,
    edges: &mut Vec<(String, String)>,
    ordering_only: &mut HashSet<(String, String)>,
    raw_results: &mut HashMap<String, Vec<ResultRecord>>,
) {
    for item in &pkg.items {
        match item {
            Item::ActionDecl(a) => touch_task(tasks, counter, &a.name),
            Item::Succession(s) => {
                push_succession(&s.first, &s.then, s.is_ordering_only, edges, ordering_only);
            }
            Item::Verification(v) => {
                if let Some(task) = strip_dod_suffix(&v.name) {
                    set_dod(tasks, counter, &task, str_attr(&v.attributes, "procedureText"));
                }
            }
            Item::Part(p) => {
                try_extract_result(&p.name, p.type_name.as_deref(), &p.attributes, raw_results);
            }
            Item::ActionDef(def) => {
                for a in &def.actions {
                    touch_task(tasks, counter, &a.name);
                }
                for s in &def.successions {
                    push_succession(&s.first, &s.then, s.is_ordering_only, edges, ordering_only);
                }
                for v in &def.verifications {
                    if let Some(task) = strip_dod_suffix(&v.name) {
                        set_dod(tasks, counter, &task, str_attr(&v.attributes, "procedureText"));
                    }
                }
                for p in &def.parts {
                    try_extract_result(&p.name, p.type_name.as_deref(), &p.attributes, raw_results);
                }
            }
            _ => {}
        }
    }
}

// ── public entry point ────────────────────────────────────────────────────────

/// Build a [`ExtractedIndex`] from all `.sysml` files under `tracking`.
///
/// Files that fail to lex or parse are silently skipped (same as the text-scan
/// orient — tolerant mode prevents one bad file from breaking the whole index).
#[must_use]
pub fn extract(tracking: &Path) -> ExtractedIndex {
    let files = collect_sysml(tracking);
    let packages: Vec<Package> = files
        .iter()
        .filter_map(|p| {
            let src = std::fs::read_to_string(p).ok()?;
            let fname = p.to_string_lossy();
            let tokens = keel_parser::tokenize(&src, &fname).ok()?;
            keel_parser::parse(tokens, &fname).ok()
        })
        .collect();

    let cursor = packages
        .iter()
        .find_map(crate::parse_cursor)
        .unwrap_or_default();

    let mut tasks: HashMap<String, TaskData> = HashMap::new();
    let mut edges: Vec<(String, String)> = Vec::new();
    let mut ordering_only: HashSet<(String, String)> = HashSet::new();
    let mut raw_results: HashMap<String, Vec<ResultRecord>> = HashMap::new();
    let mut order_counter: u32 = 0;

    for pkg in &packages {
        extract_items(pkg, &mut tasks, &mut order_counter, &mut edges, &mut ordering_only, &mut raw_results);
    }

    // Attach sorted results to known tasks only (no phantom tasks from result names).
    for (name, mut rs) in raw_results {
        rs.sort_by_key(|r| r.n);
        if let Some(task) = tasks.get_mut(&name) {
            task.results = rs;
        }
    }

    // Build deps from regular succession edges.
    for (pred, succ) in &edges {
        if let Some(task) = tasks.get_mut(succ.as_str()) {
            if !task.deps.contains(pred) {
                task.deps.push(pred.clone());
            }
        }
    }

    ExtractedIndex { tasks, ordering_only, cursor }
}
