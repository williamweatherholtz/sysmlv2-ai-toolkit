//! The launched-run lifecycle (D0181/D0182 — P5): prepare → spawn (serve's SSE path) → finish.
//!
//! WHAT A RUN IS: a console-launched, human-approved process instance executing in fresh bounded
//! context (`claude -p`). This module owns the parts that must be TESTABLE outside the SSE stream:
//! the dirty-tree refusal and snapshot at prepare, and the post-run gate + records at finish.
//!
//! RECORDS, hybrid residence (resolved fork 4): every run writes a machine-local record
//! (`.keel/runs/<id>.json` — schema declared below, exempt from existence checks); a run with a
//! NON-EMPTY diff also writes ONE tracked summary per run (`.tracking/runs/run-<id>.sysml`), a
//! `Test` + `TestResult` pair — the post-run gate as a verification, `judgedAgainst` the HEAD the
//! run started from (K12), judged by the RUN'S STAMPED ACTOR, written AFTER the gate. Consumers,
//! named per D0144: the console run view, PM's launcher-fraction metric, and the human's review
//! queue (the summary text routes the diff to review UNCONDITIONALLY, green or red — the gate
//! verifies form; substance is the human diff review, D0094/D0096).
//!
//! SINGLE-WRITER-PER-TREE (`D0182`): launch REFUSES on a dirty tree, which also serializes
//! consecutive launches behind human review/commit of the prior run — the stated consequence the
//! panel accepted. Worktree-per-run stays deferred behind its named `DoR` (P5.6).
//!
//! ROLL-UP POLICY (charter note): tracked summaries are one file per run under `.tracking/runs/`;
//! when their count grows enough to move turn-gate latency, they are archived under the governed
//! D0067 migration process — never silently deleted.

use std::path::{Path, PathBuf};

/// The machine-local run-record schema, declared here and exempt from existence checks by design.
///
/// Fields: `id, process, actor, startedTs, headAtSpawn, fingerprintAtSpawn, exit, turns,
/// durationMs, timedOut, gate ("green"|"red"|"not-run"), diffFiles`.
pub const RUN_RECORD_FIELDS: [&str; 12] = [
    "id", "process", "actor", "startedTs", "headAtSpawn", "fingerprintAtSpawn", "exit", "turns", "durationMs",
    "timedOut", "gate", "diffFiles",
];

/// A prepared (not yet spawned) run.
pub struct RunSetup {
    pub id: String,
    pub process: String,
    pub actor: String,
    pub head_at_spawn: String,
    pub fingerprint_at_spawn: u64,
    pub started: std::time::SystemTime,
}

/// Refuse-or-snapshot at launch time (D0182/P5.3).
///
/// # Errors
/// A dirty tree (the single-writer rule — the refusal is also PM's `launch-dirty-refusal` ledger
/// event, the PESS-2 watch metric), an unbound actor, or an unreadable HEAD.
pub fn prepare(root: &Path, process: &str) -> Result<RunSetup, String> {
    let dirty = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| format!("git status failed: {e}"))?;
    let dirty_list = String::from_utf8_lossy(&dirty.stdout);
    if !dirty_list.trim().is_empty() {
        ledger(root, "launch-dirty-refusal", process);
        return Err(format!(
            "launch refused: the tree is DIRTY ({} path(s)) — a run's diff must be attributable to the run alone (D0182 single-writer). Commit or revert the pending changes first; console accepts land uncommitted by design, so use the console commit action after review.",
            dirty_list.lines().count()
        ));
    }
    let actor = crate::actor::resolve(root, None).map_err(|e| format!("launch refused: {e}"))?;
    let head = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .ok_or("launch refused: cannot read HEAD (K12 needs the spawn-time tree identity)")?;
    Ok(RunSetup {
        id: crate::write::gen_uuid(),
        process: process.to_string(),
        actor,
        head_at_spawn: head,
        fingerprint_at_spawn: crate::fingerprint::of(root),
        started: std::time::SystemTime::now(),
    })
}

