//! `keel arch` — computed views over an authored `CodeElement` registry (D0148/keelArchViews).
//!
//! # keel does not read your code to decide what exists
//!
//! Every view here computes over AUTHORED facts: `CodeElement` instances, the typed edges between
//! them, and the Needs graph. A unit nobody catalogued is invisible, which is a deliberate property
//! and also a trap — a registry covering three files looks exactly like a codebase with three files.
//! `arch coverage` exists to say that out loud, and is the one view that scans source, heuristically
//! and non-blockingly.
//!
//! The second place source is touched is `arch drift`, which re-hashes the block under an
//! `@audit-hash` marker and compares it to the recorded `codeHash`. That is a staleness check on the
//! RECORD, not discovery: it can only tell you that something you already catalogued has moved on.

use crate::view::{ArchModel, CodeElementRow};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Worst-first, DERIVED from the `RiskClass` enum's DECLARATION ORDER — the schema decides the
/// ranking, not a literal here that happened to agree with it (issue120).
///
/// Read from THE PROJECT's schema, falling back to the engine's (issue128). A project's risk
/// taxonomy is its own judgment, and imposing the engine's produced a confidently wrong answer on
/// real downstream data: `SelfSync`'s 15 `durability` and 8 `concurrency` elements ranked below
/// `cosmetic` and printed as `unclassified` in the view whose whole job is what-to-audit-first.
fn risk_order(root: &Path) -> Vec<String> {
    crate::schema::project_enum_members(root, "RiskClass")
}

/// Rank of a risk class, lower = worse. Unknown/absent sorts last, never first: an element whose
/// risk nobody recorded must not outrank one someone judged to be critical.
fn risk_rank(order: &[String], risk: &str) -> usize {
    order.iter().position(|r| r == risk).unwrap_or(order.len())
}

