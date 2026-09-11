//! `keel show flow` and the git-derived cards of `keel render report flow` (dcCycleTimeReadsFromGit;
//! issue483, issue485).
//!
//! WHY GIT. Every `TestResult` carries `judgedAt` as a DATE, and nearly every sprint opens and closes on
//! the same day, so the flow report read cycle time as `0d`, time per story point as `0.01d` - a working
//! instrument whose resolution was coarser than the work it measured (issue483). D0072 kept
//! `actualHours` for the AI's time; the field was populated on 15 lines in 6 of 671 delivery files, last
//! at sprint 369, and lapsed. Nothing authored replaced it, and nothing authored should: git already
//! holds the seconds. This module is the computed replacement.
//!
//! WHAT IS MEASURED, per sprint (one `.tracking/delivery/*.sysml` file):
//! - `opened`: the first commit that touches the delivery file.
//! - `closed`: the first commit whose version of the file carries the retro gate's recorded result -
//!   `pass`, or `proposed` (D0312 B: an AI-examined pass awaiting the human; the ceremony has ended,
//!   the judgment has not). A sprint whose retro result has landed in no commit is `open`, never 0.
//! - `minutes`: `closed - opened`. This is the interval the Definition of Done names, and it is honest about what git
//!   holds: a sprint born and closed in ONE commit carries no interval and reads 0 with `commits = 1`.
//! - `elapsedMinutes`: `closed` minus the repository commit that PRECEDES `opened`. For a one-commit
//!   sprint this is the gap from whatever landed before it to its own landing - the time git holds for
//!   the work (issue483 called it the inter-commit gap per sprint). It is carried beside `minutes`,
//!   never in its place.
//!
//! Nothing here is stored (§2.1): every number is recomputed from `git log` and the working tree.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use crate::json::Json;

use super::ViewError;

/// Seconds in a day.
const DAY: i64 = 86_400;

/// The window of the calibration card: the most recently closed sprints it reads.
pub const CALIBRATION_WINDOW: usize = 80;

/// One sprint's git-derived span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SprintSpan {
    /// The delivery file's stem (`sprint592_measuresAreModelItems`).
    pub sprint: String,
    /// `estimatedPoints` as authored; 0 when the file declares none.
    pub points: i64,
    /// Epoch seconds of the first commit touching the file; `None` when git holds no commit for it.
    pub opened: Option<i64>,
    /// Epoch seconds of the commit carrying the retro gate's recorded result; `None` while open.
    pub closed: Option<i64>,
    /// Epoch seconds of the repository commit preceding `opened`; `None` at the root commit.
    pub before: Option<i64>,
    /// Commits touching the file up to and including `closed` (all of them while open).
    pub commits: usize,
}

impl SprintSpan {
    /// The Definition of Done's interval: first commit touching the file to the commit landing the retro result.
    #[must_use]
    pub fn minutes(&self) -> Option<i64> {
        Some(self.closed?.saturating_sub(self.opened?).max(0) / 60)
    }

    /// The interval counted from the commit BEFORE the file was born (see the module doc).
    #[must_use]
    pub fn elapsed_minutes(&self) -> Option<i64> {
        Some(self.closed?.saturating_sub(self.before?).max(0) / 60)
    }

    /// `closed` when the retro result has a landing commit, else `open`.
    #[must_use]
    pub const fn state(&self) -> &'static str {
        if self.closed.is_some() { "closed" } else { "open" }
    }

    /// Born and closed in one commit: the file itself carries no interval.
    #[must_use]
    pub const fn one_commit(&self) -> bool {
        self.closed.is_some() && self.commits <= 1
    }
}

/// Everything the flow lens and cards read from git, gathered in three `git log` spawns and one
/// batched blob read.
#[derive(Debug, Clone, Default)]
pub struct FlowFacts {
    /// Every sprint, ordered by `opened` (unrecorded ones last, by name).
    pub spans: Vec<SprintSpan>,
    /// Committer timestamps of every commit reachable from HEAD, ascending.
    pub commit_times: Vec<i64>,
}

