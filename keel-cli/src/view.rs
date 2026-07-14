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

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::path::Path;

use serde::Deserialize;
use keel_parser::ast::{Item, Package, Value};
use keel_parser::{parse, tokenize};

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
    #[error("invalid critique policy: {0}")]
    Policy(String),
    #[error("unknown element '{0}' (no authored item by that name)")]
    UnknownElement(String),
    #[error("section needs exactly one seed: a view name or an element name")]
    BadSection,
    #[error("element '{0}' is a {1}, not a Need (a boundary seed must be a Need)")]
    NotANeed(String, String),
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
    /// Repo-relative source file (forward-slashed) — powers the `newlyAdded` git-temporal rule scope
    /// (D0105). Empty for items constructed in tests / without a known source.
    file: String,
}

#[derive(Clone)]
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
            root.join(".engine").join("rules"),   // D0105: declared EdgeRule/ElementRule/OrderingRule instances (`keel check`)
        ];
        let mut items: HashMap<String, ItemInfo> = HashMap::new();
        let mut edges: Vec<Edge> = Vec::new();
        let paths: Vec<_> = dirs.iter().flat_map(|d| crate::collect_sysml(d)).collect();
        for path in paths {
            let name = path.display().to_string();
            let src = std::fs::read_to_string(&path).map_err(|e| ViewError::Io(name.clone(), e))?;
            let tokens = tokenize(&src, &name).map_err(|e| ViewError::Track(name.clone(), e.to_string()))?;
            let pkg = parse(tokens, &name).map_err(|e| ViewError::Track(name.clone(), e.to_string()))?;
            // Repo-relative, forward-slashed path — matches `git diff --name-only` for `newlyAdded` scope.
            let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string().replace('\\', "/");
            Self::ingest(&pkg, &mut items, &mut edges, &rel);
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

    fn ingest(pkg: &Package, items: &mut HashMap<String, ItemInfo>, edges: &mut Vec<Edge>, file: &str) {
        for item in &pkg.items {
            match item {
                Item::Part(p) => add_item(items, &p.name, p.type_name.as_deref(), &p.attributes, p.marker.as_deref(), file),
                Item::Verification(v) => add_item(items, &v.name, v.type_name.as_deref(), &v.attributes, None, file),
                Item::ActionDecl(a) => add_item_typed(items, &a.name, "action", file),
                Item::ActionDef(ad) => {
                    add_item_typed(items, &ad.name, "ActionDef", file);
                    // `contains` edges: a def structurally owns its nested parts/verifications/actions.
                    // This containment is real structure the flat item map loses; the diagram draws it
                    // so the nested children connect to their def instead of floating.
                    for p in &ad.parts {
                        add_item(items, &p.name, p.type_name.as_deref(), &p.attributes, p.marker.as_deref(), file);
                        edges.push(Edge { kind: "contains".to_string(), from: ad.name.clone(), to: p.name.clone() });
                    }
                    for v in &ad.verifications {
                        add_item(items, &v.name, v.type_name.as_deref(), &v.attributes, None, file);
                        edges.push(Edge { kind: "contains".to_string(), from: ad.name.clone(), to: v.name.clone() });
                    }
                    for a in &ad.actions {
                        add_item_typed(items, &a.name, "action", file);
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

fn add_item(items: &mut HashMap<String, ItemInfo>, name: &str, type_name: Option<&str>, attributes: &[keel_parser::ast::Attribute], marker: Option<&str>, file: &str) {
    let attrs = attributes.iter().map(|a| (a.name.clone(), value_to_string(&a.value))).collect();
    items.insert(name.to_string(), ItemInfo { type_name: type_name.unwrap_or("").to_string(), attrs, marker: marker.map(str::to_string), file: file.to_string() });
}

fn add_item_typed(items: &mut HashMap<String, ItemInfo>, name: &str, type_name: &str, file: &str) {
    items.entry(name.to_string()).or_insert_with(|| ItemInfo { type_name: type_name.to_string(), attrs: HashMap::new(), marker: None, file: file.to_string() });
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

/// The local neighbourhood of `element` (sr18ServeSectionCritique): the element itself plus every
/// element exactly one typed edge away, in either direction. This is the element-seeded section bound
/// — a subgraph small enough for local "does X make sense in its context" critique, where whole-model
/// views are too coarse (the requirement's rationale).
fn element_neighbourhood(model: &Model, element: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    set.insert(element.to_string());
    for e in &model.edges {
        if e.from == element {
            set.insert(e.to.clone());
        } else if e.to == element {
            set.insert(e.from.clone());
        }
    }
    set
}

/// A Need-SLICE boundary (sr19ServeWhiteboxBoundary).
///
/// The Need + the `SystemRequirement`s that satisfy it + the Components those SRs are allocated to + the
/// Tests verifying any element in the slice — a vertical "system" taken from the traceability structure
/// (D0100 — boundaries from existing structure, not graph clustering). Each is a recursive
/// System-of-Interest: critique its internals (white-box) and its cut edges (black-box, [`cut_edges`]).
fn need_slice(model: &Model, need: &str) -> HashSet<String> {
    let mut slice = HashSet::new();
    slice.insert(need.to_string());
    // SystemRequirements satisfying the need: a `satisfy` edge need -> sr.
    let srs: Vec<String> = model.edges.iter().filter(|e| e.kind == "satisfy" && e.from == need).map(|e| e.to.clone()).collect();
    for sr in &srs {
        slice.insert(sr.clone());
        // Components allocated from that SR: an `allocate` edge sr -> component.
        for e in &model.edges {
            if e.kind == "allocate" && &e.from == sr {
                slice.insert(e.to.clone());
            }
        }
    }
    // Tests verifying any element already in the slice: a `verify` edge test -> element.
    let current: HashSet<String> = slice.iter().cloned().collect();
    for e in &model.edges {
        if e.kind == "verify" && current.contains(&e.to) {
            slice.insert(e.from.clone());
        }
    }
    slice
}

/// The INTERFACES of a boundary (sr19 black-box): the cut edges — those with exactly ONE endpoint
/// inside `boundary` (crossing the System-of-Interest boundary). The count is a coupling signal; each is
/// a candidate interface finding (recorded as an Issue referencing the edge, D0100 — edges stay
/// lightweight, no port).
fn cut_edges(model: &Model, boundary: &HashSet<String>) -> Vec<Edge> {
    model.edges.iter().filter(|e| boundary.contains(&e.from) != boundary.contains(&e.to)).cloned().collect()
}

/// Emit a Need-slice BOUNDARY as JSON (sr19): the internal elements (white-box targets) + the interface
/// cut edges (black-box targets, each naming its external endpoint) + the coupling count.
fn boundary_emit_json(model: &Model, need: &str, slice: &HashSet<String>, cut: &[Edge]) -> String {
    let mut names: Vec<&String> = slice.iter().collect();
    names.sort();
    let items: Vec<Json> = names
        .iter()
        .filter_map(|n| {
            model.items.get(*n).map(|info| {
                let mut o = vec![
                    ("name".to_string(), Json::s((*n).clone())),
                    ("type".to_string(), Json::s(info.type_name.clone())),
                ];
                if let Some(t) = info.attrs.get("title") {
                    o.push(("title".to_string(), Json::s(t.clone())));
                }
                Json::Obj(o)
            })
        })
        .collect();
    let interfaces: Vec<Json> = cut
        .iter()
        .map(|e| {
            let (internal_end, external) = if slice.contains(&e.from) { (e.from.clone(), e.to.clone()) } else { (e.to.clone(), e.from.clone()) };
            Json::Obj(vec![
                ("kind".to_string(), Json::s(e.kind.clone())),
                ("from".to_string(), Json::s(e.from.clone())),
                ("to".to_string(), Json::s(e.to.clone())),
                ("internal".to_string(), Json::s(internal_end)),
                ("external".to_string(), Json::s(external)),
            ])
        })
        .collect();
    Json::Obj(vec![
        ("need".to_string(), Json::s(need.to_string())),
        ("internal_count".to_string(), Json::Int(i64::try_from(items.len()).unwrap_or(0))),
        ("coupling".to_string(), Json::Int(i64::try_from(cut.len()).unwrap_or(0))),
        ("internal".to_string(), Json::Arr(items)),
        ("interfaces".to_string(), Json::Arr(interfaces)),
    ])
    .dump()
}

/// Compute a Need-slice BOUNDARY as JSON (sr19ServeWhiteboxBoundary): the white-box internal element set
/// + the black-box interface cut edges + the coupling count. A computed `#View`.
///
/// # Errors
/// Returns [`ViewError`] for an unknown element, a non-Need seed, or a parse failure.
pub fn boundary_json(root: &Path, need: &str) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    match model.items.get(need) {
        Some(i) if i.type_name == "Need" => {}
        Some(i) => return Err(ViewError::NotANeed(need.to_string(), i.type_name.clone())),
        None => return Err(ViewError::UnknownElement(need.to_string())),
    }
    let slice = need_slice(&model, need);
    let cut = cut_edges(&model, &slice);
    Ok(boundary_emit_json(&model, need, &slice, &cut))
}

/// The interface descriptions of a Need-slice boundary (sr19 black-box) — one `"<kind> <internal> -> <external>"`
/// string per cut edge, for naming the interfaces in a black-box critique prompt.
///
/// # Errors
/// Returns [`ViewError`] for an unknown element, a non-Need seed, or a parse failure.
pub fn boundary_interfaces(root: &Path, need: &str) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    match model.items.get(need) {
        Some(i) if i.type_name == "Need" => {}
        Some(i) => return Err(ViewError::NotANeed(need.to_string(), i.type_name.clone())),
        None => return Err(ViewError::UnknownElement(need.to_string())),
    }
    let slice = need_slice(&model, need);
    let mut ifaces: Vec<String> = cut_edges(&model, &slice)
        .iter()
        .map(|e| {
            let (internal_end, external) = if slice.contains(&e.from) { (&e.from, &e.to) } else { (&e.to, &e.from) };
            format!("{} {internal_end} -> {external}", e.kind)
        })
        .collect();
    ifaces.sort();
    Ok(ifaces)
}

/// The tier-satisfaction white-box SWEEP (sr19; the D0098 first sweep target).
///
/// Per Need: the slice size, coupling (interface cut-edge count), SR count, and whether the Need is
/// decomposed (>=1 SR) and its SRs all verified — a per-boundary comprehensiveness reading. A computed `#View`.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn boundary_sweep_json(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut needs: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Need").map(|(n, _)| n).collect();
    needs.sort();
    let rows: Vec<Json> = needs
        .iter()
        .map(|n| {
            let slice = need_slice(&model, n);
            let cut = cut_edges(&model, &slice);
            let srs: Vec<&String> = model.edges.iter().filter(|e| e.kind == "satisfy" && &e.from == *n).map(|e| &e.to).collect();
            let verified = !srs.is_empty() && srs.iter().all(|sr| model.edges.iter().any(|e| e.kind == "verify" && &e.to == *sr));
            Json::Obj(vec![
                ("need".to_string(), Json::s((*n).clone())),
                ("internal_count".to_string(), Json::Int(i64::try_from(slice.len()).unwrap_or(0))),
                ("coupling".to_string(), Json::Int(i64::try_from(cut.len()).unwrap_or(0))),
                ("sr_count".to_string(), Json::Int(i64::try_from(srs.len()).unwrap_or(0))),
                ("decomposed".to_string(), Json::Bool(!srs.is_empty())),
                ("srs_verified".to_string(), Json::Bool(verified)),
            ])
        })
        .collect();
    Ok(Json::Obj(vec![
        ("sweep".to_string(), Json::s("tier-satisfaction white-box sweep (per Need-slice)".to_string())),
        ("needs".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(0))),
        ("rows".to_string(), Json::Arr(rows)),
    ])
    .dump())
}

/// Emit a bounded section (`names`) of `model` as JSON: `{seed, kind, count, items[], edges[]}`. Items
/// carry name + type (+ title/marker when authored); edges are the INDUCED subgraph — only those whose
/// both endpoints are inside the section. Presentation-agnostic (the console renders it).
fn section_subgraph_json(model: &Model, names: &HashSet<String>, seed: &str, kind: &str) -> String {
    let mut sorted: Vec<&String> = names.iter().collect();
    sorted.sort();
    let items: Vec<Json> = sorted
        .iter()
        .filter_map(|n| {
            model.items.get(*n).map(|info| {
                let mut o = vec![
                    ("name".to_string(), Json::s((*n).clone())),
                    ("type".to_string(), Json::s(info.type_name.clone())),
                ];
                if let Some(t) = info.attrs.get("title") {
                    o.push(("title".to_string(), Json::s(t.clone())));
                }
                if let Some(m) = &info.marker {
                    o.push(("marker".to_string(), Json::s(m.clone())));
                }
                Json::Obj(o)
            })
        })
        .collect();
    let count = items.len();
    let edges: Vec<Json> = model
        .edges
        .iter()
        .filter(|e| names.contains(&e.from) && names.contains(&e.to))
        .map(|e| {
            Json::Obj(vec![
                ("kind".to_string(), Json::s(e.kind.clone())),
                ("from".to_string(), Json::s(e.from.clone())),
                ("to".to_string(), Json::s(e.to.clone())),
            ])
        })
        .collect();
    Json::Obj(vec![
        ("seed".to_string(), Json::s(seed.to_string())),
        ("kind".to_string(), Json::s(kind.to_string())),
        ("count".to_string(), Json::Int(i64::try_from(count).unwrap_or(0))),
        ("items".to_string(), Json::Arr(items)),
        ("edges".to_string(), Json::Arr(edges)),
    ])
    .dump()
}

/// Resolve a section seed to its bounded model + element set (sr18). Either a declared view's element
/// set (`view`), or an element plus its 1-hop typed-edge neighbourhood (`element`); exactly one seed.
/// Returns `(model, kind, seed, names)`.
fn resolve_section(root: &Path, view: Option<&str>, element: Option<&str>) -> Result<(Model, &'static str, String, HashSet<String>), ViewError> {
    match (view, element) {
        (Some(v), None) => {
            let (_, model, result) = run_resolved(root, v)?;
            Ok((model, "view", v.to_string(), result))
        }
        (None, Some(el)) => {
            let model = Model::build(root)?;
            if !model.items.contains_key(el) {
                return Err(ViewError::UnknownElement(el.to_string()));
            }
            let names = element_neighbourhood(&model, el);
            Ok((model, "element", el.to_string(), names))
        }
        _ => Err(ViewError::BadSection),
    }
}

/// Compute a bounded SECTION of the model as JSON (sr18ServeSectionCritique).
///
/// Either a declared view's element set (`view`), or an element plus its 1-hop typed-edge
/// neighbourhood (`element`). Exactly one seed must be supplied. A computed `#View`: regenerate on
/// demand, never store.
///
/// # Errors
/// Returns [`ViewError`] for a missing/invalid view, an unknown element, a parse failure, or a
/// malformed request (neither or both seeds supplied).
pub fn section_json(root: &Path, view: Option<&str>, element: Option<&str>) -> Result<String, ViewError> {
    let (model, kind, seed, names) = resolve_section(root, view, element)?;
    Ok(section_subgraph_json(&model, &names, &seed, kind))
}

/// The element names composing a section (sr18).
///
/// Same seed semantics as [`section_json`], returning the sorted bounded name set for callers that
/// need just the membership (e.g. section-scoped critique context — giving the AI the local
/// neighbourhood instead of a single isolated element).
///
/// # Errors
/// As [`section_json`].
pub fn section_member_names(root: &Path, view: Option<&str>, element: Option<&str>) -> Result<Vec<String>, ViewError> {
    let (_, _, _, names) = resolve_section(root, view, element)?;
    let mut v: Vec<String> = names.into_iter().collect();
    v.sort();
    Ok(v)
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
/// `served` = renderer names a `keel` command; `unserved` = renderer is `(planned ...)`.
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

/// Generic item-detail view (D0094 serveItemIntrospect): any item's type, attrs, and edges.
///
/// Returns the item's type + authored attrs + its incoming/outgoing typed edges (with the neighbor on
/// each) — one computation for every type (Decision/Issue/Process/Need/Story/...). `found:false` if unknown.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn item_detail(root: &Path, name: &str) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    Ok(item_detail_json(&model, name).dump())
}

/// The pure Model→JSON core of [`item_detail`] (extracted so it is unit-testable without a fixture
/// dir). Resolves the `<name>DoD` procedureText (issue064) as the task's `dod` description.
fn item_detail_json(model: &Model, name: &str) -> Json {
    let Some(info) = model.items.get(name) else {
        return Json::Obj(vec![("found".to_string(), Json::Bool(false)), ("name".to_string(), Json::s(name.to_string()))]);
    };
    let mut attr_keys: Vec<&String> = info.attrs.keys().collect();
    attr_keys.sort();
    let attrs: Vec<Json> = attr_keys
        .iter()
        .filter_map(|k| info.attrs.get(*k).map(|v| Json::Obj(vec![("key".to_string(), Json::s((*k).clone())), ("value".to_string(), Json::s(v.clone()))])))
        .collect();
    let edges_for = |outgoing: bool| -> Vec<Json> {
        let mut pairs: Vec<(String, String)> = model
            .edges
            .iter()
            .filter(|e| if outgoing { e.from == name } else { e.to == name })
            .map(|e| (e.kind.clone(), if outgoing { e.to.clone() } else { e.from.clone() }))
            .collect();
        pairs.sort();
        pairs.dedup();
        pairs.into_iter().map(|(kind, node)| Json::Obj(vec![("kind".to_string(), Json::s(kind)), ("node".to_string(), Json::s(node))])).collect()
    };
    // serveItemIntrospect (issue064): an `action <name>;` task carries NO authored attrs — its human
    // description lives in the `<name>DoD` verify Test's procedureText. Surface it (+ method) so the
    // console shows a task's real content instead of an empty shell + the structural NextWork edge.
    let dod = model.items.get(&format!("{name}DoD")).map_or(Json::Null, |d| {
        Json::Obj(vec![
            ("method".to_string(), Json::s(d.attrs.get("method").cloned().unwrap_or_default())),
            ("procedureText".to_string(), Json::s(d.attrs.get("procedureText").cloned().unwrap_or_default())),
        ])
    });
    Json::Obj(vec![
        ("found".to_string(), Json::Bool(true)),
        ("name".to_string(), Json::s(name.to_string())),
        ("type".to_string(), Json::s(info.type_name.clone())),
        ("marker".to_string(), info.marker.clone().map_or(Json::Null, Json::s)),
        ("attrs".to_string(), Json::Arr(attrs)),
        ("dod".to_string(), dod),
        ("outgoing".to_string(), Json::Arr(edges_for(true))),
        ("incoming".to_string(), Json::Arr(edges_for(false))),
    ])
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
            return Some(decode_string_body(rest));
        }
    }
    None
}

/// Decode a string-literal body (the text after the opening quote) up to the first unescaped quote,
/// applying the SAME escape rules as the lexer's `lex_string` (backslash-backslash, backslash-quote,
/// `\n`, `\t`). Without this, a raw git-blob read of an escaped field (e.g. a regex containing a
/// backslash) compares unequal to the parsed model value and the element's critiques are falsely
/// flagged stale (issue044) — undercounting critique coverage. Keeps blob-extract == model-attr.
fn decode_string_body(rest: &str) -> String {
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('"') => out.push('"'),
                Some('\\') | None => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
            },
            other => out.push(other),
        }
    }
    out
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
    out.sort_by_key(|c| (c.type_name.clone(), c.element.clone()));
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
// REQUIRED lens for its type has such a critique. The required-lens policy is DECLARED, not hardcoded
// (D0097): read from `.engine/contracts/critique-policy.toml` (downstream-overridable), with the
// "Core-3" default (Need/SystemRequirement -> completeness/correctness/testability; Decision ->
// completeness/correctness/feasibility) as the built-in fallback when the file is absent. Honest by
// construction: with no critiques recorded, every element is uncovered. (Git-temporal critique-
// staleness reuses the suspect machinery.)

