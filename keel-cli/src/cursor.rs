//! `keel advance` (D0209 clause 3, dcProcessCursor — the process cursor; D0436, dcAdvanceRefusesAnyBoundStep).
//!
//! Two forms, one question - may the next step be taken?
//!
//! **A sprint** (`keel advance <sprint> [--to <Gate>]`). The ceremony is a `ProcessStep` machine with
//! per-run pass state — each step is a `verification <...>Gate` whose passing `TestResult` is the
//! step's verify-Test. The cursor is the first defined gate not yet passed; `--to <Gate>` is
//! permitted only if every DEFINED gate earlier than the target is passed, otherwise REFUSED (exit 1)
//! naming the unpassed earlier step. That is "step N+1 is refused until step N's verify-Test passes"
//! (D0098 order-gating, bounded — no topology inversion, no stored state). The gate ORDER is read
//! from the tree - `orient::gate_order`, the delivery workflow's declared chain (D0435) - never a
//! compiled table.
//!
//! **A process** (`keel advance <process> [--to <step>]`, D0436). Any process whose steps declare
//! `checkedBy` (D0434): a step bound to a guard or a declared rule is evaluated on the tree NOW; a
//! `gate:<phase>` binding needs a run to evaluate against and is reported so; an unbound step is
//! judgment and never blocks. `--to <step>` is REFUSED while an earlier step's bound check is red,
//! naming the step, the check and its first violation. A process with no bound step prints
//! `unenforceable-by-step` and exits 0 - the absence of a check is reported, never enforced as a
//! refusal (D0098).
//!
//! Everything here is COMPUTED from the tree every call (never stored, D0018).

use std::path::{Path, PathBuf};

use crate::orient::{gate_order, gate_passed};

/// A gate is DEFINED in a sprint when it has a `verification <...>Gate : Test` declaration (the
/// space-before-colon distinguishes the Test from its `<...>GateR<n> : TestResult`).
fn gate_defined(text: &str, gate: &str) -> bool {
    text.contains(&format!("{gate}Gate : Test"))
}

/// Resolve a sprint argument to a single delivery file: an exact path, else the unique
/// `.tracking/delivery/*.sysml` whose file stem CONTAINS the argument (so "443" or a slug works).
/// `Ok(None)` when nothing matches - the caller then tries the process form.
fn resolve_delivery(root: &Path, arg: &str) -> Result<Option<PathBuf>, String> {
    let direct = Path::new(arg);
    if direct.is_file() {
        return Ok(Some(direct.to_path_buf()));
    }
    let delivery = root.join(".tracking").join("delivery");
    let needle = arg.to_lowercase();
    let mut hits: Vec<PathBuf> = crate::collect_sysml(&delivery)
        .into_iter()
        .filter(|p| p.file_stem().is_some_and(|s| s.to_string_lossy().to_lowercase().contains(&needle)))
        .collect();
    hits.sort();
    match hits.len() {
        0 => Ok(None),
        1 => Ok(Some(hits.remove(0))),
        _ => Err(format!(
            "advance: '{arg}' matches {} delivery files — be more specific: {}",
            hits.len(),
            hits.iter().filter_map(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).collect::<Vec<_>>().join(", ")
        )),
    }
}

/// The cursor position for a delivery file: the defined gates, which are passed, and the first
/// defined-but-unpassed gate (the current step, `None` when the ceremony is complete).
struct Cursor {
    defined: Vec<String>,
    passed: Vec<String>,
    current: Option<String>,
}

#[must_use]
fn compute(text: &str, order: &[String]) -> Cursor {
    let defined: Vec<String> = order.iter().filter(|g| gate_defined(text, g)).cloned().collect();
    let passed: Vec<String> = defined.iter().filter(|g| gate_passed(text, g)).cloned().collect();
    let current = defined.iter().find(|g| !passed.contains(g)).cloned();
    Cursor { defined, passed, current }
}

