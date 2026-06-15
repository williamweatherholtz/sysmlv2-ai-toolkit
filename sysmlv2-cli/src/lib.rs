//! Core logic for the `sysmlv2` CLI — validate and check commands.
//!
//! - [`validate_root`]: register schema packages, semantic-validate all `.tracking/` files.
//! - [`check_files`]: parse-only check for one or more files.
#![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]

use std::path::{Path, PathBuf};

use sysmlv2_parser::{parse, tokenize, Diagnostic, PackageRegistry};

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

// ── internal parse helper ─────────────────────────────────────────────────────

fn parse_pkg(path: &Path) -> Result<sysmlv2_parser::ast::Package, CheckError> {
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
