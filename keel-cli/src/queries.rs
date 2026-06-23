//! Secondary read-only query views re-added by readdDroppedViews (resolves issue025).
//!
//! `outstanding`, `item`, `trace` (up + downstream), `trace-need`, `workflows` — query.py
//! subcommands dropped at M4, re-implemented over the Rust authority: the indexer's action DAG,
//! the parser's satisfy/allocate edges, and the workflow action defs. The `viewpoints` listing
//! is a TOML view (`keel view viewpoints`), not here.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use keel_parser::ast::Item;
use keel_parser::{parse, tokenize};

use crate::json::Json;

fn idx(root: &Path) -> crate::indexer::ExtractedIndex {
    crate::indexer::extract(&root.join(".tracking"))
}

/// `keel outstanding` — backlog tasks that are not done.
#[must_use]
pub fn outstanding(root: &Path) -> String {
    let tasks = idx(root).tasks;
    let done = crate::orient::done_names(root);
    let mut out: Vec<String> = tasks.keys().filter(|t| !done.contains(t.as_str())).cloned().collect();
    out.sort();
    Json::Obj(vec![("outstanding".to_string(), Json::Arr(out.into_iter().map(Json::s).collect()))]).dump()
}

/// `keel item <name>` — one task's detail (done, deps, `DoD` text, results).
#[must_use]
pub fn item(root: &Path, name: &str) -> String {
    let index = idx(root);
    let done = crate::orient::done_names(root);
    let Some(t) = index.tasks.get(name) else {
        return Json::Obj(vec![("error".to_string(), Json::s(format!("no task '{name}'")))]).dump();
    };
    let results: Vec<Json> = t
        .results
        .iter()
        .map(|r| {
            Json::Obj(vec![
                ("n".to_string(), Json::Int(i64::from(r.n))),
                ("outcome".to_string(), Json::s(r.outcome.clone())),
                ("judgedAgainst".to_string(), Json::s(r.judged_against.clone())),
            ])
        })
        .collect();
    Json::Obj(vec![
        ("name".to_string(), Json::s(name)),
        ("done".to_string(), Json::Bool(done.contains(name))),
        ("deps".to_string(), Json::Arr(t.deps.iter().map(|d| Json::s(d.clone())).collect())),
        ("dod".to_string(), Json::s(t.dod_text.clone().unwrap_or_default())),
        ("results".to_string(), Json::Arr(results)),
    ])
    .dump()
}

fn reach(start: &str, adj: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut q: VecDeque<String> = VecDeque::new();
    if let Some(ns) = adj.get(start) {
        q.extend(ns.iter().cloned());
    }
    while let Some(n) = q.pop_front() {
        if seen.insert(n.clone()) {
            if let Some(ns) = adj.get(&n) {
                q.extend(ns.iter().cloned());
            }
        }
    }
    let mut out: Vec<String> = seen.into_iter().collect();
    out.sort();
    out
}

/// `keel trace <name>` — transitive upstream (deps) + downstream (dependents) over the
/// succession DAG. Covers the dropped `trace`, `upstream`, and `downstream`.
#[must_use]
pub fn trace(root: &Path, name: &str) -> String {
    let tasks = idx(root).tasks;
    let upstream_adj: HashMap<String, Vec<String>> = tasks.iter().map(|(k, v)| (k.clone(), v.deps.clone())).collect();
    let mut downstream_adj: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in &tasks {
        for d in &v.deps {
            downstream_adj.entry(d.clone()).or_default().push(k.clone());
        }
    }
    Json::Obj(vec![
        ("name".to_string(), Json::s(name)),
        ("upstream".to_string(), Json::Arr(reach(name, &upstream_adj).into_iter().map(Json::s).collect())),
        ("downstream".to_string(), Json::Arr(reach(name, &downstream_adj).into_iter().map(Json::s).collect())),
    ])
    .dump()
}

fn parse_dir(dir: &Path) -> Vec<keel_parser::ast::Package> {
    crate::collect_sysml(dir)
        .iter()
        .filter_map(|p| {
            let src = std::fs::read_to_string(p).ok()?;
            let name = p.display().to_string();
            let tokens = tokenize(&src, &name).ok()?;
            parse(tokens, &name).ok()
        })
        .collect()
}

/// `keel trace-need <name>` — forward closure over satisfy/allocate edges from a Need
/// (need -satisfy-> requirement -allocate-> component).
#[must_use]
pub fn trace_need(root: &Path, name: &str) -> String {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for pkg in parse_dir(&root.join(".tracking")) {
        for it in &pkg.items {
            match it {
                Item::Satisfy(e) => adj.entry(e.need.clone()).or_default().push(e.by.clone()),
                Item::Allocate(e) => adj.entry(e.sr.clone()).or_default().push(e.to.clone()),
                _ => {}
            }
        }
    }
    Json::Obj(vec![
        ("need".to_string(), Json::s(name)),
        ("trace".to_string(), Json::Arr(reach(name, &adj).into_iter().map(Json::s).collect())),
    ])
    .dump()
}

/// `keel workflows` — each workflow action def's phases as Kahn topological waves over its
/// succession edges.
#[must_use]
pub fn workflows(root: &Path) -> String {
    let mut wfs: Vec<Json> = Vec::new();
    for pkg in parse_dir(&root.join(".engine").join("workflows")) {
        for it in &pkg.items {
            let Item::ActionDef(def) = it else { continue };
            let nodes: Vec<String> = def.actions.iter().map(|a| a.name.clone()).collect();
            if nodes.is_empty() {
                continue;
            }
            let edges: Vec<(String, String)> = def.successions.iter().map(|s| (s.first.clone(), s.then.clone())).collect();
            wfs.push(workflow_json(&pkg.name, &def.name, &nodes, &edges));
        }
    }
    Json::Obj(vec![("workflows".to_string(), Json::Arr(wfs))]).dump()
}

fn workflow_json(package: &str, name: &str, nodes: &[String], edges: &[(String, String)]) -> Json {
    let mut indeg: HashMap<&str, usize> = nodes.iter().map(|n| (n.as_str(), 0)).collect();
    let mut succ: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in edges {
        succ.entry(a.as_str()).or_default().push(b.as_str());
        *indeg.entry(b.as_str()).or_insert(0) += 1;
    }
    let mut waves: Vec<Vec<String>> = Vec::new();
    let mut placed = 0usize;
    let mut frontier: Vec<&str> = indeg.iter().filter(|(_, &d)| d == 0).map(|(n, _)| *n).collect();
    frontier.sort_unstable();
    while !frontier.is_empty() {
        waves.push(frontier.iter().map(|s| (*s).to_string()).collect());
        placed += frontier.len();
        let mut next: Vec<&str> = Vec::new();
        for n in &frontier {
            for m in succ.get(n).into_iter().flatten() {
                let e = indeg.entry(m).or_insert(0);
                *e = e.saturating_sub(1);
                if *e == 0 {
                    next.push(m);
                }
            }
        }
        next.sort_unstable();
        next.dedup();
        frontier = next;
    }
    let mut obj = vec![
        ("workflow".to_string(), Json::s(name)),
        ("package".to_string(), Json::s(package)),
        ("phaseCount".to_string(), Json::Int(i64::try_from(nodes.len()).unwrap_or(i64::MAX))),
    ];
    if placed < nodes.len() {
        obj.push(("error".to_string(), Json::s("dependency cycle (not all phases placed)")));
    }
    obj.push(("waves".to_string(), Json::Arr(waves.into_iter().map(|w| Json::Arr(w.into_iter().map(Json::s).collect())).collect())));
    Json::Obj(obj)
}