impl FlowFacts {
    /// The latest commit's timestamp - the tree's own "now", so the answer is a function of the tree.
    #[must_use]
    pub fn as_of(&self) -> Option<i64> {
        self.commit_times.last().copied()
    }

    /// The closed spans, most recently closed first.
    #[must_use]
    pub fn closed(&self) -> Vec<&SprintSpan> {
        let mut v: Vec<&SprintSpan> = self.spans.iter().filter(|s| s.closed.is_some()).collect();
        v.sort_by_key(|s| std::cmp::Reverse(s.closed));
        v
    }
}

/// Is this delivery file a sprint? It runs a retro or sizes itself; the package/dogfood file does neither.
#[must_use]
pub fn is_sprint(text: &str) -> bool {
    text.contains("RetroGate") || text.contains("estimatedPoints")
}

/// The retro gate has a RECORDED result in `text` - `pass` or `proposed` (D0312 B).
#[must_use]
pub fn retro_recorded(text: &str) -> bool {
    crate::orient::gate_recorded(text, "Retro")
}

/// The first `estimatedPoints = N;` in `text`, else 0.
#[must_use]
pub fn points_of(text: &str) -> i64 {
    text.split("estimatedPoints = ")
        .nth(1)
        .and_then(|x| x.trim_start().split(|c: char| !c.is_ascii_digit()).next()?.parse::<i64>().ok())
        .unwrap_or(0)
}

/// One `git log --format=@@%H %ct --name-only` record: the commit, its committer time, the paths.
type Touch = (String, i64, Vec<String>);

/// Parse the `@@<sha> <ct>` / path-line shape `git log --name-only` emits with that format.
#[must_use]
pub fn parse_touch_log(text: &str) -> Vec<Touch> {
    let mut out: Vec<Touch> = Vec::new();
    for line in text.lines() {
        if let Some(head) = line.strip_prefix("@@") {
            let mut it = head.split_whitespace();
            let sha = it.next().unwrap_or_default().to_string();
            let ts = it.next().and_then(|t| t.parse::<i64>().ok()).unwrap_or(0);
            out.push((sha, ts, Vec::new()));
        } else if !line.trim().is_empty() {
            if let Some(last) = out.last_mut() {
                last.2.push(line.trim().to_string());
            }
        }
    }
    out
}

/// `git -C root <args>` stdout, or the failure as text.
fn git_text(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["-c", "core.quotepath=false"])
        .args(args)
        .output()
        .map_err(|e| format!("git could not be spawned at {}: {e}", root.display()))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(format!("git {} failed at {}: {}", args.join(" "), root.display(), String::from_utf8_lossy(&out.stderr).trim()))
    }
}

/// The delivery directory's path as git names it - relative to the repository's top level, so a
/// project nested in a workspace (D0234) still keys its files correctly.
fn delivery_prefix(root: &Path) -> Result<String, String> {
    let prefix = git_text(root, &["rev-parse", "--show-prefix"])?.trim().replace('\\', "/");
    Ok(format!("{prefix}.tracking/delivery/"))
}