/// An out-of-order pass already recorded in the tree: a passed gate with an EARLIER defined gate
/// that is unpassed. Returns `(later, earlier)` pairs. Mirrors the `ceremony` guard's post-hoc check.
fn out_of_order(c: &Cursor, order: &[String]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (i, g) in order.iter().enumerate() {
        if !c.passed.contains(g) {
            continue;
        }
        for earlier in order.iter().take(i) {
            if c.defined.contains(earlier) && !c.passed.contains(earlier) {
                out.push((g.clone(), earlier.clone()));
            }
        }
    }
    out
}

// ── the process form (D0436) ────────────────────────────────────────────────────────────────────

/// One step of a process as `keel advance` reads it: its name and its `checkedBy`, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
struct StepBinding {
    step: String,
    check: Option<String>,
}

/// The `ProcessStep`s declared after `action <process> : Process` and before the next `Process` in
/// its file, in declaration order, each with its `checkedBy`. `None` when no such process exists.
fn process_steps(text: &str, process: &str) -> Option<Vec<StepBinding>> {
    let mut in_process = false;
    let mut found = false;
    let mut steps: Vec<StepBinding> = Vec::new();
    for raw in text.lines() {
        let t = raw.trim_start();
        if t.starts_with("//") {
            continue;
        }
        if let Some(rest) = t.strip_prefix("action ") {
            if let Some((name, ty)) = rest.split_once(':') {
                let ty = ty.trim_start();
                if ty.starts_with("ProcessStep") {
                    if in_process {
                        steps.push(StepBinding { step: name.trim().to_owned(), check: None });
                    }
                    continue;
                }
                if ty.starts_with("Process") {
                    if in_process {
                        break; // the next process: this one's steps are over
                    }
                    in_process = name.trim() == process;
                    found |= in_process;
                    continue;
                }
            }
        }
        if !in_process {
            continue;
        }
        if let Some(pos) = t.find(":>> checkedBy") {
            let after = t[pos + ":>> checkedBy".len()..].trim_start();
            if let Some(after) = after.strip_prefix('=') {
                let name = after.trim_start().strip_prefix('"').and_then(|v| v.split_once('"')).map(|(n, _)| n.to_owned());
                if let Some(last) = steps.last_mut() {
                    last.check = name;
                }
            }
        }
    }
    found.then_some(steps)
}

/// Every `action <name> : Process` a file declares, in order.
fn processes_in(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let t = raw.trim_start();
        if t.starts_with("//") {
            continue;
        }
        if let Some(rest) = t.strip_prefix("action ") {
            if let Some((name, ty)) = rest.split_once(':') {
                let ty = ty.trim_start();
                if ty.starts_with("Process") && !ty.starts_with("ProcessStep") {
                    out.push(name.trim().to_owned());
                }
            }
        }
    }
    out
}

/// Resolve a process argument: the name of an `action <name> : Process` under `.engine/processes/`,
/// or a process file's stem when that file declares exactly one process (a stem declaring several
/// names them and asks for one).
fn resolve_process(root: &Path, arg: &str) -> Result<(String, Vec<StepBinding>), String> {
    let dir = root.join(".engine").join("processes");
    let mut by_stem: Option<(PathBuf, Vec<String>)> = None;
    for path in crate::collect_sysml(&dir) {
        let Ok(text) = crate::corpus::read_to_string(&path) else { continue };
        if let Some(steps) = process_steps(&text, arg) {
            return Ok((arg.to_owned(), steps));
        }
        if path.file_stem().is_some_and(|s| s.to_string_lossy().eq_ignore_ascii_case(arg)) {
            by_stem = Some((path.clone(), processes_in(&text)));
        }
    }
    match by_stem {
        Some((path, names)) if names.len() == 1 => {
            let Ok(text) = crate::corpus::read_to_string(&path) else {
                return Err(format!("advance: cannot read {}", path.display()));
            };
            let Some(name) = names.into_iter().next() else {
                return Err(format!("advance: {} declares no process", path.display()));
            };
            let steps = process_steps(&text, &name).unwrap_or_default();
            Ok((name, steps))
        }
        Some((path, names)) => Err(format!(
            "advance: {} declares {} processes — name one: {}",
            path.display(),
            names.len(),
            names.join(", ")
        )),
        None => Err(format!(
            "advance: '{arg}' is neither a delivery file under .tracking/delivery/ nor a process under .engine/processes/ (an `action <name> : Process`, or a file stem declaring one)"
        )),
    }
}

