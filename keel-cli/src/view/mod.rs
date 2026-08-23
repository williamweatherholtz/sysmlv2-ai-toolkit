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

// ── the split (sprint 418, dcViewRsRestructure): leaf lenses live in cohesive submodules; the
// model core (spec, Model build, traversal, JSON emit) stays here. `pub use` keeps every
// existing `crate::view::X` path valid - callers are untouched by design.
mod checks;
mod critique;
mod knowledge;
mod reports;
mod staleness;
pub use checks::*;
pub use knowledge::*;
pub use critique::*;
pub use reports::*;
pub use staleness::*;


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

#[derive(Clone)]
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

/// The computed `displayLabel` view (schema §2.3 — declared but historically unbuilt; D0126). The human
/// label for an element: its authored `title` when present + non-blank, else the immutable `name`
/// identifier. Titles may duplicate (that is fine — identity is the `name`); `displayLabel` is never
/// stored, always computed here so every surface labels elements the same way.
fn display_label(name: &str, info: &ItemInfo) -> String {
    match info.attrs.get("title") {
        Some(t) if !t.trim().is_empty() => t.clone(),
        _ => name.to_string(),
    }
}

#[derive(Clone)]
pub(crate) struct Model {
    items: HashMap<String, ItemInfo>,
    edges: Vec<Edge>,
}

/// The directories whose `.sysml` files ARE the model.
///
/// Authored instances live in `.tracking` and in the `.engine` INSTANCE dirs. Parsing is syntactic
/// (no import resolution), so `.engine` instance files parse standalone. Schema files are the
/// vocabulary rather than instances, and are excluded.
///
/// Shared rather than inlined in `Model::build`, because a check that resolves names against the
/// model must scan exactly what the model contains. Walking `.engine` wholesale instead made
/// `edge-endpoints` fire on `docs/tracking-template.sysml` — an authoring EXAMPLE whose
/// `exampleNeed` placeholders are undeclared on purpose. Four violations against a file the model
/// never loads is how a new guard teaches its reader to ignore it.
///
/// issue104: `.engine/workflows` belongs here. The six workflow definitions are the one place this
/// repo models behaviour in the base language (an `action def` with successions), so omitting them
/// left their flow edges nowhere to land.
fn model_dirs(root: &Path) -> [std::path::PathBuf; 8] {
    [
        root.join(".tracking"),
        root.join(".knowledge"), // D0161: declared Questions/Aliases - absent dir = nothing declared
        root.join(".engine").join("decisions"),
        root.join(".engine").join("processes"),
        root.join(".engine").join("views"),
        root.join(".engine").join("skills"),
        root.join(".engine").join("rules"), // D0105: declared EdgeRule/ElementRule instances
        root.join(".engine").join("workflows"),
    ]
}

/// Known edge kinds (canonical, lowercase), DERIVED from the schema — never restated here.
///
/// This was a hardcoded list, and it had drifted from the schema in BOTH directions (issue119): it
/// rejected `derivedFrom`, `covers`, `dispositions` and `specialize`, which the schema declares and
/// which 166 edges in the model use, while accepting three kinds the schema never declared. A user
/// who read the schema and declared a viewpoint over `derivedFrom` was told the schema's own
/// vocabulary was unknown. Deriving makes that class of drift unrepresentable.
fn known_edges() -> &'static std::collections::HashSet<String> {
    static K: std::sync::LazyLock<std::collections::HashSet<String>> =
        std::sync::LazyLock::new(crate::schema::edge_kinds);
    &K
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::Str(s) | Value::Ident(s) => s.clone(),
        Value::Int(n) => n.to_string(),
        Value::EnumLit { member, .. } => member.clone(),
        // A multi-valued assignment rendered as a single scalar is a lossy view by nature. Joining
        // with ", " keeps every element VISIBLE rather than showing the first and dropping the rest,
        // which is the failure a caller would not notice. Callers needing the elements individually —
        // reference resolution, edge building — must match on `Value::Seq` directly; this function is
        // for display and for attribute lookups that are single-valued by schema.
        Value::Seq(items) => items.iter().map(value_to_string).collect::<Vec<_>>().join(", "),
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

/// Process-global memo of the last-built model, keyed by a content fingerprint (perf: a serve
/// page-load burst fires ~8 views that each call `Model::build`; without this they each re-parse all
/// ~260 files — slow on I/O-heavy hosts, e.g. Windows Defender scanning each read). Regenerable cache,
/// never truth (§2.1) — invalidated automatically when any file changes.
static MODEL_CACHE: std::sync::Mutex<Option<(u64, Model)>> = std::sync::Mutex::new(None);
/// Serializes BUILDS so a concurrent cold burst does ONE parse (others wait, then hit the cache).
static MODEL_BUILD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

impl Model {
    /// The cached model if its fingerprint matches `fp`.
    fn cached_model(fp: u64) -> Option<Self> {
        MODEL_CACHE.lock().ok().and_then(|g| g.as_ref().filter(|(c, _)| *c == fp).map(|(_, m)| m.clone()))
    }

    /// Build the model, MEMOIZED by content fingerprint (see [`MODEL_CACHE`]). A burst of concurrent
    /// callers on an unchanged model shares one parse; the cache invalidates on any file change.
    fn build(root: &Path) -> Result<Self, ViewError> {
        crate::perf::add(&crate::perf::BUILD_CALLS, 1);
        let fp = crate::fingerprint::of(root);
        if let Some(m) = Self::cached_model(fp) {
            crate::perf::add(&crate::perf::CACHE_HITS, 1);
            return Ok(m);
        }
        let _bl = MODEL_BUILD_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(m) = Self::cached_model(fp) {
            crate::perf::add(&crate::perf::CACHE_HITS, 1);
            return Ok(m); // another thread built it while we waited
        }
        let model = crate::perf::timed(&crate::perf::PARSE_NANOS, || Self::build_uncached(root))?;
        if let Ok(mut g) = MODEL_CACHE.lock() {
            *g = Some((fp, model.clone()));
        }
        Ok(model)
    }

