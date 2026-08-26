//! A WORKSPACE: one git repository holding one or more keel projects (D0234/issue270).
//!
//! WHY (measured, not supposed). Asked whether several small projects could share a repo, I built the
//! arrangement and ran it. Three things work already: root resolution walks up to `.engine/`, drift is
//! file-scoped through each project's own `deliverable-manifest.txt`, and the write lock sits beside
//! each model root. Four things break, and they are exactly the ones that matter:
//!
//! 1. **Decision numbering collides** — two fresh projects both get `d0001`.
//! 2. **The channel cannot tell them apart** — its marker is `keel-decision: d0001` and its lookup is
//!    repo-scoped in GitHub, so the second project's issue never opens and `reject d0001` is ambiguous.
//! 3. **The gate cannot cover them** — git allows ONE `core.hooksPath` per repository, so at most one
//!    project is gated; and `keel validate` at the parent printed "0 tracking file(s) validated clean"
//!    and exited 0, so a hook there would gate NOTHING while reporting success (issue269).
//! 4. **`land` gates one root and pushes the whole repo** — the other projects ride out ungated.
//!
//! DISCOVERED, NOT DECLARED. A project is a directory holding both `.engine/` and `.tracking/`. A
//! manifest at the repo root would be a second place to keep the list true, and the filesystem already
//! knows — the same reasoning `/api/projects` records for the console. Verified against both real
//! repositories on this machine: exactly one project each, no nested false positives, so the predicate
//! does not mistake a reference tree or a fixture for a project.
use std::path::{Path, PathBuf};

/// One git repository and every keel project inside it.
pub struct Workspace {
    /// The git repository root — the boundary that matters, because it is what a commit, a push and a
    /// `core.hooksPath` are all scoped to.
    pub root: PathBuf,
    /// Every project in the repo, sorted. A single-project repo yields exactly `[root]`.
    pub projects: Vec<PathBuf>,
}

impl Workspace {
    /// Does this repo hold more than one project? Everything that has to change changes ONLY here —
    /// a single-project repo must behave exactly as it did before this existed.
    #[must_use]
    pub const fn is_multi(&self) -> bool {
        self.projects.len() > 1
    }

    /// The project's label within the workspace: its path relative to the repo root, slash-separated.
    /// The root project itself labels as `.`, which is what a single-project repo has always been.
    #[must_use]
    pub fn label(&self, project: &Path) -> String {
        project
            .strip_prefix(&self.root)
            .ok()
            .map(|r| r.to_string_lossy().replace('\\', "/"))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| ".".to_string())
    }

    /// The project that owns `path`, by longest-prefix match. A file may sit under the repo root but
    /// inside no project (a README, a workspace-level script) — that is `None`, not an error.
    #[must_use]
    pub fn owner_of(&self, path: &Path) -> Option<&PathBuf> {
        self.projects
            .iter()
            .filter(|p| path.starts_with(p))
            .max_by_key(|p| p.components().count())
    }
}

/// Is this directory a keel project?
#[must_use]
pub fn is_project(dir: &Path) -> bool {
    dir.join(".engine").is_dir() && dir.join(".tracking").is_dir()
}

/// The git repository root containing `from`, or `from` itself when it is not in a repo.
fn git_root(from: &Path) -> PathBuf {
    crate::gitx::git()
        .arg("-C")
        .arg(from)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim()))
        .filter(|p| p.is_dir())
        .unwrap_or_else(|| from.to_path_buf())
}

/// Directories that never contain a project and are expensive to walk.
const SKIP: [&str; 5] = [".git", "target", "node_modules", ".keel", ".claude"];

fn walk(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    if is_project(dir) {
        out.push(dir.to_path_buf());
        // A project does not nest inside another project. Stopping here also keeps a vendored or
        // reference copy inside a project from being reported as a peer.
        return;
    }
    if depth == 0 {
        return;
    }
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if !p.is_dir() {
            continue;
        }
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if name.starts_with('.') && !is_project(&p) || SKIP.contains(&name.as_str()) {
            continue;
        }
        walk(&p, depth - 1, out);
    }
}

/// Discover the workspace containing `from`.
///
/// Depth is bounded at 3 below the repo root: a project meant to be found lives at the top or one
/// folder down, and an unbounded walk would make every command pay for the whole tree.
#[must_use]
pub fn discover(from: &Path) -> Workspace {
    let root = git_root(from);
    let mut projects = Vec::new();
    walk(&root, 3, &mut projects);
    projects.sort();
    projects.dedup();
    Workspace { root, projects }
}

