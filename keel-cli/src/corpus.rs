//! One process reads the corpus once (dcOneCorpusPerProcess, rank 3 of D0367, the corpus-reopen and
//! clone terms of issue409).
//!
//! WHY. Forty guards each walked the tree and reopened every `.sysml` file - 39 walks and some 45,000
//! opens per `keel gate guard` on a 1,160-file corpus whose bytes had not changed between one guard and the
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
//! THE PARSE IS CACHED THE SAME WAY (issue441). Two readers parsed the corpus in one process - the
//! model and the orient indexer - and under the parallel guard set the second parse of `.tracking`
//! cost 1.3-1.5 s on the critical path for an answer the first had already computed. [`parsed`]
//! serves the `Package` a file parsed to while its `(len, mtime)` still match, under the same racy
//! window; a file that fails to parse is never remembered, so every caller sees the failure it would
//! have seen.
//!
//! WHAT DOES NOT GO THROUGH HERE. The change DETECTOR - `fingerprint::compute` - walks with
//! [`crate::collect_sysml_uncached`]: the thing that detects change must not read a memo.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use crate::perf::{CORPUS_HITS, FILE_OPENS, PARSE_HITS, WALK_HITS};
use keel_parser::ast::Package;

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

struct Parsed {
    len: u64,
    mtime: SystemTime,
    package: Arc<Package>,
}

/// Why a file did not parse: the same three stages every caller distinguished by hand before.
#[derive(Debug)]
pub enum ParseFailure {
    Io(std::io::Error),
    Lex(String),
    Parse(String),
}

impl From<std::io::Error> for ParseFailure {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

static TEXTS: Mutex<Option<HashMap<PathBuf, Text>>> = Mutex::new(None);
static WALKS: Mutex<Option<HashMap<PathBuf, Walk>>> = Mutex::new(None);
static PACKAGES: Mutex<Option<HashMap<PathBuf, Parsed>>> = Mutex::new(None);

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
    read_keyed(path.as_ref()).map(|(_, _, body)| body.to_string())
}

/// The text with the `(len, mtime)` it was served under - the key the parse cache shares.
fn read_keyed(path: &Path) -> std::io::Result<(u64, SystemTime, Arc<str>)> {
    let meta = std::fs::metadata(path)?;
    let len = meta.len();
    let mtime = meta.modified()?;
    if settled(mtime) {
        let g = TEXTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(t) = g.as_ref().and_then(|m| m.get(path)) {
            if t.len == len && t.mtime == mtime {
                CORPUS_HITS.fetch_add(1, Ordering::Relaxed);
                return Ok((len, mtime, Arc::clone(&t.body)));
            }
        }
    }
    // Counted unconditionally, not behind KEEL_PERF: the open count is the receipt the DoD's test reads.
    FILE_OPENS.fetch_add(1, Ordering::Relaxed);
    let text = std::fs::read_to_string(path)?;
    let body: Arc<str> = Arc::from(text.as_str());
    TEXTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get_or_insert_with(HashMap::new)
        .insert(path.to_path_buf(), Text { len, mtime, body: Arc::clone(&body) });
    Ok((len, mtime, body))
}