    fn build_uncached(root: &Path) -> Result<Self, ViewError> {
        let dirs = model_dirs(root);
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
                // issue102 construct 2/6: use case USAGES were skipped entirely, so all 56 were absent
                // from the model — no type, no attributes, unreachable by any view or trace. Ingested
                // exactly like any other typed item; the model keys on `type_name`, so they surface as
                // `UseCase` rather than as parts.
                Item::UseCase(u) => add_item(items, &u.name, u.type_name.as_deref(), &u.attributes, None, file),
                // D0143: a typed action usage is ingested like any other typed item. The Model keys on
                // `type_name`, so a retyped Process still surfaces as `Process`.
                Item::ActionUsage(a) => add_item(items, &a.name, a.type_name.as_deref(), &a.attributes, None, file),
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
                    // `flow from A.out to B.in` (issue102): emit at the granularity the model actually
                    // has. Endpoints are dotted feature paths, and the model knows the ROOT (an action
                    // in this def) but not its ports, so the edge connects the roots. Taking the root
                    // rather than the whole path is what makes the edge resolvable at all; the full
                    // path stays in the AST for any consumer that later gains feature resolution.
                    for f in &ad.flows {
                        let root = |s: &str| s.split('.').next().unwrap_or(s).to_string();
                        edges.push(Edge { kind: "flow".to_string(), from: root(&f.from), to: root(&f.to) });
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

/// Traversal direction for a configurable slice (viewerConfigurableSlice, N-2/N-4/N-10).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SliceDir {
    /// Follow edges FROM the node (`from == node` -> `to`); the "downstream" reach.
    Down,
    /// Follow edges INTO the node (`to == node` -> `from`); the "what depends on this" reach (change-impact).
    Up,
    /// Both directions (the neighbourhood reach).
    Both,
}

impl SliceDir {
    /// Parse `down` / `up` / `both` (default `both`).
    pub(crate) fn parse(s: &str) -> Self {
        match s {
            "down" => Self::Down,
            "up" => Self::Up,
            _ => Self::Both,
        }
    }
}

/// A CONFIGURABLE slice from `seed` (viewerConfigurableSlice, N-2/N-4).
///
/// BFS to `depth` hops over the selected edge KINDS (`edges` empty = all) in `dir`; `depth=0` = seed
/// only; unknown seed -> empty. Generalizes [`element_neighbourhood`] (depth-1, `Both`, all-edges).
/// Powers seed-and-expand (N-2), cross-cutting slices (N-4), and change-impact (N-10, `dir=Up`).
#[must_use]
fn configurable_slice(model: &Model, seed: &str, depth: usize, edges: &HashSet<String>, dir: SliceDir) -> HashSet<String> {
    let mut set = HashSet::new();
    if !model.items.contains_key(seed) {
        return set;
    }
    set.insert(seed.to_string());
    let mut frontier = vec![seed.to_string()];
    for _ in 0..depth {
        let mut next = Vec::new();
        for node in &frontier {
            for e in &model.edges {
                if !edges.is_empty() && !edges.contains(&e.kind) {
                    continue;
                }
                let neighbour = match dir {
                    SliceDir::Down => (e.from == *node).then_some(&e.to),
                    SliceDir::Up => (e.to == *node).then_some(&e.from),
                    SliceDir::Both => {
                        if e.from == *node {
                            Some(&e.to)
                        } else if e.to == *node {
                            Some(&e.from)
                        } else {
                            None
                        }
                    }
                };
                if let Some(n) = neighbour {
                    if set.insert(n.clone()) {
                        next.push(n.clone());
                    }
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    set
}

/// Change-impact (srViewerChangeImpact / N-10): from `seed`, the elements reachable over `edges` in
/// `dir`, GROUPED BY DISTANCE (BFS level; each node counted once at its shortest distance, so cycles
/// terminate). Dependents are typically `dir=up` (edges pointing AT the seed). Empty `byDistance` means
/// nothing is reachable — a leaf ("nothing depends on this").
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub(crate) fn change_impact_json(root: &Path, seed: &str, edges: &HashSet<String>, dir: SliceDir) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let dir_label = match dir { SliceDir::Down => "down", SliceDir::Up => "up", SliceDir::Both => "both" };
    let mut dist: HashMap<String, usize> = HashMap::new();
    if model.items.contains_key(seed) {
        let mut frontier = vec![seed.to_string()];
        let mut level = 0usize;
        while !frontier.is_empty() {
            level += 1;
            let mut next = Vec::new();
            for node in &frontier {
                for e in &model.edges {
                    if !edges.is_empty() && !edges.contains(&e.kind) {
                        continue;
                    }
                    let neighbour = match dir {
                        SliceDir::Down => (e.from == *node).then_some(&e.to),
                        SliceDir::Up => (e.to == *node).then_some(&e.from),
                        SliceDir::Both => {
                            if e.from == *node { Some(&e.to) } else if e.to == *node { Some(&e.from) } else { None }
                        }
                    };
                    if let Some(n) = neighbour {
                        if n != seed && !dist.contains_key(n) {
                            dist.insert(n.clone(), level);
                            next.push(n.clone());
                        }
                    }
                }
            }
            frontier = next;
        }
    }
    let max_d = dist.values().copied().max().unwrap_or(0);
    let mut groups: Vec<Json> = Vec::new();
    for d in 1..=max_d {
        let mut names: Vec<&String> = dist.iter().filter(|(_, &v)| v == d).map(|(n, _)| n).collect();
        names.sort();
        let items: Vec<Json> = names
            .iter()
            .filter_map(|n| model.items.get(*n).map(|info| Json::Obj(vec![
                ("name".to_string(), Json::s((*n).clone())),
                ("type".to_string(), Json::s(info.type_name.clone())),
                ("title".to_string(), Json::s(info.attrs.get("title").cloned().unwrap_or_default())),
            ])))
            .collect();
        groups.push(Json::Obj(vec![
            ("distance".to_string(), Json::Int(i64::try_from(d).unwrap_or(0))),
            ("count".to_string(), Json::Int(i64::try_from(items.len()).unwrap_or(0))),
            ("items".to_string(), Json::Arr(items)),
        ]));
    }
    Ok(Json::Obj(vec![
        ("view".to_string(), Json::s("change-impact (srViewerChangeImpact/N-10): elements reachable from the focus, grouped by distance; cycles counted once".to_string())),
        ("seed".to_string(), Json::s(seed.to_string())),
        ("direction".to_string(), Json::s(dir_label.to_string())),
        ("impacted".to_string(), Json::Int(i64::try_from(dist.len()).unwrap_or(0))),
        ("note".to_string(), Json::s(if dist.is_empty() { "nothing depends on this (leaf) — or unknown focus".to_string() } else { String::new() })),
        ("byDistance".to_string(), Json::Arr(groups)),
    ])
    .dump())
}

/// Viewpoint SNAPSHOT (viewerExportShare / N-12): the slice for `(seed, depth, edges, dir)` STAMPED with
/// provenance — the source `commit`, its `as_of` date, and the scope — so it round-trips (re-running the
/// scope at that commit reproduces the view). Oversized slices are capped to a scoped subset with a note.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub(crate) fn snapshot_json(root: &Path, seed: &str, depth: usize, edges: &HashSet<String>, dir: SliceDir, commit: &str, as_of: &str) -> Result<String, ViewError> {
    const CAP: usize = 500;
    let model = Model::build(root)?;
    let mut names: Vec<String> = configurable_slice(&model, seed, depth, edges, dir).into_iter().collect();
    names.sort();
    let total = names.len();
    let truncated = total > CAP;
    names.truncate(CAP);
    let nameset: HashSet<&String> = names.iter().collect();
    let items: Vec<Json> = names
        .iter()
        .filter_map(|n| model.items.get(n).map(|info| Json::Obj(vec![
            ("name".to_string(), Json::s(n.clone())),
            ("type".to_string(), Json::s(info.type_name.clone())),
            ("title".to_string(), Json::s(info.attrs.get("title").cloned().unwrap_or_default())),
        ])))
        .collect();
    let edges_json: Vec<Json> = model
        .edges
        .iter()
        .filter(|e| nameset.contains(&e.from) && nameset.contains(&e.to))
        .map(|e| Json::Obj(vec![("kind".to_string(), Json::s(e.kind.clone())), ("from".to_string(), Json::s(e.from.clone())), ("to".to_string(), Json::s(e.to.clone()))]))
        .collect();
    let dir_label = match dir { SliceDir::Down => "down", SliceDir::Up => "up", SliceDir::Both => "both" };
    let mut edge_list: Vec<&String> = edges.iter().collect();
    edge_list.sort();
    let scope = Json::Obj(vec![
        ("seed".to_string(), Json::s(seed.to_string())),
        ("depth".to_string(), Json::Int(i64::try_from(depth).unwrap_or(0))),
        ("dir".to_string(), Json::s(dir_label.to_string())),
        ("edges".to_string(), Json::Arr(edge_list.into_iter().map(|e| Json::s(e.clone())).collect())),
    ]);
    let snapshot = Json::Obj(vec![
        ("commit".to_string(), Json::s(commit.to_string())),
        ("asOf".to_string(), Json::s(as_of.to_string())),
        ("scope".to_string(), scope),
        ("itemCount".to_string(), Json::Int(i64::try_from(total).unwrap_or(0))),
        ("truncated".to_string(), Json::Bool(truncated)),
        ("note".to_string(), Json::s(if truncated { format!("scoped subset: {total} elements exceeded the {CAP} cap — re-run the scope at commit {commit} for the full view") } else { String::new() })),
    ]);
    Ok(Json::Obj(vec![
        ("snapshot".to_string(), snapshot),
        ("seed".to_string(), Json::s(seed.to_string())),
        ("count".to_string(), Json::Int(i64::try_from(names.len()).unwrap_or(0))),
        ("items".to_string(), Json::Arr(items)),
        ("edges".to_string(), Json::Arr(edges_json)),
    ])
    .dump())
}

/// Build the model AS OF a git `commit` by checking that commit out into a throwaway `git worktree`,
/// building the model there, and removing the worktree. Used by baseline-compare (N-13).
fn model_at_commit(root: &Path, commit: &str, tag: &str) -> Result<Model, ViewError> {
    let root_s = root.to_string_lossy().to_string();
    let tmp = std::env::temp_dir().join(format!("keel-bl-{tag}-{}", commit.replace(|c: char| !c.is_alphanumeric(), "")));
    let tmp_s = tmp.to_string_lossy().to_string();
    let _ = crate::gitx::git().args(["-C", &root_s, "worktree", "remove", "--force", &tmp_s]).output();
    let added = crate::gitx::git()
        .args(["-C", &root_s, "worktree", "add", "--detach", "--quiet", &tmp_s, commit])
        .output()
        .is_ok_and(|o| o.status.success());
    if !added {
        return Err(ViewError::Track("baseline-compare".to_string(), format!("cannot check out commit '{commit}' (unknown ref?)")));
    }
    let model = Model::build(&tmp);
    let _ = crate::gitx::git().args(["-C", &root_s, "worktree", "remove", "--force", &tmp_s]).output();
    model
}

/// Baseline compare (viewerBaselineCompare / N-13): diff the viewpoint (the `(seed, depth, edges, dir)`
/// slice) between two commits `from`→`to`, classifying each element added / removed / changed / reverified
/// / unchanged from git history. An element in `to` but not `from` is "added since"; no differences reads
/// "no drift".
///
/// # Errors
/// Returns [`ViewError`] if either commit cannot be checked out or a model fails to build.
pub(crate) fn baseline_compare_json(root: &Path, seed: &str, from: &str, to: &str, depth: usize, edges: &HashSet<String>, dir: SliceDir) -> Result<String, ViewError> {
    let m_from = model_at_commit(root, from, "from")?;
    let m_to = model_at_commit(root, to, "to")?;
    let s_from = configurable_slice(&m_from, seed, depth, edges, dir);
    let s_to = configurable_slice(&m_to, seed, depth, edges, dir);
    let mut union: Vec<String> = s_from.union(&s_to).cloned().collect();
    union.sort();

    let (mut added, mut removed, mut changed, mut reverified) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut unchanged = 0usize;
    let entry = |n: &str, m: &Model| -> Json {
        let ty = m.items.get(n).map_or("", |i| i.type_name.as_str());
        Json::Obj(vec![("name".to_string(), Json::s(n.to_string())), ("type".to_string(), Json::s(ty.to_string()))])
    };
    for n in &union {
        let in_from = s_from.contains(n);
        let in_to = s_to.contains(n);
        if in_to && !in_from {
            added.push(entry(n, &m_to));
        } else if in_from && !in_to {
            removed.push(entry(n, &m_from));
        } else {
            let a = m_from.items.get(n);
            let b = m_to.items.get(n);
            let same = a.map(|i| (&i.type_name, &i.attrs)) == b.map(|i| (&i.type_name, &i.attrs));
            if same {
                unchanged += 1;
            } else {
                // a verification element whose judged basis moved = "re-verified"
                let is_reverify = b.is_some_and(|i| i.type_name == "TestResult")
                    && a.and_then(|i| i.attrs.get("judgedAgainst")) != b.and_then(|i| i.attrs.get("judgedAgainst"));
                if is_reverify { reverified.push(entry(n, &m_to)); } else { changed.push(entry(n, &m_to)); }
            }
        }
    }
    let drift = added.len() + removed.len() + changed.len() + reverified.len();
    Ok(Json::Obj(vec![
        ("view".to_string(), Json::s("baseline-compare (srViewerBaselineCompare/N-13): the viewpoint diffed between two commits".to_string())),
        ("seed".to_string(), Json::s(seed.to_string())),
        ("from".to_string(), Json::s(from.to_string())),
        ("to".to_string(), Json::s(to.to_string())),
        ("note".to_string(), Json::s(if drift == 0 { "no drift".to_string() } else { String::new() })),
        ("added".to_string(), Json::Arr(added)),
        ("removed".to_string(), Json::Arr(removed)),
        ("changed".to_string(), Json::Arr(changed)),
        ("reverified".to_string(), Json::Arr(reverified)),
        ("unchanged".to_string(), Json::Int(i64::try_from(unchanged).unwrap_or(0))),
    ])
    .dump())
}

/// Parse a `… def <Name>` type-definition header line → `Name` (the declared item type). Recognises the
/// `def` keywords used in `schema/core`; `None` for non-def lines. Powers the generative-UI schema
/// exposure (`viewerSchemaApi`/N-17) — the parser skips type-def bodies, so this text-scans them.
fn def_name(line: &str) -> Option<String> {
    let idx = line.find(" def ")?;
    let first = line[..idx].split_whitespace().next()?;
    if !matches!(
        first,
        "part" | "requirement" | "attribute" | "occurrence" | "item" | "enum" | "abstract" | "use" | "action" | "connection" | "port" | "interface" | "metadata"
    ) {
        return None;
    }
    let name = line.get(idx + 5..)?.split(|c: char| c.is_whitespace() || c == '{' || c == ':' || c == ';').find(|s| !s.is_empty())?;
    name.chars().next().filter(char::is_ascii_alphabetic).map(|_| name.to_string())
}

/// Parse an `attribute <name> : <Type>…` member line → `(name, type)` (type stripped of `[..]`/default/
/// `;`). `None` for non-attribute lines.
fn attr_field(line: &str) -> Option<(String, String)> {
    let rest = line.trim().strip_prefix("attribute ")?;
    let (name, ty) = rest.split_once(':')?;
    let name = name.trim();
    let ty = ty.trim().split(|c: char| c.is_whitespace() || c == ';' || c == '[' || c == '{').find(|s| !s.is_empty()).unwrap_or("");
    (!name.is_empty() && !ty.is_empty()).then(|| (name.to_string(), ty.to_string()))
}

/// The declared item types + their attribute fields (viewerSchemaApi / N-17 / D0117): the machine-readable
/// declared-model substrate a generative UI reads to build display + edit forms. Text-scans `schema/`
/// (the parser skips type-def bodies); brace-depth tracks each def's body. New types/attributes appear
/// automatically — nothing hardcoded.
///
/// # Errors
/// Returns [`ViewError`] never in practice (best-effort text scan); the `Result` signature is required
/// by the `cached` compute contract in serve.
type SchemaTypeDef = (String, Vec<(String, String)>);
type SchemaEnumDef = (String, Vec<String>);

/// In-progress type-def accumulator during the schema text-scan.
struct DefAcc {
    name: String,
    depth: i32,
    attrs: Vec<(String, String)>,
}

/// Per-(type,attribute) observed-value aggregate over the instances.
#[derive(Default)]
struct AttrAgg {
    count: usize,
    distinct: std::collections::BTreeSet<String>,
    min_num: Option<f64>,
    max_num: Option<f64>,
    min_str: Option<String>,
    max_str: Option<String>,
}

/// Text-scan `schema/` for declared type defs (+ attribute fields) and enum defs (+ members), sorted.
fn scan_schema_defs(root: &Path) -> (Vec<SchemaTypeDef>, Vec<SchemaEnumDef>) {
    let mut types: Vec<SchemaTypeDef> = Vec::new();
    let mut enums: Vec<SchemaEnumDef> = Vec::new();
    for path in crate::collect_sysml(&root.join(".engine").join("schema")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let mut depth: i32 = 0;
        let mut cur: Option<DefAcc> = None;
        for line in text.lines() {
            if let Some(e) = enum_def(line) {
                if !enums.iter().any(|(n, _)| n == &e.0) {
                    enums.push(e);
                }
            }
            if cur.is_none() {
                if let Some(name) = def_name(line) {
                    cur = Some(DefAcc { name, depth, attrs: Vec::new() });
                }
            } else if let (Some(pair), Some(acc)) = (attr_field(line), cur.as_mut()) {
                acc.attrs.push(pair);
            }
            depth += i32::try_from(line.matches('{').count()).unwrap_or(0) - i32::try_from(line.matches('}').count()).unwrap_or(0);
            if cur.as_ref().is_some_and(|acc| depth <= acc.depth) {
                if let Some(acc) = cur.take() {
                    types.push((acc.name, acc.attrs));
                }
            }
        }
    }
    types.sort_by(|a, b| a.0.cmp(&b.0));
    enums.sort_by(|a, b| a.0.cmp(&b.0));
    (types, enums)
}

/// Classify a declared attribute type into an encoding scale kind (N-18/D0120).
fn stat_kind(declared: &str, enum_names: &HashSet<String>) -> &'static str {
    if enum_names.contains(declared) {
        "enum"
    } else if declared == "Integer" || declared == "Real" {
        "numeric"
    } else if declared == "Timestamp" {
        "temporal"
    } else {
        "categorical"
    }
}

/// Per-(type,attribute) value stats from the INSTANCES → JSON — the semantic metadata an auto-encoder
/// needs to pick a scale (enum -> ordinal over members; Integer/Real -> continuous/binnable range;
/// Timestamp -> temporal range; String -> categorical distinct). N-18/D0120 encoding-viewpoints.
fn attribute_stats(model: &Model, decl: &HashMap<(String, String), String>, enum_names: &HashSet<String>) -> Vec<Json> {
    let mut agg: std::collections::BTreeMap<(String, String), AttrAgg> = std::collections::BTreeMap::new();
    for info in model.items.values() {
        for (k, v) in &info.attrs {
            let key = (info.type_name.clone(), k.clone());
            if !decl.contains_key(&key) {
                continue; // only declared attributes
            }
            let e = agg.entry(key).or_default();
            e.count += 1;
            if e.distinct.len() < 64 {
                e.distinct.insert(v.clone());
            }
            if let Ok(n) = v.parse::<f64>() {
                e.min_num = Some(e.min_num.map_or(n, |m| m.min(n)));
                e.max_num = Some(e.max_num.map_or(n, |m| m.max(n)));
            }
            if e.min_str.as_ref().is_none_or(|s| v < s) {
                e.min_str = Some(v.clone());
            }
            if e.max_str.as_ref().is_none_or(|s| v > s) {
                e.max_str = Some(v.clone());
            }
        }
    }
    agg.into_iter()
        .map(|((tn, an), a)| {
            let declared = decl.get(&(tn.clone(), an.clone())).cloned().unwrap_or_default();
            let kind = stat_kind(&declared, enum_names);
            let truncated = a.distinct.len() >= 64;
            let mut fields = vec![
                ("type".to_string(), Json::s(tn)),
                ("attribute".to_string(), Json::s(an)),
                ("declaredType".to_string(), Json::s(declared)),
                ("kind".to_string(), Json::s(kind.to_string())),
                ("count".to_string(), Json::Int(i64::try_from(a.count).unwrap_or(i64::MAX))),
            ];
            if kind == "numeric" {
                if let (Some(lo), Some(hi)) = (a.min_num, a.max_num) {
                    fields.push(("min".to_string(), Json::s(format!("{lo}"))));
                    fields.push(("max".to_string(), Json::s(format!("{hi}"))));
                }
            } else if kind == "temporal" {
                fields.push(("min".to_string(), a.min_str.map_or(Json::Null, Json::s)));
                fields.push(("max".to_string(), a.max_str.map_or(Json::Null, Json::s)));
            } else {
                fields.push(("distinct".to_string(), Json::Arr(a.distinct.into_iter().map(Json::s).collect())));
                fields.push(("distinctTruncated".to_string(), Json::Bool(truncated)));
            }
            Json::Obj(fields)
        })
        .collect()
}

/// # Errors
/// Returns [`ViewError`] if the model cannot be built (for the instance-derived attribute stats).
pub(crate) fn schema_json(root: &Path) -> Result<String, ViewError> {
    let (types, enums) = scan_schema_defs(root);
    let enum_names: HashSet<String> = enums.iter().map(|(n, _)| n.clone()).collect();
    let decl: HashMap<(String, String), String> = types
        .iter()
        .flat_map(|(tn, attrs)| attrs.iter().map(move |(an, at)| ((tn.clone(), an.clone()), at.clone())))
        .collect();
    let stats_json = attribute_stats(&Model::build(root)?, &decl, &enum_names);
    let type_json: Vec<Json> = types
        .iter()
        .map(|(name, attrs)| {
            let aj: Vec<Json> = attrs
                .iter()
                .map(|(an, at)| Json::Obj(vec![("name".to_string(), Json::s(an.clone())), ("type".to_string(), Json::s(at.clone())), ("isEnum".to_string(), Json::Bool(enum_names.contains(at)))]))
                .collect();
            Json::Obj(vec![("name".to_string(), Json::s(name.clone())), ("attributes".to_string(), Json::Arr(aj))])
        })
        .collect();
    let enum_json: Vec<Json> = enums
        .iter()
        .map(|(name, members)| Json::Obj(vec![("name".to_string(), Json::s(name.clone())), ("members".to_string(), Json::Arr(members.iter().map(|m| Json::s(m.clone())).collect()))]))
        .collect();
    Ok(Json::Obj(vec![
        ("schema".to_string(), Json::s("declared item types + attribute fields + enum members + per-attribute value stats (viewerSchemaApi/N-17; encoding-semantics N-18/D0120) — the generative-UI + auto-encoding substrate; pair with /api/launchables for actions".to_string())),
        ("types".to_string(), Json::Arr(type_json)),
        ("enums".to_string(), Json::Arr(enum_json)),
        ("attributeStats".to_string(), Json::Arr(stats_json)),
    ])
    .dump())
}

/// Parse a single-line `enum def <Name> { a; b; c; }` → `(Name, [members])`; `None` otherwise.
/// Powers the auto-encoding semantics (N-18/D0120): an enum-typed attribute's domain = its members.
fn enum_def(line: &str) -> Option<(String, Vec<String>)> {
    let rest = line.trim().strip_prefix("enum def ")?;
    let (head, body) = rest.split_once('{')?;
    let name = head.split_whitespace().next()?.to_string();
    let members: Vec<String> = body.trim_end().trim_end_matches('}').split(';').map(|m| m.trim().to_string()).filter(|m| !m.is_empty()).collect();
    if name.is_empty() || members.is_empty() {
        None
    } else {
        Some((name, members))
    }
}

/// ISO-8601 (`YYYY-MM-DD…`) lexicographic date-range test: `val` in `[since, until]` (either bound
/// optional). ISO-8601 sorts chronologically as strings, so no date parsing is needed (N-5).
fn date_in_range(val: &str, since: Option<&str>, until: Option<&str>) -> bool {
    since.is_none_or(|s| val >= s) && until.is_none_or(|u| val <= u)
}

/// An optional date-range filter for a slice (N-5 time-as-query-filter): keep members whose `attr` date
/// (e.g. `judgedAt`) is in `[since, until]`. `attr = None` disables the filter.
#[derive(Default, Clone, Copy)]
pub(crate) struct DateFilter<'a> {
    pub attr: Option<&'a str>,
    pub since: Option<&'a str>,
    pub until: Option<&'a str>,
}

/// Configurable-slice view (viewerConfigurableSlice / `/api/slice`): the induced subgraph of
/// [`configurable_slice`], emitted like a section.
///
/// TIME-AS-QUERY-FILTER (N-5, D0116): when `date_attr` is set, keep only members whose that-attribute
/// (e.g. `judgedAt`, `createdAt`) falls in `[since, until]` — "reviewed/created since/until/in a range".
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
#[allow(clippy::implicit_hasher)]
pub(crate) fn slice_json(root: &Path, seed: &str, depth: usize, edges: &HashSet<String>, dir: SliceDir, df: DateFilter) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut names = configurable_slice(&model, seed, depth, edges, dir);
    if let Some(attr) = df.attr {
        names.retain(|n| model.items.get(n).and_then(|i| i.attrs.get(attr)).is_some_and(|v| date_in_range(v, df.since, df.until)));
    }
    Ok(section_subgraph_json(&model, &names, seed, "slice"))
}

/// System-bound analysis slices (srViewerSystemBoundViews, D0126) — dynamic views from a shifting focus.
///
/// `kind`:
/// - `children` — the DOWNSTREAM closure of the focus (what it decomposes into / governs).
/// - `ancestry` — the UPSTREAM closure (the trace chain back to the governing Need/Decision).
/// - `siblings` — items of the SAME TYPE as the focus that share a parent with it (same source of an
///   incoming edge) — e.g. the other `SystemRequirement`s satisfying the same Need.
///
/// Pure computed views over the typed graph (no stored state); rendered in the slice subgraph shape so a
/// client draws them identically. Elements superpose + compose linearly along typed edges (the human's
/// intuition), so each analysis is just a directional/relational cut.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub(crate) fn relations_json(root: &Path, focus: &str, kind: &str) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    if !model.items.contains_key(focus) {
        return Ok(section_subgraph_json(&model, &HashSet::new(), focus, kind));
    }
    let no_edges: HashSet<String> = HashSet::new();
    let names: HashSet<String> = match kind {
        "ancestry" => configurable_slice(&model, focus, 20, &no_edges, SliceDir::Up),
        "siblings" => {
            let ty = model.items.get(focus).map_or_else(String::new, |i| i.type_name.clone());
            let parents: HashSet<&str> = model.edges.iter().filter(|e| e.to == focus).map(|e| e.from.as_str()).collect();
            let mut set: HashSet<String> = std::iter::once(focus.to_string()).chain(parents.iter().map(|p| (*p).to_string())).collect();
            for e in &model.edges {
                if parents.contains(e.from.as_str()) && e.to != focus && model.items.get(&e.to).is_some_and(|i| i.type_name == ty) {
                    set.insert(e.to.clone());
                }
            }
            set
        }
        _ => configurable_slice(&model, focus, 20, &no_edges, SliceDir::Down), // "children" (default)
    };
    Ok(section_subgraph_json(&model, &names, focus, kind))
}

