//! Content-addressed cache of the facts orient asks git about a commit (dcGitFactsAreContentAddressed,
//! rank 2 of D0367, the spawn term of issue409).
//!
//! WHY. On the spike's host `keel show orient` spent 3.1 of its 5.2 s in 23 git spawns, and 14 of those asked
//! questions whose answers are facts about an IMMUTABLE commit: is `<sha>` a commit, what did a task's
//! `DoD` say in a given file at `<sha>`, what does `git grep` find for it at `<sha>`, which paths differ
//! between `<sha>` and `<head>`. The same tree asked the same questions at every turn boundary and paid a
//! process spawn for each answer it had already had.
//!
//! WHAT MAKES THIS HONEST. A fact is cached only under a FULL 40-hex commit id, because a short id's
//! resolution can change as history grows and a ref's target moves; a negative is-commit answer is
//! never cached, because a fetch can make it true tomorrow. Everything else here is a pure function of
//! (commit, question) and cannot go stale - which is why the cache has no age, no eviction and no
//! invalidation: an entry is either the answer or it is not there.
//!
//! SHORT IDS. Nine in ten `judgedAgainst` anchors in this tree are 7-hex. Their resolution is NEVER read
//! from the cache: git resolves them fresh in every process - in the one `cat-file --batch-check` orient
//! already runs to validate them, whose output carries the full oid - and [`remember_resolution`] holds
//! the mapping in memory for that process only. Every question then keys by the full id, so a short
//! id costs one spawn per process instead of one per question, and nothing on disk depends on it.
//!
//! WHERE. `.keel/cache/git-facts.toml`, machine-local and gitignored with the rest of `.keel/`, written
//! temp-then-rename. It is a cache, not truth (§1): delete it and every answer is recomputed identically -
//! the `DoD`'s check is `keel show orient` byte-identical with the file absent, cold and warm.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The on-disk shape. Absent criteria are a separate list because TOML has no null.
#[derive(serde::Serialize, serde::Deserialize, Default)]
struct FileShape {
    /// Full sha -> true. Positives only.
    #[serde(default)]
    is_commit: BTreeMap<String, bool>,
    /// Full sha -> `file::task` -> the `DoD` criterion text found in that file at that commit.
    #[serde(default)]
    criterion: BTreeMap<String, BTreeMap<String, String>>,
    /// Full sha -> the `file::task` keys whose file at that commit holds no such criterion.
    #[serde(default)]
    criterion_absent: BTreeMap<String, Vec<String>>,
    /// Full sha -> task -> what `git grep` found for the task's `DoD` anywhere in `.tracking` at that commit.
    #[serde(default)]
    grep: BTreeMap<String, BTreeMap<String, String>>,
    /// Full sha -> the tasks `git grep` found nothing for at that commit.
    #[serde(default)]
    grep_absent: BTreeMap<String, Vec<String>>,
    /// Full sha -> full head sha -> the paths `git diff --name-only sha..head` lists.
    #[serde(default)]
    changed: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    /// Result `id` -> the full sha of the commit that INTRODUCED it (dcResultBindsToItsLandingCommit).
    /// Immutable under D0129: history is never rewritten, so the commit that first added a line is a
    /// fact about the id forever.
    #[serde(default)]
    introduced: BTreeMap<String, String>,
    /// The full HEAD sha up to which every `.tracking` commit has been walked for introduced ids; the
    /// next walk reads only `walked..HEAD`. Absent means never walked.
    #[serde(default)]
    introduced_walked_to: Option<String>,
    /// The version of the criterion READER that wrote `criterion` / `grep`. A text is a fact about a
    /// commit only through the reader that extracted it: when the reader changes (it learned to decode
    /// escapes, dcResultBindsToItsLandingCommit), every text it wrote before is dropped on load rather
    /// than served as the old reader's answer. Absent means the first reader.
    #[serde(default)]
    criterion_reader: u32,
}

/// Bump when the criterion extraction changes what it returns for the same blob. 1: cut at the first
/// `"`, no unescape. 2: read as the lexer does (`orient::string_literal_body`).
const CRITERION_READER: u32 = 2;

