//! `sysmlv2` — CLI entry point.
//!
//! Subcommands:
//!   `validate [ROOT]`         — semantic-validate all `.tracking/` files
//!   `check FILE...`           — parse-check one or more `.sysml` files
//!   `orient [ROOT]`           — print orient state (cursor + ready/done/outstanding) as JSON
//!   `whats-next [ROOT]`       — print ready task names, one per line
//!   `append-result [FLAGS]`   — append a `TestResult` to a tracking file
//!   `append-gate-result [FLAGS]` — append a `TestResult` for a ceremony gate (`verification`)
//!   `add-task [FLAGS]`        — add a task + `DoD` verification to an action def
//!   `coverage [ROOT]`         — assurance-coverage view (D0079 C): Need/Requirement/Decision evidence
//!   `critique-coverage [ROOT]` — per-element x required-lens critique coverage (D0080)
//!   `assured [ROOT]`           — composite assurance-readiness verdict + blockers (D0079 c)
//!   `decisions [ROOT]`         — load-bearing decisions ranked by dependence + antiquation flags
//!   `diagram [ROOT]`           — comprehensive interactive traceability diagram (HTML; computed #View)
#![forbid(unsafe_code)]
#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]
// D0074 fail-loud: authority-bearing CLI code has no silent failure paths.
// (clippy::indexing_slicing deferred to M0b with the parser cleanup — see rustFailLoudLints.)
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::todo,
    clippy::unimplemented
)]

use std::{path::PathBuf, process};

use sysmlv2_cli::{check_files, collect_sysml, validate_root};
use sysmlv2_cli::orient;
use sysmlv2_cli::write as w;

// ── repo-root discovery ───────────────────────────────────────────────────────

fn find_repo_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join(".engine").is_dir() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

// ── subcommands ───────────────────────────────────────────────────────────────

fn cmd_validate(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => if let Some(r) = find_repo_root() { r } else {
            eprintln!("error: no .engine/ directory found from the current directory upward.");
            eprintln!("usage: sysmlv2 validate [ROOT]");
            return 2;
        },
    };

    let report = validate_root(&root);

    for (path, diag) in &report.diagnostics {
        println!("ERROR: {}:{} — {}", path.display(), diag.line, diag.message);
        if let Some(hint) = &diag.suggestion {
            println!("       hint: {hint}");
        }
    }
    for err in &report.errors {
        println!("FAIL:  {} — {}", err.file.display(), err.message);
    }

    if report.is_clean() {
        println!("{} tracking file(s) validated clean.", report.validated);
        0
    } else {
        eprintln!(
            "{} tracking file(s) validated — {} parse error(s), {} semantic diagnostic(s).",
            report.validated,
            report.errors.len(),
            report.diagnostics.len()
        );
        1
    }
}

fn cmd_spec_version(args: &[String]) -> i32 {
    use sysmlv2_parser::spec_compat as sc;
    println!("grammar version (baked): {}", sc::SYSML_V2_GRAMMAR_VERSION);
    println!("pinned sha:              {}", sc::SYSML_V2_GRAMMAR_SHA);
    println!("spec url:                {}", sc::SYSML_V2_SPEC_URL);
    if sc::is_offline() || args.iter().any(|a| a == "--no-fetch") {
        println!("live check:              skipped (offline)");
        return 0;
    }
    let fetched = std::process::Command::new("curl")
        .args(["-sSL", sc::SYSML_V2_SPEC_URL])
        .output();
    let Ok(out) = fetched else {
        println!("live check:              unavailable (curl not found)");
        return 0;
    };
    if !out.status.success() || out.stdout.is_empty() {
        println!("live check:              unavailable (no network)");
        return 0;
    }
    let live = sc::sha256_hex(&out.stdout);
    println!("live sha:                {live}");
    let pinned = sc::SYSML_V2_GRAMMAR_SHA;
    if pinned.bytes().all(|b| b == b'0') {
        println!("status:                  not pinned — baked version is the reference; pin the live sha to enable drift detection");
        0
    } else if live == pinned {
        println!("status:                  CURRENT");
        0
    } else {
        println!("status:                  STALE — upstream changed since the pin");
        1
    }
}

