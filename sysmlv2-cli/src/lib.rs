//! Core logic for the `sysmlv2` CLI — validate, check, and orient commands.
//!
//! - [`validate_root`]: register schema packages, semantic-validate all `.tracking/` files.
//! - [`check_files`]: parse-only check for one or more files.
//! - [`orient_root`]: compute orient view from `.tracking/` (cursor + ready/outstanding).
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
// Tests may use unwrap/expect/panic/indexing/asserts freely.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing))]

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use sysmlv2_parser::ast::{ActionDef, Item, Package, Part, Value};
use sysmlv2_parser::{parse, tokenize, Diagnostic, PackageRegistry};

pub mod algo;
pub mod indexer;
mod json;
pub mod orient;
pub mod view;
pub mod write;

// ── file discovery ────────────────────────────────────────────────────────────

/// Recursively collect every `.sysml` file under `dir`, sorted by path.
///
/// Returns an empty `Vec` if `dir` does not exist or is not readable.
#[must_use]
pub fn collect_sysml(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            out.extend(collect_sysml(&p));
        } else if p.extension().and_then(|e| e.to_str()) == Some("sysml") {
            out.push(p);
        }
    }
    out.sort();
    out
}

// ── report types ─────────────────────────────────────────────────────────────

/// A parse or I/O failure encountered while processing a single file.
#[derive(Debug, Clone)]
pub struct CheckError {
    /// The file that caused the error.
    pub file: PathBuf,
    /// Human-readable description of the failure.
    pub message: String,
}

/// Accumulated results from a [`check_files`] or [`validate_root`] run.
#[derive(Debug, Default)]
pub struct Report {
    /// Files that could not be read or parsed.
    pub errors: Vec<CheckError>,
    /// Semantic diagnostics produced by [`PackageRegistry::validate`].
    pub diagnostics: Vec<(PathBuf, Diagnostic)>,
    /// Number of `.tracking/` files that were semantically validated.
    pub validated: usize,
}

impl Report {
    /// `true` when there are no errors and no diagnostics.
    #[must_use]
    pub const fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.diagnostics.is_empty()
    }
}

// ── orient types ─────────────────────────────────────────────────────────────

/// Workflow cursor read from `.tracking/state.sysml`.
#[derive(Debug, Clone, Default)]
pub struct Cursor {
    /// Active workflow name (e.g. `"DeliveryWorkflow"`).
    pub active_workflow: String,
    /// Active phase name (e.g. `"Delivery/implement"`).
    pub active_phase: String,
    /// ISO-8601 date the phase was entered.
    pub entered_at: String,
    /// Actor who entered the phase.
    pub entered_by: String,
}

/// Results of an [`orient_root`] computation.
#[derive(Debug, Default)]
pub struct OrientReport {
    /// Workflow cursor, if one was found.
    pub cursor: Option<Cursor>,
    /// Task names that are not done but have all predecessors done.
    pub ready: Vec<String>,
    /// Number of tasks with a passing latest `TestResult`.
    pub done: usize,
    /// Number of tasks that are not yet done.
    pub outstanding: usize,
}

impl OrientReport {
    /// Serialize as JSON matching `query.py orient` output format.
    #[must_use]
    pub fn to_json(&self) -> String {
        let cursor_json = self.cursor.as_ref().map_or_else(
            || "null".to_owned(),
            |c| format!(
                "{{\"activeWorkflow\":{},\"activePhase\":{},\"enteredAt\":{},\"enteredBy\":{}}}",
                json_str(&c.active_workflow),
                json_str(&c.active_phase),
                json_str(&c.entered_at),
                json_str(&c.entered_by),
            ),
        );
        let ready_json = self
            .ready
            .iter()
            .map(|s| json_str(s))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{\"cursor\":{cursor},\"ready\":[{ready}],",
                "\"suspect\":[],\"invalidEvidence\":[],",
                "\"counts\":{{\"done\":{done},\"outstanding\":{outstanding}}}}}"
            ),
            cursor = cursor_json,
            ready = ready_json,
            done = self.done,
            outstanding = self.outstanding,
        )
    }
}

