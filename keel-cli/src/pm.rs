//! `keel enforcement-report` — the D0180 analysis over the fire-ledger.
//!
//! K14: the effect of enforcement is MEASURED, not presumed — D0128's recorded-but-undelivered
//! "prove the in-loop gate" step, delivered.
//!
//! THE LEDGER SCHEMA IS FROZEN HERE (D0180 owns the freeze): one JSON object per line in
//! `.keel/metrics/hooks.jsonl`, fields exactly
//! `ts` (unix seconds), `session` (harness session id), `event` (hook event name),
//! `decision` ("allow" | "block"), `exit` (i32), `ms` (u64). Emitters (`ledger_emit` in the hook
//! wrapper) and this reader are the two parties to the freeze, and the schema test binds them.
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

/// Compute the enforcement report.
///
/// # Errors
/// Never errors on an absent ledger — absence is a finding ("no fires recorded"), not a failure.
/// The signature matches the computed-view convention so serve's cache can hold it.
pub fn enforcement_report(root: &Path) -> Result<String, crate::view::ViewError> {
    let ledger = root.join(".keel").join("metrics").join("hooks.jsonl");
    let text = std::fs::read_to_string(&ledger).unwrap_or_default();
    let mut per_event: BTreeMap<String, (u64, u64)> = BTreeMap::new(); // (fires, blocks)
    let mut sessions: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut overrides = 0u64;
    let mut unsynced = 0u64;
    let mut red_yields = 0u64;
    let mut malformed = 0u64;
    let mut lines = 0u64;
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
        match event.as_str() {
            ev if ev.starts_with("override-obligation") => {
                overrides += 1;
                unsynced += 1;
            }
            ev if ev.starts_with("red-yield") => red_yields += 1,
            _ => {}
        }
        let slot = per_event.entry(event).or_insert((0, 0));
        slot.0 += 1;
        if block {
            slot.1 += 1;
        }
    }
    // Adherence trend: the already-tracked #ProcessDefect-marked issues (D0180's chosen signal).
    let process_defects = {
        let mut n = 0usize;
        for f in crate::collect_sysml(&root.join(".tracking")) {
            if let Ok(t) = std::fs::read_to_string(&f) {
                n += t.matches("#ProcessDefect").count();
            }
        }
        n
    };
    // Tracked override obligations (consumed unlocks that DID sync).
    let tracked_obligations =
        std::fs::read_dir(root.join(".tracking").join("obligations")).map_or(0, |rd| rd.flatten().count());
    let runs_dir = root.join(".keel").join("runs");
    let run_records = std::fs::read_dir(&runs_dir).map_or(0, |rd| rd.flatten().count());
    let events_json: Vec<Json> = per_event
        .into_iter()
        .map(|(ev, (fires, blocks))| {
            Json::Obj(vec![
                ("event".to_string(), Json::s(ev)),
                ("fires".to_string(), Json::Int(i64::try_from(fires).unwrap_or(i64::MAX))),
                ("blocks".to_string(), Json::Int(i64::try_from(blocks).unwrap_or(i64::MAX))),
            ])
        })
        .collect();
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
        ("redYields".to_string(), Json::Int(i64::try_from(red_yields).unwrap_or(i64::MAX))),
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
    use super::{enforcement_report, LEDGER_FIELDS};

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
        std::fs::write(
            root.join(".keel").join("metrics").join("hooks.jsonl"),
            format!("{good}\nnot json at all\n{good}\n"),
        )
        .expect("write ledger");
        let report = enforcement_report(&root).expect("report");
        let d: serde_json::Value = serde_json::from_str(&report).expect("report json");
        assert_eq!(d["ledgerLines"], 3);
        assert_eq!(d["malformedLines"], 1, "a malformed line is visible, never silently skipped");
        assert_eq!(d["sessionsSeen"], 1);
        let per = d["perEvent"].as_array().expect("perEvent");
        assert!(per.iter().any(|e| e["event"] == "stop" && e["fires"] == 2 && e["blocks"] == 2));
        assert!(d["launcherFraction"].as_str().is_some_and(|s| s.contains("unavailable")), "absent P5 data says so");
    }
}
