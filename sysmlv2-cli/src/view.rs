//! `view` subcommand (D0074 migration M1; D0075) — execute a declared TOML viewpoint.
//!
//! A viewpoint is a concise `.engine/views/<name>.view.toml` file: a FILTER over the tracking
//! model — `[select]` (type / attribute / has-missing-edge) → optional `[traverse]` (typed edges,
//! direction, depth, + far-endpoint `target`) → `[project]` (types). The result is the induced
//! subgraph (items + edges), emitted as JSON; presentation is a separate layer (D0075).
//!
//! Fail-loud (D0074): unknown TOML fields and unknown edge kinds are hard errors (no silent
//! misread). M1 scope = AUTHORED attributes + the edges the AST extracts (satisfy / allocate /
//! dependency-markers / succession). COMPUTED attrs (done/ready/governingVersion), `verify`/`:>`
//! edges, temporal predicates are M1b (tracked).

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use serde::Deserialize;
use sysmlv2_parser::ast::{Item, Package, Value};
use sysmlv2_parser::{parse, tokenize};

use crate::json::Json;

#[derive(Debug, thiserror::Error)]
pub enum ViewError {
    #[error("view file not found: {0}")]
    NotFound(String),
    #[error("reading view file {0}: {1}")]
    Io(String, std::io::Error),
    #[error("invalid view TOML {0}: {1}")]
    Toml(String, Box<toml::de::Error>),
    #[error("parsing tracking file {0}: {1}")]
    Track(String, String),
    #[error("view '{view}' references unknown edge kind '{edge}' (known: {known})")]
    UnknownEdge { view: String, edge: String, known: String },
    #[error("unknown render mode '{0}' (expected: graph, table, review)")]
    UnknownMode(String),
    #[error("unknown report '{0}' (expected: assurance, traceability, quality-debt, flow, governance, friction)")]
    UnknownReport(String),
}

// ── the declared view (TOML) ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ViewSpec {
    pub name: String,
    #[serde(default)]
    pub concern: String,
    #[serde(default)]
    pub audience: String,
    pub select: Select,
    pub traverse: Option<Traverse>,
    pub project: Option<Project>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Select {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// A single named item as the seed (overrides type/attrs when set).
    pub item: Option<String>,
    /// Authored-attribute predicates: attr -> a value or a set of values (membership).
    #[serde(default)]
    pub attrs: HashMap<String, AttrPred>,
    /// Keep only items that HAVE an outgoing edge of this kind.
    pub has_edge: Option<String>,
    /// Keep only items that lack an outgoing edge of this kind.
    pub missing_edge: Option<String>,
    /// Match the part's `#Marker` prefix (D0070, M2.0) — a value or a set (e.g. process-change kind).
    pub marker: Option<AttrPred>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AttrPred {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Traverse {
    pub edges: Vec<String>,
    #[serde(default)]
    pub direction: Direction,
    #[serde(default)]
    pub depth: Depth,
    /// Far-endpoint predicate — keep a traversed edge only if its target item matches (ICD-style
    /// boundary, D0075).
    pub target: Option<Select>,
}

#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Down,
    Up,
    #[default]
    Both,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Depth {
    Steps(u32),
    Word(ClosureWord),
}

impl Default for Depth {
    fn default() -> Self {
        Self::Word(ClosureWord::Closure)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ClosureWord {
    Closure,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    #[serde(default)]
    pub types: Vec<String>,
    /// Item attributes to include in the output (e.g. `title`, `status`, `relatedTask`); the
    /// special name `marker` emits the #Marker. Empty = name+type only.
    #[serde(default)]
    pub fields: Vec<String>,
}

// ── the tracking model the view runs over ────────────────────────────────────

struct ItemInfo {
    type_name: String,
    attrs: HashMap<String, String>,
    marker: Option<String>,
}

struct Edge {
    kind: String,
    from: String,
    to: String,
}

struct Model {
    items: HashMap<String, ItemInfo>,
    edges: Vec<Edge>,
}

/// Known edge kinds (canonical, lowercase) the AST currently extracts. View edge names are
/// matched case-insensitively against this set; anything else is a hard error (fail-loud).
const KNOWN_EDGES: &[&str] = &[
    "satisfy",
    "verify",
    "allocate",
    "dependency",
    "ordering",
    "charteredby",
    "resolves",
    "prospectivechange",
    "safetychange",
    "dependson",
    "supersede",
];

fn value_to_string(v: &Value) -> String {
    match v {
        Value::Str(s) | Value::Ident(s) => s.clone(),
        Value::Int(n) => n.to_string(),
        Value::EnumLit { member, .. } => member.clone(),
    }
}

fn edge_kind_from_marker(marker: &str) -> String {
    let m = marker.trim_start_matches('#');
    if m.is_empty() {
        "dependency".to_string()
    } else {
        m.to_lowercase()
    }
}

impl Model {
    fn build(root: &Path) -> Result<Self, ViewError> {
        // Authored instances live in .tracking AND in the .engine instance dirs (decisions,
        // processes, views, skills). Parsing is syntactic (no import resolution), so .engine
        // instance files parse standalone. The schema + workflow DEFS are not instances — skip.
        let dirs = [
            root.join(".tracking"),
            root.join(".engine").join("decisions"),
            root.join(".engine").join("processes"),
            root.join(".engine").join("views"),
            root.join(".engine").join("skills"),
        ];
        let mut items: HashMap<String, ItemInfo> = HashMap::new();
        let mut edges: Vec<Edge> = Vec::new();
        let paths: Vec<_> = dirs.iter().flat_map(|d| crate::collect_sysml(d)).collect();
        for path in paths {
            let name = path.display().to_string();
            let src = std::fs::read_to_string(&path).map_err(|e| ViewError::Io(name.clone(), e))?;
            let tokens = tokenize(&src, &name).map_err(|e| ViewError::Track(name.clone(), e.to_string()))?;
            let pkg = parse(tokens, &name).map_err(|e| ViewError::Track(name.clone(), e.to_string()))?;
            Self::ingest(&pkg, &mut items, &mut edges);
        }
        // `resultof` edges: a TestResult named `<test>R<n>` records a run of Test `<test>` (gate or
        // DoD). The link is by naming convention, not a typed edge — derive it so result leaves
        // connect to their Test (which is itself `contains`-linked to its def).
        let resultofs: Vec<Edge> = items
            .iter()
            .filter(|(_, info)| info.type_name == "TestResult")
            .filter_map(|(name, _)| {
                let test = strip_result_suffix(name)?;
                items.contains_key(test).then(|| Edge { kind: "resultof".to_string(), from: name.clone(), to: test.to_string() })
            })
            .collect();
        edges.extend(resultofs);
        Ok(Self { items, edges })
    }

    fn ingest(pkg: &Package, items: &mut HashMap<String, ItemInfo>, edges: &mut Vec<Edge>) {
        for item in &pkg.items {
            match item {
                Item::Part(p) => add_item(items, &p.name, p.type_name.as_deref(), &p.attributes, p.marker.as_deref()),
                Item::Verification(v) => add_item(items, &v.name, v.type_name.as_deref(), &v.attributes, None),
                Item::ActionDecl(a) => add_item_typed(items, &a.name, "action"),
                Item::ActionDef(ad) => {
                    add_item_typed(items, &ad.name, "ActionDef");
                    // `contains` edges: a def structurally owns its nested parts/verifications/actions.
                    // This containment is real structure the flat item map loses; the diagram draws it
                    // so the nested children connect to their def instead of floating.
                    for p in &ad.parts {
                        add_item(items, &p.name, p.type_name.as_deref(), &p.attributes, p.marker.as_deref());
                        edges.push(Edge { kind: "contains".to_string(), from: ad.name.clone(), to: p.name.clone() });
                    }
                    for v in &ad.verifications {
                        add_item(items, &v.name, v.type_name.as_deref(), &v.attributes, None);
                        edges.push(Edge { kind: "contains".to_string(), from: ad.name.clone(), to: v.name.clone() });
                    }
                    for a in &ad.actions {
                        add_item_typed(items, &a.name, "action");
                        edges.push(Edge { kind: "contains".to_string(), from: ad.name.clone(), to: a.name.clone() });
                    }
                    for s in &ad.successions {
                        let kind = if s.is_ordering_only { "ordering" } else { "succession" };
                        edges.push(Edge { kind: kind.to_string(), from: s.first.clone(), to: s.then.clone() });
                    }
                }
                Item::Satisfy(e) => edges.push(Edge { kind: "satisfy".to_string(), from: e.need.clone(), to: e.by.clone() }),
                Item::Allocate(e) => edges.push(Edge { kind: "allocate".to_string(), from: e.sr.clone(), to: e.to.clone() }),
                Item::Dependency(d) => edges.push(Edge { kind: edge_kind_from_marker(&d.marker), from: d.from.clone(), to: d.to.clone() }),
                Item::Succession(s) => {
                    let kind = if s.is_ordering_only { "ordering" } else { "succession" };
                    edges.push(Edge { kind: kind.to_string(), from: s.first.clone(), to: s.then.clone() });
                }
                Item::Import(_) | Item::TypeDef(_) | Item::EnumDef(_) => {}
            }
        }
        // `contains` for Process -> its ProcessSteps. Steps are authored as siblings of the Process
        // in the same package (not AST-nested), so link by co-membership: every ProcessStep in this
        // package belongs to the Process(es) declared in it.
        let processes: Vec<&str> = pkg
            .items
            .iter()
            .filter_map(|i| match i {
                Item::Part(p) if p.type_name.as_deref() == Some("Process") => Some(p.name.as_str()),
                _ => None,
            })
            .collect();
        if !processes.is_empty() {
            for item in &pkg.items {
                if let Item::Part(p) = item {
                    if p.type_name.as_deref() == Some("ProcessStep") {
                        for proc in &processes {
                            edges.push(Edge { kind: "contains".to_string(), from: (*proc).to_string(), to: p.name.clone() });
                        }
                    }
                }
            }
        }
    }
}

fn add_item(items: &mut HashMap<String, ItemInfo>, name: &str, type_name: Option<&str>, attributes: &[sysmlv2_parser::ast::Attribute], marker: Option<&str>) {
    let attrs = attributes.iter().map(|a| (a.name.clone(), value_to_string(&a.value))).collect();
    items.insert(name.to_string(), ItemInfo { type_name: type_name.unwrap_or("").to_string(), attrs, marker: marker.map(str::to_string) });
}

fn add_item_typed(items: &mut HashMap<String, ItemInfo>, name: &str, type_name: &str) {
    items.entry(name.to_string()).or_insert_with(|| ItemInfo { type_name: type_name.to_string(), attrs: HashMap::new(), marker: None });
}

/// Strip a `R<digits>` result suffix: `storyDiagramRenderFixDoDR1` -> `storyDiagramRenderFixDoD`.
/// Returns `None` when the name does not end in `R` followed by one or more digits.
fn strip_result_suffix(name: &str) -> Option<&str> {
    let idx = name.rfind('R')?;
    let (head, tail) = name.split_at(idx);
    let digits = tail.get(1..)?;
    (!head.is_empty() && !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())).then_some(head)
}

// ── selection + traversal ─────────────────────────────────────────────────────

fn attr_matches(info: &ItemInfo, key: &str, pred: &AttrPred) -> bool {
    info.attrs.get(key).is_some_and(|actual| match pred {
        AttrPred::One(want) => actual == want,
        AttrPred::Many(wants) => wants.iter().any(|w| w == actual),
    })
}

fn has_outgoing(edges: &[Edge], name: &str, kind: &str) -> bool {
    edges.iter().any(|e| e.from == name && e.kind == kind)
}

fn selects(model: &Model, sel: &Select) -> HashSet<String> {
    if let Some(item) = &sel.item {
        return std::iter::once(item.clone()).collect();
    }
    model
        .items
        .iter()
        .filter(|(name, info)| {
            if let Some(t) = &sel.type_ {
                if &info.type_name != t {
                    return false;
                }
            }
            for (k, pred) in &sel.attrs {
                if !attr_matches(info, k, pred) {
                    return false;
                }
            }
            if let Some(pred) = &sel.marker {
                let m = info.marker.as_deref().unwrap_or("");
                let ok = match pred {
                    AttrPred::One(w) => m == w,
                    AttrPred::Many(ws) => ws.iter().any(|w| w == m),
                };
                if !ok {
                    return false;
                }
            }
            if let Some(k) = &sel.has_edge {
                if !has_outgoing(&model.edges, name, &k.to_lowercase()) {
                    return false;
                }
            }
            if let Some(k) = &sel.missing_edge {
                if has_outgoing(&model.edges, name, &k.to_lowercase()) {
                    return false;
                }
            }
            true
        })
        .map(|(name, _)| name.clone())
        .collect()
}

fn validate_edges(view: &str, tr: &Traverse) -> Result<Vec<String>, ViewError> {
    let mut out = Vec::new();
    for e in &tr.edges {
        let lc = e.to_lowercase();
        if !KNOWN_EDGES.contains(&lc.as_str()) {
            return Err(ViewError::UnknownEdge {
                view: view.to_string(),
                edge: e.clone(),
                known: KNOWN_EDGES.join(", "),
            });
        }
        out.push(lc);
    }
    Ok(out)
}

fn traverse(model: &Model, seed: &HashSet<String>, tr: &Traverse, edge_kinds: &[String]) -> HashSet<String> {
    let kinds: HashSet<&str> = edge_kinds.iter().map(String::as_str).collect();
    let target_ok = tr.target.as_ref().map(|t| selects(model, t));
    let max_steps = match tr.depth {
        Depth::Steps(n) => n,
        Depth::Word(ClosureWord::Closure) => u32::MAX,
    };
    let mut reached: HashSet<String> = seed.clone();
    let mut frontier: VecDeque<(String, u32)> = seed.iter().map(|n| (n.clone(), 0)).collect();
    while let Some((node, depth)) = frontier.pop_front() {
        if depth >= max_steps {
            continue;
        }
        for e in &model.edges {
            if !kinds.contains(e.kind.as_str()) {
                continue;
            }
            let down = matches!(tr.direction, Direction::Down | Direction::Both);
            let up = matches!(tr.direction, Direction::Up | Direction::Both);
            let next = if down && e.from == node {
                Some(&e.to)
            } else if up && e.to == node {
                Some(&e.from)
            } else {
                None
            };
            if let Some(n) = next {
                if let Some(ok) = &target_ok {
                    if !ok.contains(n) {
                        continue;
                    }
                }
                if reached.insert(n.clone()) {
                    frontier.push_back((n.clone(), depth + 1));
                }
            }
        }
    }
    reached
}

// ── JSON emit (presentation-agnostic; rendering is a separate layer) ──────────

fn json_esc(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            '\n' => vec!['\\', 'n'],
            other => vec![other],
        })
        .collect()
}

fn emit_json(spec: &ViewSpec, model: &Model, result: &HashSet<String>) -> String {
    let mut names: Vec<&String> = result.iter().collect();
    names.sort();
    let fields: &[String] = spec.project.as_ref().map_or(&[], |p| p.fields.as_slice());
    let items: Vec<String> = names
        .iter()
        .filter_map(|n| {
            model.items.get(*n).map(|info| {
                let base = format!("\"name\": \"{}\", \"type\": \"{}\"", json_esc(n), json_esc(&info.type_name));
                if fields.is_empty() {
                    return format!("    {{{base}}}");
                }
                let rendered: Vec<String> = fields
                    .iter()
                    .filter_map(|f| {
                        let val = if f == "marker" { info.marker.clone() } else { info.attrs.get(f).cloned() };
                        val.map(|v| format!("\"{}\": \"{}\"", json_esc(f), json_esc(&v)))
                    })
                    .collect();
                format!("    {{{base}, \"fields\": {{{}}}}}", rendered.join(", "))
            })
        })
        .collect();
    let edges: Vec<String> = model
        .edges
        .iter()
        .filter(|e| result.contains(&e.from) && result.contains(&e.to))
        .map(|e| {
            format!(
                "    {{\"kind\": \"{}\", \"from\": \"{}\", \"to\": \"{}\"}}",
                json_esc(&e.kind),
                json_esc(&e.from),
                json_esc(&e.to)
            )
        })
        .collect();
    format!(
        "{{\n  \"view\": \"{}\",\n  \"concern\": \"{}\",\n  \"items\": [\n{}\n  ],\n  \"edges\": [\n{}\n  ]\n}}",
        json_esc(&spec.name),
        json_esc(&spec.concern),
        items.join(",\n"),
        edges.join(",\n")
    )
}

/// Load + execute a view; return its subgraph as JSON.
///
/// # Errors
/// Returns [`ViewError`] if the view file is missing or unreadable, the view TOML is invalid
/// (unknown field, bad enum), a tracking/instance file fails to parse, or the view references
/// an unknown edge kind.
pub fn run(root: &Path, view_name: &str) -> Result<String, ViewError> {
    let (spec, model, result) = run_resolved(root, view_name)?;
    Ok(emit_json(&spec, &model, &result))
}

/// Load + execute a view, returning the resolved spec, the full model, and the selected name-set.
/// Shared by [`run`] (JSON emit) and the [`render_html`] table/review/graph-of-view modes.
///
/// # Errors
/// Returns [`ViewError`] if the view file is missing, the TOML is invalid, a tracking/instance file
/// fails to parse, or the view references an unknown edge kind.
fn run_resolved(root: &Path, view_name: &str) -> Result<(ViewSpec, Model, HashSet<String>), ViewError> {
    let path = root.join(".engine").join("views").join(format!("{view_name}.view.toml"));
    if !path.exists() {
        return Err(ViewError::NotFound(path.display().to_string()));
    }
    let pstr = path.display().to_string();
    let text = std::fs::read_to_string(&path).map_err(|e| ViewError::Io(pstr.clone(), e))?;
    let spec: ViewSpec = toml::from_str(&text).map_err(|e| ViewError::Toml(pstr, Box::new(e)))?;

    let model = Model::build(root)?;
    let mut result = selects(&model, &spec.select);
    if let Some(tr) = &spec.traverse {
        let edge_kinds = validate_edges(&spec.name, tr)?;
        result = traverse(&model, &result, tr, &edge_kinds);
    }
    if let Some(proj) = &spec.project {
        if !proj.types.is_empty() {
            let keep: HashSet<&str> = proj.types.iter().map(String::as_str).collect();
            result.retain(|n| model.items.get(n).is_some_and(|i| keep.contains(i.type_name.as_str())));
        }
    }
    Ok((spec, model, result))
}

// ── attestation-coverage (M2.2: first algorithmic view ported from query.py) ─────────────────
// Process-required-attestation coverage (D0066): every status=accepted Decision must carry a
// passing acceptance event (`{dNNNN}AcceptR1 : TestResult, outcome=pass`). Algorithmic (a
// naming + outcome correlation), so a Rust function — not a TOML filter.

fn compute_attestation(model: &Model) -> (usize, Vec<String>) {
    let mut accepted: Vec<&String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Decision" && i.attrs.get("status").map(String::as_str) == Some("accepted"))
        .map(|(n, _)| n)
        .collect();
    accepted.sort();
    let missing: Vec<String> = accepted
        .iter()
        .filter(|d| {
            let ev = format!("{d}AcceptR1");
            model.items.get(&ev).and_then(|i| i.attrs.get("outcome")).map(String::as_str) != Some("pass")
        })
        .map(|d| (*d).clone())
        .collect();
    (accepted.len(), missing)
}

/// Attestation data (D0066): `(total_accepted, missing)`.
///
/// `missing` lists accepted Decisions lacking a passing acceptance event — the structured form
/// behind both the `attestation-coverage` view and the `acceptance-events` guard (M3a).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn attestation_data(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    Ok(compute_attestation(&model))
}

