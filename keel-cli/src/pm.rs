//! `keel enforcement-report` — the D0180 analysis over the fire-ledger.
//!
//! K14: the effect of enforcement is MEASURED, not presumed — D0128's recorded-but-undelivered
//! "prove the in-loop gate" step, delivered.
//!
//! THE LEDGER SCHEMA IS FROZEN HERE (D0180 owns the freeze): one JSON object per line in
//! `.keel/metrics/hooks.jsonl`, whose CORE fields are exactly
//! `ts` (unix seconds), `session` (harness session id), `event` (hook event name),
//! `decision` ("allow" | "block"), `exit` (i32), `ms` (u64). Emitters (`ledger_emit` in the hook
//! wrapper) and this reader are the two parties to the freeze, and the schema test binds them.
//! Three ADDITIVE fields ride beside the core, each by its own Decision and each optional to the
//! reader: `bin` and `build` (issue378 - which binary and build fired), and `phases` (D0414 /
//! issue429 - present ONLY on a fire whose `ms` reached [`SLOW_FIRE_MS`], an array of
//! `{name, ms}` naming what the fire spent its time on, longest first, with the remainder no
//! counter covered as `unattributed`). A line without `phases` is a fast fire or one recorded
//! before D0414; the report says which.
//! Entirely machine-local (gitignored class): no tracked summaries until a consumer for them is
//! named (D0144 — resolved fork B).
//!
//! WHAT THE NUMBERS ARE FOR: every advisory→blocking promotion (P0 guided→strict, P1 pattern
//! promotions) cites at least N sprints of this evidence; N = 3 unless D0180 is amended. The
//! launcher-fraction hypothesis and the dirty-tree-refusal rate report here once P5's run records
//! exist; until then those rows say so rather than reading as zero.

use crate::json::Json;
use std::collections::BTreeMap;
use std::path::Path;

/// The frozen field set — the schema test asserts emitted lines carry exactly these.
pub const LEDGER_FIELDS: [&str; 6] = ["ts", "session", "event", "decision", "exit", "ms"];

/// The additive fields the reader accepts beside the core (see the module doc); anything else is
/// a schema drift the test names.
pub const LEDGER_ADDITIVE_FIELDS: [&str; 3] = ["bin", "build", "phases"];

/// A hook fire at or past this many ms is SLOW (issue429 / D0414).
///
/// It carries `phases` in its ledger line and appears in `enforcement-report`'s `slowFires`. Set from
/// the measurement that opened issue429: an idle full stop-hook run on the reference host is
/// 2 650-2 990 ms, so a fire at 3 000 is one that did more than the idle run - and the tails (28 s,
/// 35 s, 38 s, the 120 s watchdog) are the fires this exists to explain. A fire under it explains
/// nothing and carries nothing.
pub const SLOW_FIRE_MS: u64 = 3000;

/// The `phases` value for a fire of `total_ms`, or `None` when the fire is not slow - the
/// known-negative of D0414's probe pair: a fast fire carries no field at all.
#[must_use]
pub fn slow_fire_phases(total_ms: u64) -> Option<serde_json::Value> {
    if total_ms < SLOW_FIRE_MS {
        return None;
    }
    let rows = crate::perf::attribution(total_ms)
        .into_iter()
        .map(|(name, ms)| serde_json::json!({"name": name, "ms": ms}))
        .collect::<Vec<_>>();
    Some(serde_json::Value::Array(rows))
}

/// One `slowFires` row (D0414 / issue429): the fire's identity and its own attribution. A line
/// recorded before D0414 has no `phases`; the row says so instead of reading as an empty attribution.
fn slow_fire_row(v: &serde_json::Value, event: &str, ms: u64) -> Json {
    let n = |x: u64| Json::Int(i64::try_from(x).unwrap_or(i64::MAX));
    let phases = v.get("phases").and_then(serde_json::Value::as_array).map_or_else(
        || Json::s("unattributed: recorded before D0414, the line carries no phases"),
        |rows| {
            Json::Arr(
                rows.iter()
                    .map(|r| {
                        Json::Obj(vec![
                            ("name".to_string(), Json::s(r.get("name").and_then(serde_json::Value::as_str).unwrap_or("?"))),
                            ("ms".to_string(), n(r.get("ms").and_then(serde_json::Value::as_u64).unwrap_or(0))),
                        ])
                    })
                    .collect(),
            )
        },
    );
    Json::Obj(vec![
        ("ts".to_string(), n(v.get("ts").and_then(serde_json::Value::as_u64).unwrap_or(0))),
        ("session".to_string(), Json::s(v.get("session").and_then(serde_json::Value::as_str).unwrap_or(""))),
        ("event".to_string(), Json::s(event)),
        ("ms".to_string(), n(ms)),
        ("phases".to_string(), phases),
    ])
}

