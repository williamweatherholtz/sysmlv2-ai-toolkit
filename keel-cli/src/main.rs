//! `keel` — CLI entry point.
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
//!   `critique-policy [ROOT]`   — the active declared critique policy: required lenses per type (D0097)
//!   `concern-coverage [ROOT]` — which declared viewpoint concerns are served vs planned (D0057)
//!   `dispositions [ROOT]`     — >= Medium findings + their typed disposition verdict (D0092)
//!   `sitting-coverage [ROOT]` — which delivery sprints are covered by a per-sitting review (D0049)
//!   `assured [ROOT]`           — composite assurance-readiness verdict + blockers (D0079 c)
//!   `decisions [ROOT]`         — load-bearing decisions ranked by dependence + antiquation flags
//!   `diagram [ROOT]`           — comprehensive interactive traceability diagram (HTML; computed #View)
//!   `init DIR`                 — scaffold the engine into a new project (D0093 cold start)
//!   `serve [--port N] [ROOT]`  — the interactive console: localhost read dashboard (D0094 m1)
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

use std::{path::{Path, PathBuf}, process};

use include_dir::{include_dir, Dir};
use keel_cli::{check_files, collect_sysml, validate_root};
use keel_cli::orient;
use keel_cli::write as w;

// ── engine scaffold payload (D0093 `init`): the reusable engine tree + operating manual, embedded at
//    compile time so `keel init` is self-contained (no external fetch — the cytoscape precedent). ──
static ENGINE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../.engine");
// A DOWNSTREAM CLAUDE.md template (issue057): a fresh project is TRACKED BY keel, not keel itself.
// The self-build repo's own CLAUDE.md (about building the engine) is NEVER shipped to init'd projects.
const CLAUDE_MD: &str = include_str!("../assets/claude-md-template.md");
const TRACKING_STARTER: &str = "# .tracking/ — your project's instance data\n\nThis directory holds THIS project's authored facts (needs, requirements, work items, issues,\ndecisions, test results) — the per-project INSTANCE. The reusable engine lives in `.engine/`.\n\nGetting started: run the `introduction` skill (guided onboarding), or author your first `Need`\nfollowing `.engine/docs/tracking-template.sysml`. State is COMPUTED — run `keel orient .` to\nsee where things stand. The engine's design rationale is read-only in `.engine/reference/decisions/`;\nyour project authors its OWN decisions fresh in `.engine/decisions/`.\n";
/// A fresh project's deliverable-suspicion manifest is EMPTY — the shipped one lists the ENGINE's own
/// deliverable tasks (instance-specific), which would fail manifest-coverage on a new project (D0093
/// engine/instance boundary). The new project adds entries as it builds source-dependent verifications.
const STARTER_MANIFEST: &str = "# deliverable-manifest.txt — declares which verification tasks depend on which DELIVERABLE SOURCE\n# files (D0050), so `keel suspect` flags a task suspect when its source changed since it was\n# verified. One entry per line:  task: <taskName> | <relpath> <relpath> ...\n# Empty for a new project — add an entry when you have a deliverable-source-dependent verification.\n";
/// A starter actor registry scaffolded into a fresh project (`.tracking/actors.sysml`). Without it
/// the newcomer's FIRST recorded fact (any `createdBy`/`judgedBy`) fails the actors guard (D0037) —
/// there'd be no `ProjectActors` to reference. Ships placeholder actors (a human + the AI) the newcomer
/// edits to their real identities; the declared part name is the id that `createdBy`/`judgedBy` reference.
const STARTER_ACTORS: &str = "// ProjectActors — this project's actor registry (INSTANCE data). EDIT to your real actors.\n// The declared part name is the id that createdBy/judgedBy reference (enforced by `keel guard actors`).\npackage ProjectActors {\n    private import EngineElement::*;\n\n    part you : Person { :>> name = \"Your Name\"; :>> email = \"you@example.com\"; }\n    part ai : Actor { :>> name = \"AI assistant\"; :>> kind = ActorKind::ai; }\n}\n";
/// A RUST-ONLY pre-commit gate scaffolded into a fresh project (`.githooks/pre-commit`). Runs
/// `keel validate` + `keel guard` — NO conda/JVM kernel (D0048: the Rust path is the authority).
/// Enabled by the user with `git config core.hooksPath .githooks` (printed in the init Next steps).
/// Degrades to a skip (never blocks) if `keel` isn't on PATH. POSIX sh.
const PRECOMMIT_HOOK: &str = "#!/bin/sh\n# keel pre-commit gate (Rust-only; no JVM kernel) — scaffolded by `keel init` (D0048/D0093).\n# Enable: git config core.hooksPath .githooks   |   bypass once: SKIP_KEEL=1 git commit ...\n[ \"$SKIP_KEEL\" = \"1\" ] && { echo 'pre-commit: SKIP_KEEL=1 — keel gate skipped'; exit 0; }\nKEEL=\"${KEEL:-keel}\"\ncommand -v \"$KEEL\" >/dev/null 2>&1 || { echo \"pre-commit: '$KEEL' not on PATH — keel gate skipped (install keel to enforce)\"; exit 0; }\necho 'pre-commit: keel validate .'\n\"$KEEL\" validate . || { echo 'pre-commit: keel validate FAILED — commit aborted'; exit 1; }\necho 'pre-commit: keel guard'\n\"$KEEL\" guard || { echo 'pre-commit: keel guard FAILED — commit aborted'; exit 1; }\n";

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
            eprintln!("usage: keel validate [ROOT]");
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
    use keel_parser::spec_compat as sc;
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
        eprintln!("usage: keel check FILE [FILE...]  |  keel check --spec-version [--no-fetch]");
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