// ── orient helpers ────────────────────────────────────────────────────────────

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn dod_result_is_pass(task_name: &str, def: &ActionDef) -> bool {
    // Accept both `{task}DoDR{n}` (Sprint 6+ naming) and `{task}R{n}` (legacy naming).
    let dodr_prefix = format!("{task_name}DoDR");
    let legacy_prefix = format!("{task_name}R");
    let mut best: Option<(u32, bool)> = None;
    for part in &def.parts {
        let suffix = part.name.strip_prefix(&dodr_prefix)
            .or_else(|| part.name.strip_prefix(&legacy_prefix));
        if let Some(suffix) = suffix {
            if let Ok(n) = suffix.parse::<u32>() {
                let is_pass = part_outcome_is_pass(part);
                if best.is_none_or(|(b, _)| n > b) {
                    best = Some((n, is_pass));
                }
            }
        }
    }
    best.is_some_and(|(_, p)| p)
}

fn part_outcome_is_pass(part: &Part) -> bool {
    part.attributes.iter().any(|attr| {
        attr.name == "outcome"
            && matches!(
                &attr.value,
                Value::EnumLit { member, .. } if member == "pass"
            )
    })
}

fn str_value(value: &Value) -> Option<&str> {
    match value {
        Value::Str(s) | Value::Ident(s) => Some(s),
        _ => None,
    }
}

// ── public orient API ─────────────────────────────────────────────────────────

/// Extract the workflow cursor from a parsed package.
///
/// Returns `Some(Cursor)` if the package contains a `Part` with an
/// `activeWorkflow` attribute; `None` otherwise.
#[must_use]
pub fn parse_cursor(pkg: &Package) -> Option<Cursor> {
    for item in &pkg.items {
        if let Item::Part(p) = item {
            let mut cursor = Cursor::default();
            let mut found = false;
            for attr in &p.attributes {
                let Some(s) = str_value(&attr.value) else {
                    continue;
                };
                match attr.name.as_str() {
                    "activeWorkflow" => {
                        s.clone_into(&mut cursor.active_workflow);
                        found = true;
                    }
                    "activePhase" => s.clone_into(&mut cursor.active_phase),
                    "enteredAt" => s.clone_into(&mut cursor.entered_at),
                    "enteredBy" => s.clone_into(&mut cursor.entered_by),
                    _ => {}
                }
            }
            if found {
                return Some(cursor);
            }
        }
    }
    None
}

/// Compute the orient state (ready/done/outstanding) from a set of parsed packages.
///
/// Returns `(ready, done_count, outstanding_count)`.
/// - `ready`: task names that are not done but have all predecessors done.
/// - `done_count`: number of tasks with a passing latest `TestResult`.
/// - `outstanding_count`: number of tasks that are not yet done.
#[must_use]
pub fn compute_orient_state(packages: &[Package]) -> (Vec<String>, usize, usize) {
    let mut actions: Vec<String> = Vec::new();
    let mut successions: Vec<(String, String)> = Vec::new();

    for pkg in packages {
        for item in &pkg.items {
            if let Item::ActionDef(def) = item {
                for action in &def.actions {
                    actions.push(action.name.clone());
                }
                for suc in &def.successions {
                    if !suc.is_ordering_only {
                        successions.push((suc.first.clone(), suc.then.clone()));
                    }
                }
            }
        }
    }

    let done_set: HashSet<String> = actions
        .iter()
        .filter(|name| {
            packages.iter().any(|pkg| {
                pkg.items.iter().any(|item| {
                    if let Item::ActionDef(def) = item {
                        dod_result_is_pass(name, def)
                    } else {
                        false
                    }
                })
            })
        })
        .cloned()
        .collect();

    let mut ready: Vec<String> = actions
        .iter()
        .filter(|name| !done_set.contains(name.as_str()))
        .filter(|name| {
            successions
                .iter()
                .filter(|(_, then)| then.as_str() == name.as_str())
                .all(|(first, _)| done_set.contains(first.as_str()))
        })
        .cloned()
        .collect();
    ready.sort();

    let done = done_set.len();
    let outstanding = actions.len().saturating_sub(done);
    (ready, done, outstanding)
}