/// The seven `CritiqueLens` variants (schema/core `element.sysml`) — the requirement-quality canon.
/// A declared policy lens MUST be one of these (fail-loud otherwise).
const CANON_LENSES: [&str; 7] =
    ["completeness", "correctness", "ambiguity", "testability", "feasibility", "consistency", "necessity"];

/// The declared critique policy (D0097): required critique lenses per assurance-element type.
///
/// Read from `.engine/contracts/critique-policy.toml`. A type with a non-empty lens list is a critique
/// TARGET; each listed lens needs an independent `method=critique` verification for an element of that
/// type to be critique-covered. Downstream projects override the file; absent it, the built-in Core-3
/// applies.
pub struct CritiquePolicy {
    lenses: BTreeMap<String, Vec<String>>,
    from_file: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CritiquePolicyFile {
    #[serde(default)]
    lenses: BTreeMap<String, Vec<String>>,
}

impl CritiquePolicy {
    /// The built-in Core-3 default (D0080) — identical to the shipped `critique-policy.toml`, used as
    /// the fallback when no policy file is present so behavior is unchanged with or without the file.
    fn core3() -> Self {
        let mut lenses = BTreeMap::new();
        let req = || vec!["completeness".to_string(), "correctness".to_string(), "testability".to_string()];
        lenses.insert("Need".to_string(), req());
        lenses.insert("SystemRequirement".to_string(), req());
        lenses.insert(
            "Decision".to_string(),
            vec!["completeness".to_string(), "correctness".to_string(), "feasibility".to_string()],
        );
        Self { lenses, from_file: false }
    }

    /// Load the declared policy from `.engine/contracts/critique-policy.toml`, falling back to the
    /// built-in Core-3 default when the file is absent. Validates every lens name against the canon.
    ///
    /// # Errors
    /// Returns [`ViewError::Toml`] if the file is malformed, or [`ViewError::Policy`] if it lists an
    /// unknown lens name.
    pub fn load(root: &Path) -> Result<Self, ViewError> {
        let path = root.join(".engine").join("contracts").join("critique-policy.toml");
        let Ok(text) = std::fs::read_to_string(&path) else { return Ok(Self::core3()) };
        let parsed: CritiquePolicyFile =
            toml::from_str(&text).map_err(|e| ViewError::Toml(path.display().to_string(), Box::new(e)))?;
        for (ty, lenses) in &parsed.lenses {
            for l in lenses {
                if !CANON_LENSES.contains(&l.as_str()) {
                    return Err(ViewError::Policy(format!(
                        "type '{ty}' lists unknown lens '{l}' (valid: {})",
                        CANON_LENSES.join(" | ")
                    )));
                }
            }
        }
        Ok(Self { lenses: parsed.lenses, from_file: true })
    }

    /// Lenient load for ADVISORY aggregate reports: falls back to the Core-3 default on any error. A
    /// malformed policy is surfaced loudly by `critique-coverage` / `guard critique` (same gate), so the
    /// report cards needn't re-raise it.
    fn load_or_core3(root: &Path) -> Self {
        Self::load(root).unwrap_or_else(|_| Self::core3())
    }

    /// Required critique lenses for an element type (empty slice for non-targets).
    fn required_lenses(&self, type_name: &str) -> &[String] {
        self.lenses.get(type_name).map_or(&[], Vec::as_slice)
    }

    /// Whether an element TYPE is a critique target (has >= 1 required lens declared).
    fn is_target_type(&self, type_name: &str) -> bool {
        self.lenses.get(type_name).is_some_and(|v| !v.is_empty())
    }

