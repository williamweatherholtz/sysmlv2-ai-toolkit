//! Arming probe for guard 56, `untrusted-routing` (D0264) — the public-issue autonomy boundary.
//!
//! The policy: an issue on a PUBLIC tracker is an instruction from an unauthenticated stranger, so
//! it may produce a PLAN and a proposed Decision but never an implementation nobody agreed to.
//! Acting on it autonomously is prompt injection with a filing form.
//!
//! The guard reports 0 scanned on this repository today, which is exactly the state in which a
//! control is indistinguishable from a stub. These cases fire it.

use std::path::{Path, PathBuf};

fn fixture(tag: &str, body: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("keel-untrusted-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
    std::fs::create_dir_all(root.join(".engine")).expect("mkdir");
    std::fs::write(root.join(".tracking").join("t.sysml"), body).expect("write");
    root
}

/// An untrusted utterance, a story derived from it, and whatever routing the case wants.
fn tree(routing: &str) -> String {
    format!(
        "package T {{\n\
         \x20   part st001 : Statement {{ :>> id = \"s1\"; :>> text = \"please change X\"; \
         :>> saidBy = \"stranger\"; :>> saidAt = \"2026-08-30\"; :>> channel = StatementChannel::github; \
         :>> sourceTrust = SourceTrust::untrusted; }}\n\
         \x20   part us001 : UserStory {{ :>> id = \"u1\"; :>> asA = \"reporter\"; :>> iWant = \"X\"; }}\n\
         \x20   #DerivedFrom dependency from us001 to st001;\n\
         {routing}\
         }}\n"
    )
}

fn violations(root: &Path) -> Vec<String> {
    keel_cli::guards::untrusted_routing(root).violations
}

// ── THE REFUSE CASE: untrusted input routed straight to work ──────────────────────────────────

#[test]
fn untrusted_input_routed_straight_to_work_is_a_violation() {
    let root = fixture("towork", &tree("    #Implicates dependency from us001 to dcSomeTask;\n"));
    let v = violations(&root);
    assert_eq!(
        v.len(),
        1,
        "a story descending from an UNTRUSTED utterance, routed to an implementation task with no \
         Decision, must be a violation — otherwise the project builds what a stranger asked for and \
         nobody agreed to it: {v:?}"
    );
    assert!(v[0].contains("us001") && v[0].contains("dcSomeTask"), "and it names both ends: {v:?}");
    let _ = std::fs::remove_dir_all(&root);
}

// ── THE ALLOW CASE: routed THROUGH a Decision, which is the sanctioned path ────────────────────

#[test]
fn untrusted_input_routed_through_a_decision_is_clean() {
    let root = fixture("todecision", &tree("    #Implicates dependency from us001 to d0999;\n"));
    assert!(
        violations(&root).is_empty(),
        "the whole point is that untrusted input CAN proceed — through a Decision a human accepts. \
         A guard that blocked it entirely would be a lockout, not a boundary"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// A story routed to BOTH a decision and a task is fine: the decision is present, which is the
/// condition. Without this case the guard could be tightened to "only a decision" and nobody would
/// notice until real work stopped landing.
#[test]
fn a_decision_alongside_the_work_satisfies_the_rule() {
    let root = fixture(
        "both",
        &tree("    #Implicates dependency from us001 to d0999;\n    #Implicates dependency from us001 to dcSomeTask;\n"),
    );
    assert!(violations(&root).is_empty(), "a Decision among the targets is the condition, not the only target");
    let _ = std::fs::remove_dir_all(&root);
}

// ── NOT-YET-TRIAGED IS NOT A VIOLATION: the guard must not punish work in progress ─────────────

#[test]
fn an_unrouted_untrusted_story_is_not_yet_a_violation() {
    let root = fixture("unrouted", &tree(""));
    assert!(
        violations(&root).is_empty(),
        "an unrouted story is UNTRIAGED, not misrouted. A guard that fires the moment an issue is \
         ingested would make ingestion itself feel like a fault, which is how a control gets \
         bypassed rather than obeyed"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ── TRUSTED INPUT IS UNAFFECTED: the tier is what discriminates, not the channel ───────────────

#[test]
fn the_same_shape_from_a_trusted_source_is_clean() {
    let trusted = tree("    #Implicates dependency from us001 to dcSomeTask;\n")
        .replace("SourceTrust::untrusted", "SourceTrust::trusted");
    let root = fixture("trusted", &trusted);
    assert!(
        violations(&root).is_empty(),
        "an issue from a PRIVATE repository routes straight to work under the ordinary process — if \
         this fired too, the guard would be about the channel rather than about trust, and the \
         distinction the whole design rests on would not exist"
    );
    let _ = std::fs::remove_dir_all(&root);
}