struct Facts {
    root: PathBuf,
    shape: FileShape,
    dirty: bool,
}

static FACTS: std::sync::Mutex<Option<Facts>> = std::sync::Mutex::new(None);

/// `(root, short-or-full id -> full id)` as git resolved them IN THIS PROCESS. Never persisted.
static RESOLVED: std::sync::Mutex<Option<(PathBuf, std::collections::HashMap<String, String>)>> =
    std::sync::Mutex::new(None);

/// Remember that git resolved `query` to the full commit id `full` in this process. A `full` that is
/// not 40-hex is ignored.
pub fn remember_resolution(root: &Path, query: &str, full: &str) {
    if !is_full_sha(full) {
        return;
    }
    let mut g = RESOLVED.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if g.as_ref().is_none_or(|(r, _)| r != root) {
        *g = Some((root.to_path_buf(), std::collections::HashMap::new()));
    }
    if let Some((_, m)) = g.as_mut() {
        m.insert(query.to_string(), full.to_string());
    }
}

/// The full id for `sha`: itself when already full, else what git resolved it to in this process,
/// else `None` - and a `None` means the cache neither answers nor remembers.
#[must_use]
pub fn full_id(root: &Path, sha: &str) -> Option<String> {
    if is_full_sha(sha) {
        return Some(sha.to_string());
    }
    let g = RESOLVED.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    g.as_ref().filter(|(r, _)| r == root).and_then(|(_, m)| m.get(sha).cloned())
}

