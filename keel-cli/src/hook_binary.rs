//! The hook binary on a SELF-BUILD tree (D0391, issue408).
//!
//! A stable copy the hooks run, refreshed from the build output by the hooks themselves, so cargo
//! never has to unlink the file a hook is executing.
//!
//! THE DEFECT: the pin probe every hook command embeds resolves `KEEL_BIN`, then `.keel/bin/keel(.exe)`,
//! then the pin cache, then PATH. In a project whose deliverable IS the engine nothing writes
//! `.keel/bin/keel.exe` - `init` seeds `.keel/bin/<version>/<asset>` and `keelw` fetches into the same
//! shape - so the probe fell through to PATH and resolved into `target/release/keel.exe`, the exact file
//! cargo must remove to relink. `keel suite` then returned `fail - 0 passed, 0 failed` three runs in a
//! row over `failed to remove file target/release/keel.exe`, and three real regressions hid behind it.
//!
//! THE SHAPE: on a self-build tree every hook fire, before it does its own work, compares the build
//! output against the stamp of the copy it last placed; when the output is newer (and older than two
//! seconds, so a link in progress is never copied) it copies it beside itself through
//! `.next` -> rename, which Windows permits on a running image, and writes the stamp. The refresh rides
//! the build path because the first hook after a build is the build path's next event - no human
//! remembers anything. The cost this trades for is a copy that can lag the tree, so `keel status` names
//! the hook binary's build beside HEAD, and `keel sync-claude` says which binary the hooks will run.
//!
//! A non-self-build project has no `target/release/keel` and is untouched by every function here.

use std::path::{Path, PathBuf};

const BIN: &str = if cfg!(windows) { "keel.exe" } else { "keel" };

/// Where the hooks' stable copy lives - the pin probe's second stop.
#[must_use]
pub fn stable_copy(root: &Path) -> PathBuf {
    root.join(".keel").join("bin").join(BIN)
}

/// Where cargo writes the release binary - the file the build must be free to relink.
#[must_use]
pub fn build_output(root: &Path) -> PathBuf {
    root.join("target").join("release").join(BIN)
}

fn stamp_path(root: &Path) -> PathBuf {
    root.join(".keel").join("bin").join(format!("{BIN}.stamp"))
}

/// What identifies one build output: its length and modification time in seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stamp {
    pub len: u64,
    pub mtime: u64,
}

impl Stamp {
    fn of(path: &Path) -> Option<Self> {
        let m = std::fs::metadata(path).ok()?;
        let mtime = m.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs();
        Some(Self { len: m.len(), mtime })
    }
    fn render(self) -> String {
        format!("{} {}\n", self.len, self.mtime)
    }
    fn parse(text: &str) -> Option<Self> {
        let mut it = text.split_whitespace();
        Some(Self { len: it.next()?.parse().ok()?, mtime: it.next()?.parse().ok()? })
    }
}

/// The refresh decision, pure and tested: the build output exists, is at least `min_age_secs` old (a
/// link in progress is never copied), and differs from what the stable copy was last placed from.
#[must_use]
pub fn needs_refresh(output: Option<Stamp>, placed: Option<Stamp>, now_secs: u64, min_age_secs: u64) -> bool {
    let Some(out) = output else { return false };
    if now_secs.saturating_sub(out.mtime) < min_age_secs {
        return false;
    }
    placed != Some(out)
}

/// What a refresh did, for the ledger and the caller's one line.
#[derive(Debug)]
pub struct Refreshed {
    pub copy: PathBuf,
    pub from: PathBuf,
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// On a self-build tree, place or refresh the stable copy from the build output when it changed.
///
/// `None` when nothing was done - not a self-build, no build output yet, output too young, or the
/// copy is current. Best-effort by design: a failure is reported by the caller, never a blocked hook.
///
/// # Errors
/// When the copy cannot be written or placed; the caller prints it and the hook proceeds on the
/// image it has.
pub fn refresh(root: &Path) -> Result<Option<Refreshed>, String> {
    if !crate::suite::is_self_build(root) {
        return Ok(None);
    }
    let from = build_output(root);
    let output = Stamp::of(&from);
    let placed = std::fs::read_to_string(stamp_path(root)).ok().and_then(|t| Stamp::parse(&t));
    let copy = stable_copy(root);
    // a stamp with no copy behind it (someone deleted the copy) must not read as current
    let placed = if copy.is_file() { placed } else { None };
    if !needs_refresh(output, placed, now_secs(), 2) {
        return Ok(None);
    }
    let dir = copy.parent().ok_or("no .keel/bin parent")?;
    std::fs::create_dir_all(dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
    let next = dir.join(format!("{BIN}.next"));
    std::fs::copy(&from, &next).map_err(|e| format!("copy {} -> {}: {e}", from.display(), next.display()))?;
    if copy.exists() {
        // the copy may be the image running THIS hook: a rename is permitted where an overwrite is not
        let prev = dir.join(format!("{BIN}.prev"));
        let _ = std::fs::remove_file(&prev);
        let parked = if std::fs::rename(&copy, &prev).is_ok() { prev } else {
            let unique = dir.join(format!("{BIN}.prev-{}", now_secs()));
            std::fs::rename(&copy, &unique).map_err(|e| format!("park {}: {e}", copy.display()))?;
            unique
        };
        let _ = std::fs::remove_file(parked); // fails while the parked image runs; the next refresh sweeps it
    }
    std::fs::rename(&next, &copy).map_err(|e| format!("place {}: {e}", copy.display()))?;
    if let Some(s) = output {
        let _ = std::fs::write(stamp_path(root), s.render());
    }
    Ok(Some(Refreshed { copy, from }))
}

/// One line saying which binary the hooks will run here, for `sync-claude` and `status`.
#[must_use]
pub fn describe(root: &Path) -> Option<String> {
    if !crate::suite::is_self_build(root) {
        return None;
    }
    let copy = stable_copy(root);
    Some(if copy.is_file() {
        format!(
            "hook binary: {} (self-build stable copy, refreshed by the hooks from {} when the build changes; D0391)",
            copy.display(),
            build_output(root).display()
        )
    } else {
        format!(
            "hook binary: NO stable copy at {} - the pin probe falls through to PATH and the hooks run the file cargo must relink (issue408); the next hook fire or `keel sync-claude` places it from {}",
            copy.display(),
            build_output(root).display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{needs_refresh, Stamp};

    #[test]
    fn a_changed_output_older_than_the_grace_refreshes_and_nothing_else_does() {
        let a = Stamp { len: 10, mtime: 100 };
        let b = Stamp { len: 11, mtime: 200 };
        assert!(!needs_refresh(None, None, 1000, 2), "no build output: nothing to place");
        assert!(needs_refresh(Some(a), None, 1000, 2), "an output and no copy yet: place it");
        assert!(!needs_refresh(Some(a), Some(a), 1000, 2), "the copy was placed from this very output");
        assert!(needs_refresh(Some(b), Some(a), 1000, 2), "the output changed since the copy was placed");
        assert!(!needs_refresh(Some(b), Some(a), 201, 2), "an output one second old may still be linking");
        assert!(needs_refresh(Some(b), Some(a), 202, 2), "two seconds old is settled");
    }

    #[test]
    fn a_stamp_round_trips() {
        let s = Stamp { len: 12_345_678, mtime: 1_788_900_000 };
        assert_eq!(Stamp::parse(&s.render()), Some(s));
        assert_eq!(Stamp::parse("garbage"), None);
    }
}