/// Attestation-coverage view (D0066) as JSON: accepted Decisions lacking a passing acceptance event.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn attestation_coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let (total, missing) = compute_attestation(&model);
    let covered = total - missing.len();
    let miss = missing.iter().map(|m| format!("\"{}\"", json_esc(m))).collect::<Vec<_>>().join(", ");
    Ok(format!(
        "{{\n  \"attestation\": \"accepted Decision -> acceptance event (dNNNNAccept, D0066)\",\n  \"total_accepted\": {total},\n  \"covered\": {covered},\n  \"missing\": [{miss}]\n}}"
    ))
}

// ── open-issues (D0077 Issue Resolution Loop) ─────────────────────────────────────────────────
// An Issue is RESOLVED (computed, never stored) iff a #Resolves resolver is COMPLETE — an action
// in `done` OR a Decision with status=accepted; else OPEN. An issue with no #Resolves edge is OPEN
// AND untriaged. `done` is supplied by orient (the single done-set authority).

struct ResolverStatus {
    name: String,
    kind: &'static str, // "action" | "decision"
    complete: bool,
}

struct IssueStatus {
    issue: String,
    resolvers: Vec<ResolverStatus>,
    open: bool,
}

/// The latest recorded disposition verdict on a finding Issue (D0092): the `disposition` attr of a
/// `#Dispositions`-linked confirmation Test (`act` | `acceptRisk` | `dismiss`), or `None` if
/// undispositioned. Reads the TYPED verdict — not a prose/proxy inference.
fn issue_disposition(model: &Model, issue: &str) -> Option<String> {
    model
        .edges
        .iter()
        .filter(|e| e.kind == "dispositions" && e.to == issue)
        .filter_map(|e| model.items.get(&e.from).and_then(|t| t.attrs.get("disposition")).cloned())
        .next_back()
}

fn compute_issue_resolution<S: std::hash::BuildHasher>(model: &Model, done: &HashSet<String, S>) -> Vec<IssueStatus> {
    let mut issues: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Issue").map(|(n, _)| n).collect();
    issues.sort();
    issues
        .into_iter()
        .map(|iss| {
            let mut resolvers: Vec<ResolverStatus> = model
                .edges
                .iter()
                .filter(|e| e.kind == "resolves" && &e.to == iss)
                .map(|e| {
                    let is_decision = model.items.get(&e.from).is_some_and(|i| i.type_name == "Decision");
                    let complete = if is_decision {
                        model.items.get(&e.from).and_then(|i| i.attrs.get("status")).map(String::as_str) == Some("accepted")
                    } else {
                        done.contains(e.from.as_str())
                    };
                    ResolverStatus { name: e.from.clone(), kind: if is_decision { "decision" } else { "action" }, complete }
                })
                .collect();
            // D0092: an ACCEPT-RISK or DISMISS disposition CLOSES the issue on its own (the verdict IS
            // the resolution); ACT does not — it still needs its #Resolves resolver done.
            if let Some(v) = issue_disposition(model, iss) {
                if v == "acceptRisk" || v == "dismiss" {
                    resolvers.push(ResolverStatus { name: format!("disposition:{v}"), kind: "disposition", complete: true });
                }
            }
            resolvers.sort_by(|a, b| a.name.cmp(&b.name));
            let open = !resolvers.iter().any(|r| r.complete);
            IssueStatus { issue: iss.clone(), resolvers, open }
        })
        .collect()
}

/// Names of OPEN issues (no complete `#Resolves` resolver), sorted. Used by orient to surface
/// `open_issues`. `done` is orient's done-set.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn open_issue_names<S: std::hash::BuildHasher>(root: &Path, done: &HashSet<String, S>) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    Ok(compute_issue_resolution(&model, done).into_iter().filter(|i| i.open).map(|i| i.issue).collect())
}

/// `(total_issues, untriaged)` — issues with NO `#Resolves` edge at all (D0077). Pure structure
/// (no done-set needed); the `issues` guard fails on a non-empty untriaged list.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn untriaged_issues(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    let issues: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Issue").map(|(n, _)| n).collect();
    let mut untriaged: Vec<String> = issues
        .iter()
        .filter(|n| !model.edges.iter().any(|e| e.kind == "resolves" && &e.to == **n))
        .map(|n| (*n).clone())
        .collect();
    untriaged.sort();
    Ok((issues.len(), untriaged))
}

/// Open-issues view (D0077) as JSON: every OPEN issue + its resolvers + completeness, with counts.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn open_issues(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let done = crate::orient::done_names(root);
    let all = compute_issue_resolution(&model, &done);
    let total = all.len();
    let open_count = all.iter().filter(|i| i.open).count();
    let open_list: Vec<Json> = all
        .iter()
        .filter(|i| i.open)
        .map(|i| {
            let resolvers: Vec<Json> = i
                .resolvers
                .iter()
                .map(|r| {
                    Json::Obj(vec![
                        ("name".to_string(), Json::s(r.name.clone())),
                        ("kind".to_string(), Json::s(r.kind)),
                        ("complete".to_string(), Json::Bool(r.complete)),
                    ])
                })
                .collect();
            Json::Obj(vec![
                ("issue".to_string(), Json::s(i.issue.clone())),
                ("untriaged".to_string(), Json::Bool(i.resolvers.is_empty())),
                ("resolvers".to_string(), Json::Arr(resolvers)),
            ])
        })
        .collect();
    let out = Json::Obj(vec![
        ("total_issues".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
        ("open".to_string(), Json::Int(i64::try_from(open_count).unwrap_or(i64::MAX))),
        ("resolved".to_string(), Json::Int(i64::try_from(total - open_count).unwrap_or(i64::MAX))),
        ("open_issues".to_string(), Json::Arr(open_list)),
    ]);
    Ok(out.dump())
}

/// Concern-coverage view (D0057/issue035): which declared stakeholder concerns (Viewpoints) are
/// SERVED by a real computed renderer, and which are still unserved (renderer `(planned ...)`).
///
/// d0057 delivered the Viewpoint registry but its promised payoff — an audit of which concerns lack
/// a working viewpoint — was never built. This is that audit, as a VIEW (not a guard): a `(planned)`
/// viewpoint is a legitimately-deferred concern, not a violation, so it is reported, not failed.
/// `served` = renderer names a `sysmlv2` command; `unserved` = renderer is `(planned ...)`.
///
/// # Errors
/// Returns `ViewError::Io` if the viewpoint registry cannot be read.
pub fn concern_coverage(root: &Path) -> Result<String, ViewError> {
    let path = root.join(".engine").join("views").join("viewpoint-registry.sysml");
    let text = std::fs::read_to_string(&path).map_err(|e| ViewError::Io(path.display().to_string(), e))?;
    let quoted = |line: &str, key: &str| -> Option<String> {
        let needle = format!(":>> {key} = \"");
        line.split(needle.as_str()).nth(1)?.split('"').next().map(str::to_string)
    };
    let (mut title, mut concern) = (String::new(), String::new());
    let mut served: Vec<(String, String, String)> = Vec::new();
    let mut unserved: Vec<(String, String, String)> = Vec::new();
    for line in text.lines() {
        let t = line.trim_start();
        if let Some(v) = quoted(t, "title") {
            title = v;
        } else if let Some(v) = quoted(t, "concernText") {
            concern = v;
        } else if let Some(r) = quoted(t, "renderer") {
            let row = (title.clone(), concern.clone(), r.clone());
            if r.starts_with("(planned") {
                unserved.push(row);
            } else {
                served.push(row);
            }
        }
    }
    let total = served.len() + unserved.len();
    let to_json = |rows: &[(String, String, String)]| -> Vec<Json> {
        rows.iter()
            .map(|(t, c, r)| {
                Json::Obj(vec![
                    ("viewpoint".to_string(), Json::s(t.clone())),
                    ("concern".to_string(), Json::s(c.clone())),
                    ("renderer".to_string(), Json::s(r.clone())),
                ])
            })
            .collect()
    };
    let out = Json::Obj(vec![
        ("total_concerns".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
        ("served".to_string(), Json::Int(i64::try_from(served.len()).unwrap_or(i64::MAX))),
        ("unserved".to_string(), Json::Int(i64::try_from(unserved.len()).unwrap_or(i64::MAX))),
        ("coverage_pct".to_string(), Json::s(format!("{}", pct(served.len(), total)))),
        ("unserved_concerns".to_string(), Json::Arr(to_json(&unserved))),
        ("served_concerns".to_string(), Json::Arr(to_json(&served))),
    ]);
    Ok(out.dump())
}

/// Dispositions view (D0092): every >= Medium finding + its typed disposition verdict.
///
/// Each verdict is `act`/`acceptRisk`/`dismiss` or `undispositioned` — the computed read of the
/// human-judgment gate (reads the typed verdict, not prose/proxy). `undispositioned` is what `assured`
/// enforces.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn dispositions(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut findings: Vec<(&String, &ItemInfo)> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Issue" && i.attrs.get("severity").is_some_and(|s| at_least_medium(s)))
        .collect();
    findings.sort_by(|a, b| a.0.cmp(b.0));
    let mut undisp = 0usize;
    let rows: Vec<Json> = findings
        .iter()
        .map(|(name, info)| {
            let verdict = issue_disposition(&model, name);
            if verdict.is_none() {
                undisp += 1;
            }
            Json::Obj(vec![
                ("finding".to_string(), Json::s((*name).clone())),
                ("severity".to_string(), Json::s(info.attrs.get("severity").cloned().unwrap_or_default())),
                ("dispositioned".to_string(), Json::Bool(verdict.is_some())),
                ("disposition".to_string(), verdict.map_or_else(|| Json::s("undispositioned".to_string()), Json::s)),
            ])
        })
        .collect();
    let total = rows.len();
    let out = Json::Obj(vec![
        ("ge_medium_findings".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
        ("dispositioned".to_string(), Json::Int(i64::try_from(total - undisp).unwrap_or(i64::MAX))),
        ("undispositioned".to_string(), Json::Int(i64::try_from(undisp).unwrap_or(i64::MAX))),
        ("findings".to_string(), Json::Arr(rows)),
    ]);
    Ok(out.dump())
}

/// The set of sprint Story names covered by a `#Covers` edge (review -> sprint). Pure (for self-test).
fn covered_sprints(model: &Model) -> HashSet<&str> {
    model.edges.iter().filter(|e| e.kind == "covers").map(|e| e.to.as_str()).collect()
}

/// Sitting-coverage view (D0049/D0092 issue040): which delivery sprints are covered by a review.
///
/// A "sitting review" attests its sprints via `#Covers` edges (review -> sprint `Story`); a sprint is
/// covered iff some `#Covers` edge points to it. Makes the previously-unmodeled "sitting" UNIT
/// computable (the human gate's coverage). A VIEW, not a gate — the human reviews per sitting at their
/// own cadence (batchable, D0019); an uncovered sprint is surfaced, not blocked.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn sitting_coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut sprints: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Story").map(|(n, _)| n).collect();
    sprints.sort();
    let covered: HashSet<&str> = covered_sprints(&model);
    let uncovered: Vec<Json> = sprints.iter().filter(|s| !covered.contains(s.as_str())).map(|s| Json::s((*s).clone())).collect();
    // Each per-sitting review (a source of #Covers edges) + the sprints it attests.
    let mut review_names: Vec<&String> = model.edges.iter().filter(|e| e.kind == "covers").map(|e| &e.from).collect();
    review_names.sort_unstable();
    review_names.dedup();
    let reviews: Vec<Json> = review_names
        .iter()
        .map(|r| {
            let mut covers: Vec<String> = model.edges.iter().filter(|e| e.kind == "covers" && &e.from == *r).map(|e| e.to.clone()).collect();
            covers.sort();
            Json::Obj(vec![("review".to_string(), Json::s((*r).clone())), ("covers".to_string(), Json::Arr(covers.into_iter().map(Json::s).collect()))])
        })
        .collect();
    let total = sprints.len();
    let uncovered_n = uncovered.len();
    let out = Json::Obj(vec![
        ("sprints".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
        ("covered".to_string(), Json::Int(i64::try_from(total - uncovered_n).unwrap_or(i64::MAX))),
        ("uncovered".to_string(), Json::Int(i64::try_from(uncovered_n).unwrap_or(i64::MAX))),
        ("sitting_reviews".to_string(), Json::Arr(reviews)),
        ("uncovered_sprints".to_string(), Json::Arr(uncovered)),
    ]);
    Ok(out.dump())
}

/// True if an action name looks like it produces a permanent automated GUARD/check (D0047/issue039).
/// Heuristic (this feeds a WARN diagnostic, not a hard gate): the resolver naming convention in use.
fn is_guard_producing(name: &str) -> bool {
    let n = name.to_lowercase();
    ["guard", "check", "rule", "audit", "lint", "validat"].iter().any(|k| n.contains(k))
}

/// Defect-guard-coverage diagnostic (D0047/issue039): every `#ProcessDefect` finding must resolve to
/// a guard-producing action.
///
/// The meta-audit that "corrections become guards" is actually followed (previously enforced only by
/// vigilance). Returns `(examined, warnings)` — each warning is a process-defect whose `#Resolves`
/// resolver is not guard-producing.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn defect_guard_coverage(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    let mut defects: Vec<&String> = model.edges.iter().filter(|e| e.kind == "processdefect").map(|e| &e.from).collect();
    defects.sort_unstable();
    defects.dedup();
    let mut warns = Vec::new();
    for d in &defects {
        let resolvers: Vec<&String> = model.edges.iter().filter(|e| e.kind == "resolves" && &e.to == *d).map(|e| &e.from).collect();
        if !resolvers.iter().any(|r| is_guard_producing(r)) {
            let names = if resolvers.is_empty() { "none".to_string() } else { resolvers.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ") };
            warns.push(format!("{d}: #ProcessDefect finding (D0047) with no guard-producing resolver (resolver(s): {names}) — a recurrable process defect must become a permanent automated guard"));
        }
    }
    Ok((defects.len(), warns))
}

// ── assurance coverage (D0079 C — the computed-state.md coverageState/satisfaction/gaps view) ──
// For each Need / SystemRequirement / Decision: is there COMPLETE + PASSING + NON-STALE evidence
// it has been addressed? Verifier kinds, strongest first:
//   - explicit-test — a Test `verify`-linked to the target (computed-state.md V&V chain; 0 today
//     because no `verify` edges are authored yet — that absence is itself the headline gap).
//   - charter-dod   — a work item `#CharteredBy` the target whose DoD is `done` (orient) and not
//     `suspect`. The charter `from` is a Story; it maps to the done/suspect set via two name forms
//     (`<base>` backlog action, `story<Base>` delivery action) — verified to cover every charter.
//   - satisfy       — (Need only) its `satisfy`-linked SystemRequirement is itself covered.
// Honest by construction: a target with no complete verifier is `uncovered` — coverage is never
// fabricated. State ∈ {covered, suspect, uncovered}; `basis` names the strongest covering kind.

const ASSURANCE_TYPES: [&str; 3] = ["Need", "SystemRequirement", "Decision"];

struct Verifier {
    name: String,
    kind: &'static str, // "explicit-test" | "charter-dod" | "satisfy"
    complete: bool,
    suspect: bool,
}

struct Coverage {
    element: String,
    type_name: String,
    tier: &'static str,          // D0082: verified | attested | addressed | suspect | uncovered
    basis: Option<&'static str>, // the strongest covering verifier kind
    verifiers: Vec<Verifier>,
}

/// Candidate done/suspect-set name forms for a charter `from` (a Story). `<X>Story` maps to the
/// backlog action `<X>` and the delivery action `story<X>`; the raw name is kept as a fallback.
fn charter_forms(from: &str) -> Vec<String> {
    let base = from.strip_suffix("Story").unwrap_or(from);
    let mut forms = vec![from.to_string(), base.to_string()];
    if let Some(c) = base.chars().next() {
        forms.push(format!("story{}{}", c.to_uppercase(), &base[c.len_utf8()..]));
    }
    forms
}

/// The coverage TIER a verifier kind confers (D0082): objective evidence vs attestation vs a
/// mere claim. `satisfy` is transitive — it is only added as a verifier when the satisfied
/// requirement is itself verified, so it legitimately confers `verified`.
fn tier_for_kind(kind: &str) -> &'static str {
    match kind {
        "explicit-test" | "satisfy" => "verified",
        "accept-event" => "attested",
        _ => "addressed", // charter-dod: work was done, not evidence the element holds
    }
}

/// Coverage TIER + basis from a verifier set (D0082 three-tier model): the STRONGEST complete,
/// non-stale verifier wins — `verified` (reproducible evidence) > `attested` (human confirmation,
/// where judgment isn't testable) > `addressed` (work/trace only — a claim, not evidence). If the
/// only complete evidence is a stale verify-Test → `suspect`; nothing → `uncovered`.
/// A tier the GATE accepts as covered (D0082): objective evidence or a defensible attestation.
/// `addressed` (claim only), `suspect` (stale), and `uncovered` are gaps.
fn is_covered_tier(tier: &str) -> bool {
    matches!(tier, "verified" | "attested")
}

/// Gate-covered % over `cov`, optionally restricted to type `ty` (empty = all): the fraction whose
/// tier is gate-covered (verified|attested). The single coverage-ratio formula (D0090) — `metric_value`
/// AND the report scalar cards both source from here, so the number is computed in exactly one place.
fn coverage_pct_of(cov: &[Coverage], ty: &str) -> u32 {
    let rows: Vec<&Coverage> = cov.iter().filter(|c| ty.is_empty() || c.type_name == ty).collect();
    pct(rows.iter().filter(|c| is_covered_tier(c.tier)).count(), rows.len())
}

/// Verified % over `cov` restricted to type `ty` (empty = all): the fraction at the strongest
/// (`verified`) tier — V&V traceability. Shared by `metric_value` (`req_verified_pct`/
/// `needs_verified_pct`) and the traceability scorecard (D0090; single-source).
fn verified_pct_of(cov: &[Coverage], ty: &str) -> u32 {
    let rows: Vec<&Coverage> = cov.iter().filter(|c| ty.is_empty() || c.type_name == ty).collect();
    pct(rows.iter().filter(|c| c.tier == "verified").count(), rows.len())
}

fn tier_of(verifiers: &[Verifier]) -> (&'static str, Option<&'static str>) {
    for want in ["verified", "attested", "addressed"] {
        if let Some(v) = verifiers.iter().find(|v| v.complete && !v.suspect && tier_for_kind(v.kind) == want) {
            return (want, Some(v.kind));
        }
    }
    if verifiers.iter().any(|v| v.complete && v.suspect) {
        return ("suspect", None);
    }
    ("uncovered", None)
}

// ── element-content staleness (D0084 — targeted suspicion: re-verify/re-critique on element change) ──
// A verify/critique of an assurance element goes SUSPECT when the element's SEMANTIC field changed
// since the verification's latest result commit AND the element existed then (so same-sprint
// create+verify isn't falsely flagged). Reuses orient's batched `git cat-file`. Honors D0005's
// material-change intent at the element grain that coverage/critique actually depend on.

/// The semantic field whose change should re-suspect verification of this element type.
fn semantic_field(type_name: &str) -> Option<&'static str> {
    match type_name {
        "Need" | "SystemRequirement" => Some("statement"),
        "Decision" => Some("decision"),
        _ => None,
    }
}

/// `(outcome, judgedAgainst)` of the HIGHEST-numbered `<v>R<n>` result for verification `v`.
fn latest_result(model: &Model, v: &str) -> Option<(String, String)> {
    let mut best: Option<(u32, String, String)> = None;
    for (name, info) in &model.items {
        let Some(suf) = name.strip_prefix(v) else { continue };
        let Some(digits) = suf.strip_prefix('R') else { continue };
        if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let Ok(n) = digits.parse::<u32>() else { continue };
        if best.as_ref().is_none_or(|(bn, _, _)| n > *bn) {
            let outcome = info.attrs.get("outcome").cloned().unwrap_or_default();
            let sha = info.attrs.get("judgedAgainst").cloned().unwrap_or_default();
            best = Some((n, outcome, sha));
        }
    }
    best.map(|(_, o, s)| (o, s))
}