/// Gather the flow facts from git and the working tree.
///
/// # Errors
/// The text of the git failure when `root` is not in a repository or git cannot run.
pub fn facts(root: &Path) -> Result<FlowFacts, String> {
    // The pathspec is relative to `root` (git's cwd under `-C`); the paths `--name-only` prints are
    // relative to the repository's top level, so the keys carry `prefix`.
    let prefix = delivery_prefix(root)?;
    let pathspec = ".tracking/delivery";
    // 1. every commit touching the delivery directory, oldest first, with the files it touched.
    let touches = parse_touch_log(&git_text(root, &["log", "--reverse", "--format=@@%H%x20%ct", "--name-only", "--", pathspec])?);
    // 2. the commits where a file's count of `RetroGate` mentions changed: the candidates for the
    //    commit that landed the retro result (a result line adds one mention).
    let candidates = parse_touch_log(&git_text(root, &["log", "--reverse", "--format=@@%H%x20%ct", "--name-only", "-SRetroGate", "--", pathspec])?);
    // 3. every commit's time, for the inter-commit gaps and the commit preceding each birth.
    let mut commit_times: Vec<i64> = git_text(root, &["log", "--format=%ct"])?.lines().filter_map(|l| l.trim().parse::<i64>().ok()).collect();
    commit_times.sort_unstable();

    let mut touched_by: HashMap<&str, Vec<(&str, i64)>> = HashMap::new();
    for (sha, ts, paths) in &touches {
        for p in paths {
            touched_by.entry(p.as_str()).or_default().push((sha.as_str(), *ts));
        }
    }
    let mut candidates_by: HashMap<&str, Vec<(&str, i64)>> = HashMap::new();
    for (sha, ts, paths) in &candidates {
        for p in paths {
            candidates_by.entry(p.as_str()).or_default().push((sha.as_str(), *ts));
        }
    }

    // The working tree's sprint files decide the population; git decides their spans.
    let mut files: Vec<(String, String, i64)> = Vec::new(); // (stem, git path, points)
    for path in crate::collect_sysml(&root.join(".tracking").join("delivery")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        if !is_sprint(&text) {
            continue;
        }
        let name = path.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        files.push((stem, format!("{prefix}{name}"), points_of(&text)));
    }
    // One batched blob read for every candidate version.
    let keys: Vec<String> = files
        .iter()
        .flat_map(|(_, gp, _)| candidates_by.get(gp.as_str()).into_iter().flatten().map(move |(sha, _)| format!("{sha}:{gp}")))
        .collect();
    let blobs = crate::orient::batch_cat_blobs(root, &keys);

    let mut spans: Vec<SprintSpan> = Vec::new();
    for (stem, gp, points) in files {
        let history = touched_by.get(gp.as_str()).cloned().unwrap_or_default();
        let opened = history.first().map(|(_, ts)| *ts);
        let closed = candidates_by
            .get(gp.as_str())
            .into_iter()
            .flatten()
            .find(|(sha, _)| blobs.get(&format!("{sha}:{gp}")).and_then(Option::as_deref).is_some_and(retro_recorded))
            .map(|(_, ts)| *ts);
        let commits = closed.map_or(history.len(), |c| history.iter().filter(|(_, ts)| *ts <= c).count());
        let before = opened.and_then(|o| commit_times.iter().rev().find(|t| **t < o).copied());
        spans.push(SprintSpan { sprint: stem, points, opened, closed, before, commits });
    }
    spans.sort_by(|a, b| match (a.opened, b.opened) {
        (Some(x), Some(y)) => x.cmp(&y).then_with(|| a.sprint.cmp(&b.sprint)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.sprint.cmp(&b.sprint),
    });
    Ok(FlowFacts { spans, commit_times })
}

// ── statistics ─────────────────────────────────────────────────────────────────────────────────

/// The median of `v` (sorted in place); the lower-middle mean for an even count. `None` when empty.
pub fn median(v: &mut [i64]) -> Option<i64> {
    v.sort_unstable();
    let n = v.len();
    if n == 0 {
        return None;
    }
    let mid = n / 2;
    if n % 2 == 1 {
        v.get(mid).copied()
    } else {
        Some((v.get(mid - 1)? + v.get(mid)?) / 2)
    }
}

/// The nearest-rank 90th percentile of an ASCENDING slice. `None` when empty.
#[must_use]
pub fn p90(sorted: &[i64]) -> Option<i64> {
    let n = sorted.len();
    if n == 0 {
        return None;
    }
    // Nearest rank: the ceil(0.9 n)-th value, 1-based - never below the median.
    sorted.get((n * 9).div_ceil(10) - 1).copied()
}

/// Median and p90 of a sample, in one pass.
fn med_p90(mut v: Vec<i64>) -> (Option<i64>, Option<i64>) {
    let m = median(&mut v);
    (m, p90(&v))
}

/// A minute count in the unit a reader can hold: hours under a day, days from a day.
#[must_use]
pub fn fmt_minutes(minutes: i64) -> String {
    let m = f64::from(i32::try_from(minutes).unwrap_or(i32::MAX));
    if minutes < 1440 {
        format!("{:.1} h", m / 60.0)
    } else {
        format!("{:.1} d", m / 1440.0)
    }
}

fn opt_minutes(m: Option<i64>) -> String {
    m.map_or_else(|| "none".to_string(), |v| format!("{v} min"))
}

// ── inter-commit gap ───────────────────────────────────────────────────────────────────────────

/// Gaps between consecutive commits over one window.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GapWindow {
    /// Commits whose time falls in `(from, to]`.
    pub commits: usize,
    /// Median minutes between consecutive commits in the window.
    pub median: Option<i64>,
    /// p90 minutes between consecutive commits in the window.
    pub p90: Option<i64>,
}