/// What a step's binding says about it on this tree, now.
#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    /// Bound to a guard or rule that holds on the tree.
    Green,
    /// Bound to a guard or rule with a violation - the first one.
    Red(String),
    /// Bound to `gate:<phase>`: a per-run check, evaluated against a sprint (`keel advance <sprint>`).
    NeedsRun,
    /// No `checkedBy`: judgment, reported and never blocking.
    Judgment,
    /// Bound to a name nothing runs - `step-check-resolves` fails this at the commit; here it is red.
    Unresolved,
}

/// Evaluate every step's binding. Rules are read once (`view::check` evaluates the whole set).
fn evaluate(root: &Path, steps: &[StepBinding]) -> Vec<Verdict> {
    let mut rules: Option<serde_json::Value> = None;
    steps
        .iter()
        .map(|s| {
            let Some(check) = s.check.as_deref() else { return Verdict::Judgment };
            if check.starts_with("gate:") {
                return Verdict::NeedsRun;
            }
            if let Some(report) = crate::guards::run_one(check, root) {
                return report
                    .violations
                    .first()
                    .map_or(Verdict::Green, |v| Verdict::Red(v.clone()));
            }
            if rules.is_none() {
                rules = crate::view::check(root).ok().and_then(|j| serde_json::from_str(&j).ok());
            }
            let empty = Vec::new();
            let hit = rules
                .as_ref()
                .and_then(|v| v.get("rules"))
                .and_then(|r| r.as_array())
                .unwrap_or(&empty)
                .iter()
                .find(|r| r.get("rule").and_then(|n| n.as_str()) == Some(check));
            hit.map_or(Verdict::Unresolved, |r| {
                r.get("violations")
                    .and_then(|v| v.as_array())
                    .and_then(|v| v.first())
                    .map_or(Verdict::Green, |v| {
                        Verdict::Red(v.as_str().map_or_else(|| v.to_string(), str::to_owned))
                    })
            })
        })
        .collect()
}

fn describe(s: &StepBinding, v: &Verdict) -> String {
    let check = s.check.as_deref().unwrap_or("");
    match v {
        Verdict::Green => format!("  {:<28} checkedBy {check:<28} GREEN", s.step),
        Verdict::Red(first) => format!("  {:<28} checkedBy {check:<28} RED - {first}", s.step),
        Verdict::NeedsRun => format!("  {:<28} checkedBy {check:<28} needs a run (keel advance <sprint>)", s.step),
        Verdict::Judgment => format!("  {:<28} (judgment - no checkedBy)", s.step),
        Verdict::Unresolved => format!("  {:<28} checkedBy {check:<28} UNRESOLVED - nothing of that name runs (step-check-resolves)", s.step),
    }
}

/// `keel advance <process> [--to <step>]` (D0436).
fn advance_process(root: &Path, name: &str, steps: &[StepBinding], to: Option<&String>) -> i32 {
    if !steps.iter().any(|s| s.check.is_some()) {
        println!(
            "advance: {name} unenforceable-by-step — none of its {} steps declares checkedBy; every step is judgment (keel show hardening: stepEnforcement).",
            steps.len()
        );
        return 0;
    }
    let verdicts = evaluate(root, steps);
    if let Some(target) = to {
        let Some(t_idx) = steps.iter().position(|s| s.step.eq_ignore_ascii_case(target)) else {
            eprintln!(
                "advance: '{target}' is not a step of {name} (steps: {})",
                steps.iter().map(|s| s.step.as_str()).collect::<Vec<_>>().join(", ")
            );
            return 1;
        };
        let blocking: Vec<String> = steps
            .iter()
            .take(t_idx)
            .zip(&verdicts)
            .filter_map(|(s, v)| match v {
                Verdict::Red(first) => Some(format!("{} ({}: {first})", s.step, s.check.as_deref().unwrap_or(""))),
                Verdict::Unresolved => Some(format!("{} ({} - nothing of that name runs)", s.step, s.check.as_deref().unwrap_or(""))),
                _ => None,
            })
            .collect();
        let judged: Vec<&str> = steps
            .iter()
            .take(t_idx)
            .zip(&verdicts)
            .filter(|(_, v)| matches!(v, Verdict::Judgment | Verdict::NeedsRun))
            .map(|(s, _)| s.step.as_str())
            .collect();
        let target_step = steps.get(t_idx).map_or(target.as_str(), |s| s.step.as_str());
        if blocking.is_empty() {
            println!(
                "advance: {name} -> {target_step} PERMITTED — every earlier step bound to a guard or rule holds on the tree now.{}",
                if judged.is_empty() { String::new() } else { format!(" Not judged here: {}.", judged.join(", ")) }
            );
            0
        } else {
            println!(
                "advance: {name} -> {target_step} REFUSED — earlier step(s) whose bound check is red: {}. A step's check must hold before the next step (D0098 order-gating, D0436).",
                blocking.join("; ")
            );
            1
        }
    } else {
        println!("cursor: {name} (process form — guard and rule bindings evaluated on the tree now)");
        for (s, v) in steps.iter().zip(&verdicts) {
            println!("{}", describe(s, v));
        }
        0
    }
}

