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

/// Per-argv spawn tally, populated only at `KEEL_PERF=2`.
///
/// A total tells you spawns are the cost; only the breakdown tells you WHICH call to batch. Guessing the
/// caller from a total is how the earlier bad attributions happened - including issue147, which named
/// D0084 staleness when the answer was 480 `git show -s` calls from a guard.
pub static GIT_ARGV: std::sync::Mutex<Option<std::collections::BTreeMap<String, u64>>> =
    std::sync::Mutex::new(None);

/// Record one spawn's shape. Cheap and skipped entirely below `KEEL_PERF=2`.
pub fn note_git(args: &[&str]) {
    if !verbose() {
        return;
    }
    // Just the subcommand and its first flag-ish token: the full argv would be one line per element,
    // which is the noise the tally exists to collapse.
    let shape = args.iter().take(2).copied().collect::<Vec<_>>().join(" ");
    if let Ok(mut g) = GIT_ARGV.lock() {
        *g.get_or_insert_with(std::collections::BTreeMap::new).entry(shape).or_insert(0) += 1;
    }
}

/// `KEEL_PERF=2` — the per-argv breakdown as well as the totals.
#[must_use]
pub fn verbose() -> bool {
    static ON: std::sync::LazyLock<bool> =
        std::sync::LazyLock::new(|| std::env::var("KEEL_PERF").is_ok_and(|v| v == "2"));
    *ON
}

/// Named phase timings, populated only when instrumentation is on.
///
/// A composite view is a sequence of expensive steps, and a total says nothing about which one to
/// attack - the mistake that produced three wrong cost attributions before the per-argv tally existed.
pub static PHASES: std::sync::Mutex<Option<std::collections::BTreeMap<String, u64>>> =
    std::sync::Mutex::new(None);

/// Time `f` under a phase name. Free when instrumentation is off.
pub fn phase<T>(name: &str, f: impl FnOnce() -> T) -> T {
    if !enabled() {
        return f();
    }
    let t0 = std::time::Instant::now();
    let out = f();
    let ns = u64::try_from(t0.elapsed().as_nanos()).unwrap_or(u64::MAX);
    if let Ok(mut g) = PHASES.lock() {
        *g.get_or_insert_with(std::collections::BTreeMap::new).entry(name.to_string()).or_insert(0) += ns;
    }
    out
}

/// Calls to `grandfathered_under`.
///
/// A COUNT, not a duration: phase timings on this host vary ~15% run to run, and a count is immune to
/// that - the lesson the git-spawn work taught after three wrong attributions from wall clock. This
/// counter is what refuted a memoization I had already written: 2 calls per command, so there was nothing
/// to reuse and the memo was reverted rather than kept on a plausible story.
pub static GF_CALLS: AtomicU64 = AtomicU64::new(0);

/// Whether instrumentation is on. Read once; an env lookup per build call would itself be a cost.
#[must_use]
pub fn enabled() -> bool {
    static ON: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
        std::env::var("KEEL_PERF").is_ok_and(|v| v == "1" || v == "2")
    });
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
/// The per-argv tally as report lines, or empty when not at `KEEL_PERF=2`. Sorted by count descending,
/// because the thing to batch is whatever is at the top.
fn git_breakdown() -> String {
    use std::fmt::Write as _;
    let Ok(g) = GIT_ARGV.lock() else { return String::new() };
    let Some(map) = g.as_ref() else { return String::new() };
    let mut rows: Vec<(&String, &u64)> = map.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1));
    rows.iter().fold(String::new(), |mut s, (shape, n)| {
        let _ = write!(s, "
  git {shape} x{n}");
        s
    })
}

/// Phase timings as report lines, slowest first.
fn phase_breakdown() -> String {
    use std::fmt::Write as _;
    let Ok(g) = PHASES.lock() else { return String::new() };
    let Some(map) = g.as_ref() else { return String::new() };
    let mut rows: Vec<(&String, &u64)> = map.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1));
    rows.iter().fold(String::new(), |mut s, (name, ns)| {
        let _ = write!(s, "
  phase {name} {}ms", ms(**ns));
        s
    })
}

/// Nanoseconds as whole milliseconds. Integer division, not a float cast: a report a human reads never
/// needs sub-millisecond precision, and `u64 as f64` silently loses bits above 2^52.
const fn ms(nanos: u64) -> u64 {
    nanos / 1_000_000
}

#[must_use]
/// Read the counters, then ZERO them, returning a one-line summary of the interval.
///
/// A long-running server can never be observed by [`report`], which prints once at process exit: `keel
/// serve` does not exit until the human stops caring. This is the same numbers scoped to an INTERVAL, so
/// each HTTP request can report its own cost - the only way to see which part of a cache HIT is slow.
pub fn interval() -> Option<String> {
    if !enabled() {
        return None;
    }
    let take = |c: &AtomicU64| c.swap(0, std::sync::atomic::Ordering::Relaxed);
    let (fp, parse, stats, builds, cached, git, gitns) = (
        take(&FINGERPRINT_NANOS),
        take(&PARSE_NANOS),
        take(&FILES_STATTED),
        take(&BUILD_CALLS),
        take(&CACHE_HITS),
        take(&GIT_CALLS),
        take(&GIT_NANOS),
    );
    Some(format!(
        "fp {}ms/{stats} stat · parse {}ms · build x{builds} ({cached} cached) · git x{git} in {}ms",
        fp / 1_000_000,
        parse / 1_000_000,
        gitns / 1_000_000
    ))
}

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
    ) + &format!(" | grandfathered x{}", GF_CALLS.load(Ordering::Relaxed)) + &git_breakdown() + &phase_breakdown())
}