/// The `slowFires` section: the threshold, the count in the ledger, and the LAST 25 rows newest
/// first - the tails are what the human asked about, and a row per fire is what the ledger line
/// alone could not say (issue429).
fn slow_fires_json(mut rows: Vec<Json>) -> Json {
    let count = rows.len();
    rows.reverse();
    rows.truncate(25);
    Json::Obj(vec![
        ("thresholdMs".to_string(), Json::Int(i64::try_from(SLOW_FIRE_MS).unwrap_or(i64::MAX))),
        ("count".to_string(), Json::Int(i64::try_from(count).unwrap_or(i64::MAX))),
        ("rows".to_string(), Json::Arr(rows)),
        ("how".to_string(), Json::s("every ledger line with ms >= thresholdMs; phases are the fire's own attribution written by the hook process (perf::attribute), longest first, remainder as unattributed")),
    ])
}

/// (median, p90, p99, max) of a sample, in ms; zeros for an empty sample. Nearest-rank percentiles,
/// so every reported value is one that actually occurred (D0389: a documented cost is a distribution).
#[must_use]
pub fn latency(ms: &[u64]) -> (i64, i64, i64, i64) {
    if ms.is_empty() {
        return (0, 0, 0, 0);
    }
    let mut s = ms.to_vec();
    s.sort_unstable();
    let n = s.len();
    // nearest rank in integer arithmetic: the ceil(p * n)-th value, 1-based, clamped into the sample
    let rank = |pct: usize| -> i64 {
        let i = (n * pct).div_ceil(100).clamp(1, n) - 1;
        s.get(i).copied().and_then(|v| i64::try_from(v).ok()).unwrap_or(i64::MAX)
    };
    (rank(50), rank(90), rank(99), s.last().copied().and_then(|v| i64::try_from(v).ok()).unwrap_or(i64::MAX))
}

/// The tracked-side counters (`#ProcessDefect` marks, synced override obligations, run records) —
/// extracted from [`enforcement_report`] for the line budget; behavior identical.
fn tracked_counts(root: &Path) -> (usize, usize, usize) {
    let mut process_defects = 0usize;
    for f in crate::collect_sysml(&root.join(".tracking")) {
        if let Ok(t) = std::fs::read_to_string(&f) {
            process_defects += t.matches("#ProcessDefect").count();
        }
    }
    let tracked_obligations =
        std::fs::read_dir(root.join(".tracking").join("obligations")).map_or(0, |rd| rd.flatten().count());
    let run_records = std::fs::read_dir(root.join(".keel").join("runs")).map_or(0, |rd| rd.flatten().count());
    (process_defects, tracked_obligations, run_records)
}

/// One row per hook event: fires, blocks, and the latency DISTRIBUTION (D0389/issue402) - a documented
/// cost is never the best case; the per-edit tier documented at ~0.35 s had a median of 0 ms and a
/// maximum of 54 s in this ledger.
fn event_rows(per_event: BTreeMap<String, (u64, u64)>, per_event_ms: &BTreeMap<String, Vec<u64>>) -> Vec<Json> {
    per_event
        .into_iter()
        .map(|(ev, (fires, blocks))| {
            let d = latency(per_event_ms.get(&ev).map_or(&[][..], Vec::as_slice));
            Json::Obj(vec![
                ("event".to_string(), Json::s(ev)),
                ("fires".to_string(), Json::Int(i64::try_from(fires).unwrap_or(i64::MAX))),
                ("blocks".to_string(), Json::Int(i64::try_from(blocks).unwrap_or(i64::MAX))),
                ("msMedian".to_string(), Json::Int(d.0)),
                ("msP90".to_string(), Json::Int(d.1)),
                ("msP99".to_string(), Json::Int(d.2)),
                ("msMax".to_string(), Json::Int(d.3)),
            ])
        })
        .collect()
}