    /// The declared target types, sorted (the `BTreeMap` keeps them ordered).
    fn target_types(&self) -> impl Iterator<Item = &String> {
        self.lenses.iter().filter(|(_, v)| !v.is_empty()).map(|(k, _)| k)
    }
}

struct LensStatus {
    lens: String,
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

fn compute_critique_coverage<S: std::hash::BuildHasher>(
    model: &Model,
    stale: &HashSet<String, S>,
    policy: &CritiquePolicy,
) -> Vec<CritiqueCoverage> {
    // Targets = the policy's declared types (D0097). Decisions are critiqued only once accepted (an
    // accepted Decision is a final commitment) — that accepted-only rule is intrinsic, not config.
    let is_target = |i: &ItemInfo| {
        if !policy.is_target_type(&i.type_name) {
            return false;
        }
        if i.type_name == "Decision" {
            return i.attrs.get("status").map(String::as_str) == Some("accepted");
        }
        true
    };
    let mut targets: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| is_target(i)).collect();
    targets.sort_by(|a, b| a.0.cmp(b.0));
    targets
        .into_iter()
        .map(|(name, info)| {
            let author = info.attrs.get("createdBy").map_or("", String::as_str);
            let lenses: Vec<LensStatus> = policy
                .required_lenses(&info.type_name)
                .iter()
                .map(|lens| {
                    let lens = lens.as_str();
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
                    LensStatus { lens: lens.to_string(), critiqued, critic, outcome }
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
    let policy = CritiquePolicy::load(root)?;
    let gf = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let stale = compute_stale_verifications(root, &model);
    Ok(compute_critique_coverage(&model, &stale, &policy)
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
    // D0102: a fail critique whose finding Issue is dispositioned ACCEPT-RISK/DISMISS no longer induces
    // suspicion — the verdict consciously resolved it. The finding->critique link is the typed `#DependsOn`
    // edge from the Issue to the critique Test (so the computation has a typed path, not prose).
    let mut accepted: HashSet<String> = HashSet::new(); // critique Tests whose finding is accept-risk/dismiss
    for (iname, iinfo) in &model.items {
        if iinfo.type_name != "Issue" {
            continue;
        }
        if matches!(issue_disposition(model, iname).as_deref(), Some("acceptRisk" | "dismiss")) {
            for e in &model.edges {
                if e.kind == "dependson" && &e.from == iname {
                    accepted.insert(e.to.clone());
                }
            }
        }
    }
    let mut suspect: HashSet<String> = HashSet::new();
    for e in &model.edges {
        if e.kind != "verify" {
            continue;
        }
        let Some(src) = model.items.get(&e.from) else { continue };
        if src.attrs.get("method").map(String::as_str) != Some("critique") {
            continue;
        }
        if accepted.contains(&e.from) {
            continue; // D0102: this fail critique's finding was accept-risk'd / dismissed
        }
        if matches!(latest_result(model, &e.from), Some((ref o, _)) if o == "fail") {
            suspect.insert(e.to.clone());
        }
    }
    let mut out: Vec<String> = suspect.into_iter().collect();
    out.sort();
    out
}

/// True if `token` occurs in `haystack` as a whole identifier (not a substring of a longer one) — so
/// `sr1` does not match inside `sr15ServeIntrospect`. Used by [`decision_requirement_prose_links`].
fn contains_token(haystack: &str, token: &str) -> bool {
    let bytes = haystack.as_bytes();
    haystack.match_indices(token).any(|(i, _)| {
        let before_ok = i == 0 || bytes.get(i - 1).is_none_or(|b| !b.is_ascii_alphanumeric());
        let after_ok = bytes.get(i + token.len()).is_none_or(|b| !b.is_ascii_alphanumeric());
        before_ok && after_ok
    })
}

/// Decisions whose `context` OR `rationale` is blank/trivial (D0103): trimmed length < 20 chars.
///
/// Returns `(total decisions, weak names)`. A recorded decision without a substantive why is ill-formed
/// state — the `decision-rationale` hard guard reads this. (`decision`/`consequences` stay schema-required.)
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn decisions_weak_rationale(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    let decisions: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| i.type_name == "Decision").collect();
    let mut weak: Vec<String> = decisions
        .iter()
        .filter(|(_, info)| {
            let blank = |f: &str| info.attrs.get(f).is_none_or(|v| v.trim().len() < 20);
            blank("context") || blank("rationale")
        })
        .map(|(n, _)| (*n).clone())
        .collect();
    weak.sort();
    Ok((decisions.len(), weak))
}

/// Governance verbs (D0104): a Decision GOVERNS a requirement (vs merely mentioning it) when one of these
/// sits near the requirement name. Matched as a lowercase substring so inflections count (amended/descoped).
const GOV_VERBS: &[&str] = &["amend", "supersede", "descope", "revise", "cancel", "replace", "retire", "rescope", "moot"];

/// True if `window` cites a FOREIGN decision id (`dNNNN`/`DNNNN` whose digits != `own_digits`).
fn has_foreign_decision_id(window: &str, own_digits: &str) -> bool {
    let b = window.as_bytes();
    (0..b.len()).any(|i| {
        let id_start = matches!(b.get(i), Some(b'd' | b'D'))
            && b.get(i + 1..i + 5).is_some_and(|d| d.iter().all(u8::is_ascii_digit))
            && b.get(i + 5).is_none_or(|c| !c.is_ascii_digit())
            && (i == 0 || b.get(i - 1).is_none_or(|c| !c.is_ascii_alphanumeric()));
        id_start && b.get(i + 1..i + 5).is_some_and(|d| d.iter().map(|&c| c as char).collect::<String>() != own_digits)
    })
}

/// True if `window` (text around a requirement mention) reads as GOVERNANCE by the decision `own_digits`
/// (D0104): a governance verb is present AND no FOREIGN decision id is cited (a foreign id means the verb is
/// attributed to ANOTHER decision — a citation, not this decision's action).
fn is_governance_mention(window: &str, own_digits: &str) -> bool {
    let lower = window.to_lowercase();
    GOV_VERBS.iter().any(|v| lower.contains(v)) && !has_foreign_decision_id(window, own_digits)
}

/// Decision→requirement GOVERNANCE references that exist only in PROSE (D0102/D0104, the issue052 class).
///
/// For each accepted Decision, the Needs/SystemRequirements its text GOVERNS (a governance verb near the
/// exact name, no foreign decision id) but to which it carries NO typed edge — a governance link that
/// should be typed, not prose. Contextual mentions/examples are excluded (D0104). A computed `#View`.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn decision_requirement_prose_links(root: &Path) -> Result<Vec<(String, String)>, ViewError> {
    let model = Model::build(root)?;
    let reqs: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Need" || i.type_name == "SystemRequirement").map(|(n, _)| n).collect();
    let mut out: Vec<(String, String)> = Vec::new();
    for (dname, dinfo) in &model.items {
        if dinfo.type_name != "Decision" || dinfo.attrs.get("status").map(String::as_str) != Some("accepted") {
            continue;
        }
        let own_digits: String = dname.chars().filter(char::is_ascii_digit).take(4).collect();
        let text: String = ["context", "decision", "rationale", "consequences"].iter().filter_map(|f| dinfo.attrs.get(*f)).cloned().collect::<Vec<_>>().join(" ");
        for r in &reqs {
            if !contains_token(&text, r) {
                continue;
            }
            if model.edges.iter().any(|e| (&e.from == dname && e.to == **r) || (e.from == **r && &e.to == dname)) {
                continue; // already typed-linked
            }
            // D0104: flag only a GOVERNANCE mention (governance verb near R, no foreign decision id) — not a
            // contextual example or a description of another decision's action.
            let governs = text.match_indices(r.as_str()).any(|(i, _)| {
                let bytes = text.as_bytes();
                let boundary = (i == 0 || bytes.get(i - 1).is_none_or(|b| !b.is_ascii_alphanumeric())) && bytes.get(i + r.len()).is_none_or(|b| !b.is_ascii_alphanumeric());
                if !boundary {
                    return false;
                }
                let mut lo = i.saturating_sub(60);
                let mut hi = (i + r.len() + 60).min(text.len());
                while lo > 0 && !text.is_char_boundary(lo) {
                    lo -= 1;
                }
                while hi < text.len() && !text.is_char_boundary(hi) {
                    hi += 1;
                }
                is_governance_mention(text.get(lo..hi).unwrap_or(&text), &own_digits)
            });
            if governs {
                out.push((dname.clone(), (*r).clone()));
            }
        }
    }
    out.sort();
    Ok(out)
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
    let policy = CritiquePolicy::load(root)?;
    let stale = compute_stale_verifications(root, &model);
    let cov = compute_critique_coverage(&model, &stale, &policy);
    let gf = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let in_scope = |c: &CritiqueCoverage| governed(gf.as_ref(), &c.element);

    // Summary over GOVERNED elements only (the grandfathered ones aren't required); per the policy's
    // DECLARED target types (D0097), so a downstream-added target type is summarized too.
    let mut summary: Vec<Json> = Vec::new();
    for ty in policy.target_types() {
        let rows: Vec<&CritiqueCoverage> = cov.iter().filter(|c| &c.type_name == ty && in_scope(c)).collect();
        if rows.is_empty() {
            continue;
        }
        let covered = i64::try_from(rows.iter().filter(|c| c.covered).count()).unwrap_or(i64::MAX);
        summary.push(Json::Obj(vec![
            ("type".to_string(), Json::s(ty.clone())),
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
                        ("lens".to_string(), Json::s(l.lens.clone())),
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
            Json::s("critique-coverage: GOVERNED elements (created after D0080, charter-time D0081) of each declared target type (critique-policy.toml, D0097) x required lens -> an independent method=critique verification #Verify-linked to the element"),
        ),
        ("summary".to_string(), Json::Arr(summary)),
        ("gaps".to_string(), Json::Arr(gaps)),
        ("elements".to_string(), Json::Arr(elements)),
    ]);
    Ok(out.dump())
}

/// The ACTIVE critique policy (D0097) as JSON: the source (declared file vs built-in default) + the
/// required lenses per target type. Lets a project confirm an override took effect. Honest, computed.
///
/// # Errors
/// Returns [`ViewError::Toml`]/[`ViewError::Policy`] if the policy file is malformed or names an
/// unknown lens.
pub fn critique_policy(root: &Path) -> Result<String, ViewError> {
    let policy = CritiquePolicy::load(root)?;
    let types: Vec<Json> = policy
        .lenses
        .iter()
        .map(|(ty, lenses)| {
            Json::Obj(vec![
                ("type".to_string(), Json::s(ty.clone())),
                ("lenses".to_string(), Json::Arr(lenses.iter().map(|l| Json::s(l.clone())).collect())),
                ("target".to_string(), Json::Bool(!lenses.is_empty())),
            ])
        })
        .collect();
    let out = Json::Obj(vec![
        (
            "critique_policy".to_string(),
            Json::s("required antagonistic critique lenses per assurance-element type (D0097); a type with >=1 lens is a critique target"),
        ),
        (
            "source".to_string(),
            Json::s(if policy.from_file { ".engine/contracts/critique-policy.toml" } else { "built-in Core-3 default (no policy file)" }),
        ),
        ("canon".to_string(), Json::Arr(CANON_LENSES.iter().map(|l| Json::s(*l)).collect())),
        ("types".to_string(), Json::Arr(types)),
    ]);
    Ok(out.dump())
}

// ── requirement rootedness (D0098/issue047 — every chartered capability traces to a driving Need) ──
// UPWARD integrity (an HONESTY check, not completeness): a delivery Story must reach a Need through its
// #CharteredBy chain — directly (#CharteredBy a Need), via a SystemRequirement it charters to (the SR
// `satisfy`-traces to a Need), or via a Decision carrying a #DerivedFrom edge to a Need. A Story that
// reaches NO Need is UNROOTED: it ships work whose stakeholder justification is unstated (the serve
// class, issue046). Computed from authored edges; nothing stored.

fn rd_is_need(model: &Model, n: &str) -> bool {
    model.items.get(n).is_some_and(|i| i.type_name == "Need")
}
/// A `SystemRequirement` reaches a Need iff some Need `satisfy`-traces to it (satisfy edge Need->SR).
fn rd_sr_rooted(model: &Model, sr: &str) -> bool {
    model.edges.iter().any(|e| e.kind == "satisfy" && e.to == sr && rd_is_need(model, &e.from))
}
/// An item reaches a Need iff it carries a `#DerivedFrom`/`derive` edge to a Need.
fn rd_derives_need(model: &Model, item: &str) -> bool {
    model.edges.iter().any(|e| matches!(e.kind.as_str(), "derivedfrom" | "derive") && e.from == item && rd_is_need(model, &e.to))
}
/// Whether an item carries the `#Capability` marker (a user-facing feature — D0099).
fn rd_is_capability(model: &Model, item: &str) -> bool {
    model.items.get(item).and_then(|i| i.marker.as_deref()).is_some_and(|m| m.trim_start_matches('#').eq_ignore_ascii_case("capability"))
}

/// Charter class of a delivery Story (D0098 rootedness burndown).
///
/// `need` = its charter reaches a Need (directly, via a satisfy'd `SystemRequirement`, or via a
/// Decision `#DerivedFrom` a Need); `decision` = chartered by a Decision (legitimate decision-driven
/// engine evolution, D0064); `orphan` = no `#CharteredBy` edge at all.
fn rd_charter_class(model: &Model, story: &str) -> &'static str {
    let charters: Vec<&String> =
        model.edges.iter().filter(|e| e.kind == "charteredby" && e.from == story).map(|e| &e.to).collect();
    if charters.is_empty() {
        return "orphan";
    }
    let reaches_need = charters.iter().any(|t| {
        let tk = model.items.get(*t).map_or("", |i| i.type_name.as_str());
        tk == "Need" || (tk == "SystemRequirement" && rd_sr_rooted(model, t)) || rd_derives_need(model, t)
    });
    if reaches_need {
        "need"
    } else {
        "decision"
    }
}

/// `#Capability` items lacking a `#DerivedFrom` edge to a Need — the requirement-rootedness HARD gate
/// set (D0099): a declared user-facing capability whose driving Need is unstated. Sorted. (Unmarked
/// work is exempt — decision-driven engine evolution is legitimate, D0064.)
fn capability_root_violations(model: &Model) -> Vec<String> {
    let mut out: Vec<String> = model
        .items
        .keys()
        .filter(|name| rd_is_capability(model, name) && !rd_derives_need(model, name))
        .cloned()
        .collect();
    out.sort();
    out
}

/// `#Capability` items with no `#DerivedFrom`->Need link (the rootedness gap set for `guard requirement-rootedness`).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn rootedness_gaps(root: &Path) -> Result<Vec<String>, ViewError> {
    Ok(capability_root_violations(&Model::build(root)?))
}

// ── `keel check` (D0105 EXPAND step 2): the generic evaluator over DECLARED rules ────────────────

/// Does `info` match an `EdgeRule` `subjectType` — a `#Marker` (marker match) or a bare type name?
fn rc_matches_subject(info: &ItemInfo, subject: &str) -> bool {
    subject.strip_prefix('#').map_or_else(
        || info.type_name == subject,
        |marker| info.marker.as_deref().is_some_and(|m| m.trim_start_matches('#').eq_ignore_ascii_case(marker)),
    )
}

/// `EdgeRule` violations: `subject` instances lacking `edge` (at `cardinality`) to an existing instance
/// of `object` (`"*"` = any target). Sorted subject names. The generic core that subsumes the ~9
/// conformance guards once each rule reaches parity.
/// Repo-relative, forward-slashed files git reports as newly-ADDED in the staged index — the `newlyAdded`
/// scope set (matches the charter/sprint-coverage guards' forward-only semantics). Empty if git fails.
fn staged_added_files(root: &Path) -> std::collections::HashSet<String> {
    std::process::Command::new("git")
        .arg("-C").arg(root)
        .args(["diff", "--cached", "--name-only", "--diff-filter=A"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().map(|l| l.trim().replace('\\', "/")).collect())
        .unwrap_or_default()
}

fn edge_rule_violations(model: &Model, subject: &str, edge: &str, object: &str, direction: &str, cardinality: &str, scope_files: Option<&std::collections::HashSet<String>>) -> Vec<String> {
    let incoming = direction == "incoming";
    let mut out: Vec<String> = Vec::new();
    for (name, info) in &model.items {
        if !rc_matches_subject(info, subject) {
            continue;
        }
        // `newlyAdded` scope: only subjects whose source file is in the staged-added set.
        if scope_files.is_some_and(|files| !files.contains(&info.file)) {
            continue;
        }
        let count = model
            .edges
            .iter()
            .filter(|e| {
                let (near, far) = if incoming { (&e.to, &e.from) } else { (&e.from, &e.to) };
                e.kind == edge
                    && near == name
                    && (object == "*" || model.items.get(far).is_some_and(|t| t.type_name == object))
            })
            .count();
        let ok = if cardinality == "exactlyOne" { count == 1 } else { count >= 1 };
        if !ok {
            out.push(name.clone());
        }
    }
    out.sort();
    out
}

/// `strip_prefix(p)` then `strip_suffix(')')` — pulls `args` out of a `fn(args)` predicate term.
fn predicate_args<'a>(t: &'a str, p: &str) -> Option<&'a str> {
    t.strip_prefix(p).and_then(|r| r.strip_suffix(')'))
}

/// Is `name` within a rule's `appliesWhen` SCOPE? `all` (always), `whereStatus(v)` (the item's `status`
/// attr == v), `whereKind(v)` (the item's `kind` `WorkKind` == v). `Some(bool)`, or `None` if the scope
/// predicate is unsupported (caller marks the rule not-evaluated). Git-temporal scopes (`newlyAdded`)
/// are a later sub-step, reported unsupported here.
fn subject_in_scope(info: &ItemInfo, scope: &str) -> Option<bool> {
    let scope = scope.trim();
    if scope == "all" {
        return Some(true);
    }
    if let Some(v) = predicate_args(scope, "whereStatus(") {
        return Some(info.attrs.get("status").map(String::as_str) == Some(v.trim()));
    }
    // whereKind(v): the item's WorkKind == v (e.g. research) — scopes a rule to a work-kind (issue055
    // researchSpikeCharterRule: only WorkKind::research Stories). The parser stores the enum MEMBER,
    // so `WorkKind::research` reads back as "research".
    if let Some(v) = predicate_args(scope, "whereKind(") {
        return Some(info.attrs.get("kind").map(String::as_str) == Some(v.trim()));
    }
    None
}

/// Evaluate one `ElementRule` predicate TERM for item `name`. Closed vocabulary so far: `nonBlank(field)`
/// (trimmed len > 0), `minLength(field,n)` (trimmed len >= n), `hasPassingResult(suffix)` (a sibling
/// item `<name><suffix>R1` exists with `outcome=pass` — the acceptance/DoD naming convention),
/// `resultJudgedByHuman(suffix)`, the `[not]matchesPattern[CI]` substring family, and
/// `charterTargetType(T1,...)` (every outgoing `#CharteredBy` edge targets an allow-listed type).
/// Unknown term returns `None` (the caller reports the rule `evaluated=false` rather than silently passing).
fn eval_predicate_term(model: &Model, name: &str, term: &str) -> Option<bool> {
    let term = term.trim();
    let attrs = &model.items.get(name)?.attrs;
    if let Some(args) = predicate_args(term, "nonBlank(") {
        return Some(attrs.get(args.trim()).is_some_and(|v| !v.trim().is_empty()));
    }
    if let Some(args) = predicate_args(term, "minLength(") {
        let mut parts = args.split(',');
        let field = parts.next()?.trim();
        let n: usize = parts.next()?.trim().parse().ok()?;
        return Some(attrs.get(field).is_some_and(|v| v.trim().chars().count() >= n));
    }
    if let Some(suffix) = predicate_args(term, "hasPassingResult(") {
        let ev = format!("{name}{}R1", suffix.trim());
        return Some(model.items.get(&ev).and_then(|i| i.attrs.get("outcome")).map(String::as_str) == Some("pass"));
    }
    // resultJudgedByHuman(suffix): the sibling result <name><suffix>R1 was judged by a HUMAN actor — its
    // judgedBy names a `Person`-typed item (D0106 confirmation-authenticity: sign-off is never AI-fabricated).
    if let Some(suffix) = predicate_args(term, "resultJudgedByHuman(") {
        let ev = format!("{name}{}R1", suffix.trim());
        let judged_by = model.items.get(&ev).and_then(|i| i.attrs.get("judgedBy"));
        return Some(judged_by.and_then(|jb| model.items.get(jb)).is_some_and(|a| a.type_name == "Person"));
    }
    // charterTargetType(T1,T2,...): every OUTGOING #CharteredBy edge from `name` targets an item whose
    // TYPE is in the allow-list. The enforceable slice of research-spike routing (issue055): once a
    // spike EXISTS, its charter must point at a real Issue or Decision, so the routing convention gains a
    // control on the structurally-visible side (the "did analysis skip the spike?" judgment stays
    // reminder-enforced — a commit gate cannot see a conversation). Vacuously true when `name` has no
    // charter edge — edge EXISTENCE is charterRule's job (D0068), not this rule's.
    if let Some(args) = predicate_args(term, "charterTargetType(") {
        let allow: Vec<&str> = args.split(',').map(str::trim).collect();
        let ok = model
            .edges
            .iter()
            .filter(|e| e.kind == "charteredby" && e.from == name)
            .all(|e| model.items.get(&e.to).is_some_and(|t| allow.contains(&t.type_name.as_str())));
        return Some(ok);
    }
    // matchesPattern(field, needle) / notMatchesPattern(field, needle): case-sensitive substring on an attr.
    // The `CI` variants are case-insensitive. `needle` may contain spaces (after the first comma); no ')'.
    for (prefix, want, ci) in [
        ("matchesPatternCI(", true, true),
        ("notMatchesPatternCI(", false, true),
        ("matchesPattern(", true, false),
        ("notMatchesPattern(", false, false),
    ] {
        if let Some(args) = predicate_args(term, prefix) {
            let (field, needle) = args.split_once(',')?;
            let hit = attrs.get(field.trim()).is_some_and(|v| {
                if ci {
                    v.to_lowercase().contains(&needle.trim().to_lowercase())
                } else {
                    v.contains(needle.trim())
                }
            });
            return Some(hit == want);
        }
    }
    None
}

/// Evaluate a full `ElementRule` `predicate` (TERMs joined by ` and `) for item `name`. Returns `None`
/// if ANY term is unsupported (so the rule reports `evaluated=false`, never a false pass). Conjunction.
fn eval_predicate(model: &Model, name: &str, predicate: &str) -> Option<bool> {
    let mut all = true;
    for term in predicate.split(" and ") {
        all &= eval_predicate_term(model, name, term)?;
    }
    Some(all)
}

/// `ElementRule` violations: `scope`d `subject` instances whose `predicate` is false. `Some(sorted
/// names)`, or `None` if the scope or predicate uses an unsupported term (caller marks the rule
/// not-evaluated). Subsumes the ~5 structural guards as each predicate becomes expressible.
fn element_rule_violations(model: &Model, subject: &str, predicate: &str, scope: &str) -> Option<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    for (name, info) in &model.items {
        if !rc_matches_subject(info, subject) {
            continue;
        }
        if !subject_in_scope(info, scope)? {
            continue;
        }
        if !eval_predicate(model, name, predicate)? {
            out.push(name.clone());
        }
    }
    out.sort();
    Some(out)
}

/// Business-layer view (serveBusinessNeedsView): the Brief, Personas, Needs and use cases.
///
/// The "what/why" layer the `keel serve` console lacked. A computed `#View`; each Need carries a
/// `decomposed` flag (some `SystemRequirement` `satisfy`-links it) so the human sees the trace frontier.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn business(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let by_type = |ty: &str| {
        let mut v: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| i.type_name == ty).collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    };
    let field = |i: &ItemInfo, k: &str| Json::s(i.attrs.get(k).cloned().unwrap_or_default());
    let briefs = by_type("Brief").into_iter().map(|(n, i)| Json::Obj(vec![
        ("name".to_string(), Json::s(n.clone())),
        ("title".to_string(), field(i, "title")),
        ("problem".to_string(), field(i, "problem")),
        ("opportunity".to_string(), field(i, "opportunity")),
        ("constraintsNote".to_string(), field(i, "constraintsNote")),
    ])).collect();
    let personas = by_type("Persona").into_iter().map(|(n, i)| Json::Obj(vec![
        ("name".to_string(), Json::s(n.clone())),
        ("title".to_string(), field(i, "title")),
        ("description".to_string(), field(i, "description")),
        ("goals".to_string(), field(i, "goals")),
    ])).collect();
    let needs = by_type("Need").into_iter().map(|(n, i)| {
        let decomposed = model.edges.iter().any(|e| e.kind == "satisfy" && &e.from == n);
        Json::Obj(vec![
            ("name".to_string(), Json::s(n.clone())),
            ("title".to_string(), field(i, "title")),
            ("statement".to_string(), field(i, "statement")),
            ("priority".to_string(), field(i, "priority")),
            ("source".to_string(), field(i, "source")),
            ("decomposed".to_string(), Json::Bool(decomposed)),
        ])
    }).collect();
    let use_cases = by_type("UseCase").into_iter().map(|(n, i)| Json::Obj(vec![
        ("name".to_string(), Json::s(n.clone())),
        ("title".to_string(), field(i, "title")),
    ])).collect();
    Ok(Json::Obj(vec![
        ("business".to_string(), Json::s("Business layer (Brief -> Personas -> Needs -> UseCases) — the what/why (D0105 serveBusinessNeedsView)")),
        ("briefs".to_string(), Json::Arr(briefs)),
        ("personas".to_string(), Json::Arr(personas)),
        ("needs".to_string(), Json::Arr(needs)),
        ("useCases".to_string(), Json::Arr(use_cases)),
    ]).dump())
}