/// A deterministic ITERATION PLAN for iterative AI critique (viewerIterativeCritique, N-15): the ordered
/// AXIS (the slice from `seed`) and, per element, its type + local context (neighbour names) + the lens.
///
/// keel supplies the plan (deterministic, testable, shape B); the viewer app drives the existing agent
/// bridge (`/api/agent/stream?action=critique&target=<element>`) once per axis item. Powers UC-14 (a
/// process + its downstream, best-practice lens) and UC-15 (a requirement + its downstream, sufficiency).
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
#[allow(clippy::implicit_hasher)]
pub(crate) fn critique_plan_json(root: &Path, seed: &str, depth: usize, edges: &HashSet<String>, dir: SliceDir, lens: &str) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut axis: Vec<String> = configurable_slice(&model, seed, depth, edges, dir).into_iter().collect();
    axis.sort();
    let items: Vec<Json> = axis
        .iter()
        .map(|el| {
            let mut ctx: Vec<String> = element_neighbourhood(&model, el).into_iter().collect();
            ctx.sort();
            let ty = model.items.get(el).map_or_else(String::new, |i| i.type_name.clone());
            Json::Obj(vec![
                ("element".to_string(), Json::s(el.clone())),
                ("type".to_string(), Json::s(ty)),
                ("target".to_string(), Json::s(format!("/api/agent/stream?action=critique&target={el}"))),
                ("context".to_string(), Json::Arr(ctx.into_iter().map(Json::s).collect())),
            ])
        })
        .collect();
    Ok(Json::Obj(vec![
        ("critiquePlan".to_string(), Json::s("deterministic iteration plan (viewerIterativeCritique/N-15); the viewer drives the agent bridge per axis item".to_string())),
        ("seed".to_string(), Json::s(seed.to_string())),
        ("lens".to_string(), Json::s(lens.to_string())),
        ("count".to_string(), Json::Int(i64::try_from(items.len()).unwrap_or(i64::MAX))),
        ("axis".to_string(), Json::Arr(items)),
    ])
    .dump())
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
                    ("displayLabel".to_string(), Json::s(display_label(n, info))),
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
                    ("displayLabel".to_string(), Json::s(display_label(n, info))),
                ];
                if let Some(t) = info.attrs.get("title") {
                    o.push(("title".to_string(), Json::s(t.clone())));
                }
                if let Some(m) = &info.marker {
                    o.push(("marker".to_string(), Json::s(m.clone())));
                }
                // all authored attribute values — lets a consumer ENCODE by attribute (N-18/D0120)
                let mut aobj: Vec<(String, Json)> = info.attrs.iter().map(|(k, v)| (k.clone(), Json::s(v.clone()))).collect();
                aobj.sort_by(|a, b| a.0.cmp(&b.0));
                o.push(("attrs".to_string(), Json::Obj(aobj)));
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
        if !known_edges().contains(&lc) {
            let mut known: Vec<&str> = known_edges().iter().map(String::as_str).collect();
            known.sort_unstable();
            return Err(ViewError::UnknownEdge {
                view: view.to_string(),
                edge: e.clone(),
                known: known.join(", "),
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

/// The human REVIEW QUEUE (D0121) — user-gated items that need an explicit HUMAN judgment and don't
/// have one yet: (a) proposed Decisions (`status=proposed`), and (b) pending confirmation gates
/// (`method=confirmation` Tests with no PASSING result). Everything else is machine-verifiable or
/// already judged, so it is out of scope. A pure computed view — nothing stored; the `keel serve`
/// "Review" surface renders this and records the human's acceptance back via the write API (D0106:
/// the human's action + note IS the attestation, never AI-fabricated).
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub(crate) fn review_queue_json(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;

    let mut decisions: Vec<&String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Decision" && i.attrs.get("status").map(String::as_str) == Some("proposed"))
        .map(|(n, _)| n)
        .collect();
    decisions.sort();

    let mut gates: Vec<&String> = model
        .items
        .iter()
        .filter(|(n, i)| {
            // truly UNJUDGED confirmation gate: no result yet (a pass=accepted or fail=rejected both
            // count as judged and drop out of the queue).
            i.attrs.get("method").map(String::as_str) == Some("confirmation") && latest_result(&model, n).is_none()
        })
        .map(|(n, _)| n)
        .collect();
    gates.sort();

    let attr = |name: &str, key: &str| json_esc(model.items.get(name).and_then(|i| i.attrs.get(key)).map_or("", String::as_str));
    let file_of = |name: &str| json_esc(model.items.get(name).map_or("", |i| i.file.as_str()));
    // Referenced items (both edge directions) — the linked context a reviewer wants one click away
    // (e.g. the Need an acceptance gate gates, the items a Decision derives from / depends on).
    let refs_of = |name: &str| -> String {
        let mut seen = std::collections::BTreeSet::new();
        let mut out = Vec::new();
        for e in &model.edges {
            let (other, dir) = if e.from == name {
                (&e.to, "out")
            } else if e.to == name {
                (&e.from, "in")
            } else {
                continue;
            };
            if other != name && seen.insert((e.kind.clone(), other.clone())) {
                out.push(format!("{{\"kind\":\"{}\",\"dir\":\"{dir}\",\"name\":\"{}\"}}", json_esc(&e.kind), json_esc(other)));
            }
        }
        format!("[{}]", out.join(","))
    };

    let mut items: Vec<String> = Vec::with_capacity(decisions.len() + gates.len());
    for n in &decisions {
        items.push(format!(
            "{{\"kind\":\"decision\",\"name\":\"{n}\",\"ceremony\":false,\"title\":\"{}\",\"context\":\"{}\",\"decision\":\"{}\",\"rationale\":\"{}\",\"consequences\":\"{}\",\"file\":\"{}\",\"references\":{}}}",
            attr(n, "title"), attr(n, "context"), attr(n, "decision"), attr(n, "rationale"), attr(n, "consequences"), file_of(n), refs_of(n)
        ));
    }
    let phases = declared_workflow_phases(root);
    let mut actionable_gates = 0usize;
    for n in &gates {
        let ceremony = is_ceremony_gate(n, &phases);
        if !ceremony {
            actionable_gates += 1;
        }
        items.push(format!(
            "{{\"kind\":\"gate\",\"name\":\"{n}\",\"ceremony\":{ceremony},\"title\":\"{}\",\"procedureText\":\"{}\",\"file\":\"{}\",\"references\":{}}}",
            attr(n, "title"), attr(n, "procedureText"), file_of(n), refs_of(n)
        ));
    }
    let count = items.len();
    let actionable = decisions.len() + actionable_gates;
    Ok(format!(
        "{{\n  \"queue\": \"user-gated items awaiting human judgment (D0121): proposed Decisions + pending confirmation gates. 'ceremony' gates are legacy pre-D0049/D0051 sprint DoD/retro confirmations (autonomous now) — filtered from the actionable default.\",\n  \"count\": {count},\n  \"actionable\": {actionable},\n  \"proposedDecisions\": {},\n  \"pendingGates\": {},\n  \"items\": [\n    {}\n  ]\n}}",
        decisions.len(),
        gates.len(),
        items.join(",\n    ")
    ))
}

/// The phase-name vocabulary DECLARED by the project's own workflows — the sub-`action`s of each
/// `action def` in `.engine/workflows/*.sysml` (e.g. refine/standup/implement/review/closeOut/retro
/// for Delivery). Read from the declared model so ceremony detection ADAPTS to the project's processes
/// rather than assuming keel's names. Empty if no workflows are declared (then nothing is ceremony).
fn declared_workflow_phases(root: &Path) -> Vec<String> {
    let dir = root.join(".engine").join("workflows");
    let Ok(entries) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut phases = Vec::new();
    let mut in_def = 0i32; // brace depth inside an `action def`
    for entry in entries.flatten() {
        if entry.path().extension().is_none_or(|e| e != "sysml") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(entry.path()) else { continue };
        for raw in text.lines() {
            let line = raw.trim();
            if line.starts_with("action def ") {
                in_def = 1;
                continue;
            }
            if in_def > 0 {
                in_def += i32::try_from(line.matches('{').count()).unwrap_or(0);
                in_def -= i32::try_from(line.matches('}').count()).unwrap_or(0);
                // a phase is a sub-action: `action <name> { ... }` (not `action def`, not a flow/first)
                if let Some(rest) = line.strip_prefix("action ") {
                    if let Some(name) = rest.split([' ', '{', ';']).next() {
                        if !name.is_empty() && name != "def" && !phases.iter().any(|p| p == name) {
                            phases.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    phases
}

/// True for a sprint-internal ceremony / definition-of-done confirmation gate — legacy pre-D0049/D0051
/// `method=confirmation` gates that are autonomous now (inspect/analyze) and are NOT things a human
/// signs off. Derived from the project's DECLARED workflow `phases` (a gate named `<x><Phase>Gate` for
/// a declared phase), plus the `DoD` definition-of-done convention. No hardcoded keel phase list.
fn is_ceremony_gate(name: &str, phases: &[String]) -> bool {
    if name.ends_with("DoD") {
        return true;
    }
    let lower = name.to_ascii_lowercase();
    phases.iter().any(|p| lower.ends_with(&format!("{}gate", p.to_ascii_lowercase())))
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

/// ACTOR TRACE — everything an actor authored or judged, computed from provenance (issue106).
///
/// The panel's finding (e) was that `createdBy` / `owner` / `judgedBy` are Strings while `part def Actor`
/// exists, so "you cannot select an actor and see what they authored" — the one trace that survives a
/// provenance audit is the one with no edge. The complaint is right. The proposed fix, `ref createdBy :
/// Actor`, is NOT available: `.engine` carries 137 provenance values and the registry is `ProjectActors`,
/// declared as per-project INSTANCE data. Making it a reference would force every engine decision file to
/// import a project's actor registry, inverting the D0093 engine/instance boundary — `.engine` ships to
/// downstream projects via `keel init`, where this project's `claudeOpus5` does not exist.
///
/// So the navigability is delivered as a COMPUTATION over the strings instead, which is the same answer
/// the human reached for `Assumption` (issue105): if it is derivable, derive it. This needs no schema
/// change, no layering violation, and cannot go stale.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn actor_trace(root: &Path, actor: &str) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut authored: Vec<&String> = Vec::new();
    let mut judged: Vec<&String> = Vec::new();
    let mut owned: Vec<&String> = Vec::new();
    for (name, info) in &model.items {
        for (field, bucket) in
            [("createdBy", &mut authored), ("judgedBy", &mut judged), ("owner", &mut owned)]
        {
            if info.attrs.get(field).is_some_and(|v| v == actor) {
                bucket.push(name);
            }
        }
    }
    for b in [&mut authored, &mut judged, &mut owned] {
        b.sort_unstable();
    }
    let arr = |v: &[&String]| {
        v.iter().map(|n| format!("\"{n}\"")).collect::<Vec<_>>().join(", ")
    };
    Ok(format!(
        "{{\n  \"actor\": \"{actor}\",\n  \"note\": \"computed from provenance attributes (issue106). Provenance stays a STRING because `.engine` cannot import a project's actor registry without inverting the D0093 engine/instance boundary; navigability is delivered by computing it.\",\n  \"counts\": {{\"authored\": {}, \"judged\": {}, \"owned\": {}}},\n  \"authored\": [{}],\n  \"judged\": [{}],\n  \"owned\": [{}]\n}}",
        authored.len(),
        judged.len(),
        owned.len(),
        arr(&authored),
        arr(&judged),
        arr(&owned)
    ))
}

/// ASSUMPTIONS — accepted, unverified, and depended upon (issue105).
///
/// The `Assumption` TYPE was deleted as dead schema in D0142, and I recorded that as "the one genuine
/// loss" because `requirements.sysml` had argued the type was first-class: when an assumption breaks,
/// everything derived from it should go suspect. The human's correction is decisive and this function
/// is it — *any accepted unverified requirement that has downstream non-verification dependencies IS an
/// assumption*, so the class is COMPUTABLE and §1.1 makes it a view, not an authored type: "can it be
/// regenerated from other authored facts? Yes → it's a view."
///
/// Nothing was lost, and the capability was never missing. Suspicion ALREADY propagates downstream along
/// satisfy / verify / `:>` / allocate / semantic dependsOn, so "when this breaks, everything derived from
/// it goes suspect" has been working the whole time. What was missing is a lens that NAMES the class, and
/// authoring a type per assumption would have been the dual-truth error the engine exists to prevent —
/// a stored fact restating what the edges already say.
///
/// An element qualifies when NOTHING verifies it and at least one other item depends on it through a
/// non-verification edge. Verification edges are excluded from the dependency test on purpose: a Test
/// pointing at a requirement is what DISCHARGES the assumption, so counting it as a dependent would
/// make every verified requirement look like an assumption.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn assumptions(root: &Path) -> Result<String, ViewError> {
    // Types where verification is the expected discharge. A Story is not here: its discharge is a DoD
    // TestResult, not a `#Verify` edge, so including it would report the entire backlog.
    const SUBJECT_TYPES: [&str; 4] = ["Need", "SystemRequirement", "SubsystemRequirement", "Decision"];
    // Edges that make something DEPEND on the target. `satisfy` runs Need -> SR, so a Need is depended
    // upon when it is the `from`; the rest point AT the thing depended upon.
    const DEPENDS_INBOUND: [&str; 4] = ["derivedfrom", "charteredby", "dependency", "allocate"];

    let model = Model::build(root)?;
    let mut rows: Vec<String> = Vec::new();
    let mut names: Vec<&String> = model
        .items
        .iter()
        .filter(|(_, i)| SUBJECT_TYPES.contains(&i.type_name.as_str()))
        .map(|(n, _)| n)
        .collect();
    names.sort_unstable();
    for name in names {
        if model.edges.iter().any(|e| e.kind == "verify" && &e.to == name) {
            continue; // verified — the assumption is discharged
        }
        let mut deps: Vec<String> = model
            .edges
            .iter()
            .filter(|e| {
                (&e.to == name && DEPENDS_INBOUND.contains(&e.kind.as_str()))
                    || (&e.from == name && e.kind == "satisfy")
            })
            .map(|e| {
                let other = if &e.to == name { &e.from } else { &e.to };
                format!("{{\"kind\":\"{}\",\"item\":\"{}\"}}", e.kind, other)
            })
            .collect();
        if deps.is_empty() {
            continue; // unverified but nothing rests on it — not an assumption, just unverified
        }
        deps.sort();
        deps.dedup();
        let ty = model.items.get(name).map_or("", |i| i.type_name.as_str());
        rows.push(format!(
            "{{\"name\":\"{name}\",\"type\":\"{ty}\",\"dependentCount\":{},\"dependents\":[{}]}}",
            deps.len(),
            deps.join(", ")
        ));
    }
    Ok(format!(
        "{{\n  \"note\": \"an ASSUMPTION is computed, never authored (issue105): unverified, and something depends on it through a non-verification edge. Suspicion already propagates downstream, so this lens names the class rather than creating it.\",\n  \"count\": {},\n  \"assumptions\": [{}]\n}}",
        rows.len(),
        rows.join(", ")
    ))
}

/// Marker census: EDGE count and PROSE count per marker (issue099).
///
/// Exists because I recorded `#Verify` at 613, `#DerivedFrom` at 91 and `#DependsOn` at 91 in an
/// ACCEPTED Decision. The real edge counts are 456, 37 and 42. The inflation came from `grep -c
/// '#Verify'`, which also matches the marker name inside `description` and `procedureText` prose —
/// including, recursively, the text discussing the migration itself, so the number GROWS as more is
/// written about it. A corpus containing prose about itself inflates its own census.
///
/// CLAUDE.md §4 requires a bulk migration to reconcile CONTROL TOTALS. An inflated total guarantees the
/// reconciliation either fails for the wrong reason or is quietly adjusted to match a wrong baseline, so
/// this figure must come from one computed place rather than from a fresh grep each time.
///
/// Edges are counted from the AST (`Item::Dependency`), which cannot see prose. The prose figure is the
/// remainder against a raw text scan — reported SEPARATELY rather than subtracted away, because the gap
/// between the two is the tell that a hand-rolled count is about to be wrong.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn marker_census(root: &Path) -> Result<String, ViewError> {
    let mut edges: BTreeMap<String, usize> = BTreeMap::new();
    let mut raw: BTreeMap<String, usize> = BTreeMap::new();
    for dir in [root.join(".tracking"), root.join(".engine")] {
        if !dir.is_dir() {
            continue;
        }
        for path in crate::collect_sysml(&dir) {
            if let Ok(pkg) = crate::parse_pkg(&path) {
                for item in &pkg.items {
                    if let Item::Dependency(d) = item {
                        *edges.entry(d.marker.clone()).or_default() += 1;
                    }
                }
            }
            if let Ok(text) = std::fs::read_to_string(&path) {
                for m in crate::guards::engine_markers() {
                    let n = text.matches(&format!("#{m}")).count();
                    if n > 0 {
                        *raw.entry(m.clone()).or_default() += n;
                    }
                }
            }
        }
    }
    let mut names: Vec<&String> = raw.keys().chain(edges.keys()).collect();
    names.sort_unstable();
    names.dedup();
    let rows: Vec<String> = names
        .iter()
        .map(|n| {
            let e = edges.get(*n).copied().unwrap_or(0);
            let r = raw.get(*n).copied().unwrap_or(0);
            format!(
                "{{\"marker\":\"{n}\",\"edges\":{e},\"proseMentions\":{}}}",
                r.saturating_sub(e)
            )
        })
        .collect();
    Ok(format!(
        "{{\n  \"note\": \"edges are AST-counted and are the CONTROL TOTAL for any migration; proseMentions are marker names appearing inside strings and must never be counted as edges (issue099)\",\n  \"markers\": [{}]\n}}",
        rows.join(", ")
    ))
}

/// Typed-edge endpoints that resolve to NO declared item (issue109).
///
/// A `#Marker dependency from A to B;` whose endpoint is declared nowhere is a claim about a
/// relationship that does not exist, and it is worse than a missing edge because every consumer
/// treats it as present: `issue060` read as TRIAGED by a resolver that had never been declared, and
/// sprint171's Story read as CHARTERED by an origin no commit ever contained. Both passed
/// `keel validate`, `keel check-engine` and all 28 guards, because each of those checks that the
/// EDGE is present and none checked that its endpoints resolve.
///
/// Found by the conformance lane rather than by any Rust check — the kernel resolves references and
/// said so, which is exactly the oracle gap issue097 named.
///
/// Reads `Item::Dependency` from the AST, so a marker written inside a `description` string cannot
/// produce a false hit. That is not incidental: a text scan for the same thing reports two extra
/// hits from D0133's own prose ABOUT edges, which is the identical self-referential-corpus trap
/// that inflated the census in issue099.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
/// The population `dangling_edge_endpoints` examines, so its guard can report a scan count (issue180).
///
/// The guard used to hardcode `scanned: 0` while finding real violations, printing
/// `0 scanned, 1 violation(s)` - output that contradicts itself, since a violation cannot be found in an
/// empty population. `scanned` is the only signal separating a guard whose population is legitimately
/// empty from one that is mis-aimed and can never fire.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub fn dangling_edge_endpoints_scanned(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let n = Model::build(root)?.edges.len();
    Ok((n, dangling_edge_endpoints(root)?))
}

/// Typed edges whose endpoint names are not declared items — the `edge-endpoints` guard's finding set.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub fn dangling_edge_endpoints(root: &Path) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    // A DOWNSTREAM project resolves endpoints against one more place than the Model views: the
    // engine's own decisions, which `keel init` scaffolds read-only to `.engine/reference/decisions/`
    // and which `model_dirs` excludes ON PURPOSE (D0093 — the engine's 144 architecture decisions
    // must not enter a project's computed views). The shipped `.engine/rules/rules.sysml` carries
    // `#JustifiedBy` edges to those decisions, so on a freshly inited project every one of them
    // looked dangling and this guard failed the init smoke test.
    //
    // They are NOT dangling: `d0066` is declared, it is simply out of VIEW scope for a different
    // reason. Excluding a name from the views and asserting it does not exist are different claims,
    // and conflating them would have made a new hard guard fail every downstream project on day one —
    // the issue089/issue090 failure this engine has already paid for twice.
    let mut declared_elsewhere: std::collections::HashSet<String> = std::collections::HashSet::new();
    for path in crate::collect_sysml(&root.join(".engine").join("reference").join("decisions")) {
        if let Ok(pkg) = crate::parse_pkg(&path) {
            for item in &pkg.items {
                if let Item::Part(p) = item {
                    declared_elsewhere.insert(p.name.clone());
                }
            }
        }
    }
    let mut out = Vec::new();
    for dir in model_dirs(root) {
        if !dir.is_dir() {
            continue;
        }
        for path in crate::collect_sysml(&dir) {
            let Ok(pkg) = crate::parse_pkg(&path) else { continue };
            let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
            for item in &pkg.items {
                if let Item::Dependency(d) = item {
                    for (side, name) in [("from", &d.from), ("to", &d.to)] {
                        // Qualified names (`Pkg::name`) resolve on their last segment, which is how
                        // the rest of the Model keys items.
                        let base = name.rsplit("::").next().unwrap_or(name);
                        if !model.items.contains_key(base) && !declared_elsewhere.contains(base) {
                            out.push(format!("{rel}: #{} dependency {side} `{base}` — declared nowhere", d.marker));
                        }
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

/// Names that are the TARGET of a `#Supersede` edge — i.e. deliberately retired (§1.4).
///
/// Exists because `ready` ignored supersede entirely (issue100): a task recorded as superseded stayed
/// on the ranked frontier forever, so the authored fact said "retired" while the computed view said
/// "do this next". Since the AI auto-follows the frontier (D0052), that is not a cosmetic
/// disagreement — it actively schedules work a Decision has forbidden. Found when D0140 superseded two
/// migration items that would have silently deleted 493 edges, and they stayed ready.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn superseded_names(root: &Path) -> Result<HashSet<String>, ViewError> {
    let model = Model::build(root)?;
    Ok(model.edges.iter().filter(|e| e.kind == "supersede").map(|e| e.to.clone()).collect())
}

/// Decisions with `status = proposed` — the ones WAITING on the single human gate (issue096).
///
/// Acceptance is the one human gate in an otherwise autonomous loop, and the console's Decisions
/// surface was wired to the `keel decisions` SCORECARD, which filters to accepted and therefore
/// rendered everything except what needs action. It failed silently too: nothing anywhere stated
/// that decisions were waiting, so an unattended proposal is indistinguishable from none.
///
/// Sourced from the model rather than from the scorecard on purpose. The scorecard's accepted-only
/// scope is CORRECT for what it does — scoring citations and critique coverage is meaningful only
/// for a committed decision — so this reads the same authored facts by a different question instead
/// of widening a view that is right as it stands.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn pending_acceptances(root: &Path) -> Result<Vec<String>, ViewError> {
    Ok(proposed_decisions(&Model::build(root)?))
}

/// Is `name` a declared item in the model?
///
/// Exists so a write path can REFUSE before authoring rather than leave the `edge-endpoints` guard
/// to catch it afterwards: `keel record issue` must produce a triaged Issue whose `#Resolves` edge
/// actually lands somewhere, and an edge to a name declared nowhere is worse than a missing edge
/// because every consumer treats it as present (issue109).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn item_exists(root: &Path, name: &str) -> Result<bool, ViewError> {
    Ok(Model::build(root)?.items.contains_key(name))
}

/// Pure core of [`pending_acceptances`], for self-test.
///
/// Matches on the suffix because the authored value is the enum path `DecisionStatus::proposed`,
/// and matching the bare word would also catch a status like `counterproposed` if one were ever
/// added — while matching the full path would silently stop working if the enum were renamed.
fn proposed_decisions(model: &Model) -> Vec<String> {
    let mut pending: Vec<String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Decision")
        .filter(|(_, i)| i.attrs.get("status").is_some_and(|s| s.ends_with("::proposed") || s == "proposed"))
        .map(|(n, _)| n.clone())
        .collect();
    pending.sort();
    pending
}

/// Task names blocked on a human acceptance: they `#DependsOn` a Decision that is still `proposed`
/// (issue112).
///
/// The frontier is AUTO-FOLLOWED (D0052), so an item it ranks is an item the next contributor will
/// start. `dcWorkClaim` needs a `Claim` type that only a human can sign into frozen core, and it
/// nonetheless ranked FIRST — so a successor would rediscover the wall this sprint just hit, and the
/// computed view would have told them the work was ready when it provably was not.
///
/// This is the same defect as issue100 (a superseded task staying ready) with a different cause, and
/// it needs its own predicate: superseded means RETIRED, blocked-on-acceptance means WAITING, and
/// conflating them would either hide work that resumes the moment a human answers, or retire it.
///
/// Exact, not heuristic: a `#DependsOn` edge to a Decision whose `status` is `proposed`. An accepted
/// or rejected Decision unblocks the item with no further edit, because nothing here is stored.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn blocked_on_acceptance(root: &Path) -> Result<HashSet<String>, ViewError> {
    Ok(blocked_by(&Model::build(root)?))
}

/// Pure core of [`blocked_on_acceptance`], for self-test.
fn blocked_by(model: &Model) -> HashSet<String> {
    let pending: HashSet<&String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Decision")
        .filter(|(_, i)| i.attrs.get("status").is_some_and(|s| s.ends_with("::proposed") || s == "proposed"))
        .map(|(n, _)| n)
        .collect();
    model
        .edges
        .iter()
        .filter(|e| e.kind == "dependson" || e.kind == "dependency")
        .filter(|e| pending.contains(&e.to))
        .map(|e| e.from.clone())
        .collect()
}

/// Every `#Resolves` edge as `(resolver, issue, resolver_type)`.
///
/// `resolver_type` is the declared item type, or `""` when the resolver is not a typed item — which is
/// the normal case, since most resolvers are `action` names.
///
/// Feeds `guard resolver-kind`. Separate from [`untriaged_issues`] on purpose: that answers whether an
/// edge EXISTS, and this answers whether the thing on the other end could resolve anything.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn resolves_edges(root: &Path) -> Result<Vec<(String, String, String)>, ViewError> {
    let model = Model::build(root)?;
    let mut out: Vec<(String, String, String)> = model
        .edges
        .iter()
        .filter(|e| e.kind == "resolves")
        .map(|e| (e.from.clone(), e.to.clone(), model.items.get(&e.from).map(|i| i.type_name.clone()).unwrap_or_default()))
        .collect();
    out.sort();
    Ok(out)
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

/// One declared `Viewpoint`, as every consumer of the registry needs it.
pub struct ViewpointRow {
    /// The declaration's element name.
    pub name: String,
    /// Human title (what the guard and the views label it by).
    pub title: String,
    /// The `renderer` string — a `keel` command, or `(planned ...)`.
    pub renderer: String,
    /// `concernText` — the question the lens answers.
    pub concern: String,
    /// The declared top-level surface, empty when undeclared (D0154).
    pub surface: String,
}

/// EVERY declared `Viewpoint`, from the MODEL — the single answer to "what viewpoints exist" (issue139).
///
/// Sourced from the model rather than from `.engine/views/viewpoint-registry.sysml` by name, because
/// reading one hardcoded file gave the engine TWO answers: the model-driven paths saw every Viewpoint
/// while the `viewpoint-renderer` HARD GUARD and `concern-coverage` saw only the registry file's. Proven,
/// not suspected — a probe viewpoint in another views file, with the renderer `keel
/// definitely-not-a-real-command`, passed the guard while it reported "32 scanned, 0 violations".
///
/// It also unblocks per-viewpoint files (issue138): splitting the registry would have silently disabled
/// the guard for all 32, because none of them would have been in the file it read.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn declared_viewpoints(root: &Path) -> Result<Vec<ViewpointRow>, ViewError> {
    let model = Model::build(root)?;
    let mut names: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Viewpoint").map(|(n, _)| n).collect();
    names.sort();
    Ok(names
        .into_iter()
        .filter_map(|n| {
            let i = model.items.get(n)?;
            let g = |k: &str| i.attrs.get(k).cloned().unwrap_or_default();
            Some(ViewpointRow { name: n.clone(), title: g("title"), renderer: g("renderer"), concern: g("concernText"), surface: g("surface") })
        })
        .collect())
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
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn concern_coverage(root: &Path) -> Result<String, ViewError> {
    let mut served: Vec<(String, String, String)> = Vec::new();
    let mut unserved: Vec<(String, String, String)> = Vec::new();
    for vp in declared_viewpoints(root)? {
        let row = (vp.title.clone(), vp.concern.clone(), vp.renderer.clone());
        if vp.renderer.starts_with("(planned") {
            unserved.push(row);
        } else {
            served.push(row);
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

/// Latest `{stem}R<n>` outcome among the model's `TestResult`s (e.g. stem = `dcMintCommandDoD`),
/// optionally scoped to one source file.
fn dft_latest_result(model: &Model, stem: &str, file: Option<&str>) -> Option<String> {
    let prefix = format!("{stem}R");
    let mut best: Option<(u32, String)> = None;
    for (rname, rinfo) in &model.items {
        if rinfo.type_name != "TestResult" || file.is_some_and(|f| rinfo.file != f) {
            continue;
        }
        let Some(n) = rname.strip_prefix(&prefix).and_then(|t| t.parse::<u32>().ok()) else { continue };
        let outcome =
            rinfo.attrs.get("outcome").map(|o| o.rsplit(':').next().unwrap_or(o).to_string()).unwrap_or_default();
        if best.as_ref().is_none_or(|(bn, _)| n > *bn) {
            best = Some((n, outcome));
        }
    }
    best.map(|(_, o)| o)
}

/// An item's evidence state: its own `{item}DoD` results; for a sprint Story, the file's `story…DoD`
/// results (the ceremony convention — the story's doneness IS its sprint `DoD`); else no evidence.
fn dft_evidence(model: &Model, item: &str) -> String {
    if let Some(o) = dft_latest_result(model, &format!("{item}DoD"), None) {
        return o;
    }
    if let Some(info) = model.items.get(item) {
        if info.type_name == "Story" && !info.file.is_empty() {
            let stems: std::collections::BTreeSet<String> = model
                .items
                .iter()
                .filter(|(n, i)| {
                    i.type_name == "TestResult" && i.file == info.file && n.starts_with("story") && n.contains("DoDR")
                })
                .filter_map(|(n, _)| n.rfind("DoDR").map(|k| n[..k + 3].to_string()))
                .collect();
            for stem in stems {
                if let Some(o) = dft_latest_result(model, &stem, Some(&info.file)) {
                    return o;
                }
            }
        }
    }
    "no evidence".to_string()
}

/// `keel decision-follow-through` — the promise-to-work chain, per accepted Decision (us020/issue174).
///
/// Their note, verbatim in st015: "the scaffolding under a decision isn't being made - is there no
/// downstream backlog items? user stories? verification test cases?". For EVERY accepted Decision:
/// the tracked items reaching it by typed edge, each item's evidence state, and the GAPS — accepted
/// Decisions with zero downstream items.
///
/// FIRST-CLASS, not a lens buried in `keel hardening`: that lens answers a narrower question
/// (declared artifact promises from `.engine/contracts/decision-artifacts.toml`); this view answers
/// the model's own question — does the promise-to-work chain exist as EDGES — and the output names
/// the lens as complementary rather than leaving the reader to reconcile two numbers.
///
/// NOT A GATE: a Decision may be legitimately accepted with no work ("we won't do X" needs nothing),
/// which is why `gaps` is a list to read, not a verdict. Whether some subset becomes a guard is
/// dcDecisionScaffoldingGuard's PROPOSED Decision, gated on the human's st014 ("depends on what is
/// being asserted").
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be read.
pub fn decision_follow_through(root: &Path) -> Result<String, ViewError> {
    const INBOUND: [&str; 4] = ["charteredby", "derivedfrom", "resolves", "satisfy"];
    let model = Model::build(root)?;
    let evidence = |item: &str| dft_evidence(&model, item);

    let mut decisions: Vec<(&String, &ItemInfo)> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Decision" && i.attrs.get("status").is_some_and(|s| s.ends_with("accepted")))
        .collect();
    decisions.sort_by(|a, b| a.0.cmp(b.0));

    let mut gaps: Vec<Json> = Vec::new();
    let mut with_downstream = 0usize;
    let mut rows: Vec<Json> = Vec::new();
    for (name, info) in &decisions {
        let mut inbound: Vec<&Edge> = model
            .edges
            .iter()
            .filter(|e| e.to == **name && INBOUND.contains(&e.kind.as_str()))
            .collect();
        inbound.sort_by(|a, b| a.from.cmp(&b.from).then(a.kind.cmp(&b.kind)));
        inbound.dedup_by(|a, b| a.from == b.from && a.kind == b.kind);
        if inbound.is_empty() {
            gaps.push(Json::s((*name).clone()));
            continue;
        }
        with_downstream += 1;
        let items: Vec<Json> = inbound
            .iter()
            .map(|e| {
                Json::Obj(vec![
                    ("item".to_string(), Json::s(e.from.clone())),
                    ("edge".to_string(), Json::s(e.kind.clone())),
                    (
                        "kind".to_string(),
                        Json::s(model.items.get(&e.from).map(|i| i.type_name.clone()).unwrap_or_default()),
                    ),
                    ("evidence".to_string(), Json::s(evidence(&e.from))),
                ])
            })
            .collect();
        rows.push(Json::Obj(vec![
            ("decision".to_string(), Json::s((*name).clone())),
            ("title".to_string(), Json::s(info.attrs.get("title").cloned().unwrap_or_default())),
            ("items".to_string(), Json::Arr(items)),
        ]));
    }
    let out = Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "The promise-to-work chain, computed from typed edges (charteredby/derivedfrom/resolves/satisfy). \
                 `gaps` lists accepted Decisions with ZERO downstream tracked items - some are legitimate \
                 (a Decision that needs no work), which is why this is a view to read, never a verdict. \
                 Complementary: `keel hardening`'s decisionFollowThrough lens checks DECLARED artifact \
                 promises; this view checks the model's edges.",
            ),
        ),
        ("acceptedDecisions".to_string(), Json::Int(i64::try_from(decisions.len()).unwrap_or(i64::MAX))),
        ("withDownstream".to_string(), Json::Int(i64::try_from(with_downstream).unwrap_or(i64::MAX))),
        ("gapCount".to_string(), Json::Int(i64::try_from(gaps.len()).unwrap_or(i64::MAX))),
        ("gaps".to_string(), Json::Arr(gaps)),
        ("decisions".to_string(), Json::Arr(rows)),
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
/// `uncovered` is every sitting with no review, unchanged. `due` is the LIVE obligation: uncovered and
/// created after D0155's grandfather line, drawn at that Decision's introduction commit. The 313
/// sittings uncovered when it landed are accepted-unreviewed by human attestation and reported as
/// `grandfathered_unreviewed` — a waived obligation stays on screen, or the waiver is unfalsifiable.
///
/// If the line can't be resolved, NOTHING is grandfathered and every uncovered sitting is due. That is
/// the opposite of the gate stance (D0050 grandfathers everything on git failure so a gate never
/// spuriously blocks): a gate failing open is cautious, but an obligation surface reporting nothing owed
/// when it doesn't know is the one thing N-C2 forbids.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn sitting_coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut sprints: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Story").map(|(n, _)| n).collect();
    sprints.sort();
    let covered: HashSet<&str> = covered_sprints(&model);
    let gf = crate::govern::grandfathered_under(root, SITTING_DECISION);
    let uncovered_names: Vec<&String> = sprints.iter().filter(|s| !covered.contains(s.as_str())).copied().collect();
    let is_gf = |s: &str| gf.as_ref().is_some_and(|g| g.contains(s));
    let due: Vec<Json> = uncovered_names.iter().filter(|s| !is_gf(s)).map(|s| Json::s((*s).clone())).collect();
    let gf_n = uncovered_names.len() - due.len();
    let basis = match gf.as_ref() {
        None => "GRANDFATHER LINE UNRESOLVED (D0155 not yet committed, or git unavailable) — nothing is grandfathered and every uncovered sitting is reported as due, because a surface that cannot resolve its boundary must overstate the obligation rather than understate it".to_string(),
        Some(_) => format!("due = uncovered AND not present at D0155's introduction commit; {gf_n} sitting(s) accepted-unreviewed by human attestation (D0155), reported and not deleted"),
    };
    let uncovered: Vec<Json> = uncovered_names.iter().map(|s| Json::s((*s).clone())).collect();
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
    let due_n = due.len();
    let out = Json::Obj(vec![
        ("sprints".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
        ("covered".to_string(), Json::Int(i64::try_from(total - uncovered_n).unwrap_or(i64::MAX))),
        ("uncovered".to_string(), Json::Int(i64::try_from(uncovered_n).unwrap_or(i64::MAX))),
        ("due".to_string(), Json::Int(i64::try_from(due_n).unwrap_or(i64::MAX))),
        ("grandfathered_unreviewed".to_string(), Json::Int(i64::try_from(gf_n).unwrap_or(i64::MAX))),
        ("grandfatherBasis".to_string(), Json::s(basis)),
        ("sitting_reviews".to_string(), Json::Arr(reviews)),
        ("due_sprints".to_string(), Json::Arr(due)),
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
        ("displayLabel".to_string(), Json::s(display_label(name, info))),
        ("title".to_string(), info.attrs.get("title").filter(|t| !t.trim().is_empty()).map_or(Json::Null, |t| Json::s(t.clone()))),
        ("type".to_string(), Json::s(info.type_name.clone())),
        ("marker".to_string(), info.marker.clone().map_or(Json::Null, Json::s)),
        ("attrs".to_string(), Json::Arr(attrs)),
        ("dod".to_string(), dod),
        ("outgoing".to_string(), Json::Arr(edges_for(true))),
        ("incoming".to_string(), Json::Arr(edges_for(false))),
    ])
}

/// The BROWSABLE INDEX (D0126 — browse-first discovery).
///
/// Every SUBSTANTIVE item with its computed `displayLabel`, type, authored `createdAt` date, and edge
/// degree — the register the viewer lists so a user finds elements without knowing an identifier.
/// Verification bookkeeping (`Test`/`TestResult`) and structural action shells are EXCLUDED by default
/// (they are reachable via a parent's detail) so the register is about the architecture, not the ~5000
/// test records. Filtering (type/text/date) is done by the consumer over this list.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn index_json(root: &Path) -> Result<String, ViewError> {
    const EXCLUDE: [&str; 5] = ["Test", "TestResult", "action", "ActionDef", ""];
    let model = Model::build(root)?;
    // edge degree per item (in + out) — a cheap "how connected" signal for the register.
    let mut degree: HashMap<&str, usize> = HashMap::new();
    for e in &model.edges {
        *degree.entry(e.from.as_str()).or_insert(0) += 1;
        *degree.entry(e.to.as_str()).or_insert(0) += 1;
    }
    let mut rows: Vec<(&String, &ItemInfo)> = model
        .items
        .iter()
        .filter(|(_, info)| !EXCLUDE.contains(&info.type_name.as_str()))
        .collect();
    rows.sort_by(|a, b| a.1.type_name.cmp(&b.1.type_name).then_with(|| display_label(a.0, a.1).cmp(&display_label(b.0, b.1))));
    let items: Vec<Json> = rows
        .iter()
        .map(|(n, info)| {
            let date = info.attrs.get("createdAt").or_else(|| info.attrs.get("judgedAt")).cloned().unwrap_or_default();
            Json::Obj(vec![
                ("name".to_string(), Json::s((*n).clone())),
                ("displayLabel".to_string(), Json::s(display_label(n, info))),
                ("title".to_string(), info.attrs.get("title").filter(|t| !t.trim().is_empty()).map_or(Json::Null, |t| Json::s(t.clone()))),
                ("type".to_string(), Json::s(info.type_name.clone())),
                ("date".to_string(), Json::s(date)),
                ("edges".to_string(), Json::Int(i64::try_from(*degree.get(n.as_str()).unwrap_or(&0)).unwrap_or(0))),
            ])
        })
        .collect();
    Ok(Json::Obj(vec![
        ("index".to_string(), Json::s("browsable register of substantive items (D0126); Test/TestResult + action shells excluded".to_string())),
        ("count".to_string(), Json::Int(i64::try_from(items.len()).unwrap_or(0))),
        ("items".to_string(), Json::Arr(items)),
    ])
    .dump())
}

/// The RELATIONSHIP GRAMMAR (D0126/D0127) — computed, not authored.
///
/// The distinct `(sourceType, edgeKind, targetType)` triples that ACTUALLY occur in the model, plus a
/// per-type up/down summary. This is the observed grammar: what connects to what, derived from real
/// structure (a view, §2.1) — so a client offers only valid, in-scope connections/creations generically
/// for ANY project's schema, with zero authored metamodel. `byType[T].down` = kinds+types T points at;
/// `byType[T].up` = kinds+types that point at T.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn grammar_json(root: &Path) -> Result<String, ViewError> {
    use std::collections::BTreeSet;
    let model = Model::build(root)?;
    let ty = |n: &str| model.items.get(n).map_or("", |i| i.type_name.as_str());
    let mut triples: BTreeSet<(String, String, String)> = BTreeSet::new();
    for e in &model.edges {
        let (f, t) = (ty(&e.from), ty(&e.to));
        if f.is_empty() || t.is_empty() { continue; }
        triples.insert((f.to_string(), e.kind.clone(), t.to_string()));
    }
    let triples_json: Vec<Json> = triples.iter().map(|(f, k, t)| Json::Obj(vec![
        ("from".to_string(), Json::s(f.clone())),
        ("edge".to_string(), Json::s(k.clone())),
        ("to".to_string(), Json::s(t.clone())),
    ])).collect();
    // per-type up/down
    let mut types: BTreeSet<&str> = BTreeSet::new();
    for (f, _, t) in &triples { types.insert(f); types.insert(t); }
    let by_type: Vec<Json> = types.iter().map(|ty_name| {
        let down: Vec<Json> = triples.iter().filter(|(f, _, _)| f == ty_name)
            .map(|(_, k, t)| Json::Obj(vec![("edge".to_string(), Json::s(k.clone())), ("type".to_string(), Json::s(t.clone()))])).collect();
        let up: Vec<Json> = triples.iter().filter(|(_, _, t)| t == ty_name)
            .map(|(f, k, _)| Json::Obj(vec![("edge".to_string(), Json::s(k.clone())), ("type".to_string(), Json::s(f.clone()))])).collect();
        Json::Obj(vec![
            ("type".to_string(), Json::s((*ty_name).to_string())),
            ("down".to_string(), Json::Arr(down)),
            ("up".to_string(), Json::Arr(up)),
        ])
    }).collect();
    Ok(Json::Obj(vec![
        ("grammar".to_string(), Json::s("observed (sourceType, edge, targetType) triples — the computed relationship grammar (D0126/D0127)".to_string())),
        ("triples".to_string(), Json::Arr(triples_json)),
        ("byType".to_string(), Json::Arr(by_type)),
    ]).dump())
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

pub(crate) struct Coverage {
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
        return "need";
    }
    // ISSUE176: `decision` used to be ONE bucket covering two different things. D0064 permits
    // decision-driven engine evolution, and that permission is what made the number unfalsifiable -
    // every charter was defensible by construction, so the metric could not fail and therefore could
    // not inform. Split by whether the CHARTERING DECISION is itself grounded in something a human
    // said or wanted: a Decision citing a Statement (D0166) or a Need is `decision_rooted`; a Decision
    // resting only on my own judgment is `decision_ungrounded`. Neither is a violation - the point is
    // that they are now COUNTABLE apart.
    if charters.iter().any(|d| rd_decision_grounded(model, d)) {
        "decision_rooted"
    } else {
        "decision_ungrounded"
    }
}

/// Does this chartering Decision itself rest on something a human asked for?
///
/// Grounded means: an inbound `derivedfrom`/`implicates` edge from a `Statement` or `UserStory` (the
/// intake chain, D0166), or an edge of its own reaching a `Need`. Anything else is my own judgment,
/// which is legitimate under D0064 and still worth counting separately.
fn rd_decision_grounded(model: &Model, decision: &str) -> bool {
    let touches = |name: &str| -> bool {
        matches!(model.items.get(name).map_or("", |i| i.type_name.as_str()), "Statement" | "UserStory" | "Need")
    };
    model
        .edges
        .iter()
        .any(|e| (e.to == decision && touches(&e.from)) || (e.from == decision && touches(&e.to)))
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
    /// Items excluded from `total` and `gaps` because a Decision DESCOPED them (issue088).
    ///
    /// Reported rather than dropped: §2.4 makes a superseding Decision the engine's own scope
    /// mechanism, so descoped work is a legitimate outcome — but an excluded item that becomes an
    /// INVISIBLE item is how a metric starts quietly flattering itself. The count keeps the descoping
    /// on screen next to the number it improves.
    superseded: Vec<String>,
}


/// `keel controls` (D0195, panel R1 aerospace flip 2) — the two-way hazard/control diff.
///
/// Computes over `.tracking/architecture/engine-safety.sysml` (Hazard instances) and
/// `control-map.sysml` (`SystemSafetyConstraint` instances with plain dependency edges to the
/// hazards they discharge): hazards NO constraint reaches (the uncovered failure conditions — the
/// question ARP4754A asks that could not previously be asked of this model), and constraints
/// anchored to NO hazard (process-quality controls — reported, never warned away). Clause 7 rides
/// along: a High/Critical Issue created after D0195's acceptance that references no hazard is the
/// loss/hazard list going stale by the very EHZ8 mechanism it documents, and is listed.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn controls(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let hazards: Vec<&String> = {
        let mut v: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Hazard").map(|(n, _)| n).collect();
        v.sort();
        v
    };
    let constraints: Vec<&String> = {
        let mut v: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "SystemSafetyConstraint").map(|(n, _)| n).collect();
        v.sort();
        v
    };
    let reaches = |c: &str, h: &str| model.edges.iter().any(|e| e.kind == "dependency" && e.from == c && e.to == h);
    let mut rows: Vec<Json> = Vec::new();
    let mut uncovered: Vec<String> = Vec::new();
    for h in &hazards {
        let cs: Vec<Json> = constraints.iter().filter(|c| reaches(c, h)).map(|c| Json::s((*c).clone())).collect();
        if cs.is_empty() {
            uncovered.push((*h).clone());
        }
        rows.push(Json::Obj(vec![
            ("hazard".to_string(), Json::s((*h).clone())),
            ("controls".to_string(), Json::Arr(cs)),
        ]));
    }
    let unanchored: Vec<Json> = constraints
        .iter()
        .filter(|c| !hazards.iter().any(|h| reaches(c, h)))
        .map(|c| Json::s((*c).clone()))
        .collect();
    // Clause 7 (panel R2, automotive finding 1): forward-only from D0195's acceptance date.
    let mut unlinked_incidents: Vec<Json> = Vec::new();
    for (n, i) in &model.items {
        if i.type_name != "Issue" {
            continue;
        }
        let sev = i.attrs.get("severity").cloned().unwrap_or_default();
        if sev != "High" && sev != "Critical" {
            continue;
        }
        let created = i.attrs.get("createdAt").cloned().unwrap_or_default();
        if created.as_str() < "2026-08-22" {
            continue;
        }
        let text = i.attrs.get("description").cloned().unwrap_or_default();
        let linked = text.to_ascii_lowercase().contains("ehz") || model.edges.iter().any(|e| e.kind == "dependency" && e.from == *n && e.to.starts_with("ehz"));
        if !linked {
            unlinked_incidents.push(Json::s(n.clone()));
        }
    }
    unlinked_incidents.sort_by_key(Json::dump);
    Ok(Json::Obj(vec![
        ("controls".to_string(), Json::s("the two-way hazard/control diff (D0195): every failure condition's standing controls as edges, computable - and the two honest gap classes on either side".to_string())),
        ("hazards".to_string(), Json::Arr(rows)),
        ("uncoveredHazards".to_string(), Json::Arr(uncovered.into_iter().map(Json::s).collect())),
        ("unanchoredConstraints".to_string(), Json::Arr(unanchored)),
        ("unlinkedHighIncidentsSinceD0195".to_string(), Json::Arr(unlinked_incidents)),
    ])
    .dump())
}

fn compute_tier_satisfaction(model: &Model) -> Vec<TierStat> {
    let has_out = |kind: &str, from: &str| model.edges.iter().any(|e| e.kind == kind && e.from == from);
    let has_in = |kind: &str, to: &str| model.edges.iter().any(|e| e.kind == kind && e.to == to);
    // A `#Supersede` edge INTO an item is the authored statement that it was deliberately cut (§2.4).
    // Counting a descoped Need as an undecomposed gap understates completeness AND — the worse half —
    // points a future contributor at authoring SystemRequirements for work that was explicitly
    // dropped. The metric was actively recommending wrong work (issue088).
    let superseded_item = |n: &str| model.edges.iter().any(|e| e.kind == "supersede" && e.to == n);
    let tier = |ty: &str, relation: &'static str, pred: &dyn Fn(&str) -> bool, label: &'static str| -> TierStat {
        let mut names: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == ty).map(|(n, _)| n).collect();
        names.sort();
        let mut gaps: Vec<String> = Vec::new();
        let mut superseded: Vec<String> = Vec::new();
        let mut satisfied = 0;
        let mut total = 0;
        for n in &names {
            if superseded_item(n) {
                superseded.push((*n).clone());
                continue;
            }
            total += 1;
            if pred(n) {
                satisfied += 1;
            } else {
                gaps.push((*n).clone());
            }
        }
        TierStat { tier: label, relation, total, satisfied, gaps, superseded }
    };
    vec![
        // A Need is decomposed iff some SystemRequirement satisfies it (satisfy edge Need->SR).
        tier("Need", "satisfied-by SystemRequirement", &|n| has_out("satisfy", n), "Need"),
        // A SystemRequirement is verified iff a Test #Verify-links to it (verify edge Test->SR).
        tier("SystemRequirement", "verified-by Test", &|sr| has_in("verify", sr), "SystemRequirement"),
        // D0194 (panel R1, all five panelists): an SR is ALLOCATED iff an allocate edge leaves it
        // for a target that is not itself superseded - the coverage leg whose absence let 100%
        // silently decay to ~25% over two months with nothing re-firing the obligation. Reads LOW
        // and honest at introduction; the floor is the point, not the flattery (D0098).
        tier(
            "SystemRequirement",
            "allocated-to live Component/CodeElement",
            &|sr| model.edges.iter().any(|e| e.kind == "allocate" && e.from == *sr && !superseded_item(&e.to)),
            "SystemRequirement (allocation)",
        ),
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
    let model = Model::build(root)?;
    let stats = compute_tier_satisfaction(&model);
    let mix = verified_method_mix(&model);
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
                // Descoped items, kept ON SCREEN beside the number they improve (issue088): an
                // excluded item that becomes invisible is how a metric starts flattering itself.
                ("supersededExcluded".to_string(), n(t.superseded.len())),
                ("superseded".to_string(), Json::Arr(t.superseded.iter().map(|s| Json::s(s.clone())).collect())),
            ])
        })
        .collect();
    let out = Json::Obj(vec![
        ("tier_satisfaction".to_string(), Json::s("tier-satisfaction comprehensiveness (D0098/issue047): STRUCTURAL downstream-satisfaction floor per tier — Needs decomposed into SystemRequirements (satisfy), SystemRequirements verified by Tests (verify). A leading indicator of insufficient implementation; thin downstream = predicted under-implementation. (Deeper 'do the SRs fully discharge the Need' is the AI white-box layer, not yet built.)")),
        ("tiers".to_string(), Json::Arr(tiers)),
        // issue082/D0130: 'verified' means SOME Test #Verify-links the SR — it does NOT mean a passing
        // functional test. Report the METHOD mix so the percentage is self-describing and cannot be
        // read as functional verification when it is predominantly critique coverage.
        ("verifiedByMethod".to_string(), Json::Obj(mix.iter().map(|(m, c)| (m.clone(), n(*c))).collect())),
        ("verifiedByMethodNote".to_string(), Json::s("Counts verify edges into SystemRequirements by the verifying Test's method. sr_verified_pct counts ANY #Verify-linked Test — read this mix before treating it as functional-test coverage.")),
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

/// `true` if a severity string is >= High — the tier a priority inversion is reported against.
#[allow(clippy::missing_const_for_fn)] // cannot match on `str` in a const fn
fn at_least_high(sev: &str) -> bool {
    matches!(sev, "Critical" | "High")
}

/// Ready items outranking a >= High-severity item while carrying lower or no severity.
///
/// Pure core of [`priority_inversions`]. `ready` is in PRIORITY ORDER (declaration order, D0052),
/// so "outranks" is simply "appears earlier". Returns `(outranking item, high item, its severity)`.
fn inversion_pairs(ready: &[(String, Option<String>)]) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for (i, (high, sev)) in ready.iter().enumerate() {
        let Some(s) = sev.as_deref().filter(|s| at_least_high(s)) else { continue };
        for (lower, lsev) in ready.iter().take(i) {
            if lsev.as_deref().is_none_or(|l| !at_least_high(l)) {
                out.push((lower.clone(), high.clone(), s.to_string()));
            }
        }
    }
    out
}

/// Backlog priority inversions: a ready item ranked ABOVE work that resolves a >= High Issue.
///
/// Closes issue084 (D0130). D0052 makes backlog DECLARATION ORDER the priority and requires the AI to
/// auto-follow the ranked frontier — but nothing computed whether recorded ORDER agreed with recorded
/// SEVERITY, so a mis-ordered backlog was indistinguishable from a curated one. It was mis-ordered:
/// `keelArchViews` (issue069, Low) ranked FIRST purely because an earlier session appended it to the
/// end of a COMPLETED block, while `dcStaleKernelInstanceGate` (issue081, High — an enforced commit
/// gate being routinely bypassed) ranked 14th, and the AI then narrated priority in prose instead of
/// reordering the file. Both inputs are recorded facts, so the inversion is COMPUTABLE.
///
/// Reported, never enforced: priority is a human judgment and ordering may be deliberate (a High item
/// can be legitimately deferred behind an enabler). The value is that the trade-off becomes VISIBLE
/// instead of resting on whoever last appended to the file.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn priority_inversions(root: &Path) -> Result<Vec<(String, String, String)>, ViewError> {
    let model = Model::build(root)?;
    let ready = crate::orient::compute(root).ready; // already in declaration/priority order
    let severity_of = |task: &str| -> Option<String> {
        model
            .edges
            .iter()
            .filter(|e| e.kind == "resolves" && e.from == task)
            .filter_map(|e| model.items.get(&e.to))
            .filter_map(|i| i.attrs.get("severity").cloned())
            .max_by_key(|s| match s.as_str() {
                "Critical" => 4,
                "High" => 3,
                "Medium" => 2,
                "Low" => 1,
                _ => 0,
            })
    };
    let pairs: Vec<(String, Option<String>)> = ready.iter().map(|t| (t.clone(), severity_of(t))).collect();
    Ok(inversion_pairs(&pairs))
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
    // PHASE-TIMED (dcSharedParsedModel): `assured` is 7-9s of which fingerprint, parse and git are
    // ~1.4s, so the remaining 6s is in-view computation and a total cannot say which step. Each step
    // is now named, so the next optimisation is aimed rather than guessed.
    let done = crate::perf::phase("doneNames", || crate::orient::done_names(root));
    let suspect_vec = crate::perf::phase("orientCompute", || crate::orient::compute(root).suspect);
    let task_suspect: HashSet<String> = suspect_vec.iter().cloned().collect();
    let stale = compute_stale_verifications(root, &model);

    // Charter-time scoping (D0081): only GOVERNED elements (created after the governing decision)
    // count as gaps — grandfathered elements are out of the gate.
    let gf_cov = crate::perf::phase("grandfatherCoverage", || crate::govern::grandfathered_under(root, COVERAGE_DECISION));
    let gf_crit = crate::perf::phase("grandfatherCritique", || crate::govern::grandfathered_under(root, CRITIQUE_DECISION));
    let coverage_gaps: Vec<String> = crate::perf::phase("computeCoverage", || compute_coverage(&model, &done, &task_suspect, &stale))
        .into_iter()
        .filter(|c| !is_covered_tier(c.tier) && governed(gf_cov.as_ref(), &c.element))
        .map(|c| c.element)
        .collect();
    let policy = CritiquePolicy::load(root)?;
    let critique_gaps: Vec<String> = crate::perf::phase("computeCritiqueCoverage", || compute_critique_coverage(&model, &stale, &policy))
        .into_iter()
        .filter(|c| !c.covered && governed(gf_crit.as_ref(), &c.element))
        .map(|c| c.element)
        .collect();

    let (undispositioned_findings, unfixed_critical) = finding_blockers(&compute_issue_resolution(&model, &done), &model);

    // Base invariant guards only — EXCLUDE `assured` (would recurse) and `critique` (composed
    // separately as critique_gaps). This is what "invariants green" means for readiness.
    // THE WHOLE GUARD SUITE, INSIDE A VIEW. Legitimate - readiness means invariants hold - but it makes
    // `keel assured` cost `keel guard` PLUS every composed view, which is the single largest term and was
    // invisible until it was named.
    let invariant_violations: Vec<String> = crate::perf::phase("allGuards", || {
        crate::guards::GUARD_NAMES
            .iter()
            .copied()
            .filter(|n| !matches!(*n, "assured" | "critique"))
            .filter_map(|n| crate::guards::run_one(n, root))
            .flat_map(|r| r.violations.into_iter().map(move |v| format!("{}: {v}", r.name)))
            .collect()
    });

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

// ── contention (D0129 srDcContentionAdjudication) ────────────────────────────

/// Contentions: places where two contributors have reached conclusions that cannot both stand, and
/// which D0108 clause 5 says a HUMAN adjudicates — never the contributor who holds one of them.
///
/// # What is computed, and what deliberately is not
///
/// The item names three dimensions. Two are exactly computable and are computed:
///
/// * **contradictory judgments** — one verification carrying passing AND failing results from
///   DIFFERENT actors. Same actor, different outcomes over time is a re-judgement, not a contention,
///   and conflating them would flood this view with ordinary re-verification.
/// * **rival proposals** — two still-`proposed` Decisions resolving the SAME Issue. Two open answers
///   to one question is a contention even when neither author knows about the other, which is the
///   normal case in an asynchronous team.
///
/// The third — two live CLAIMS on one item — cannot be computed: there is no `Claim` type, and
/// D0147 proposing one is itself awaiting human sign-off. It is REPORTED AS NOT COMPUTED with the
/// reason rather than omitted, because a view listing two of three dimensions and saying nothing
/// about the third reads as "no claim contentions exist" (the D0138 lesson, and the reason `orient`
/// emits an empty `pendingAcceptances` rather than dropping the key).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn contentions(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut rows: Vec<String> = Vec::new();

    // (1) Contradictory judgments on one verification, by different actors.
    let mut by_verification: BTreeMap<String, Vec<(&String, &str, &str)>> = BTreeMap::new();
    for e in model.edges.iter().filter(|e| e.kind == "resultof") {
        if let Some(r) = model.items.get(&e.from) {
            let outcome = r.attrs.get("outcome").map_or("", String::as_str);
            let by = r.attrs.get("judgedBy").map_or("", String::as_str);
            by_verification.entry(e.to.clone()).or_default().push((&e.from, outcome, by));
        }
    }
    // Results are also named `<verification>R<n>`, which is how most of this repo links them.
    for (name, info) in &model.items {
        if info.type_name != "TestResult" {
            continue;
        }
        if let Some(v) = name.rfind('R').and_then(|i| name.get(..i)).filter(|v| model.items.contains_key(*v)) {
            let outcome = info.attrs.get("outcome").map_or("", String::as_str);
            let by = info.attrs.get("judgedBy").map_or("", String::as_str);
            let entry = by_verification.entry(v.to_owned()).or_default();
            if !entry.iter().any(|(n, _, _)| *n == name) {
                entry.push((name, outcome, by));
            }
        }
    }
    for (verification, results) in &by_verification {
        let passes: Vec<&str> = results.iter().filter(|(_, o, _)| o.ends_with("pass")).map(|(_, _, b)| *b).collect();
        let fails: Vec<&str> = results.iter().filter(|(_, o, _)| o.ends_with("fail")).map(|(_, _, b)| *b).collect();
        if passes.is_empty() || fails.is_empty() {
            continue;
        }
        // Different ACTORS, or it is one actor re-judging their own work over time.
        let contended = passes.iter().any(|p| fails.iter().any(|f| p != f && !p.is_empty() && !f.is_empty()));
        if contended {
            rows.push(format!(
                "{{\"kind\":\"contradictoryJudgment\",\"subject\":\"{verification}\",\"passedBy\":[{}],\"failedBy\":[{}]}}",
                passes.iter().map(|a| format!("\"{a}\"")).collect::<Vec<_>>().join(","),
                fails.iter().map(|a| format!("\"{a}\"")).collect::<Vec<_>>().join(",")
            ));
        }
    }

    // (2) Two still-proposed Decisions resolving the same Issue.
    let proposed: HashSet<&String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Decision")
        .filter(|(_, i)| i.attrs.get("status").is_some_and(|s| s.ends_with("::proposed") || s == "proposed"))
        .map(|(n, _)| n)
        .collect();
    let mut per_issue: BTreeMap<&String, Vec<&String>> = BTreeMap::new();
    for e in model.edges.iter().filter(|e| e.kind == "resolves") {
        if proposed.contains(&e.from) {
            per_issue.entry(&e.to).or_default().push(&e.from);
        }
    }
    for (issue, decisions) in &per_issue {
        if decisions.len() > 1 {
            rows.push(format!(
                "{{\"kind\":\"rivalProposals\",\"subject\":\"{issue}\",\"decisions\":[{}]}}",
                decisions.iter().map(|d| format!("\"{d}\"")).collect::<Vec<_>>().join(",")
            ));
        }
    }

    Ok(format!(
        "{{\n  \"contentions\": [{}],\n  \"count\": {},\n  \"notComputed\": [{{\"dimension\":\"liveClaimCollision\",\"reason\":\"no Claim type exists; D0147 proposes one and is awaiting human sign-off. Reported rather than omitted so this view is not read as 'no claim contentions exist'.\"}}],\n  \"note\": \"D0108 clause 5: a contention is adjudicated by a HUMAN, recorded as a Decision. No contributor may resolve one in favour of its own conclusion.\"\n}}",
        rows.join(", "),
        rows.len()
    ))
}

// ── human-authority queue (D0129 srDcHumanAuthorityQueue) ────────────────────

/// The date each human obligation came into force — the Decision that created it.
///
/// ISSUE068'S LESSON, APPLIED AS A FILTER. A new obligation must never retro-fail work that was
/// correct when written, and this queue is where that rule bites hardest: 287 sprints predate the
/// per-sitting review requirement and 54 findings predate the disposition lifecycle. Listing them
/// all as "awaiting human authority" would hand a human ~340 items nobody ever owed — which is not
/// thoroughness but the rubber-stamping this item exists to prevent, since a queue nobody can work
/// is a queue nobody reads.
const OBLIGATION_FROM: &[(&str, &str, &str)] = &[
    ("perSittingReview", "2026-06-18", "D0073"),
    ("findingDisposition", "2026-06-22", "D0092"),
];

fn in_force_from(kind: &str) -> (&'static str, &'static str) {
    OBLIGATION_FROM
        .iter()
        .find(|(k, _, _)| *k == kind)
        .map_or(("0000-00-00", "-"), |(_, d, dec)| (*d, *dec))
}

/// Whole days between two ISO dates, or 0 if either is unparseable.
///
/// Dates only: the model records dates, and reporting a finer resolution than the data carries would
/// be false precision. Uses the standard civil-date algorithm rather than a dependency.
fn days_between(from: &str, to: &str) -> i64 {
    let parse = |s: &str| -> Option<(i64, i64, i64)> {
        let mut it = s.split('-');
        Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?, it.next()?.parse().ok()?))
    };
    let to_days = |(y, m, d): (i64, i64, i64)| -> i64 {
        let y = if m <= 2 { y - 1 } else { y };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let mp = (m + 9) % 12;
        let doy = (153 * mp + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    };
    match (parse(from), parse(to)) {
        (Some(a), Some(b)) => to_days(b) - to_days(a),
        _ => 0,
    }
}

