//! `keel library` — the machine-local cache of the author's portable-content repository (D0250).
//!
//! # The shape, in one paragraph
//!
//! The library is a GIT REPOSITORY the author owns (D0250 clause A, chosen by the human: st029
//! "#1 yeah i think repo for sure"). Each machine holds a CLONE under `<home>/.keel/library`,
//! beside the console registry and the fire-ledger — machine-local state outside every project.
//! The clone is a CACHE and never a source: `sync` is fetch + fast-forward ONLY (clause B), an
//! unreachable remote is a STATED staleness over the last-good cache (clause C), and nothing a
//! project's gate reads lives here (clause F — availability is never activation, and the
//! differential test in `library_read_side.rs` holds that property, byte-for-byte).
//!
//! # Discovered, not declared
//!
//! There is no library config file. The clone IS the state, and its `origin` remote IS the declared
//! remote — the D0245 console-registry rule applied again: a second place to keep the list true is
//! a second place for it to be false.
//!
//! # Why divergence is a defect and AHEAD is not
//!
//! Downstream, the cache is read-only, so a non-fast-forward means something wrote to the clone
//! outside the sanctioned publish path — the fork-your-local-copy move that ends the parity story,
//! reported loudly rather than merged quietly. But a clone AHEAD of the remote is the normal state
//! after `keel process publish` (D0250 clause D): local commits awaiting an explicit push. Sync
//! tolerates ahead, refuses diverged.

use std::path::{Path, PathBuf};

/// `<home>/.keel/library` — the machine-local clone. Same home resolution as the console registry.
#[must_use]
pub fn clone_dir() -> Option<PathBuf> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
    Some(Path::new(&home).join(".keel").join("library"))
}