/// A full 40-hex commit id - the only key this cache accepts.
#[must_use]
pub fn is_full_sha(s: &str) -> bool {
    s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

fn path_for(root: &Path) -> PathBuf {
    root.join(".keel").join("cache").join("git-facts.toml")
}

fn load(root: &Path) -> Facts {
    let mut shape = std::fs::read_to_string(path_for(root))
        .ok()
        .and_then(|t| toml::from_str::<FileShape>(&t).ok())
        .unwrap_or_default();
    let dirty = if shape.criterion_reader < CRITERION_READER {
        shape.criterion.clear();
        shape.criterion_absent.clear();
        shape.grep.clear();
        shape.grep_absent.clear();
        shape.criterion_reader = CRITERION_READER;
        true
    } else {
        false
    };
    Facts { root: root.to_path_buf(), shape, dirty }
}

/// Run `f` against the facts for `root`, loading the file once per process (a second root reloads).
fn with<T>(root: &Path, f: impl FnOnce(&mut Facts) -> T) -> T {
    let mut g = FACTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let reload = g.as_ref().is_none_or(|fx| fx.root != root);
    if reload {
        *g = Some(crate::perf::phase("gitfacts:load", || load(root)));
    }
    g.as_mut().map_or_else(|| unreachable!("the slot was filled just above"), f)
}

/// Write the facts back if anything was learned. Called at the end of each orient step that asked git;
/// a failure to write is a cache that did not fill, never an error for the caller.
pub fn flush(root: &Path) {
    with(root, |fx| {
        if !fx.dirty {
            return;
        }
        let Ok(text) = toml::to_string(&fx.shape) else { return };
        let path = path_for(&fx.root);
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if crate::perf::phase("gitfacts:flush", || crate::write::write_atomic(&path, text)).is_ok() {
            fx.dirty = false;
        }
    });
}

/// Is `sha` a commit this cache has already seen confirmed? A `false` means "not known", never "not a commit".
#[must_use]
pub fn commit_known(root: &Path, sha: &str) -> bool {
    is_full_sha(sha) && with(root, |fx| fx.shape.is_commit.get(sha).copied().unwrap_or(false))
}

/// Remember that `sha` IS a commit. A short id or a negative is ignored (see the module doc).
pub fn remember_commit(root: &Path, sha: &str, is_commit: bool) {
    if !is_full_sha(sha) || !is_commit {
        return;
    }
    with(root, |fx| {
        if fx.shape.is_commit.insert(sha.to_string(), true).is_none() {
            fx.dirty = true;
        }
    });
}

fn ft_key(file: &str, task: &str) -> String {
    format!("{file}::{task}")
}

/// The `DoD` criterion of `task` as `file` held it at `sha`: `Some(Some(text))` found, `Some(None)` the
/// file at that commit has no such criterion, `None` not cached.
#[must_use]
pub fn criterion(root: &Path, sha: &str, file: &str, task: &str) -> Option<Option<String>> {
    let sha = &full_id(root, sha)?;
    let key = ft_key(file, task);
    with(root, |fx| {
        if let Some(t) = fx.shape.criterion.get(sha).and_then(|m| m.get(&key)) {
            return Some(Some(t.clone()));
        }
        if fx.shape.criterion_absent.get(sha).is_some_and(|v| v.iter().any(|k| k == &key)) {
            return Some(None);
        }
        None
    })
}

/// Remember what `file` at `sha` said about `task`'s criterion.
pub fn remember_criterion(root: &Path, sha: &str, file: &str, task: &str, text: Option<&str>) {
    let Some(sha) = full_id(root, sha) else { return };
    let sha = sha.as_str();
    let key = ft_key(file, task);
    with(root, |fx| {
        if let Some(t) = text {
            fx.shape.criterion.entry(sha.to_string()).or_default().insert(key, t.to_string());
        } else {
            let v = fx.shape.criterion_absent.entry(sha.to_string()).or_default();
            if !v.iter().any(|k| k == &key) {
                v.push(key);
            }
        }
        fx.dirty = true;
    });
}

/// What `git grep` found for `task`'s `DoD` at `sha` (the moved-file fallback), same tri-state as [`criterion`].
#[must_use]
pub fn grep(root: &Path, sha: &str, task: &str) -> Option<Option<String>> {
    let sha = &full_id(root, sha)?;
    with(root, |fx| {
        if let Some(t) = fx.shape.grep.get(sha).and_then(|m| m.get(task)) {
            return Some(Some(t.clone()));
        }
        if fx.shape.grep_absent.get(sha).is_some_and(|v| v.iter().any(|k| k == task)) {
            return Some(None);
        }
        None
    })
}

/// Remember the grep fallback's answer for `task` at `sha`.
pub fn remember_grep(root: &Path, sha: &str, task: &str, text: Option<&str>) {
    let Some(sha) = full_id(root, sha) else { return };
    let sha = sha.as_str();
    with(root, |fx| {
        if let Some(t) = text {
            fx.shape.grep.entry(sha.to_string()).or_default().insert(task.to_string(), t.to_string());
        } else {
            let v = fx.shape.grep_absent.entry(sha.to_string()).or_default();
            if !v.iter().any(|k| k == task) {
                v.push(task.to_string());
            }
        }
        fx.dirty = true;
    });
}

/// The paths `git diff --name-only sha..head` lists, if cached. Both ids must be full.
#[must_use]
pub fn changed(root: &Path, sha: &str, head: &str) -> Option<Vec<String>> {
    if !is_full_sha(head) {
        return None;
    }
    let sha = &full_id(root, sha)?;
    with(root, |fx| fx.shape.changed.get(sha).and_then(|m| m.get(head)).cloned())
}

/// Remember the diff between two full commit ids - and keep the rows of THIS head only.
///
/// The fact is immutable, but its key moves: `head` is HEAD, and under D0129 HEAD only advances, so a
/// row written under the last commit is never asked for again on this clone. Left in place the section
/// grew by one full path list per verified commit per HEAD - 24 of the file's 27 MB after fourteen
/// commits, loaded and parsed by every process that asked the cache anything, 416 ms per hook fire
/// and rising (issue440). A checkout of an older commit misses and recomputes the identical answer.
pub fn remember_changed(root: &Path, sha: &str, head: &str, paths: &[String]) {
    if !is_full_sha(head) {
        return;
    }
    let Some(sha) = full_id(root, sha) else { return };
    let sha = sha.as_str();
    with(root, |fx| {
        for rows in fx.shape.changed.values_mut() {
            rows.retain(|h, _| h == head);
        }
        fx.shape.changed.retain(|_, rows| !rows.is_empty());
        fx.shape.changed.entry(sha.to_string()).or_default().insert(head.to_string(), paths.to_vec());
        fx.dirty = true;
    });
}

/// The commit that introduced result `id`, if a walk has seen it.
#[must_use]
pub fn introduced(root: &Path, id: &str) -> Option<String> {
    with(root, |fx| fx.shape.introduced.get(id).cloned())
}

/// The full HEAD sha the introduced-id walk last reached, if any.
#[must_use]
pub fn introduced_walked_to(root: &Path) -> Option<String> {
    with(root, |fx| fx.shape.introduced_walked_to.clone())
}

/// Remember the ids a walk found, and the full HEAD it reached. An id already known keeps its first
/// (oldest) commit: a later commit that re-adds the same line moved it, it did not introduce it.
pub fn remember_introduced(root: &Path, found: &[(String, String)], walked_to: &str) {
    if !is_full_sha(walked_to) {
        return;
    }
    with(root, |fx| {
        for (id, sha) in found {
            if is_full_sha(sha) {
                fx.shape.introduced.entry(id.clone()).or_insert_with(|| sha.clone());
            }
        }
        fx.shape.introduced_walked_to = Some(walked_to.to_string());
        fx.dirty = true;
    });
}

/// HEAD's full commit id, one spawn per process. `None` when git cannot answer - callers then skip the
/// cache, never guess a head.
#[must_use]
pub fn head_sha(root: &Path) -> Option<String> {
    static HEAD: std::sync::Mutex<Option<(PathBuf, Option<String>)>> = std::sync::Mutex::new(None);
    let mut g = HEAD.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((r, h)) = g.as_ref() {
        if r == root {
            return h.clone();
        }
    }
    let h = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| is_full_sha(s));
    *g = Some((root.to_path_buf(), h.clone()));
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL: &str = "0123456789abcdef0123456789abcdef01234567";

    fn fresh_root(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("keel-gitfacts-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("root");
        root
    }

    /// The cache tests share one process-global slot, so they run under one lock and each begins by
    /// pointing the slot at its own root (a different root reloads).
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// dcGitFactsAreContentAddressed: an entry keyed by a full sha is written, survives a reload from
    /// disk, and answers the same question; a short-sha query is never written.
    #[test]
    fn a_full_sha_entry_round_trips_and_a_short_sha_is_never_written() {
        let _s = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = fresh_root("roundtrip");
        remember_criterion(&root, FULL, ".tracking/backlog.sysml", "dcX", Some("DONE MEANS: x"));
        remember_criterion(&root, FULL, ".tracking/backlog.sysml", "dcGone", None);
        remember_grep(&root, FULL, "dcMoved", Some("moved text"));
        remember_commit(&root, FULL, true);
        remember_changed(&root, FULL, FULL, &["a/b.rs".to_string()]);
        // Short ids and negatives: refused silently - unless git resolved the short id in this process,
        // in which case the fact lands under the FULL key and the short id is never on disk.
        remember_criterion(&root, "0123456", "f", "dcShort", Some("no"));
        remember_resolution(&root, "fedcba9", FULL);
        remember_criterion(&root, "fedcba9", "f", "dcResolved", Some("via short"));
        remember_resolution(&root, "1234567", "not-a-full-id");
        remember_criterion(&root, "1234567", "f", "dcBadResolution", Some("no"));
        remember_commit(&root, FULL.replace('0', "f").as_str(), false);
        flush(&root);
        let text = std::fs::read_to_string(path_for(&root)).expect("cache file written");
        assert!(text.contains(FULL) && text.contains("DONE MEANS: x"), "{text}");
        assert!(!text.contains("dcShort"), "a short sha must never be written: {text}");
        assert!(!text.contains("dcBadResolution"), "a resolution that is not a full id must not be trusted: {text}");
        assert!(text.contains("dcResolved") && !text.contains("fedcba9"), "a resolved short id is keyed by its FULL id: {text}");
        assert!(!text.contains(&FULL.replace('0', "f")), "a negative is-commit must never be written: {text}");

        // Reload from disk by pointing the slot elsewhere and back.
        let other = fresh_root("other");
        assert!(!commit_known(&other, FULL));
        assert_eq!(criterion(&root, FULL, ".tracking/backlog.sysml", "dcX"), Some(Some("DONE MEANS: x".to_string())));
        assert_eq!(criterion(&root, FULL, ".tracking/backlog.sysml", "dcGone"), Some(None));
        assert_eq!(criterion(&root, FULL, ".tracking/backlog.sysml", "dcNever"), None);
        assert_eq!(grep(&root, FULL, "dcMoved"), Some(Some("moved text".to_string())));
        assert!(commit_known(&root, FULL));
        assert_eq!(changed(&root, FULL, FULL), Some(vec!["a/b.rs".to_string()]));
        assert_eq!(criterion(&root, "0123456", "f", "dcShort"), None);
        assert_eq!(criterion(&root, "fedcba9", "f", "dcResolved"), Some(Some("via short".to_string())));
        assert_eq!(criterion(&root, FULL, "f", "dcResolved"), Some(Some("via short".to_string())));
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&other);
    }

    /// issue440 (dcChangedCacheKeepsOneHead): the changed-paths section keeps the rows of the head it was
    /// last asked under and drops every earlier head's - HEAD only advances (D0129), so a row keyed by a
    /// past HEAD is never read again on this clone, and left in place the section grew by one path list
    /// per verified commit per HEAD (24 of 27 MB after fourteen commits, parsed by every hook fire).
    #[test]
    fn changed_rows_of_an_earlier_head_are_dropped_when_a_new_head_is_remembered() {
        let _s = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = fresh_root("onehead");
        let head1 = FULL;
        let head2 = FULL.replace('0', "e");
        let sha_a = FULL.replace('0', "a");
        let sha_b = FULL.replace('0', "b");
        remember_changed(&root, &sha_a, head1, &["a.rs".to_string()]);
        remember_changed(&root, &sha_b, head1, &["b.rs".to_string()]);
        assert_eq!(changed(&root, &sha_a, head1), Some(vec!["a.rs".to_string()]));
        // A second row under the SAME head keeps the first: the head is what moved, not the sha.
        assert_eq!(changed(&root, &sha_b, head1), Some(vec!["b.rs".to_string()]));
        // HEAD advances: the first row written under the new head drops every row of the old one.
        remember_changed(&root, &sha_a, &head2, &["a2.rs".to_string()]);
        assert_eq!(changed(&root, &sha_a, &head2), Some(vec!["a2.rs".to_string()]));
        assert_eq!(changed(&root, &sha_a, head1), None, "the earlier head's row must be gone");
        assert_eq!(changed(&root, &sha_b, head1), None, "every earlier-head row must be gone, not only this sha's");
        flush(&root);
        let text = std::fs::read_to_string(path_for(&root)).expect("cache file written");
        assert!(text.contains("a2.rs") && !text.contains("\"a.rs\"") && !text.contains("b.rs"), "{text}");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// An unreadable or corrupt cache file is an EMPTY cache, never an error: the answers are recomputed.
    #[test]
    fn a_corrupt_cache_file_reads_as_empty() {
        let _s = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = fresh_root("corrupt");
        std::fs::create_dir_all(root.join(".keel").join("cache")).expect("dir");
        std::fs::write(path_for(&root), "this is not toml = = =").expect("write");
        assert!(!commit_known(&root, FULL));
        assert_eq!(criterion(&root, FULL, "f", "t"), None);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn full_sha_is_forty_hex() {
        assert!(is_full_sha(FULL));
        assert!(!is_full_sha("0123456"));
        assert!(!is_full_sha(&format!("{}g", &FULL[..39])));
        assert!(!is_full_sha("HEAD"));
    }
}