/// "Now", taken from git rather than the wall clock (D0013): the HEAD commit date.
///
/// Deterministic — two contributors computing this queue against the same commit get the same ages,
/// which a clock would not give them.
fn repo_today(root: &Path) -> String {
    crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["log", "-1", "--format=%cs"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .filter(|s| s.len() == 10)
        .unwrap_or_default()
}

/// Escalation threshold in days. It MARKS rather than blocks: human judgment is the one resource in
/// a mostly-AI team that cannot be scaled by adding contributors, so it must never sit on the
/// critical path of a unit of work — a contributor stalling on the queue would make the bottleneck
/// worse, not visible.
const ESCALATE_AFTER_DAYS: i64 = 14;

/// One row of the queue.
struct Awaiting {
    kind: String,
    item: String,
    origin: String,
    since: String,
    note: String,
}

/// The two per-item obligation classes, split out to keep `authority_queue` readable.
fn collect_decision_and_finding_obligations(model: &Model, awaiting: &mut Vec<Awaiting>) {
    // (1) Decisions awaiting acceptance. NOT grandfathered: a proposed Decision is a live request
    // whenever it was raised, and no obligation post-dates it — it is the obligation.
    let mut pending: Vec<&String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Decision")
        .filter(|(_, i)| i.attrs.get("status").is_some_and(|s| s.ends_with("::proposed") || s == "proposed"))
        .map(|(n, _)| n)
        .collect();
    pending.sort();
    for d in pending {
        let info = model.items.get(d);
        awaiting.push(Awaiting {
            kind: "decisionAcceptance".to_owned(),
            item: d.clone(),
            origin: info.and_then(|i| i.attrs.get("createdBy")).cloned().unwrap_or_default(),
            since: info.and_then(|i| i.attrs.get("createdAt")).cloned().unwrap_or_default(),
            note: "an AI actor cannot supply this (D0106)".to_owned(),
        });
    }

    // (2) Findings at or above the disposition threshold, undispositioned and in force (D0092).
    let (disp_from, disp_dec) = in_force_from("findingDisposition");
    let mut findings: Vec<&String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Issue")
        .filter(|(_, i)| {
            i.attrs.get("severity").is_some_and(|s| s.ends_with("Critical") || s.ends_with("High") || s.ends_with("Medium"))
        })
        .map(|(n, _)| n)
        .collect();
    findings.sort();
    for f in findings {
        if issue_disposition(model, f).is_some() {
            continue;
        }
        let info = model.items.get(f);
        let since = info.and_then(|i| i.attrs.get("createdAt")).cloned().unwrap_or_default();
        if since.is_empty() || days_between(disp_from, &since) < 0 {
            continue; // predates the obligation — issue068: never retro-fail correct-when-written work
        }
        awaiting.push(Awaiting {
            kind: "findingDisposition".to_owned(),
            item: f.clone(),
            origin: info.and_then(|i| i.attrs.get("createdBy")).cloned().unwrap_or_default(),
            since,
            note: format!("in force from {disp_from} ({disp_dec})"),
        });
    }

}

/// Everything genuinely awaiting HUMAN authority, with waiting age and originating contributor.
///
/// # What is deliberately excluded
///
/// Anything an automated check already settles. D0051 is explicit — confirm only what tests cannot —
/// and asking a human to re-affirm a passing test degrades review into rubber-stamping, which
/// launders unreviewed work as reviewed. So a gate that passed by `method=test` never appears here;
/// only obligations REQUIRING a human verdict do: accepting a Decision, dispositioning a finding at
/// or above the threshold, adjudicating a contention, and the per-sitting review.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn authority_queue(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let today = repo_today(root);
    let mut awaiting: Vec<Awaiting> = Vec::new();
    collect_decision_and_finding_obligations(&model, &mut awaiting);

    // (3) Contentions — D0108 clause 5: a human adjudicates, never a contributor holding one side.
    let contention_rows = contentions(root)?.matches("\"kind\"").count();
    if contention_rows > 0 {
        awaiting.push(Awaiting {
            kind: "contentionAdjudication".to_owned(),
            item: "see `keel contentions`".to_owned(),
            origin: "multiple".to_owned(),
            since: String::new(),
            note: format!("{contention_rows} contention(s); no contributor may resolve one in favour of its own conclusion (D0108)"),
        });
    }

    // (4) Per-sitting reviews still owed, in force from D0073. Reported as ONE batched row, because
    // D0049 makes the review per SITTING rather than per sprint — a row per sprint would misstate
    // the ask by two orders of magnitude and invite exactly the rubber-stamping being avoided.
    //
    // issue227 (the process-value panel's High finding): this row previously grandfathered only on
    // the D0073 in-force DATE while `sitting-coverage` grandfathers on the D0155 human attestation
    // (present-at-introduction-commit) — two views, one obligation, two answers (279 vs 6). ONE
    // computation now: the same D0155 basis, with the grandfathered count reported beside the due
    // count rather than hidden, and the escalation clock starting from the oldest DUE item.
    let (rev_from, rev_dec) = in_force_from("perSittingReview");
    let covered = covered_sprints(&model);
    let gf = crate::govern::grandfathered_under(root, SITTING_DECISION);
    let is_gf = |s: &str| gf.as_ref().is_some_and(|g| g.contains(s));
    let mut sprints: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Story").map(|(n, _)| n).collect();
    sprints.sort();
    let mut owed = 0usize;
    let mut grandfathered = 0usize;
    let mut oldest = String::new();
    for s in sprints {
        if covered.contains(s.as_str()) {
            continue;
        }
        let since = model.items.get(s).and_then(|i| i.attrs.get("createdAt")).cloned().unwrap_or_default();
        if since.is_empty() || days_between(rev_from, &since) < 0 {
            continue; // predates the per-sitting review obligation itself (D0073)
        }
        if is_gf(s) {
            grandfathered += 1; // accepted-unreviewed by human attestation (D0155) — counted, never due
            continue;
        }
        owed += 1;
        if oldest.is_empty() || since < oldest {
            oldest.clone_from(&since);
        }
    }
    if owed > 0 {
        awaiting.push(Awaiting {
            kind: "perSittingReview".to_owned(),
            item: format!("{owed} sprint(s) awaiting a sitting review"),
            origin: "multiple".to_owned(),
            since: oldest,
            note: format!(
                "BATCHED per sitting (D0049), not per sprint — in force from {rev_from} ({rev_dec}); due = uncovered AND not D0155-grandfathered, matching `keel sitting-coverage` (issue227); {grandfathered} grandfathered sitting(s) excluded and reported by that view"
            ),
        });
    }

    let mut escalated = 0usize;
    let rows: Vec<Json> = awaiting
        .iter()
        .map(|a| {
            let age = if today.is_empty() || a.since.is_empty() { -1 } else { days_between(&a.since, &today) };
            let esc = age >= ESCALATE_AFTER_DAYS;
            if esc {
                escalated += 1;
            }
            Json::Obj(vec![
                ("kind".to_owned(), Json::s(a.kind.clone())),
                ("item".to_owned(), Json::s(a.item.clone())),
                ("origin".to_owned(), Json::s(a.origin.clone())),
                ("waitingSince".to_owned(), Json::s(a.since.clone())),
                ("waitingDays".to_owned(), Json::Int(age)),
                ("escalated".to_owned(), Json::Bool(esc)),
                ("note".to_owned(), Json::s(a.note.clone())),
            ])
        })
        .collect();
    let count = rows.len();
    Ok(Json::Obj(vec![
        ("asOf".to_owned(), Json::s(today)),
        ("asOfSource".to_owned(), Json::s("HEAD commit date (D0013 — git-derived, so two contributors computing this against the same commit agree)".to_owned())),
        ("escalateAfterDays".to_owned(), Json::Int(ESCALATE_AFTER_DAYS)),
        ("awaiting".to_owned(), Json::Arr(rows)),
        ("count".to_owned(), Json::Int(i64::try_from(count).unwrap_or(i64::MAX))),
        ("escalated".to_owned(), Json::Int(i64::try_from(escalated).unwrap_or(i64::MAX))),
        ("excluded".to_owned(), Json::s("anything a passing automated check already settles (D0051: confirm only what tests cannot), and obligations that post-date the work (issue068 grandfathering)".to_owned())),
    ])
    .dump())
}