fn cmd_serve(args: &[String]) -> i32 {
    let mut port: u16 = 7777;
    let mut root_arg: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--port" {
            if let Some(v) = it.next() {
                if let Ok(p) = v.parse::<u16>() {
                    port = p;
                }
            }
        } else if !a.starts_with("--") {
            root_arg = Some(a.clone());
        }
    }
    let root = match root_arg {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel serve [--port N] [ROOT]");
                return 2;
            }
        }
    };
    keel_cli::serve::run(root, port)
}

fn cmd_orient(args: &[String]) -> i32 {
    let html = args.iter().any(|a| a == "--html");
    let root = match args.iter().find(|a| !a.starts_with("--")) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ directory found from the current directory upward.");
                eprintln!("usage: keel orient [ROOT] [--html]");
                return 2;
            }
        }
    };
    if html {
        return match keel_cli::view::orient_html(&root) {
            Ok(h) => {
                println!("{h}");
                0
            }
            Err(e) => {
                eprintln!("orient --html error: {e}");
                1
            }
        };
    }
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
                eprintln!("usage: keel attestation-coverage [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::attestation_coverage(&root) {
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
                eprintln!("usage: keel orphans [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::algo::orphans(&root) {
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
                eprintln!("usage: keel audit [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::algo::audit(&root) {
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

/// A string that names a runnable guard (an enforced one, or a runnable-only diagnostic).
fn is_guard_name(s: &str) -> bool {
    keel_cli::guards::GUARD_NAMES.contains(&s) || matches!(s, "assured" | "critique" | "critique-rigor" | "defect-guard-coverage")
}

/// Classify `keel guard` args into `(guard name to run, root arg)`. A first arg that is a known guard
/// name runs THAT guard; `all`, no arg, or a non-name first arg (a ROOT path like `.` or a dir) runs
/// ALL guards on that root. This is what lets `keel guard <ROOT>` work like `keel validate <ROOT>`.
fn classify_guard_args(args: &[String]) -> (Option<&str>, Option<&str>) {
    match args.first().map(String::as_str) {
        None => (None, None),
        Some("all") => (None, args.get(1).map(String::as_str)),
        Some(a) if is_guard_name(a) => (Some(a), args.get(1).map(String::as_str)),
        Some(a) => (None, Some(a)), // a bare ROOT, not a guard name
    }
}

fn cmd_guard(args: &[String]) -> i32 {
    // `keel guard` / `guard [ROOT]` / `guard all [ROOT]` → run all; `guard <name> [ROOT]` → run one.
    let (name, root_arg) = classify_guard_args(args);
    let Some(root) = resolve_guard_root(root_arg.map(String::from).as_ref()) else {
        eprintln!("error: no .engine/ directory found. usage: keel guard [<name>] [ROOT]");
        return 2;
    };
    let Some(name) = name else {
        let reports = keel_cli::guards::run_all(&root);
        let mut all_ok = true;
        for r in &reports {
            r.print();
            all_ok &= r.ok();
        }
        println!("[guard] {}", if all_ok { "ALL PASS" } else { "FAILED" });
        return i32::from(!all_ok);
    };
    let Some(report) = keel_cli::guards::run_one(name, &root) else {
        eprintln!(
            "unknown guard '{name}' (enforced: {} | runnable diagnostics: assured, critique, critique-rigor, defect-guard-coverage)",
            keel_cli::guards::GUARD_NAMES.join(", ")
        );
        return 2;
    };
    report.print();
    i32::from(!report.ok())
}

// Root-only query: `keel <name> [ROOT]`.
fn cmd_query0(args: &[String], usage: &str, f: fn(&std::path::Path) -> String) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel {usage} [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", f(&root));
    0
}

// Name + optional root: `keel <name> <arg> [ROOT]`.
fn cmd_query1(args: &[String], usage: &str, f: fn(&std::path::Path, &str) -> String) -> i32 {
    let Some(arg) = args.first() else {
        eprintln!("usage: keel {usage} <name> [ROOT]");
        return 2;
    };
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel {usage} <name> [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", f(&root, arg));
    0
}

/// `keel reverify [--all-drift | --task NAME] [--by ACTOR] [ROOT]` (D0101) — re-run the configured gate
/// at HEAD and stamp a fresh `TestResult` on each drift-suspect task on green.
fn cmd_reverify(args: &[String]) -> i32 {
    let mut task: Option<String> = None;
    let mut by = "claudeOpus".to_string();
    let mut root: Option<PathBuf> = None;
    let mut i = 0;
    while let Some(a) = args.get(i) {
        match a.as_str() {
            "--all-drift" => {}
            "--task" => {
                i += 1;
                task = args.get(i).cloned();
            }
            "--by" => {
                i += 1;
                if let Some(b) = args.get(i) {
                    by.clone_from(b);
                }
            }
            other => root = Some(PathBuf::from(other)),
        }
        i += 1;
    }
    let root = root.or_else(find_repo_root).unwrap_or_else(|| PathBuf::from("."));
    keel_cli::reverify::run(&root, task.as_deref(), &by)
}

fn cmd_open_issues(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel open-issues [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::open_issues(&root) {
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

fn cmd_dispositions(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel dispositions [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::dispositions(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("dispositions error: {e}");
            1
        }
    }
}

fn cmd_sitting_coverage(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel sitting-coverage [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::sitting_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("sitting-coverage error: {e}");
            1
        }
    }
}

fn cmd_concern_coverage(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel concern-coverage [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::concern_coverage(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("concern-coverage error: {e}");
            1
        }
    }
}

// `keel rules [ROOT]` (D0105 EXPAND step 2): evaluate the DECLARED rules (`keel check` is taken by the
// spec-compat file checker; the D0105 name reconciliation is a tracked follow-up). Runs ALONGSIDE
// `keel guard` until parity retires each guard (guardsToRulesMigration).
fn cmd_rules(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel rules [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::check(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("rules error: {e}");
            1
        }
    }
}

// `keel launchables [ROOT]` (srServeModelDrivenRegistry, Tier 1a): the model-declared launchable set.
fn cmd_launchables(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel launchables [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::launchables(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("launchables error: {e}");
            1
        }
    }
}

// `keel business [ROOT]` (serveBusinessNeedsView): the Business layer (Brief/Personas/Needs/UseCases).
fn cmd_business(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel business [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::business(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("business error: {e}");
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
                eprintln!("usage: keel coverage [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::coverage(&root) {
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
                eprintln!("usage: keel diagram [ROOT]  (redirect to a .html file)");
                return 2;
            }
        }
    };
    match keel_cli::view::diagram_html(&root) {
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
                eprintln!("usage: keel decisions [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::decisions_report(&root) {
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
                eprintln!("usage: keel assured [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::assured(&root) {
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
                eprintln!("usage: keel critique-coverage [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::critique_coverage(&root) {
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

fn cmd_critique_policy(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel critique-policy [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::critique_policy(&root) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("critique-policy error: {e}");
            1
        }
    }
}

fn cmd_governing_version(args: &[String]) -> i32 {
    let Some(item) = args.first() else {
        eprintln!("usage: keel governing-version <delivery Story name> [ROOT]");
        return 2;
    };
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel governing-version <delivery Story name> [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", keel_cli::govern::governing_version(&root, item));
    0
}

fn cmd_reprocess_candidates(args: &[String]) -> i32 {
    let root = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("usage: keel reprocess-candidates [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", keel_cli::govern::reprocess_candidates(&root));
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
                eprintln!("usage: keel suspect [--explain] [ROOT]");
                return 2;
            }
        }
    };
    println!("{}", keel_cli::govern::suspect(&root, explain));
    0
}

fn cmd_view(args: &[String]) -> i32 {
    let Some(name) = args.first() else {
        eprintln!("usage: keel view <name> [ROOT]");
        return 2;
    };
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(r) = find_repo_root() {
                r
            } else {
                eprintln!("error: no .engine/ directory found from the current directory upward.");
                eprintln!("usage: keel view <name> [ROOT]");
                return 2;
            }
        }
    };
    match keel_cli::view::run(&root, name) {
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
                eprintln!("usage: keel whats-next [ROOT]");
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
        eprintln!("usage: keel append-result --file FILE --task TASK --sha SHA [--verdict pass|fail] [--judged-by ACTOR] [--judged-at DATE]");
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
    let judged_by = flag(args, "judged-by").unwrap_or_else(|| "keel-cli".to_owned());
    // Callers should pass --judged-at for determinism; this is a safe fallback.
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());

    match w::append_result(&file, &task, &sha, &verdict, &judged_at, &judged_by) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

fn cmd_append_gate_result(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: keel append-gate-result --file FILE --gate GATE --sha SHA [--verdict pass|fail] [--judged-by ACTOR] [--judged-at DATE]");
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
    let judged_by = flag(args, "judged-by").unwrap_or_else(|| "keel-cli".to_owned());
    // Callers should pass --judged-at for determinism; this is a safe fallback.
    let judged_at = flag(args, "judged-at").unwrap_or_else(|| "2026-01-01".to_owned());

    match w::append_gate_result(&file, &gate, &sha, &verdict, &judged_at, &judged_by) {
        Ok(uuid) => { println!("{uuid}"); 0 }
        Err(e) => { eprintln!("error: {e}"); 1 }
    }
}

/// `keel record <type> ...` — the closed RMWX `record` verb (D0105/D0106; issue054 C1). Currently
/// records a Decision: `keel record decision --slug S --title T --context C --decision D --rationale R
/// --consequences Q --date YYYY-MM-DD --author A [--root ROOT]` → writes a proposed Decision file
/// (auto NNNN + UUID), killing point-of-decision friction (D0054). Acceptance stays a separate human gate.
fn cmd_record(args: &[String]) -> i32 {
    if args.first().map(String::as_str) != Some("decision") {
        eprintln!("usage: keel record decision --slug S --title T --context C --decision D --rationale R --consequences Q --date YYYY-MM-DD --author A [--root ROOT]");
        return 2;
    }
    let root = flag(args, "root").map_or_else(
        || find_repo_root().unwrap_or_else(|| PathBuf::from(".")),
        PathBuf::from,
    );
    let req = |name: &str| flag(args, name);
    let (Some(slug), Some(title), Some(context), Some(decision), Some(rationale), Some(consequences)) =
        (req("slug"), req("title"), req("context"), req("decision"), req("rationale"), req("consequences"))
    else {
        eprintln!("error: --slug --title --context --decision --rationale --consequences are all required (a substantive why — D0103)");
        return 2;
    };
    let date = flag(args, "date").unwrap_or_default();
    let author = flag(args, "author").unwrap_or_else(|| "wweatherholtz".to_owned());
    if date.is_empty() {
        eprintln!("error: --date YYYY-MM-DD required (the attestation time is its own irreducible fact)");
        return 2;
    }
    match w::record_decision(&root, &slug, &title, &date, &author, &context, &decision, &rationale, &consequences) {
        Ok((nnnn, path)) => {
            println!("recorded D{nnnn} (proposed) -> {path}");
            println!("accept later via an explicit human sign-off (flip status + add the d{nnnn}Accept event).");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn cmd_add_task(args: &[String]) -> i32 {
    let Some(file_str) = flag(args, "file") else {
        eprintln!("usage: keel add-task --file FILE --def DEF --task TASK --dod TEXT --method METHOD");
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
        eprintln!("usage: keel render <view> [--mode graph|table|review] [--root ROOT]");
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
    match keel_cli::view::render_html(&root, view, &mode) {
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

/// `report <name> [--html] [--root ROOT]` — computed aggregate scorecard (D0087): assurance |
/// traceability | quality-debt | flow. JSON by default; `--html` emits a human-digestible scorecard.
fn cmd_report(args: &[String]) -> i32 {
    let Some(name) = args.first().filter(|v| !v.starts_with("--")) else {
        eprintln!("usage: keel report <assurance|traceability|quality-debt|flow|governance> [--html] [--trend] [--root ROOT]");
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
    let html = args.iter().any(|a| a == "--html");
    let trend = args.iter().any(|a| a == "--trend");
    let result = if html { keel_cli::view::report_html(&root, name, trend) } else { keel_cli::view::report(&root, name, trend) };
    match result {
        Ok(out) => {
            println!("{out}");
            0
        }
        Err(e) => {
            eprintln!("report error: {e}");
            1
        }
    }
}

/// `indicators [--trend] [--root ROOT]` — monitored measures (D0089) with direction-aware status.
/// Computed indicators show current value (full series with `--trend`); pulled/manual show their
/// recorded-Measurement series + status.
fn cmd_indicators(args: &[String]) -> i32 {
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
    let trend = args.iter().any(|a| a == "--trend");
    match keel_cli::view::indicators(&root, trend) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("indicators error: {e}");
            1
        }
    }
}

/// `record-measurement --indicator I --value V [--at DATE] [--source S] [--by ACTOR] [--file F]` —
/// record a Measurement datapoint (D0089) for a pulled/manual indicator (write path).
fn cmd_record_measurement(args: &[String]) -> i32 {
    let Some(indicator) = flag(args, "indicator") else {
        eprintln!("usage: keel record-measurement --indicator I --value V [--at DATE] [--source S] [--by ACTOR] [--file F]");
        return 2;
    };
    let Some(value) = flag(args, "value") else {
        eprintln!("error: --value required");
        return 2;
    };
    let file = flag(args, "file").map_or_else(
        || find_repo_root().map_or_else(|| PathBuf::from(".tracking/indicators.sysml"), |r| r.join(".tracking").join("indicators.sysml")),
        PathBuf::from,
    );
    let at = flag(args, "at").unwrap_or_else(|| "2026-01-01".to_owned());
    let source = flag(args, "source").unwrap_or_default();
    let by = flag(args, "by").unwrap_or_else(|| "keel-cli".to_owned());
    match w::append_measurement(&file, &indicator, &value, &at, &source, &by) {
        Ok(name) => {
            println!("{name}");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `snapshot-indicators [--at DATE] [--by ACTOR] [--file F] [--root ROOT]` — take a reading of every
/// COMPUTED indicator (its current `metric_value`) and bank it as a `Measurement` (D0091). Run per
/// sprint/quarter to build a durable, fast series alongside the pulled/manual observations.
fn cmd_snapshot_indicators(args: &[String]) -> i32 {
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
    let file = flag(args, "file").map_or_else(|| root.join(".tracking").join("indicators.sysml"), PathBuf::from);
    let at = flag(args, "at").unwrap_or_else(|| "2026-01-01".to_owned());
    let by = flag(args, "by").unwrap_or_else(|| "keel-cli".to_owned());
    let keys = match keel_cli::view::computed_indicator_keys(&root) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let mut count = 0u32;
    for (indicator, key) in &keys {
        let Some(v) = keel_cli::view::metric_value(&root, key) else {
            eprintln!("skip {indicator}: metric '{key}' not computable");
            continue;
        };
        match w::append_measurement(&file, indicator, &format!("{v:.6}"), &at, "snapshot (computed reading)", &by) {
            Ok(name) => {
                println!("{name}  ({indicator} = {v:.2})");
                count += 1;
            }
            Err(e) => {
                eprintln!("error on {indicator}: {e}");
                return 1;
            }
        }
    }
    println!("banked {count} computed-indicator snapshot(s) @ {at} into {}", file.display());
    0
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
        eprintln!("usage: keel apply-review --batch FILE [--sha SHA] [--judged-by ACTOR] [--judged-at DATE] [--root ROOT]");
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
        // Finding disposition (D0092): act / accept-risk / dismiss target a finding ISSUE, written as a
        // method=confirmation disposition (#Dispositions-linked), not a critique.
        if let Some(verdict) = match d.verdict.as_str() {
            "act" => Some("act"),
            "accept-risk" | "acceptRisk" => Some("acceptRisk"),
            "dismiss" => Some("dismiss"),
            _ => None,
        } {
            let disp = w::Disposition { finding: &d.element, verdict, rationale: &d.rationale, sha: &sha, judged_at: &judged_at, judged_by: &judged_by };
            match w::append_disposition(&critiques, &disp) {
                Ok(name) => {
                    println!("{name}  ({} disposition:{verdict})", d.element);
                    count += 1;
                }
                Err(e) => {
                    eprintln!("error on {}: {e}", d.element);
                    return 1;
                }
            }
            continue;
        }
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

/// Write one embedded engine file into `dst_engine`, remapping `decisions/*` -> `reference/decisions/*`
/// (read-only reference, NOT instance — the engine's architecture decisions must not enter the new
/// project's computed views, which scan `.engine/decisions`; D0093 engine/instance boundary).
/// Remap an embedded engine-relative path for scaffolding: `decisions/*` -> `reference/decisions/*`
/// (read-only reference, not instance — D0093 boundary); everything else is unchanged.
fn remap_engine_path(rel: &Path) -> PathBuf {
    rel.strip_prefix("decisions")
        .map_or_else(|_| rel.to_path_buf(), |rest| Path::new("reference").join("decisions").join(rest))
}

/// Engine-DEV-only embedded paths EXCLUDED from the `keel init` scaffold (D0093 boundary): the kernel/
/// Python toolchain (`.engine/tools/` — validators, spikes, probes, migrations) + any compiled-Python
/// cache. Downstream projects use the Rust path (`keel validate`/`guard`, D0048) and never need these;
/// shipping them would baffle a conda-less consumer. The reusable engine (schema/workflows/processes/
/// skills/decisions->reference/docs/views/contracts) still scaffolds.
fn is_engine_dev_only(rel: &Path) -> bool {
    rel.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == "tools" || s == "__pycache__"
    }) || rel.extension().is_some_and(|e| e == "pyc")
}

fn write_engine_file(f: &include_dir::File, dst_engine: &Path, count: &mut u32) -> std::io::Result<()> {
    let rel = f.path();
    if is_engine_dev_only(rel) {
        return Ok(()); // engine-dev-only (kernel/python toolchain) — not shipped to downstream projects
    }
    let dst = dst_engine.join(remap_engine_path(rel));
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // The deliverable-suspicion manifest is instance-specific (lists the ENGINE's own tasks) — reset it
    // to a starter so a fresh project passes manifest-coverage (D0093 engine/instance boundary).
    if rel == Path::new("deliverable-manifest.txt") {
        std::fs::write(&dst, STARTER_MANIFEST)?;
    } else {
        std::fs::write(&dst, f.contents())?;
    }
    *count += 1;
    Ok(())
}

/// Recursively scaffold the embedded engine tree into `dst_engine`. `include_dir`'s `File::path()` is
/// root-relative, so the remap in `write_engine_file` sees the full path regardless of nesting.
fn scaffold_engine(dir: &Dir, dst_engine: &Path, count: &mut u32) -> std::io::Result<()> {
    for f in dir.files() {
        write_engine_file(f, dst_engine, count)?;
    }
    for d in dir.dirs() {
        scaffold_engine(d, dst_engine, count)?;
    }
    Ok(())
}

/// `keel init DIR` (D0093) — scaffold a fresh project: the embedded engine (`.engine/`, with the
/// architecture decisions remapped to read-only `reference/`), `CLAUDE.md`, and a starter `.tracking/`.
/// Self-contained cold start; refuses to overwrite an existing `.engine/`.
fn cmd_init(args: &[String]) -> i32 {
    let Some(target) = args.first() else {
        eprintln!("usage: keel init DIR");
        return 2;
    };
    let dir = PathBuf::from(target);
    let engine_dst = dir.join(".engine");
    if engine_dst.exists() {
        eprintln!("error: {} already contains a .engine/ — refusing to overwrite", dir.display());
        return 2;
    }
    let mut count = 0u32;
    if let Err(e) = scaffold_engine(&ENGINE_DIR, &engine_dst, &mut count) {
        eprintln!("error scaffolding engine: {e}");
        return 1;
    }
    // Empty .engine/decisions/ — where the NEW project authors its own decisions (the engine's ship
    // as read-only reference under .engine/reference/decisions/).
    if let Err(e) = std::fs::create_dir_all(engine_dst.join("decisions")) {
        eprintln!("error creating .engine/decisions: {e}");
        return 1;
    }
    if let Err(e) = std::fs::write(dir.join("CLAUDE.md"), CLAUDE_MD) {
        eprintln!("error writing CLAUDE.md: {e}");
        return 1;
    }
    let tracking = dir.join(".tracking");
    if let Err(e) = std::fs::create_dir_all(&tracking) {
        eprintln!("error creating .tracking: {e}");
        return 1;
    }
    if let Err(e) = std::fs::write(tracking.join("README.md"), TRACKING_STARTER) {
        eprintln!("error writing .tracking/README.md: {e}");
        return 1;
    }
    // A starter actor registry so the newcomer's first recorded fact (createdBy/judgedBy) passes the
    // actors guard (D0037) — they edit it to their real identities.
    if let Err(e) = std::fs::write(tracking.join("actors.sysml"), STARTER_ACTORS) {
        eprintln!("error writing .tracking/actors.sysml: {e}");
        return 1;
    }
    // Scaffold a RUST-ONLY commit gate (.githooks/pre-commit) so the project has an automated
    // keel validate/guard gate from day one — no conda/kernel (D0048). The user enables it with
    // `git config core.hooksPath .githooks` (printed below).
    let hooks = dir.join(".githooks");
    if let Err(e) = std::fs::create_dir_all(&hooks) {
        eprintln!("error creating .githooks: {e}");
        return 1;
    }
    let hook_path = hooks.join("pre-commit");
    if let Err(e) = std::fs::write(&hook_path, PRECOMMIT_HOOK) {
        eprintln!("error writing .githooks/pre-commit: {e}");
        return 1;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let _ = std::fs::set_permissions(&hook_path, std::fs::Permissions::from_mode(0o755));
    }
    println!("Scaffolded the engine into {} ({count} engine file(s)).", dir.display());
    println!();
    println!("Next:");
    println!("  1. cd {}", dir.display());
    println!("  2. git init && git config core.hooksPath .githooks   (enable the keel pre-commit gate)");
    println!("  3. Read CLAUDE.md — how to work here (text is truth; the AI drives the CLI, you supervise).");
    println!("  4. Run the `introduction` skill (guided onboarding) — capture your first need + run your first sprint.");
    println!("     Or: keel orient .   (where things stand)");
    println!();
    println!("The .githooks/pre-commit gate runs `keel validate` + `keel guard` (Rust-only, no kernel).");
    println!("Engine design rationale is read-only reference in .engine/reference/decisions/;");
    println!("your project authors its OWN decisions fresh in .engine/decisions/.");
    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rest: &[String] = args.get(2..).unwrap_or(&[]);
    let code = match args.get(1).map(String::as_str) {
        Some("init") => cmd_init(rest),
        Some("serve") => cmd_serve(rest),
        Some("validate") => cmd_validate(rest),
        Some("check") => cmd_check(rest),
        Some("rules") => cmd_rules(rest),
        Some("business") => cmd_business(rest),
        Some("launchables") => cmd_launchables(rest),
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
        Some("dispositions") => cmd_dispositions(rest),
        Some("sitting-coverage") => cmd_sitting_coverage(rest),
        Some("concern-coverage") => cmd_concern_coverage(rest),
        Some("coverage") => cmd_coverage(rest),
        Some("critique-coverage") => cmd_critique_coverage(rest),
        Some("critique-policy") => cmd_critique_policy(rest),
        Some("rootedness") => cmd_query0(rest, "keel rootedness [ROOT]", |r| keel_cli::view::rootedness(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("tier-satisfaction") => cmd_query0(rest, "keel tier-satisfaction [ROOT]", |r| keel_cli::view::tier_satisfaction(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("recent") => cmd_query0(rest, "keel recent [ROOT]", |r| keel_cli::view::recent(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("boundary") => cmd_query1(rest, "boundary", |r, need| keel_cli::view::boundary_json(r, need).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("boundary-sweep") => cmd_query0(rest, "keel boundary-sweep [ROOT]", |r| keel_cli::view::boundary_sweep_json(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))),
        Some("reverify") => cmd_reverify(rest),
        Some("assured") => cmd_assured(rest),
        Some("decisions") => cmd_decisions(rest),
        Some("diagram") => cmd_diagram(rest),
        Some("render") => cmd_render(rest),
        Some("report") => cmd_report(rest),
        Some("indicators") => cmd_indicators(rest),
        Some("record-measurement") => cmd_record_measurement(rest),
        Some("snapshot-indicators") => cmd_snapshot_indicators(rest),
        Some("apply-review") => cmd_apply_review(rest),
        Some("outstanding") => cmd_query0(rest, "outstanding", keel_cli::queries::outstanding),
        Some("workflows") => cmd_query0(rest, "workflows", keel_cli::queries::workflows),
        Some("item") => cmd_query1(rest, "item", keel_cli::queries::item),
        Some("trace") => cmd_query1(rest, "trace", keel_cli::queries::trace),
        Some("trace-need") => cmd_query1(rest, "trace-need", keel_cli::queries::trace_need),
        Some("append-result") => cmd_append_result(rest),
        Some("append-gate-result") => cmd_append_gate_result(rest),
        Some("add-task") => cmd_add_task(rest),
        Some("record") => cmd_record(rest),
        _ => {
            eprintln!("keel <subcommand> [args]");
            eprintln!("  init DIR                     scaffold the engine into a NEW project (D0093 cold start)");
            eprintln!("  serve [--port N] [ROOT]      the interactive console — localhost read dashboard (D0094 m1)");
            eprintln!("  validate [ROOT]              semantic-validate all .tracking/ files");
            eprintln!("  check FILE...                parse-check one or more .sysml files");
        eprintln!("  check --spec-version         report the baked grammar version vs upstream (--no-fetch to skip the live check)");
            eprintln!("  ls [ROOT]                    list .tracking/ .sysml files");
            eprintln!("  orient [ROOT] [--html]       orient state as JSON, or --html = the human dashboard #View (D0093)");
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

#[cfg(test)]
mod tests {
    use super::{classify_guard_args, remap_engine_path, Path};

    #[test]
    fn guard_args_distinguish_name_from_root() {
        // Regression (v0.1.0 release smoke): `keel guard <ROOT>` must run all guards on ROOT, not read
        // ROOT as a guard name. A known name runs that one guard; "all"/no-arg/a path runs all.
        let s = |v: &[&str]| v.iter().map(|x| (*x).to_string()).collect::<Vec<_>>();
        assert_eq!(classify_guard_args(&s(&[])), (None, None)); // run all, default root
        assert_eq!(classify_guard_args(&s(&["all"])), (None, None)); // run all
        assert_eq!(classify_guard_args(&s(&["myproj"])), (None, Some("myproj"))); // bare ROOT -> run all on it
        assert_eq!(classify_guard_args(&s(&["."])), (None, Some("."))); // "." is a ROOT, not a guard
        assert_eq!(classify_guard_args(&s(&["ceremony"])), (Some("ceremony"), None)); // a known guard name
        assert_eq!(classify_guard_args(&s(&["ceremony", "myproj"])), (Some("ceremony"), Some("myproj"))); // name + root
        assert_eq!(classify_guard_args(&s(&["all", "myproj"])), (None, Some("myproj"))); // all on root
    }

    #[test]
    fn engine_path_remap_isolates_decisions() {
        // D0093 boundary: decisions ship as read-only reference, never as the new project's instance.
        assert_eq!(remap_engine_path(Path::new("decisions/0001-x.sysml")), Path::new("reference/decisions/0001-x.sysml"));
        // Everything else is scaffolded unchanged.
        assert_eq!(remap_engine_path(Path::new("schema/core/element.sysml")), Path::new("schema/core/element.sysml"));
        assert_eq!(remap_engine_path(Path::new("processes/introduction.sysml")), Path::new("processes/introduction.sysml"));
    }

    #[test]
    fn init_ships_downstream_claude_md_not_self_build() {
        // issue057 (field defect): `keel init` must ship a DOWNSTREAM "tracked by keel" CLAUDE.md,
        // NEVER the self-build's ("This repo is a work-tracking engine"). D0047 permanent control.
        assert!(super::CLAUDE_MD.contains("tracked by keel"), "init CLAUDE.md must frame the project as tracked BY keel");
        assert!(!super::CLAUDE_MD.contains("is a work-tracking engine"), "init must NOT ship the self-build CLAUDE.md");
        assert!(super::CLAUDE_MD.contains("Parsed:"), "downstream CLAUDE.md must carry the D0106 parse-first discipline");
    }
}