/// FNV-1a over whitespace-normalised text.
///
/// NOT `DefaultHasher`: that is explicitly not stable across Rust releases, so a recorded hash would
/// drift when the toolchain moved and every element would report as changed after an upgrade. Lines
/// are trimmed and blanks dropped so a CRLF checkout does not read as drift — this repo is developed
/// on Windows and consumed on CI, and a hash that disagrees between the two would be worse than none.
fn stable_hash(text: &str) -> String {
    let norm: String =
        text.lines().map(str::trim_end).filter(|l| !l.trim().is_empty()).collect::<Vec<_>>().join("\n");
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in norm.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

/// The source block a `// @audit-hash` marker introduces: from the marker to the end of the brace
/// block that follows it.
///
/// Returns `None` when the marker is absent — distinct from present-but-changed, because "never
/// marked" and "marked and drifted" call for different work and collapsing them would make the
/// re-audit frontier meaningless.
fn marked_block(src: &str, element: &str) -> Option<String> {
    let lines: Vec<&str> = src.lines().collect();
    let start = lines.iter().position(|l| l.contains("@audit-hash") && l.contains(element))?;
    let mut depth: i32 = 0;
    let mut seen_open = false;
    let mut out = Vec::new();
    for line in lines.iter().skip(start + 1) {
        out.push(*line);
        for c in line.chars() {
            match c {
                '{' => {
                    depth += 1;
                    seen_open = true;
                }
                '}' => depth -= 1,
                _ => {}
            }
        }
        if seen_open && depth <= 0 {
            break;
        }
    }
    Some(out.join("\n"))
}

/// Highest `MoSCoW` priority of any `Need` reachable from `element` along trace edges.
///
/// Bounded traversal over the edges that actually mean "this exists because of that" — an unbounded
/// walk of every edge kind would reach a Need from almost anything and the bump would stop
/// discriminating.
fn traced_need_priority(el: &str, m: &ArchModel) -> Option<String> {
    // Edge kinds are LOWERCASED at ingest (`view.rs::edge_kind_from_marker`), so these must be too —
    // matching `"DerivedFrom"` silently found nothing and the criticality bump never fired.
    const TRACE: [&str; 4] = ["satisfy", "derivedfrom", "satisfies", "verify"];
    let rank = |p: &str| match p {
        "must" => 0,
        "should" => 1,
        "could" => 2,
        _ => 3,
    };
    let mut frontier = vec![el.to_string()];
    let mut seen: HashSet<String> = HashSet::new();
    let mut best: Option<String> = None;
    for _ in 0..4 {
        let mut next = Vec::new();
        for node in std::mem::take(&mut frontier) {
            if !seen.insert(node.clone()) {
                continue;
            }
            if let Some(p) = m.need_priority.get(&node) {
                if best.as_ref().is_none_or(|b| rank(p) < rank(b)) {
                    best = Some(p.clone());
                }
            }
            for (kind, from, to) in &m.edges {
                if *from == node && TRACE.contains(&kind.as_str()) {
                    next.push(to.clone());
                }
            }
        }
        frontier = next;
        if frontier.is_empty() {
            break;
        }
    }
    best
}

fn flag(args: &[String], name: &str) -> Option<String> {
    let i = args.iter().position(|a| a == name)?;
    args.get(i + 1).filter(|v| !v.starts_with("--")).cloned()
}

fn load(root: &Path) -> Result<ArchModel, i32> {
    // A root with no `.tracking/` is a WRONG PATH, not an empty registry, and the two must not print
    // the same thing. `arch elements .` briefly resolved its root to `./elements` and reported "no
    // CodeElement instances authored" — a confident empty answer about a directory that did not
    // exist, which is exactly the failure this engine's premise cannot survive.
    if !root.join(".tracking").is_dir() {
        eprintln!("error: {} has no .tracking/ — not a keel repo root.", root.display());
        eprintln!("  usage: keel arch <subcommand> [ROOT]   (the ROOT goes AFTER the subcommand)");
        return Err(2);
    }
    match crate::view::arch_model(root) {
        Ok(m) if m.elements.is_empty() => {
            println!("no CodeElement instances authored.");
            println!("  `arch` computes over an AUTHORED registry — it does not discover code. Author");
            println!("  CodeElement instances importing EngineCodeAudit (D0148), then re-run.");
            Err(0)
        }
        Ok(m) => Ok(m),
        Err(e) => {
            eprintln!("error: {e}");
            Err(1)
        }
    }
}

fn cmd_elements(m: &ArchModel, args: &[String]) -> i32 {
    let kind = flag(args, "--kind");
    let file = flag(args, "--file");
    let rows: Vec<&CodeElementRow> = m
        .elements
        .iter()
        .filter(|e| kind.as_ref().is_none_or(|k| &e.kind == k))
        .filter(|e| file.as_ref().is_none_or(|f| e.file.contains(f.as_str())))
        .collect();
    println!("{} code element(s):", rows.len());
    for e in rows {
        let pattern = if e.design_pattern.is_empty() { String::new() } else { format!(" [{}]", e.design_pattern) };
        println!("  {:<28} {:<10} {:<8} {}{}", e.label, e.kind, e.risk_class, e.file, pattern);
    }
    0
}

fn cmd_criticality(m: &ArchModel, order: &[String]) -> i32 {
    // The ranked audit frontier: risk class, bumped one tier when the element traces to a `must`
    // Need. A bump can only ever RAISE criticality — an element with no Need trace keeps its
    // authored risk rather than being demoted, because absence of a trace is missing information,
    // not evidence that the element is safe.
    let mut rows: Vec<(usize, bool, &CodeElementRow)> = m
        .elements
        .iter()
        .map(|e| {
            let bumped = traced_need_priority(&e.name, m).as_deref() == Some("must");
            let base = risk_rank(order, &e.risk_class);
            (if bumped { base.saturating_sub(1) } else { base }, bumped, e)
        })
        .collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.name.cmp(&b.2.name)));
    println!("audit frontier, most critical first ({} element(s)):", rows.len());
    for (rank, bumped, e) in rows {
        let tier = order.get(rank).map_or("unclassified", String::as_str);
        let why = if bumped { "  <- bumped: traces to a `must` Need" } else { "" };
        let safety = if e.invariant_safety.is_empty() { "-" } else { e.invariant_safety.as_str() };
        println!("  {tier:<12} {:<28} invariants={safety}{why}", e.label);
    }
    0
}