#[cfg(test)]
mod authority_queue_tests {
    use super::{days_between, in_force_from};

    #[test]
    fn day_arithmetic_is_exact_across_months_and_leap_years() {
        assert_eq!(days_between("2026-08-15", "2026-08-15"), 0);
        assert_eq!(days_between("2026-06-18", "2026-08-15"), 58);
        assert_eq!(days_between("2026-02-28", "2026-03-01"), 1, "2026 is not a leap year");
        assert_eq!(days_between("2024-02-28", "2024-03-01"), 2, "2024 is");
        // NEGATIVE means the item predates the obligation — that is the grandfathering test, so the
        // sign has to be right or issue068's rule inverts and every historical item is resurrected.
        assert!(days_between("2026-06-22", "2026-06-01") < 0);
        assert!(days_between("2026-06-22", "2026-07-01") > 0);
        // Unparseable dates yield 0 rather than a panic or a wild number: an item with a malformed
        // date must not silently escalate.
        assert_eq!(days_between("", "2026-08-15"), 0);
        assert_eq!(days_between("not-a-date", "2026-08-15"), 0);
    }

    #[test]
    fn every_obligation_names_the_decision_that_created_it() {
        for kind in ["perSittingReview", "findingDisposition"] {
            let (from, dec) = in_force_from(kind);
            assert_eq!(from.len(), 10, "{kind} must carry a real in-force date");
            assert!(dec.starts_with('D'), "{kind} must name its Decision, so the filter can be audited");
        }
        // An unknown obligation grandfathers NOTHING rather than everything: the fallback date is
        // before any possible item, so a typo cannot silently empty the queue.
        assert_eq!(in_force_from("nonexistent").0, "0000-00-00");
    }
}