/// D0389/issue402: the memory channel's degradation, counted. A `recall-skipped` line is written by the
/// user-prompt hook when the walk exceeded its cap and the turn went without facts; the rate is over
/// user-prompt fires, and the turns that lost their facts are identifiable afterwards by ts + session.
fn recall_turns(rows: &[(u64, String, u64)]) -> Json {
    Json::Arr(rows.iter().rev().take(20).map(|(ts, session, ms)| {
        Json::Obj(vec![
            ("ts".to_string(), Json::Int(i64::try_from(*ts).unwrap_or(i64::MAX))),
            ("session".to_string(), Json::s(session.clone())),
            ("ms".to_string(), Json::Int(i64::try_from(*ms).unwrap_or(i64::MAX))),
        ])
    }).collect())
}

fn recall_degradation(skips: &[(u64, String, u64)], slow: &[(u64, String, u64)], user_prompt_fires: u64) -> Json {
    let pct = |n: usize| -> Json {
        if user_prompt_fires == 0 {
            Json::Null
        } else {
            let a = f64::from(u32::try_from(n).unwrap_or(u32::MAX));
            let f = f64::from(u32::try_from(user_prompt_fires).unwrap_or(u32::MAX));
            Json::s(format!("{:.2}", a * 100.0 / f))
        }
    };
    Json::Obj(vec![
        // D0390: past the cap the facts are now pushed LATE and counted `recall-slow` (the turn keeps its
        // memory); `recall-skipped` is the pre-D0390 history where the facts were dropped.
        ("slow".to_string(), Json::Int(i64::try_from(slow.len()).unwrap_or(i64::MAX))),
        ("slowRatePct".to_string(), pct(slow.len())),
        ("slowTurns".to_string(), recall_turns(slow)),
        ("skipped".to_string(), Json::Int(i64::try_from(skips.len()).unwrap_or(i64::MAX))),
        ("skipRatePct".to_string(), pct(skips.len())),
        ("skippedTurns".to_string(), recall_turns(skips)),
        ("note".to_string(), Json::s("D0390 (2026-09-09): a recall past the cap is pushed LATE and counted recall-slow, not dropped; recall-skipped is the pre-D0390 history when facts were dropped. Both are counted from their ledger event; earlier still, a user-prompt fire over the cap is the proxy.".to_string())),
    ])
}