/// Map each assurance element (`requirement <n/sr>` / `part <d> : Decision`) to its repo-relative
/// file (one working-tree pass, no git) — to fetch its historical content for staleness.
fn build_element_files(root: &Path) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    let dirs = [root.join(".tracking"), root.join(".engine").join("decisions")];
    for path in dirs.iter().flat_map(|d| crate::collect_sysml(d)) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Some(rel) = path.strip_prefix(root).ok().and_then(std::path::Path::to_str).map(|s| s.replace('\\', "/")) else {
            continue;
        };
        for line in text.lines() {
            let t = line.trim_start();
            let name = t.strip_prefix("requirement ").or_else(|| t.strip_prefix("part ")).and_then(|r| r.split([' ', ':']).next());
            if let Some(name) = name {
                if !name.is_empty() {
                    out.entry(name.to_owned()).or_insert_with(|| rel.clone());
                }
            }
        }
    }
    out
}

/// Extract `element`'s `:>> <field> = "..."` value from a file blob (the FIRST occurrence inside the
/// element's block). `None` if the element/field isn't present (e.g. it didn't exist at that commit).
fn extract_field(blob: &str, element: &str, field: &str) -> Option<String> {
    let decl_r = format!("requirement {element} ");
    let decl_p = format!("part {element} ");
    let fieldpat = format!(":>> {field} = \"");
    let mut in_elem = false;
    for line in blob.lines() {
        let t = line.trim_start();
        if !in_elem {
            if t.starts_with(&decl_r) || t.starts_with(&decl_p) {
                in_elem = true;
            }
            continue;
        }
        if t.starts_with("part ") || t.starts_with("requirement ") || t.starts_with("verification ") || t.starts_with("action ") {
            break; // next top-level item — left the element's block without finding the field
        }
        if let Some(idx) = t.find(&fieldpat) {
            let rest = t.get(idx + fieldpat.len()..)?;
            let end = rest.find('"')?;
            return rest.get(..end).map(str::to_owned);
        }
    }
    None
}

/// Names of verify/critique Tests that are STALE: their target assurance element's semantic field
/// changed since the Test's latest result commit, and the element existed at that commit (D0084).
fn compute_stale_verifications(root: &Path, model: &Model) -> HashSet<String> {
    let elem_files = build_element_files(root);
    let mut work: Vec<(String, String, &'static str, String)> = Vec::new(); // (test, element, field, sha)
    let mut keys: HashSet<String> = HashSet::new();
    for e in model.edges.iter().filter(|e| e.kind == "verify") {
        let Some(info) = model.items.get(&e.to) else { continue };
        if info.type_name == "Decision" && info.attrs.get("status").map(String::as_str) != Some("accepted") {
            continue;
        }
        let Some(field) = semantic_field(&info.type_name) else { continue };
        let Some((_, sha)) = latest_result(model, &e.from) else { continue };
        if sha.is_empty() {
            continue;
        }
        let Some(rel) = elem_files.get(&e.to) else { continue };
        keys.insert(format!("{sha}:{rel}"));
        work.push((e.from.clone(), e.to.clone(), field, sha));
    }
    let blobs = crate::orient::batch_cat_blobs(root, &keys.into_iter().collect::<Vec<_>>());
    let mut stale: HashSet<String> = HashSet::new();
    for (test, element, field, sha) in work {
        let Some(rel) = elem_files.get(&element) else { continue };
        let Some(Some(blob)) = blobs.get(&format!("{sha}:{rel}")) else { continue }; // file absent at sha
        let Some(old) = extract_field(blob, &element, field) else { continue }; // element absent then -> not stale
        let cur = model.items.get(&element).and_then(|i| i.attrs.get(field)).map_or("", String::as_str);
        if old != cur {
            stale.insert(test);
        }
    }
    stale
}

/// Direct verifiers of `target`, strongest first: explicit-test edge → accept-event (Decisions
/// only) → charter-dod. Needs add a transitive `satisfy` verifier separately (it depends on
/// requirement coverage computed first). `stale` (D0084) marks verify-edge Tests whose target's
/// content drifted since they were judged.
fn direct_verifiers<S: std::hash::BuildHasher>(
    model: &Model,
    target: &str,
    is_decision: bool,
    done: &HashSet<String, S>,
    task_suspect: &HashSet<String, S>,
    stale: &HashSet<String, S>,
) -> Vec<Verifier> {
    let mut vs: Vec<Verifier> = Vec::new();
    for e in model.edges.iter().filter(|e| e.kind == "verify" && e.to == target) {
        // A verify-edge Test is complete iff its LATEST TestResult passed — read from the Model
        // (a standalone `verification`/`part <name>R<n> : TestResult`), NOT the action-task `done`
        // set (these Tests aren't action DoDs). Mirrors accept-event + critique-coverage.
        // EXCLUDE method=critique Tests: critiques are also #Verify-linked but belong to
        // critique-coverage (an adversarial lens), not objective assurance coverage (D0082).
        let src = model.items.get(&e.from);
        if src.and_then(|i| i.attrs.get("method")).map(String::as_str) == Some("critique") {
            continue;
        }
        let pass = latest_result(model, &e.from).is_some_and(|(o, _)| o == "pass");
        vs.push(Verifier {
            complete: pass,
            suspect: stale.contains(&e.from), // D0084: element-content drift since the verifying commit
            name: e.from.clone(),
            kind: "explicit-test",
        });
    }
    // A Decision's canonical assurance is its recorded human acceptance event (D0066): a passing
    // `<decision>AcceptR1` TestResult. (Attestation-staleness is a future refinement.)
    if is_decision {
        let ev = format!("{target}AcceptR1");
        if model.items.get(&ev).and_then(|i| i.attrs.get("outcome")).map(String::as_str) == Some("pass") {
            vs.push(Verifier { name: ev, kind: "accept-event", complete: true, suspect: false });
        }
    }
    for e in model.edges.iter().filter(|e| e.kind == "charteredby" && e.to == target) {
        let forms = charter_forms(&e.from);
        vs.push(Verifier {
            complete: forms.iter().any(|f| done.contains(f)),
            suspect: forms.iter().any(|f| task_suspect.contains(f)),
            name: e.from.clone(),
            kind: "charter-dod",
        });
    }
    vs
}

fn compute_coverage<S: std::hash::BuildHasher>(
    model: &Model,
    done: &HashSet<String, S>,
    task_suspect: &HashSet<String, S>,
    stale: &HashSet<String, S>,
) -> Vec<Coverage> {
    // Pass 1: requirements + decisions (their coverage is direct).
    let mut req_tier: HashMap<String, &'static str> = HashMap::new();
    let mut out: Vec<Coverage> = Vec::new();
    // Assurance targets: Needs, SystemRequirements, and ACCEPTED Decisions only. Superseded /
    // rejected / proposed Decisions are not active commitments (mirrors the attestation guard's
    // accepted-only scope, D0066) — including them would report false gaps.
    let is_target = |i: &ItemInfo| match i.type_name.as_str() {
        "Need" | "SystemRequirement" => true,
        "Decision" => i.attrs.get("status").map(String::as_str) == Some("accepted"),
        _ => false,
    };
    let mut targets: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| is_target(i)).collect();
    targets.sort_by(|a, b| a.0.cmp(b.0));
    for (name, info) in &targets {
        if info.type_name == "Need" {
            continue; // pass 2
        }
        let verifiers = direct_verifiers(model, name, info.type_name == "Decision", done, task_suspect, stale);
        let (tier, basis) = tier_of(&verifiers);
        if info.type_name == "SystemRequirement" {
            req_tier.insert((*name).clone(), tier);
        }
        out.push(Coverage { element: (*name).clone(), type_name: info.type_name.clone(), tier, basis, verifiers });
    }
    // Pass 2: needs — direct verifiers plus a transitive `satisfy` verifier that confers `verified`
    // ONLY when the satisfied requirement is itself verified (transitive satisfaction, the contract).
    for (name, info) in &targets {
        if info.type_name != "Need" {
            continue;
        }
        let mut verifiers = direct_verifiers(model, name, false, done, task_suspect, stale);
        for e in model.edges.iter().filter(|e| e.kind == "satisfy" && &e.from == *name) {
            let req_verified = req_tier.get(&e.to).copied() == Some("verified");
            verifiers.push(Verifier { name: e.to.clone(), kind: "satisfy", complete: req_verified, suspect: false });
        }
        let (tier, basis) = tier_of(&verifiers);
        out.push(Coverage { element: (*name).clone(), type_name: info.type_name.clone(), tier, basis, verifiers });
    }
    out.sort_by(|a, b| (a.type_name.clone(), a.element.clone()).cmp(&(b.type_name.clone(), b.element.clone())));
    out
}

/// Assurance-coverage view (D0079 C) as JSON.
///
/// Emits per-element coverage state + basis + verifiers, a per-type summary, and the flat gap set
/// (uncovered + suspect). Reuses the orient done/suspect authorities — never stores a verdict.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let done = crate::orient::done_names(root);
    let task_suspect: HashSet<String> = crate::orient::compute(root).suspect.into_iter().collect();
    let stale = compute_stale_verifications(root, &model);
    let cov = compute_coverage(&model, &done, &task_suspect, &stale);
    let gf = crate::govern::grandfathered_under(root, COVERAGE_DECISION);

    // Per-type summary, counted by TIER (D0082).
    let mut summary: Vec<Json> = Vec::new();
    for ty in ASSURANCE_TYPES {
        let rows: Vec<&Coverage> = cov.iter().filter(|c| c.type_name == ty).collect();
        if rows.is_empty() {
            continue;
        }
        let count = |t: &str| i64::try_from(rows.iter().filter(|c| c.tier == t).count()).unwrap_or(i64::MAX);
        summary.push(Json::Obj(vec![
            ("type".to_string(), Json::s(ty)),
            ("total".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX))),
            ("verified".to_string(), Json::Int(count("verified"))),
            ("attested".to_string(), Json::Int(count("attested"))),
            ("addressed".to_string(), Json::Int(count("addressed"))),
            ("suspect".to_string(), Json::Int(count("suspect"))),
            ("uncovered".to_string(), Json::Int(count("uncovered"))),
        ]));
    }

    let elements: Vec<Json> = cov
        .iter()
        .map(|c| {
            let verifiers: Vec<Json> = c
                .verifiers
                .iter()
                .map(|v| {
                    Json::Obj(vec![
                        ("name".to_string(), Json::s(v.name.clone())),
                        ("kind".to_string(), Json::s(v.kind)),
                        ("complete".to_string(), Json::Bool(v.complete)),
                        ("suspect".to_string(), Json::Bool(v.suspect)),
                    ])
                })
                .collect();
            Json::Obj(vec![
                ("element".to_string(), Json::s(c.element.clone())),
                ("type".to_string(), Json::s(c.type_name.clone())),
                ("tier".to_string(), Json::s(c.tier)),
                ("basis".to_string(), c.basis.map_or(Json::Null, Json::s)),
                ("governed".to_string(), Json::Bool(governed(gf.as_ref(), &c.element))),
                ("verifiers".to_string(), Json::Arr(verifiers)),
            ])
        })
        .collect();

    // A tier counts as covered for the GATE iff verified or attested (D0082). addressed/suspect/
    // uncovered are gaps. Full gap set (honest measurement) + the GOVERNED subset the gate uses.
    let gaps: Vec<Json> =
        cov.iter().filter(|c| !is_covered_tier(c.tier)).map(|c| Json::s(c.element.clone())).collect();
    let governed_gaps: Vec<Json> =
        cov.iter().filter(|c| !is_covered_tier(c.tier) && governed(gf.as_ref(), &c.element)).map(|c| Json::s(c.element.clone())).collect();

    let out = Json::Obj(vec![
        (
            "assurance".to_string(),
            Json::s("coverage tiers (D0082): verified (reproducible verify-edge evidence; needs transitively via a verified requirement) > attested (decision acceptance event) > addressed (charter-dod work only — a claim) > uncovered. Gate-covered = verified|attested. (D0079 C)"),
        ),
        ("summary".to_string(), Json::Arr(summary)),
        ("gaps".to_string(), Json::Arr(gaps)),
        ("governed_gaps".to_string(), Json::Arr(governed_gaps)),
        ("elements".to_string(), Json::Arr(elements)),
    ]);
    Ok(out.dump())
}

// ── critique coverage (D0080/D0079 — per-element x required-lens critique coverage) ────────────
// An antagonistic critique is a `method=critique` Test with a `lens`, `#Verify`-linked to its
// target (parsed as a "verify" marker-edge), with a result by an INDEPENDENT critic (the result's
// judgedBy must differ from the target's createdBy). An element is critique-COVERED iff every
// REQUIRED lens for its type has such a critique. Required-lens policy (Core-3, human-accepted):
// Need/SystemRequirement -> completeness/correctness/testability; Decision -> completeness/
// correctness/feasibility. Honest by construction: with no critiques recorded, every element is
// uncovered. (Full git-temporal critique-staleness reuses the suspect machinery — a later step.)

/// Required critique lenses per assurance-element type (Core-3, D0080). Empty for non-targets.
fn required_lenses(type_name: &str) -> &'static [&'static str] {
    match type_name {
        "Need" | "SystemRequirement" => &["completeness", "correctness", "testability"],
        "Decision" => &["completeness", "correctness", "feasibility"],
        _ => &[],
    }
}

struct LensStatus {
    lens: &'static str,
    critiqued: bool,
    critic: Option<String>,  // result judgedBy (independent of the target author)
    outcome: Option<String>, // pass = survived the lens; fail = a finding was raised
}

struct CritiqueCoverage {
    element: String,
    type_name: String,
    lenses: Vec<LensStatus>,
    covered: bool, // every required lens critiqued
}

fn compute_critique_coverage<S: std::hash::BuildHasher>(model: &Model, stale: &HashSet<String, S>) -> Vec<CritiqueCoverage> {
    // Same target scope as assurance coverage: Needs, SystemRequirements, accepted Decisions.
    let is_target = |i: &ItemInfo| match i.type_name.as_str() {
        "Need" | "SystemRequirement" => true,
        "Decision" => i.attrs.get("status").map(String::as_str) == Some("accepted"),
        _ => false,
    };
    let mut targets: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| is_target(i)).collect();
    targets.sort_by(|a, b| a.0.cmp(b.0));
    targets
        .into_iter()
        .map(|(name, info)| {
            let author = info.attrs.get("createdBy").map_or("", String::as_str);
            let lenses: Vec<LensStatus> = required_lenses(&info.type_name)
                .iter()
                .map(|&lens| {
                    // A critique of this element via this lens: a verify-edge (critique -> element)
                    // whose source is a method=critique Test with this lens, having an independent result.
                    let mut critiqued = false;
                    let mut critic = None;
                    let mut outcome = None;
                    for e in model.edges.iter().filter(|e| e.kind == "verify" && e.to == *name) {
                        let Some(c) = model.items.get(&e.from) else { continue };
                        if c.attrs.get("method").map(String::as_str) != Some("critique") {
                            continue;
                        }
                        if c.attrs.get("lens").map(String::as_str) != Some(lens) {
                            continue;
                        }
                        // D0084: a critique whose target's content drifted since it ran is STALE —
                        // it no longer covers the lens (re-critique needed).
                        if stale.contains(&e.from) {
                            continue;
                        }
                        let res = model.items.get(&format!("{}R1", e.from));
                        let by = res.and_then(|r| r.attrs.get("judgedBy")).map(String::as_str);
                        let out = res.and_then(|r| r.attrs.get("outcome")).map(String::as_str);
                        // Independence: the critic must differ from the target's author.
                        if let (Some(by), Some(out)) = (by, out) {
                            if by != author {
                                critiqued = true;
                                critic = Some(by.to_string());
                                outcome = Some(out.to_string());
                                break;
                            }
                        }
                    }
                    LensStatus { lens, critiqued, critic, outcome }
                })
                .collect();
            let covered = !lenses.is_empty() && lenses.iter().all(|l| l.critiqued);
            CritiqueCoverage { element: name.clone(), type_name: info.type_name.clone(), lenses, covered }
        })
        .collect()
}

// Charter-time governance (D0068/D0081): the assurance requirements are PROSPECTIVE — they bind only
// elements created after the governing decision landed. coverage(C) is governed by D0079; the
// critique requirement by D0080. Pre-decision elements are grandfathered (out of the GATE's gap set,
// though still shown in the VIEW with `governed=false` for transparency).
const COVERAGE_DECISION: &str = "d0079";
const CRITIQUE_DECISION: &str = "d0080";

/// Whether `name` is GOVERNED (in scope) given a grandfather set: in scope iff present and not
/// grandfathered. A `None` set (git unavailable) yields `false` — conservative: the gate never
/// spuriously blocks when charter history can't be read.
fn governed(grandfathered: Option<&HashSet<String>>, name: &str) -> bool {
    grandfathered.is_some_and(|gf| !gf.contains(name))
}

/// Names of GOVERNED elements (created after D0080) missing >= 1 required-lens critique — the
/// `guard critique` gap set (charter-time scoped, D0081), sorted.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_gaps(root: &Path) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    let gf = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let stale = compute_stale_verifications(root, &model);
    Ok(compute_critique_coverage(&model, &stale)
        .into_iter()
        .filter(|c| !c.covered && governed(gf.as_ref(), &c.element))
        .map(|c| c.element)
        .collect())
}

/// Elements rendered SUSPECT by an unresolved failing critique (D0086).
///
/// An element with a `method=critique` Test (`#Verify`-linked to it) whose latest result is `fail`
/// "induces suspicion" — computed from the authored critique, nothing stored; re-clear by appending
/// a passing result to that critique (or a later passing critique). Returns the sorted element set.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_suspect(root: &Path) -> Result<Vec<String>, ViewError> {
    Ok(critique_suspect_set(&Model::build(root)?))
}

/// Pure core of [`critique_suspect`]: the sorted set of elements with an unresolved failing critique.
fn critique_suspect_set(model: &Model) -> Vec<String> {
    let mut suspect: HashSet<String> = HashSet::new();
    for e in &model.edges {
        if e.kind != "verify" {
            continue;
        }
        let Some(src) = model.items.get(&e.from) else { continue };
        if src.attrs.get("method").map(String::as_str) != Some("critique") {
            continue;
        }
        if matches!(latest_result(model, &e.from), Some((ref o, _)) if o == "fail") {
            suspect.insert(e.to.clone());
        }
    }
    let mut out: Vec<String> = suspect.into_iter().collect();
    out.sort();
    out
}

/// Critical-finding targets lacking a non-aiModel critic (D0080/issue031 independence).
///
/// An element verified by a `method=critique` Test with `severity=Critical` whose latest result is
/// `fail` (a Critical finding) MUST also carry a critique by a human/tool critic — aiModel-vs-aiModel
/// shares blind spots, so the highest-stakes findings require cognition-distinct independence. Returns
/// the gap set (vacuous until a Critical finding exists).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critical_independence_gaps(root: &Path) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    let mut critical_targets: HashSet<String> = HashSet::new();
    let mut non_ai_covered: HashSet<String> = HashSet::new();
    for e in &model.edges {
        if e.kind != "verify" {
            continue;
        }
        let Some(src) = model.items.get(&e.from) else { continue };
        if src.attrs.get("method").map(String::as_str) != Some("critique") {
            continue;
        }
        if src.attrs.get("severity").map(String::as_str) == Some("Critical") && matches!(latest_result(&model, &e.from), Some((ref o, _)) if o == "fail") {
            critical_targets.insert(e.to.clone());
        }
        if matches!(src.attrs.get("critiquedBy").map(String::as_str), Some("human" | "tool")) {
            non_ai_covered.insert(e.to.clone());
        }
    }
    let mut gaps: Vec<String> = critical_targets.into_iter().filter(|t| !non_ai_covered.contains(t)).collect();
    gaps.sort();
    Ok(gaps)
}

