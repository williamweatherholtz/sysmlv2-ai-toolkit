//! The engine tree embedded in this binary - ONE `include_dir!`, shared by the two things that need it.
//!
//! `keel init` scaffolds it and `keel migrate` resyncs a project from it (D0093 / D0275); since D0441
//! the `process-change` guard reads it too, to tell the engine's own published text arriving on a
//! project from a hand edit to a locked file. A second `include_dir!` of the same tree would embed
//! every byte twice, so the static lives here in the library and `main.rs` borrows it.

use include_dir::{include_dir, Dir};

/// The reusable engine tree + operating manual, embedded at compile time so `keel init` is
/// self-contained (no external fetch - the cytoscape precedent).
pub static ENGINE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../.engine");

/// The embedded engine's text at `rel` (a path relative to `.engine/`, forward slashes), or `None`
/// when the engine ships no file there or the file is not UTF-8.
#[must_use]
pub fn engine_text(rel: &str) -> Option<&'static str> {
    ENGINE_DIR.get_file(rel).and_then(include_dir::File::contents_utf8)
}