/// Launchable-set view (srServeModelDrivenRegistry, Tier 1a): the processes + skills keel serve may launch.
///
/// Computed from the DECLARED model (no separately-authored list — nServeReuseModel). Each entry carries
/// its name/title/kind. A computed `#View`; finer per-launchable output schemas are a later increment.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn launchables(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let of_kind = |ty: &str| -> Vec<Json> {
        let mut v: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| i.type_name == ty).collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v.into_iter()
            .map(|(n, i)| Json::Obj(vec![
                ("name".to_string(), Json::s(n.clone())),
                ("title".to_string(), Json::s(i.attrs.get("title").cloned().unwrap_or_default())),
                ("kind".to_string(), Json::s(ty)),
            ]))
            .collect()
    };
    let skills = of_kind("AISkill");
    let processes = of_kind("Process");
    let total = skills.len() + processes.len();
    Ok(Json::Obj(vec![
        ("launchables".to_string(), Json::s("keel serve launchable set — declared skills + processes (srServeModelDrivenRegistry, D0109). Only these may be launched; no freeform path.")),
        ("skills".to_string(), Json::Arr(skills)),
        ("processes".to_string(), Json::Arr(processes)),
        ("total".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
    ])
    .dump())
}

/// Whether `target` is a declared launchable (a `Process` or `AISkill` in the model) — the guardrail
/// behind srServeLauncherDefinedOnly (no freeform launch). Tier 1a helper.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn is_launchable(root: &Path, target: &str) -> Result<bool, ViewError> {
    let model = Model::build(root)?;
    Ok(model.items.get(target).is_some_and(|i| matches!(i.type_name.as_str(), "Process" | "AISkill")))
}