/// Why a critique's `procedureText` reads as low-rigor (D0080/issue030), or `None` if it passes.
fn low_rigor_reason(pt: &str) -> Option<&'static str> {
    if pt.chars().count() < 120 {
        return Some("below the 120-char substance floor");
    }
    let up = pt.to_uppercase();
    if up.contains("ATTACK") || up.contains("FINDING") || up.contains("SURVIVED") {
        None
    } else {
        Some("no ATTACK/FINDING/SURVIVED adversarial structure")
    }
}

/// Critique-rigor diagnostics (D0080/issue030): low-rigor critiques + affirming-only critics.
///
/// A critique is low-rigor if its `procedureText` lacks adversarial structure (no ATTACK/FINDING/
/// SURVIVED reasoning) or is below a substance floor (120 chars). A critic (result `judgedBy`) with
/// many critiques and zero findings is flagged as suspiciously affirming. A diagnostic, not a gate.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_rigor(root: &Path) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    let mut out = Vec::new();
    let mut tally: HashMap<String, (u32, u32)> = HashMap::new();
    for (name, info) in &model.items {
        if info.attrs.get("method").map(String::as_str) != Some("critique") {
            continue;
        }
        if let Some(why) = low_rigor_reason(info.attrs.get("procedureText").map_or("", String::as_str)) {
            out.push(format!("low-rigor critique '{name}': {why}"));
        }
        if let Some(res) = model.items.get(&format!("{name}R1")) {
            let by = res.attrs.get("judgedBy").cloned().unwrap_or_default();
            let entry = tally.entry(by).or_insert((0, 0));
            entry.0 += 1;
            if res.attrs.get("outcome").map(String::as_str) == Some("fail") {
                entry.1 += 1;
            }
        }
    }
    let mut critics: Vec<(&String, &(u32, u32))> = tally.iter().collect();
    critics.sort();
    for (by, (total, fails)) in critics {
        if *total >= 5 && *fails == 0 {
            out.push(format!("affirming-only critic '{by}': {total} critiques, 0 findings — verify rigor (D0080)"));
        }
    }
    out.sort();
    Ok(out)
}

/// Critique-coverage view (D0080) as JSON.
///
/// Per-element required-lens matrix + per-type summary + the gap set (elements missing a required
/// lens). Honest by construction; never stored.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let stale = compute_stale_verifications(root, &model);
    let cov = compute_critique_coverage(&model, &stale);
    let gf = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let in_scope = |c: &CritiqueCoverage| governed(gf.as_ref(), &c.element);

    // Summary over GOVERNED elements only (the grandfathered ones aren't required).
    let mut summary: Vec<Json> = Vec::new();
    for ty in ASSURANCE_TYPES {
        let rows: Vec<&CritiqueCoverage> = cov.iter().filter(|c| c.type_name == ty && in_scope(c)).collect();
        if rows.is_empty() {
            continue;
        }
        let covered = i64::try_from(rows.iter().filter(|c| c.covered).count()).unwrap_or(i64::MAX);
        summary.push(Json::Obj(vec![
            ("type".to_string(), Json::s(ty)),
            ("governed".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX))),
            ("covered".to_string(), Json::Int(covered)),
            ("uncovered".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX) - covered)),
        ]));
    }

    let elements: Vec<Json> = cov
        .iter()
        .map(|c| {
            let lenses: Vec<Json> = c
                .lenses
                .iter()
                .map(|l| {
                    Json::Obj(vec![
                        ("lens".to_string(), Json::s(l.lens)),
                        ("critiqued".to_string(), Json::Bool(l.critiqued)),
                        ("critic".to_string(), l.critic.clone().map_or(Json::Null, Json::s)),
                        ("outcome".to_string(), l.outcome.clone().map_or(Json::Null, Json::s)),
                    ])
                })
                .collect();
            Json::Obj(vec![
                ("element".to_string(), Json::s(c.element.clone())),
                ("type".to_string(), Json::s(c.type_name.clone())),
                ("governed".to_string(), Json::Bool(in_scope(c))),
                ("covered".to_string(), Json::Bool(c.covered)),
                ("lenses".to_string(), Json::Arr(lenses)),
            ])
        })
        .collect();

    // The gap set is the GATE's view: governed + uncovered (grandfathered elements never gate).
    let gaps: Vec<Json> = cov.iter().filter(|c| !c.covered && in_scope(c)).map(|c| Json::s(c.element.clone())).collect();

    let out = Json::Obj(vec![
        (
            "critique".to_string(),
            Json::s("critique-coverage: GOVERNED Need/SystemRequirement/Decision (created after D0080, charter-time D0081) x required lens (Core-3) -> an independent method=critique verification #Verify-linked to the element"),
        ),
        ("summary".to_string(), Json::Arr(summary)),
        ("gaps".to_string(), Json::Arr(gaps)),
        ("elements".to_string(), Json::Arr(elements)),
    ]);
    Ok(out.dump())
}

// ── assurance readiness (D0079 c — the composite capstone gate) ───────────────────────────────
// `assured` composes the whole assurance picture into ONE verdict: the deliverable is READY iff
// (1) coverage complete (every Need/Requirement/Decision covered), (2) critique complete (every
// required lens critiqued), (3) no stale verification (suspect empty), (4) every finding >= Medium
// is dispositioned (no open >= Medium finding Issue), (5) no Critical finding left open, and
// (6) invariants green (all enforced guards pass). NOT-READY lists the exact blockers per category.
// Nothing stored — recomputed from authored facts + git.

/// `true` if a severity string is >= Medium (the human-disposition tier, D0079).
#[allow(clippy::missing_const_for_fn)] // cannot match on `str` in a const fn
fn at_least_medium(sev: &str) -> bool {
    matches!(sev, "Critical" | "High" | "Medium")
}

struct ReadinessBlockers {
    coverage_gaps: Vec<String>,
    critique_gaps: Vec<String>,
    stale_verifications: Vec<String>,
    undispositioned_findings: Vec<String>, // open finding Issues with severity >= Medium
    unfixed_critical: Vec<String>,         // open finding Issues with severity == Critical
    invariant_violations: Vec<String>,     // enforced-guard violations (guard all)
}

impl ReadinessBlockers {
    /// READY = all BLOCKING categories empty. `stale_verifications` is ADVISORY (the D0050
    /// informational signal — cleared by re-verification, never a commit gate), so it does not
    /// affect readiness; it is surfaced separately.
    const fn ready(&self) -> bool {
        self.coverage_gaps.is_empty()
            && self.critique_gaps.is_empty()
            && self.undispositioned_findings.is_empty()
            && self.unfixed_critical.is_empty()
            && self.invariant_violations.is_empty()
    }
}

/// Readiness finding-blockers from issue resolution (D0079/D0080/D0092), as `(undispositioned_ge_medium,
/// open_critical)`. A finding is UNDISPOSITIONED iff it is open AND carries NO typed `#Dispositions`
/// verdict (D0092 retires the prior `resolvers.is_empty()` proxy — D0079 requires every >= Medium
/// finding be DISPOSITIONED (ACT/ACCEPT-RISK/DISMISS), so an ACT'd finding whose resolver is still in
/// flight is dispositioned and does NOT block). An open Critical always blocks until fixed (D0080).
fn finding_blockers(resolution: &[IssueStatus], model: &Model) -> (Vec<String>, Vec<String>) {
    let mut undisp: Vec<String> = Vec::new();
    let mut critical: Vec<String> = Vec::new();
    for i in resolution {
        if !i.open {
            continue;
        }
        let Some(sev) = model.items.get(&i.issue).and_then(|x| x.attrs.get("severity")) else { continue };
        if at_least_medium(sev) && issue_disposition(model, &i.issue).is_none() {
            undisp.push(i.issue.clone());
        }
        if sev == "Critical" {
            critical.push(i.issue.clone());
        }
    }
    undisp.sort();
    critical.sort();
    (undisp, critical)
}

fn compute_readiness(root: &Path) -> Result<ReadinessBlockers, ViewError> {
    let model = Model::build(root)?;
    let done = crate::orient::done_names(root);
    let suspect_vec = crate::orient::compute(root).suspect;
    let task_suspect: HashSet<String> = suspect_vec.iter().cloned().collect();
    let stale = compute_stale_verifications(root, &model);

    // Charter-time scoping (D0081): only GOVERNED elements (created after the governing decision)
    // count as gaps — grandfathered elements are out of the gate.
    let gf_cov = crate::govern::grandfathered_under(root, COVERAGE_DECISION);
    let gf_crit = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let coverage_gaps: Vec<String> = compute_coverage(&model, &done, &task_suspect, &stale)
        .into_iter()
        .filter(|c| !is_covered_tier(c.tier) && governed(gf_cov.as_ref(), &c.element))
        .map(|c| c.element)
        .collect();
    let critique_gaps: Vec<String> = compute_critique_coverage(&model, &stale)
        .into_iter()
        .filter(|c| !c.covered && governed(gf_crit.as_ref(), &c.element))
        .map(|c| c.element)
        .collect();

    let (undispositioned_findings, unfixed_critical) = finding_blockers(&compute_issue_resolution(&model, &done), &model);

    // Base invariant guards only — EXCLUDE `assured` (would recurse) and `critique` (composed
    // separately as critique_gaps). This is what "invariants green" means for readiness.
    let invariant_violations: Vec<String> = crate::guards::GUARD_NAMES
        .iter()
        .copied()
        .filter(|n| !matches!(*n, "assured" | "critique"))
        .filter_map(|n| crate::guards::run_one(n, root))
        .flat_map(|r| r.violations.into_iter().map(move |v| format!("{}: {v}", r.name)))
        .collect();

    Ok(ReadinessBlockers {
        coverage_gaps,
        critique_gaps,
        stale_verifications: suspect_vec,
        undispositioned_findings,
        unfixed_critical,
        invariant_violations,
    })
}

/// Readiness blocker summaries (the `guard assured` violation set) — empty iff READY.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn assured_blockers(root: &Path) -> Result<Vec<String>, ViewError> {
    let b = compute_readiness(root)?;
    let mut out = Vec::new();
    let note = |out: &mut Vec<String>, label: &str, v: &[String]| {
        if !v.is_empty() {
            out.push(format!("{label}: {} ({})", v.len(), v.iter().take(5).cloned().collect::<Vec<_>>().join(", ")));
        }
    };
    // BLOCKING categories only (stale_verifications is advisory — see ReadinessBlockers::ready).
    note(&mut out, "coverage gaps", &b.coverage_gaps);
    note(&mut out, "critique gaps", &b.critique_gaps);
    note(&mut out, "undispositioned >=Medium findings", &b.undispositioned_findings);
    note(&mut out, "unfixed Critical findings", &b.unfixed_critical);
    note(&mut out, "invariant violations", &b.invariant_violations);
    Ok(out)
}

/// Assurance-readiness view (D0079 c) as JSON: the composite READY/NOT-READY verdict + per-category
/// blocker counts and samples. The single "is the deliverable assured?" answer; never stored.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn assured(root: &Path) -> Result<String, ViewError> {
    let b = compute_readiness(root)?;
    let cat = |label: &str, v: &[String]| {
        Json::Obj(vec![
            ("category".to_string(), Json::s(label)),
            ("count".to_string(), Json::Int(i64::try_from(v.len()).unwrap_or(i64::MAX))),
            ("sample".to_string(), Json::Arr(v.iter().take(10).map(|s| Json::s(s.clone())).collect())),
        ])
    };
    let blockers = Json::Arr(vec![
        cat("coverage_gaps", &b.coverage_gaps),
        cat("critique_gaps", &b.critique_gaps),
        cat("undispositioned_findings", &b.undispositioned_findings),
        cat("unfixed_critical", &b.unfixed_critical),
        cat("invariant_violations", &b.invariant_violations),
    ]);
    // Advisory: surfaced for the full picture but NOT gating (cleared by re-verification, D0050).
    let advisories = Json::Arr(vec![cat("stale_verifications", &b.stale_verifications)]);
    let out = Json::Obj(vec![
        (
            "assured".to_string(),
            Json::s("assurance readiness (D0079 c; charter-time scoped, D0081): READY iff GOVERNED coverage complete AND GOVERNED critique complete AND every >=Medium finding dispositioned AND no Critical open AND invariants green. stale_verifications is advisory (re-verify; not gating)"),
        ),
        ("ready".to_string(), Json::Bool(b.ready())),
        ("blockers".to_string(), blockers),
        ("advisories".to_string(), advisories),
    ]);
    Ok(out.dump())
}

// ── load-bearing decisions report (formalized; replaces ad-hoc ranking scripts) ─────────────────
// Ranks accepted Decisions by dependence (charters-to x2 + cross-citations from other decisions)
// and flags "antiquated" signals: uncritiqued (no full Core-3 element-critique), references-retired
// (cites a retired mechanism), superseded-in-part (a later decision supersedes/retires/replaces it).
// Superseded ZOMBIES (status != accepted) are out of scope here by design (handled separately).

/// Mechanisms retired/superseded across the project — a decision still citing one signals its
/// process context has moved on (the D0048 case). Curated; extend as more retire.
const RETIRED_MECHANISMS: &[&str] =
    &["query.py", "parity_check", "validate_all", "validate_sysml", "RESUME.md", "StateCursor", "kill_stale_kernels"];

/// The `dNNNN` decision name declared in a decision file's text, if any (handles a `#Marker` prefix).
fn find_decision_name(text: &str) -> Option<String> {
    for line in text.lines() {
        let l = line.trim_start().trim_start_matches('#');
        let Some(rest) = l
            .strip_prefix("part ")
            .or_else(|| l.strip_prefix("ProspectiveChange part "))
            .or_else(|| l.strip_prefix("SafetyChange part "))
        else {
            continue;
        };
        let name = rest.split([' ', ':']).next().unwrap_or("");
        if name.len() == 5 && name.starts_with('d') && name.get(1..).is_some_and(|d| d.chars().all(|c| c.is_ascii_digit())) {
            return Some(name.to_owned());
        }
    }
    None
}

/// Count word-ish mentions of a `dNNNN` decision name (both `d` and `D` forms) in `text`.
fn count_mentions(text: &str, name: &str) -> usize {
    let upper = format!("D{}", name.get(1..).unwrap_or(""));
    text.matches(name).count() + text.matches(&upper).count()
}

/// True if any line mentions `name` alongside a supersede/retire/replace verb (a later decision
/// revising this one).
fn supersede_near(text: &str, name: &str) -> bool {
    let upper = format!("D{}", name.get(1..).unwrap_or(""));
    text.lines().any(|line| {
        (line.contains(name) || line.contains(&upper))
            && (line.contains("supersede") || line.contains("Supersede") || line.contains("retire") || line.contains("replace"))
    })
}

struct DecisionRow {
    name: String,
    charters: usize,
    citations: usize,
    score: usize,
    uncritiqued: bool,
    references_retired: Vec<String>,
    superseded_in_part: Vec<String>,
}

/// Load-bearing decisions report (formalized) as JSON: accepted Decisions ranked by dependence,
/// each with critique-coverage + antiquation flags. Computed from authored facts; no stored data.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn decisions_report(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    // Decision-file texts keyed by decision name (for citation + supersede + retired scans).
    let mut texts: Vec<(String, String)> = Vec::new();
    for path in crate::collect_sysml(&root.join(".engine").join("decisions")) {
        if let Ok(t) = std::fs::read_to_string(&path) {
            if let Some(name) = find_decision_name(&t) {
                texts.push((name, t));
            }
        }
    }
    // Decisions with FULL Core-3 critique coverage (so `uncritiqued` = not in this set).
    let stale = compute_stale_verifications(root, &model);
    let critiqued: HashSet<String> = compute_critique_coverage(&model, &stale)
        .into_iter()
        .filter(|c| c.covered && c.type_name == "Decision")
        .map(|c| c.element)
        .collect();

    let mut rows: Vec<DecisionRow> = Vec::new();
    for (name, info) in &model.items {
        if info.type_name != "Decision" || info.attrs.get("status").map(String::as_str) != Some("accepted") {
            continue; // accepted decisions only — zombies (non-accepted) are out of scope here
        }
        let charters = model.edges.iter().filter(|e| e.kind == "charteredby" && &e.to == name).count();
        let mut citations = 0;
        let mut superseded_in_part: Vec<String> = Vec::new();
        let mut own_text = "";
        for (other, t) in &texts {
            if other == name {
                own_text = t;
                continue;
            }
            let n = count_mentions(t, name);
            citations += n;
            if n > 0 && supersede_near(t, name) {
                superseded_in_part.push(other.clone());
            }
        }
        superseded_in_part.sort();
        let references_retired: Vec<String> =
            RETIRED_MECHANISMS.iter().filter(|m| own_text.contains(**m)).map(|m| (*m).to_owned()).collect();
        rows.push(DecisionRow {
            charters,
            citations,
            score: charters * 2 + citations,
            uncritiqued: !critiqued.contains(name),
            references_retired,
            superseded_in_part,
            name: name.clone(),
        });
    }
    rows.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.name.cmp(&b.name)));

    let decisions: Vec<Json> = rows
        .iter()
        .map(|r| {
            Json::Obj(vec![
                ("decision".to_string(), Json::s(r.name.clone())),
                ("score".to_string(), Json::Int(i64::try_from(r.score).unwrap_or(i64::MAX))),
                ("charters".to_string(), Json::Int(i64::try_from(r.charters).unwrap_or(i64::MAX))),
                ("citations".to_string(), Json::Int(i64::try_from(r.citations).unwrap_or(i64::MAX))),
                ("uncritiqued".to_string(), Json::Bool(r.uncritiqued)),
                ("references_retired".to_string(), Json::Arr(r.references_retired.iter().map(|s| Json::s(s.clone())).collect())),
                ("superseded_in_part".to_string(), Json::Arr(r.superseded_in_part.iter().map(|s| Json::s(s.clone())).collect())),
            ])
        })
        .collect();
    let out = Json::Obj(vec![
        (
            "report".to_string(),
            Json::s("load-bearing decisions: accepted Decisions ranked by dependence (charters x2 + cross-citations) + antiquation flags. uncritiqued = lacks full Core-3 element-critique; references_retired = cites a retired mechanism; superseded_in_part = HEURISTIC (a later decision's text mentions it near supersede/retire/replace — a hint to review, not authority). Zombies (status != accepted) are out of scope. Computed, never stored."),
        ),
        ("decisions".to_string(), Json::Arr(decisions)),
    ]);
    Ok(out.dump())
}

// ── comprehensive traceability diagram (computed view; interactive self-contained HTML, D0085) ──
// The whole model — every element (node, typed + metadata) and every typed edge — emitted as ONE
// interactive HTML page (cytoscape): filter by node type / edge kind, search, click-to-focus a
// neighborhood, fit. Regenerated on demand from authored facts; never committed as truth (§2.1/D0015).

// ── computed report / scorecard layer (D0087) ─────────────────────────────────────────────────
// Human-digestible AGGREGATE reports rolling up the per-element views into totals/percentages +
// a health/opportunity read. Each report emits a `cards` array (label/value/detail/tone) so ONE
// HTML template renders all of them. Computed on demand; never authored, never committed (§2.1).

/// Integer percentage `n/d` (vacuously 100% when there is nothing to measure).
fn pct(n: usize, d: usize) -> u32 {
    if d == 0 { 100 } else { u32::try_from(n.saturating_mul(100) / d).unwrap_or(0) }
}