fn cmd_coupling(m: &ArchModel) -> i32 {
    let names: HashSet<&str> = m.elements.iter().map(|e| e.name.as_str()).collect();
    let deps: Vec<(&str, &str)> = m
        .edges
        .iter()
        .filter(|(k, f, t)| k == "dependson" && names.contains(f.as_str()) && names.contains(t.as_str()))
        .map(|(_, f, t)| (f.as_str(), t.as_str()))
        .collect();
    let mut ca: HashMap<&str, usize> = HashMap::new();
    let mut ce: HashMap<&str, usize> = HashMap::new();
    for (f, t) in &deps {
        *ce.entry(f).or_default() += 1;
        *ca.entry(t).or_default() += 1;
    }
    println!("coupling over {} #DependsOn edge(s) among catalogued elements:", deps.len());
    println!("  {:<28} {:>3} {:>3} {:>6} {:>6}", "element", "Ca", "Ce", "I", "D");
    for e in &m.elements {
        let (a, b) = (ca.get(e.name.as_str()).copied().unwrap_or(0), ce.get(e.name.as_str()).copied().unwrap_or(0));
        #[allow(clippy::cast_precision_loss)]
        let inst = if a + b == 0 { 0.0 } else { b as f64 / (a + b) as f64 };
        // Distance from the main sequence needs an AUTHORED abstractness; `-` where absent, rather
        // than a number derived from `kind` and presented as a measurement.
        let dist = e.abstractness.map_or_else(
            || "     -".to_string(),
            |abs| format!("{:>6.2}", (abs + inst - 1.0).abs()),
        );
        println!("  {:<28} {a:>3} {b:>3} {inst:>6.2} {dist}", e.label);
    }
    for cyc in cycles(&deps) {
        println!("  CYCLE: {}", cyc.join(" -> "));
    }
    0
}

/// Dependency cycles, reported as the node sequence that closes the loop.
fn cycles<'a>(deps: &[(&'a str, &'a str)]) -> Vec<Vec<&'a str>> {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for (f, t) in deps {
        adj.entry(f).or_default().push(t);
    }
    let mut found = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for start in adj.keys().copied() {
        if seen.contains(start) {
            continue;
        }
        let mut stack = vec![(start, Vec::new())];
        while let Some((node, path)) = stack.pop() {
            if path.contains(&node) {
                let mut c: Vec<&str> = path.into_iter().skip_while(|n| *n != node).collect();
                c.push(node);
                found.push(c);
                continue;
            }
            if path.len() > 24 {
                continue;
            }
            seen.insert(node);
            let mut p = path;
            p.push(node);
            for n in adj.get(node).into_iter().flatten() {
                stack.push((n, p.clone()));
            }
        }
    }
    found.truncate(20);
    found
}

fn cmd_drift(m: &ArchModel, root: &Path) -> i32 {
    let (mut drifted, mut unaudited, mut unmarked, mut ok, mut unlocated) = (0, 0, 0, 0, 0);
    println!("re-audit frontier:");
    for e in &m.elements {
        if e.code_hash.is_empty() {
            unaudited += 1;
            println!("  UNAUDITED  {:<28} (catalogued, never hashed)", e.label);
            continue;
        }
        // NO PATH AT ALL is not a missing file, and conflating them made this view lie on real data
        // (issue130): a downstream registry that records the path under a different attribute name
        // yielded an empty `file`, `root.join("")` resolved to the repo root, reading a DIRECTORY
        // failed, and all 169 elements reported as `MISSING ... not readable` — a confident wrong
        // answer about files that were never named rather than files that were absent.
        if e.file.trim().is_empty() {
            unlocated += 1;
            println!("  UNLOCATED  {:<28} no filePath authored — nothing to hash against", e.label);
            continue;
        }
        let Ok(src) = std::fs::read_to_string(root.join(&e.file)) else {
            println!("  MISSING    {:<28} {} not readable", e.label, e.file);
            drifted += 1;
            continue;
        };
        match marked_block(&src, &e.name) {
            None => {
                unmarked += 1;
                println!("  NO-MARKER  {:<28} no `@audit-hash {}` in {}", e.label, e.name, e.file);
            }
            Some(block) => {
                let live = stable_hash(&block);
                if live == e.code_hash {
                    ok += 1;
                } else {
                    drifted += 1;
                    println!("  DRIFTED    {:<28} recorded {} -> live {live}", e.label, e.code_hash);
                }
            }
        }
    }
    println!("  {drifted} drifted, {unaudited} unaudited, {unmarked} unmarked, {unlocated} unlocated, {ok} current");
    0
}