/// Render an attribute value as the string a comparison should use.
///
/// Public because the ownership guard compares two ASTs of the same file and needs the identical
/// rendering on both sides — two renderings that could drift would make an unchanged field look
/// edited, which is the one false positive an ownership check cannot afford.
#[must_use]
pub fn attr_value_string(v: &Value) -> String {
    value_to_string(v)
}

/// `(disposition, issue, judgedBy)` for one offending disposition.
/// `(disposition, issue, gap)` — the gap is WHY the judge fails the declared policy, not merely who.
pub type AiJudgedDisposition = (String, String, String);

/// Dispositions of findings at or above Medium whose result is NOT judged by a registered `Person`.
///
/// Returns `(scanned, [(disposition, issue, judgedBy)])`.
///
/// D0080 permits an AI to disposition a LOW finding and this repo contains a correct example, so the
/// severity filter is not a nicety — without it the guard would fail documented, legitimate work.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn ai_judged_high_dispositions(root: &Path) -> Result<(usize, Vec<AiJudgedDisposition>), ViewError> {
    let model = Model::build(root)?;
    let policy = crate::activation::attestation_policy(root, "findingDisposition");
    let mut scanned = 0usize;
    let mut bad = Vec::new();
    let mut edges: Vec<&Edge> = model.edges.iter().filter(|e| e.kind == "dispositions").collect();
    edges.sort_by(|a, b| a.from.cmp(&b.from));
    for e in edges {
        let above_threshold = model
            .items
            .get(&e.to)
            .and_then(|i| i.attrs.get("severity"))
            .is_some_and(|s| s.ends_with("Critical") || s.ends_with("High") || s.ends_with("Medium"));
        if !above_threshold {
            continue;
        }
        scanned += 1;
        let result = format!("{}R1", e.from);
        let judged_by = model.items.get(&result).and_then(|i| i.attrs.get("judgedBy")).cloned().unwrap_or_default();
        // Check the actor against the DECLARED policy rather than a hardcoded "is a Person"
        // (D0146/srDcAuthorityFromRegistry: kind AND role, against the policy for this class).
        // A `Person`'s schema pins `kind = ActorKind::human`, so an actor typed Person satisfies the
        // kind test even without the attribute written out.
        let actor = model.items.get(&judged_by);
        let kind = actor.map(|a| {
            a.attrs.get("kind").cloned().unwrap_or_else(|| {
                if a.type_name == "Person" { "human".to_owned() } else { String::new() }
            })
        });
        let role = actor.and_then(|a| a.attrs.get("role")).cloned();
        if let Some(gap) = crate::activation::authority_gap(kind.as_deref(), role.as_deref(), &policy) {
            bad.push((e.from.clone(), e.to.clone(), format!("{judged_by} ({gap})")));
        }
    }
    Ok((scanned, bad))
}

/// Live requirement names paired with the verification methods reaching each.
pub type SrMethods = (Vec<String>, HashMap<String, std::collections::BTreeSet<String>>);

/// Live `SystemRequirement` names, and the verification METHODS that `#Verify`-reach each.
///
/// Superseded requirements are excluded, the same scope rule `coverage`, `tier-satisfaction` and
/// (since issue127) `critique-coverage` use — a retired requirement is not pending anything.
///
/// Returns methods rather than a boolean because the caller's whole purpose is to STOP collapsing
/// them: `critique` and `test` are both "verified" under `sr_verified_pct` and answer entirely
/// different questions.
///
/// # Errors
/// Returns [`ViewError`] if a tracking file fails to parse.
pub fn sr_verification_methods(
    root: &Path,
) -> Result<SrMethods, ViewError> {
    let model = Model::build(root)?;
    let retired: HashSet<&str> =
        model.edges.iter().filter(|e| e.kind == "supersede").map(|e| e.to.as_str()).collect();
    let live: Vec<String> = model
        .items
        .iter()
        .filter(|(n, i)| i.type_name == "SystemRequirement" && !retired.contains(n.as_str()))
        .map(|(n, _)| n.clone())
        .collect();
    let live_set: HashSet<&str> = live.iter().map(String::as_str).collect();
    let mut out: HashMap<String, std::collections::BTreeSet<String>> = HashMap::new();
    for e in model.edges.iter().filter(|e| e.kind == "verify") {
        if !live_set.contains(e.to.as_str()) {
            continue;
        }
        if let Some(t) = model.items.get(&e.from) {
            if let Some(m) = t.attrs.get("method") {
                out.entry(e.to.clone()).or_default().insert(m.clone());
            }
        }
    }
    Ok((live, out))
}

