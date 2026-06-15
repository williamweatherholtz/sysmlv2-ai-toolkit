//! Integration test: parse every .sysml file in the repo without error.

use std::path::{Path, PathBuf};
use sysmlv2_parser::{parse, tokenize};

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