fn cmd_stpa_inputs(m: &ArchModel) -> i32 {
    // A HAND-OFF, not an analysis. This prints the recorded control structure so an STPA process
    // (EngineSafety) can start from it. Deriving unsafe control actions here would be an analysis
    // wearing a view's clothes, and it would carry none of the human judgment STPA requires.
    let controllers: Vec<&CodeElementRow> = m.elements.iter().filter(|e| e.stpa_role == "controller").collect();
    println!("control structure for STPA hand-off ({} controller(s)):", controllers.len());
    for c in controllers {
        println!("  controller {}", c.label);
        for a in &c.control_actions {
            println!("      issues: {a}");
        }
        for (k, f, t) in &m.edges {
            if f == &c.name && k == "controls" {
                println!("      controls -> {t}");
            }
            if t == &c.name && k == "feedback" {
                println!("      feedback <- {f}");
            }
        }
        let has_feedback = m.edges.iter().any(|(k, _, t)| k == "feedback" && t == &c.name);
        if !has_feedback {
            println!("      NOTE: no #Feedback edge — a control loop with no return path is the");
            println!("            classic STPA finding, and this view can only flag it, not judge it.");
        }
    }
    let roles: Vec<&CodeElementRow> = m.elements.iter().filter(|e| !e.stpa_role.is_empty()).collect();
    println!("  {} element(s) carry an stpaRole; {} carry none.", roles.len(), m.elements.len() - roles.len());
    0
}

fn cmd_coverage(m: &ArchModel, root: &Path) -> i32 {
    // NON-BLOCKING and HEURISTIC, and says so, because it is the one view that guesses. It counts
    // top-level Rust definitions per file and compares against what the registry catalogues. An
    // over-count is expected; the number is an indicator, never a gate (§1.7).
    let files: HashSet<&str> = m.elements.iter().map(|e| e.file.as_str()).collect();
    let mut total_defs = 0usize;
    let mut per_file: Vec<(String, usize, usize, bool)> = Vec::new();
    for f in files {
        let Ok(src) = std::fs::read_to_string(root.join(f)) else { continue };
        let defs = src
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                l.starts_with(|c: char| !c.is_whitespace())
                    && (t.starts_with("pub fn ")
                        || t.starts_with("fn ")
                        || t.starts_with("pub struct ")
                        || t.starts_with("struct ")
                        || t.starts_with("pub enum ")
                        || t.starts_with("enum ")
                        || t.starts_with("impl "))
            })
            .count();
        let catalogued = m.elements.iter().filter(|e| e.file == f).count();
        let module = m.elements.iter().any(|e| e.file == f && e.kind == "module");
        total_defs += defs;
        per_file.push((f.to_string(), catalogued, defs, module));
    }
    per_file.sort();
    println!("registry coverage (HEURISTIC, non-blocking):");
    for (f, cat, defs, module) in &per_file {
        // GRANULARITY MATTERS, and mixing it made this view lie. A `module` element takes
        // responsibility for the whole file, so counting every function in that file as an
        // un-catalogued gap reported 220 gaps against view.rs when the file is in fact covered at the
        // granularity someone chose. A file covered at module level reports its def count as CONTEXT;
        // only a file catalogued purely element-by-element gets a gap number, where the subtraction
        // is between like and like.
        if *module {
            println!("  {f:<44} module-covered ({cat} element(s), ~{defs} def(s) inside)");
        } else {
            let gap = defs.saturating_sub(*cat);
            println!("  {f:<44} {cat:>3} element(s) / ~{defs:>3} def(s){}", if gap > 0 { format!("  ({gap} un-catalogued)") } else { String::new() });
        }
    }
    let module_files = per_file.iter().filter(|(_, _, _, m)| *m).count();
    println!("  {} catalogued across {} file(s), {module_files} covered at module level; ~{total_defs} top-level definitions seen.", m.elements.len(), per_file.len());
    println!("  Files with NO catalogued element are invisible to this count — it can only look where");
    println!("  the registry already points, so it under-reports exactly where coverage is worst.");
    0
}

