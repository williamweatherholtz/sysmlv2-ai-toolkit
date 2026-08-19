//! Opt-in performance instrumentation (issue142/dcSharedParsedModel).
//!
//! Built because three separate wall-clock inferences about this codebase were WRONG: a 33-second
//! endpoint that was not reproducible, a "git is 62% of coverage" attribution that changed the code
//! path it was measuring, and a "31% improvement" that was noise under five samples. A fourth guess
//! is not wanted. These counters report what the process ACTUALLY did — how many times the model was
//! built, how many of those hit the cache, and how the time split between KEYING the cache and
//! FILLING it — so the next optimisation is aimed at a measured cost.
//!
//! OFF unless `KEEL_PERF=1`, and the counters themselves are relaxed atomics: no allocation, no
//! locking, nothing on the hot path but an add. The report goes to STDERR so it can never contaminate
//! the JSON on stdout that every computed view emits.

use std::sync::atomic::{AtomicU64, Ordering};

/// Times `Model::build` was called at all — the number a caller controls.
pub static BUILD_CALLS: AtomicU64 = AtomicU64::new(0);
/// Of those, the ones served from `MODEL_CACHE`.
pub static CACHE_HITS: AtomicU64 = AtomicU64::new(0);
/// Nanoseconds spent computing the content fingerprint. Paid on EVERY call, cache hit included —
/// which is the whole reason this counter is separate from the parse.
pub static FINGERPRINT_NANOS: AtomicU64 = AtomicU64::new(0);
/// Nanoseconds spent in `build_uncached` — read, tokenize, parse, ingest.
pub static PARSE_NANOS: AtomicU64 = AtomicU64::new(0);
/// `metadata()` calls made while fingerprinting. The stat storm, counted rather than assumed.
pub static FILES_STATTED: AtomicU64 = AtomicU64::new(0);
/// Directory trees walked by `collect_sysml` — each one a recursive `read_dir` plus a sort.
pub static TREES_WALKED: AtomicU64 = AtomicU64::new(0);

/// `git` subprocesses spawned. A process spawn is the most expensive thing this program does on
/// Windows, and it is invisible in a wall-clock number — hence a counter rather than another inference.
pub static GIT_CALLS: AtomicU64 = AtomicU64::new(0);
/// Nanoseconds spent waiting on `git`.
pub static GIT_NANOS: AtomicU64 = AtomicU64::new(0);

/// Whether instrumentation is on. Read once; an env lookup per build call would itself be a cost.
#[must_use]
pub fn enabled() -> bool {
    static ON: std::sync::LazyLock<bool> =
        std::sync::LazyLock::new(|| std::env::var("KEEL_PERF").is_ok_and(|v| v == "1"));
    *ON
}

/// Add `n` to a counter, but only when instrumentation is on.
pub fn add(counter: &AtomicU64, n: u64) {
    if enabled() {
        counter.fetch_add(n, Ordering::Relaxed);
    }
}

/// Time `f`, adding its duration to `counter`. Returns `f`'s value either way.
pub fn timed<T>(counter: &AtomicU64, f: impl FnOnce() -> T) -> T {
    if !enabled() {
        return f();
    }
    let t0 = std::time::Instant::now();
    let out = f();
    counter.fetch_add(u64::try_from(t0.elapsed().as_nanos()).unwrap_or(u64::MAX), Ordering::Relaxed);
    out
}

/// The report, or `None` when instrumentation is off. Written to stderr by the caller at process end.
///
/// Deliberately reports the MISS COUNT rather than a hit RATE: a rate flatters a command that builds
/// the model fifty times and hits the cache forty-nine, when the honest finding is that it keyed the
/// cache fifty times to parse once.
/// Nanoseconds as whole milliseconds. Integer division, not a float cast: a report a human reads never
/// needs sub-millisecond precision, and `u64 as f64` silently loses bits above 2^52.
const fn ms(nanos: u64) -> u64 {
    nanos / 1_000_000
}

#[must_use]
pub fn report() -> Option<String> {
    if !enabled() {
        return None;
    }
    let calls = BUILD_CALLS.load(Ordering::Relaxed);
    if calls == 0 {
        return Some("keel perf: Model::build was never called".to_string());
    }
    let hits = CACHE_HITS.load(Ordering::Relaxed);
    let fp_ns = FINGERPRINT_NANOS.load(Ordering::Relaxed);
    Some(format!(
        "keel perf: Model::build x{calls} ({hits} cached, {} parsed) | fingerprint {}ms ({}ms/call) | parse {}ms | {} stat(s), {} tree walk(s) | git x{} in {}ms",
        calls - hits,
        ms(fp_ns),
        ms(fp_ns / calls),
        ms(PARSE_NANOS.load(Ordering::Relaxed)),
        FILES_STATTED.load(Ordering::Relaxed),
        TREES_WALKED.load(Ordering::Relaxed),
        GIT_CALLS.load(Ordering::Relaxed),
        ms(GIT_NANOS.load(Ordering::Relaxed)),
    ))
}