/// The `Package` a `.sysml` file parses to, from the process cache while its `(len, mtime)` still match
/// what was parsed. A shared handle, never a clone of the tree.
///
/// The file name the lexer and parser are given is the path as displayed, which is what both former
/// call sites passed; it reaches only positions and error text.
///
/// # Errors
/// [`ParseFailure`] names the stage: the read, the lexer, or the parser. A failure is never cached.
pub fn parsed(path: &Path) -> Result<Arc<Package>, ParseFailure> {
    let (len, mtime, body) = read_keyed(path)?;
    if settled(mtime) {
        let g = PACKAGES.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(p) = g.as_ref().and_then(|m| m.get(path)) {
            if p.len == len && p.mtime == mtime {
                PARSE_HITS.fetch_add(1, Ordering::Relaxed);
                return Ok(Arc::clone(&p.package));
            }
        }
    }
    let fname = path.display().to_string();
    let tokens = keel_parser::tokenize(&body, &fname).map_err(|e| ParseFailure::Lex(e.to_string()))?;
    let package = Arc::new(keel_parser::parse(tokens, &fname).map_err(|e| ParseFailure::Parse(e.to_string()))?);
    PACKAGES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get_or_insert_with(HashMap::new)
        .insert(path.to_path_buf(), Parsed { len, mtime, package: Arc::clone(&package) });
    Ok(package)
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
    ///
    /// The witness is the cached body itself: a read served from the cache hands back the SAME
    /// `Arc<str>` allocation, a read that opened the file a new one. `FILE_OPENS` is process-global,
    /// and cargo runs the lib's tests in parallel, so a counter equality across two calls fails
    /// whenever an unrelated test opens a file in the gap - which it did on 2026-09-10 once two
    /// more tests walked `.engine/processes` (issue466).
    #[test]
    fn a_second_read_of_an_unchanged_file_does_not_open_it_and_a_rewrite_does() {
        let d = fresh_dir("reads");
        let f = d.join("a.sysml");
        std::fs::write(&f, "package A;").expect("write");
        age(&f);
        let (_, _, first) = read_keyed(&f).expect("read");
        assert_eq!(&*first, "package A;");
        let (_, _, second) = read_keyed(&f).expect("read");
        assert!(Arc::ptr_eq(&first, &second), "the second read of an unchanged settled file must be served from the cache");
        // Rewritten: different length, and a fresh mtime that is also inside the racy window - both
        // roads lead to a re-read.
        std::fs::write(&f, "package A; part x;").expect("rewrite");
        let (_, _, third) = read_keyed(&f).expect("read");
        assert_eq!(&*third, "package A; part x;");
        assert!(!Arc::ptr_eq(&second, &third), "a rewritten file is re-read");
        // Rewritten to the SAME length with an aged mtime: the (len, mtime) test alone would be fooled
        // by an equal mtime, so the entry's mtime must differ from the aged one - it does, because we
        // age to a different instant than the cached read saw.
        std::fs::write(&f, "package A; part y;").expect("rewrite same len");
        age(&f);
        assert_eq!(read_to_string(&f).expect("read"), "package A; part y;");
        let _ = std::fs::remove_dir_all(&d);
    }

    /// issue441: a second parse of an unchanged file is the same tree (one handle, no re-lex); a
    /// rewritten file is parsed again; a file that does not parse is an error every time, never a hit.
    #[test]
    fn a_second_parse_of_an_unchanged_file_shares_the_tree_and_a_rewrite_reparses() {
        let d = fresh_dir("parsed");
        let f = d.join("p.sysml");
        std::fs::write(&f, "package P { part x { attribute a = 1; } }").expect("write");
        age(&f);
        let first = parsed(&f).expect("parse");
        let hits0 = PARSE_HITS.load(Ordering::Relaxed);
        let second = parsed(&f).expect("parse again");
        assert!(Arc::ptr_eq(&first, &second), "an unchanged settled file must serve the same tree");
        assert!(PARSE_HITS.load(Ordering::Relaxed) > hits0, "the second parse is a hit");
        std::fs::write(&f, "package P { part x { attribute a = 1; } part y { attribute a = 2; } }").expect("rewrite");
        age(&f);
        let third = parsed(&f).expect("reparse");
        assert!(!Arc::ptr_eq(&first, &third), "a rewritten file is parsed again");
        assert_eq!(third.items.len(), 2, "and the new tree is the rewritten file's");
        std::fs::write(&f, "package P { part x { ").expect("break it");
        age(&f);
        assert!(matches!(parsed(&f), Err(ParseFailure::Parse(_) | ParseFailure::Lex(_))), "a broken file fails");
        assert!(matches!(parsed(&f), Err(ParseFailure::Parse(_) | ParseFailure::Lex(_))), "and fails again: never cached");
        std::fs::remove_file(&f).expect("remove");
        assert!(matches!(parsed(&f), Err(ParseFailure::Io(_))), "a removed file is an io error, not a hit");
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