/// Compute the enforcement report.
///
/// # Errors
/// Never errors on an absent ledger — absence is a finding, not a failure; the `Result` signature
/// matches the computed-view convention so serve's cache can hold it.
#[allow(clippy::too_many_lines)] // one pass over the ledger = one place a line's every reading is made
pub fn enforcement_report(root: &Path) -> Result<String, crate::view::ViewError> {
    let ledger = root.join(".keel").join("metrics").join("hooks.jsonl");
    let text = std::fs::read_to_string(&ledger).unwrap_or_default();
    let mut per_event: BTreeMap<String, (u64, u64)> = BTreeMap::new(); // (fires, blocks)
    let mut per_event_ms: BTreeMap<String, Vec<u64>> = BTreeMap::new(); // D0389: the cost is a distribution
    let mut recall_skips: Vec<(u64, String, u64)> = Vec::new(); // (ts, session, ms) - turns that lost their facts (pre-D0390)
    let mut recall_slow: Vec<(u64, String, u64)> = Vec::new(); // (ts, session, ms) - turns pushed LATE past the cap (D0390)
    let mut sessions: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut overrides = 0u64;
    let mut unsynced = 0u64;
    let mut red_yields = 0u64;
    let mut advisory_issued = 0u64;
    let mut advisory_repeated = 0u64;
    let mut malformed = 0u64;
    let mut lines = 0u64;
    // D0414 / issue429: every fire at or past the slow threshold, with what it attributed itself to.
    let mut slow_fires: Vec<Json> = Vec::new();
    for line in text.lines() {
        lines += 1;
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            malformed += 1;
            continue;
        };
        let event = v.get("event").and_then(serde_json::Value::as_str).unwrap_or("?").to_string();
        let block = v.get("decision").and_then(serde_json::Value::as_str) == Some("block")
            || v.get("exit").and_then(serde_json::Value::as_i64).unwrap_or(0) != 0;
        if let Some(s) = v.get("session").and_then(serde_json::Value::as_str) {
            if !s.is_empty() {
                sessions.insert(s.to_string());
            }
        }
        let ms = v.get("ms").and_then(serde_json::Value::as_u64).unwrap_or(0);
        per_event_ms.entry(event.clone()).or_default().push(ms);
        if ms >= SLOW_FIRE_MS {
            slow_fires.push(slow_fire_row(&v, &event, ms));
        }
        match event.as_str() {
            "recall-skipped" => recall_skips.push((
                v.get("ts").and_then(serde_json::Value::as_u64).unwrap_or(0),
                v.get("session").and_then(serde_json::Value::as_str).unwrap_or("").to_string(),
                ms,
            )),
            "recall-slow" => recall_slow.push((
                v.get("ts").and_then(serde_json::Value::as_u64).unwrap_or(0),
                v.get("session").and_then(serde_json::Value::as_str).unwrap_or("").to_string(),
                ms,
            )),
            "advisory-issued" => advisory_issued += 1,
            "advisory-repeated" => advisory_repeated += 1,
            ev if ev.starts_with("override-obligation") => {
                overrides += 1;
                unsynced += 1;
            }
            // issue207/D0193: the happy path emits override-consumed; both paths count as overrides
            // and only the failure path counts as unsynced.
            "override-consumed" => overrides += 1,
            ev if ev.starts_with("red-yield") => red_yields += 1,
            _ => {}
        }
        let slot = per_event.entry(event).or_insert((0, 0));
        slot.0 += 1;
        if block {
            slot.1 += 1;
        }
    }
    let (process_defects, tracked_obligations, run_records) = tracked_counts(root);
    let user_prompt_fires = per_event.get("user-prompt").map_or(0, |s| s.0);
    let events_json = event_rows(per_event, &per_event_ms);
    let out = Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "K14/D0180: promotion of any advisory control to blocking cites at least 3 sprints of \
                 this evidence. Machine-local; the ledger schema is frozen in pm.rs and bound by test.",
            ),
        ),
        ("ledgerLines".to_string(), Json::Int(i64::try_from(lines).unwrap_or(i64::MAX))),
        ("malformedLines".to_string(), Json::Int(i64::try_from(malformed).unwrap_or(i64::MAX))),
        ("sessionsSeen".to_string(), Json::Int(i64::try_from(sessions.len()).unwrap_or(i64::MAX))),
        ("perEvent".to_string(), Json::Arr(events_json)),
        ("slowFires".to_string(), slow_fires_json(slow_fires)),
        ("recall".to_string(), recall_degradation(&recall_skips, &recall_slow, user_prompt_fires)),
        ("redYields".to_string(), Json::Int(i64::try_from(red_yields).unwrap_or(i64::MAX))),
        // issue230: spoken advisories vs silent fires, and the repeat-as-ignore signal. APPROXIMATE
        // by stated design: heeded = issued without the same advice hash recurring in-session; a
        // heeded advisory and a never-retried command are indistinguishable, and the note says so.
        ("advisoryIssued".to_string(), Json::Int(i64::try_from(advisory_issued).unwrap_or(i64::MAX))),
        ("advisoryRepeated".to_string(), Json::Int(i64::try_from(advisory_repeated).unwrap_or(i64::MAX))),
        ("advisoryApproxHeedPct".to_string(), if advisory_issued == 0 { Json::Null } else {
            let rep = f64::from(u32::try_from(advisory_repeated).unwrap_or(u32::MAX));
            let iss = f64::from(u32::try_from(advisory_issued).unwrap_or(u32::MAX));
            Json::s(format!("{:.0}", 100.0 - rep * 100.0 / iss))
        }),
        ("advisoryNote".to_string(), Json::s("issued counts advisories that actually SPOKE (silent fires excluded); repeated = the same advice hash again in the same session (the mechanical ignore signal). D0197's revisit condition reads these two numbers.".to_string())),
        ("overrideLedgerEvents".to_string(), Json::Int(i64::try_from(overrides).unwrap_or(i64::MAX))),
        ("overrideObligationsUnsynced".to_string(), Json::Int(i64::try_from(unsynced).unwrap_or(i64::MAX))),
        ("overrideObligationsTracked".to_string(), Json::Int(i64::try_from(tracked_obligations).unwrap_or(i64::MAX))),
        ("processDefectMarks".to_string(), Json::Int(i64::try_from(process_defects).unwrap_or(i64::MAX))),
        (
            "launcherFraction".to_string(),
            if run_records == 0 {
                Json::s("unavailable: no run records yet - reports once the P5 launcher writes .keel/runs/")
            } else {
                Json::Int(i64::try_from(run_records).unwrap_or(i64::MAX))
            },
        ),
        (
            "dirtyTreeRefusals".to_string(),
            Json::s("unavailable: recorded by the P5 launcher at launch time - the PESS-2 watch-item reports here"),
        ),
    ]);
    Ok(out.dump())
}

