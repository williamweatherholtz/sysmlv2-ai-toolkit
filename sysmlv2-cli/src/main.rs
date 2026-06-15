//! `sysmlv2` — CLI entry point.
//!
//! Subcommands:
//!   `validate [ROOT]`   — semantic-validate all `.tracking/` files
//!   `check FILE...`     — parse-check one or more `.sysml` files
//!   `orient [ROOT]`     — print orient state (cursor + ready/done/outstanding) as JSON
#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]

use std::{path::PathBuf, process};

use sysmlv2_cli::{check_files, collect_sysml, orient_root, validate_root};

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

fn cmd_check(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("usage: sysmlv2 check FILE [FILE...]");
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
    println!("{}", orient_root(&root).to_json());
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("validate") => cmd_validate(&args[2..]),
        Some("check") => cmd_check(&args[2..]),
        Some("ls") => cmd_ls(&args[2..]),
        Some("orient") => cmd_orient(&args[2..]),
        _ => {
            eprintln!("sysmlv2 <subcommand> [args]");
            eprintln!("  validate [ROOT]    semantic-validate all .tracking/ files");
            eprintln!("  check FILE...      parse-check one or more .sysml files");
            eprintln!("  ls [ROOT]          list .tracking/ .sysml files");
            eprintln!("  orient [ROOT]      print orient state as JSON");
            2
        }
    };
    process::exit(code);
}