/// Evaluate ONE declared rule by name → `(subjects_scanned, sorted violations)`.
///
/// The CONTRACT single source (D0107): the 5 migrated guards source their violations here instead of a
/// bespoke Rust predicate.
///
/// # Errors
/// [`ViewError`] on a parse failure, an unknown rule name, or an unsupported predicate/scope.
pub fn rule_violations(root: &Path, rule_name: &str) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    let Some(info) = model.items.get(rule_name) else {
        return Err(ViewError::Track(rule_name.to_string(), format!("declared rule '{rule_name}' not found")));
    };
    let a = |k: &str| info.attrs.get(k).cloned().unwrap_or_default();
    let scope = {
        let s = a("appliesWhen");
        if s.is_empty() { "all".to_string() } else { s }
    };
    let subject = a("subjectType");
    match info.type_name.as_str() {
        "EdgeRule" => {
            let scope_files = if scope == "newlyAdded" { Some(staged_added_files(root)) } else { None };
            let scanned = model.items.values().filter(|i| rc_matches_subject(i, &subject) && scope_files.as_ref().is_none_or(|f| f.contains(&i.file))).count();
            let v = edge_rule_violations(&model, &subject, &a("requiredEdge").to_lowercase(), &a("objectType"), &a("edgeDirection"), &a("cardinality"), scope_files.as_ref());
            Ok((scanned, v))
        }
        "ElementRule" => {
            let scanned = model.items.values().filter(|i| rc_matches_subject(i, &subject) && subject_in_scope(i, &scope).unwrap_or(false)).count();
            let v = element_rule_violations(&model, &subject, &a("predicate"), &scope)
                .ok_or_else(|| ViewError::Track(rule_name.to_string(), "unsupported predicate/scope term".to_string()))?;
            Ok((scanned, v))
        }
        other => Err(ViewError::Track(rule_name.to_string(), format!("unknown rule kind '{other}'"))),
    }
}

/// `keel rules` (D0105 EXPAND step 2): evaluate DECLARED rules over the model.
///
/// The generic evaluator that replaces the bespoke guards once each reaches PARITY
/// (guardsToRulesMigration). This walking skeleton evaluates `EdgeRule` with `appliesWhen="all"`;
/// `ElementRule`/`OrderingRule` and the full scope sub-language are later EXPAND steps (reported
/// `evaluated=false` meanwhile). Runs ALONGSIDE `keel guard` — nothing is retired here.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance/rule file fails to parse.
pub fn check(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut rule_names: Vec<&String> =
        model.items.iter().filter(|(_, i)| matches!(i.type_name.as_str(), "EdgeRule" | "ElementRule")).map(|(n, _)| n).collect();
    rule_names.sort();
    let mut rules_json: Vec<Json> = Vec::new();
    let mut total = 0usize;
    for rname in rule_names {
        let Some(info) = model.items.get(rname) else { continue };
        let kind = info.type_name.clone();
        let a = |k: &str| info.attrs.get(k).cloned().unwrap_or_default();
        let scope = {
            let s = a("appliesWhen");
            if s.is_empty() { "all".to_string() } else { s }
        };
        let (violations, evaluated) = if kind == "EdgeRule" {
            // EdgeRule scope: `all` (whole model) or `newlyAdded` (git staged-added files); else unsupported.
            let scope_files = if scope == "newlyAdded" { Some(staged_added_files(root)) } else { None };
            if scope == "all" || scope == "newlyAdded" {
                (edge_rule_violations(&model, &a("subjectType"), &a("requiredEdge").to_lowercase(), &a("objectType"), &a("edgeDirection"), &a("cardinality"), scope_files.as_ref()), true)
            } else {
                (Vec::new(), false)
            }
        } else {
            // ElementRule handles scope (all / whereStatus) itself; None => unsupported scope/predicate.
            element_rule_violations(&model, &a("subjectType"), &a("predicate"), &scope).map_or((Vec::new(), false), |v| (v, true))
        };
        total += violations.len();
        rules_json.push(Json::Obj(vec![
            ("rule".to_string(), Json::s(rname.clone())),
            ("kind".to_string(), Json::s(kind)),
            ("severity".to_string(), Json::s(a("severity"))),
            ("scope".to_string(), Json::s(scope)),
            ("evaluated".to_string(), Json::Bool(evaluated)),
            ("violations".to_string(), Json::Arr(violations.into_iter().map(Json::s).collect())),
        ]));
    }
    Ok(Json::Obj(vec![
        ("check".to_string(), Json::s("declared-rule evaluation (D0105; EdgeRule + ElementRule, appliesWhen=all)")),
        ("rules".to_string(), Json::Arr(rules_json)),
        ("total_violations".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
    ])
    .dump())
}

/// Requirement-rootedness view (D0098/D0099, issue047): the charter-source BURNDOWN (need-rooted vs
/// decision-driven vs orphan) over all delivery Stories, plus the `#Capability` gate set.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn rootedness(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut stories: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Story").map(|(n, _)| n).collect();
    stories.sort();
    let n = |c: usize| Json::Int(i64::try_from(c).unwrap_or(i64::MAX));
    let class_of: Vec<(&String, &str)> = stories.iter().map(|s| (*s, rd_charter_class(&model, s))).collect();
    let count = |k: &str| class_of.iter().filter(|(_, c)| *c == k).count();
    let orphans: Vec<Json> = class_of.iter().filter(|(_, c)| *c == "orphan").map(|(s, _)| Json::s((*s).clone())).collect();
    let gate: Vec<Json> = capability_root_violations(&model).into_iter().map(Json::s).collect();
    let out = Json::Obj(vec![
        ("rootedness".to_string(), Json::s("requirement rootedness (D0098/D0099, issue047): charter-source burndown over delivery Stories — `need` reaches a Need, `decision` is legitimate decision-driven engine evolution (D0064), `orphan` has no charter. The HARD gate (`guard requirement-rootedness`) fires only on a #Capability item with no #DerivedFrom->Need.")),
        ("total".to_string(), n(stories.len())),
        ("need_rooted".to_string(), n(count("need"))),
        ("decision_chartered".to_string(), n(count("decision"))),
        ("orphan".to_string(), n(count("orphan"))),
        ("orphans".to_string(), Json::Arr(orphans)),
        ("capability_violations".to_string(), Json::Arr(gate)),
    ]);
    Ok(out.dump())
}

// ── tier-satisfaction comprehensiveness (D0098/issue047 — the DOWNWARD integrity burndown) ──────
// Is each tier cleanly + comprehensively satisfied by its downstream items? STRUCTURAL floor (the
// measurable leading indicator): a Need is decomposed iff it has >=1 satisfying SystemRequirement
// (satisfy edge); a SystemRequirement is verified iff it has >=1 verify edge (a Test #Verify-linked).
// Thin downstream satisfaction predicts insufficient implementation. (DEEPER "comprehensive" judgment —
// do the SRs fully discharge the Need — is the AI white-box layer, SR-3c, not yet built.) Non-blocking
// burndown (D0098); computed from authored edges, nothing stored.

struct TierStat {
    tier: &'static str,
    relation: &'static str,
    total: usize,
    satisfied: usize,
    gaps: Vec<String>,
}

fn compute_tier_satisfaction(model: &Model) -> Vec<TierStat> {
    let has_out = |kind: &str, from: &str| model.edges.iter().any(|e| e.kind == kind && e.from == from);
    let has_in = |kind: &str, to: &str| model.edges.iter().any(|e| e.kind == kind && e.to == to);
    let tier = |ty: &str, relation: &'static str, pred: &dyn Fn(&str) -> bool, label: &'static str| -> TierStat {
        let mut names: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == ty).map(|(n, _)| n).collect();
        names.sort();
        let mut gaps: Vec<String> = Vec::new();
        let mut satisfied = 0;
        for n in &names {
            if pred(n) {
                satisfied += 1;
            } else {
                gaps.push((*n).clone());
            }
        }
        TierStat { tier: label, relation, total: names.len(), satisfied, gaps }
    };
    vec![
        // A Need is decomposed iff some SystemRequirement satisfies it (satisfy edge Need->SR).
        tier("Need", "satisfied-by SystemRequirement", &|n| has_out("satisfy", n), "Need"),
        // A SystemRequirement is verified iff a Test #Verify-links to it (verify edge Test->SR).
        tier("SystemRequirement", "verified-by Test", &|sr| has_in("verify", sr), "SystemRequirement"),
    ]
}

/// Tier-satisfaction comprehensiveness view (D0098/issue047).
///
/// Per tier, the fraction cleanly satisfied downstream (Needs decomposed into SRs; SRs verified by
/// Tests) + the gap set — a leading indicator of insufficient implementation.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn tier_satisfaction(root: &Path) -> Result<String, ViewError> {
    let stats = compute_tier_satisfaction(&Model::build(root)?);
    let n = |c: usize| Json::Int(i64::try_from(c).unwrap_or(i64::MAX));
    let tiers: Vec<Json> = stats
        .iter()
        .map(|t| {
            Json::Obj(vec![
                ("tier".to_string(), Json::s(t.tier)),
                ("relation".to_string(), Json::s(t.relation)),
                ("total".to_string(), n(t.total)),
                ("satisfied".to_string(), n(t.satisfied)),
                ("pct".to_string(), Json::Int(i64::from(pct(t.satisfied, t.total)))),
                ("gaps".to_string(), Json::Arr(t.gaps.iter().map(|g| Json::s(g.clone())).collect())),
            ])
        })
        .collect();
    let out = Json::Obj(vec![
        ("tier_satisfaction".to_string(), Json::s("tier-satisfaction comprehensiveness (D0098/issue047): STRUCTURAL downstream-satisfaction floor per tier — Needs decomposed into SystemRequirements (satisfy), SystemRequirements verified by Tests (verify). A leading indicator of insufficient implementation; thin downstream = predicted under-implementation. (Deeper 'do the SRs fully discharge the Need' is the AI white-box layer, not yet built.)")),
        ("tiers".to_string(), Json::Arr(tiers)),
    ]);
    Ok(out.dump())
}