/// `keel arch <subcommand>`.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    let Some(sub) = args.first().map(String::as_str) else {
        eprintln!("usage: keel arch <elements|criticality|coupling|drift|stpa-inputs|coverage> [ROOT] [flags]");
        return 2;
    };
    let m = match load(root) {
        Ok(m) => m,
        Err(code) => return code,
    };
    let rest = args.get(1..).unwrap_or(&[]);
    match sub {
        "elements" => cmd_elements(&m, rest),
        "criticality" => cmd_criticality(&m, &risk_order(root)),
        "coupling" => cmd_coupling(&m),
        "drift" => cmd_drift(&m, root),
        "stpa-inputs" => cmd_stpa_inputs(&m),
        "coverage" => cmd_coverage(&m, root),
        other => {
            eprintln!("error: unknown `arch` subcommand `{other}`");
            eprintln!("  one of: elements criticality coupling drift stpa-inputs coverage");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_order_is_worst_first_and_unknown_never_outranks() {
        let o = risk_order(&repo_root());
        assert!(risk_rank(&o, "dataLoss") < risk_rank(&o, "security"));
        assert!(risk_rank(&o, "cosmetic") < risk_rank(&o, "not-a-class"));
    }

    #[test]
    fn hash_ignores_line_endings_and_blank_lines() {
        assert_eq!(stable_hash("fn a() {\r\n  b();\r\n}\r\n"), stable_hash("fn a() {\n\n  b();\n}\n"));
        assert_ne!(stable_hash("fn a() { b(); }"), stable_hash("fn a() { c(); }"));
    }

    #[test]
    fn marked_block_stops_at_the_matching_brace() {
        let src = "// @audit-hash thing\nfn thing() {\n  if x { y(); }\n}\nfn other() { z(); }\n";
        let b = marked_block(src, "thing").expect("marker present");
        assert!(b.contains("y();"), "captured the nested block");
        assert!(!b.contains("z();"), "stopped before the next definition");
    }

    #[test]
    fn missing_marker_is_none_not_empty() {
        assert!(marked_block("fn thing() {}\n", "thing").is_none());
    }

    #[test]
    fn attr_list_reads_scalar_and_list_forms() {
        assert_eq!(crate::view::attr_list("\"a\""), vec!["a".to_string()]);
        assert_eq!(crate::view::attr_list("(\"a\", \"b\")"), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn cycles_finds_a_two_node_loop() {
        let found = cycles(&[("a", "b"), ("b", "a")]);
        assert!(!found.is_empty(), "a <-> b is a cycle");
    }

    /// A COVERAGE FLOOR per command (keelArchViewsDoD): each of the six views must run against the
    /// repo's own registry and produce output. Deliberately a floor rather than golden-output asserts —
    /// a golden file over 12 authored elements would fail on every honest registry edit and be deleted
    /// within a sprint, which is worse than a floor that survives.
    fn repo_root() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    #[test]
    fn every_arch_view_runs_against_the_repo_registry() {
        let root = repo_root();
        if !root.join(".tracking/architecture/code-registry.sysml").exists() {
            return; // consumed as a library elsewhere; nothing to floor against
        }
        for sub in ["elements", "criticality", "coupling", "drift", "stpa-inputs", "coverage"] {
            assert_eq!(cmd(&[sub.to_string()], &root), 0, "`arch {sub}` must succeed on the repo registry");
        }
    }

    #[test]
    fn the_registry_ranks_a_data_loss_element_first_and_carries_dependson_edges() {
        let root = repo_root();
        let Ok(m) = crate::view::arch_model(&root) else { return };
        if m.elements.is_empty() {
            return;
        }
        let o = risk_order(&root);
        let mut ranked: Vec<usize> = m.elements.iter().map(|e| risk_rank(&o, &e.risk_class)).collect();
        ranked.sort_unstable();
        assert_eq!(ranked[0], 0, "the registry must classify at least one dataLoss element");
        assert!(
            m.edges.iter().any(|(k, _, _)| k == "dependson"),
            "coupling needs #DependsOn edges, and the edge kind is LOWERCASED at ingest"
        );
    }

    #[test]
    fn a_wrong_root_is_an_error_not_an_empty_registry() {
        let bogus = repo_root().join("definitely-not-a-repo-root");
        assert!(matches!(load(&bogus), Err(2)), "a wrong path must not read as an empty registry");
    }

    #[test]
    fn a_projects_own_risk_taxonomy_wins_over_the_engines() {
        // issue128: ranking a downstream project against the ENGINE's RiskClass sorted its
        // `durability` elements below `cosmetic` and printed them as `unclassified`, in the one view
        // whose purpose is what-to-audit-first. A project's risk taxonomy is its own judgement.
        let engine = crate::schema::enum_members("RiskClass");
        assert!(!engine.contains(&"durability".to_string()), "the engine does not declare durability");
        let o = risk_order(&repo_root());
        assert_eq!(o.first().map(String::as_str), Some("dataLoss"), "this repo declares no override");
        // An unknown class must sort LAST, never first — absence of a judgement is not safety.
        assert!(risk_rank(&o, "not-a-declared-class") >= o.len());
    }
}