/// One authored `CodeElement`, flattened for the `arch` views (D0148/`EngineCodeAudit`).
pub struct CodeElementRow {
    pub name: String,
    pub label: String,
    pub kind: String,
    pub file: String,
    pub code_hash: String,
    pub risk_class: String,
    pub invariant_safety: String,
    pub stpa_role: String,
    pub design_pattern: String,
    pub control_actions: Vec<String>,
    pub marker: Option<String>,
    /// Authored 0.0..1.0, or `None` where nobody recorded one — the distinction `arch coupling`
    /// prints as `-` rather than defaulting to a number it would then present as measured.
    pub abstractness: Option<f64>,
}

/// Everything the `arch` views compute over, from ONE model build.
///
/// Bundled rather than exposed as three accessors because each `Model::build` re-parses every
/// tracking file, and six subcommands each rebuilding three times is a visible cost on a repo this
/// size for no gain — the three pieces are always wanted together.
pub struct ArchModel {
    pub elements: Vec<CodeElementRow>,
    /// Every typed edge as `(kind, from, to)`.
    pub edges: Vec<(String, String, String)>,
    /// `Need` name -> authored `MoSCoW` `priority`, for the criticality bump.
    pub need_priority: HashMap<String, String>,
}

/// Read an attribute that may be authored as a scalar or as a `(a, b)` list.
///
/// The parser flattens attributes to strings, so a `String[0..*]` arrives as whatever the author
/// wrote. Handling both shapes here keeps that detail out of every caller, and means a registry
/// authored either way computes the same.
#[must_use]
pub fn attr_list(raw: &str) -> Vec<String> {
    let t = raw.trim();
    let inner = t.strip_prefix('(').and_then(|s| s.strip_suffix(')')).unwrap_or(t);
    inner
        .split(',')
        .map(|p| p.trim().trim_matches('"').trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

/// The authored code-audit registry plus the graph the `arch` views need.
///
/// # Errors
/// Returns [`ViewError`] if a tracking file fails to parse.
pub fn arch_model(root: &Path) -> Result<ArchModel, ViewError> {
    let model = Model::build(root)?;
    let mut elements: Vec<CodeElementRow> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "CodeElement")
        .map(|(n, i)| {
            let g = |k: &str| i.attrs.get(k).cloned().unwrap_or_default();
            CodeElementRow {
                name: n.clone(),
                label: display_label(n, i),
                kind: g("kind"),
                file: g("filePath"),
                code_hash: g("codeHash"),
                risk_class: g("riskClass"),
                invariant_safety: g("invariantSafety"),
                stpa_role: g("stpaRole"),
                design_pattern: g("designPattern"),
                control_actions: attr_list(&g("controlActions")),
                marker: i.marker.clone(),
                abstractness: i.attrs.get("abstractness").and_then(|v| v.trim().parse::<f64>().ok()),
            }
        })
        .collect();
    elements.sort_by(|a, b| a.name.cmp(&b.name));
    let need_priority = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Need")
        .map(|(n, i)| (n.clone(), i.attrs.get("priority").cloned().unwrap_or_default()))
        .collect();
    let edges = model.edges.iter().map(|e| (e.kind.clone(), e.from.clone(), e.to.clone())).collect();
    Ok(ArchModel { elements, edges, need_priority })
}

/// One claim as authored: `(name, item, by, at, against)`. Liveness is NOT here — it is computed.
pub type ClaimRow = (String, String, String, String, String);

/// Raw claim tuples from the model, for `claim::claims`.
///
/// # Errors
/// Returns [`ViewError`] if a tracking file fails to parse.
pub fn claim_rows(root: &Path) -> Result<Vec<ClaimRow>, ViewError> {
    let model = Model::build(root)?;
    let mut out: Vec<ClaimRow> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Claim")
        .map(|(n, i)| {
            let g = |k: &str| i.attrs.get(k).cloned().unwrap_or_default();
            (n.clone(), g("claimedItem"), g("claimedBy"), g("claimedAt"), g("claimedAgainst"))
        })
        .collect();
    out.sort();
    Ok(out)
}

/// Claim name -> its element id (a UUID), for the claim holder tie-break.
///
/// # Errors
/// Returns [`ViewError`] if a tracking file fails to parse.
pub fn claim_ids(root: &Path) -> Result<HashMap<String, String>, ViewError> {
    let model = Model::build(root)?;
    Ok(model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Claim")
        .map(|(n, i)| (n.clone(), i.attrs.get("id").cloned().unwrap_or_default()))
        .collect())
}

/// The HEAD commit date, exposed for the claim expiry computation (D0013 — git-derived, so two
/// contributors computing liveness against the same commit agree).
#[must_use]
pub fn repo_today_pub(root: &Path) -> String {
    repo_today(root)
}

/// Whole days between two ISO dates, exposed for the claim expiry computation.
#[must_use]
pub fn days_between_pub(from: &str, to: &str) -> i64 {
    days_between(from, to)
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
    fn configurable_slice_respects_depth_dir_and_edges() {
        // a -satisfy-> b -verify-> c ; a -dependency-> d
        let mut items = HashMap::new();
        for n in ["a", "b", "c", "d"] {
            items.insert(n.to_string(), item("Need", None));
        }
        let edges = vec![
            Edge { kind: "satisfy".to_string(), from: "a".to_string(), to: "b".to_string() },
            Edge { kind: "verify".to_string(), from: "b".to_string(), to: "c".to_string() },
            Edge { kind: "dependency".to_string(), from: "a".to_string(), to: "d".to_string() },
        ];
        let model = Model { items, edges };
        let all: HashSet<String> = HashSet::new();
        let names = |v: &[&str]| -> HashSet<String> { v.iter().map(|s| (*s).to_string()).collect() };
        // depth 1 down from a: a,b,d
        assert_eq!(configurable_slice(&model, "a", 1, &all, SliceDir::Down), names(&["a", "b", "d"]));
        // depth 2 down from a: a,b,c,d
        assert_eq!(configurable_slice(&model, "a", 2, &all, SliceDir::Down), names(&["a", "b", "c", "d"]));
        // satisfy-only edge filter, depth 2 down: verify is not followed -> a,b
        let sat: HashSet<String> = std::iter::once("satisfy".to_string()).collect();
        assert_eq!(configurable_slice(&model, "a", 2, &sat, SliceDir::Down), names(&["a", "b"]));
        // Up from c (change-impact / what reaches c): c,b,a
        assert_eq!(configurable_slice(&model, "c", 9, &all, SliceDir::Up), names(&["a", "b", "c"]));
        // unknown seed -> empty
        assert!(configurable_slice(&model, "zzz", 5, &all, SliceDir::Both).is_empty());
    }

    #[test]
    fn schema_scan_parses_defs_and_attributes() {
        // viewerSchemaApi (N-17): the text-scan recognizes type-def headers + attribute fields.
        assert_eq!(def_name("    part def Need :> TrackedRequirement {"), Some("Need".to_string()));
        assert_eq!(def_name("requirement def SystemRequirement {"), Some("SystemRequirement".to_string()));
        assert_eq!(def_name("    enum def NeedSource { customer; operator; }"), Some("NeedSource".to_string()));
        assert_eq!(def_name("    part someInstance : Need {"), None); // an instance, not a def
        assert_eq!(def_name("    // a comment mentioning def in prose"), None);
        assert_eq!(attr_field("        attribute source : NeedSource;"), Some(("source".to_string(), "NeedSource".to_string())));
        assert_eq!(attr_field("        attribute goals : String[*];"), Some(("goals".to_string(), "String".to_string())));
        assert_eq!(attr_field("        part def X {"), None);
    }

    #[test]
    fn enum_def_parses_members() {
        // encoding-semantics (N-18/D0120): an enum-typed attribute's domain = its declared members.
        assert_eq!(enum_def("    enum def DecisionStatus { proposed; accepted; rejected; superseded; }"),
            Some(("DecisionStatus".to_string(), vec!["proposed".to_string(), "accepted".to_string(), "rejected".to_string(), "superseded".to_string()])));
        assert_eq!(enum_def("enum def ActorKind { human; ai; }"),
            Some(("ActorKind".to_string(), vec!["human".to_string(), "ai".to_string()])));
        assert_eq!(enum_def("    part def Need :> X {"), None);
        assert_eq!(enum_def("    // enum def in a comment"), None);
    }

    #[test]
    fn ceremony_gate_derives_from_declared_phases() {
        // D0121 review queue — ceremony detection ADAPTS to the project's declared workflow phases;
        // it does NOT hardcode keel's phase names (the fix for "dynamically adjust to existing processes").
        let phases: Vec<String> = ["refine", "review", "closeOut"].iter().map(|s| (*s).to_string()).collect();
        assert!(is_ceremony_gate("vamReviewGate", &phases)); // declared phase -> ceremony
        assert!(is_ceremony_gate("vamCloseOutGate", &phases));
        assert!(is_ceremony_gate("storyFooDoD", &phases)); // DoD convention
        assert!(!is_ceremony_gate("keelViewerNeedsAccept5", &phases)); // genuine acceptance gate
        assert!(!is_ceremony_gate("someUndeclaredGate", &phases)); // 'undeclared' isn't a phase -> NOT hidden
        assert!(!is_ceremony_gate("vamReviewGate", &[])); // no declared workflow -> only DoD is ceremony
    }

    #[test]
    fn date_in_range_iso_lexicographic() {
        // N-5 time-as-query-filter: ISO-8601 dates compare chronologically as strings.
        assert!(date_in_range("2026-07-15", Some("2026-01-01"), Some("2026-12-31")));
        assert!(date_in_range("2026-07-15", Some("2026-07-15"), None)); // inclusive lower bound
        assert!(!date_in_range("2025-12-31", Some("2026-01-01"), None)); // before `since`
        assert!(!date_in_range("2027-01-01", None, Some("2026-12-31"))); // after `until`
        assert!(date_in_range("2026-07-15", None, None)); // no bounds -> always in range
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
    fn an_unresolved_grandfather_line_reports_every_sitting_as_due() {
        // D0155: the obligation surface must OVERSTATE rather than understate when it cannot resolve its
        // own boundary. A non-repo root has no D0155 introduction commit, so nothing may be grandfathered
        // — the opposite of the gate stance (D0050), because "nothing owed" must never be a guess.
        let dir = std::env::temp_dir().join("keel_sitting_gf_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".tracking/delivery")).unwrap();
        std::fs::create_dir_all(dir.join(".engine")).unwrap();
        std::fs::write(
            dir.join(".tracking/delivery/s.sysml"),
            "package P {\n    part s1 : Story { :>> id = \"11111111-1111-4111-8111-111111111111\"; :>> title = \"t\"; }\n}\n",
        )
        .unwrap();
        let out = sitting_coverage(&dir).unwrap();
        assert!(out.contains("\"uncovered\": 1"), "one uncovered sitting: {out}");
        assert!(out.contains("\"due\": 1"), "an unresolved line grandfathers NOTHING: {out}");
        assert!(out.contains("\"grandfathered_unreviewed\": 0"), "{out}");
        assert!(out.contains("GRANDFATHER LINE UNRESOLVED"), "the basis must SAY why it could not scope: {out}");
        let _ = std::fs::remove_dir_all(&dir);
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

    /// Build a passing (or failing) result item for verification `v`, as `latest_result` reads it.
    fn result_for(v: &str, outcome: &str) -> (String, ItemInfo) {
        let mut attrs = HashMap::new();
        attrs.insert("outcome".to_string(), outcome.to_string());
        (format!("{v}R1"), ItemInfo { type_name: "TestResult".to_string(), attrs, marker: None, file: String::new() })
    }

    /// A verification carrying `procedureText`.
    fn verification_named(text: &str) -> ItemInfo {
        let mut attrs = HashMap::new();
        attrs.insert("procedureText".to_string(), text.to_string());
        ItemInfo { type_name: "Test".to_string(), attrs, marker: None, file: String::new() }
    }

    /// A `method=confirmation` verification carrying `text`.
    fn confirmation(text: &str) -> ItemInfo {
        let mut attrs = HashMap::new();
        attrs.insert("method".to_string(), "confirmation".to_string());
        attrs.insert("procedureText".to_string(), text.to_string());
        ItemInfo { type_name: "Test".to_string(), attrs, marker: None, file: String::new() }
    }

    /// A registered human actor, as the bare-name check reads it.
    fn person(display: &str) -> ItemInfo {
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), display.to_string());
        ItemInfo { type_name: "Person".to_string(), attrs, marker: None, file: String::new() }
    }

    #[test]
    fn marker_scan_ignores_prose_and_catches_a_misspelling() {
        use crate::guards::{markers_declared_for_test as declared, markers_used_for_test as used};

        // Real syntactic positions are picked up; a marker QUOTED IN PROSE is not. That distinction is
        // load-bearing: procedureText fields legitimately discuss markers (`#Marker dependency from a
        // to b`), and without stripping string literals those alone produce false violations on a
        // HARD guard.
        let text = concat!(
            "package P {\n",
            "    #Verify dependency from t1 to sr1;\n",
            "    #Capability part feature : Story { }\n",
            "    verification x : Test { :>> procedureText = \"authored via #Marker dependency from a to b\"; }\n",
            "    // a comment mentioning #Covers dependency should not count\n",
            "    #DerivdFrom dependency from d1 to n1;\n",
            "}\n"
        );
        let found: Vec<String> = used(text).into_iter().map(|(m, _)| m).collect();
        assert_eq!(found, vec!["Verify", "Capability", "DerivdFrom"], "got {found:?}");

        // The declared set comes from `metadata def` lines PLUS the engine's builtin algebra.
        let schema = vec!["package R {\n    metadata def Verify;\n    metadata def Capability;\n}\n".to_string()];
        let dec = declared(&schema);
        assert!(dec.contains("Verify") && dec.contains("Capability"));

        // D0136/issue089: the ENGINE's own markers are valid with NO project declaration at all.
        // D0133 shipped a hard guard in the BINARY whose passing condition was project-side schema
        // content, so an existing downstream project (old .engine, new binary) hit 566 violations and
        // every commit blocked — on the engine's own shipped files — with the only remedy being an
        // edit to FROZEN schema/core. The engine's algebra is the engine's contract; the binary owns it.
        let none: Vec<String> = Vec::new();
        let bare = declared(&none);
        for m in ["Verify", "DerivedFrom", "CharteredBy", "Resolves", "Capability", "JustifiedBy"] {
            assert!(bare.contains(m), "engine marker #{m} must be valid without any project declaration");
        }
        // ...and a misspelling is still in NEITHER set, so the typo protection survives the fix.
        assert!(!bare.contains("DerivdFrom"), "a misspelling must never become valid");
        // The misspelling is exactly what must NOT be in the declared set — this is the real
        // failure mode: `#DerivdFrom` validated clean and silently removed its item from the HARD
        // requirement-rootedness guard's view.
        assert!(!dec.contains("DerivdFrom"), "a misspelling must not resolve as declared");
    }

    #[test]
    fn retro_backlog_warns_only_when_a_finding_is_neither_tracked_nor_justified() {
        use crate::guards::retro_backlog_warnings_for_test as warn;
        let sprint = |t: &str| vec![(".tracking/delivery/sprint999_x.sysml".to_string(), t.to_string())];
        let staged_sprint_only = vec![".tracking/delivery/sprint999_x.sysml".to_string()];
        let staged_with_item = vec![
            ".tracking/delivery/sprint999_x.sysml".to_string(),
            ".tracking/issues.sysml".to_string(),
        ];

        // A finding with nothing tracked and no reason -> warn (the sprint-247 failure).
        assert_eq!(warn(&staged_sprint_only, &sprint("AVOIDABLE-ISSUE 1: piping hung the kernel.")).len(), 1);
        // D0172/issue189 CHANGED THIS CASE: co-staging a tracked file no longer excuses the finding
        // - that exemption let one failure class reach five retros and zero items, because every
        // commit staged issues.sysml for something else. The retro must NAME its item now.
        assert_eq!(warn(&staged_with_item, &sprint("AVOIDABLE-ISSUE 1: piping hung the kernel.")).len(), 1);
        // Naming the item the finding produced -> clean (the D0172 tie).
        assert!(warn(&staged_with_item, &sprint("AVOIDABLE-ISSUE 1: piping hung the kernel - tracked as issue073.")).is_empty());
        // Same finding, explicitly justified as needing none -> clean. The obligation is that the
        // CHOICE is stated, not that an item always exists (a duplicate control is noise).
        assert!(warn(&staged_sprint_only, &sprint("AVOIDABLE-ISSUE 1: x — no new item, already guarded.")).is_empty());
        // A retro naming nothing avoidable -> clean; the guard must not demand findings.
        assert!(warn(&staged_sprint_only, &sprint("WELL: everything went fine.")).is_empty());
    }

    #[test]
    fn inversion_pairs_flags_lower_severity_work_ranked_above_high() {
        // issue084: order and severity are BOTH recorded, so disagreement between them is computable.
        // `ready` is in declaration/priority order (D0052), so "outranks" == "appears earlier".
        let sev = |s: &str| Some(s.to_string());
        let ready = vec![
            ("enabler".to_string(), None),                 // no issue at all
            ("cosmetic".to_string(), sev("Low")),
            ("urgent".to_string(), sev("High")),           // outranked by both above
            ("later".to_string(), sev("Medium")),
        ];
        let out = inversion_pairs(&ready);
        assert_eq!(
            out,
            vec![
                ("enabler".to_string(), "urgent".to_string(), "High".to_string()),
                ("cosmetic".to_string(), "urgent".to_string(), "High".to_string()),
            ],
            "got {out:?}"
        );
        // `later` (Medium) is NOT reported: the threshold is >= High, so ordinary triage ordering
        // does not generate noise — only work resolving a High/Critical issue being outranked does.
    }

    #[test]
    fn inversion_pairs_clean_when_severity_matches_order() {
        let sev = |s: &str| Some(s.to_string());
        let ready = vec![
            ("critical".to_string(), sev("Critical")),
            ("urgent".to_string(), sev("High")),
            ("enabler".to_string(), None),
            ("cosmetic".to_string(), sev("Low")),
        ];
        assert!(inversion_pairs(&ready).is_empty(), "severity-ordered frontier must be clean");
        // Two >= High items may outrank each other freely — relative order among them is judgment.
        assert!(inversion_pairs(&[("a".to_string(), sev("High")), ("b".to_string(), sev("Critical"))]).is_empty());
    }

    #[test]
    fn thin_attestation_list_catches_empty_stock_and_bare_name() {
        // issue083: `acceptance-events` and `confirmation-authenticity` verify an acceptance EXISTS and
        // is HUMAN-judged, never that it SAYS anything — so d0129Accept passed every guard while empty.
        // Length alone is the wrong test: "william weatherholtz" is exactly 20 chars and is not evidence.
        let mut items = HashMap::new();
        items.insert("wweatherholtz".to_string(), person("William Weatherholtz"));
        items.insert("aEmpty".to_string(), confirmation(""));
        items.insert("bStock".to_string(), confirmation("accepted"));
        items.insert("cBareName".to_string(), confirmation("william weatherholtz"));
        items.insert("dBareId".to_string(), confirmation("wweatherholtz"));
        items.insert("eShort".to_string(), confirmation("what done means"));
        items.insert("fGood".to_string(), confirmation("wweatherholtz attests acceptance of d0130 after reading its full text"));
        for n in ["aEmpty", "bStock", "cBareName", "dBareId", "eShort", "fGood"] {
            let (rn, ri) = result_for(n, "pass");
            items.insert(rn, ri);
        }
        let flagged: Vec<String> = thin_attestation_list(&Model { items, edges: Vec::new() }).into_iter().map(|(n, _)| n).collect();
        assert_eq!(flagged, vec!["aEmpty", "bStock", "cBareName", "dBareId", "eShort"], "got {flagged:?}");

        // Each reason is reported distinctly, so the fix is obvious from the message.
        let mut items2 = HashMap::new();
        items2.insert("wweatherholtz".to_string(), person("William Weatherholtz"));
        items2.insert("cBareName".to_string(), confirmation("William Weatherholtz"));
        let (rn, ri) = result_for("cBareName", "pass");
        items2.insert(rn, ri);
        let out = thin_attestation_list(&Model { items: items2, edges: Vec::new() });
        assert!(out[0].1.contains("only names an actor"), "reason should name the cause: {out:?}");
    }

    #[test]
    fn thin_attestation_list_ignores_unanswered_and_non_confirmation() {
        // An unanswered confirmation is a PENDING HUMAN OBLIGATION, not a defect — flagging it would
        // punish the human for not having signed off yet. And a thin procedureText on a method=test
        // verification is fine: a test carries its own evidence (D0016).
        let mut items = HashMap::new();
        items.insert("pending".to_string(), confirmation("")); // no result at all
        items.insert("failed".to_string(), confirmation(""));
        items.insert("aTest".to_string(), {
            let mut a = HashMap::new();
            a.insert("method".to_string(), "test".to_string());
            a.insert("procedureText".to_string(), "ok".to_string());
            ItemInfo { type_name: "Test".to_string(), attrs: a, marker: None, file: String::new() }
        });
        let (rn, ri) = result_for("failed", "fail");
        items.insert(rn, ri);
        let (rn2, ri2) = result_for("aTest", "pass");
        items.insert(rn2, ri2);
        assert!(thin_attestation_list(&Model { items, edges: Vec::new() }).is_empty());
    }

    #[test]
    fn untraced_links_flags_delivered_work_whose_requirement_has_no_verify_edge() {
        // issue082/D0130: the six sprint-247 SRs were DELIVERED (passing DoD + CI green) yet reported
        // unverified, because the DoD Tests #Verify-linked the backlog ACTION, not the requirement.
        let phases: Vec<String> = ["refine", "review"].iter().map(|s| (*s).to_string()).collect();
        let mut items = HashMap::new();
        items.insert("srAlpha".to_string(), item("SystemRequirement", None)); // delivered, untraced
        items.insert("srBeta".to_string(), item("SystemRequirement", None)); // already verified
        items.insert("srGamma".to_string(), item("SystemRequirement", None)); // named by unfinished work
        items.insert("doneDoD".to_string(), verification_named("delivers srAlpha and also srBeta"));
        items.insert("openDoD".to_string(), verification_named("will deliver srGamma"));
        items.insert("fooReviewGate".to_string(), verification_named("reviewed srAlpha"));
        items.insert("tBeta".to_string(), item("Test", None));
        for (n, i) in [result_for("doneDoD", "pass"), result_for("openDoD", "fail"), result_for("fooReviewGate", "pass")] {
            items.insert(n, i);
        }
        let edges = vec![Edge { kind: "verify".to_string(), from: "tBeta".to_string(), to: "srBeta".to_string() }];
        let model = Model { items, edges };

        let out = untraced_links(&model, &phases);
        assert_eq!(out, vec![("doneDoD".to_string(), "srAlpha".to_string())], "got {out:?}");
        // srBeta excluded: already #Verify-linked, so nothing is missing.
        // srGamma excluded: its work has NOT passed — an unverified planned requirement is honest
        // burndown (D0098), not a traceability defect.
        // fooReviewGate excluded: a declared PHASE gate verifies the sprint process, not the
        // requirement (this filter removed 37 of 103 real-repo findings as noise).
    }

    #[test]
    fn untraced_links_goes_clean_once_the_verify_edge_is_authored() {
        // The fix path the guard's message prescribes must actually clear the finding.
        let mut items = HashMap::new();
        items.insert("srAlpha".to_string(), item("SystemRequirement", None));
        items.insert("doneDoD".to_string(), verification_named("delivers srAlpha"));
        let (rn, ri) = result_for("doneDoD", "pass");
        items.insert(rn, ri);
        let model = Model { items: items.clone(), edges: Vec::new() };
        assert_eq!(untraced_links(&model, &[]).len(), 1, "flagged before the edge exists");

        let linked = Model {
            items,
            edges: vec![Edge { kind: "verify".to_string(), from: "doneDoD".to_string(), to: "srAlpha".to_string() }],
        };
        assert!(untraced_links(&linked, &[]).is_empty(), "clean once #Verify-linked");
    }

    #[test]
    fn untraced_links_requires_a_whole_token_match() {
        // `srAlpha` must not match inside `srAlphaBeta`, or the guard would invent findings.
        let mut items = HashMap::new();
        items.insert("srAlpha".to_string(), item("SystemRequirement", None));
        items.insert("doneDoD".to_string(), verification_named("delivers srAlphaBeta only"));
        let (rn, ri) = result_for("doneDoD", "pass");
        items.insert(rn, ri);
        let model = Model { items, edges: Vec::new() };
        assert!(untraced_links(&model, &[]).is_empty(), "substring must not count as a mention");
    }

    #[test]
    fn verified_method_mix_distinguishes_critique_from_test() {
        // issue082: `sr_verified_pct` counts ANY #Verify-linked Test, and in this repo the verified set
        // is ~70% method=critique — the ambiguity that let 34% be narrated as functional verification.
        let mut items = HashMap::new();
        items.insert("srA".to_string(), item("SystemRequirement", None));
        items.insert("srB".to_string(), item("SystemRequirement", None));
        items.insert("nC".to_string(), item("Need", None)); // verify edges to non-SRs are ignored
        let mut crit = HashMap::new();
        crit.insert("method".to_string(), "critique".to_string());
        items.insert("tCrit".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: crit, marker: None, file: String::new() });
        let mut tst = HashMap::new();
        tst.insert("method".to_string(), "test".to_string());
        items.insert("tTest".to_string(), ItemInfo { type_name: "Test".to_string(), attrs: tst, marker: None, file: String::new() });
        items.insert("tBare".to_string(), item("Test", None)); // no method attr -> "unstated"
        let edges = vec![
            Edge { kind: "verify".to_string(), from: "tCrit".to_string(), to: "srA".to_string() },
            Edge { kind: "verify".to_string(), from: "tTest".to_string(), to: "srB".to_string() },
            Edge { kind: "verify".to_string(), from: "tBare".to_string(), to: "srB".to_string() },
            Edge { kind: "verify".to_string(), from: "tCrit".to_string(), to: "nC".to_string() },
            Edge { kind: "satisfy".to_string(), from: "nC".to_string(), to: "srA".to_string() },
        ];
        let mix = verified_method_mix(&Model { items, edges });
        let get = |k: &str| mix.iter().find(|(m, _)| m == k).map_or(0, |(_, c)| *c);
        assert_eq!(get("critique"), 1, "the Need-targeted verify edge must NOT be counted: {mix:?}");
        assert_eq!(get("test"), 1);
        assert_eq!(get("unstated"), 1, "a method-less verifier is reported, not silently dropped");
    }

    #[test]
    fn a_descoped_need_is_excluded_from_the_gaps_and_reported_separately() {
        // issue088: counting a DESCOPED Need as an undecomposed gap understates completeness and —
        // the worse half — points a future contributor at authoring SystemRequirements for work that
        // was explicitly cut. The metric was recommending wrong work.
        let mut items = HashMap::new();
        items.insert("nKept".to_string(), item("Need", None)); // genuinely undecomposed
        items.insert("nCut".to_string(), item("Need", None)); // descoped by a Decision
        items.insert("nDone".to_string(), item("Need", None)); // decomposed
        items.insert("sr1".to_string(), item("SystemRequirement", None));
        let edges = vec![
            Edge { kind: "satisfy".to_string(), from: "nDone".to_string(), to: "sr1".to_string() },
            Edge { kind: "supersede".to_string(), from: "someDecision".to_string(), to: "nCut".to_string() },
        ];
        let stats = compute_tier_satisfaction(&Model { items, edges });
        let need = stats.iter().find(|t| t.tier == "Need").unwrap();
        assert_eq!(need.total, 2, "the descoped Need leaves the DENOMINATOR, not just the gap list");
        assert_eq!(need.satisfied, 1);
        assert_eq!(need.gaps, vec!["nKept".to_string()], "an undecomposed Need with no supersede edge STAYS a gap");
        assert_eq!(need.superseded, vec!["nCut".to_string()], "and the descoping stays visible — excluded must not mean invisible");
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

    #[test]
    fn pending_acceptances_are_the_proposed_decisions_only() {
        // issue096: the console rendered the accepted-only scorecard, so it showed everything EXCEPT
        // what needs the human. Only `proposed` is waiting — rejected and superseded are settled, and
        // counting them would recreate the same uselessness from the other direction.
        let with_status = |ty: &str, status: &str| {
            let mut a = HashMap::new();
            a.insert("status".to_string(), status.to_string());
            ItemInfo { type_name: ty.to_string(), attrs: a, marker: None, file: String::new() }
        };
        let mut items = HashMap::new();
        items.insert("d0002".to_string(), with_status("Decision", "DecisionStatus::proposed"));
        items.insert("d0001".to_string(), with_status("Decision", "DecisionStatus::accepted"));
        items.insert("d0003".to_string(), with_status("Decision", "DecisionStatus::rejected"));
        items.insert("d0004".to_string(), with_status("Decision", "DecisionStatus::superseded"));
        items.insert("d0005".to_string(), with_status("Decision", "DecisionStatus::proposed"));
        // A non-Decision carrying the same attribute must not leak in.
        items.insert("someStory".to_string(), with_status("Story", "DecisionStatus::proposed"));
        let model = Model { items, edges: Vec::new() };
        assert_eq!(proposed_decisions(&model), vec!["d0002".to_string(), "d0005".to_string()]);

        // And the empty case returns an EMPTY list rather than anything absent: orient always emits
        // the field, because a field that vanishes when empty is indistinguishable from one nobody
        // computed (the D0138 lesson).
        let mut only_accepted = HashMap::new();
        only_accepted.insert("d0001".to_string(), with_status("Decision", "DecisionStatus::accepted"));
        assert!(proposed_decisions(&Model { items: only_accepted, edges: Vec::new() }).is_empty());
    }

    #[test]
    fn a_task_depending_on_a_proposed_decision_is_blocked_and_unblocks_by_itself() {
        // issue112: the frontier is auto-followed (D0052), so ranking an item that cannot be started
        // points the next contributor at a wall. Distinct from superseded — this is WAITING, not
        // retired, and it must clear with no edit once the human answers.
        let with_status = |ty: &str, status: &str| {
            let mut a = HashMap::new();
            a.insert("status".to_string(), status.to_string());
            ItemInfo { type_name: ty.to_string(), attrs: a, marker: None, file: String::new() }
        };
        let mut items = HashMap::new();
        items.insert("dPending".to_string(), with_status("Decision", "DecisionStatus::proposed"));
        items.insert("dSettled".to_string(), with_status("Decision", "DecisionStatus::accepted"));
        let edges = vec![
            Edge { kind: "dependson".to_string(), from: "blockedTask".to_string(), to: "dPending".to_string() },
            Edge { kind: "dependson".to_string(), from: "freeTask".to_string(), to: "dSettled".to_string() },
        ];
        let model = Model { items: items.clone(), edges: edges.clone() };
        let blocked = blocked_by(&model);
        assert!(blocked.contains("blockedTask"), "{blocked:?}");
        assert!(!blocked.contains("freeTask"), "an ACCEPTED decision blocks nothing: {blocked:?}");

        // Accepting the decision unblocks the task with no other edit — nothing is stored.
        let mut accepted = items;
        accepted.insert("dPending".to_string(), with_status("Decision", "DecisionStatus::accepted"));
        assert!(blocked_by(&Model { items: accepted, edges }).is_empty());
    }

    #[test]
    fn critique_coverage_excludes_superseded_like_its_sibling_views() {
        // issue127: `compute_coverage` and `compute_tier_satisfaction` both drop supersede-targeted
        // elements; `critique_coverage` did not, so a RETIRED element sat in the denominator forever.
        // The property is that a superseded element is OUT OF SCOPE — `governed: false` and absent
        // from the gap set — NOT that it vanishes from the detail listing. The first draft of this
        // test asserted the name appeared nowhere in the JSON, which failed on `nViewerMultiStakeholder`
        // for the wrong reason: the elements array lists every element and marks its scope per row,
        // which is what makes the view auditable. Asserting absence would have driven a fix that hid
        // retired elements instead of descoping them.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let Ok(model) = Model::build(&root) else { return };
        let superseded: Vec<String> =
            model.edges.iter().filter(|e| e.kind == "supersede").map(|e| e.to.clone()).collect();
        if superseded.is_empty() {
            return; // nothing retired in this tree; the property is vacuous rather than false
        }
        let Ok(json) = critique_coverage(&root) else { return };
        for name in &superseded {
            if let Some(i) = json.find(&format!("\"element\": \"{name}\"")) {
                let row = &json[i..(i + 400).min(json.len())];
                if let Some(g) = row.find("\"governed\":") {
                    assert!(
                        row[g..].starts_with("\"governed\": false"),
                        "`{name}` is superseded and must be out of scope (issue127): {}",
                        &row[g..(g + 40).min(row.len())]
                    );
                }
            }
        }
    }
}

/// GET /api/surfaces — the console's navigation, COMPUTED from the declared Viewpoint registry
/// (D0152/D0154, srConsoleNavigationDerived).
///
/// # Why this exists rather than a list in the console
///
/// The console's navigation was a literal array of twelve entries organised by data type, grown one
/// per API endpoint. N-17 forbids exactly that — the surface must generate itself from the declared
/// model — and srConsoleArrivalBounded requires that adding a viewpoint NOT add a top-level choice.
/// Both hold here because the top level is the DISTINCT SET OF DECLARED SURFACES: 32 viewpoints
/// currently resolve to 6 surfaces, and declaring a 33rd changes the count only if it names a
/// surface no other viewpoint claims.
///
/// A viewpoint with NO declared surface is returned under `unsurfaced` rather than dropped or
/// defaulted — an absent grouping is a gap the reader must see (N-C2), and silently filing it under
/// some default would be the confident-wrong-answer failure this engine keeps producing.
///
/// # Errors
/// Returns [`ViewError`] if the registry cannot be read.
pub(crate) fn surfaces_json(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut by_surface: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    let mut unsurfaced: Vec<Json> = Vec::new();
    let mut names: Vec<&String> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Viewpoint")
        .map(|(n, _)| n)
        .collect();
    names.sort();
    // An INACTIVE viewpoint is not offered (D0164). Filtering here rather than in each consumer means the
    // console nav, the act-surface obligation bar and anything else derived from surfaces all agree, and a
    // project that deactivated a lens does not keep being invited to look through it.
    let act = crate::activation::Activation::load(root);
    for n in names {
        if !act.is_viewpoint_active(n) {
            continue;
        }
        let Some(i) = model.items.get(n) else { continue };
        let g = |k: &str| i.attrs.get(k).cloned().unwrap_or_default();
        let entry = Json::Obj(vec![
            ("viewpoint".to_string(), Json::s(n.clone())),
            ("title".to_string(), Json::s(g("title"))),
            ("concern".to_string(), Json::s(g("concernText"))),
            ("audience".to_string(), Json::s(g("audience"))),
            ("renderer".to_string(), Json::s(g("renderer"))),
        ]);
        let s = g("surface");
        if s.trim().is_empty() {
            unsurfaced.push(entry);
        } else {
            by_surface.entry(s).or_default().push(entry);
        }
    }
    let surfaces: Vec<Json> = by_surface
        .into_iter()
        .map(|(name, vps)| {
            Json::Obj(vec![
                ("surface".to_string(), Json::s(name)),
                ("count".to_string(), Json::Int(i64::try_from(vps.len()).unwrap_or(i64::MAX))),
                ("viewpoints".to_string(), Json::Arr(vps)),
            ])
        })
        .collect();
    Ok(Json::Obj(vec![
        (
            "surfaces_note".to_string(),
            Json::s(
                "console navigation, COMPUTED from the declared Viewpoint registry (D0154). Top-level \
                 choices are the DISTINCT declared surfaces, so declaring a viewpoint does not add one \
                 unless it names a new surface. A viewpoint with no declared surface appears under \
                 `unsurfaced` — never defaulted onto a surface it did not claim.",
            ),
        ),
        ("status".to_string(), Json::s("computed")),
        ("surfaces".to_string(), Json::Arr(surfaces)),
        ("unsurfaced".to_string(), Json::Arr(unsurfaced)),
    ])
    .dump())
}

/// Every item name paired with the repo-relative file it was authored in (N-C3 model scope).
///
/// The authoring file is the only honest basis for an item's model scope: `.engine/` holds the
/// engine's own definitions and `.tracking/` holds the work being tracked. Anything else is UNSCOPED,
/// which the caller must report rather than default — a wrong scope is worse than an admitted unknown.
///
/// # Errors
/// Returns [`ViewError`] if a tracking file fails to parse.
pub fn item_files(root: &Path) -> Result<Vec<(String, String)>, ViewError> {
    let model = Model::build(root)?;
    let mut out: Vec<(String, String)> =
        model.items.iter().map(|(n, i)| (n.clone(), i.file.clone())).collect();
    out.sort();
    Ok(out)
}