/// Compact non-blocking BURNDOWN summary (D0098) for `orient`.
///
/// The always-visible "what's incomplete" headline. Cheap (graph-only, no git): tier-satisfaction
/// structural pcts + rootedness counts. Detail lives in `keel tier-satisfaction` / `keel rootedness` /
/// `keel assured` / `keel critique-coverage`.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn burndown_summary_json(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let tiers = compute_tier_satisfaction(&model);
    let need = tiers.iter().find(|t| t.tier == "Need");
    let sr = tiers.iter().find(|t| t.tier == "SystemRequirement");
    let pct_of = |t: Option<&TierStat>| t.map_or(100, |s| pct(s.satisfied, s.total));
    let unrooted_caps = capability_root_violations(&model).len();
    let mut stories: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Story").map(|(n, _)| n).collect();
    stories.sort();
    let orphan_stories = stories.iter().filter(|s| rd_charter_class(&model, s) == "orphan").count();
    let n = |c: usize| Json::Int(i64::try_from(c).unwrap_or(i64::MAX));
    Ok(Json::Obj(vec![
        ("need_decomposed_pct".to_string(), Json::Int(i64::from(pct_of(need)))),
        ("sr_verified_pct".to_string(), Json::Int(i64::from(pct_of(sr)))),
        ("unrooted_capabilities".to_string(), n(unrooted_caps)),
        ("orphan_stories".to_string(), n(orphan_stories)),
        ("detail".to_string(), Json::s("keel tier-satisfaction | rootedness | assured | critique-coverage")),
    ])
    .dump())
}

/// Append a parsed commit to the recent-activity timeline (helper for [`recent`]).
fn recent_flush(cur: Option<&(String, String, String)>, files: &[String], out: &mut Vec<Json>) {
    if let Some((sha, date, subj)) = cur {
        out.push(Json::Obj(vec![
            ("sha".to_string(), Json::s(sha.clone())),
            ("date".to_string(), Json::s(date.clone())),
            ("subject".to_string(), Json::s(subj.clone())),
            ("files".to_string(), Json::Arr(files.iter().map(|f| Json::s(f.clone())).collect())),
        ]));
    }
}