/// `keel projects [ROOT] [--json]` — every project in this workspace, and which one you are in.
#[must_use]
pub fn cmd(args: &[String]) -> i32 {
    let here = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map_or_else(|| std::path::PathBuf::from("."), std::path::PathBuf::from);
    let here = here.canonicalize().unwrap_or(here);
    let ws = discover(&here);
    let current = ws.owner_of(&here);

    if args.iter().any(|a| a == "--json") {
        let esc = |s: &str| s.replace('\\', "/").replace('"', "'");
        let rows: Vec<String> = ws
            .projects
            .iter()
            .map(|p| {
                format!(
                    "{{\"label\":\"{}\",\"root\":\"{}\",\"current\":{}}}",
                    esc(&ws.label(p)),
                    esc(&p.display().to_string()),
                    current.is_some_and(|c| c == p)
                )
            })
            .collect();
        println!(
            "{{\"workspaceRoot\":\"{}\",\"multiProject\":{},\"projects\":[{}]}}",
            esc(&ws.root.display().to_string()),
            ws.is_multi(),
            rows.join(",")
        );
        return 0;
    }

    println!("workspace: {}", ws.root.display());
    if ws.projects.is_empty() {
        println!("  NO keel project found in this repository (a project holds both .engine/ and .tracking/).");
        println!("  `keel init <dir>` scaffolds one. Reported rather than passed over in silence: an");
        println!("  absent project is why a gate can pass while checking nothing (issue269).");
        return 1;
    }
    for p in &ws.projects {
        let here = if current.is_some_and(|c| c == p) { " <- you are here" } else { "" };
        println!("  {}{here}", ws.label(p));
    }
    if ws.is_multi() {
        println!();
        println!("{} projects share this repository, so three things are workspace-scoped:", ws.projects.len());
        println!("  - the COMMIT GATE: git allows one core.hooksPath per repo, so the hook lives at the");
        println!("    repo root and runs `keel gate --workspace`; a per-project hook can only cover one.");
        println!("  - `keel land`: a push carries the whole repo, so every project is gated before it.");
        println!("  - DECISION identity: `dNNNN` is unique per project, so the channel qualifies it with");
        println!("    the project label - `alpha/d0001` - or two projects' first decisions collide.");
    }
    0
}

