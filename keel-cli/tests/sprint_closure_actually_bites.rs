//! Arming probe for guard 55, `sprint-closure` (D0260) — the sprint483 class.
//!
//! The miss it exists to prevent: a sprint's task is finished and verified, its TestResult is
//! never appended, and the ready frontier therefore serves FINISHED work as OPEN work for weeks.
//! One occurrence in 496 sprints, found only when D0258's priority-assessment step first read the
//! frontier's head item by item.
//!
//! A guard that has never been observed to fail is a claim, not a control. This probe fires it in
//! the refuse case AND pins the exemption that keeps it from becoming the issue272 lockout — an
//! IN-PROGRESS sprint has unstamped tasks by definition and must still commit.

use std::path::{Path, PathBuf};

fn delivery(root: &Path) -> PathBuf {
    let d = root.join(".tracking").join("delivery");
    std::fs::create_dir_all(&d).expect("mkdir");
    d
}

/// A sprint file with one task, stamped or not.
fn sprint(root: &Path, n: u32, task: &str, stamped: bool) {
    let result = if stamped {
        format!("        part {task}DoDR1 : TestResult {{ :>> id = \"r{n}\"; :>> outcome = VerdictKind::pass; }}\n")
    } else {
        String::new()
    };
    let body = format!(
        "package S{n} {{\n    action def Run{n} {{\n        action {task};\n\
         \x20       verification {task}DoD : Test {{ :>> id = \"t{n}\"; }}\n{result}    }}\n}}\n"
    );
    std::fs::write(delivery(root).join(format!("sprint{n:03}_probe.sysml")), body).expect("write");
}

fn fresh(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-closure-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".engine")).expect("mkdir");
    root
}

fn violations(root: &Path) -> Vec<String> {
    keel_cli::guards::sprint_closure(root).violations
}

// ── THE REFUSE CASE: the sprint483 reproduction ───────────────────────────────────────────────

#[test]
fn a_sprint_left_unstamped_while_work_moved_on_is_a_violation() {
    let root = fresh("bites");
    sprint(&root, 483, "storyLeftUnstamped", false);
    sprint(&root, 484, "storyMovedOn", true); // work moved on — 483 is no longer in progress
    let v = violations(&root);
    assert_eq!(
        v.len(),
        1,
        "an unstamped task in a sprint the work has MOVED ON from must be a violation — this is \
         exactly how finished work was served as ready for three weeks (sprint483): {v:?}"
    );
    assert!(
        v[0].contains("storyLeftUnstamped") && v[0].contains("sprint483"),
        "the violation names the task AND its sprint, or it cannot be acted on: {v:?}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE ALLOW CASE: the issue272 lesson — never lock out legitimate work ──────────────────────

#[test]
fn the_newest_sprint_may_be_unstamped_because_it_is_in_progress() {
    let root = fresh("inprogress");
    sprint(&root, 483, "storyClosed", true);
    sprint(&root, 484, "storyStillRunning", false); // the sprint being worked right now
    assert!(
        violations(&root).is_empty(),
        "the highest-numbered sprint is IN PROGRESS and has unstamped tasks by definition — \
         gating it would make the guard a lockout, the issue272 failure repeated"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE DISCRIMINATION: it is the STAMP that matters, not the file's existence ────────────────

#[test]
fn stamping_the_task_clears_the_violation() {
    let root = fresh("clears");
    sprint(&root, 483, "storyLeftUnstamped", false);
    sprint(&root, 484, "storyMovedOn", true);
    assert_eq!(violations(&root).len(), 1, "precondition");
    sprint(&root, 483, "storyLeftUnstamped", true); // append the owed result
    assert!(
        violations(&root).is_empty(),
        "appending the TestResult must clear it — otherwise the guard reports a state no action \
         can fix, which is how a control gets bypassed rather than obeyed"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE LIVE TREE: the ratchet starts clean, and that is asserted, not assumed ────────────────

#[test]
fn this_repository_is_clean_under_the_new_guard() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root").to_path_buf();
    let report = keel_cli::guards::sprint_closure(&root);
    assert!(
        report.violations.is_empty(),
        "the guard is a RATCHET on an already-perfect record (496/496 sprints stamp every task); \
         a violation here means it was mis-scoped, not that the tree is dirty: {:?}",
        report.violations
    );
    assert!(report.scanned > 400, "must actually scan the corpus, not silently see nothing: {}", report.scanned);
    let _ = std::fs::remove_dir_all(root.join("nonexistent"));
}