/// Git-derived recent-activity timeline (sr15) — the introspection "what changed recently" lens.
///
/// The latest commits touching `.tracking`/`.engine` and the element files each changed, newest first;
/// computed from git, nothing stored.
///
/// # Errors
/// Returns [`ViewError`] only on JSON assembly; a git failure yields an empty timeline (best-effort).
pub fn recent(root: &Path) -> Result<String, ViewError> {
    let raw = git_out(
        root,
        &["log", "--no-merges", "-n", "25", "--date=short", "--format=__C__%h\u{1f}%ad\u{1f}%s", "--name-only", "--", ".tracking", ".engine"],
    )
    .unwrap_or_default();
    let mut commits: Vec<Json> = Vec::new();
    let mut cur: Option<(String, String, String)> = None;
    let mut files: Vec<String> = Vec::new();
    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("__C__") {
            recent_flush(cur.as_ref(), &files, &mut commits);
            files.clear();
            let p: Vec<&str> = rest.splitn(3, '\u{1f}').collect();
            cur = Some((
                (*p.first().unwrap_or(&"")).to_string(),
                (*p.get(1).unwrap_or(&"")).to_string(),
                (*p.get(2).unwrap_or(&"")).to_string(),
            ));
        } else if !line.trim().is_empty() {
            files.push(line.trim().to_string());
        }
    }
    recent_flush(cur.as_ref(), &files, &mut commits);
    Ok(Json::Obj(vec![
        ("recent".to_string(), Json::s("git-derived recent-activity timeline (sr15): the latest commits touching .tracking/.engine + the element files each changed; newest first")),
        ("commits".to_string(), Json::Arr(commits)),
    ])
    .dump())
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
    let critique_gaps: Vec<String> = compute_critique_coverage(&model, &stale, &CritiquePolicy::load(root)?)
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
    let critiqued: HashSet<String> = compute_critique_coverage(&model, &stale, &CritiquePolicy::load(root)?)
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
    n.saturating_mul(100).checked_div(d).map_or(100, |x| u32::try_from(x).unwrap_or(0))
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
            let crit = compute_critique_coverage(&model, &stale, &CritiquePolicy::load_or_core3(root));
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
        let Some(wt) = std::env::temp_dir().join(format!("keel-trend-{short}")).to_str().map(str::to_string) else { continue };
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
/// `keel orient` JSON authority.
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
    let crit = compute_critique_coverage(model, stale, &CritiquePolicy::load(root)?);
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
    let crit_debt = compute_critique_coverage(model, stale, &CritiquePolicy::load_or_core3(root)).into_iter().filter(|c| !c.covered && gf_crit.as_ref().is_some_and(|g| g.contains(&c.element))).count();
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
<html lang="en"><head><meta charset="utf-8"><title>keel report</title>
<meta name="generator" content="keel report (computed #View; regenerate, do not commit as truth)">
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
<header><h1>keel · <span id="t"></span></h1><p>computed aggregate report (D0087) — regenerate, never commit as truth</p></header>
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
<html lang="en"><head><meta charset="utf-8"><title>keel traceability</title>
<meta name="generator" content="keel diagram (computed #View; regenerate, do not commit as truth)">
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
<html lang="en"><head><meta charset="utf-8"><title>keel view</title>
<meta name="generator" content="keel render --mode table (computed #View; regenerate, do not commit as truth)">
/*STYLE*/</head><body>
<header><h1>keel · <span id="vn"></span></h1><p id="cn"></p></header>
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
<html lang="en"><head><meta charset="utf-8"><title>keel review</title>
<meta name="generator" content="keel render --mode review (computed #View; capture is exported to JSON for apply-review)">
/*STYLE*/</head><body>
<header><h1>keel review · <span id="vn"></span></h1><p id="cn"></p></header>
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
/// `#View`: regenerate on demand (`keel diagram . > graph.html`), never commit it as truth.
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
/// an Export-JSON button that emits a batch consumable by `keel apply-review`).
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

    #[test]
    fn critique_policy_default_is_core3() {
        // Absent a policy file, the built-in Core-3 default applies (D0097) — identical to the shipped
        // critique-policy.toml, so behavior is unchanged whether or not a project authors an override.
        let p = CritiquePolicy::core3();
        assert!(p.is_target_type("Need"));
        assert!(p.is_target_type("Decision"));
        assert!(!p.is_target_type("Issue"));
        assert_eq!(p.required_lenses("Need"), ["completeness", "correctness", "testability"]);
        assert_eq!(p.required_lenses("Decision"), ["completeness", "correctness", "feasibility"]);
        assert!(p.required_lenses("Issue").is_empty());
        assert_eq!(p.target_types().count(), 3);
    }

    #[test]
    fn critique_policy_load_validates_and_overrides() {
        // load() reads .engine/contracts/critique-policy.toml when present: an unknown lens fails loud,
        // a valid override (extra lens / extra type) takes effect, and a missing file falls back.
        let dir = std::env::temp_dir().join(format!("keel_cpol_{}", std::process::id()));
        let contracts = dir.join(".engine").join("contracts");
        std::fs::create_dir_all(&contracts).unwrap();
        let policy_file = contracts.join("critique-policy.toml");

        // Missing file -> built-in default.
        std::fs::remove_file(&policy_file).ok();
        let empty = dir.join("no-such-engine-root-xyz");
        assert!(!CritiquePolicy::load(&empty).unwrap().from_file);

        // Unknown lens -> fail-loud Policy error.
        std::fs::write(&policy_file, "[lenses]\nNeed = [\"completeness\", \"bogus\"]\n").unwrap();
        assert!(matches!(CritiquePolicy::load(&dir), Err(ViewError::Policy(_))));

        // Valid override: add a lens to Need + gate a new type.
        std::fs::write(
            &policy_file,
            "[lenses]\nNeed = [\"completeness\", \"necessity\"]\nArchitecture = [\"feasibility\"]\n",
        )
        .unwrap();
        let p = CritiquePolicy::load(&dir).unwrap();
        assert!(p.from_file);
        assert_eq!(p.required_lenses("Need"), ["completeness", "necessity"]);
        assert!(p.is_target_type("Architecture"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn extract_field_unescapes_like_the_lexer() {
        // issue044 regression: a field with backslash escapes must extract to the SAME value the
        // parser stores, or its critiques false-flag stale. `\\s` (raw blob) -> `\s` (model value).
        let blob = "    part d0023 : Decision {\n        :>> decision = \"regex r'part\\\\s+(\\\\w+?)' and a quote \\\" inside\";\n    }\n";
        let got = extract_field(blob, "d0023", "decision");
        assert_eq!(got.as_deref(), Some("regex r'part\\s+(\\w+?)' and a quote \" inside"));
    }

    #[test]
    fn extract_field_plain_value() {
        let blob = "    part d1 : Decision {\n        :>> decision = \"plain text\";\n    }\n";
        assert_eq!(extract_field(blob, "d1", "decision").as_deref(), Some("plain text"));
    }

    fn model() -> Model {
        let mut items = HashMap::new();
        items.insert("r1".to_string(), ItemInfo { type_name: "Requirement".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        items.insert("c1".to_string(), ItemInfo { type_name: "Component".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        let mut dattrs = HashMap::new();
        dattrs.insert("status".to_string(), "accepted".to_string());
        items.insert("d1".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: dattrs, marker: Some("ProspectiveChange".to_string()), file: String::new() });
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
    fn element_section_is_element_plus_one_hop_neighbours() {
        // sr18: an element-seeded section = the element + every element one typed edge away (the local
        // neighbourhood), and no further. model(): r1 --satisfy--> c1; d1 is isolated. Section(r1) =
        // {r1, c1}; d1 (unconnected) is NOT in the section.
        let got = element_neighbourhood(&model(), "r1");
        assert_eq!(got.len(), 2);
        assert!(got.contains("r1"));
        assert!(got.contains("c1"));
        assert!(!got.contains("d1"));
    }

    #[test]
    fn element_section_includes_incoming_neighbours() {
        // The neighbourhood is direction-agnostic: c1 is reached only via an INCOMING satisfy edge
        // (r1 -> c1), so Section(c1) must still include r1.
        let got = element_neighbourhood(&model(), "c1");
        assert_eq!(got.len(), 2);
        assert!(got.contains("c1"));
        assert!(got.contains("r1"));
    }

    #[test]
    fn contains_token_is_identifier_bounded() {
        // D0102 decision-requirement-link: exact-identifier match, not substring — sr1 must NOT match
        // inside sr15ServeIntrospect, but a real reference (any non-alphanumeric boundary) does.
        assert!(contains_token("descoped by sr19ServeWhiteboxBoundary today", "sr19ServeWhiteboxBoundary"));
        assert!(contains_token("see (d0100).", "d0100"));
        assert!(contains_token("n17ServeGranularWhitebox", "n17ServeGranularWhitebox"));
        assert!(!contains_token("sr15ServeIntrospect", "sr1"));
        assert!(!contains_token("sr150Foo", "sr15"));
    }

    #[test]
    fn governance_mention_distinguishes_governance_from_context() {
        // D0104: this decision's own governance (verb + no foreign id) -> true.
        assert!(is_governance_mention("Amend the statements of need n11FastStart and requirement sr11FastStart to state both bars", "0083"));
        // describes ANOTHER decision's action (foreign id D0083 near) -> false.
        assert!(!is_governance_mention("amending sr11FastStart's statement (D0083) left sr11Verify falsely stale", "0084"));
        // pure example, no governance verb -> false.
        assert!(!is_governance_mention("sr11FastStart is a measured GAP (orient ~13.6s vs <500ms)", "0082"));
        // cites a foreign decision's descope -> false.
        assert!(!is_governance_mention("D0100 descoped sr19ServeWhiteboxBoundary's AI-clustering boundary mode", "0102"));
    }

    #[test]
    fn need_slice_collects_srs_components_and_tests() {
        // sr19 white-box boundary = a Need-slice: the Need + its satisfying SRs + their allocated
        // Components + the Tests verifying any of them. n1 --satisfy--> sr1 --allocate--> comp1;
        // t1 --verify--> sr1. Unrelated Need u1 is NOT in the slice.
        let mut items = HashMap::new();
        for (n, t) in [("n1", "Need"), ("sr1", "SystemRequirement"), ("comp1", "Component"), ("t1", "Test"), ("u1", "Need")] {
            items.insert(n.to_string(), ItemInfo { type_name: t.to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        }
        let edges = vec![
            Edge { kind: "satisfy".to_string(), from: "n1".to_string(), to: "sr1".to_string() },
            Edge { kind: "allocate".to_string(), from: "sr1".to_string(), to: "comp1".to_string() },
            Edge { kind: "verify".to_string(), from: "t1".to_string(), to: "sr1".to_string() },
        ];
        let slice = need_slice(&Model { items, edges }, "n1");
        assert_eq!(slice.len(), 4);
        for n in ["n1", "sr1", "comp1", "t1"] {
            assert!(slice.contains(n), "slice should contain {n}");
        }
        assert!(!slice.contains("u1"));
    }

    #[test]
    fn boundary_json_emits_internals_interfaces_and_coupling() {
        // n1 slice {n1, sr1}; one cut edge sr1 --dependency--> ext (ext is OUTSIDE the boundary).
        let mut items = HashMap::new();
        for (n, t) in [("n1", "Need"), ("sr1", "SystemRequirement"), ("ext", "Component")] {
            items.insert(n.to_string(), ItemInfo { type_name: t.to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        }
        let edges = vec![
            Edge { kind: "satisfy".to_string(), from: "n1".to_string(), to: "sr1".to_string() },
            Edge { kind: "dependency".to_string(), from: "sr1".to_string(), to: "ext".to_string() },
        ];
        let m = Model { items, edges };
        let slice = need_slice(&m, "n1");
        let cut = cut_edges(&m, &slice);
        let out = boundary_emit_json(&m, "n1", &slice, &cut);
        assert!(out.contains("\"need\": \"n1\""));
        assert!(out.contains("\"coupling\": 1"));
        assert!(out.contains("\"internal\""));
        assert!(out.contains("\"interfaces\""));
        assert!(out.contains("\"external\": \"ext\"")); // the outside endpoint named
        assert!(out.contains("\"sr1\"")); // internal element present
    }

    #[test]
    fn cut_edges_are_the_interfaces_leaving_the_boundary() {
        // sr19 black-box: a boundary's interfaces = edges with exactly ONE endpoint inside. Boundary
        // {a,b}: a->b is internal (both in); b->x leaves; y->a enters. cut = {b->x, y->a}.
        let mut items = HashMap::new();
        for n in ["a", "b", "x", "y"] {
            items.insert(n.to_string(), ItemInfo { type_name: "X".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        }
        let edges = vec![
            Edge { kind: "dependency".to_string(), from: "a".to_string(), to: "b".to_string() },
            Edge { kind: "dependency".to_string(), from: "b".to_string(), to: "x".to_string() },
            Edge { kind: "satisfy".to_string(), from: "y".to_string(), to: "a".to_string() },
        ];
        let m = Model { items, edges };
        let boundary: HashSet<String> = ["a", "b"].iter().map(|s| (*s).to_string()).collect();
        let cut = cut_edges(&m, &boundary);
        assert_eq!(cut.len(), 2);
        assert!(cut.iter().any(|e| e.from == "b" && e.to == "x"));
        assert!(cut.iter().any(|e| e.from == "y" && e.to == "a"));
        assert!(!cut.iter().any(|e| e.from == "a" && e.to == "b"));
    }

    #[test]
    fn section_json_emits_items_and_induced_edges() {
        // The section emit carries the seed + kind and each element's name+type; an edge is emitted
        // only when BOTH endpoints are inside the section (an induced subgraph).
        let set: HashSet<String> = ["r1", "c1"].iter().map(|s| (*s).to_string()).collect();
        let out = section_subgraph_json(&model(), &set, "r1", "element");
        assert!(out.contains("\"seed\": \"r1\""));
        assert!(out.contains("\"kind\": \"element\""));
        assert!(out.contains("\"name\": \"r1\""));
        assert!(out.contains("\"name\": \"c1\""));
        assert!(out.contains("\"satisfy\"")); // r1 --satisfy--> c1: both endpoints in-section
    }

    #[test]
    fn section_json_excludes_edges_leaving_the_section() {
        // Section = {r1} only; the r1 --satisfy--> c1 edge has c1 OUTSIDE the bound, so it is dropped.
        let set: HashSet<String> = std::iter::once("r1".to_string()).collect();
        let out = section_subgraph_json(&model(), &set, "r1", "element");
        assert!(!out.contains("\"satisfy\""));
        assert!(!out.contains("\"name\": \"c1\""));
    }

    #[test]
    fn element_section_stops_at_one_hop() {
        // a -> b -> c chain; Section(a) = {a, b} only — c is two hops away, beyond the local section.
        let mut items = HashMap::new();
        for n in ["a", "b", "c"] {
            items.insert(n.to_string(), ItemInfo { type_name: "X".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        }
        let edges = vec![
            Edge { kind: "dependency".to_string(), from: "a".to_string(), to: "b".to_string() },
            Edge { kind: "dependency".to_string(), from: "b".to_string(), to: "c".to_string() },
        ];
        let got = element_neighbourhood(&Model { items, edges }, "a");
        assert_eq!(got.len(), 2);
        assert!(got.contains("a"));
        assert!(got.contains("b"));
        assert!(!got.contains("c"));
    }

    #[test]
    fn issue_resolution_open_vs_resolved() {
        // i1 resolved by a done action; i2 open (resolver action not done); i3 untriaged (no edge).
        let mut items = HashMap::new();
        for n in ["i1", "i2", "i3"] {
            items.insert(n.to_string(), ItemInfo { type_name: "Issue".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        }
        items.insert("actDone".to_string(), ItemInfo { type_name: "action".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        items.insert("actOpen".to_string(), ItemInfo { type_name: "action".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
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
        items.insert("i9".to_string(), ItemInfo { type_name: "Issue".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        let mut dattrs = HashMap::new();
        dattrs.insert("status".to_string(), "accepted".to_string());
        items.insert("d99".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: dattrs, marker: None, file: String::new() });
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
            items.insert(d.to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: accepted(), marker: None, file: String::new() });
        }
        for sr in ["sr1", "sr2"] {
            items.insert(sr.to_string(), ItemInfo { type_name: "SystemRequirement".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        }
        items.insert("n1".to_string(), ItemInfo { type_name: "Need".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        items.insert("n2".to_string(), ItemInfo { type_name: "Need".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        items.insert("d4".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: accepted(), marker: None, file: String::new() });
        let mut ev = HashMap::new();
        ev.insert("outcome".to_string(), "pass".to_string());
        items.insert("d4AcceptR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: ev, marker: None, file: String::new() });
        items.insert("vt".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        let mut vtres = HashMap::new();
        vtres.insert("outcome".to_string(), "pass".to_string());
        items.insert("vtR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: vtres, marker: None, file: String::new() });
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
        items.insert("sr1".to_string(), ItemInfo { type_name: "SystemRequirement".to_string(), attrs: req, marker: None, file: String::new() });
        let crit = |lens: &str| {
            let mut a = HashMap::new();
            a.insert("method".to_string(), "critique".to_string());
            a.insert("lens".to_string(), lens.to_string());
            a
        };
        items.insert("c1".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit("completeness"), marker: None, file: String::new() });
        items.insert("c2".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit("correctness"), marker: None, file: String::new() });
        let res = |by: &str| {
            let mut a = HashMap::new();
            a.insert("outcome".to_string(), "pass".to_string());
            a.insert("judgedBy".to_string(), by.to_string());
            a
        };
        items.insert("c1R1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("claudeOpus"), marker: None, file: String::new() });
        items.insert("c2R1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("wweatherholtz"), marker: None, file: String::new() });
        let edges = vec![
            Edge { kind: "verify".to_string(), from: "c1".to_string(), to: "sr1".to_string() },
            Edge { kind: "verify".to_string(), from: "c2".to_string(), to: "sr1".to_string() },
        ];
        let model = Model { items, edges };
        let cov = compute_critique_coverage(&model, &HashSet::<String>::new(), &CritiquePolicy::core3());
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
        items.insert("d1".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        items.insert("d2".to_string(), ItemInfo { type_name: "Decision".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        let crit = || {
            let mut a = HashMap::new();
            a.insert("method".to_string(), "critique".to_string());
            a
        };
        items.insert("cFail".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit(), marker: None, file: String::new() });
        items.insert("cPass".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit(), marker: None, file: String::new() });
        let res = |o: &str| {
            let mut a = HashMap::new();
            a.insert("outcome".to_string(), o.to_string());
            a
        };
        items.insert("cFailR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("fail"), marker: None, file: String::new() });
        items.insert("cPassR1".to_string(), ItemInfo { type_name: "TestResult".to_string(), attrs: res("pass"), marker: None, file: String::new() });
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
            ItemInfo { type_name: "Issue".to_string(), attrs: a, marker: None, file: String::new() }
        };
        let disp = |verdict: &str| {
            let mut a = HashMap::new();
            a.insert("disposition".to_string(), verdict.to_string());
            ItemInfo { type_name: "Test".to_string(), attrs: a, marker: None, file: String::new() }
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
        let story = |t: &str| ItemInfo { type_name: t.to_string(), attrs: HashMap::new(), marker: None, file: String::new() };
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

    fn item(ty: &str, marker: Option<&str>) -> ItemInfo {
        ItemInfo { type_name: ty.to_string(), attrs: HashMap::new(), marker: marker.map(str::to_string), file: String::new() }
    }

    #[test]
    fn item_detail_surfaces_dod_procedure_text() {
        // issue064: an action task carries no authored attrs; its description is the <name>DoD
        // procedureText. item_detail_json surfaces it as `dod`; a task without a DoD sibling -> no dod.
        let mut items = HashMap::new();
        items.insert("proveX".to_string(), item("Action", None)); // no authored attrs
        let mut dod_attrs = HashMap::new();
        dod_attrs.insert("method".to_string(), "inspect".to_string());
        dod_attrs.insert("procedureText".to_string(), "Resolves issueNNN: do the thing and verify Y.".to_string());
        items.insert("proveXDoD".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: dod_attrs, marker: None, file: String::new() });
        items.insert("bareTask".to_string(), item("Action", None)); // no DoD sibling
        let model = Model { items, edges: vec![] };
        let with_dod = item_detail_json(&model, "proveX").dump();
        assert!(with_dod.contains("Resolves issueNNN: do the thing and verify Y."), "dod procedureText surfaced: {with_dod}");
        assert!(with_dod.contains("inspect"), "dod method surfaced: {with_dod}");
        let no_dod = item_detail_json(&model, "bareTask").dump();
        assert!(!no_dod.contains("procedureText"), "task without a DoD carries no dod text: {no_dod}");
    }

    #[test]
    fn capability_must_derive_a_need() {
        // D0099: a #Capability with no #DerivedFrom->Need is the rootedness violation; one WITH it is clean.
        // Unmarked decision-driven work is exempt entirely.
        let mut items = HashMap::new();
        items.insert("n1".to_string(), item("Need", None));
        items.insert("capA".to_string(), item("Decision", Some("Capability"))); // unrooted
        items.insert("capB".to_string(), item("Decision", Some("Capability"))); // rooted
        items.insert("plain".to_string(), item("Decision", None)); // exempt (unmarked)
        let edges = vec![Edge { kind: "derivedfrom".to_string(), from: "capB".to_string(), to: "n1".to_string() }];
        let model = Model { items, edges };
        assert_eq!(capability_root_violations(&model), vec!["capA".to_string()]);
    }

    #[test]
    fn edge_rule_evaluator_reaches_parity_with_the_rootedness_guard() {
        // D0105 EXPAND parity: the GENERIC EdgeRule evaluator must reproduce guard:requirement-rootedness
        // (capabilityRootednessRule: subject=#Capability, edge=derivedFrom, object=Need, atLeastOne).
        let mut items = HashMap::new();
        items.insert("n1".to_string(), item("Need", None));
        items.insert("capA".to_string(), item("Decision", Some("Capability"))); // unrooted
        items.insert("capB".to_string(), item("Decision", Some("Capability"))); // rooted
        items.insert("plain".to_string(), item("Decision", None)); // exempt (unmarked)
        let edges = vec![Edge { kind: "derivedfrom".to_string(), from: "capB".to_string(), to: "n1".to_string() }];
        let model = Model { items, edges };
        assert_eq!(
            edge_rule_violations(&model, "#Capability", "derivedfrom", "Need", "outgoing", "atLeastOne", None),
            capability_root_violations(&model),
        );
    }

    #[test]
    fn edge_rule_incoming_flags_untriaged_issue() {
        // D0105: issuesTriagedRule = an Issue must carry an INCOMING #Resolves edge (some resolver -> issue).
        let mut items = HashMap::new();
        items.insert("issueA".to_string(), item("Issue", None)); // untriaged
        items.insert("issueB".to_string(), item("Issue", None)); // triaged
        items.insert("fixB".to_string(), item("Story", None));
        let edges = vec![Edge { kind: "resolves".to_string(), from: "fixB".to_string(), to: "issueB".to_string() }];
        let model = Model { items, edges };
        assert_eq!(
            edge_rule_violations(&model, "Issue", "resolves", "*", "incoming", "atLeastOne", None),
            vec!["issueA".to_string()],
        );
    }

    #[test]
    fn element_rule_reaches_parity_with_decision_rationale_guard() {
        // D0105: decisionRationaleRule = minLength(context,20) and minLength(rationale,20) — must reproduce
        // view::decisions_weak_rationale (a Decision whose context OR rationale is < 20 trimmed chars).
        let long = "x".repeat(25);
        let mk = |ctx: &str, rat: &str| {
            let mut a = HashMap::new();
            a.insert("context".to_string(), ctx.to_string());
            a.insert("rationale".to_string(), rat.to_string());
            ItemInfo { type_name: "Decision".to_string(), attrs: a, marker: None, file: String::new() }
        };
        let mut items = HashMap::new();
        items.insert("dGood".to_string(), mk(&long, &long));
        items.insert("dWeakCtx".to_string(), mk("short", &long));
        items.insert("dWeakRat".to_string(), mk(&long, "short"));
        let model = Model { items, edges: vec![] };
        let via_rule = element_rule_violations(&model, "Decision", "minLength(context,20) and minLength(rationale,20)", "all").unwrap();
        // The guard's own logic (blank = trimmed len < 20 on context OR rationale).
        let mut via_guard: Vec<String> = model.items.iter()
            .filter(|(_, i)| i.type_name == "Decision")
            .filter(|(_, i)| { let b = |f: &str| i.attrs.get(f).is_none_or(|v| v.trim().chars().count() < 20); b("context") || b("rationale") })
            .map(|(n, _)| n.clone()).collect();
        via_guard.sort();
        assert_eq!(via_rule, via_guard);
        assert_eq!(via_rule, vec!["dWeakCtx".to_string(), "dWeakRat".to_string()]);
    }

    #[test]
    fn edge_rule_newly_added_scope_restricts_to_staged_files() {
        // D0105 charterRule: an uncharted Story in a NEWLY-ADDED file is flagged; one in an existing
        // (not-added) file is out of scope. Mirrors guard:charter's forward-only (staged-added) semantics.
        let story = |file: &str| ItemInfo { type_name: "Story".to_string(), attrs: HashMap::new(), marker: None, file: file.to_string() };
        let mut items = HashMap::new();
        items.insert("newUncharted".to_string(), story(".tracking/delivery/sprintNew.sysml")); // added + uncharted
        items.insert("oldUncharted".to_string(), story(".tracking/delivery/sprintOld.sysml")); // uncharted but NOT added
        let edges = vec![]; // neither is chartered
        let model = Model { items, edges };
        let added: std::collections::HashSet<String> = std::iter::once(".tracking/delivery/sprintNew.sysml".to_string()).collect();
        // newlyAdded scope: only the story in the staged-added file is flagged.
        assert_eq!(
            edge_rule_violations(&model, "Story", "charteredby", "*", "outgoing", "atLeastOne", Some(&added)),
            vec!["newUncharted".to_string()],
        );
        // all scope (None): both uncharted stories flagged — confirms the scope filter is what narrows it.
        assert_eq!(
            edge_rule_violations(&model, "Story", "charteredby", "*", "outgoing", "atLeastOne", None),
            vec!["newUncharted".to_string(), "oldUncharted".to_string()],
        );
    }

    #[test]
    fn launchable_set_is_processes_and_skills_only() {
        // srServeLauncherDefinedOnly (Tier 1a): a Process/AISkill is launchable; anything else (or unknown) is not.
        fn is_launchable_in(model: &Model, target: &str) -> bool {
            model.items.get(target).is_some_and(|i| matches!(i.type_name.as_str(), "Process" | "AISkill"))
        }
        let mut items = HashMap::new();
        items.insert("someProcess".to_string(), item("Process", None));
        items.insert("someSkill".to_string(), item("AISkill", None));
        items.insert("someDecision".to_string(), item("Decision", None));
        let model = Model { items, edges: vec![] };
        assert!(is_launchable_in(&model, "someProcess"));
        assert!(is_launchable_in(&model, "someSkill"));
        assert!(!is_launchable_in(&model, "someDecision")); // not launchable
        assert!(!is_launchable_in(&model, "doesNotExist")); // freeform target -> not launchable
    }

    #[test]
    fn element_rule_flags_ai_judged_acceptance() {
        // issue059/D0106: an accepted Decision whose acceptance event is AI-judged is flagged; human-judged passes.
        let dec = || ItemInfo { type_name: "Decision".to_string(), attrs: {
            let mut a = HashMap::new(); a.insert("status".to_string(), "accepted".to_string()); a
        }, marker: None, file: String::new() };
        let ev = |by: &str| ItemInfo { type_name: "TestResult".to_string(), attrs: {
            let mut a = HashMap::new(); a.insert("judgedBy".to_string(), by.to_string()); a
        }, marker: None, file: String::new() };
        let mut items = HashMap::new();
        items.insert("will".to_string(), ItemInfo { type_name: "Person".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        items.insert("claudeOpus".to_string(), ItemInfo { type_name: "Actor".to_string(), attrs: HashMap::new(), marker: None, file: String::new() });
        items.insert("dHuman".to_string(), dec());
        items.insert("dHumanAcceptR1".to_string(), ev("will")); // human -> ok
        items.insert("dAi".to_string(), dec());
        items.insert("dAiAcceptR1".to_string(), ev("claudeOpus")); // AI-judged -> violation
        let model = Model { items, edges: vec![] };
        assert_eq!(
            element_rule_violations(&model, "Decision", "resultJudgedByHuman(Accept)", "whereStatus(accepted)").unwrap(),
            vec!["dAi".to_string()],
        );
    }

    #[test]
    fn element_rule_flags_research_spike_bad_charter() {
        // issue055 researchSpikeCharterRule: a WorkKind::research Story must charter to a legitimate
        // governing source — Decision/Need/SystemRequirement/Issue (the D0068 union). whereKind(research)
        // scope + charterTargetType(...) predicate. A spike chartered to an arbitrary element is flagged.
        let story = |kind: &str| {
            let mut a = HashMap::new();
            a.insert("kind".to_string(), kind.to_string());
            ItemInfo { type_name: "Story".to_string(), attrs: a, marker: None, file: String::new() }
        };
        let bare = |ty: &str| ItemInfo { type_name: ty.to_string(), attrs: HashMap::new(), marker: None, file: String::new() };
        let ch = |from: &str, to: &str| Edge { kind: "charteredby".to_string(), from: from.to_string(), to: to.to_string() };
        let mut items = HashMap::new();
        items.insert("iss".to_string(), bare("Issue"));
        items.insert("dec".to_string(), bare("Decision"));
        items.insert("ndd".to_string(), bare("Need"));
        items.insert("sr".to_string(), bare("SystemRequirement"));
        items.insert("otherStory".to_string(), bare("Story"));
        items.insert("spikeToIssue".to_string(), story("research")); // -> Issue: ok
        items.insert("spikeToDecision".to_string(), story("research")); // -> Decision: ok
        items.insert("spikeToNeed".to_string(), story("research")); // -> Need: ok (originating source)
        items.insert("spikeToSr".to_string(), story("research")); // -> SystemRequirement: ok (sr19SpikeStory case)
        items.insert("spikeToStory".to_string(), story("research")); // -> Story: VIOLATION (not a governing source)
        items.insert("spikeUnchartered".to_string(), story("research")); // no charter: vacuously ok (charterRule's job)
        items.insert("codeToStory".to_string(), story("code")); // non-research: out of scope, ignored
        let edges = vec![
            ch("spikeToIssue", "iss"),
            ch("spikeToDecision", "dec"),
            ch("spikeToNeed", "ndd"),
            ch("spikeToSr", "sr"),
            ch("spikeToStory", "otherStory"),
            ch("codeToStory", "otherStory"),
        ];
        let model = Model { items, edges };
        assert_eq!(
            element_rule_violations(&model, "Story", "charterTargetType(Issue,Decision,Need,SystemRequirement)", "whereKind(research)").unwrap(),
            vec!["spikeToStory".to_string()],
        );
    }

    #[test]
    fn element_rule_not_matches_pattern_flags_verdict_prose() {
        // issue058 decisionNoVerdictProseRule: a Decision restating "ACCEPTED 202..." in prose is flagged.
        let dec = |consequences: &str| {
            let mut a = HashMap::new();
            a.insert("consequences".to_string(), consequences.to_string());
            ItemInfo { type_name: "Decision".to_string(), attrs: a, marker: None, file: String::new() }
        };
        let mut items = HashMap::new();
        items.insert("dClean".to_string(), dec("no verdict prose here"));
        items.insert("dDual".to_string(), dec("... ACCEPTED 2026-07-03 by wweatherholtz ...")); // dual-truth
        items.insert("dLower".to_string(), dec("was accepted 2026 by the human")); // CI variant (issue062)
        let model = Model { items, edges: vec![] };
        // case-sensitive: only the uppercase form.
        assert_eq!(
            element_rule_violations(&model, "Decision", "notMatchesPattern(consequences,ACCEPTED 202)", "all").unwrap(),
            vec!["dDual".to_string()],
        );
        // case-insensitive (the broadened rule, issue062): catches BOTH forms.
        assert_eq!(
            element_rule_violations(&model, "Decision", "notMatchesPatternCI(consequences,accepted 202)", "all").unwrap(),
            vec!["dDual".to_string(), "dLower".to_string()],
        );
    }

    #[test]
    fn element_rule_scope_and_result_reach_parity_with_acceptance_events() {
        // D0105: acceptanceEventRule = whereStatus(accepted) Decision must hasPassingResult(Accept) —
        // must reproduce compute_attestation (accepted Decision lacking a passing <name>AcceptR1).
        let dec = |status: &str| {
            let mut a = HashMap::new();
            a.insert("status".to_string(), status.to_string());
            ItemInfo { type_name: "Decision".to_string(), attrs: a, marker: None, file: String::new() }
        };
        let result = |outcome: &str| {
            let mut a = HashMap::new();
            a.insert("outcome".to_string(), outcome.to_string());
            ItemInfo { type_name: "TestResult".to_string(), attrs: a, marker: None, file: String::new() }
        };
        let mut items = HashMap::new();
        items.insert("dAcc".to_string(), dec("accepted")); // accepted + passing event => ok
        items.insert("dAccAcceptR1".to_string(), result("pass"));
        items.insert("dGap".to_string(), dec("accepted")); // accepted, NO event => violation
        items.insert("dProp".to_string(), dec("proposed")); // proposed => out of scope, ignored
        let model = Model { items, edges: vec![] };
        let via_rule = element_rule_violations(&model, "Decision", "hasPassingResult(Accept)", "whereStatus(accepted)").unwrap();
        let (_total, via_guard) = compute_attestation(&model);
        assert_eq!(via_rule, via_guard);
        assert_eq!(via_rule, vec!["dGap".to_string()]);
    }

    #[test]
    fn tier_satisfaction_counts_decomposition_and_verification() {
        // D0098: a Need is decomposed iff some SR satisfies it; an SR is verified iff a Test #Verify-links it.
        let mut items = HashMap::new();
        items.insert("n1".to_string(), item("Need", None)); // decomposed
        items.insert("n2".to_string(), item("Need", None)); // gap
        items.insert("sr1".to_string(), item("SystemRequirement", None)); // verified
        items.insert("sr2".to_string(), item("SystemRequirement", None)); // gap
        items.insert("t1".to_string(), item("Test", None));
        let edges = vec![
            Edge { kind: "satisfy".to_string(), from: "n1".to_string(), to: "sr1".to_string() },
            Edge { kind: "verify".to_string(), from: "t1".to_string(), to: "sr1".to_string() },
        ];
        let model = Model { items, edges };
        let stats = compute_tier_satisfaction(&model);
        let need = stats.iter().find(|t| t.tier == "Need").unwrap();
        assert_eq!((need.total, need.satisfied), (2, 1));
        assert_eq!(need.gaps, vec!["n2".to_string()]);
        let sr = stats.iter().find(|t| t.tier == "SystemRequirement").unwrap();
        assert_eq!((sr.total, sr.satisfied), (2, 1));
        assert_eq!(sr.gaps, vec!["sr2".to_string()]);
    }
}
