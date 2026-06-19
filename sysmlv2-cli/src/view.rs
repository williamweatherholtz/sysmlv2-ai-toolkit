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
    "allocate",
    "dependency",
    "ordering",
    "charteredby",
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
                    for p in &ad.parts {
                        add_item(items, &p.name, p.type_name.as_deref(), &p.attributes, p.marker.as_deref());
                    }
                    for v in &ad.verifications {
                        add_item(items, &v.name, v.type_name.as_deref(), &v.attributes, None);
                    }
                    for a in &ad.actions {
                        add_item_typed(items, &a.name, "action");
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
    }
}

fn add_item(items: &mut HashMap<String, ItemInfo>, name: &str, type_name: Option<&str>, attributes: &[sysmlv2_parser::ast::Attribute], marker: Option<&str>) {
    let attrs = attributes.iter().map(|a| (a.name.clone(), value_to_string(&a.value))).collect();
    items.insert(name.to_string(), ItemInfo { type_name: type_name.unwrap_or("").to_string(), attrs, marker: marker.map(str::to_string) });
}

fn add_item_typed(items: &mut HashMap<String, ItemInfo>, name: &str, type_name: &str) {
    items.entry(name.to_string()).or_insert_with(|| ItemInfo { type_name: type_name.to_string(), attrs: HashMap::new(), marker: None });
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
    Ok(emit_json(&spec, &model, &result))
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
    fn toml_rejects_unknown_field() {
        let bad = "name=\"x\"\n[select]\ntype=\"Story\"\nbogusfield=1\n";
        assert!(toml::from_str::<ViewSpec>(bad).is_err());
    }
}
