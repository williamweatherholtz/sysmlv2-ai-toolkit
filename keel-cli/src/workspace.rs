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
//! manifest at the repo root would be a second place to keep the list true, and git already knows —
//! the same reasoning `/api/projects` records for the console.
//!
//! Measured across the NINE keel repositories on this machine: exactly one project each, no nested
//! false positives. An earlier version of this note claimed the same thing about "both real
//! repositories", which undercounted by seven and was VACUOUS besides — discovery short-circuited at
//! a root project, so it examined no subdirectory in any of them and could not have found a false
//! positive if one existed. Discovery now descends (`discover`), so the claim is a measurement.
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
    ///
    /// The argument is normalised first (`canon`), because the two sides reach here from different
    /// sources that spell the same directory differently — see `canon` for what that cost.
    #[must_use]
    pub fn owner_of(&self, path: &Path) -> Option<&PathBuf> {
        let needle = canon(path);
        self.projects
            .iter()
            .filter(|p| needle.starts_with(p))
            .max_by_key(|p| p.components().count())
    }
}

/// Is this directory a keel project?
#[must_use]
pub fn is_project(dir: &Path) -> bool {
    dir.join(".engine").is_dir() && dir.join(".tracking").is_dir()
}

/// One spelling for a path, so two that name the same directory compare equal.
///
/// Every prefix comparison in this module has one side from git (`rev-parse --show-toplevel`, long
/// names, forward slashes) and the other from the process (a CLI argument, `env::temp_dir()`,
/// `canonicalize`). On Windows those differ in three ways at once: `canonicalize` returns the
/// extended-length `\\?\` prefix, git returns forward slashes, and a path may arrive in 8.3 short
/// form (`WILLIA~1`) that git has already expanded. Any one of them makes `starts_with` fail on
/// paths that are in fact the same, which is why `keel projects` reported `current: false` for every
/// project in all eight real keel repos on this machine and printed no you-are-here marker.
///
/// `canonicalize` resolves short names and symlinks; stripping `\\?\` puts the result back in the
/// form the rest of the codebase prints and joins. A path that does not exist yet cannot be
/// canonicalised, so it is returned unchanged — comparisons between two such literals still work.
#[must_use]
pub fn canon(p: &Path) -> PathBuf {
    p.canonicalize().map_or_else(
        |_| p.to_path_buf(),
        |c| {
            let s = c.to_string_lossy().to_string();
            PathBuf::from(s.strip_prefix(r"\\?\").map_or(s.as_str(), |t| t).to_string())
        },
    )
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
        .map_or_else(|| canon(from), |p| canon(&p))
}

/// Directories that never contain a project and are expensive to walk.
const SKIP: [&str; 5] = [".git", "target", "node_modules", ".keel", ".claude"];

fn walk(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    if is_project(dir) {
        out.push(dir.to_path_buf());
        // issue275: this used to `return` here, on the reasoning that a project does not nest inside
        // another project. `keel init` refuses neither position, so the reasoning described a
        // convention rather than the code — and a project nested inside another was invisible to
        // every workspace-scoped mechanism, which meant it rode out UNGATED. Keep descending; a
        // reference or vendored copy is not a false positive because it has no `.tracking/`.
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

/// Every path git knows about under `root` — tracked plus untracked-and-not-ignored — repo-relative
/// with forward slashes. Empty when `root` is not a git repository.
///
/// Untracked-but-not-ignored matters: a project one minute old has no tracked files yet, and it is
/// exactly the project most likely to be missed.
fn git_known_paths(root: &Path) -> Vec<String> {
    crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["-c", "core.quotePath=false", "ls-files", "-c", "-o", "--exclude-standard"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .map(|l| l.trim().replace('\\', "/"))
                .filter(|l| !l.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Discover the workspace containing `from`.
///
/// # Why git, and not a directory walk (issue275)
///
/// This used to be a depth-3 recursive walk that RETURNED at the first project it found. Three
/// panelists found the same consequence independently: a project nested inside another, or deeper
/// than three, was invisible to every workspace-scoped mechanism and therefore rode out UNGATED.
/// The verified reproduction was `keel init .` then `keel init sub` in one repo reporting a single
/// project, after which a commit that added an unparseable tracking file AND downgraded every
/// blocking rule in the invisible project exited 0 while the gate printed clean.
///
/// Asking git is strictly better than a deeper walk. It is bounded by what the repository actually
/// contains rather than by a guessed depth, it costs one process instead of a recursive stat storm,
/// it already excludes `.git`, `target`, and anything else `.gitignore` covers, and — the property
/// that matters for a gate — it enumerates exactly the set a push can carry. A project git does not
/// know about cannot reach a commit, so not gating it is a definition rather than a hole.
///
/// The filesystem walk survives as the fallback for a directory that is not a git repository at all,
/// where there is no push and no shared hook for a workspace mechanism to be responsible for.
#[must_use]
pub fn discover(from: &Path) -> Workspace {
    let root = git_root(from);
    let mut projects = Vec::new();
    for rel in git_known_paths(&root) {
        // A project is the directory holding `.engine/` and `.tracking/`, so every path under either
        // marker names one. Taking the prefix before the marker finds it at ANY depth.
        for marker in [".engine/", ".tracking/"] {
            let Some(i) = rel.find(marker) else { continue };
            // The marker has to start a path component: `vendor/.engine/x` counts, `my.engine/x`
            // does not.
            if i != 0 && !rel[..i].ends_with('/') {
                continue;
            }
            let prefix = rel[..i].trim_end_matches('/');
            let dir = if prefix.is_empty() { root.clone() } else { root.join(prefix) };
            if is_project(&dir) {
                projects.push(canon(&dir));
            }
        }
    }
    if projects.is_empty() {
        walk(&root, 3, &mut projects);
    }
    projects.sort();
    projects.dedup();
    Workspace { root, projects }
}

/// Staged paths that are KEYSTONE events and that no project gate covers (issue276).
///
/// # The hole this closes
///
/// In a workspace the repo-root enforcement surface — the ONE `.githooks/pre-commit` that gates every
/// project, and `.github/workflows/` — belongs to no project, and the keystone lock matches
/// project-relative paths. So the lock could never fire on it. Verified: the single hook that gates
/// every project, replaced with a two-line `exit 0` and staged alone with no Decision, and
/// `gate --workspace` reported 2 projects gated clean, exit 0 — after PRINTING a line that named the
/// unowned file. A gate that reports what it is not checking and passes anyway is the false-green
/// class, stated out loud.
///
/// Two rules, because the second is not a special case of the first:
///
/// 1. An unowned staged path that `guards::is_locked_path` claims is a keystone event. At a workspace
///    root the staged paths are repo-relative, so `.githooks/**` and `.github/workflows/**` match the
///    same predicate the per-project lock uses.
/// 2. Any staged DELETION of a path under an `.engine/` directory, at any depth, owned or not. This
///    is not covered by rule 1: deleting a whole project makes its directory stop satisfying
///    `is_project`, so every one of its paths becomes UNOWNED — which is how staging the deletion of
///    an entire project, 445 paths including every process definition and its guards, passed the gate
///    silently. A project's engine files are owned while the project exists and unowned at exactly
///    the moment that matters.
fn unowned_keystone_events(unowned: &[String], deleted: &[String]) -> Vec<String> {
    let mut events: Vec<String> = unowned
        .iter()
        .filter(|p| crate::guards::is_locked_path(p))
        .map(|p| format!("{p} (repo-root enforcement surface — owned by no project)"))
        .collect();
    for d in deleted {
        let slashed = d.replace('\\', "/");
        if slashed.starts_with(".engine/") || slashed.contains("/.engine/") {
            events.push(format!("{d} (DELETED engine path — removing a control is a keystone event)"));
        }
    }
    events.sort();
    events.dedup();
    events
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
    // issue276: the keystone lock, applied to what no project gate covers. Printing the unowned files
    // and then exiting 0 was the whole defect — this is the line that makes the report a verdict.
    let deleted: Vec<String> = crate::gitx::git()
        .arg("-C")
        .arg(&ws.root)
        .args(["-c", "core.quotePath=false", "diff", "--cached", "--name-only", "--diff-filter=D"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().map(str::to_owned).collect())
        .unwrap_or_default();
    let events = unowned_keystone_events(&workspace_level, &deleted);
    if !events.is_empty() && !crate::guards::staged_marked_decision(&ws.root) {
        eprintln!("gate: KEYSTONE — {} staged change(s) to the locked surface carry no co-committed", events.len());
        eprintln!("  Decision bearing #ProspectiveChange or #SafetyChange (D0070/D0209 clause 2):");
        for e in events.iter().take(10) {
            eprintln!("    {e}");
        }
        if events.len() > 10 {
            eprintln!("    ... and {} more", events.len() - 10);
        }
        failed.push("workspace keystone".to_string());
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
    use super::{discover, is_project, unowned_keystone_events, Workspace};
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
    /// issue275: discovery must find EVERY project git knows about — nested, and deeper than the old
    /// depth bound. This is the test the previous one lacked: the earlier unit test constructed a
    /// `Workspace` by hand from POSIX literals and never went through `discover`, so it could not
    /// have caught a discovery defect, and the one test that did call `discover` asserted a single
    /// project against this repo, where one is the correct answer.
    ///
    /// Builds a real git repo on disk, because the whole fix is "ask git" and a hand-built fixture
    /// would test the string arithmetic while skipping the part that was wrong.
    #[test]
    fn discovery_finds_nested_and_deep_projects() {
        let dir = std::env::temp_dir().join(format!("keel_ws_discover_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mk = |p: &std::path::Path| {
            let _ = std::fs::create_dir_all(p.join(".engine"));
            let _ = std::fs::create_dir_all(p.join(".tracking"));
            let _ = std::fs::write(p.join(".tracking").join("x.sysml"), "package X {}
");
        };
        let _ = std::fs::create_dir_all(&dir);
        let git = |args: &[&str]| {
            let _ = crate::gitx::git().arg("-C").arg(&dir).args(args).output();
        };
        git(&["init", "-q", "."]);

        mk(&dir); // a project AT the repo root
        let nested = dir.join("sub"); // a project INSIDE it — the layout the panel reproduced
        let _ = std::fs::create_dir_all(&nested);
        mk(&nested);
        let deep = dir.join("a").join("b").join("c").join("d"); // depth 4 — past the old bound
        let _ = std::fs::create_dir_all(&deep);
        mk(&deep);

        let ws = discover(&dir);
        let found: Vec<String> = ws.projects.iter().map(|p| ws.label(p)).collect();
        assert_eq!(ws.projects.len(), 3, "all three projects must be found, got {found:?}");
        assert!(found.iter().any(|l| l == "."), "the root project: {found:?}");
        assert!(found.iter().any(|l| l == "sub"), "the NESTED project: {found:?}");
        assert!(found.iter().any(|l| l == "a/b/c/d"), "the depth-4 project: {found:?}");
        assert!(ws.is_multi(), "three projects is a workspace");

        // Ownership resolves through real discovery, not a hand-built structure: the nested project
        // owns its own files even though its parent is also a project.
        // Compared by LABEL, not by path string: the point is which project owns the file, and the
        // two paths legitimately differ in spelling (`canon` resolves the 8.3 short name that
        // `env::temp_dir()` hands back on this platform, which is what dcOwnerOfMatchesOnWindows was).
        let owner = ws.owner_of(&nested.join(".tracking").join("x.sysml"));
        assert_eq!(
            owner.map(|p| ws.label(p)).as_deref(),
            Some("sub"),
            "the nested project owns its own files even though its parent is also a project"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
    /// issue276: the two rules that make the unowned surface a verdict rather than a printed note.
    #[test]
    fn the_unowned_surface_is_under_the_keystone_lock() {
        // Rule 1: the ONE repo-root hook that gates every project, and the root workflows dir.
        let unowned = vec![
            ".githooks/pre-commit".to_string(),
            ".github/workflows/keel-gate.yml".to_string(),
            "README.md".to_string(),          // workspace-level but NOT locked
            "scripts/build.sh".to_string(),   // ditto
        ];
        let events = unowned_keystone_events(&unowned, &[]);
        assert_eq!(events.len(), 2, "exactly the locked pair, got {events:?}");
        assert!(events.iter().any(|e| e.contains(".githooks/pre-commit")), "{events:?}");
        assert!(events.iter().any(|e| e.contains("keel-gate.yml")), "{events:?}");

        // Rule 2 is NOT a special case of rule 1: deleting a project makes its directory stop being a
        // project, so its engine paths become unowned at exactly the moment they are removed. They do
        // not match the root-relative locked predicate, which is how 445 staged deletions passed.
        let deleted = vec![
            "beta/.engine/processes/delivery.sysml".to_string(),
            ".engine/guards/x.sysml".to_string(),
            "beta/.tracking/backlog.sysml".to_string(), // tracking is instance data, not a control
            "docs/readme.md".to_string(),
        ];
        let events = unowned_keystone_events(&[], &deleted);
        assert_eq!(events.len(), 2, "both engine deletions, at either depth: {events:?}");
        assert!(events.iter().all(|e| e.contains("DELETED engine path")), "{events:?}");

        // A commit touching neither is not a keystone event, or every commit would need a signature.
        assert!(unowned_keystone_events(&["README.md".to_string()], &["docs/x.md".to_string()]).is_empty());
    }
}