/// Return the list of ready tasks from a project root — the `whats-next` view.
///
/// Identical to `orient_root(root).ready`; exported as a first-class API so
/// callers and tests can use it without constructing a full `OrientReport`.
#[must_use]
pub fn whats_next_root(root: &Path) -> Vec<String> {
    orient_root(root).ready
}

/// Compute the orient view from a project root.
///
/// Reads all `.sysml` files under `root/.tracking/`, extracts the cursor
/// from the first package containing an `activeWorkflow` attribute, and
/// computes done/ready/outstanding from action def `DoD` `TestResults`.
#[must_use]
pub fn orient_root(root: &Path) -> OrientReport {
    let tracking_dir = root.join(".tracking");
    let packages: Vec<Package> = collect_sysml(&tracking_dir)
        .iter()
        .filter_map(|path| parse_pkg(path).ok())
        .collect();

    let cursor = packages.iter().find_map(parse_cursor);
    let (ready, done, outstanding) = compute_orient_state(&packages);

    OrientReport { cursor, ready, done, outstanding }
}

// ── internal parse helper ─────────────────────────────────────────────────────

fn parse_pkg(path: &Path) -> Result<Package, CheckError> {
    let src = std::fs::read_to_string(path).map_err(|e| CheckError {
        file: path.to_path_buf(),
        message: e.to_string(),
    })?;
    let name = path.to_string_lossy();
    let tokens = tokenize(&src, &name).map_err(|e| CheckError {
        file: path.to_path_buf(),
        message: e.to_string(),
    })?;
    parse(tokens, &name).map_err(|e| CheckError {
        file: path.to_path_buf(),
        message: e.to_string(),
    })
}

// ── public commands ───────────────────────────────────────────────────────────

/// Parse-check each file in `files` without semantic validation.
///
/// Reads and tokenizes each file; adds a [`CheckError`] for any file that
/// cannot be read or that produces a lex/parse error.
#[must_use]
pub fn check_files(files: &[PathBuf]) -> Report {
    let mut report = Report::default();
    for path in files {
        if let Err(e) = parse_pkg(path) {
            report.errors.push(e);
        }
    }
    report
}

/// Register all schema packages under `root/.engine/` then semantically
/// validate every `.sysml` file under `root/.tracking/`.
///
/// Schema files are registered as ground truth but are not themselves
/// validated (they may reference `ScalarValues::*` which the registry
/// treats as a system namespace).  Tracking files are both registered and
/// validated so they may import each other.
#[must_use]
pub fn validate_root(root: &Path) -> Report {
    let mut report = Report::default();
    let mut registry = PackageRegistry::new();

    // Phase 1 — register all schema packages.
    let engine_dir = root.join(".engine");
    if engine_dir.is_dir() {
        for path in collect_sysml(&engine_dir) {
            match parse_pkg(&path) {
                Ok(pkg) => registry.register(&pkg),
                Err(e) => report.errors.push(e),
            }
        }
    }

    // Phase 2 — register + validate every tracking file.
    let tracking_dir = root.join(".tracking");
    if tracking_dir.is_dir() {
        for path in collect_sysml(&tracking_dir) {
            match parse_pkg(&path) {
                Ok(pkg) => {
                    registry.register(&pkg);
                    let diags = registry.validate(&pkg, &path.to_string_lossy());
                    report.validated += 1;
                    for d in diags {
                        report.diagnostics.push((path.clone(), d));
                    }
                }
                Err(e) => report.errors.push(e),
            }
        }
    }

    report
}
