//! `keel advance` (D0209 clause 3, dcProcessCursor — the process cursor).
//!
//! The sprint ceremony (`Refine -> Standup -> Implement -> Review -> CloseOut -> Retro`) is the one
//! declared `ProcessStep` machine with per-run pass state — each step is a `verification <...>Gate`
//! whose passing `TestResult` is the step's verify-Test. This promotes that machine from a
//! reference the AI reads (and the `ceremony` guard checks only AFTER the fact, at commit) into an
//! EXECUTED cursor the AI can query and be REFUSED by IN-LOOP.
//!
//! `keel advance <sprint>` prints the cursor: the current step (the first defined gate not yet
//! passed), what is passed, and what remains; it exits 1 if the tree already holds an out-of-order
//! pass (a later gate passed while an earlier defined gate is unpassed).
//!
//! `keel advance <sprint> --to <Gate>` is the forward gate: permitted only if every DEFINED gate
//! earlier than the target is passed, otherwise REFUSED (exit 1) naming the unpassed earlier step.
//! That is "step N+1 is refused until step N's verify-Test passes" (D0098 order-gating, bounded —
//! no topology inversion, no stored state).
//!
//! The cursor is COMPUTED from the delivery file every call (never stored, D0018). The step order
//! is `orient::GATE_ORDER` — the single source, shared with orient and the `ceremony` guard.

use std::path::Path;

use crate::orient::{gate_passed, GATE_ORDER};

/// A gate is DEFINED in a sprint when it has a `verification <...>Gate : Test` declaration (the
/// space-before-colon distinguishes the Test from its `<...>GateR<n> : TestResult`).
fn gate_defined(text: &str, gate: &str) -> bool {
    text.contains(&format!("{gate}Gate : Test"))
}

/// Resolve a sprint argument to a single delivery file: an exact path, else the unique
/// `.tracking/delivery/*.sysml` whose file stem CONTAINS the argument (so "443" or a slug works).
fn resolve_delivery(root: &Path, arg: &str) -> Result<std::path::PathBuf, String> {
    let direct = Path::new(arg);
    if direct.is_file() {
        return Ok(direct.to_path_buf());
    }
    let delivery = root.join(".tracking").join("delivery");
    let needle = arg.to_lowercase();
    let mut hits: Vec<std::path::PathBuf> = crate::collect_sysml(&delivery)
        .into_iter()
        .filter(|p| {
            p.file_stem()
                .is_some_and(|s| s.to_string_lossy().to_lowercase().contains(&needle))
        })
        .collect();
    hits.sort();
    match hits.len() {
        0 => Err(format!("advance: no delivery file matches '{arg}' (looked in .tracking/delivery/)")),
        1 => Ok(hits.remove(0)),
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
    defined: Vec<&'static str>,
    passed: Vec<&'static str>,
    current: Option<&'static str>,
}

#[must_use]
fn compute(text: &str) -> Cursor {
    let defined: Vec<&'static str> = GATE_ORDER.into_iter().filter(|g| gate_defined(text, g)).collect();
    let passed: Vec<&'static str> = defined.iter().copied().filter(|g| gate_passed(text, g)).collect();
    let current = defined.iter().copied().find(|g| !passed.contains(g));
    Cursor { defined, passed, current }
}

/// An out-of-order pass already recorded in the tree: a passed gate with an EARLIER defined gate
/// that is unpassed. Returns `(later, earlier)` pairs. Mirrors the `ceremony` guard's post-hoc check.
fn out_of_order(c: &Cursor) -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    for (i, g) in GATE_ORDER.into_iter().enumerate() {
        if !c.passed.contains(&g) {
            continue;
        }
        for earlier in GATE_ORDER.into_iter().take(i) {
            if c.defined.contains(&earlier) && !c.passed.contains(&earlier) {
                out.push((g, earlier));
            }
        }
    }
    out
}

/// `keel advance <sprint> [--to <Gate>]`.
///
/// # Returns
/// Exit code 0 when the requested transition is permitted (or the status query is clean); 1 when a
/// transition is REFUSED (an earlier step's verify-Test has not passed) or the tree already holds an
/// out-of-order pass, or on a usage/resolution error.
#[must_use]
pub fn advance_cmd(args: &[String], root: &Path) -> i32 {
    let Some(sprint) = args.iter().find(|a| !a.starts_with("--")) else {
        eprintln!("usage: keel advance <sprint> [--to <Gate>]   (Gate: {})", GATE_ORDER.join(" | "));
        return 1;
    };
    let to = args.iter().position(|a| a == "--to").and_then(|i| args.get(i + 1));
    let path = match resolve_delivery(root, sprint) {
        Ok(p) => p,
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
    let c = compute(&text);
    if c.defined.is_empty() {
        println!("advance: {sprint_name} declares no ceremony gates — nothing to advance.");
        return 0;
    }

    // A forward transition request: --to <Gate>.
    if let Some(target) = to {
        let Some((t_idx, &target_name)) =
            GATE_ORDER.iter().enumerate().find(|(_, g)| g.eq_ignore_ascii_case(target))
        else {
            eprintln!("advance: '{target}' is not a ceremony gate (expected one of: {})", GATE_ORDER.join(", "));
            return 1;
        };
        // Permitted only if every DEFINED gate strictly earlier than the target is passed.
        let blocking: Vec<&str> = GATE_ORDER
            .into_iter()
            .take(t_idx)
            .filter(|g| c.defined.contains(g) && !c.passed.contains(g))
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
        let disorder = out_of_order(&c);
        println!("cursor: {sprint_name}");
        println!("  defined: {}", c.defined.join(" -> "));
        println!("  passed:  {}", if c.passed.is_empty() { "(none)".to_string() } else { c.passed.join(", ") });
        match c.current {
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
        let c = compute(&sprint(&all, &["Refine", "Standup"]));
        assert_eq!(c.current, Some("Implement"));
        assert_eq!(c.passed, vec!["Refine", "Standup"]);
    }

    #[test]
    fn complete_sprint_has_no_current_step() {
        let all = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];
        let c = compute(&sprint(&all, &all));
        assert_eq!(c.current, None);
    }

    #[test]
    fn out_of_order_pass_is_detected() {
        let all = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];
        // CloseOut passed while Implement (earlier) is not.
        let c = compute(&sprint(&all, &["Refine", "Standup", "CloseOut"]));
        let d = out_of_order(&c);
        assert!(d.contains(&("CloseOut", "Implement")), "{d:?}");
        assert!(d.contains(&("CloseOut", "Review")), "{d:?}");
    }

    #[test]
    fn defined_needs_the_test_declaration_not_the_result() {
        // A file with only the RESULT line must not count the gate as defined.
        let only_result = "package X {\n    part fooReviewGateR1 : TestResult { :>> outcome = VerdictKind::pass; }\n}\n";
        assert!(!gate_defined(only_result, "Review"));
        assert!(gate_defined("verification fooReviewGate : Test { }", "Review"));
    }
}