/// `keel advance <sprint|process> [--to <Gate|step>]`.
///
/// # Returns
/// Exit code 0 when the requested transition is permitted (or the status query is clean); 1 when a
/// transition is REFUSED (an earlier step's verify-Test has not passed, or its bound check is red) or
/// the tree already holds an out-of-order pass, or on a usage/resolution error.
#[must_use]
pub fn advance_cmd(args: &[String], root: &Path) -> i32 {
    let order = gate_order(root);
    let Some(arg) = args.iter().find(|a| !a.starts_with("--")) else {
        eprintln!(
            "usage: keel advance <sprint|process> [--to <Gate|step>]   (Gate: {})",
            if order.is_empty() { "none declared".to_owned() } else { order.join(" | ") }
        );
        return 1;
    };
    let to = args.iter().position(|a| a == "--to").and_then(|i| args.get(i + 1));
    let path = match resolve_delivery(root, arg) {
        Ok(Some(p)) => p,
        Ok(None) => {
            return match resolve_process(root, arg) {
                Ok((name, steps)) => advance_process(root, &name, &steps, to),
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            };
        }
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("advance: cannot read {}", path.display());
        return 1;
    };
    let sprint_name = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    if order.is_empty() {
        println!("advance: {sprint_name} unenforceable-by-step — no ProcessStep binds a `gate:<phase>` of a declared workflow chain, so this tree has no ceremony order (D0435).");
        return 0;
    }
    let c = compute(&text, &order);
    if c.defined.is_empty() {
        println!("advance: {sprint_name} declares no ceremony gates — nothing to advance.");
        return 0;
    }

    // A forward transition request: --to <Gate>.
    if let Some(target) = to {
        let Some((t_idx, target_name)) = order.iter().enumerate().find(|(_, g)| g.eq_ignore_ascii_case(target)) else {
            eprintln!("advance: '{target}' is not a ceremony gate (expected one of: {})", order.join(", "));
            return 1;
        };
        // Permitted only if every DEFINED gate strictly earlier than the target is passed.
        let blocking: Vec<&str> = order
            .iter()
            .take(t_idx)
            .filter(|g| c.defined.contains(g) && !c.passed.contains(g))
            .map(String::as_str)
            .collect();
        if blocking.is_empty() {
            println!("advance: {sprint_name} -> {target_name} PERMITTED — every earlier defined step has a passing verify-Test.");
            0
        } else {
            println!(
                "advance: {sprint_name} -> {target_name} REFUSED — earlier step(s) not passed: {}. \
                 A step's verify-Test must pass before the next step (D0098 order-gating, D0209 clause 3).",
                blocking.join(", ")
            );
            1
        }
    } else {
        // Status query: report the cursor.
        let disorder = out_of_order(&c, &order);
        println!("cursor: {sprint_name}");
        println!("  defined: {}", c.defined.join(" -> "));
        println!("  passed:  {}", if c.passed.is_empty() { "(none)".to_string() } else { c.passed.join(", ") });
        match &c.current {
            Some(step) => println!("  current step: {step} (complete its verify-Test to advance)"),
            None => println!("  current step: (none) — all defined gates passed, ceremony complete"),
        }
        if disorder.is_empty() {
            0
        } else {
            for (later, earlier) in &disorder {
                println!("  OUT OF ORDER: {later} passed while earlier {earlier} is unpassed");
            }
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order() -> Vec<String> {
        ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"].iter().map(|s| (*s).to_owned()).collect()
    }

    // A sprint text with the given gates DEFINED and the given gates PASSED.
    fn sprint(defined: &[&str], passed: &[&str]) -> String {
        use std::fmt::Write as _;
        let mut s = String::from("package X {\n");
        for g in defined {
            let _ = writeln!(s, "    verification foo{g}Gate : Test {{ :>> id = \"x\"; }}");
        }
        for g in passed {
            let _ = writeln!(
                s,
                "    part foo{g}GateR1 : TestResult {{ :>> id = \"y\"; :>> outcome = VerdictKind::pass; }}"
            );
        }
        s.push_str("}\n");
        s
    }

    #[test]
    fn cursor_is_first_defined_unpassed_gate() {
        let all = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];
        let c = compute(&sprint(&all, &["Refine", "Standup"]), &order());
        assert_eq!(c.current.as_deref(), Some("Implement"));
        assert_eq!(c.passed, vec!["Refine", "Standup"]);
    }

    #[test]
    fn complete_sprint_has_no_current_step() {
        let all = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];
        let c = compute(&sprint(&all, &all), &order());
        assert_eq!(c.current, None);
    }

    #[test]
    fn out_of_order_pass_is_detected() {
        let all = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];
        // CloseOut passed while Implement (earlier) is not.
        let c = compute(&sprint(&all, &["Refine", "Standup", "CloseOut"]), &order());
        let d = out_of_order(&c, &order());
        assert!(d.contains(&("CloseOut".to_owned(), "Implement".to_owned())), "{d:?}");
        assert!(d.contains(&("CloseOut".to_owned(), "Review".to_owned())), "{d:?}");
    }

    #[test]
    fn defined_needs_the_test_declaration_not_the_result() {
        // A file with only the RESULT line must not count the gate as defined.
        let only_result = "package X {\n    part fooReviewGateR1 : TestResult { :>> outcome = VerdictKind::pass; }\n}\n";
        assert!(!gate_defined(only_result, "Review"));
        assert!(gate_defined("verification fooReviewGate : Test { }", "Review"));
    }

    // ── the process form (D0436) ────────────────────────────────────────────────────────────────

    const TWO_PROCESSES: &str = "package P {\n\
        \x20   action alpha : Process { :>> purpose = \"a\"; }\n\
        \x20   action a1 : ProcessStep {\n        :>> actionText = \"first\";\n        :>> owner = Owner::ai;\n        :>> checkedBy = \"doc-guard-count\";\n    }\n\
        \x20   action a2 : ProcessStep {\n        :>> actionText = \"second\";\n        :>> owner = Owner::human;\n    }\n\
        \x20   action a3 : ProcessStep {\n        :>> actionText = \"third\";\n        :>> owner = Owner::ai;\n        :>> checkedBy = \"gate:refine\";\n    }\n\
        \x20   action beta : Process { :>> purpose = \"b\"; }\n\
        \x20   action b1 : ProcessStep {\n        :>> actionText = \"only\";\n        :>> owner = Owner::ai;\n    }\n\
        }\n";

    #[test]
    fn process_steps_stop_at_the_next_process_and_carry_their_binding() {
        let alpha = process_steps(TWO_PROCESSES, "alpha").expect("alpha declared");
        assert_eq!(
            alpha,
            vec![
                StepBinding { step: "a1".into(), check: Some("doc-guard-count".into()) },
                StepBinding { step: "a2".into(), check: None },
                StepBinding { step: "a3".into(), check: Some("gate:refine".into()) },
            ]
        );
        let beta = process_steps(TWO_PROCESSES, "beta").expect("beta declared");
        assert_eq!(beta, vec![StepBinding { step: "b1".into(), check: None }]);
        assert!(process_steps(TWO_PROCESSES, "gamma").is_none());
        assert_eq!(processes_in(TWO_PROCESSES), ["alpha", "beta"]);
    }

    struct Fixture(PathBuf);
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A tree whose one process has two bound steps: the first bound to `doc-guard-count`, which is
    /// RED when `.engine/docs/guards.md` hardcodes a total guard count and GREEN when it does not.
    fn fixture(tag: &str, guards_md: &str) -> Fixture {
        let dir = std::env::temp_dir().join(format!("keel-advance-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".engine/processes")).expect("mkdir");
        std::fs::create_dir_all(dir.join(".engine/docs")).expect("mkdir");
        std::fs::create_dir_all(dir.join(".tracking/delivery")).expect("mkdir");
        std::fs::write(dir.join(".engine/docs/guards.md"), guards_md).expect("write");
        std::fs::write(
            dir.join(".engine/processes/probe.sysml"),
            "package Probe {\n    action probe : Process { :>> purpose = \"p\"; }\n    action s1 : ProcessStep {\n        :>> actionText = \"first\";\n        :>> owner = Owner::ai;\n        :>> checkedBy = \"doc-guard-count\";\n    }\n    action s2 : ProcessStep {\n        :>> actionText = \"second\";\n        :>> owner = Owner::ai;\n        :>> checkedBy = \"doc-guard-count\";\n    }\n}\n",
        )
        .expect("write");
        Fixture(dir)
    }

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    /// Known-negative then known-positive on the same fixture: `--to s2` is REFUSED while s1's guard
    /// is red on the tree and PERMITTED once it is green.
    #[test]
    fn a_bound_earlier_step_refuses_the_advance_while_red_and_permits_it_when_green() {
        let red = fixture("red", "this project enforces 999 forward guards\n");
        let wrong = evaluate(&red.0, &process_steps(&std::fs::read_to_string(red.0.join(".engine/processes/probe.sysml")).expect("read"), "probe").expect("probe"));
        assert!(matches!(wrong[0], Verdict::Red(_)), "doc-guard-count must be red on a hardcoded total: {wrong:?}");
        assert_eq!(advance_cmd(&args(&["probe", "--to", "s2"]), &red.0), 1, "refused while s1's guard is red");
        assert_eq!(advance_cmd(&args(&["probe"]), &red.0), 0, "the status query never refuses");

        let green = fixture("green", "the guard count has one home, keel version\n");
        let right = evaluate(&green.0, &process_steps(&std::fs::read_to_string(green.0.join(".engine/processes/probe.sysml")).expect("read"), "probe").expect("probe"));
        assert_eq!(right[0], Verdict::Green, "doc-guard-count must be green with no hardcoded total: {right:?}");
        assert_eq!(advance_cmd(&args(&["probe", "--to", "s2"]), &green.0), 0, "permitted once s1's guard is green");
        assert_eq!(advance_cmd(&args(&["probe", "--to", "s1"]), &green.0), 0, "the first step has nothing before it");
        assert_eq!(advance_cmd(&args(&["probe", "--to", "s9"]), &green.0), 1, "an unknown step is a usage error");
    }

    /// A process with no bound step is reported unenforceable-by-step, exit 0, with or without --to.
    #[test]
    fn a_process_with_no_bound_step_is_unenforceable_not_refused() {
        let f = fixture("unbound", "x\n");
        std::fs::write(
            f.0.join(".engine/processes/probe.sysml"),
            "package Probe {\n    action probe : Process { :>> purpose = \"p\"; }\n    action s1 : ProcessStep {\n        :>> actionText = \"first\";\n        :>> owner = Owner::ai;\n    }\n    action s2 : ProcessStep {\n        :>> actionText = \"second\";\n        :>> owner = Owner::human;\n    }\n}\n",
        )
        .expect("write");
        assert_eq!(advance_cmd(&args(&["probe", "--to", "s2"]), &f.0), 0);
        assert_eq!(advance_cmd(&args(&["probe"]), &f.0), 0);
        assert_eq!(advance_cmd(&args(&["nothing-of-this-name"]), &f.0), 1, "neither a sprint nor a process");
    }
}