/// The finished run's verdicts.
pub struct RunOutcome {
    pub gate_green: bool,
    pub diff_files: Vec<String>,
    pub summary_path: Option<PathBuf>,
    pub local_record: PathBuf,
    pub problems: Vec<String>,
}

/// Post-run gate + records (D0182/P5.2-P5.4). The tree was CLEAN at spawn, so the working-tree diff
/// IS the run's diff and a whole-tree gate verdict is attributable to the run (single-writer).
///
/// # Errors
/// Only on an unwritable `.keel/` — gate redness is an OUTCOME (`gate_green: false`), not an error.
#[allow(clippy::too_many_lines)]
pub fn finish(root: &Path, setup: &RunSetup, exit: Option<i32>, turns: u64, timed_out: bool) -> Result<RunOutcome, String> {
    crate::fingerprint::new_epoch(); // the run wrote; the memo must not serve the spawn-time value
    let diff = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    let diff_files: Vec<String> = diff.lines().filter_map(|l| l.get(3..)).map(str::to_string).collect();

    // The gate: validate + activation-filtered guards + blocking rules — the same three surfaces
    // every other tier runs (K15: all re-derivable from the tree; no hook is trusted to have run).
    let mut problems: Vec<String> = Vec::new();
    if !diff_files.is_empty() {
        let report = crate::validate_root(root);
        for (p, d) in report.diagnostics.iter().take(10) {
            problems.push(format!("validate {}:{} {}", p.display(), d.line, d.message));
        }
        for e in report.errors.iter().take(10) {
            problems.push(format!("parse {} {}", e.file.display(), e.message));
        }
        for r in crate::guards::run_all(root) {
            for v in r.violations.iter().take(5) {
                problems.push(format!("[{}] {v}", r.name));
            }
        }
        if let Ok(json) = crate::view::check(root) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&json) {
                let empty = Vec::new();
                for r in v.get("rules").and_then(|x| x.as_array()).unwrap_or(&empty) {
                    if r.get("severity").and_then(|s| s.as_str()) == Some("blocking") {
                        for viol in r.get("violations").and_then(|x| x.as_array()).unwrap_or(&empty).iter().take(5) {
                            problems.push(format!("[rule] {viol}"));
                        }
                    }
                }
            }
        }
        // Fire-ledger evidence against ACCIDENTAL enforcement loss (missing KEEL_BIN, unloaded
        // settings) — conditional on the run having written anything, never tamper-proof.
        let fired_since_spawn = std::fs::read_to_string(root.join(".keel").join("metrics").join("hooks.jsonl"))
            .ok()
            .and_then(|t| {
                let spawn_ts = setup.started.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs();
                Some(t.lines().rev().take(200).any(|l| {
                    serde_json::from_str::<serde_json::Value>(l)
                        .ok()
                        .and_then(|v| v.get("ts").and_then(serde_json::Value::as_u64))
                        .is_some_and(|ts| ts >= spawn_ts)
                }))
            })
            .unwrap_or(false);
        // K5 minimal completion (P5.4): a launched run must have recorded its outputs through the
        // write API - at minimum, SOMETHING landed in .tracking. Full artifact contracts wait for
        // the typed producedArtifact field (D0185 trigger); no checker is built over free text.
        if !diff_files.iter().any(|f| f.replace('\\', "/").contains(".tracking/")) {
            problems.push(
                "the run changed files but recorded NO tracked outputs - a launched process records its results through the write API (K5 minimal completion; relaunch with the recording step, bounded re-prompt)"
                    .to_string(),
            );
        }
        if !fired_since_spawn {
            problems.push(
                "no fire-ledger line since spawn: the run's hooks may not have fired (missing KEEL_BIN or unloaded settings) — enforcement loss is ACCIDENTAL-class evidence, review the transcript (K3)"
                    .to_string(),
            );
        }
    }
    let gate_green = problems.is_empty();

    // Machine-local record — always, empty diff included.
    let runs = root.join(".keel").join("runs");
    std::fs::create_dir_all(&runs).map_err(|e| e.to_string())?;
    let started_ts = setup.started.duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let duration_ms =
        u64::try_from(setup.started.elapsed().map_or(0, |d| d.as_millis())).unwrap_or(u64::MAX);
    let record = serde_json::json!({
        "id": setup.id, "process": setup.process, "actor": setup.actor,
        "startedTs": started_ts, "headAtSpawn": setup.head_at_spawn,
        "fingerprintAtSpawn": setup.fingerprint_at_spawn.to_string(),
        "exit": exit, "turns": turns, "durationMs": duration_ms, "timedOut": timed_out,
        "gate": match (diff_files.is_empty(), gate_green) { (true, _) => "not-run", (false, true) => "green", (false, false) => "red" },
        "diffFiles": diff_files,
    });
    let local_record = runs.join(format!("{}.json", setup.id));
    crate::write::write_atomic(&local_record, record.to_string()).map_err(|e| e.to_string())?;

    // ONE tracked summary per NON-EMPTY-diff run (empty-diff runs stay local-only), written AFTER
    // the gate, under the RUN'S stamped actor, carrying the spawn HEAD as judgedAgainst (K12).
    let summary_path = if diff_files.is_empty() {
        None
    } else {
        let short = setup.id.get(..8).unwrap_or("run");
        let dir = root.join(".tracking").join("runs");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let path = dir.join(format!("run-{short}.sysml"));
        let verdict = if gate_green { "pass" } else { "fail" };
        let esc = |s: &str| s.replace('"', "'").replace(['\n', '\r'], " ");
        let text = format!(
            "// LAUNCHED-RUN SUMMARY (auto-recorded after the post-run gate, D0182/K12). The gate verifies\n\
             // FORM; substance verification is the human diff review, which this run's diff awaits\n\
             // UNCONDITIONALLY, green or red (D0094/D0096).\n\
             package Run{short} {{\n\
             \x20   private import EngineElement::*;\n\
             \x20   private import EngineVerification::*;\n\n\
             \x20   verification run{short}Gate : Test {{ :>> id = \"{}\"; :>> title = \"launched run {short}: {} - post-run gate\"; :>> createdAt = \"{}\"; :>> createdBy = \"{}\"; :>> method = VerificationMethod::test; :>> procedureText = \"Post-run gate over the run-start-to-working-tree diff ({} file(s): {}). validate + activation-filtered guards + blocking rules. Turns {}; duration {}ms; timedOut {}. THE DIFF AWAITS HUMAN REVIEW regardless of this verdict. Problems at gate: {}\"; }}\n\
             \x20   part run{short}GateR1 : TestResult {{ :>> id = \"{}\"; :>> outcome = VerdictKind::{verdict}; :>> judgedAgainst = \"{}\"; :>> judgedAt = \"{}\"; :>> judgedBy = \"{}\"; }}\n\
             }}\n",
            crate::write::gen_uuid(),
            esc(&setup.process),
            crate::scaffold::today(),
            setup.actor,
            diff_files.len(),
            esc(&diff_files.join(", ")),
            turns,
            duration_ms,
            timed_out,
            if problems.is_empty() { "none".to_string() } else { esc(&problems.join(" | ")).chars().take(1500).collect::<String>() },
            crate::write::gen_uuid(),
            setup.head_at_spawn,
            crate::scaffold::today(),
            setup.actor,
        );
        crate::write::write_atomic(&path, text).map_err(|e| e.to_string())?;
        Some(path)
    };
    Ok(RunOutcome { gate_green, diff_files, summary_path, local_record, problems })
}