/// The gaps between consecutive commits whose LATER commit falls in `(from, to]`, from ascending times.
#[must_use]
pub fn inter_commit_gaps(times: &[i64], from: i64, to: i64) -> GapWindow {
    let mut gaps: Vec<i64> = Vec::new();
    let mut commits = 0usize;
    for (i, t) in times.iter().enumerate() {
        if *t > from && *t <= to {
            commits += 1;
            if let Some(prev) = i.checked_sub(1).and_then(|j| times.get(j)) {
                gaps.push((t - prev).max(0) / 60);
            }
        }
    }
    let (median, p90) = med_p90(gaps);
    GapWindow { commits, median, p90 }
}

impl GapWindow {
    fn json(&self) -> Json {
        Json::Obj(vec![
            ("commits".to_string(), Json::Int(i64::try_from(self.commits).unwrap_or(i64::MAX))),
            ("medianMinutes".to_string(), self.median.map_or(Json::Null, Json::Int)),
            ("p90Minutes".to_string(), self.p90.map_or(Json::Null, Json::Int)),
        ])
    }
}

// ── calibration ────────────────────────────────────────────────────────────────────────────────

/// One point bucket of the calibration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    pub points: i64,
    pub n: usize,
    /// Median / p90 of the Definition-of-Done interval (`minutes`).
    pub median: Option<i64>,
    pub p90: Option<i64>,
    /// Median / p90 of `elapsedMinutes`.
    pub median_elapsed: Option<i64>,
    pub p90_elapsed: Option<i64>,
}

/// Do story points predict effort? Read over the most recently closed `window` sprints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calibration {
    pub window: usize,
    /// Closed sprints actually read (at most `window`).
    pub read: usize,
    /// How many of those were born and closed in one commit.
    pub one_commit: usize,
    /// `minutes` when the file intervals carry data for at least half the sprints, else `elapsedMinutes`.
    pub basis: &'static str,
    pub buckets: Vec<Bucket>,
    /// `predictive`, `not predictive`, or `no data`.
    pub verdict: &'static str,
}

/// Read the calibration from the most recently closed `window` spans.
#[must_use]
pub fn calibration(facts: &FlowFacts, window: usize) -> Calibration {
    let recent: Vec<&SprintSpan> = facts.closed().into_iter().take(window).collect();
    let read = recent.len();
    let one_commit = recent.iter().filter(|s| s.one_commit()).count();
    let basis = if read > 0 && one_commit * 2 > read { "elapsedMinutes" } else { "minutes" };
    let mut by: BTreeMap<i64, (Vec<i64>, Vec<i64>)> = BTreeMap::new();
    for s in &recent {
        let e = by.entry(s.points).or_default();
        if let Some(m) = s.minutes() {
            e.0.push(m);
        }
        if let Some(m) = s.elapsed_minutes() {
            e.1.push(m);
        }
    }
    let buckets: Vec<Bucket> = by
        .into_iter()
        .map(|(points, (mins, elapsed))| {
            let n = mins.len();
            let (median, p90) = med_p90(mins);
            let (median_elapsed, p90_elapsed) = med_p90(elapsed);
            Bucket { points, n, median, p90, median_elapsed, p90_elapsed }
        })
        .collect();
    // The verdict reads buckets with at least three sprints: a bucket of one is a sprint, not a size.
    let medians: Vec<i64> = buckets
        .iter()
        .filter(|b| b.n >= 3)
        .filter_map(|b| if basis == "minutes" { b.median } else { b.median_elapsed })
        .collect();
    let verdict = if medians.len() < 2 {
        "no data"
    } else if medians.windows(2).all(|w| w.first().zip(w.get(1)).is_some_and(|(a, b)| a < b)) {
        "predictive"
    } else {
        "not predictive"
    };
    Calibration { window, read, one_commit, basis, buckets, verdict }
}