fn cmd_check(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--spec-version") {
        return cmd_spec_version(args);
    }
    if args.is_empty() {
        eprintln!("usage: sysmlv2 check FILE [FILE...]  |  sysmlv2 check --spec-version [--no-fetch]");
        return 2;
    }
    let files: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();
    let report = check_files(&files);

    for err in &report.errors {
        println!("ERROR: {} — {}", err.file.display(), err.message);
    }
    if report.is_clean() {
        println!("{} file(s) checked clean.", files.len());
        0
    } else {
        eprintln!(
            "{} file(s) checked — {} error(s).",
            files.len(),
            report.errors.len()
        );
        1
    }
}

fn cmd_orient(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ directory found from the current directory upward.");
                eprintln!("usage: sysmlv2 orient [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", orient::compute(&root).to_json());
    0
}

fn cmd_attestation_coverage(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 attestation-coverage [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::attestation_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("attestation-coverage error: {e}");
            1
        }
    }
}

fn cmd_orphans(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 orphans [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::algo::orphans(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("orphans error: {e}");
            1
        }
    }
}

fn cmd_audit(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 audit [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::algo::audit(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("audit error: {e}");
            1
        }
    }
}

fn resolve_guard_root(arg: Option<&String>) -> Option<PathBuf> {
    arg.map_or_else(find_repo_root, |p| Some(PathBuf::from(p)))
}

fn cmd_guard(args: &[String]) -> i32 {
    // `sysmlv2 guard` / `guard all [ROOT]` → run all six; `guard <name> [ROOT]` → run one.
    let run_all = args.first().is_none_or(|a| a == "all");
    let Some(root) = resolve_guard_root(args.get(1)) else {
        eprintln!("error: no .engine/ directory found. usage: sysmlv2 guard [<name>] [ROOT]");
        return 2;
    };
    if run_all {
        let reports = sysmlv2_cli::guards::run_all(&root);
        let mut all_ok = true;
        for r in &reports {
            r.print();
            all_ok &= r.ok();
        }
        println!("[guard] {}", if all_ok { "ALL PASS" } else { "FAILED" });
        return i32::from(!all_ok);
    }
    let Some(name) = args.first() else { return 2 };
    let Some(report) = sysmlv2_cli::guards::run_one(name, &root) else {
        eprintln!("unknown guard '{name}' (known: {})", sysmlv2_cli::guards::GUARD_NAMES.join(", "));
        return 2;
    };
    report.print();
    i32::from(!report.ok())
}

// Root-only query: `sysmlv2 <name> [ROOT]`.
fn cmd_query0(args: &[String], usage: &str, f: fn(&std::path::Path) -> String) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 {usage} [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", f(&root));
    0
}

// Name + optional root: `sysmlv2 <name> <arg> [ROOT]`.
fn cmd_query1(args: &[String], usage: &str, f: fn(&std::path::Path, &str) -> String) -> i32 {
    let Some(arg) = args.first() else {
        eprintln!("usage: sysmlv2 {usage} <name> [ROOT]");
        return 2;
    };
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 {usage} <name> [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", f(&root, arg));
    0
}

fn cmd_open_issues(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 open-issues [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::open_issues(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("open-issues error: {e}");
            1
        }
    }
}

fn cmd_coverage(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 coverage [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("coverage error: {e}");
            1
        }
    }
}

fn cmd_diagram(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 diagram [ROOT]  (redirect to a .html file)");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::diagram_html(&root) {
        Ok(html) => {
            println!("{html}");
            0
        }
        Err(e) => {
            eprintln!("diagram error: {e}");
            1
        }
    }
}

fn cmd_decisions(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 decisions [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::decisions_report(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("decisions error: {e}");
            1
        }
    }
}

fn cmd_assured(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 assured [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::assured(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("assured error: {e}");
            1
        }
    }
}

fn cmd_critique_coverage(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 critique-coverage [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::critique_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("critique-coverage error: {e}");
            1
        }
    }
}