/// Minimal ledger append for launcher events (same file + frozen schema as the hook ledger).
fn ledger(root: &Path, event: &str, session: &str) {
    use std::io::Write as _;
    let dir = root.join(".keel").join("metrics");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let line = format!(
        "{}\n",
        serde_json::json!({"ts": ts, "session": session, "event": event, "decision": "block", "exit": 1, "ms": 0})
    );
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("hooks.jsonl"))
        .and_then(|mut f| f.write_all(line.as_bytes()));
}

#[cfg(test)]
mod tests {
    use super::{finish, prepare};

    fn git_root(tag: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("keel-launcher-{tag}"));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        std::fs::write(
            root.join(".tracking").join("actors.sysml"),
            "package ProjectActors {\n    private import EngineElement::*;\n\n    part hum : Person { :>> name = \"H\"; }\n}\n",
        )
        .expect("actors");
        // machine-local state is gitignored exactly as `keel init` scaffolds it — without this the
        // ledger/binding writes would dirty the tree the test is asserting clean
        std::fs::write(root.join(".gitignore"), ".keel/\n").expect("gitignore");
        let run = |args: &[&str]| {
            assert!(
                crate::gitx::git().arg("-C").arg(&root).args(args).output().expect("git").status.success(),
                "git {args:?}"
            );
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "t@t"]);
        run(&["config", "user.name", "t"]);
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "base"]);
        root
    }

    /// D0182: a dirty tree refuses (and the refusal is a ledger event — PM's watch metric); a clean
    /// tree prepares with the spawn HEAD captured for K12.
    #[test]
    fn dirty_tree_refuses_and_clean_tree_snapshots() {
        let root = git_root("prepare");
        std::fs::write(root.join("scratch.txt"), "uncommitted").expect("write");
        let refused = prepare(&root, "critique").map(|_| ());
        assert!(refused.is_err(), "dirty tree must refuse");
        assert!(refused.unwrap_err().contains("DIRTY"));
        let ledger = std::fs::read_to_string(root.join(".keel").join("metrics").join("hooks.jsonl")).expect("ledger");
        assert!(ledger.contains("launch-dirty-refusal"), "the refusal is PM's watch metric");
        std::fs::remove_file(root.join("scratch.txt")).expect("rm");
        std::fs::write(root.join(".keel").join("actor"), "hum").expect("bind");
        let setup = prepare(&root, "critique").expect("clean tree prepares");
        assert!(!setup.head_at_spawn.is_empty(), "K12 needs the spawn HEAD");
    }

    /// P5.2/P5.3: an empty-diff run stays local-only; a run that wrote gets the gate AND one tracked
    /// summary carrying the spawn HEAD as judgedAgainst, under the run's actor.
    #[test]
    fn empty_diff_stays_local_and_written_diff_gets_gated_summary() {
        let root = git_root("finish");
        std::fs::create_dir_all(root.join(".keel")).expect("mkdir");
        std::fs::write(root.join(".keel").join("actor"), "hum").expect("bind");
        let setup = prepare(&root, "demo").expect("prepare");
        let clean = finish(&root, &setup, Some(0), 3, false).expect("finish clean");
        assert!(clean.summary_path.is_none(), "empty diff -> no tracked summary");
        assert!(clean.local_record.exists(), "the local record always exists");
        // the run writes a (valid) tracking file
        std::fs::write(
            root.join(".tracking").join("note.sysml"),
            "package RunNote {\n    private import EngineElement::*;\n}\n",
        )
        .expect("run write");
        let wrote = finish(&root, &setup, Some(0), 5, false).expect("finish wrote");
        let summary = wrote.summary_path.expect("non-empty diff -> tracked summary");
        let text = std::fs::read_to_string(&summary).expect("summary readable");
        assert!(text.contains(&format!("judgedAgainst = \"{}\"", setup.head_at_spawn)), "K12: spawn HEAD");
        assert!(text.contains("judgedBy = \"hum\""), "the RUN'S stamped actor, not the machine binding");
        assert!(text.contains("AWAITS HUMAN REVIEW"), "review is unconditional");
    }
}