impl Calibration {
    /// One line per bucket: `1 pt n=14 median 0 / p90 0 min (elapsed 48 / 120)`.
    #[must_use]
    pub fn detail(&self) -> String {
        if self.read == 0 {
            return "no closed sprint has a landing commit yet".to_string();
        }
        let rows: Vec<String> = self
            .buckets
            .iter()
            .map(|b| {
                format!(
                    "{} pt n={} median {} / p90 {} (elapsed {} / {})",
                    b.points,
                    b.n,
                    opt_minutes(b.median),
                    opt_minutes(b.p90),
                    opt_minutes(b.median_elapsed),
                    opt_minutes(b.p90_elapsed)
                )
            })
            .collect();
        let basis = if self.basis == "minutes" {
            "verdict on the file interval".to_string()
        } else {
            format!("the file interval is absent for {} of {} (born and closed in one commit); verdict on the elapsed time from the preceding commit", self.one_commit, self.read)
        };
        format!("last {} closed sprints; {basis}; buckets under 3 sprints do not vote. {}", self.read, rows.join("; "))
    }

    fn json(&self) -> Json {
        let buckets: Vec<Json> = self
            .buckets
            .iter()
            .map(|b| {
                Json::Obj(vec![
                    ("points".to_string(), Json::Int(b.points)),
                    ("n".to_string(), Json::Int(i64::try_from(b.n).unwrap_or(i64::MAX))),
                    ("medianMinutes".to_string(), b.median.map_or(Json::Null, Json::Int)),
                    ("p90Minutes".to_string(), b.p90.map_or(Json::Null, Json::Int)),
                    ("medianElapsedMinutes".to_string(), b.median_elapsed.map_or(Json::Null, Json::Int)),
                    ("p90ElapsedMinutes".to_string(), b.p90_elapsed.map_or(Json::Null, Json::Int)),
                ])
            })
            .collect();
        Json::Obj(vec![
            ("window".to_string(), Json::Int(i64::try_from(self.window).unwrap_or(i64::MAX))),
            ("read".to_string(), Json::Int(i64::try_from(self.read).unwrap_or(i64::MAX))),
            ("oneCommit".to_string(), Json::Int(i64::try_from(self.one_commit).unwrap_or(i64::MAX))),
            ("basis".to_string(), Json::s(self.basis)),
            ("verdict".to_string(), Json::s(self.verdict)),
            ("buckets".to_string(), Json::Arr(buckets)),
        ])
    }
}

// ── the report's git-derived cards ─────────────────────────────────────────────────────────────