/// Tone for a coverage-style percentage (higher is better).
const fn cov_tone(p: u32) -> &'static str {
    if p >= 90 { "good" } else if p >= 70 { "warn" } else { "bad" }
}

/// One scorecard metric card.
fn card(label: &str, value: String, detail: String, tone: &str) -> Json {
    Json::Obj(vec![
        ("label".to_string(), Json::s(label.to_string())),
        ("value".to_string(), Json::s(value)),
        ("detail".to_string(), Json::s(detail)),
        ("tone".to_string(), Json::s(tone.to_string())),
    ])
}

/// Compute a report's `(title, cards)`; shared by the JSON emitter and the HTML scorecard.
fn report_cards(root: &Path, name: &str) -> Result<(String, Vec<Json>), ViewError> {
    let model = Model::build(root)?;
    let orient = crate::orient::compute(root);
    let done = crate::orient::done_names(root);
    let task_suspect: HashSet<String> = orient.suspect.iter().cloned().collect();
    let stale = compute_stale_verifications(root, &model);
    let cov = compute_coverage(&model, &done, &task_suspect, &stale);
    match name {
        "assurance" => Ok(("Assurance Scorecard".to_string(), assurance_cards(root, &model, &cov, &stale, &done, &task_suspect)?)),
        "traceability" => Ok(("Traceability / V&V Coverage".to_string(), traceability_cards(&model, &cov))),
        "quality-debt" => Ok(("Quality & Debt".to_string(), quality_debt_cards(root, &model, &cov, &stale, &task_suspect))),
        "flow" => Ok(("Flow / Velocity".to_string(), flow_cards(root, &model, &orient))),
        "governance" => Ok(("Governance / Decisions".to_string(), governance_cards(&model))),
        "friction" => Ok(("Authoring Friction (vs spreadsheet)".to_string(), friction_cards())),
        other => Err(ViewError::UnknownReport(other.to_string())),
    }
}

/// Computed report as JSON (D0087); `trend` adds a git-derived time-series for the headline metric.
///
/// # Errors
/// Returns [`ViewError`] for an unknown report name or a parse failure.
pub fn report(root: &Path, name: &str, trend: bool) -> Result<String, ViewError> {
    let (title, cards) = report_cards(root, name)?;
    let mut obj = vec![
        ("report".to_string(), Json::s(name.to_string())),
        ("title".to_string(), Json::s(title)),
        ("note".to_string(), Json::s("computed aggregate (D0087) — regenerate, never commit as truth".to_string())),
        ("cards".to_string(), Json::Arr(cards)),
    ];
    if trend {
        obj.push(("trend".to_string(), trend_json(root, name)));
    }
    Ok(Json::Obj(obj).dump())
}

/// A report's headline metric git-derived series `{label, series:[{commit,value}]}` over recent
/// commits (the report's primary scalar, via [`metric_value`]).
fn trend_json(root: &Path, report: &str) -> Json {
    let series: Vec<Json> = trend_series(root, report_headline_key(report))
        .into_iter()
        .map(|(sha, v)| Json::Obj(vec![("commit".to_string(), Json::s(sha)), ("value".to_string(), Json::s(format!("{v:.2}")))]))
        .collect();
    Json::Obj(vec![
        ("label".to_string(), Json::s(headline_label(report).to_string())),
        ("series".to_string(), Json::Arr(series)),
    ])
}

/// Recent commits touching the model (chronological: oldest → newest), capped at `n`.
fn sampled_commits(root: &Path, n: usize) -> Vec<String> {
    let out = git_out(root, &["log", &format!("-n{n}"), "--format=%H", "--", ".tracking", ".engine"]).unwrap_or_default();
    let mut v: Vec<String> = out.lines().map(str::to_string).collect();
    v.reverse();
    v
}

/// Run `git -C root <args>` and capture stdout, or `None` on non-zero exit / failure.
fn git_out(root: &Path, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("git").arg("-C").arg(root).args(args).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The headline metric's display label per report.
const fn headline_label(name: &str) -> &str {
    match name.as_bytes() {
        b"assurance" => "Verification coverage %",
        b"traceability" => "Requirements verified %",
        b"quality-debt" => "Supersede edges (volatility)",
        b"governance" => "Accepted decisions",
        b"friction" => "Write-API verbs (1-command facts)",
        _ => "Delivered points (burnup)",
    }
}

/// The single shared computation for a canonical scalar metric (D0090).
///
/// Both the report cards and the computed Indicators source their numeric value from this keyed
/// registry, so each metric is computed in exactly one place. `None` if the key is unknown or the
/// model fails to build.
#[must_use]
pub fn metric_value(root: &Path, key: &str) -> Option<f64> {
    let model = Model::build(root).ok()?;
    let cnt = |n: usize| -> f64 { f64::from(u32::try_from(n).unwrap_or(u32::MAX)) };
    match key {
        // coverage-family (the full tier pipeline)
        "coverage_pct" | "req_verified_pct" | "needs_verified_pct" => {
            let done = crate::orient::done_names(root);
            let task_suspect: HashSet<String> = crate::orient::compute(root).suspect.into_iter().collect();
            let stale = compute_stale_verifications(root, &model);
            let cov = compute_coverage(&model, &done, &task_suspect, &stale);
            Some(f64::from(match key {
                "req_verified_pct" => verified_pct_of(&cov, "SystemRequirement"),
                "needs_verified_pct" => verified_pct_of(&cov, "Need"),
                _ => coverage_pct_of(&cov, ""),
            }))
        }
        "critique_pct" => {
            let stale = compute_stale_verifications(root, &model);
            let crit = compute_critique_coverage(&model, &stale);
            Some(f64::from(pct(crit.iter().filter(|c| c.covered).count(), crit.len())))
        }
        "attestation_pct" => {
            let (total, missing) = compute_attestation(&model);
            Some(f64::from(pct(total - missing.len(), total)))
        }
        "volatility" => Some(cnt(model.edges.iter().filter(|e| e.kind == "supersede").count())),
        "accepted_decisions" => Some(cnt(model.items.values().filter(|i| i.type_name == "Decision" && i.attrs.get("status").map(String::as_str) == Some("accepted")).count())),
        "open_findings" => {
            let done = crate::orient::done_names(root);
            let (undisp, crit) = finding_blockers(&compute_issue_resolution(&model, &done), &model);
            Some(cnt(undisp.len() + crit.len()))
        }
        "friction_verbs" => Some(4.0), // the 4 one-command write-API verbs (a fixed benchmark)
        "velocity" | "burnup" | "throughput" => {
            let flows = collect_flows(root);
            match key {
                "throughput" => Some(cnt(flows.len())),
                "burnup" => Some(f64::from(i32::try_from(flows.iter().map(|f| f.points).sum::<i64>()).unwrap_or(i32::MAX))),
                _ => Some(velocity_of(&flows)),
            }
        }
        _ => None,
    }
}

/// The headline metric key a report's `--trend` tracks (the report's primary scalar).
fn report_headline_key(report: &str) -> &str {
    match report {
        "assurance" => "coverage_pct",
        "traceability" => "req_verified_pct",
        "quality-debt" => "volatility",
        "governance" => "accepted_decisions",
        "friction" => "friction_verbs",
        _ => "burnup", // flow
    }
}

/// Compute a keyed metric ([`metric_value`]) at each sampled commit via a throwaway git worktree
/// (reuses the whole pipeline unchanged at that commit). Commits that fail to check out are skipped.
fn trend_series(root: &Path, key: &str) -> Vec<(String, f64)> {
    let mut out = Vec::new();
    // 12 recent commits balances a readable trendline against the per-commit worktree+pipeline cost.
    for sha in sampled_commits(root, 12) {
        let short: String = sha.chars().take(8).collect();
        let Some(wt) = std::env::temp_dir().join(format!("sysmlv2-trend-{short}")).to_str().map(str::to_string) else { continue };
        // Best-effort clean, then add a detached worktree at the commit.
        let _ = git_out(root, &["worktree", "remove", "--force", &wt]);
        if git_out(root, &["worktree", "add", "--detach", &wt, &sha]).is_some() {
            if let Some(v) = metric_value(Path::new(&wt), key) {
                out.push((short, v));
            }
            let _ = git_out(root, &["worktree", "remove", "--force", &wt]);
        }
    }
    let _ = git_out(root, &["worktree", "prune"]);
    out
}

/// Computed report rendered as a human-digestible HTML scorecard (D0087).
///
/// # Errors
/// Returns [`ViewError`] for an unknown report name or a parse failure.
pub fn report_html(root: &Path, name: &str, trend: bool) -> Result<String, ViewError> {
    let (title, cards) = report_cards(root, name)?;
    let trend_data = if trend { trend_json(root, name) } else { Json::Null };
    Ok(REPORT_TEMPLATE
        .replace("/*STYLE*/", TABLE_STYLE)
        .replace("/*TITLE*/", &json_esc(&title))
        .replace("/*TREND*/", &trend_data.dump())
        .replace("/*CARDS*/", &Json::Arr(cards).dump()))
}

/// The orient DASHBOARD as a self-contained HTML scorecard (D0093) — the human's recurring home.
///
/// Cards: where things stand + what's ready + open issues + suspect/stale + assurance readiness,
/// reusing the report card template. A computed #View (regenerate-don't-commit), drilling down to the
/// `sysmlv2 orient` JSON authority.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn orient_html(root: &Path) -> Result<String, ViewError> {
    let o = crate::orient::compute(root);
    let preview = |items: &[String], n: usize| -> String {
        if items.is_empty() {
            return "\u{2014}".to_string();
        }
        let shown: Vec<&str> = items.iter().take(n).map(String::as_str).collect();
        let more = items.len().saturating_sub(n);
        if more > 0 { format!("{} \u{2026} +{more} more", shown.join(", ")) } else { shown.join(", ") }
    };
    let rb = compute_readiness(root)?;
    let ready = rb.ready();
    let wip: Vec<String> = o
        .in_progress_sprints
        .iter()
        .map(|s| format!("{} (pending {})", s.sprint, s.pending.clone().unwrap_or_else(|| "\u{2014}".to_string())))
        .collect();
    let suspect_total = o.suspect.len() + o.invalid_evidence.len();
    let cards = vec![
        card("Progress", format!("{} / {}", o.done, o.outstanding), "completed vs outstanding tasks".to_string(), "good"),
        card("Ready to start", o.ready.len().to_string(), format!("unblocked now: {}", preview(&o.ready, 6)), if o.ready.is_empty() { "warn" } else { "good" }),
        card("Sprints in progress", o.in_progress_sprints.len().to_string(), if wip.is_empty() { "none".to_string() } else { preview(&wip, 4) }, if o.in_progress_sprints.len() <= 2 { "good" } else { "warn" }),
        card("Open issues", o.open_issues.len().to_string(), format!("unresolved: {}", preview(&o.open_issues, 6)), if o.open_issues.is_empty() { "good" } else { "warn" }),
        card("Suspect / stale", suspect_total.to_string(), format!("{} drift/criterion + {} invalid-evidence \u{2014} re-verify", o.suspect.len(), o.invalid_evidence.len()), if suspect_total == 0 { "good" } else { "warn" }),
        card(
            "Assurance readiness",
            if ready { "READY".to_string() } else { "NOT READY".to_string() },
            format!("{} coverage + {} critique + {} \u{2265}Medium + {} Critical + {} invariant blocker(s)", rb.coverage_gaps.len(), rb.critique_gaps.len(), rb.undispositioned_findings.len(), rb.unfixed_critical.len(), rb.invariant_violations.len()),
            if ready { "good" } else { "bad" },
        ),
    ];
    Ok(REPORT_TEMPLATE
        .replace("/*STYLE*/", TABLE_STYLE)
        .replace("/*TITLE*/", &json_esc("Orient \u{00b7} where things stand"))
        .replace("/*TREND*/", "null")
        .replace("/*CARDS*/", &Json::Arr(cards).dump()))
}

fn assurance_cards(root: &Path, model: &Model, cov: &[Coverage], stale: &HashSet<String>, done: &HashSet<String>, task_suspect: &HashSet<String>) -> Result<Vec<Json>, ViewError> {
    let total = cov.len();
    let ct = |t: &str| cov.iter().filter(|c| c.tier == t).count();
    let (verified, attested) = (ct("verified"), ct("attested"));
    // Headline from the shared coverage-ratio formula (D0090) — computed in exactly one place
    // (`coverage_pct_of`); verified/attested/total below are the structural breakdown for the detail.
    let covered_pct = coverage_pct_of(cov, "");
    let crit = compute_critique_coverage(model, stale);
    let crit_cov = crit.iter().filter(|c| c.covered).count();
    let crit_pct = pct(crit_cov, crit.len());
    let (att_total, att_missing) = compute_attestation(model);
    let att_pct = pct(att_total - att_missing.len(), att_total);
    // Open finding Issues by severity.
    let open: HashSet<String> = compute_issue_resolution(model, done).into_iter().filter(|i| i.open).map(|i| i.issue).collect();
    let sev_count = |s: &str| open.iter().filter(|n| model.items.get(*n).and_then(|i| i.attrs.get("severity")).map(String::as_str) == Some(s)).count();
    let (crit_f, high_f, med_f, low_f) = (sev_count("Critical"), sev_count("High"), sev_count("Medium"), sev_count("Low"));
    let undisp = crit_f + high_f + med_f;
    let suspect_load = task_suspect.len() + critique_suspect_set(model).len();
    let rb = compute_readiness(root)?;
    let ready = rb.ready();
    Ok(vec![
        card("Verification coverage", format!("{covered_pct}%"), format!("{verified} verified + {attested} attested of {total} (gate-covered)"), cov_tone(covered_pct)),
        card("Critique coverage", format!("{crit_pct}%"), format!("{crit_cov} of {} elements Core-3 critiqued", crit.len()), cov_tone(crit_pct)),
        card("Acceptance integrity", format!("{att_pct}%"), format!("{} of {att_total} accepted decisions attested", att_total - att_missing.len()), cov_tone(att_pct)),
        card("Open findings (\u{2265}Medium)", undisp.to_string(), format!("{crit_f} Critical / {high_f} High / {med_f} Medium / {low_f} Low open"), if crit_f > 0 { "bad" } else if undisp > 0 { "warn" } else { "good" }),
        card("Suspect load", suspect_load.to_string(), format!("{} drift/criterion + {} failing-critique; {} stale verifications", task_suspect.len(), critique_suspect_set(model).len(), stale.len()), if suspect_load == 0 { "good" } else { "warn" }),
        card("Assurance readiness", if ready { "READY".to_string() } else { "NOT READY".to_string() }, format!("{} coverage + {} critique + {} \u{2265}Medium + {} Critical + {} invariant blocker(s)", rb.coverage_gaps.len(), rb.critique_gaps.len(), rb.undispositioned_findings.len(), rb.unfixed_critical.len(), rb.invariant_violations.len()), if ready { "good" } else { "bad" }),
    ])
}

fn traceability_cards(model: &Model, cov: &[Coverage]) -> Vec<Json> {
    let by_type = |ty: &str| -> Vec<&Coverage> { cov.iter().filter(|c| c.type_name == ty).collect() };
    let needs = by_type("Need");
    let reqs = by_type("SystemRequirement");
    // Headlines from the shared verified-ratio formula (D0090; single-source with metric_value); the
    // per-tier breakdown below stays local for the detail text.
    let n_pct = verified_pct_of(cov, "Need");
    let r_pct = verified_pct_of(cov, "SystemRequirement");
    // Edge completeness. A Need is satisfied by an OUTGOING satisfy edge (need -> requirement); a
    // requirement is verified by an INCOMING verify edge (test/critique -> requirement, #Verify).
    let names_of = |ty: &str| -> Vec<&String> { model.items.iter().filter(|(_, i)| i.type_name == ty).map(|(n, _)| n).collect() };
    let needs_names = names_of("Need");
    let n_tot = needs_names.len();
    let n_sat = needs_names.iter().filter(|n| has_outgoing(&model.edges, n, "satisfy")).count();
    let req_names = names_of("SystemRequirement");
    let r_tot = req_names.len();
    let r_ver = req_names
        .iter()
        .filter(|n| model.edges.iter().any(|e| e.kind == "verify" && &e.to == **n))
        .count();
    let r_tier = |t: &str| reqs.iter().filter(|c| c.tier == t).count();
    vec![
        card("Needs verified", format!("{n_pct}%"), format!("{} of {} needs reach a verified requirement", needs.iter().filter(|c| c.tier == "verified").count(), needs.len()), cov_tone(n_pct)),
        card("Requirements verified", format!("{r_pct}%"), format!("{} verified / {} attested / {} addressed / {} uncovered of {}", r_tier("verified"), r_tier("attested"), r_tier("addressed"), r_tier("uncovered") + r_tier("suspect"), reqs.len()), cov_tone(r_pct)),
        card("Needs with satisfy edge", format!("{}%", pct(n_sat, n_tot)), format!("{n_sat} of {n_tot} needs carry a satisfy edge"), cov_tone(pct(n_sat, n_tot))),
        card("Requirements with verify edge", format!("{}%", pct(r_ver, r_tot)), format!("{r_ver} of {r_tot} requirements carry a verify edge (DO-178C-style traceability)"), cov_tone(pct(r_ver, r_tot))),
    ]
}

fn quality_debt_cards(root: &Path, model: &Model, cov: &[Coverage], stale: &HashSet<String>, task_suspect: &HashSet<String>) -> Vec<Json> {
    // Charter debt: grandfathered elements (pre-rigor) that are still not gate-covered or not critiqued.
    let gf_cov = crate::govern::grandfathered_under(root, COVERAGE_DECISION);
    let gf_crit = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let cov_debt = cov.iter().filter(|c| !is_covered_tier(c.tier) && gf_cov.as_ref().is_some_and(|g| g.contains(&c.element))).count();
    let crit_debt = compute_critique_coverage(model, stale).into_iter().filter(|c| !c.covered && gf_crit.as_ref().is_some_and(|g| g.contains(&c.element))).count();
    // Requirements volatility: supersede edges (churn signal).
    let supersedes = model.edges.iter().filter(|e| e.kind == "supersede").count();
    let decisions = model.items.values().filter(|i| i.type_name == "Decision").count();
    let vol_pct = pct(supersedes, decisions);
    let suspect_total = task_suspect.len() + critique_suspect_set(model).len();
    vec![
        card("Charter debt (coverage)", cov_debt.to_string(), format!("{cov_debt} grandfathered elements still not gate-covered (pre-D0079 rigor backlog)"), if cov_debt == 0 { "good" } else { "warn" }),
        card("Charter debt (critique)", crit_debt.to_string(), format!("{crit_debt} grandfathered elements still missing Core-3 critique (pre-D0080)"), if crit_debt == 0 { "good" } else { "warn" }),
        card("Requirements volatility", format!("{vol_pct}%"), format!("{supersedes} supersede edges across {decisions} decisions (churn / early-warning signal)"), if vol_pct >= 30 { "warn" } else { "good" }),
        card("Suspect + stale", suspect_total.to_string(), format!("{suspect_total} elements suspect; {} stale verifications to re-run", stale.len()), if suspect_total == 0 { "good" } else { "warn" }),
    ]
}