fn cmd_governing_version(args: &[String]) -> i32 {
    let Some(item) = args.first() else {
        eprintln!("usage: sysmlv2 governing-version <delivery Story name> [ROOT]");
        return 2;
    };
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 governing-version <delivery Story name> [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", sysmlv2_cli::govern::governing_version(&root, item));
    0
}

fn cmd_reprocess_candidates(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 reprocess-candidates [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", sysmlv2_cli::govern::reprocess_candidates(&root));
    0
}

fn cmd_suspect(args: &[String]) -> i32 {
    let explain = args.iter().any(|a| a == "--explain");
    let root = match args.iter().find(|a| !a.starts_with("--")) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 suspect [--explain] [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", sysmlv2_cli::govern::suspect(&root, explain));
    0
}

fn cmd_view(args: &[String]) -> i32 {
    let Some(name) = args.first() else {
        eprintln!("usage: sysmlv2 view <name> [ROOT]");
        return 2;
    };
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ directory found from the current directory upward.");
                eprintln!("usage: sysmlv2 view <name> [ROOT]");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::run(&root, name) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("view error: {e}");
            1
        }
    }
}

fn cmd_whats_next(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ directory found from the current directory upward.");
                eprintln!("usage: sysmlv2 whats-next [ROOT]");
                return 2;
            }
        }
    };
    for task in orient::compute(&root).ready {
        println!("{task}");
    }
    0
}

fn cmd_ls(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => if let Some(r) = find_repo_root() { r } else {
            eprintln!("error: no .engine/ directory found.");
            return 2;
        },
    };
    let dir = root.join(".tracking");
    for p in collect_sysml(&dir) {
        println!("{}", p.display());
    }
    0
}

/// Parse simple `--key value` flag pairs from a flat args slice.
fn flag(args: &[String], name: &str) -> Option<String> {
    let key = format!("--{name}");
    args.windows(2).find_map(|w| match w {
        [k, v] if *k == key => Some(v.clone()),
        _ => None,
    })
}

fn cmd_append_result(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: sysmlv2 append-result --file FILE --task TASK --sha SHA [--verdict pass|fail] [--judged-by ACTOR] [--judged-at DATE]");
        return 2;
    };
    let Some(task) = flag(args, "task") else {
        eprintln!("error: --task required");
        return 2;
    };
    let Some(sha) = flag(args, "sha") else {
        eprintln!("error: --sha required");
        return 2;
    };
    let file = PathBuf::from(file_str);
    let verdict = flag(args, "verdict").unwrap_or_else(|| "pass".to_owned());
    let judged_by = flag(args, "judged-by").unwrap_or_else(|| "sysmlv2-cli".to_owned());
    // Callers should pass --judged-at for determinism; this is a safe fallback.
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());

    match w::append_result(&file, &task, &sha, &verdict, &judged_at, &judged_by) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

fn cmd_append_gate_result(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: sysmlv2 append-gate-result --file FILE --gate GATE --sha SHA [--verdict pass|fail] [--judged-by ACTOR] [--judged-at DATE]");
        return 2;
    };
    let Some(gate) = flag(args, "gate") else {
        eprintln!("error: --gate required");
        return 2;
    };
    let Some(sha) = flag(args, "sha") else {
        eprintln!("error: --sha required");
        return 2;
    };
    let file = PathBuf::from(file_str);
    let verdict = flag(args, "verdict").unwrap_or_else(|| "pass".to_owned());
    let judged_by = flag(args, "judged-by").unwrap_or_else(|| "sysmlv2-cli".to_owned());
    // Callers should pass --judged-at for determinism; this is a safe fallback.
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());

    match w::append_gate_result(&file, &gate, &sha, &verdict, &judged_at, &judged_by) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

