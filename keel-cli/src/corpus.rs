//! One process reads the corpus once (dcOneCorpusPerProcess, rank 3 of D0367, the corpus-reopen and
//! clone terms of issue409).
//!
//! WHY. Forty guards each walked the tree and reopened every `.sysml` file - 39 walks and some 45,000
//! opens per `keel guard` on a 1,160-file corpus whose bytes had not changed between one guard and the
//! next. On a Defender host the OPEN is the cost, not the bytes: real-time scanning runs on the open,
//! a metadata query does not. So this module trades an open for a stat.
//!
//! HOW IT STAYS CORRECT - by the file's own facts, never by the caller's discipline. Sprint 598's
//! retro named the class: a memo that was right only because its callers were serial. Here every
//! [`read_to_string`] stats the file and serves the cached text only when `(len, mtime)` are equal to
//! what was read, so a file rewritten inside the process is re-read and an unchanged one is not
//! reopened. [`collect_sysml`] serves a remembered walk only while every directory it walked still
//! carries the mtime it had, which is the fact a filesystem keeps about a directory's entries.
//!
//! THE RACY WINDOW, named. A file rewritten with the same length inside one mtime tick would pass the
//! test - git's "racy" case. So an entry whose mtime is within [`RACY`] of now is never served from the
//! cache: it is re-read (or re-walked) and the cost is one open for a file touched in the last two
//! seconds, which is the only file that could be lying.
//!
//! WHAT DOES NOT GO THROUGH HERE. The change DETECTOR - `fingerprint::compute` - walks with
//! [`crate::collect_sysml_uncached`]: the thing that detects change must not read a memo.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use crate::perf::{CORPUS_HITS, FILE_OPENS, WALK_HITS};

/// An mtime younger than this is not trusted to distinguish two writes (see the module doc).
const RACY: Duration = Duration::from_secs(2);

struct Text {
    len: u64,
    mtime: SystemTime,
    body: Arc<str>,
}

struct Walk {
    /// Every directory the walk entered, with the mtime it carried then.
    dirs: Vec<(PathBuf, SystemTime)>,
    files: Arc<Vec<PathBuf>>,
}

static TEXTS: Mutex<Option<HashMap<PathBuf, Text>>> = Mutex::new(None);
static WALKS: Mutex<Option<HashMap<PathBuf, Walk>>> = Mutex::new(None);

fn settled(mtime: SystemTime) -> bool {
    SystemTime::now().duration_since(mtime).is_ok_and(|age| age >= RACY)
}

/// Read a file's text, from the process cache when its `(len, mtime)` still match what was read.
///
/// Drop-in for `std::fs::read_to_string`.
///
/// # Errors
/// The same as `std::fs::read_to_string`'s: a missing file, a directory, or non-UTF-8 content. A
/// cached entry is never served for a path whose metadata cannot be read.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let path = path.as_ref();
    let meta = std::fs::metadata(path)?;
    let len = meta.len();
    let mtime = meta.modified()?;
    if settled(mtime) {
        let g = TEXTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(t) = g.as_ref().and_then(|m| m.get(path)) {
            if t.len == len && t.mtime == mtime {
                CORPUS_HITS.fetch_add(1, Ordering::Relaxed);
                return Ok(t.body.to_string());
            }
        }
    }
    // Counted unconditionally, not behind KEEL_PERF: the open count is the receipt the DoD's test reads.
    FILE_OPENS.fetch_add(1, Ordering::Relaxed);
    let text = std::fs::read_to_string(path)?;
    let entry = Text { len, mtime, body: Arc::from(text.as_str()) };
    TEXTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get_or_insert_with(HashMap::new)
        .insert(path.to_path_buf(), entry);
    Ok(text)
}

fn dir_mtime(dir: &Path) -> Option<SystemTime> {
    std::fs::metadata(dir).ok().filter(std::fs::Metadata::is_dir).and_then(|m| m.modified().ok())
}

/// Walk `dir` recording every directory entered with its mtime.
fn walk(dir: &Path, dirs: &mut Vec<(PathBuf, SystemTime)>, files: &mut Vec<PathBuf>) {
    let Some(mt) = dir_mtime(dir) else { return };
    dirs.push((dir.to_path_buf(), mt));
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk(&p, dirs, files);
        } else if p.extension().and_then(|e| e.to_str()) == Some("sysml") {
            files.push(p);
        }
    }
}

