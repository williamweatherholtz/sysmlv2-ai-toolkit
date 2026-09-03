//! `keel verification` — what has been EXAMINED, what has been EXERCISED, and what is pending.
//!
//! # Why this is two numbers and not one
//!
//! `sr_verified_pct` counts a `SystemRequirement` as verified when ANY Test `#Verify`-links it, and
//! that single number conflates two unrelated claims:
//!
//! - **EXAMINED** — someone formed a judgment ABOUT THE REQUIREMENT: an adversarial `critique`, an
//!   `inspect`, an `analyze`. This needs no implementation and says nothing about whether the system
//!   does the thing.
//! - **EXERCISED** — the SYSTEM WAS RUN against it: a `test` or a `demo`. This says nothing about
//!   whether the requirement was ever a good requirement.
//!
//! A requirement can be thoroughly critiqued and never executed, or executed for years and never
//! adversarially read. Both are real gaps and they need opposite work, so reporting their UNION as
//! one percentage tells a reader neither. Measured here when the split was first built: 84% verified
//! decomposed into 32% examined and 75% exercised, with 41 requirements exercised but never examined
//! and 7 examined but never exercised.
//!
//! # Superseded requirements are out of scope
//!
//! A retired requirement is not pending anything. Same rule as `coverage`, `tier-satisfaction` and
//! (since issue127) `critique-coverage` — one notion of scope across every view.

use crate::view::ViewError;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// Methods that examine the REQUIREMENT. `confirmation` sits here because a human attestation is a
/// judgment formed about the claim, not a run of the system.
const EXAMINED: [&str; 4] = ["critique", "inspect", "analyze", "confirmation"];
/// Methods that exercise the SYSTEM.
const EXERCISED: [&str; 2] = ["test", "demo"];

/// One requirement's two-dimensional verification state.
pub struct Row {
    pub name: String,
    pub methods: BTreeSet<String>,
}

impl Row {
    fn examined(&self) -> bool {
        self.methods.iter().any(|m| EXAMINED.contains(&m.as_str()))
    }
    fn exercised(&self) -> bool {
        self.methods.iter().any(|m| EXERCISED.contains(&m.as_str()))
    }
}

/// Live `SystemRequirement`s with the verification methods that reach each.
///
/// # Errors
/// Returns [`ViewError`] if a tracking file fails to parse.
pub fn rows(root: &Path) -> Result<Vec<Row>, ViewError> {
    let (live, by_sr) = crate::view::sr_verification_methods(root)?;
    let mut out: Vec<Row> = live
        .into_iter()
        .map(|name| {
            let methods = by_sr.get(&name).cloned().unwrap_or_default();
            Row { name, methods }
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

const fn pct(n: usize, total: usize) -> usize {
    // No requirements in scope: nothing is DONE, so 0% - never a vacuous 100 (D0286).
    match n.saturating_mul(100).checked_div(total) {
        Some(p) => p,
        None => 0,
    }
}

/// What verification a requirement already has, for the pending listings.
fn has_of(r: &Row) -> String {
    if r.methods.is_empty() {
        "nothing".to_string()
    } else {
        r.methods.iter().map(String::as_str).collect::<Vec<_>>().join(", ")
    }
}

/// `keel verification [ROOT] [--pending]`.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    let Ok(rows) = rows(root) else {
        eprintln!("error: cannot compute verification state");
        return 1;
    };
    if rows.is_empty() {
        println!("no live SystemRequirements.");
        return 0;
    }
    let total = rows.len();
    let examined: Vec<&Row> = rows.iter().filter(|r| r.examined()).collect();
    let exercised: Vec<&Row> = rows.iter().filter(|r| r.exercised()).collect();
    let neither = rows.iter().filter(|r| r.methods.is_empty()).count();
    let pending_exam: Vec<&Row> = rows.iter().filter(|r| !r.examined()).collect();
    let pending_exer: Vec<&Row> = rows.iter().filter(|r| !r.exercised()).collect();

    println!("verification of {total} live SystemRequirement(s) — TWO dimensions, never one number:");
    println!();
    println!("  EXAMINED   {:>3}/{total}  {:>3}%   a judgment was formed ABOUT THE REQUIREMENT", examined.len(), pct(examined.len(), total));
    println!("             (critique | inspect | analyze | confirmation — needs no implementation)");
    println!("  EXERCISED  {:>3}/{total}  {:>3}%   THE SYSTEM WAS RUN against it", exercised.len(), pct(exercised.len(), total));
    println!("             (test | demo — says nothing about whether the requirement is any good)");
    println!();
    println!("  exercised but NEVER examined : {:>3}   nobody has adversarially read these", exercised.iter().filter(|r| !r.examined()).count());
    println!("  examined but NEVER exercised : {:>3}   nothing has ever run against these", examined.iter().filter(|r| !r.exercised()).count());
    println!("  neither                      : {neither:>3}");
    println!();
    println!("  A single 'verified' percentage is the UNION of these and answers neither question.");

    let mut by_method: BTreeMap<&str, usize> = BTreeMap::new();
    for r in &rows {
        for m in &r.methods {
            *by_method.entry(m.as_str()).or_default() += 1;
        }
    }
    println!();
    println!("  REQUIREMENTS reached, by method — distinct requirements, NOT edges:");
    for (m, n) in &by_method {
        let class = if EXAMINED.contains(m) { "examines" } else if EXERCISED.contains(m) { "exercises" } else { "?" };
        println!("    {m:<14} {n:>3}   ({class})");
    }
    println!("    `keel tier-satisfaction`'s verifiedByMethod counts EDGES and will be larger — a");
    println!("    Core-3 critique is three edges onto one requirement, so 72 edges there is 19 here.");

    println!();
    if args.iter().any(|a| a == "--pending") {
        println!("PENDING EXAMINATION — no critique/inspect/analyze reaches these ({}):", pending_exam.len());
        for r in &pending_exam {
            println!("  {:<36} has: {}", r.name, has_of(r));
        }
        println!();
        println!("PENDING EXERCISE — no test/demo reaches these ({}):", pending_exer.len());
        for r in &pending_exer {
            println!("  {:<36} has: {}", r.name, has_of(r));
        }
    } else {
        println!("  `keel verification --pending` lists exactly which requirements are pending in each dimension.");
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(methods: &[&str]) -> Row {
        Row { name: "sr".into(), methods: methods.iter().map(|s| (*s).to_string()).collect() }
    }

    #[test]
    fn the_two_dimensions_are_independent() {
        assert!(row(&["critique"]).examined() && !row(&["critique"]).exercised());
        assert!(row(&["test"]).exercised() && !row(&["test"]).examined());
        assert!(row(&["demo", "inspect"]).examined() && row(&["demo", "inspect"]).exercised());
        assert!(!row(&[]).examined() && !row(&[]).exercised());
    }

    #[test]
    fn confirmation_examines_rather_than_exercises() {
        // A human attestation is a judgment formed about the claim, not a run of the system —
        // counting it as test coverage is exactly the conflation this command exists to undo.
        let r = row(&["confirmation"]);
        assert!(r.examined(), "a confirmation is a judgment about the requirement");
        assert!(!r.exercised(), "a confirmation never ran anything");
    }

    #[test]
    fn a_requirement_with_no_verification_is_pending_in_both() {
        let r = row(&[]);
        assert!(!r.examined() && !r.exercised());
    }
}
