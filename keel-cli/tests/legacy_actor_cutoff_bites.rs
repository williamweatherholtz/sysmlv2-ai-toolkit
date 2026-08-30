//! Arming probe for the date-bounded legacy-actor rule (D0261).
//!
//! Why the rule changed: 52 of the warning channel's 96 lines were `legacy actor "user"
//! (pre-convention)` — undischargeable by construction, because rewriting a `judgedBy` on a June
//! record would FALSIFY provenance. A warning nobody can act on trains blindness to the ones they
//! can, and it did: eight real `verification-trace` findings against my own four-day-old work sat
//! unread behind them.
//!
//! Collapsing them to a count is only safe if a NEW occurrence still bites. The verdict is derived
//! from the RECORD'S OWN DATE, so there is no second baseline list to drift: tolerated before the
//! cutoff, violation on or after it.

use std::path::{Path, PathBuf};

fn fixture(tag: &str, date: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-legacy-actor-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine")).expect("mkdir");
    std::fs::write(
        root.join(".tracking").join("probe.sysml"),
        format!(
            "package Probe {{\n    part r1 : TestResult {{ :>> id = \"p1\"; :>> outcome = VerdictKind::pass; \
             :>> judgedAt = \"{date}\"; :>> judgedBy = \"user\"; }}\n}}\n"
        ),
    )
    .expect("write");
    root
}

fn report(root: &Path) -> keel_cli::guards::GuardReport {
    keel_cli::guards::actors(root)
}

#[test]
fn a_legacy_actor_on_a_record_dated_after_the_cutoff_is_a_violation() {
    let root = fixture("after", "2026-08-30");
    let r = report(&root);
    assert_eq!(
        r.violations.len(),
        1,
        "a legacy actor name on a record dated AFTER the convention must be a violation — without \
         this, collapsing the historic ones to a count would hide new occurrences: {:?}",
        r.violations
    );
    assert!(r.violations[0].contains("user"), "the violation names the actor: {:?}", r.violations);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_legacy_actor_on_a_pre_convention_record_is_counted_not_enumerated() {
    let root = fixture("before", "2026-06-10");
    let r = report(&root);
    assert!(r.violations.is_empty(), "history must not become a violation retroactively: {:?}", r.violations);
    assert_eq!(
        r.warnings.len(),
        1,
        "the historic ones collapse to ONE counted warning, not one line each — the whole point: {:?}",
        r.warnings
    );
    assert!(
        r.warnings[0].contains('1') && r.warnings[0].contains("legacy actor"),
        "and the count is stated, so nothing is hidden: {:?}",
        r.warnings
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_cutoff_boundary_is_inclusive_on_the_violation_side() {
    // The cutoff day itself is post-convention: a record dated ON it must fail. An off-by-one here
    // would leave exactly one day of silent tolerance, which is how a ratchet loosens unnoticed.
    let root = fixture("boundary", "2026-06-12");
    assert_eq!(
        report(&root).violations.len(),
        1,
        "a record dated ON the cutoff is on/after it and must violate"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn this_repository_has_no_post_cutoff_legacy_actor() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root");
    let r = report(root);
    assert!(
        r.violations.is_empty(),
        "the ratchet starts clean — every legacy reference in this corpus predates the convention: {:?}",
        r.violations
    );
    assert!(r.scanned > 400, "and it actually scanned the corpus rather than seeing nothing: {}", r.scanned);
}
