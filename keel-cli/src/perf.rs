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
/// Walks answered from the corpus memo because every directory's mtime still matched.
pub static WALK_HITS: AtomicU64 = AtomicU64::new(0);
/// Files actually opened by `corpus::read_to_string`. On a Defender host the open is the cost.
pub static FILE_OPENS: AtomicU64 = AtomicU64::new(0);
/// Reads answered from the corpus cache because the file's `(len, mtime)` still matched.
pub static CORPUS_HITS: AtomicU64 = AtomicU64::new(0);
/// Parses answered from the corpus's package cache (`corpus::parsed`, issue441).
pub static PARSE_HITS: AtomicU64 = AtomicU64::new(0);

/// `git` subprocesses spawned. A process spawn is the most expensive thing this program does on
/// Windows, and it is invisible in a wall-clock number — hence a counter rather than another inference.
pub static GIT_CALLS: AtomicU64 = AtomicU64::new(0);
/// Nanoseconds spent waiting on `git`.
///
/// Every run through `gitx::Git` plus the two batch waits in orient (dcGuardsRunInParallelAndTimed);
/// before it only two helpers timed, and the line read 0 ms over 4.4 s of spawns (issue410).
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

/// Time `f` under a phase name. Free when neither instrumentation nor phase collection is on.
pub fn phase<T>(name: &str, f: impl FnOnce() -> T) -> T {
    if !collecting() {
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

/// PHASE COLLECTION FOR THE FIRE-LEDGER (issue429 / D0414).
///
/// A hook fire that took 28 s wrote a ledger line saying `ms: 28000` and nothing else, and no measurement
/// taken afterwards could say which phase it was: the tails are the fires nobody is watching. So a hook
/// process collects phases WITHOUT `KEEL_PERF` - the counters stay relaxed atomics, `phase` pays one
/// `Instant` and a map insert per named step - and a fire past `pm::SLOW_FIRE_MS` writes its attribution
/// into the line. `enabled()` still decides whether the report is PRINTED; this only decides whether the
/// numbers exist.
static PHASES_ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Turn phase collection on for this process (the hook wrapper calls it at entry).
pub fn collect_phases() {
    PHASES_ON.store(true, Ordering::Relaxed);
}

fn collecting() -> bool {
    enabled() || PHASES_ON.load(Ordering::Relaxed)
}

/// The prefix a hook's own SERIAL steps carry (`hook:validate`, `hook:guards`, ...).
///
/// Their sum is the part of a fire's total that is attributed; the remainder is reported as
/// `unattributed`, never dropped.
pub const HOOK_PHASE: &str = "hook:";

/// This process's attribution of a hook fire whose wall clock was `total_ms` - see [`attribute`].
#[must_use]
pub fn attribution(total_ms: u64) -> Vec<(String, u64)> {
    let phases = PHASES.lock().ok().and_then(|g| g.clone()).unwrap_or_default();
    let git = (GIT_CALLS.load(Ordering::Relaxed), GIT_NANOS.load(Ordering::Relaxed) / 1_000_000);
    attribute(&phases, git, PARSE_NANOS.load(Ordering::Relaxed) / 1_000_000, total_ms)
}

/// Attribute a hook fire's `total_ms` to what was measured, longest step first (issue429 / D0414).
///
/// The list is in two parts. First the WALL-CLOCK steps, longest first: every `hook:*` phase is a
/// serial step of the fire and is listed with its ms; the longest `guard:*` phase is the guard
/// runner's critical path (the guards run in parallel, so their sum is not a duration - only the
/// longest bounds the wall clock); and `unattributed` is `total_ms` minus the sum of the `hook:*`
/// steps - a phase no counter covers is reported as that number rather than omitted, so a tail this
/// attribution cannot explain says so in the line itself. Then the CROSS-CUTTING counters, which are
/// not steps and never lead: `git xN summed` is the time spent waiting on N git spawns across every
/// thread, so it can exceed the total and is labelled as summed; `parse` is the model build. Pure, so
/// the probe pair in `phase_attribution_tests` runs it.
#[must_use]
pub fn attribute(
    phases: &std::collections::BTreeMap<String, u64>,
    git: (u64, u64),
    parse_ms: u64,
    total_ms: u64,
) -> Vec<(String, u64)> {
    let ms = |ns: u64| ns / 1_000_000;
    let mut out: Vec<(String, u64)> = Vec::new();
    let mut serial = 0u64;
    for (name, ns) in phases.iter().filter(|(n, _)| n.starts_with(HOOK_PHASE)) {
        serial = serial.saturating_add(ms(*ns));
        out.push((name.clone(), ms(*ns)));
    }
    if let Some((name, ns)) = phases.iter().filter(|(n, _)| n.starts_with("guard:")).max_by_key(|(_, ns)| **ns) {
        out.push((format!("{name} (critical path)"), ms(*ns)));
    }
    out.push(("unattributed".to_string(), total_ms.saturating_sub(serial)));
    out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    if git.0 > 0 {
        out.push((format!("git x{} summed", git.0), git.1));
    }
    if parse_ms > 0 {
        out.push(("parse".to_string(), parse_ms));
    }
    out
}

/// Add `n` to a counter, but only when instrumentation or phase collection is on.
pub fn add(counter: &AtomicU64, n: u64) {
    if collecting() {
        counter.fetch_add(n, Ordering::Relaxed);
    }
}

/// Time `f`, adding its duration to `counter`. Returns `f`'s value either way.
pub fn timed<T>(counter: &AtomicU64, f: impl FnOnce() -> T) -> T {
    if !collecting() {
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
        "keel perf: Model::build x{calls} ({hits} cached, {} parsed) | fingerprint {}ms ({}ms/call) | parse {}ms | {} stat(s), {} tree walk(s) ({} from memo) | {} file open(s) ({} from corpus) | git x{} in {}ms",
        calls - hits,
        ms(fp_ns),
        ms(fp_ns / calls),
        ms(PARSE_NANOS.load(Ordering::Relaxed)),
        FILES_STATTED.load(Ordering::Relaxed),
        TREES_WALKED.load(Ordering::Relaxed),
        WALK_HITS.load(Ordering::Relaxed),
        FILE_OPENS.load(Ordering::Relaxed),
        CORPUS_HITS.load(Ordering::Relaxed),
        GIT_CALLS.load(Ordering::Relaxed),
        ms(GIT_NANOS.load(Ordering::Relaxed)),
    ) + &format!(" | grandfathered x{}", GF_CALLS.load(Ordering::Relaxed)) + &git_breakdown() + &phase_breakdown())
}

#[cfg(test)]
mod phase_attribution_tests {
    use super::attribute;
    use std::collections::BTreeMap;

    fn phases(pairs: &[(&str, u64)]) -> BTreeMap<String, u64> {
        pairs.iter().map(|(n, ms)| ((*n).to_string(), ms * 1_000_000)).collect()
    }

    /// issue429 / D0414, the probe pair chosen before the live ledger was read. KNOWN-POSITIVE: a fire
    /// whose validate step carried 5 000 of 6 000 ms names `hook:validate` FIRST and the 300 ms no step
    /// covered as `unattributed`, never dropped. KNOWN-NEGATIVE: a fire that measured no hook step at
    /// all attributes nothing but the total itself as unattributed - the line still explains that it
    /// cannot explain.
    #[test]
    fn the_longest_step_leads_and_the_uncovered_remainder_is_named() {
        let p = phases(&[("hook:validate", 5000), ("hook:guards", 700), ("guard:priority-inversion", 650), ("guard:actors", 20)]);
        let a = attribute(&p, (44, 14_114), 380, 6000);
        assert_eq!(a[0], ("hook:validate".to_string(), 5000), "{a:?}");
        assert_eq!(a[1], ("hook:guards".to_string(), 700), "{a:?}");
        let git = a.iter().position(|(n, _)| n == "git x44 summed").expect("git row");
        assert!(git > a.iter().position(|(n, _)| n == "unattributed").expect("unattributed row"), "a summed counter never leads, even at 14 s against a 6 s total: {a:?}");
        assert!(a.contains(&("git x44 summed".to_string(), 14_114)), "{a:?}");
        assert!(a.contains(&("guard:priority-inversion (critical path)".to_string(), 650)), "only the longest guard: {a:?}");
        assert!(!a.iter().any(|(n, _)| n.contains("guard:actors")), "{a:?}");
        assert!(a.contains(&("unattributed".to_string(), 300)), "6000 - (5000 + 700): {a:?}");
        assert!(a.contains(&("parse".to_string(), 380)), "{a:?}");
        let none = attribute(&BTreeMap::new(), (0, 0), 0, 28_000);
        assert_eq!(none, vec![("unattributed".to_string(), 28_000)], "{none:?}");
    }

    /// The serial sum can exceed the total under clock skew or a nested phase; the remainder saturates
    /// at zero rather than wrapping into a number that reads as a 584-million-year phase.
    #[test]
    fn an_over_attributed_fire_reports_zero_unattributed() {
        let p = phases(&[("hook:validate", 900), ("hook:guards", 900)]);
        let a = attribute(&p, (0, 0), 0, 1000);
        assert!(a.contains(&("unattributed".to_string(), 0)), "{a:?}");
    }
}
