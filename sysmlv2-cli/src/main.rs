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

fn cmd_guard(args: &[String]) -> i32 {
    let Some(name) = args.first() else {
        eprintln!("usage: sysmlv2 guard <actors|acceptance-events|sprint-coverage> [ROOT]");
        return 2;
    };
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 guard <name> [ROOT]");
                return 2;
            }
        }
    };
    let report = match name.as_str() {
        "actors" => sysmlv2_cli::guards::actors(&root),
        "acceptance-events" => sysmlv2_cli::guards::acceptance_events(&root),
        "sprint-coverage" => sysmlv2_cli::guards::sprint_coverage(&root),
        other => {
            eprintln!("unknown guard '{other}' (known: actors, acceptance-events, sprint-coverage)");
            return 2;
        }
    };
    report.print();
    i32::from(!report.ok())
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
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: sysmlv2 suspect [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", sysmlv2_cli::govern::suspect(&root));
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
            eprintln!("  append-result --file F --task T --sha S [--verdict pass|fail] [--judged-by A] [--judged-at D]");
            eprintln!("  append-gate-result --file F --gate G --sha S [--verdict pass|fail] [--judged-by A] [--judged-at D]");
            eprintln!("  add-task --file F --def D --task T --dod TEXT [--method test|inspect|confirmation|demo|analysis]");
            2
        }
    };
    process::exit(code);
}