fn git_in(dir: &Path, args: &[&str]) -> Result<String, String> {
    let out = crate::gitx::git()
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|e| format!("git failed to run: {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

/// `keel library init <remote>` — clone the author's library onto this machine.
#[must_use]
pub fn cmd_init(remote: &str) -> i32 {
    let Some(dst) = clone_dir() else {
        eprintln!("library: no home directory resolvable (USERPROFILE/HOME) — machine-local state needs one");
        return 2;
    };
    if dst.join(".git").exists() {
        eprintln!("library: already initialised at {} — `keel library sync` refreshes it", dst.display());
        return 2;
    }
    if let Some(parent) = dst.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match crate::gitx::git().args(["clone", "-q"]).arg(remote).arg(&dst).output() {
        Ok(o) if o.status.success() => {
            // An EMPTY remote gives the clone git's init.defaultBranch (often `master`), and the
            // first `push origin HEAD` then CREATES that branch upstream while a later default
            // lands on `main` — two branches, and `sync` (which fetches origin HEAD) serves the one
            // without the publishes. Found live stocking keel-lib. A non-empty remote is immune
            // (the clone tracks its real default), so only the empty-clone branch is renamed.
            let _ = crate::gitx::git().arg("-C").arg(&dst).args(["branch", "-m", "main"]).output();
            println!("library: cloned {} -> {}", remote, dst.display());
            0
        }
        Ok(o) => {
            eprintln!("library: clone failed: {}", String::from_utf8_lossy(&o.stderr).trim());
            1
        }
        Err(e) => {
            eprintln!("library: git failed to run: {e}");
            1
        }
    }
}

/// `keel library sync` — fetch + fast-forward only; ahead tolerated; diverged refused; offline stated.
#[must_use]
pub fn cmd_sync() -> i32 {
    let Some(dir) = clone_dir().filter(|d| d.join(".git").exists()) else {
        eprintln!("library: not initialised on this machine — `keel library init <remote>` first");
        return 2;
    };
    if git_in(&dir, &["fetch", "-q", "origin", "HEAD"]).is_err() {
        // Clause C: offline is a STATED fallback, never a block and never a silent pretence.
        let last = git_in(&dir, &["log", "-1", "--format=%ci %h"]).unwrap_or_else(|_| "unknown".into());
        println!("library: remote UNREACHABLE — serving the last-good cache, which is STALE as of its last commit: {last}");
        return 0;
    }
    let head = git_in(&dir, &["rev-parse", "HEAD"]).unwrap_or_default();
    let fetched = git_in(&dir, &["rev-parse", "FETCH_HEAD"]).unwrap_or_default();
    if head == fetched || git_in(&dir, &["merge-base", "--is-ancestor", "FETCH_HEAD", "HEAD"]).is_ok() {
        // Up to date, or AHEAD (unpushed publishes) — both fine, both quiet on success.
        return 0;
    }
    if git_in(&dir, &["merge-base", "--is-ancestor", "HEAD", "FETCH_HEAD"]).is_ok() {
        return match git_in(&dir, &["merge", "--ff-only", "-q", "FETCH_HEAD"]) {
            Ok(_) => 0, // clause B: silent when it succeeds
            Err(e) => {
                eprintln!("library: fast-forward failed unexpectedly: {e}");
                1
            }
        };
    }
    eprintln!("library: the cache has DIVERGED from the remote — something wrote to {} outside `keel process publish`.", dir.display());
    eprintln!("  The cache is read-only downstream, so this is a DEFECT to investigate, never a merge to perform (D0250 clause B).");
    eprintln!("  Inspect with git -C {} log --oneline --all; a disposable cache can simply be deleted and re-`library init`ed.", dir.display());
    1
}

/// A unit its upstream has RETIRED in the library (issue381 / GH#57, GH#58).
///
/// `keel process retire` writes `retired = true`, `retiredAt`, `retiredWhy` and optionally
/// `replacedBy` into the unit's `unit.toml`. A retired unit stays in the library as history - its
/// consumers can still read what they installed - but `import --from-library` refuses it, `list` and
/// `status` mark it, and the currency pass carries the mark. The case that needed it: decision-channel
/// v6 ships a script that records an acceptance in a hardcoded human's name, and its upstream deleted
/// the channel (D0291), so the unit could neither be republished from a landed tree nor left as-is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Retirement {
    pub at: String,
    pub why: String,
    pub replaced_by: Option<String>,
}

impl Retirement {
    /// One line for a list, a status or a refusal.
    #[must_use]
    pub fn line(&self) -> String {
        self.replaced_by.as_ref().map_or_else(|| format!("RETIRED {}: {}", self.at, self.why), |r| format!("RETIRED {}: {} Replaced by: {r}", self.at, self.why))
    }
}

/// The retirement recorded in a unit's `unit.toml`, or `None` for a live unit.
#[must_use]
pub fn retirement(unit_dir: &Path) -> Option<Retirement> {
    let text = std::fs::read_to_string(unit_dir.join("unit.toml")).ok()?;
    let v = text.parse::<toml::Value>().ok()?;
    if v.get("retired").and_then(toml::Value::as_bool) != Some(true) {
        return None;
    }
    let s = |k: &str| v.get(k).and_then(toml::Value::as_str).map(str::to_owned);
    Some(Retirement { at: s("retiredAt").unwrap_or_else(|| "?".into()), why: s("retiredWhy").unwrap_or_else(|| "(no reason recorded)".into()), replaced_by: s("replacedBy") })
}

/// `keel library list` — every unit in the cache, with its version, plus the cache's currency.
#[must_use]
pub fn cmd_list() -> i32 {
    let Some(dir) = clone_dir().filter(|d| d.join(".git").exists()) else {
        eprintln!("library: not initialised on this machine — `keel library init <remote>` first");
        return 2;
    };
    let mut rows: Vec<(String, String)> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let unit_toml = e.path().join("unit.toml");
            let Ok(text) = std::fs::read_to_string(&unit_toml) else { continue };
            let version = text
                .lines()
                .find_map(|l| l.trim().strip_prefix("version = "))
                .unwrap_or("?")
                .trim()
                .to_string();
            let mark = retirement(&e.path()).map_or_else(String::new, |r| format!("   {}", r.line()));
            rows.push((e.file_name().to_string_lossy().to_string(), format!("{version}{mark}")));
        }
    }
    rows.sort();
    let last = git_in(&dir, &["log", "-1", "--format=%ci %h"]).unwrap_or_else(|_| "unknown".into());
    println!("library at {} — {} unit(s), cache as of {last}:", dir.display(), rows.len());
    for (name, v) in rows {
        println!("  {name}  v{v}");
    }
    println!("  (import into a project with `keel process import --from-library <name>`)");
    0
}

/// Resolve a unit name against the cache for `process import --from-library` (clause A: the cache
/// serves reads; the import itself is the existing, single import path).
/// # Errors
/// A message when the library is uninitialised or the unit is absent — the caller prints it.
pub fn resolve_unit(name: &str) -> Result<PathBuf, String> {
    let dir = clone_dir()
        .filter(|d| d.join(".git").exists())
        .ok_or_else(|| "library: not initialised on this machine — `keel library init <remote>` first".to_string())?;
    let unit = dir.join(name);
    if unit.join("unit.toml").exists() {
        // issue381: a retired unit is history, not an offer - importing it would install the defect
        // its upstream retired it for.
        if let Some(r) = retirement(&unit) {
            return Err(format!("library: `{name}` is {} - not imported. `keel library list` shows what is live.", r.line()));
        }
        Ok(unit)
    } else {
        Err(format!(
            "library: no unit named `{name}` in the cache at {} — `keel library list` shows what it holds, `keel library sync` refreshes it",
            dir.display()
        ))
    }
}

/// The `keel library <init|sync|list>` dispatcher.
#[must_use]
pub fn run(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("init") => args.get(1).map_or_else(
            || {
                eprintln!("usage: keel library init <remote>");
                2
            },
            |remote| cmd_init(remote),
        ),
        Some("sync") => cmd_sync(),
        Some("list") => cmd_list(),
        _ => {
            eprintln!("usage: keel library <init <remote> | sync | list>");
            eprintln!("  the machine-local cache of your portable-content repository (D0250)");
            2
        }
    }
}