fn cmd_add_task(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: sysmlv2 add-task --file FILE --def DEF --task TASK --dod TEXT --method METHOD");
        return 2;
    };
    let Some(def_name) = flag(args, "def") else {
        eprintln!("error: --def required");
        return 2;
    };
    let Some(task) = flag(args, "task") else {
        eprintln!("error: --task required");
        return 2;
    };
    let Some(dod) = flag(args, "dod") else {
        eprintln!("error: --dod required");
        return 2;
    };
    let file = PathBuf::from(file_str);
    let method = flag(args, "method").unwrap_or_else(|| "test".to_owned());

    match w::add_task(&file, &def_name, &task, &dod, &method) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

/// `render <view> [--mode graph|table|review] [--root ROOT]` — modular interactive-artifact
/// renderer over the view layer (D0086). Emits self-contained HTML to stdout (redirect to a file).
fn cmd_render(args: &[String]) -> i32 {
    let Some(view) = args.first().filter(|v| !v.starts_with("--")) else {
        eprintln!("usage: sysmlv2 render <view> [--mode graph|table|review] [--root ROOT]");
        eprintln!("  <view> = a declared view name (e.g. decisions, issues), or 'model' for the whole-model graph");
        return 2;
    };
    let mode = flag(args, "mode").unwrap_or_else(|| "graph".to_owned());
    let root = match flag(args, "root") {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ found from cwd upward; pass --root ROOT");
                return 2;
            }
        }
    };
    match sysmlv2_cli::view::render_html(&root, view, &mode) {
        Ok(html) => {
            println!("{html}");
            0
        }
        Err(e) => {
            eprintln!("render error: {e}");
            1
        }
    }
}

#[derive(serde::Deserialize)]
struct ReviewBatch {
    #[serde(default)]
    dispositions: Vec<ReviewDisp>,
    #[serde(default, rename = "judgedBy")]
    judged_by: String,
    #[serde(default, rename = "judgedAgainst")]
    judged_against: String,
}

#[derive(serde::Deserialize)]
struct ReviewDisp {
    element: String,
    verdict: String,
    #[serde(default)]
    lens: String,
    #[serde(default)]
    severity: String,
    #[serde(default)]
    rationale: String,
    #[serde(default)]
    actionable: bool,
}