/// The full gate for ONE project. Split out so `gate_cmd` stays readable and so the per-project
/// standard has a single definition — a workspace gate that checked less than the per-project one
/// would make adopting a workspace a quiet downgrade.
fn gate_one(p: &Path, label: &str) -> Result<(), String> {
    println!("gate [{label}] validate + guard + rules");
    let report = crate::validate_root(p);
    if !report.diagnostics.is_empty() || !report.errors.is_empty() {
        for (path, d) in &report.diagnostics {
            println!("  ERROR {}:{} — {}", path.display(), d.line, d.message);
        }
        for e in &report.errors {
            println!("  PARSE {} — {}", e.file.display(), e.message);
        }
        return Err(format!("{label} (validate)"));
    }
    let violations: Vec<String> = crate::guards::run_all(p)
        .into_iter()
        .flat_map(|g| g.violations.into_iter().map(move |v| format!("{}: {v}", g.name)))
        .collect();
    if !violations.is_empty() {
        for v in violations.iter().take(8) {
            println!("  VIOLATION {v}");
        }
        return Err(format!("{label} (guard)"));
    }
    let rule_violations: Vec<String> = crate::view::check(p)
        .ok()
        .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
        .map(|v| {
            v.get("rules")
                .and_then(|r| r.as_array())
                .map(|rules| {
                    rules
                        .iter()
                        .filter(|r| r.get("severity").and_then(|s| s.as_str()) != Some("warning"))
                        .flat_map(|r| {
                            let name =
                                r.get("rule").and_then(|n| n.as_str()).unwrap_or("rule").to_string();
                            r.get("violations")
                                .and_then(|x| x.as_array())
                                .cloned()
                                .unwrap_or_default()
                                .into_iter()
                                .map(move |v| format!("{name}: {v}"))
                        })
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default();
    if rule_violations.is_empty() {
        println!("  clean ({} file(s))", report.validated);
        Ok(())
    } else {
        for v in rule_violations.iter().take(8) {
            println!("  RULE {v}");
        }
        Err(format!("{label} (rules)"))
    }
}

/// `keel gate --workspace [ROOT]` — the COMMIT gate for a repo holding several projects.
///
/// git allows one `core.hooksPath` per repository, so a per-project hook can only ever gate one
/// project; the hook has to live at the repo root and gate every project the commit touches. That is
/// what this is: staged files are mapped to their owning projects, and each such project gets the full
/// gate. Projects the commit does not touch are NAMED as skipped rather than passed over quietly —
/// silence is how a gate that checked nothing comes to look like a gate that passed.
///
/// FAILS LOUDLY when it cannot run (K2): no projects at all is a non-zero exit, not a clean tree.
#[must_use]
pub fn gate_cmd(args: &[String]) -> i32 {
    let here = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map_or_else(|| std::path::PathBuf::from("."), std::path::PathBuf::from);
    let here = here.canonicalize().unwrap_or(here);
    let ws = discover(&here);

    if ws.projects.is_empty() {
        eprintln!("gate: NO keel project in {} — a gate that cannot run must not pass (K2).", ws.root.display());
        return 1;
    }

    // Which projects does this commit actually touch? Staged paths are repo-relative.
    let staged = crate::gitx::git()
        .arg("-C")
        .arg(&ws.root)
        .args(["diff", "--cached", "--name-only"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    let mut touched: Vec<PathBuf> = Vec::new();
    let mut workspace_level: Vec<String> = Vec::new();
    for line in staged.lines().filter(|l| !l.trim().is_empty()) {
        let abs = ws.root.join(line);
        match ws.owner_of(&abs) {
            Some(p) if !touched.contains(p) => touched.push(p.clone()),
            Some(_) => {}
            None => workspace_level.push(line.to_string()),
        }
    }
    // Nothing staged (or no git) means gate EVERYTHING: a caller asking for the workspace gate
    // without a commit in flight wants the whole answer, and guessing narrower would under-report.
    let gated: Vec<PathBuf> = if touched.is_empty() { ws.projects.clone() } else { touched };

    let mut failed: Vec<String> = Vec::new();
    for p in &gated {
        let label = ws.label(p);
        if let Err(which) = gate_one(p, &label) {
            failed.push(which);
        }
    }
    // Say what was NOT gated. A skipped project is a fact the reader needs, not noise.
    let skipped: Vec<String> =
        ws.projects.iter().filter(|p| !gated.contains(p)).map(|p| ws.label(p)).collect();
    if !skipped.is_empty() {
        println!("gate: {} project(s) untouched by this commit, so NOT gated: {}", skipped.len(), skipped.join(", "));
    }
    if !workspace_level.is_empty() {
        println!(
            "gate: {} staged file(s) belong to no project (workspace-level), so no project gate covers them: {}",
            workspace_level.len(),
            workspace_level.iter().take(5).cloned().collect::<Vec<_>>().join(", ")
        );
    }
    if failed.is_empty() {
        println!("gate: {} project(s) gated clean.", gated.len());
        0
    } else {
        eprintln!("gate: FAILED in {}: {}", failed.len(), failed.join(", "));
        1
    }
}

#[cfg(test)]
mod tests {
    use super::{discover, is_project, Workspace};
    use std::path::PathBuf;

    #[test]
    fn a_single_project_repo_is_unchanged_by_any_of_this() {
        // The whole design rests on this: everything workspace-aware must be a no-op for the repos
        // that exist today, or the feature is a migration rather than an addition.
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let ws = discover(&root);
        assert!(is_project(&ws.root), "this repo's root is itself a project");
        assert_eq!(ws.projects.len(), 1, "discovery must find exactly one project here, got {:?}", ws.projects);
        assert!(!ws.is_multi(), "a single-project repo must not take any workspace branch");
        assert_eq!(ws.label(&ws.projects[0]), ".", "the root project labels as `.`");
    }

    #[test]
    fn labels_and_ownership_resolve_within_the_workspace() {
        let ws = Workspace {
            root: PathBuf::from("/repo"),
            projects: vec![PathBuf::from("/repo/alpha"), PathBuf::from("/repo/nested/beta")],
        };
        assert!(ws.is_multi());
        assert_eq!(ws.label(&PathBuf::from("/repo/alpha")), "alpha");
        assert_eq!(ws.label(&PathBuf::from("/repo/nested/beta")), "nested/beta");
        // A staged file resolves to the project that owns it - that is what lets the hook gate only
        // the projects a commit actually touches.
        assert_eq!(ws.owner_of(&PathBuf::from("/repo/alpha/.tracking/x.sysml")), Some(&PathBuf::from("/repo/alpha")));
        assert_eq!(ws.owner_of(&PathBuf::from("/repo/nested/beta/.engine/y.sysml")), Some(&PathBuf::from("/repo/nested/beta")));
        // A workspace-level file belongs to no project. That is an answer, not an error.
        assert_eq!(ws.owner_of(&PathBuf::from("/repo/README.md")), None);
    }
}