#[cfg(test)]
mod tests {
    use super::{enforcement_report, latency, slow_fire_phases, LEDGER_ADDITIVE_FIELDS, LEDGER_FIELDS, SLOW_FIRE_MS};

    /// THE SCHEMA FREEZE, bound: a line emitted with exactly the frozen fields parses and counts;
    /// a malformed line is COUNTED as malformed, never silently skipped (K2 applied to evidence).
    #[test]
    #[allow(clippy::expect_used)] // test setup
    fn ledger_schema_is_frozen_and_malformed_lines_are_visible() {
        let root = std::env::temp_dir().join("keel-pm-report");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".keel").join("metrics")).expect("mkdir");
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        let good = r#"{"ts":1,"session":"s1","event":"stop","decision":"block","exit":2,"ms":10}"#;
        let v: serde_json::Value = serde_json::from_str(good).expect("fixture parses");
        let keys: Vec<&str> = v.as_object().expect("obj").keys().map(String::as_str).collect();
        let mut frozen = LEDGER_FIELDS.to_vec();
        frozen.sort_unstable();
        let mut got = keys.clone();
        got.sort_unstable();
        assert_eq!(got, frozen, "the fixture IS the frozen schema");
        for extra in LEDGER_ADDITIVE_FIELDS {
            assert!(!LEDGER_FIELDS.contains(&extra), "an additive field never shadows a core one: {extra}");
        }
        std::fs::write(
            root.join(".keel").join("metrics").join("hooks.jsonl"),
            format!("{good}\nnot json at all\n{good}\n"),
        )
        .expect("write ledger");
        let report = enforcement_report(&root).expect("report");
        let d: serde_json::Value = serde_json::from_str(&report).expect("report json");
        assert_eq!(d["ledgerLines"], 3);
        assert_eq!(d["slowFires"]["count"], 0, "a 10 ms fire is not slow");
        assert_eq!(d["malformedLines"], 1, "a malformed line is visible, never silently skipped");
        assert_eq!(d["sessionsSeen"], 1);
        let per = d["perEvent"].as_array().expect("perEvent");
        assert!(per.iter().any(|e| e["event"] == "stop" && e["fires"] == 2 && e["blocks"] == 2));
        assert!(d["launcherFraction"].as_str().is_some_and(|s| s.contains("unavailable")), "absent P5 data says so");
    }

    /// issue207/D0193: BOTH override paths count as overrides; only the failure path counts as
    /// unsynced. Before the fix the happy path emitted nothing and the report structurally
    /// under-read override pressure in the K14 promotion evidence.
    #[test]
    #[allow(clippy::expect_used)] // test setup
    fn successful_override_consumptions_are_counted() {
        let root = std::env::temp_dir().join("keel-pm-overrides");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".keel").join("metrics")).expect("mkdir");
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        let lines = [
            r#"{"ts":1,"session":"s1","event":"override-consumed","decision":"allow","exit":0,"ms":0}"#,
            r#"{"ts":2,"session":"s1","event":"override-consumed","decision":"allow","exit":0,"ms":0}"#,
            r#"{"ts":3,"session":"s1","event":"override-obligation-UNSYNCED","decision":"block","exit":1,"ms":0}"#,
        ]
        .join("
");
        std::fs::write(root.join(".keel").join("metrics").join("hooks.jsonl"), lines).expect("write ledger");
        let report = enforcement_report(&root).expect("report");
        let d: serde_json::Value = serde_json::from_str(&report).expect("json");
        assert_eq!(d["overrideLedgerEvents"], 3, "consumed + unsynced both count as overrides");
        assert_eq!(d["overrideObligationsUnsynced"], 1, "only the failure path is unsynced");
    }

    /// D0389/issue402: a documented cost is a distribution, and every reported value is one that occurred.
    #[test]
    fn latency_is_nearest_rank_over_the_sample() {
        assert_eq!(latency(&[]), (0, 0, 0, 0));
        assert_eq!(latency(&[7]), (7, 7, 7, 7));
        // the ledger's own shape: mostly zero, a long tail
        let mut s = vec![0u64; 90];
        s.extend([100, 200, 300, 400, 500, 600, 700, 800, 900, 54_439]);
        let (med, p90, p99, max) = latency(&s);
        assert_eq!((med, p90), (0, 0), "the best case is what the median and p90 read on this shape");
        assert_eq!(p99, 900, "p99 is the 99th of 100 sorted values");
        assert_eq!(max, 54_439, "the maximum is reported, not smoothed");
    }

    /// D0389/issue402: a recall skip is COUNTED, its rate is over user-prompt fires, and the turn that
    /// lost its facts is identifiable afterwards by ts and session.
    #[test]
    #[allow(clippy::expect_used)] // test setup
    fn recall_skips_are_counted_with_the_turns_they_cost() {
        let root = std::env::temp_dir().join("keel-pm-recall");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".keel").join("metrics")).expect("mkdir");
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        let lines = [
            r#"{"ts":10,"session":"s1","event":"user-prompt","decision":"allow","exit":0,"ms":700}"#,
            r#"{"ts":20,"session":"s1","event":"user-prompt","decision":"allow","exit":0,"ms":3300}"#,
            r#"{"ts":20,"session":"s1","event":"recall-skipped","decision":"allow","exit":0,"ms":3300}"#,
            r#"{"ts":30,"session":"s2","event":"user-prompt","decision":"allow","exit":0,"ms":0}"#,
            r#"{"ts":40,"session":"s2","event":"user-prompt","decision":"allow","exit":0,"ms":800}"#,
        ]
        .join("\n");
        std::fs::write(root.join(".keel").join("metrics").join("hooks.jsonl"), lines).expect("write ledger");
        let report = enforcement_report(&root).expect("report");
        let d: serde_json::Value = serde_json::from_str(&report).expect("json");
        assert_eq!(d["recall"]["skipped"], 1);
        assert_eq!(d["recall"]["skipRatePct"], "25.00", "one skip over four user-prompt fires");
        let turns = d["recall"]["skippedTurns"].as_array().expect("turns");
        assert_eq!(turns.len(), 1);
        assert!(turns[0]["ts"] == 20 && turns[0]["session"] == "s1" && turns[0]["ms"] == 3300, "the turn is named: {turns:?}");
        let up = d["perEvent"].as_array().expect("perEvent").iter().find(|e| e["event"] == "user-prompt").expect("user-prompt row").clone();
        assert!(up["msMedian"] == 750 || up["msMedian"] == 700 || up["msMedian"] == 800, "a value from the sample: {up}");
        assert_eq!(up["msMax"], 3300, "the tail is reported beside the count");
    }

    /// D0390 (2026-09-09): past the cap the facts are pushed LATE and counted `recall-slow`; the report
    /// counts slow BESIDE the pre-D0390 skipped, each with its rate and named turns.
    #[test]
    #[allow(clippy::expect_used)] // test setup
    fn recall_slow_is_counted_beside_the_pre_d0390_skipped() {
        let root = std::env::temp_dir().join("keel-pm-recall-slow");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".keel").join("metrics")).expect("mkdir");
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        let lines = [
            r#"{"ts":10,"session":"s1","event":"user-prompt","decision":"allow","exit":0,"ms":700}"#,
            r#"{"ts":20,"session":"s1","event":"user-prompt","decision":"allow","exit":0,"ms":4000}"#,
            r#"{"ts":20,"session":"s1","event":"recall-slow","decision":"allow","exit":0,"ms":4000}"#,
            r#"{"ts":30,"session":"s2","event":"user-prompt","decision":"allow","exit":0,"ms":9000}"#,
            r#"{"ts":30,"session":"s2","event":"recall-skipped","decision":"allow","exit":0,"ms":9000}"#,
        ]
        .join("\n");
        std::fs::write(root.join(".keel").join("metrics").join("hooks.jsonl"), lines).expect("write ledger");
        let report = enforcement_report(&root).expect("report");
        let d: serde_json::Value = serde_json::from_str(&report).expect("json");
        assert_eq!(d["recall"]["slow"], 1, "the late push is counted recall-slow");
        assert_eq!(d["recall"]["skipped"], 1, "the pre-D0390 drop is still counted");
        assert_eq!(d["recall"]["slowRatePct"], "33.33", "one slow over three user-prompt fires");
        let slow = d["recall"]["slowTurns"].as_array().expect("slowTurns");
        assert!(slow.len() == 1 && slow[0]["session"] == "s1" && slow[0]["ms"] == 4000, "the slow turn is named: {slow:?}");
        let skipped = d["recall"]["skippedTurns"].as_array().expect("skippedTurns");
        assert!(skipped.len() == 1 && skipped[0]["ms"] == 9000, "the skipped turn is named apart: {skipped:?}");
    }

    /// D0414 / issue429: a slow fire's row carries the phases the hook wrote, longest first; a fast
    /// fire is not a row; a slow line recorded before the field says it is unattributed rather than
    /// reading as an empty attribution. The threshold itself is the known-negative boundary.
    #[test]
    #[allow(clippy::expect_used)] // test setup
    fn a_slow_fire_is_a_row_with_its_phases_and_a_fast_one_is_not() {
        let root = std::env::temp_dir().join("keel-pm-slow-fires");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".keel").join("metrics")).expect("mkdir");
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        let fast = r#"{"ts":1,"session":"s1","event":"stop","decision":"allow","exit":0,"ms":217}"#;
        let slow = r#"{"ts":2,"session":"s1","event":"stop","decision":"allow","exit":0,"ms":28000,"phases":[{"name":"hook:guards","ms":27000},{"name":"guard:priority-inversion (critical path)","ms":26900},{"name":"unattributed","ms":600}]}"#;
        let old = r#"{"ts":3,"session":"s0","event":"stop","decision":"allow","exit":0,"ms":35000}"#;
        std::fs::write(root.join(".keel").join("metrics").join("hooks.jsonl"), format!("{fast}\n{slow}\n{old}\n")).expect("write");
        let d: serde_json::Value = serde_json::from_str(&enforcement_report(&root).expect("report")).expect("json");
        assert_eq!(d["slowFires"]["thresholdMs"], SLOW_FIRE_MS);
        assert_eq!(d["slowFires"]["count"], 2, "the 217 ms fire is not a row");
        let rows = d["slowFires"]["rows"].as_array().expect("rows");
        assert_eq!(rows[0]["ts"], 3, "newest first");
        assert!(rows[0]["phases"].as_str().is_some_and(|s| s.contains("unattributed: recorded before D0414")), "{}", rows[0]);
        assert_eq!(rows[1]["ms"], 28000);
        assert_eq!(rows[1]["phases"][0]["name"], "hook:guards", "the phase the hook named first is read back first");
        assert_eq!(rows[1]["phases"][1]["name"], "guard:priority-inversion (critical path)");
    }

    /// The ledger emitter's decision: below the threshold no field is written at all (the
    /// known-negative); at it the field exists and always ends in an accounted remainder.
    #[test]
    fn a_fast_fire_carries_no_phases_and_a_slow_one_always_carries_the_remainder() {
        assert!(slow_fire_phases(SLOW_FIRE_MS - 1).is_none());
        let v = slow_fire_phases(SLOW_FIRE_MS).expect("a slow fire attributes itself");
        let rows = v.as_array().expect("array");
        assert!(rows.iter().any(|r| r["name"] == "unattributed"), "{v}");
    }
}