/// `apply-review --batch FILE [--sha SHA] [--judged-by ACTOR] [--judged-at DATE] [--root ROOT]` —
/// ingest a review batch exported by `render --mode review` and write each disposition back as a new
/// linked critique (D0086) via the write path. `accept`->pass, `finding`/`reject`->fail (a finding,
/// which induces computed suspicion). Writes into `.tracking/critiques.sysml`.
fn cmd_apply_review(args: &[String]) -> i32 {
    let Some(batch_str) = flag(args, "batch") else {
        eprintln!("usage: sysmlv2 apply-review --batch FILE [--sha SHA] [--judged-by ACTOR] [--judged-at DATE] [--root ROOT]");
        return 2;
    };
    let root = match flag(args, "root") {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ found from cwd upward; pass --root ROOT");
                return 2;
            }
        }
    };
    let text = match std::fs::read_to_string(&batch_str) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error reading batch {batch_str}: {e}");
            return 2;
        }
    };
    let batch: ReviewBatch = match serde_json::from_str(&text) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: invalid review batch JSON: {e}");
            return 2;
        }
    };
    let judged_by = flag(args, "judged-by").filter(|s| !s.is_empty()).or_else(|| Some(batch.judged_by.clone()).filter(|s| !s.is_empty())).unwrap_or_else(|| "human".to_owned());
    let sha = flag(args, "sha").filter(|s| !s.is_empty()).or_else(|| Some(batch.judged_against.clone()).filter(|s| !s.is_empty())).unwrap_or_else(|| "uncommitted".to_owned());
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());
    let critiques = root.join(".tracking").join("critiques.sysml");

    let mut count = 0u32;
    for d in &batch.dispositions {
        let outcome = match d.verdict.as_str() {
            "accept" => "pass",
            "finding" | "reject" => "fail",
            other => {
                eprintln!("skip {}: unknown verdict '{other}'", d.element);
                continue;
            }
        };
        let severity = (outcome == "fail" && !d.severity.is_empty()).then_some(d.severity.as_str());
        let lens = if d.lens.is_empty() { "correctness" } else { d.lens.as_str() };
        let mut rationale = d.rationale.clone();
        if d.actionable {
            rationale.push_str(" [actionable: warrants new implementation]");
        }
        let c = w::Critique {
            element: &d.element,
            method: "critique",
            lens,
            critiqued_by: "human",
            severity,
            rationale: &rationale,
            outcome,
            sha: &sha,
            judged_at: &judged_at,
            judged_by: &judged_by,
        };
        match w::append_critique(&critiques, &c) {
            Ok(name) => {
                println!("{name}  ({} {})", d.element, outcome);
                count += 1;
            }
            Err(e) => {
                eprintln!("error on {}: {e}", d.element);
                return 1;
            }
        }
    }
    println!("applied {count} disposition(s) to {}", critiques.display());
    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rest: &[String] = args.get(2..).unwrap_or(&[]);
    let code = match args.get(1).map(String::as_str) {
        Some("validate") => cmd_validate(rest),
        Some("check") => cmd_check(rest),
        Some("ls") => cmd_ls(rest),
        Some("orient") => cmd_orient(rest),
        Some("whats-next") => cmd_whats_next(rest),
        Some("view") => cmd_view(rest),
        Some("attestation-coverage") => cmd_attestation_coverage(rest),
        Some("orphans") => cmd_orphans(rest),
        Some("audit") => cmd_audit(rest),
        Some("guard") => cmd_guard(rest),
        Some("governing-version") => cmd_governing_version(rest),
        Some("reprocess-candidates") => cmd_reprocess_candidates(rest),
        Some("suspect") => cmd_suspect(rest),
        Some("open-issues") => cmd_open_issues(rest),
        Some("coverage") => cmd_coverage(rest),
        Some("critique-coverage") => cmd_critique_coverage(rest),
        Some("assured") => cmd_assured(rest),
        Some("decisions") => cmd_decisions(rest),
        Some("diagram") => cmd_diagram(rest),
        Some("render") => cmd_render(rest),
        Some("apply-review") => cmd_apply_review(rest),
        Some("outstanding") => cmd_query0(rest, "outstanding", sysmlv2_cli::queries::outstanding),
        Some("workflows") => cmd_query0(rest, "workflows", sysmlv2_cli::queries::workflows),
        Some("item") => cmd_query1(rest, "item", sysmlv2_cli::queries::item),
        Some("trace") => cmd_query1(rest, "trace", sysmlv2_cli::queries::trace),
        Some("trace-need") => cmd_query1(rest, "trace-need", sysmlv2_cli::queries::trace_need),
        Some("append-result") => cmd_append_result(rest),
        Some("append-gate-result") => cmd_append_gate_result(rest),
        Some("add-task") => cmd_add_task(rest),
        _ => {
            eprintln!("sysmlv2 <subcommand> [args]");
            eprintln!("  validate [ROOT]              semantic-validate all .tracking/ files");
            eprintln!("  check FILE...                parse-check one or more .sysml files");
        eprintln!("  check --spec-version         report the baked grammar version vs upstream (--no-fetch to skip the live check)");
            eprintln!("  ls [ROOT]                    list .tracking/ .sysml files");
            eprintln!("  orient [ROOT]                print orient state as JSON");
            eprintln!("  whats-next [ROOT]            print ready task names (one per line)");
            eprintln!("  diagram [ROOT]               whole-model interactive graph HTML (D0085; redirect to .html)");
            eprintln!("  render <view> [--mode graph|table|review]  render any declared view as HTML (D0086)");
            eprintln!("  apply-review --batch F [--sha S] [--judged-by A] [--judged-at D]  write a review batch back as linked critiques (D0086)");
            eprintln!("  append-result --file F --task T --sha S [--verdict pass|fail] [--judged-by A] [--judged-at D]");
            eprintln!("  append-gate-result --file F --gate G --sha S [--verdict pass|fail] [--judged-by A] [--judged-at D]");
            eprintln!("  add-task --file F --def D --task T --dod TEXT [--method test|inspect|confirmation|demo|analysis]");
            2
        }
    };
    process::exit(code);
}