/// Days since 1970-01-01 for a civil date (Hinnant's algorithm; exact, no deps).
const fn days_from_civil(y0: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y0 - 1 } else { y0 };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Parse `"YYYY-MM-DD"` to days-since-epoch.
fn parse_ymd(s: &str) -> Option<i64> {
    let mut it = s.trim().splitn(3, '-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.chars().take_while(char::is_ascii_digit).collect::<String>().parse().ok()?;
    Some(days_from_civil(y, m, d))
}

/// The quoted value immediately after `key` on a line (`key` includes the opening quote).
fn quoted_after(line: &str, key: &str) -> Option<String> {
    line.split(key).nth(1)?.split('"').next().map(str::to_string)
}

/// Per-sprint flow facts pulled from one delivery file.
struct SprintFlow {
    points: i64,
    created: Option<i64>,
    refine: Option<i64>,
    retro: Option<i64>,
}

fn sprint_flow(text: &str) -> SprintFlow {
    let mut sf = SprintFlow { points: 0, created: None, refine: None, retro: None };
    for line in text.lines() {
        let t = line.trim_start();
        if sf.points == 0 {
            if let Some(p) = t.split("estimatedPoints = ").nth(1).and_then(|x| x.trim().trim_end_matches(';').trim().parse::<i64>().ok()) {
                sf.points = p;
            }
        }
        if sf.created.is_none() {
            if let Some(c) = quoted_after(t, "createdAt = \"") {
                sf.created = parse_ymd(&c);
            }
        }
        if t.contains("RefineGateR") {
            if let Some(d) = quoted_after(t, "judgedAt = \"") {
                sf.refine = parse_ymd(&d);
            }
        }
        if t.contains("RetroGateR") {
            if let Some(d) = quoted_after(t, "judgedAt = \"") {
                sf.retro = parse_ymd(&d);
            }
        }
    }
    sf
}

/// Per-sprint flow facts for every delivery file. Shared by `metric_value` (velocity/throughput/
/// burnup) and the flow scorecard (D0090) so the sprint set is parsed in exactly one place.
fn collect_flows(root: &Path) -> Vec<SprintFlow> {
    crate::collect_sysml(&root.join(".tracking").join("delivery"))
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok().map(|t| sprint_flow(&t)))
        .collect()
}

/// Mean delivered points per sprint (the canonical velocity, f64) over `flows`. The single velocity
/// formula — shared by the velocity Indicator/metric and the flow scorecard card (D0090).
fn velocity_of(flows: &[SprintFlow]) -> f64 {
    if flows.is_empty() {
        return 0.0;
    }
    let points: i64 = flows.iter().map(|f| f.points).sum();
    f64::from(i32::try_from(points).unwrap_or(i32::MAX)) / f64::from(u32::try_from(flows.len()).unwrap_or(u32::MAX))
}

fn flow_cards(root: &Path, model: &Model, orient: &crate::orient::Output) -> Vec<Json> {
    let _ = model;
    let ready = orient.ready.len();
    let wip = orient.in_progress_sprints.len();
    let open_issues = orient.open_issues.len();
    let flows = collect_flows(root);
    let sprints = flows.len();
    let total_points: i64 = flows.iter().map(|f| f.points).sum();
    // Canonical velocity from the shared formula (D0090) — same number the velocity Indicator shows.
    let velocity = velocity_of(&flows);
    // Cycle time (refine→retro) + lead time (created→retro), in days, over sprints with both dates.
    let cycles: Vec<i64> = flows.iter().filter_map(|f| Some(f.retro? - f.refine?)).collect();
    let cycle_mean = if cycles.is_empty() { 0 } else { cycles.iter().sum::<i64>() / i64::try_from(cycles.len()).unwrap_or(1) };
    let cycle_pts: i64 = flows.iter().filter(|f| f.retro.is_some() && f.refine.is_some()).map(|f| f.points).sum();
    let cycle_days_total: i64 = cycles.iter().sum();
    let per_point = if cycle_pts == 0 { 0.0 } else { f64::from(i32::try_from(cycle_days_total).unwrap_or(0)) / f64::from(i32::try_from(cycle_pts).unwrap_or(1)) };
    let leads: Vec<i64> = flows.iter().filter_map(|f| Some(f.retro? - f.created?)).collect();
    let lead_mean = if leads.is_empty() { 0 } else { leads.iter().sum::<i64>() / i64::try_from(leads.len()).unwrap_or(1) };
    // Predictability: spread of per-sprint points.
    let pts: Vec<i64> = flows.iter().map(|f| f.points).filter(|p| *p > 0).collect();
    let (pmin, pmax) = (pts.iter().min().copied().unwrap_or(0), pts.iter().max().copied().unwrap_or(0));
    // Aging WIP: as-of (latest recorded date) minus the refine date of any started-but-unfinished sprint.
    let as_of = flows.iter().filter_map(|f| f.retro.or(f.refine)).max().unwrap_or(0);
    let aging = flows.iter().filter(|f| f.refine.is_some() && f.retro.is_none()).filter_map(|f| Some(as_of - f.refine?)).max().unwrap_or(0);
    vec![
        card("Ready frontier", ready.to_string(), format!("{ready} task(s) ready to start now"), if ready == 0 { "warn" } else { "good" }),
        card("Work in progress", wip.to_string(), format!("{wip} sprint(s) with ceremony in progress (low WIP is healthy)"), if wip <= 2 { "good" } else { "warn" }),
        card("Velocity", format!("{velocity:.2}"), format!("~{velocity:.1} points/sprint (mean across {sprints} sprints, {total_points} pts total)"), "good"),
        card("Cycle time", format!("{cycle_mean}d"), format!("mean refine→retro across {} sprints (same-day autonomous = ~0)", cycles.len()), "good"),
        card("Time / story point", format!("{per_point:.2}d"), format!("{cycle_days_total} cycle-days / {cycle_pts} points (lower = faster delivery)"), "good"),
        card("Lead time", format!("{lead_mean}d"), format!("mean created→retro across {} sprints (DORA-style lead time)", leads.len()), "good"),
        card("Predictability", format!("{pmin}–{pmax} pts"), format!("per-sprint point spread (velocity {})", if pmax - pmin <= 4 { "consistent" } else { "variable" }), if pmax - pmin <= 4 { "good" } else { "warn" }),
        card("Throughput", sprints.to_string(), format!("{sprints} delivery sprints recorded"), "good"),
        card("Aging WIP", format!("{aging}d"), format!("oldest unfinished sprint age (as-of latest recorded date); {wip} in progress"), if aging <= 7 { "good" } else { "warn" }),
        card("Open issues", open_issues.to_string(), format!("{open_issues} open issue(s) on the board"), if open_issues == 0 { "good" } else { "warn" }),
    ]
}

/// Authoring-friction benchmark (D0054/issue029): record one canonical fact (a passing test result)
/// via the write API vs the hand-edit and spreadsheet baselines. Makes the D0054 first-class friction
/// requirement VERIFIABLE — "the write path beats a spreadsheet" becomes a checkable claim.
fn friction_cards() -> Vec<Json> {
    vec![
        card("Write API: record a fact", "1 command".to_string(), "append-result / append-gate-result / add-task / apply-review — one invocation, with auto UUID + who/when/commit provenance + append-only enforcement".to_string(), "good"),
        card("Hand-edit .sysml", "~6 steps".to_string(), "open file, locate the DoD, author the TestResult line, generate a UUID, find the insertion point, save — error-prone, no enforcement".to_string(), "warn"),
        card("Spreadsheet (baseline)", "1 row".to_string(), "fast to type, but NO provenance, NO validation, NO computed resolution/suspicion — the JPL friction trap (D0054)".to_string(), "warn"),
        card("Verdict vs spreadsheet", "beats it".to_string(), "the write path ties the spreadsheet on steps (1 command) and dominates on provenance + validation + computed state — satisfies the D0054 first-class friction requirement".to_string(), "good"),
    ]
}

// ── indicators (D0089: monitored measures; computed/pulled/manual; source-agnostic status) ──────

/// Direction-aware status of an indicator given its `goal` and its baseline->latest movement.
fn indicator_status(goal: &str, baseline: f64, latest: f64) -> &'static str {
    let d = latest - baseline;
    match goal {
        "maximize" => {
            if d > 0.0 { "improving" } else if d < 0.0 { "degrading" } else { "flat" }
        }
        "minimize" => {
            if d < 0.0 { "improving" } else if d > 0.0 { "degrading" } else { "flat" }
        }
        _ => "observed",
    }
}

/// The recorded-Measurement series `(measuredAt, value)` (oldest->newest) banked for an indicator —
/// items typed `Measurement` with a `#Measures` edge to the indicator, sorted by `measuredAt`. Works
/// for any method: pulled/manual observations AND computed-indicator snapshots (D0091).
fn measurement_series(model: &Model, indicator: &str) -> Vec<(String, f64)> {
    let mut pts: Vec<(String, f64)> = model
        .edges
        .iter()
        .filter(|e| e.kind == "measures" && e.to == indicator)
        .filter_map(|e| model.items.get(&e.from).filter(|i| i.type_name == "Measurement"))
        .filter_map(|i| {
            let at = i.attrs.get("measuredAt").cloned().unwrap_or_default();
            let v = i.attrs.get("value").and_then(|s| s.parse::<f64>().ok())?;
            Some((at, v))
        })
        .collect();
    pts.sort_by(|a, b| a.0.cmp(&b.0));
    pts
}

/// `(name, metric-key)` for every `computed` Indicator — the snapshot worklist (D0091).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn computed_indicator_keys(root: &Path) -> Result<Vec<(String, String)>, ViewError> {
    let model = Model::build(root)?;
    let mut out: Vec<(String, String)> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Indicator" && i.attrs.get("method").map(String::as_str) == Some("computed"))
        .filter_map(|(n, i)| i.attrs.get("collectionRef").map(|k| (n.clone(), k.clone())))
        .collect();
    out.sort();
    Ok(out)
}

/// Indicators view (D0089): each declared `Indicator`'s value + direction-aware status.
///
/// Source-agnostic over the measurement method — computed series come from the report/trend engine
/// (current value only unless `trend`), pulled/manual series from recorded `Measurement`s.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn indicators(root: &Path, trend: bool) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut names: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Indicator").map(|(n, _)| n).collect();
    names.sort();
    let mut out: Vec<Json> = Vec::new();
    for name in names {
        let Some(info) = model.items.get(name) else { continue };
        let method = info.attrs.get("method").map_or("manual", String::as_str);
        let goal = info.attrs.get("goal").map_or("observe", String::as_str);
        let binding = info.attrs.get("collectionRef").cloned().unwrap_or_default();
        // The banked (measuredAt, value) datapoint series (recorded observations + computed snapshots).
        // A snapshot stores only value + timestamp — no "latest" label; latest is CALCULATED (issue037).
        let banked = measurement_series(&model, name);
        // LIVE current value (computed indicators only) — authoritative + never stale (issue037/038):
        // the live recompute is the source of truth; the bank is historical record, never overrides it.
        let live: Option<f64> = if method == "computed" { metric_value(root, &binding) } else { None };
        // The displayed series: the bank; or, for a computed indicator with no bank, --trend / the live point.
        let series: Vec<(String, f64)> = if banked.is_empty() && method == "computed" {
            if trend {
                trend_series(root, &binding)
            } else {
                live.map(|v| vec![("live".to_string(), v)]).unwrap_or_default()
            }
        } else {
            banked.clone()
        };
        let baseline = series.first().map(|(_, v)| *v);
        // latest is computed: live current for a computed indicator; else the most recent datapoint.
        let latest = live.or_else(|| series.last().map(|(_, v)| *v));
        let banked_latest = banked.last().map(|(_, v)| *v);
        // Drift guardrail (issue038): for a computed indicator, has the live value moved off the last snapshot?
        let drift = matches!((live, banked_latest), (Some(l), Some(b)) if (l - b).abs() > 0.001);
        let status = if baseline.is_some() && latest.is_some() && (series.len() > 1 || live.is_some()) {
            indicator_status(goal, baseline.unwrap_or(0.0), latest.unwrap_or(0.0))
        } else if latest.is_some() {
            "single-point"
        } else {
            "no-data"
        };
        let fmt = |o: Option<f64>| o.map_or_else(|| Json::Null, |v| Json::s(format!("{v:.2}")));
        let series_json: Vec<Json> = series
            .iter()
            .map(|(at, v)| Json::Obj(vec![("at".to_string(), Json::s(at.clone())), ("value".to_string(), Json::s(format!("{v:.2}")))]))
            .collect();
        out.push(Json::Obj(vec![
            ("indicator".to_string(), Json::s(name.clone())),
            ("measures".to_string(), Json::s(info.attrs.get("measures").cloned().unwrap_or_default())),
            ("method".to_string(), Json::s(method.to_string())),
            ("goal".to_string(), Json::s(goal.to_string())),
            ("unit".to_string(), Json::s(info.attrs.get("unit").cloned().unwrap_or_default())),
            ("latest".to_string(), fmt(latest)),       // calculated: live for computed, last datapoint otherwise
            ("live".to_string(), fmt(live)),           // authoritative current recompute (computed only)
            ("baseline".to_string(), fmt(baseline)),
            ("banked_latest".to_string(), fmt(banked_latest)),
            ("drift".to_string(), Json::Bool(drift)),  // computed: live has moved off the last snapshot
            ("points".to_string(), Json::Int(i64::try_from(series.len()).unwrap_or(0))),
            ("status".to_string(), Json::s(status.to_string())),
            ("series".to_string(), Json::Arr(series_json)),
        ]));
    }
    Ok(Json::Obj(vec![
        ("view".to_string(), Json::s("indicators (D0089/D0091): monitored measures + direction-aware status + the banked datapoint series. `latest` is CALCULATED (live recompute for computed indicators — authoritative, never stale; last datapoint otherwise); the bank stores value+timestamp only; `drift`=true when a computed indicator's live value has moved off its last snapshot (the bank is historical record, never overrides live).".to_string())),
        ("indicators".to_string(), Json::Arr(out)),
    ])
    .dump())
}

fn governance_cards(model: &Model) -> Vec<Json> {
    let decisions: Vec<&ItemInfo> = model.items.values().filter(|i| i.type_name == "Decision").collect();
    let total = decisions.len();
    let accepted = decisions.iter().filter(|i| i.attrs.get("status").map(String::as_str) == Some("accepted")).count();
    let superseded = decisions.iter().filter(|i| i.attrs.get("status").map(String::as_str) == Some("superseded")).count();
    let proc_change = decisions.iter().filter(|i| matches!(i.marker.as_deref(), Some("ProspectiveChange" | "SafetyChange"))).count();
    let (att_total, att_missing) = compute_attestation(model);
    let att_pct = pct(att_total - att_missing.len(), att_total);
    let supersede_edges = model.edges.iter().filter(|e| e.kind == "supersede").count();
    vec![
        card("Decisions", total.to_string(), format!("{accepted} accepted / {superseded} superseded of {total} total"), "good"),
        card("Acceptance integrity", format!("{att_pct}%"), format!("{} of {att_total} accepted decisions carry an attestation event", att_total - att_missing.len()), cov_tone(att_pct)),
        card("Process-change decisions", proc_change.to_string(), format!("{proc_change} #ProspectiveChange/#SafetyChange (governed process edits, D0070)"), "good"),
        card("Supersession", supersede_edges.to_string(), format!("{supersede_edges} supersede edges (decision evolution / churn)"), if supersede_edges <= total / 3 { "good" } else { "warn" }),
    ]
}