/// Every `.sysml` under `dir`, sorted - from the remembered walk while every directory it entered
/// still carries the mtime it had (and none is younger than [`RACY`]).
///
/// A `dir` that does not exist is never remembered, so its later creation is seen.
pub fn collect_sysml(dir: &Path) -> Vec<PathBuf> {
    {
        let g = WALKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(w) = g.as_ref().and_then(|m| m.get(dir)) {
            let current = w.dirs.iter().all(|(d, then)| dir_mtime(d).is_some_and(|now| now == *then && settled(now)));
            if current {
                WALK_HITS.fetch_add(1, Ordering::Relaxed);
                return w.files.to_vec();
            }
        }
    }
    crate::perf::add(&crate::perf::TREES_WALKED, 1);
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    walk(dir, &mut dirs, &mut files);
    files.sort();
    if !dirs.is_empty() {
        let mut g = WALKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        g.get_or_insert_with(HashMap::new)
            .insert(dir.to_path_buf(), Walk { dirs, files: Arc::new(files.clone()) });
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("keel-corpus-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("dir");
        d
    }

    /// Set a file's mtime well into the past so the racy window does not apply to it.
    fn age(path: &Path) {
        let old = SystemTime::now() - Duration::from_mins(10);
        let f = std::fs::OpenOptions::new().write(true).open(path).expect("open for utime");
        f.set_modified(old).expect("set mtime");
    }

    /// dcOneCorpusPerProcess: a second read of an unchanged file does not open it; a rewritten one does.
    #[test]
    fn a_second_read_of_an_unchanged_file_does_not_open_it_and_a_rewrite_does() {
        let d = fresh_dir("reads");
        let f = d.join("a.sysml");
        std::fs::write(&f, "package A;").expect("write");
        age(&f);
        let opens0 = FILE_OPENS.load(Ordering::Relaxed);
        assert_eq!(read_to_string(&f).expect("read"), "package A;");
        let opens1 = FILE_OPENS.load(Ordering::Relaxed);
        assert_eq!(read_to_string(&f).expect("read"), "package A;");
        let opens2 = FILE_OPENS.load(Ordering::Relaxed);
        assert!(opens1 > opens0, "the first read opens the file");
        assert_eq!(opens2, opens1, "the second read of an unchanged settled file must not open it");
        // Rewritten: different length, and a fresh mtime that is also inside the racy window - both
        // roads lead to a re-read.
        std::fs::write(&f, "package A; part x;").expect("rewrite");
        assert_eq!(read_to_string(&f).expect("read"), "package A; part x;");
        assert!(FILE_OPENS.load(Ordering::Relaxed) > opens2, "a rewritten file is re-read");
        // Rewritten to the SAME length with an aged mtime: the (len, mtime) test alone would be fooled
        // by an equal mtime, so the entry's mtime must differ from the aged one - it does, because we
        // age to a different instant than the cached read saw.
        std::fs::write(&f, "package A; part y;").expect("rewrite same len");
        age(&f);
        assert_eq!(read_to_string(&f).expect("read"), "package A; part y;");
        let _ = std::fs::remove_dir_all(&d);
    }

    /// The error surface is `std::fs::read_to_string`'s: a missing file is an error, not an empty hit.
    #[test]
    fn a_missing_file_is_an_error_even_after_it_was_cached() {
        let d = fresh_dir("missing");
        let f = d.join("gone.sysml");
        std::fs::write(&f, "package G;").expect("write");
        age(&f);
        let _ = read_to_string(&f).expect("first read");
        std::fs::remove_file(&f).expect("remove");
        assert!(read_to_string(&f).is_err(), "a removed file must not be served from the cache");
        let _ = std::fs::remove_dir_all(&d);
    }

    /// A file created after the first walk is seen: by the change detector's uncached walk in the same
    /// epoch, and by the memoized walk because its directory's mtime moved.
    #[test]
    fn a_file_created_after_the_first_walk_is_seen() {
        let d = fresh_dir("walk");
        let sub = d.join("sub");
        std::fs::create_dir_all(&sub).expect("sub");
        std::fs::write(sub.join("one.sysml"), "package One;").expect("write");
        let first = collect_sysml(&d);
        assert_eq!(first.len(), 1);
        std::fs::write(sub.join("two.sysml"), "package Two;").expect("write");
        assert_eq!(crate::collect_sysml_uncached(&d).len(), 2, "the detector's walk is never memoized");
        assert_eq!(collect_sysml(&d).len(), 2, "a directory whose mtime moved is re-walked");
        let _ = std::fs::remove_dir_all(&d);
    }

    /// A directory that does not exist is not remembered: creating it later is seen.
    #[test]
    fn an_absent_directory_is_not_remembered() {
        let d = fresh_dir("absent");
        let later = d.join("later");
        assert!(collect_sysml(&later).is_empty());
        std::fs::create_dir_all(&later).expect("mk");
        std::fs::write(later.join("x.sysml"), "package X;").expect("write");
        assert_eq!(collect_sysml(&later).len(), 1);
        let _ = std::fs::remove_dir_all(&d);
    }
}
