//! No command reports GREEN over nothing (issue281).
//!
//! The issue269 refusal was applied to `validate` alone, so at a WORKSPACE ROOT — a repository holding
//! projects with none at the root — every other model-reading command answered about an empty model
//! and called it an answer. Measured in a two-project workspace full of work, before the fix:
//!
//!   * `orient` exited 0 with an empty ready list, zero outstanding and `answerStatus: COMPUTED` — and
//!     `orient` is the surface CLAUDE.md makes the AI's ONLY legitimate state read;
//!   * `whats-next` printed "COMPUTED-EMPTY — this is an answer, not a failure" over zero items;
//!   * `check-engine` printed "validated clean" over zero files, while being a BLOCKING step in this
//!     repository's own commit gate.
//!
//! The commands below deliberately span all THREE root-resolution paths, because a single chokepoint
//! did not cover them: `root_arg` (most), `cmd_view0` (the zero-argument views), `repo_arg` plus an
//! explicit check (`verification`), and a module resolving its own root (`attestation`). Nine refused
//! after the first fix and two still exited 0; only sweeping found them.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn keel() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keel"))
}

struct Tmp(PathBuf);
impl Drop for Tmp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Every command that reads a MODEL. Each must refuse at a workspace root and work inside a project.
/// Invocations, not bare verbs: D0273 collapsed the lens family, so six of these are now reached as
/// `keel show <lens>` and the refusal has to hold THROUGH the router. A router that resolved its own
/// root before delegating would answer green over nothing for every lens at once, which is the
/// failure this file exists to prevent made 35 times worse.
const MODEL_READERS: &[&[&str]] = &[
    &["orient"],          // root_arg — the AI's only legitimate state read
    &["whats-next"],      // root_arg — the ranked frontier
    &["check-engine"],    // root_arg — a BLOCKING step in the commit gate
    &["validate"],        // root_arg — already refused (issue269); pinned so it stays refused
    &["show", "coverage"],
    &["show", "suspect"],
    &["audit"],
    &["show", "open-issues"],
    &["show", "indicators"],
    &["show", "verification"],    // repo_arg + an explicit check
    &["show", "controls"],        // cmd_view0 — resolves its own root
    &["attestation"],             // resolves its own root inside its module
];

fn init_project(at: &Path) {
    std::fs::create_dir_all(at).expect("mkdir");
    let out = keel()
        .args(["init", at.to_str().unwrap(), "--profile", "guided"])
        .output()
        .expect("run keel init");
    assert!(out.status.success(), "init failed: {}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn no_model_command_answers_green_at_a_workspace_root() {
    let base = std::env::temp_dir().join(format!("keel_no_green_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let _cleanup = Tmp(base.clone());
    std::fs::create_dir_all(&base).expect("mkdir");
    Command::new("git").args(["init", "-q"]).arg(&base).output().expect("git init");
    init_project(&base.join("alpha"));
    init_project(&base.join("beta"));

    // At the workspace root every one of them must REFUSE. An empty answer here is a false clean.
    let mut green: Vec<&&[&str]> = Vec::new();
    for cmd in MODEL_READERS {
        let out = keel().args(*cmd).arg(&base).output().expect("run keel");
        if out.status.success() {
            green.push(cmd);
        }
    }
    assert!(
        green.is_empty(),
        "these answered GREEN over nothing at a workspace root: {green:?} — an empty model is not an \
         answer, it is a gate that could not run (K2)"
    );

    // And the refusal has to be USEFUL: name the projects the repository does hold, or the reader
    // cannot act on it.
    let out = keel().arg("orient").arg(&base).output().expect("run keel orient");
    let said = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(said.contains("alpha") && said.contains("beta"), "the refusal must name the projects: {said}");

    // Inside a project they must all still work, or this is a refusal rather than a fix.
    let mut broken: Vec<&&[&str]> = Vec::new();
    for cmd in MODEL_READERS {
        let out = keel().args(*cmd).arg(base.join("alpha")).output().expect("run keel");
        if !out.status.success() {
            broken.push(cmd);
        }
    }
    assert!(broken.is_empty(), "these stopped working INSIDE a project: {broken:?}");
}

/// The other half of issue281: the root search must stop at the repository boundary, or a command
/// answers about a repository the caller is not in — and that answer looks right.
#[test]
fn the_root_search_does_not_leave_the_repository() {
    let base = std::env::temp_dir().join(format!("keel_no_green_bnd_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let _cleanup = Tmp(base.clone());
    std::fs::create_dir_all(&base).expect("mkdir");
    Command::new("git").args(["init", "-q"]).arg(&base).output().expect("git init");
    init_project(&base);

    // A SEPARATE repository nested inside the project, with no `.engine` of its own.
    let nested = base.join("unrelated");
    std::fs::create_dir_all(&nested).expect("mkdir");
    Command::new("git").args(["init", "-q"]).arg(&nested).output().expect("git init nested");
    let out = keel().arg("validate").current_dir(&nested).output().expect("run keel validate");
    assert!(
        !out.status.success(),
        "validate answered for the OUTER repository from inside a different one: {}",
        String::from_utf8_lossy(&out.stdout)
    );

    // But a PLAIN subdirectory of the project must still resolve to it — that is ordinary use.
    let docs = base.join("docs");
    std::fs::create_dir_all(&docs).expect("mkdir");
    let out = keel().arg("validate").current_dir(&docs).output().expect("run keel validate");
    assert!(
        out.status.success(),
        "validate from a plain subdirectory must still find its own project: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
