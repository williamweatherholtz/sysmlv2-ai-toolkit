//! Integration test: parse every .sysml file in the repo without error;
//! plus registry validation: all .tracking/ files validate semantically clean.

use std::path::{Path, PathBuf};
use keel_parser::{parse, tokenize, PackageRegistry};

fn repo_root() -> PathBuf {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    Path::new(&manifest).parent().expect("manifest has no parent").to_path_buf()
}

fn parse_file(path: &Path) {
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let filename = path.to_string_lossy();
    let tokens = tokenize(&src, &filename)
        .unwrap_or_else(|e| panic!("lex error in {}: {e}", path.display()));
    parse(tokens, &filename)
        .unwrap_or_else(|e| panic!("parse error in {}: {e}", path.display()));
}

fn sysml_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            out.extend(sysml_files_under(&p));
        } else if p.extension().and_then(|e| e.to_str()) == Some("sysml") {
            out.push(p);
        }
    }
    out
}

#[test]
fn parse_all_engine_files() {
    let dir = repo_root().join(".engine");
    let files = sysml_files_under(&dir);
    assert!(!files.is_empty(), "no .sysml files found under .engine/");
    for f in files {
        parse_file(&f);
    }
}

#[test]
fn parse_all_tracking_files() {
    let dir = repo_root().join(".tracking");
    let files = sysml_files_under(&dir);
    assert!(!files.is_empty(), "no .sysml files found under .tracking/");
    for f in files {
        parse_file(&f);
    }
}

fn parse_pkg(path: &Path) -> keel_parser::ast::Package {
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let filename = path.to_string_lossy();
    let tokens = tokenize(&src, &filename)
        .unwrap_or_else(|e| panic!("lex error in {}: {e}", path.display()));
    parse(tokens, &filename)
        .unwrap_or_else(|e| panic!("parse error in {}: {e}", path.display()))
}

/// Load all schema packages into the registry, then validate every .tracking/ file.
/// All tracking files must produce zero diagnostics.
#[test]
fn validate_all_tracking_with_registry() {
    let root = repo_root();

    // Phase 1 — register all schema packages (ground truth, not validated).
    let mut registry = PackageRegistry::new();
    for f in sysml_files_under(&root.join(".engine")) {
        let pkg = parse_pkg(&f);
        registry.register(&pkg);
    }

    // Phase 2 — register and validate every tracking file.
    let tracking_files = sysml_files_under(&root.join(".tracking"));
    assert!(!tracking_files.is_empty(), "no .sysml files found under .tracking/");

    let mut failures: Vec<String> = Vec::new();
    for f in &tracking_files {
        let pkg = parse_pkg(f);
        registry.register(&pkg); // register tracking packages too (they may import each other)
        let diags = registry.validate(&pkg, &f.to_string_lossy());
        for d in diags {
            failures.push(format!(
                "{}:{} — {} {}",
                f.display(),
                d.line,
                d.message,
                d.suggestion.as_deref().map_or(String::new(), |s| format!("(hint: {s})")),
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Registry validation failures in .tracking/ files:\n{}",
        failures.join("\n")
    );
}