/// The four cards `keel render report flow` reads from git: cycle time, time per story point, the
/// inter-commit gap and the point calibration.
#[must_use]
pub fn cards(facts: &FlowFacts) -> Vec<Json> {
    let closed = facts.closed();
    let open = facts.spans.len().saturating_sub(closed.len());
    let one_commit = closed.iter().filter(|s| s.one_commit()).count();
    let (cycle_med, cycle_p90) = med_p90(closed.iter().filter_map(|s| s.minutes()).collect());
    let (elapsed_med, _) = med_p90(closed.iter().filter_map(|s| s.elapsed_minutes()).collect());
    let pts: i64 = closed.iter().map(|s| s.points).sum();
    let total_min: i64 = closed.iter().filter_map(|s| s.minutes()).sum();
    let total_elapsed: i64 = closed.iter().filter_map(|s| s.elapsed_minutes()).sum();
    let per_point = |total: i64| if pts == 0 { 0 } else { total / pts };
    let as_of = facts.as_of().unwrap_or(0);
    let last7 = inter_commit_gaps(&facts.commit_times, as_of - 7 * DAY, as_of);
    let prior7 = inter_commit_gaps(&facts.commit_times, as_of - 14 * DAY, as_of - 7 * DAY);
    let calib = calibration(facts, CALIBRATION_WINDOW);
    let gap_tone = match (last7.median, prior7.median) {
        (Some(a), Some(b)) if a > b => "warn",
        _ => "good",
    };
    vec![
        super::card(
            "Cycle time",
            cycle_med.map_or_else(|| "open".to_string(), fmt_minutes),
            format!(
                "median first commit -> retro landing across {} closed sprints (p90 {}); {one_commit} born and closed in one commit read 0; {open} open (retro not landed); elapsed from the preceding commit: median {}",
                closed.len(),
                opt_minutes(cycle_p90),
                elapsed_med.map_or_else(|| "none".to_string(), fmt_minutes)
            ),
            "good",
        ),
        super::card(
            "Time / story point",
            if pts == 0 { "no points".to_string() } else { fmt_minutes(per_point(total_min)) },
            format!("{total_min} min / {pts} pts over closed sprints (elapsed basis: {} per point, {total_elapsed} min)", fmt_minutes(per_point(total_elapsed))),
            "good",
        ),
        super::card(
            "Inter-commit gap",
            format!("{} median, {} p90", opt_minutes(last7.median), opt_minutes(last7.p90)),
            format!(
                "last 7 days ({} commits) against the 7 before: {} median, {} p90 ({} commits); as-of the latest commit",
                last7.commits,
                opt_minutes(prior7.median),
                opt_minutes(prior7.p90),
                prior7.commits
            ),
            gap_tone,
        ),
        super::card(
            "Point calibration",
            calib.verdict.to_string(),
            calib.detail(),
            match calib.verdict {
                "predictive" => "good",
                "not predictive" => "warn",
                _ => "empty",
            },
        ),
    ]
}

/// The same four cards when git could not be read: the reason, never a 0.
#[must_use]
pub fn unavailable_cards(reason: &str) -> Vec<Json> {
    ["Cycle time", "Time / story point", "Inter-commit gap", "Point calibration"]
        .into_iter()
        .map(|label| super::card(label, "unavailable".to_string(), format!("git history could not be read: {reason}"), "empty"))
        .collect()
}

// ── the lens ───────────────────────────────────────────────────────────────────────────────────

