//! The content fingerprint of the model files — ONE home, memoized per epoch (issue142/issue145).
//!
//! WHY THIS MODULE EXISTS. The fingerprint answers "did any `.sysml` file change?" by folding
//! (path, len, mtime) over `.tracking` and `.engine`. It costs a recursive `read_dir` per tree plus a
//! `metadata()` per file — about 40ms and 582 stats on this corpus. That was fine when it was called
//! once. It is not fine at the rate it was actually being called: instrumentation measured `keel
//! assured` computing it 37 times in one command, 1472ms of fingerprinting to avoid a 160ms parse,
//! 21,534 stats and 74 tree walks. THE CACHE KEY COST NINE TIMES WHAT THE CACHE SAVED.
//!
//! It also existed TWICE — identical logic in `view::Model::fingerprint` and `serve::fingerprint`,
//! with a comment in one saying "same shape as serve's". Two copies of a predicate is the dual truth
//! §1 forbids, and it meant the server and the view layer could disagree about whether a file changed.
//!
//! HOW THE MEMO STAYS CORRECT, which is the condition the human attached to authorising a cache
//! ("max cache is just fine, if there's a good mechanism to piece-wise update the cache when changes
//! are detected"): the memo is keyed by an EXPLICIT EPOCH, never by elapsed time. Nothing expires on a
//! timer and nothing is guessed.
//!
//!   - A CLI command is one-shot and read-only, so its whole process is ONE epoch: the fingerprint is
//!     computed once no matter how many views ask for it. There is no window in which a file could
//!     change and be missed, because the process ends.
//!   - In `serve`, the epoch advances on an OBSERVED CHANGE or a WRITE — never on a read (D0167).
//!     Bumping it per request meant the six requests of one page load each re-statted 604 files to ask
//!     a question none of them had changed the answer to; measured, a cache HIT cost 137–208ms of which
//!     the fingerprint was ALL of it. The single watcher task compares fingerprints every 1.5s and bumps
//!     when they DIFFER, and a non-GET bumps both before and after it runs, so the console's own acts are
//!     visible to the next read immediately. An observation is not elapsed time: nothing here expires on
//!     a timer, which was the condition attached to authorising the memo at all.
//!     THE COST, stated rather than buried: an edit made OUTSIDE the server is seen up to 1.5s late, and
//!     during that window a read is answered `computed` from the pre-edit value. That window is bounded
//!     by the change-detection interval, which already governed when a page could learn of an out-of-band
//!     edit — nothing tells the page to refetch sooner — so it is not a new blindness, only a relocated
//!     one. A write through the server has NO window.
//!   - The change DETECTOR must never read a memo — it is the thing detecting change. The watcher calls
//!     [`compute`] directly.
//!
//! The memo is keyed by root as well as epoch: one process may be asked about more than one tree, and
//! answering for the wrong one would be a correctness bug rather than a slow path.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

/// Bumped by anything that establishes a new point-in-time view of the tree. Ordering is `SeqCst`
/// because a bump must be visible to every thread before the request that caused it is served.
static EPOCH: AtomicU64 = AtomicU64::new(0);

/// `(epoch, root, fingerprint)` of the last computation.
static MEMO: std::sync::Mutex<Option<(u64, std::path::PathBuf, u64)>> = std::sync::Mutex::new(None);

/// Declare a new point in time: the next [`of`] call re-reads the tree.
///
/// In `serve`, called by the watcher when it OBSERVES a change and by the request middleware around a
/// non-GET (D0167) — never by a read, which is what keeps a cache hit from costing 604 stats. NOT called
/// by CLI commands: a one-shot read-only process is a single point in time by construction, which is what
/// makes one fingerprint per process correct rather than merely cheap.
pub fn new_epoch() {
    EPOCH.fetch_add(1, Ordering::SeqCst);
}

/// The current epoch - a counter with no meaning but ORDER. Read by the serve middleware's test to
/// observe that reads leave it alone and writes move it (D0167); nothing computes from its value.
#[must_use]
pub fn epoch() -> u64 {
    EPOCH.load(Ordering::SeqCst)
}

/// Serialises the fingerprint COMPUTE so a parallel cold burst does one walk (see [`of`]).
static COMPUTE: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The fingerprint of `root`, computed at most once per epoch.
pub fn of(root: &Path) -> u64 {
    let epoch = EPOCH.load(Ordering::SeqCst);
    if let Ok(g) = MEMO.lock() {
        if let Some((e, r, fp)) = g.as_ref() {
            if *e == epoch && r.as_path() == root {
                return *fp;
            }
        }
    }
    // Serialise the COMPUTE, not just the memo: with the guards running in parallel
    // (dcGuardsRunInParallelAndTimed) a cold burst of twenty callers used to compute twelve fingerprints
    // - 36 tree walks and 6.3 s of thread time where the serial loop paid one. The second look under the
    // lock is what turns the burst back into one walk.
    let _cl = COMPUTE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Ok(g) = MEMO.lock() {
        if let Some((e, r, fp)) = g.as_ref() {
            if *e == epoch && r.as_path() == root {
                return *fp;
            }
        }
    }
    let fp = crate::perf::timed(&crate::perf::FINGERPRINT_NANOS, || compute(root));
    if let Ok(mut g) = MEMO.lock() {
        *g = Some((epoch, root.to_path_buf(), fp));
    }
    fp
}

/// Compute the fingerprint, ALWAYS reading the tree. For change detection, which must not be memoized.
#[must_use]
pub fn compute(root: &Path) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for base in [".tracking", ".engine", ".knowledge"] {
        crate::perf::add(&crate::perf::TREES_WALKED, 1);
        let files = crate::collect_sysml_uncached(&root.join(base));
        crate::perf::add(&crate::perf::FILES_STATTED, files.len() as u64);
        for f in files {
            if let Ok(m) = std::fs::metadata(&f) {
                f.to_string_lossy().hash(&mut h);
                m.len().hash(&mut h);
                if let Ok(t) = m.modified() {
                    if let Ok(d) = t.duration_since(std::time::UNIX_EPOCH) {
                        d.as_nanos().hash(&mut h);
                    }
                }
            }
        }
    }
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The memo must not answer for a tree it was not asked about. Two roots in one epoch is the case
    /// a single-slot memo gets wrong, and getting it wrong returns another tree's answer.
    #[test]
    fn the_memo_is_keyed_by_root_not_only_by_epoch() {
        let a = Path::new(".");
        let b = Path::new("..");
        let fa = of(a);
        let fb = of(b);
        assert_eq!(of(a), fa, "asking again for the same root in one epoch must be stable");
        assert_ne!(fa, fb, "two different trees must not share one memoized fingerprint");
    }

    /// A new epoch must re-read. Verified by observing that the memo is not consulted across a bump,
    /// which is the property `serve` depends on for a write to be visible to the next request.
    #[test]
    fn a_new_epoch_invalidates_the_memo() {
        let root = Path::new(".");
        let first = of(root);
        new_epoch();
        assert_eq!(of(root), first, "an unchanged tree must fingerprint the same after a bump");
        let before = crate::perf::TREES_WALKED.load(Ordering::Relaxed);
        new_epoch();
        let _ = of(root);
        let after = crate::perf::TREES_WALKED.load(Ordering::Relaxed);
        if crate::perf::enabled() {
            assert!(after > before, "a bumped epoch must re-walk the tree rather than answer from memo");
        }
    }
}