const REPORT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>sysmlv2 report</title>
<meta name="generator" content="sysmlv2 report (computed #View; regenerate, do not commit as truth)">
/*STYLE*/
<style>
 .cards{display:flex;flex-wrap:wrap;gap:12px;padding:14px}
 .c{flex:1 1 220px;min-width:200px;border:1px solid #ddd;border-radius:8px;padding:12px;background:#fff}
 .c .l{font-size:11px;text-transform:uppercase;color:#666;letter-spacing:.03em}
 .c .v{font-size:30px;font-weight:600;margin:4px 0}
 .c .d{font-size:11px;color:#555;line-height:1.4}
 .c.good{border-left:5px solid #59a14f} .c.warn{border-left:5px solid #f2a900} .c.bad{border-left:5px solid #e15759}
 .c.good .v{color:#3d7a34} .c.warn .v{color:#b07a00} .c.bad .v{color:#b03a3c}
</style></head><body>
<header><h1>sysmlv2 · <span id="t"></span></h1><p>computed aggregate report (D0087) — regenerate, never commit as truth</p></header>
<div id="trend" style="padding:0 14px"></div>
<div class="cards" id="cards"></div>
<script>
var TITLE="/*TITLE*/",CARDS=/*CARDS*/,TREND=/*TREND*/;
document.getElementById('t').textContent=TITLE;
var box=document.getElementById('cards');
CARDS.forEach(function(c){var d=document.createElement('div');d.className='c '+(c.tone||'');
 d.innerHTML='<div class=l></div><div class=v></div><div class=d></div>';
 d.querySelector('.l').textContent=c.label;d.querySelector('.v').textContent=c.value;d.querySelector('.d').textContent=c.detail;box.appendChild(d)});
if(TREND&&TREND.series&&TREND.series.length){var s=TREND.series.map(function(p){return p.value});
 var lo=Math.min.apply(null,s),hi=Math.max.apply(null,s),bl='▁▂▃▄▅▆▇█';
 var spark=s.map(function(v){var i=hi===lo?0:Math.round((v-lo)/(hi-lo)*7);return bl.charAt(i)}).join('');
 var first=s[0],last=s[s.length-1],delta=last-first,arrow=delta>0?'▲ +'+delta:delta<0?'▼ '+delta:'→ 0';
 document.getElementById('trend').innerHTML='<div class="c" style="border-left:5px solid #4e79a7"><div class=l>Trend · '+TREND.label+' ('+s.length+' commits)</div><div class=v style="font-family:monospace;font-size:22px">'+spark+'</div><div class=d>'+first+' → '+last+'  ('+arrow+'); range '+lo+'–'+hi+'. Computed from git history (worktree per commit); never stored.</div></div>'}
</script></body></html>"#;

/// Vendored cytoscape.js, INLINED into every generated diagram so it is fully self-contained +
/// offline (no CDN). ~373KB; the only third-party JS in the page.
const CYTOSCAPE_LIB: &str = include_str!("../assets/cytoscape.min.js");

const DIAGRAM_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>sysmlv2 traceability</title>
<meta name="generator" content="sysmlv2 diagram (computed #View; regenerate, do not commit as truth)">
<script>/*CYTOSCAPE_LIB*/</script>
<style>
 html,body{margin:0;height:100%;font:12px system-ui,sans-serif}
 #cy{position:absolute;left:230px;right:0;top:0;bottom:0}
 #panel{position:absolute;left:0;top:0;bottom:0;width:230px;overflow:auto;background:#f7f7f7;border-right:1px solid #ccc;padding:8px;box-sizing:border-box}
 #panel h3{margin:8px 0 4px;font-size:11px;text-transform:uppercase;color:#555}
 #panel label{display:block;font-size:11px;line-height:1.5;cursor:pointer}
 #panel .sw{display:inline-block;width:9px;height:9px;margin-right:4px;border-radius:2px;vertical-align:middle}
 #search{width:100%;box-sizing:border-box;margin-bottom:6px}
 button{font-size:11px;margin:2px 2px 6px 0;cursor:pointer}
 #info{position:absolute;right:8px;top:8px;max-width:360px;background:#fff;border:1px solid #ccc;padding:8px;font-size:11px;display:none;white-space:pre-wrap;max-height:60%;overflow:auto}
</style></head><body>
<div id="panel">
 <input id="search" placeholder="search id… (Enter to fit)">
 <button onclick="cy.fit(undefined,30)">Fit</button><button onclick="resetView()">Reset</button>
 <h3>Node types</h3><div id="types"></div>
 <h3>Edge kinds</h3><div id="kinds"></div>
 <p style="color:#777;font-size:10px">Click a node = focus its neighborhood. Click background = reset. Computed view — regenerate, never commit as truth.</p>
</div>
<div id="cy"></div><div id="info"></div>
<script>
var elements = /*ELEMENTS*/;
var typeColors={Decision:'#4e79a7',Need:'#59a14f',SystemRequirement:'#76b7b2',Story:'#f28e2b',Test:'#9c755f',TestResult:'#bab0ac',Issue:'#e15759',action:'#edc948',ActionDef:'#e6d27a',Process:'#b07aa1',ProcessStep:'#d4a6c8',AISkill:'#86bcb6'};
var edgeColors={satisfy:'#59a14f',verify:'#4e79a7',charteredby:'#f28e2b',supersede:'#e15759',resolves:'#af7aa1',dependency:'#bab0ac',allocate:'#76b7b2',succession:'#9aa',ordering:'#ccc',prospectivechange:'#9c27b0',safetychange:'#d62728',dependson:'#888',contains:'#dcdcc8',resultof:'#e8e0d0'};
var cy=cytoscape({container:document.getElementById('cy'),elements:elements,
 style:[{selector:'node',style:{'label':'data(label)','font-size':6,'width':11,'height':11,'background-color':function(n){return typeColors[n.data('ntype')]||'#888'},'text-wrap':'wrap','text-max-width':130,'color':'#222'}},
  {selector:'edge',style:{'width':1,'line-color':function(e){return edgeColors[e.data('kind')]||'#bbb'},'target-arrow-color':function(e){return edgeColors[e.data('kind')]||'#bbb'},'target-arrow-shape':'triangle','arrow-scale':0.6,'curve-style':'bezier'}},
  {selector:'.hidden',style:{'display':'none'}},{selector:'.faded',style:{'opacity':0.07}},{selector:'.hi',style:{'background-color':'#ffd400','border-width':2,'border-color':'#c80'}}],
 layout:{name:'grid'}});
var offTypes={},offKinds={};
function refresh(){cy.batch(function(){
 cy.nodes().forEach(function(n){n.toggleClass('hidden',!!offTypes[n.data('ntype')])});
 cy.edges().forEach(function(e){var h=!!offKinds[e.data('kind')]||e.source().hasClass('hidden')||e.target().hasClass('hidden');e.toggleClass('hidden',h)});});}
function resetView(){offTypes={};offKinds={};cy.elements().removeClass('hidden faded hi');document.querySelectorAll('#panel input[type=checkbox]').forEach(function(c){c.checked=true});document.getElementById('info').style.display='none';cy.fit(undefined,30);}
function mkFilters(id,vals,colors,store,offDefault){var d=document.getElementById(id);vals.sort().forEach(function(v){var off=offDefault.indexOf(v)>=0;if(off)store[v]=true;var l=document.createElement('label');var c=document.createElement('input');c.type='checkbox';c.checked=!off;c.onchange=function(){store[v]=!c.checked;refresh()};var sw='<span class=sw style="background:'+(colors[v]||'#888')+'"></span>';l.appendChild(c);l.insertAdjacentHTML('beforeend',sw+v+(off?' (off)':''));d.appendChild(l)})}
mkFilters('types',Array.from(new Set(cy.nodes().map(function(n){return n.data('ntype')}))),typeColors,offTypes,['Test','TestResult']);
mkFilters('kinds',Array.from(new Set(cy.edges().map(function(e){return e.data('kind')}))),edgeColors,offKinds,[]);
refresh();
cy.elements(':visible').layout({name:'cose',animate:false,idealEdgeLength:55,nodeRepulsion:5000,componentSpacing:60}).run();
cy.fit(undefined,30);
cy.on('tap','node',function(ev){var n=ev.target;var nb=n.closedNeighborhood();cy.elements().addClass('faded');nb.removeClass('faded');var d=n.data();var s='';Object.keys(d).forEach(function(k){if(k!=='label')s+=k+': '+d[k]+'\n'});var inf=document.getElementById('info');inf.textContent=s;inf.style.display='block'});
cy.on('tap',function(ev){if(ev.target===cy){cy.elements().removeClass('faded');document.getElementById('info').style.display='none'}});
document.getElementById('search').addEventListener('input',function(e){var q=e.target.value.toLowerCase();cy.nodes().removeClass('hi');if(q)cy.nodes().filter(function(n){return (n.id()+' '+(n.data('label')||'')).toLowerCase().indexOf(q)>=0}).addClass('hi')});
document.getElementById('search').addEventListener('keydown',function(e){if(e.key==='Enter'){var hi=cy.nodes('.hi');if(hi.length)cy.fit(hi,50)}});
</script></body></html>"#;

const TABLE_STYLE: &str = r"<style>
 body{margin:0;font:13px system-ui,sans-serif;color:#222}
 header{padding:10px 14px;background:#f7f7f7;border-bottom:1px solid #ccc}
 header h1{margin:0;font-size:15px} header p{margin:3px 0 0;color:#666;font-size:12px}
 #bar{padding:8px 14px;display:flex;gap:8px;align-items:center;flex-wrap:wrap}
 input,select{padding:3px 4px;font:12px system-ui,sans-serif}
 table{border-collapse:collapse;width:100%;font-size:12px}
 th,td{border:1px solid #ddd;padding:4px 7px;text-align:left;vertical-align:top}
 th{background:#eee;cursor:pointer;position:sticky;top:0}
 tbody tr:nth-child(even){background:#fafafa}
 td.name{font-family:ui-monospace,monospace;white-space:nowrap}
 .count{color:#666;font-size:12px} button{cursor:pointer;padding:4px 9px}
 textarea{width:100%;box-sizing:border-box;font:12px system-ui,sans-serif;min-height:30px}
</style>";

const TABLE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>sysmlv2 view</title>
<meta name="generator" content="sysmlv2 render --mode table (computed #View; regenerate, do not commit as truth)">
/*STYLE*/</head><body>
<header><h1>sysmlv2 · <span id="vn"></span></h1><p id="cn"></p></header>
<div id="bar"><input id="q" placeholder="filter rows…" size="36"><span class="count" id="ct"></span></div>
<table id="t"><thead></thead><tbody></tbody></table>
<script>
var VIEW="/*VIEW*/",CONCERN="/*CONCERN*/",COLS=/*COLS*/,ROWS=/*ROWS*/;
document.getElementById('vn').textContent=VIEW;document.getElementById('cn').textContent=CONCERN;
var allCols=['name','type'].concat(COLS),sortCol=null,sortDir=1,filter='';
function visible(){var rows=ROWS.filter(function(r){return !filter||allCols.some(function(c){return (''+(r[c]||'')).toLowerCase().indexOf(filter)>=0})});
 if(sortCol)rows.sort(function(a,b){var x=''+(a[sortCol]||''),y=''+(b[sortCol]||'');return x<y?-sortDir:x>y?sortDir:0});return rows}
function render(){var th=document.querySelector('#t thead');th.innerHTML='';var tr=document.createElement('tr');
 allCols.forEach(function(c){var h=document.createElement('th');h.textContent=c+(sortCol===c?(sortDir>0?' ▲':' ▼'):'');h.onclick=function(){if(sortCol===c)sortDir=-sortDir;else{sortCol=c;sortDir=1}render()};tr.appendChild(h)});
 th.appendChild(tr);var rows=visible(),tb=document.querySelector('#t tbody');tb.innerHTML='';
 rows.forEach(function(r){var t=document.createElement('tr');allCols.forEach(function(c){var d=document.createElement('td');if(c==='name')d.className='name';d.textContent=r[c]||'';t.appendChild(d)});tb.appendChild(t)});
 document.getElementById('ct').textContent=rows.length+' / '+ROWS.length+' rows';}
document.getElementById('q').addEventListener('input',function(e){filter=e.target.value.toLowerCase();render()});render();
</script></body></html>"#;

const REVIEW_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>sysmlv2 review</title>
<meta name="generator" content="sysmlv2 render --mode review (computed #View; capture is exported to JSON for apply-review)">
/*STYLE*/</head><body>
<header><h1>sysmlv2 review · <span id="vn"></span></h1><p id="cn"></p></header>
<div id="bar">
 reviewer <input id="who" placeholder="your id" size="12">
 commit <input id="sha" placeholder="judgedAgainst (optional)" size="12">
 <input id="q" placeholder="filter rows…" size="22">
 <button onclick="exportJSON()">Export JSON</button>
 <span class="count" id="ct"></span>
</div>
<table id="t"><thead></thead><tbody></tbody></table>
<script>
var VIEW="/*VIEW*/",CONCERN="/*CONCERN*/",COLS=/*COLS*/,ROWS=/*ROWS*/;
document.getElementById('vn').textContent=VIEW;document.getElementById('cn').textContent=CONCERN;
var LENSES=['correctness','completeness','ambiguity','testability','feasibility','consistency','necessity'];
var SEV=['Medium','High','Critical','Low'];
var infoCols=['name','type'].concat(COLS),disp={};
function st(n){if(!disp[n])disp[n]={verdict:'',lens:'correctness',severity:'Medium',rationale:'',actionable:false};return disp[n]}
function visible(){return ROWS.filter(function(r){return !filter||infoCols.some(function(c){return (''+(r[c]||'')).toLowerCase().indexOf(filter)>=0})})}
var filter='';
function sel(opts,val){var s=document.createElement('select');opts.forEach(function(o){var e=document.createElement('option');e.value=o;e.textContent=o;if(o===val)e.selected=true;s.appendChild(e)});return s}
function render(){var th=document.querySelector('#t thead');th.innerHTML='';var tr=document.createElement('tr');
 infoCols.concat(['verdict','lens','severity','actionable?','rationale']).forEach(function(c){var h=document.createElement('th');h.textContent=c;tr.appendChild(h)});th.appendChild(tr);
 var rows=visible(),tb=document.querySelector('#t tbody');tb.innerHTML='';
 rows.forEach(function(r){var n=r.name,s=st(n),t=document.createElement('tr');
  infoCols.forEach(function(c){var d=document.createElement('td');if(c==='name')d.className='name';d.textContent=r[c]||'';t.appendChild(d)});
  var dv=document.createElement('td');var v=sel(['','accept','finding'],s.verdict);v.onchange=function(){s.verdict=v.value};dv.appendChild(v);t.appendChild(dv);
  var dl=document.createElement('td');var l=sel(LENSES,s.lens);l.onchange=function(){s.lens=l.value};dl.appendChild(l);t.appendChild(dl);
  var ds=document.createElement('td');var sv=sel(SEV,s.severity);sv.onchange=function(){s.severity=sv.value};ds.appendChild(sv);t.appendChild(ds);
  var da=document.createElement('td');var a=document.createElement('input');a.type='checkbox';a.checked=s.actionable;a.onchange=function(){s.actionable=a.checked};da.appendChild(a);t.appendChild(da);
  var dr=document.createElement('td');var ta=document.createElement('textarea');ta.value=s.rationale;ta.oninput=function(){s.rationale=ta.value};dr.appendChild(ta);t.appendChild(dr);
  tb.appendChild(t)});
 document.getElementById('ct').textContent=rows.length+' rows';}
function exportJSON(){var out={view:VIEW,judgedBy:document.getElementById('who').value,judgedAgainst:document.getElementById('sha').value,dispositions:[]};
 Object.keys(disp).forEach(function(n){var d=disp[n];if(d&&d.verdict){out.dispositions.push({element:n,verdict:d.verdict,lens:d.lens,severity:d.severity,rationale:d.rationale,actionable:d.actionable})}});
 if(!out.dispositions.length){alert('No dispositions set — choose a verdict on at least one row.');return}
 var b=new Blob([JSON.stringify(out,null,2)],{type:'application/json'});var a=document.createElement('a');a.href=URL.createObjectURL(b);a.download='review-batch.json';a.click();}
document.getElementById('q').addEventListener('input',function(e){filter=e.target.value.toLowerCase();render()});render();
</script></body></html>"#;

/// Comprehensive traceability diagram as a self-contained interactive HTML page (D0085).
///
/// Emits the WHOLE model — every element (typed node + its authored metadata) and every typed edge
/// (satisfy/verify/charteredby/supersede/resolves/dependency/allocate/succession/process-change/...) —
/// into one cytoscape page with type/edge filters, search, click-to-focus, and fit. A computed
/// `#View`: regenerate on demand (`sysmlv2 diagram . > graph.html`), never commit it as truth.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn diagram_html(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let elements = graph_elements(&model, None);
    Ok(DIAGRAM_TEMPLATE
        .replace("/*CYTOSCAPE_LIB*/", CYTOSCAPE_LIB)
        .replace("/*ELEMENTS*/", &Json::Arr(elements).dump()))
}

/// Build the cytoscape element array (typed nodes + their metadata, then edges with both endpoints
/// present) for a model. When `only` is `Some`, restrict to that name-set (a view-scoped subgraph);
/// `None` renders the whole model.
fn graph_elements(model: &Model, only: Option<&HashSet<String>>) -> Vec<Json> {
    let meta_keys = ["title", "status", "severity", "lens", "kind", "priority", "outcome", "method", "critiquedBy", "createdBy"];
    let included = |n: &str| only.is_none_or(|s| s.contains(n));
    let mut items: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(n, _)| included(n)).collect();
    items.sort_by(|a, b| a.0.cmp(b.0));
    let mut elements: Vec<Json> = items
        .iter()
        .map(|(name, info)| {
            // Label with the authored `title` (human string) when present, truncated for legibility;
            // fall back to the part name. The id stays the name (identity); full title is in the
            // click-info panel.
            let label = info.attrs.get("title").map_or_else(
                || (*name).clone(),
                |t| {
                    if t.chars().count() > 60 {
                        format!("{}…", t.chars().take(59).collect::<String>())
                    } else {
                        t.clone()
                    }
                },
            );
            let mut data = vec![
                ("id".to_string(), Json::s((*name).clone())),
                ("label".to_string(), Json::s(label)),
                ("ntype".to_string(), Json::s(if info.type_name.is_empty() { "unknown".to_string() } else { info.type_name.clone() })),
            ];
            for k in meta_keys {
                if let Some(v) = info.attrs.get(k) {
                    data.push((k.to_string(), Json::s(v.clone())));
                }
            }
            if let Some(m) = &info.marker {
                data.push(("marker".to_string(), Json::s(m.clone())));
            }
            Json::Obj(vec![("data".to_string(), Json::Obj(data))])
        })
        .collect();
    // Edges: only those whose BOTH endpoints are present nodes (cytoscape errors on dangling edges);
    // when scoped, both endpoints must also be in the name-set.
    for (i, e) in model.edges.iter().enumerate() {
        if model.items.contains_key(&e.from) && model.items.contains_key(&e.to) && included(&e.from) && included(&e.to) {
            elements.push(Json::Obj(vec![(
                "data".to_string(),
                Json::Obj(vec![
                    ("id".to_string(), Json::s(format!("e{i}"))),
                    ("source".to_string(), Json::s(e.from.clone())),
                    ("target".to_string(), Json::s(e.to.clone())),
                    ("kind".to_string(), Json::s(e.kind.clone())),
                ]),
            )]));
        }
    }
    elements
}

/// Modular interactive-artifact renderer (D0086).
///
/// Renders a declared view as self-contained HTML in one of three modes — `graph` (cytoscape; the
/// whole model when `view` is `model`/`all`, else the view's selected subgraph), `table`
/// (sortable/searchable rows), or `review` (table + per-row accept/finding + rationale capture with
/// a JSON export for `apply-review`). A computed `#View`: regenerate on demand, never commit as truth.
///
/// # Errors
/// Returns [`ViewError`] for an unknown mode, a missing/invalid view, or a parse failure.
pub fn render_html(root: &Path, view: &str, mode: &str) -> Result<String, ViewError> {
    match mode {
        "graph" => {
            if matches!(view, "model" | "all" | "whole") {
                return diagram_html(root);
            }
            let (_, model, result) = run_resolved(root, view)?;
            let elements = graph_elements(&model, Some(&result));
            Ok(DIAGRAM_TEMPLATE
                .replace("/*CYTOSCAPE_LIB*/", CYTOSCAPE_LIB)
                .replace("/*ELEMENTS*/", &Json::Arr(elements).dump()))
        }
        "table" | "review" => {
            let (spec, model, result) = run_resolved(root, view)?;
            Ok(table_or_review_html(&spec, &model, &result, mode == "review"))
        }
        other => Err(ViewError::UnknownMode(other.to_string())),
    }
}

/// Columns rendered for a view's rows: `name`, `type`, then the view's projected fields (or a small
/// default set of common authored fields when the view declares no projection).
fn view_columns(spec: &ViewSpec, model: &Model, result: &HashSet<String>) -> Vec<String> {
    if let Some(p) = &spec.project {
        if !p.fields.is_empty() {
            return p.fields.clone();
        }
    }
    let defaults = ["title", "status", "severity", "outcome", "lens", "method", "kind", "priority"];
    defaults
        .iter()
        .filter(|f| result.iter().any(|n| model.items.get(n).is_some_and(|i| i.attrs.contains_key(**f))))
        .map(|f| (*f).to_string())
        .collect()
}

/// Render a view's rows as either a read-only table or a review surface (extra capture columns +
/// an Export-JSON button that emits a batch consumable by `sysmlv2 apply-review`).
fn table_or_review_html(spec: &ViewSpec, model: &Model, result: &HashSet<String>, review: bool) -> String {
    let cols = view_columns(spec, model, result);
    let mut names: Vec<&String> = result.iter().collect();
    names.sort();
    let rows: Vec<Json> = names
        .iter()
        .filter_map(|n| {
            model.items.get(*n).map(|info| {
                let mut o = vec![
                    ("name".to_string(), Json::s((*n).clone())),
                    ("type".to_string(), Json::s(info.type_name.clone())),
                ];
                for c in &cols {
                    let v = if c == "marker" { info.marker.clone().unwrap_or_default() } else { info.attrs.get(c).cloned().unwrap_or_default() };
                    o.push((c.clone(), Json::s(v)));
                }
                Json::Obj(o)
            })
        })
        .collect();
    let template = if review { REVIEW_TEMPLATE } else { TABLE_TEMPLATE };
    template
        .replace("/*STYLE*/", TABLE_STYLE)
        .replace("/*VIEW*/", &json_esc(&spec.name))
        .replace("/*CONCERN*/", &json_esc(&spec.concern))
        .replace("/*COLS*/", &Json::Arr(cols.iter().map(|c| Json::s(c.clone())).collect()).dump())
        .replace("/*ROWS*/", &Json::Arr(rows).dump())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> Model {
        let mut items = HashMap::new();
        items.insert("r1".to_string(), ItemInfo { type_name: "Requirement".to_string(), attrs: HashMap::new(), marker: None });
        items.insert("c1".to_string(), ItemInfo { type_name: "Component".to_string(), attrs: HashMap::new(), marker: None });
        let mut dattrs = HashMap::new();
        dattrs.insert("status".to_string(), "accepted".to_string());
        items.insert("d1".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: dattrs, marker: Some("ProspectiveChange".to_string()) });
        let edges = vec![Edge { kind: "satisfy".to_string(), from: "r1".to_string(), to: "c1".to_string() }];
        Model { items, edges }
    }

    #[test]
    fn select_by_type() {
        let got = selects(&model(), &Select { type_: Some("Decision".to_string()), ..Default::default() });
        assert_eq!(got.len(), 1);
        assert!(got.contains("d1"));
    }

    #[test]
    fn issue_resolution_open_vs_resolved() {
        // i1 resolved by a done action; i2 open (resolver action not done); i3 untriaged (no edge).
        let mut items = HashMap::new();
        for n in ["i1", "i2", "i3"] {
            items.insert(n.to_string(), ItemInfo { type_name: "Issue".to_string(), attrs: HashMap::new(), marker: None });
        }
        items.insert("actDone".to_string(), ItemInfo { type_name: "action".to_string(), attrs: HashMap::new(), marker: None });
        items.insert("actOpen".to_string(), ItemInfo { type_name: "action".to_string(), attrs: HashMap::new(), marker: None });
        let edges = vec![
            Edge { kind: "resolves".to_string(), from: "actDone".to_string(), to: "i1".to_string() },
            Edge { kind: "resolves".to_string(), from: "actOpen".to_string(), to: "i2".to_string() },
        ];
        let model = Model { items, edges };
        let done: HashSet<String> = std::iter::once("actDone".to_string()).collect();
        let res = compute_issue_resolution(&model, &done);
        let open: Vec<&str> = res.iter().filter(|i| i.open).map(|i| i.issue.as_str()).collect();
        assert_eq!(open, vec!["i2", "i3"]); // i1 resolved; i2 + i3 open
        let i3 = res.iter().find(|i| i.issue == "i3").unwrap();
        assert!(i3.resolvers.is_empty(), "i3 is untriaged");
    }

    #[test]
    fn issue_resolved_by_accepted_decision() {
        let mut items = HashMap::new();
        items.insert("i9".to_string(), ItemInfo { type_name: "Issue".to_string(), attrs: HashMap::new(), marker: None });
        let mut dattrs = HashMap::new();
        dattrs.insert("status".to_string(), "accepted".to_string());
        items.insert("d99".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: dattrs, marker: None });
        let edges = vec![Edge { kind: "resolves".to_string(), from: "d99".to_string(), to: "i9".to_string() }];
        let model = Model { items, edges };
        let res = compute_issue_resolution(&model, &HashSet::new());
        assert!(!res[0].open, "accepted Decision resolves the issue");
        assert_eq!(res[0].resolvers[0].kind, "decision");
    }

    #[test]
    fn coverage_tiers_and_transitive_verification() {
        // D0082 tiers: d1 charter-dod -> ADDRESSED (work, not evidence); d2 stale charter -> suspect;
        // d3 chartered-not-done -> uncovered; d4 accept-event -> ATTESTED; sr1 none -> uncovered;
        // sr2 verify-edge passing -> VERIFIED; n1 satisfy sr2(verified) -> VERIFIED (transitive);
        // n2 satisfy sr1(uncovered) -> uncovered.
        let mut items = HashMap::new();
        let accepted = || {
            let mut a = HashMap::new();
            a.insert("status".to_string(), "accepted".to_string());
            a
        };
        for d in ["d1", "d2", "d3"] {
            items.insert(d.to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: accepted(), marker: None });
        }
        for sr in ["sr1", "sr2"] {
            items.insert(sr.to_string(), ItemInfo { type_name: "SystemRequirement".to_string(), attrs: HashMap::new(), marker: None });
        }
        items.insert("n1".to_string(), ItemInfo { type_name: "Need".to_string(), attrs: HashMap::new(), marker: None });
        items.insert("n2".to_string(), ItemInfo { type_name: "Need".to_string(), attrs: HashMap::new(), marker: None });
        items.insert("d4".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: accepted(), marker: None });
        let mut ev = HashMap::new();
        ev.insert("outcome".to_string(), "pass".to_string());
        items.insert("d4AcceptR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: ev, marker: None });
        items.insert("vt".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: HashMap::new(), marker: None });
        let mut vtres = HashMap::new();
        vtres.insert("outcome".to_string(), "pass".to_string());
        items.insert("vtR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: vtres, marker: None });
        let edges = vec![
            Edge { kind: "charteredby".to_string(), from: "aStory".to_string(), to: "d1".to_string() },
            Edge { kind: "charteredby".to_string(), from: "bStory".to_string(), to: "d2".to_string() },
            Edge { kind: "charteredby".to_string(), from: "cStory".to_string(), to: "d3".to_string() },
            Edge { kind: "verify".to_string(), from: "vt".to_string(), to: "sr2".to_string() },
            Edge { kind: "satisfy".to_string(), from: "n1".to_string(), to: "sr2".to_string() },
            Edge { kind: "satisfy".to_string(), from: "n2".to_string(), to: "sr1".to_string() },
        ];
        let model = Model { items, edges };
        let done: HashSet<String> = ["a", "b", "vt"].iter().map(|s| (*s).to_string()).collect();
        let task_suspect: HashSet<String> = std::iter::once("b".to_string()).collect();
        let stale: HashSet<String> = HashSet::new();
        let cov = compute_coverage(&model, &done, &task_suspect, &stale);
        let get = |name: &str| cov.iter().find(|c| c.element == name).unwrap();
        assert_eq!((get("d1").tier, get("d1").basis), ("addressed", Some("charter-dod")));
        assert_eq!(get("d2").tier, "suspect");
        assert_eq!(get("d3").tier, "uncovered");
        assert_eq!(get("d4").tier, "attested");
        assert_eq!(get("sr1").tier, "uncovered");
        assert_eq!((get("sr2").tier, get("sr2").basis), ("verified", Some("explicit-test")));
        assert_eq!((get("n1").tier, get("n1").basis), ("verified", Some("satisfy")));
        assert_eq!(get("n2").tier, "uncovered");
    }

    #[test]
    fn critique_coverage_requires_independent_lens_critiques() {
        // sr1: completeness critiqued by an independent critic; correctness self-critiqued (author)
        // -> NOT counted; testability uncritiqued. So sr1 is uncovered (only 1/3 required lenses).
        let mut items = HashMap::new();
        let mut req = HashMap::new();
        req.insert("createdBy".to_string(), "wweatherholtz".to_string());
        items.insert("sr1".to_string(), ItemInfo { type_name: "SystemRequirement".to_string(), attrs: req, marker: None });
        let crit = |lens: &str| {
            let mut a = HashMap::new();
            a.insert("method".to_string(), "critique".to_string());
            a.insert("lens".to_string(), lens.to_string());
            a
        };
        items.insert("c1".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit("completeness"), marker: None });
        items.insert("c2".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit("correctness"), marker: None });
        let res = |by: &str| {
            let mut a = HashMap::new();
            a.insert("outcome".to_string(), "pass".to_string());
            a.insert("judgedBy".to_string(), by.to_string());
            a
        };
        items.insert("c1R1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("claudeOpus"), marker: None });
        items.insert("c2R1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("wweatherholtz"), marker: None });
        let edges = vec![
            Edge { kind: "verify".to_string(), from: "c1".to_string(), to: "sr1".to_string() },
            Edge { kind: "verify".to_string(), from: "c2".to_string(), to: "sr1".to_string() },
        ];
        let model = Model { items, edges };
        let cov = compute_critique_coverage(&model, &HashSet::<String>::new());
        let sr1 = cov.iter().find(|c| c.element == "sr1").unwrap();
        assert!(!sr1.covered, "only 1/3 required lenses independently critiqued");
        let lens = |n: &str| sr1.lenses.iter().find(|l| l.lens == n).unwrap();
        assert!(lens("completeness").critiqued, "independent critic counts");
        assert!(!lens("correctness").critiqued, "self-critique (author) does NOT count");
        assert!(!lens("testability").critiqued, "no critique recorded");
    }

    #[test]
    fn critique_suspect_flags_unresolved_failing_critique() {
        // D0086: an element with a failing critique is suspect; a passing critique is not.
        let mut items = HashMap::new();
        items.insert("d1".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: HashMap::new(), marker: None });
        items.insert("d2".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: HashMap::new(), marker: None });
        let crit = || {
            let mut a = HashMap::new();
            a.insert("method".to_string(), "critique".to_string());
            a
        };
        items.insert("cFail".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit(), marker: None });
        items.insert("cPass".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit(), marker: None });
        let res = |o: &str| {
            let mut a = HashMap::new();
            a.insert("outcome".to_string(), o.to_string());
            a
        };
        items.insert("cFailR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("fail"), marker: None });
        items.insert("cPassR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("pass"), marker: None });
        let edges = vec![
            Edge { kind: "verify".to_string(), from: "cFail".to_string(), to: "d1".to_string() },
            Edge { kind: "verify".to_string(), from: "cPass".to_string(), to: "d2".to_string() },
        ];
        let model = Model { items, edges };
        assert_eq!(critique_suspect_set(&model), vec!["d1".to_string()], "only the failing-critique element is suspect");
    }

    #[test]
    fn dispositioned_finding_does_not_block_readiness() {
        // Regression (D0047/D0092): assured must treat a >= Medium finding carrying a TYPED
        // #Dispositions verdict (ACT'd) as dispositioned, not undispositioned; only a finding with
        // NO disposition blocks. Open Critical always blocks regardless of disposition. D0092 reads
        // the typed verdict (not the prior resolver-presence proxy).
        let mk = |sev: &str| {
            let mut a = HashMap::new();
            a.insert("severity".to_string(), sev.to_string());
            ItemInfo { type_name: "Issue".to_string(), attrs: a, marker: None }
        };
        let disp = |verdict: &str| {
            let mut a = HashMap::new();
            a.insert("disposition".to_string(), verdict.to_string());
            ItemInfo { type_name: "Test".to_string(), attrs: a, marker: None }
        };
        let mut items = HashMap::new();
        items.insert("iActed".to_string(), mk("Medium"));
        items.insert("iActedDisp1".to_string(), disp("act"));
        items.insert("iRaw".to_string(), mk("Medium"));
        items.insert("iCrit".to_string(), mk("Critical"));
        items.insert("iCritDisp1".to_string(), disp("act"));
        // iActed + iCrit carry a typed ACT disposition; only iRaw is undispositioned.
        let edges = vec![
            Edge { kind: "dispositions".to_string(), from: "iActedDisp1".to_string(), to: "iActed".to_string() },
            Edge { kind: "dispositions".to_string(), from: "iCritDisp1".to_string(), to: "iCrit".to_string() },
        ];
        let model = Model { items, edges };
        let res = vec![
            IssueStatus { issue: "iActed".to_string(), resolvers: vec![ResolverStatus { name: "act".to_string(), kind: "action", complete: false }], open: true },
            IssueStatus { issue: "iRaw".to_string(), resolvers: Vec::new(), open: true },
            IssueStatus { issue: "iCrit".to_string(), resolvers: Vec::new(), open: true },
        ];
        let (undisp, critical) = finding_blockers(&res, &model);
        assert_eq!(undisp, vec!["iRaw".to_string()], "only the un-dispositioned Medium finding blocks (iActed has a typed verdict)");
        assert_eq!(critical, vec!["iCrit".to_string()], "open Critical blocks even when dispositioned");
    }

    #[test]
    fn guard_producing_heuristic() {
        // D0047/issue039: the resolver naming convention the defect-guard-coverage diagnostic keys on.
        for ok in ["ceremonyGateGuard", "critiqueRigorCheck", "criticIndependenceRule", "coverageAudits", "manifestCoverageGuard"] {
            assert!(is_guard_producing(ok), "{ok} should read as guard-producing");
        }
        for not in ["frictionMetric", "sittingModel", "reportIndicatorRender"] {
            assert!(!is_guard_producing(not), "{not} should NOT read as guard-producing");
        }
    }

    #[test]
    fn sitting_coverage_detects_covered_sprints() {
        // D0049/issue040: a #Covers edge (review -> sprint Story) marks that sprint covered.
        let mut items = HashMap::new();
        let story = |t: &str| ItemInfo { type_name: t.to_string(), attrs: HashMap::new(), marker: None };
        items.insert("s1".to_string(), story("Story"));
        items.insert("s2".to_string(), story("Story"));
        items.insert("sittingRev1".to_string(), story("Test"));
        let edges = vec![Edge { kind: "covers".to_string(), from: "sittingRev1".to_string(), to: "s1".to_string() }];
        let model = Model { items, edges };
        let covered = covered_sprints(&model);
        assert!(covered.contains("s1"), "s1 is covered by the sitting review");
        assert!(!covered.contains("s2"), "s2 has no covering review");
        assert_eq!(covered.len(), 1);
    }

    #[test]
    fn indicator_status_is_direction_aware() {
        // D0089: "better" depends on goal — maximize wants up, minimize wants down, observe is neutral.
        assert_eq!(indicator_status("maximize", 73.0, 100.0), "improving");
        assert_eq!(indicator_status("maximize", 100.0, 73.0), "degrading");
        assert_eq!(indicator_status("minimize", 4.0, 2.0), "improving");
        assert_eq!(indicator_status("minimize", 2.0, 4.0), "degrading");
        assert_eq!(indicator_status("maximize", 5.0, 5.0), "flat");
        assert_eq!(indicator_status("observe", 42.0, 47.0), "observed");
    }

    #[test]
    fn headline_labels_are_stable() {
        // D0087 Stage 2: each report's git-trend headline metric has a fixed label.
        assert_eq!(headline_label("assurance"), "Verification coverage %");
        assert_eq!(headline_label("traceability"), "Requirements verified %");
        assert_eq!(headline_label("quality-debt"), "Supersede edges (volatility)");
        assert_eq!(headline_label("flow"), "Delivered points (burnup)");
        assert_eq!(headline_label("governance"), "Accepted decisions");
    }

    #[test]
    fn low_rigor_reason_flags_shallow_critiques() {
        // D0080/issue030: too-short or structure-less critiques are low-rigor; a substantive
        // adversarial one passes.
        assert!(low_rigor_reason("too short").is_some());
        let no_struct = "x".repeat(200);
        assert_eq!(low_rigor_reason(&no_struct), Some("no ATTACK/FINDING/SURVIVED adversarial structure"));
        let good = format!("ATTACK: is the edge direction right? SURVIVED: verified against the schema. {}", "y".repeat(80));
        assert_eq!(low_rigor_reason(&good), None);
    }

    #[test]
    fn pct_handles_empty_denominator() {
        assert_eq!(pct(0, 0), 100, "nothing to measure = vacuously complete");
        assert_eq!(pct(1, 2), 50);
        assert_eq!(pct(9, 10), 90);
        assert_eq!(cov_tone(90), "good");
        assert_eq!(cov_tone(80), "warn");
        assert_eq!(cov_tone(50), "bad");
    }

    #[test]
    fn report_produces_cards_and_rejects_unknown() {
        // D0087: each report yields a non-empty cards array; unknown report errors. (cwd = crate dir.)
        let root = std::path::Path::new("..");
        for name in ["assurance", "traceability", "quality-debt", "flow", "governance", "friction"] {
            let json = report(root, name, false).unwrap_or_else(|e| panic!("report {name}: {e}"));
            assert!(json.contains("\"cards\""), "{name} has cards");
            assert!(json.contains("\"tone\""), "{name} cards carry a tone");
        }
        assert!(report(root, "bogus", false).is_err(), "unknown report errors");
        let html = report_html(root, "assurance", false).expect("assurance html");
        assert!(html.contains("class=\"cards\"") && !html.contains("/*CARDS*/"), "scorecard cards injected");
    }

    #[test]
    fn render_dispatches_modes_and_rejects_unknown() {
        // D0086: graph/table/review render; unknown mode errors. (cwd = crate dir in tests; the
        // declared view files live one level up at the repo root.)
        let root = std::path::Path::new("..");
        let g = render_html(root, "model", "graph").expect("graph");
        assert!(g.contains("Cytoscape Consortium"), "graph uses the inlined cytoscape lib");
        let t = render_html(root, "decisions", "table").expect("table");
        assert!(t.contains("<table") && !t.contains("/*ROWS*/") && !t.contains("/*STYLE*/"), "table rows + style injected");
        let r = render_html(root, "decisions", "review").expect("review");
        assert!(r.contains("exportJSON") && r.contains("apply-review"), "review mode has capture/export");
        assert!(render_html(root, "decisions", "bogus").is_err(), "unknown mode errors");
    }

    #[test]
    fn diagram_is_self_contained_and_layouts_visible_only() {
        // issue028 regression: the diagram must (1) inline cytoscape (no CDN — works offline) and
        // (2) lay out only the VISIBLE subset (cose on all ~2500 nodes froze the browser = blank).
        let html = diagram_html(std::path::Path::new(".")).expect("diagram_html");
        assert!(!html.contains("unpkg.com") && !html.contains("cdn"), "no CDN dependency — must be self-contained");
        assert!(html.contains("Cytoscape Consortium"), "cytoscape.js must be inlined");
        assert!(html.contains(":visible').layout"), "must lay out only visible nodes (no cose-on-all freeze)");
        assert!(!html.contains("/*CYTOSCAPE_LIB*/") && !html.contains("/*ELEMENTS*/"), "all template placeholders replaced");
    }

    #[test]
    fn decision_name_and_mentions_helpers() {
        // find_decision_name must skip the leading comment lines (the `?`-returns-None bug).
        let text = "// header comment\n// another\npackage Decision0048 {\n    part d0048 : Decision { :>> status = DecisionStatus::accepted; }\n}\n";
        assert_eq!(find_decision_name(text), Some("d0048".to_string()));
        // count_mentions counts both d/D forms; supersede_near needs the name + a verb on one line.
        let other = "D0048's parity_check is retired here.\nThis decision supersedes d0048 entirely.";
        assert_eq!(count_mentions(other, "d0048"), 2);
        assert!(supersede_near(other, "d0048"));
        assert!(!supersede_near("just mentions d0048 in passing", "d0048"));
    }

    #[test]
    fn charter_forms_bridges_story_names() {
        let forms = charter_forms("frontierCleanupsStory");
        assert!(forms.contains(&"frontierCleanups".to_string()), "backlog-action form");
        assert!(forms.contains(&"storyFrontierCleanups".to_string()), "delivery-action form");
    }

    #[test]
    fn attestation_flags_accepted_without_event() {
        // model()'s d1 is an accepted Decision with no d1AcceptR1 -> flagged missing.
        let (total, missing) = compute_attestation(&model());
        assert_eq!(total, 1);
        assert_eq!(missing, vec!["d1".to_string()]);
    }

    #[test]
    fn select_by_marker() {
        // M2.0: a process-change Decision is selectable by its #ProspectiveChange marker.
        let sel = Select { marker: Some(AttrPred::One("ProspectiveChange".to_string())), ..Default::default() };
        let got = selects(&model(), &sel);
        assert_eq!(got.len(), 1);
        assert!(got.contains("d1"));
    }

    #[test]
    fn select_attr_in_set() {
        let mut attrs = HashMap::new();
        attrs.insert("status".to_string(), AttrPred::Many(vec!["accepted".to_string(), "superseded".to_string()]));
        let sel = Select { type_: Some("Decision".to_string()), attrs, ..Default::default() };
        assert_eq!(selects(&model(), &sel).len(), 1);
    }

    #[test]
    fn traverse_follows_satisfy_down() {
        let m = model();
        let seed: HashSet<String> = std::iter::once("r1".to_string()).collect();
        let tr = Traverse {
            edges: vec!["satisfy".to_string()],
            direction: Direction::Down,
            depth: Depth::default(),
            target: None,
        };
        let got = traverse(&m, &seed, &tr, &["satisfy".to_string()]);
        assert!(got.contains("c1"), "satisfy edge should reach c1 from r1");
        assert!(got.contains("r1"), "seed retained");
    }

    #[test]
    fn unknown_edge_is_rejected() {
        let tr = Traverse {
            edges: vec!["bogus".to_string()],
            direction: Direction::Both,
            depth: Depth::default(),
            target: None,
        };
        assert!(validate_edges("v", &tr).is_err());
    }

    #[test]
    fn resolves_edge_is_known() {
        // D0077: #Resolves is a recognized edge (issue-resolution loop); a traverse over it
        // must validate (not fail-loud as unknown). Case-insensitive, like the others.
        let tr = Traverse {
            edges: vec!["Resolves".to_string()],
            direction: Direction::Both,
            depth: Depth::default(),
            target: None,
        };
        assert_eq!(validate_edges("v", &tr).unwrap(), vec!["resolves".to_string()]);
    }

    #[test]
    fn toml_rejects_unknown_field() {
        let bad = "name=\"x\"\n[select]\ntype=\"Story\"\nbogusfield=1\n";
        assert!(toml::from_str::<ViewSpec>(bad).is_err());
    }
}