/// `keel show flow [--json]`: the per-sprint series, the inter-commit gaps and the calibration.
///
/// # Errors
/// [`ViewError::Io`] when git cannot be read at `root`.
pub fn flow(root: &Path) -> Result<String, ViewError> {
    let facts = facts(root).map_err(|e| ViewError::Io("git log".to_string(), std::io::Error::other(e)))?;
    let as_of = facts.as_of().unwrap_or(0);
    let sprints: Vec<Json> = facts
        .spans
        .iter()
        .map(|s| {
            Json::Obj(vec![
                ("sprint".to_string(), Json::s(s.sprint.clone())),
                ("points".to_string(), Json::Int(s.points)),
                ("state".to_string(), Json::s(s.state())),
                ("minutes".to_string(), s.minutes().map_or(Json::Null, Json::Int)),
                ("elapsedMinutes".to_string(), s.elapsed_minutes().map_or(Json::Null, Json::Int)),
                ("commits".to_string(), Json::Int(i64::try_from(s.commits).unwrap_or(i64::MAX))),
                ("opened".to_string(), s.opened.map_or(Json::Null, Json::Int)),
                ("closed".to_string(), s.closed.map_or(Json::Null, Json::Int)),
            ])
        })
        .collect();
    let closed = facts.closed().len();
    Ok(Json::Obj(vec![
        ("lens".to_string(), Json::s("flow")),
        ("asOf".to_string(), Json::Int(as_of)),
        ("resolution".to_string(), Json::s("minutes, from commit timestamps")),
        ("sprints".to_string(), Json::Arr(sprints)),
        ("counts".to_string(), Json::Obj(vec![
            ("sprints".to_string(), Json::Int(i64::try_from(facts.spans.len()).unwrap_or(i64::MAX))),
            ("closed".to_string(), Json::Int(i64::try_from(closed).unwrap_or(i64::MAX))),
            ("open".to_string(), Json::Int(i64::try_from(facts.spans.len().saturating_sub(closed)).unwrap_or(i64::MAX))),
        ])),
        ("interCommitGap".to_string(), Json::Obj(vec![
            ("last7d".to_string(), inter_commit_gaps(&facts.commit_times, as_of - 7 * DAY, as_of).json()),
            ("prior7d".to_string(), inter_commit_gaps(&facts.commit_times, as_of - 14 * DAY, as_of - 7 * DAY).json()),
        ])),
        ("calibration".to_string(), calibration(&facts, CALIBRATION_WINDOW).json()),
        ("note".to_string(), Json::s("minutes = first commit touching the delivery file -> the commit landing its retro result (pass or proposed, D0312 B); a sprint whose retro has not landed is open, never 0; elapsedMinutes counts from the repository commit preceding the file's first commit - for a sprint born and closed in one commit it is the time git holds for its work. Computed; never stored.")),
    ])
    .dump())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(name: &str, points: i64, opened: Option<i64>, closed: Option<i64>, before: Option<i64>, commits: usize) -> SprintSpan {
        SprintSpan { sprint: name.to_string(), points, opened, closed, before, commits }
    }

    #[test]
    fn a_span_reads_its_minutes_from_the_two_commits() {
        let s = span("a", 2, Some(1_000 * 60), Some(1_030 * 60), Some(990 * 60), 2);
        assert_eq!(s.minutes(), Some(30));
        assert_eq!(s.elapsed_minutes(), Some(40));
        assert_eq!(s.state(), "closed");
        assert!(!s.one_commit());
    }

    /// D0388 negative: a retro result with no landing commit is `open`, and its minutes are absent -
    /// never 0.
    #[test]
    fn a_sprint_whose_retro_has_not_landed_is_open_never_zero() {
        let s = span("b", 3, Some(60), None, None, 1);
        assert_eq!(s.state(), "open");
        assert_eq!(s.minutes(), None);
        assert_eq!(s.elapsed_minutes(), None);
        let unrecorded = span("c", 3, None, None, None, 0);
        assert_eq!(unrecorded.state(), "open");
        assert_eq!(unrecorded.minutes(), None);
    }

    #[test]
    fn the_touch_log_parses_into_commits_and_their_paths() {
        let text = "@@aaaa 100\n\n.tracking/delivery/s1.sysml\n@@bbbb 160\n\n.tracking/delivery/s1.sysml\n.tracking/delivery/s2.sysml\n";
        let got = parse_touch_log(text);
        assert_eq!(got.len(), 2);
        assert_eq!(got[0], ("aaaa".to_string(), 100, vec![".tracking/delivery/s1.sysml".to_string()]));
        assert_eq!(got[1].2.len(), 2);
    }

    #[test]
    fn the_retro_reader_accepts_pass_and_proposed_and_refuses_fail() {
        let pass = "    part xRetroGateR1 : TestResult { :>> outcome = VerdictKind::pass; }\n";
        let proposed = "    part xRetroGateR1 : TestResult { :>> outcome = VerdictKind::proposed; }\n";
        let fail = "    part xRetroGateR1 : TestResult { :>> outcome = VerdictKind::fail; }\n";
        let declared_only = "    verification xRetroGate : Test { :>> title = \"retro\"; }\n";
        assert!(retro_recorded(pass));
        assert!(retro_recorded(proposed));
        assert!(!retro_recorded(fail));
        assert!(!retro_recorded(declared_only));
    }

    #[test]
    fn points_and_sprintness_read_from_the_text() {
        assert_eq!(points_of(":>> owner = \"x\"; :>> estimatedPoints = 5;\n"), 5);
        assert_eq!(points_of("nothing"), 0);
        assert!(is_sprint("verification xRetroGate : Test {}"));
        assert!(!is_sprint("package ProjectDelivery {}"));
    }

    #[test]
    fn median_and_p90_are_nearest_rank() {
        let mut v = vec![90, 30];
        assert_eq!(median(&mut v), Some(60));
        let mut w = vec![5, 1, 3];
        assert_eq!(median(&mut w), Some(3));
        assert_eq!(p90(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), Some(9));
        assert_eq!(p90(&[]), None);
        assert_eq!(median(&mut []), None);
    }

    #[test]
    fn minutes_read_in_hours_under_a_day_and_days_from_a_day() {
        assert_eq!(fmt_minutes(30), "0.5 h");
        assert_eq!(fmt_minutes(0), "0.0 h");
        assert_eq!(fmt_minutes(1439), "24.0 h");
        assert_eq!(fmt_minutes(2880), "2.0 d");
    }

    #[test]
    fn inter_commit_gaps_read_the_window_by_the_later_commit() {
        let times: Vec<i64> = vec![0, 600, 1_200, 4_800, 5_400];
        // (1_200, 5_400]: commits at 4_800 (gap 60 min) and 5_400 (gap 10 min).
        let g = inter_commit_gaps(&times, 1_200, 5_400);
        assert_eq!(g.commits, 2);
        assert_eq!(g.median, Some(35));
        assert_eq!(g.p90, Some(60));
        let empty = inter_commit_gaps(&times, 10_000, 20_000);
        assert_eq!(empty, GapWindow::default());
    }

    /// D0388 positive: two sprints 30 and 90 minutes apart read 30 and 90 - and the calibration puts
    /// them in their point buckets.
    #[test]
    fn calibration_buckets_by_points_and_votes_only_with_three() {
        let mut spans = Vec::new();
        for i in 0..3_i64 {
            spans.push(span(&format!("one{i}"), 1, Some(i * 10_000), Some(i * 10_000 + 30 * 60), Some(i * 10_000 - 600), 2));
            spans.push(span(&format!("three{i}"), 3, Some(i * 10_000 + 5_000), Some(i * 10_000 + 5_000 + 90 * 60), Some(i * 10_000 + 4_400), 2));
        }
        let facts = FlowFacts { spans, commit_times: vec![] };
        let c = calibration(&facts, 80);
        assert_eq!(c.read, 6);
        assert_eq!(c.basis, "minutes");
        assert_eq!(c.verdict, "predictive");
        assert_eq!(c.buckets.iter().map(|b| (b.points, b.median)).collect::<Vec<_>>(), vec![(1, Some(30)), (3, Some(90))]);
    }

    /// When the files carry no interval (every sprint one commit), the verdict moves to the elapsed
    /// basis and says so; equal medians are `not predictive`.
    #[test]
    fn calibration_falls_back_to_elapsed_when_files_carry_no_interval() {
        let mut spans = Vec::new();
        for i in 0..3_i64 {
            spans.push(span(&format!("one{i}"), 1, Some(i * 10_000), Some(i * 10_000), Some(i * 10_000 - 2_400), 1));
            spans.push(span(&format!("two{i}"), 2, Some(i * 10_000 + 5_000), Some(i * 10_000 + 5_000), Some(i * 10_000 + 5_000 - 2_400), 1));
        }
        let facts = FlowFacts { spans, commit_times: vec![] };
        let c = calibration(&facts, 80);
        assert_eq!(c.one_commit, 6);
        assert_eq!(c.basis, "elapsedMinutes");
        assert_eq!(c.verdict, "not predictive");
        assert!(c.detail().contains("absent for 6 of 6"));
        let none = calibration(&FlowFacts::default(), 80);
        assert_eq!(none.verdict, "no data");
    }
}
