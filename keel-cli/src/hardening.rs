//! `keel hardening` — the architectural-critique process's own questions, COMPUTED (issue171/D0169).
//!
//! WHY THIS MODULE EXISTS, and it is the least flattering reason in this codebase. D0046's critique
//! process asks a stable, recurring set of questions, and every pass re-derived them as throwaway python.
//! In the pass that produced this module, FOUR probes were wrong before they were right:
//!
//!   - a regex matching `: Process` also matched `: ProcessStep` — 131 processes reported, 24 exist;
//!   - help extraction reported `keel orient` and `keel assured` as nonexistent, minutes after both had
//!     been run in the same session;
//!   - a second attempt at the same question reported 0 of 72 subcommands documented;
//!   - a registry probe reported 0 registered skills against 35 real declarations, which would have
//!     become a phantom-drift finding of exactly the kind CR-10 already fixed once in 2026-06.
//!
//! Each wrong number was ONE EDIT from becoming a recorded finding, and a recorded finding directs the
//! next session. D0040 says a recurring task without a skill leaks process knowledge into conversation
//! history; this leaked worse than that — it leaked WRONG ANSWERS toward the model.
//!
//! Everything here reads SOURCE rather than running the binary. A view that shells out to itself to
//! report on itself cannot be trusted, and the help question in particular is about what the source
//! claims, not about what one invocation happened to print.

use crate::json::Json;
use std::path::Path;

/// The hardening lens set.
///
/// # Errors
/// Never returns `Err` today; the signature matches the other view functions so it can be served by
/// `keel serve`'s cache, which is typed over `Result`.
pub fn hardening(root: &Path) -> Result<String, crate::view::ViewError> {
    Ok(Json::Obj(vec![
        (
            "hardening".to_string(),
            Json::s(
                "the architectural-critique process's own questions, computed rather than re-probed \
                 (issue171). Every number here was once a hand-written regex that got it wrong.",
            ),
        ),
        ("helpCoverage".to_string(), help_coverage(root)),
        ("processEnforcement".to_string(), process_enforcement(root)),
        ("decisionFollowThrough".to_string(), decision_follow_through(root)),
    ])
    .dump())
}

fn pct(part: usize, whole: usize) -> i64 {
    if whole == 0 {
        return 100;
    }
    i64::try_from(part * 100 / whole).unwrap_or(0)
}

fn count(n: usize) -> Json {
    Json::Int(i64::try_from(n).unwrap_or(0))
}

// ── lens 1: does the CLI describe itself? ────────────────────────────────────────────────────────

/// Which top-level subcommands `keel --help` names, and which it does not (issue172).
///
/// The CLI is the authority and the automation substrate (D0093). A subcommand absent from help is
/// reachable only by reading `main.rs` or CLAUDE.md, which is not discoverability.
fn help_coverage(root: &Path) -> Json {
    let main = std::fs::read_to_string(root.join("keel-cli/src/main.rs")).unwrap_or_default();
    let dispatched = dispatch_arms(&main);
    let help = usage_text(&main);
    let (named, absent): (Vec<String>, Vec<String>) =
        dispatched.iter().cloned().partition(|c| help_names(&help, c));
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "Help is a hand-maintained string beside a hand-maintained match, so the two drift \
                 silently and only ever one way: a new command gets dispatched and never described.",
            ),
        ),
        ("dispatched".to_string(), count(dispatched.len())),
        ("namedInHelp".to_string(), count(named.len())),
        ("coveragePct".to_string(), Json::Int(pct(named.len(), dispatched.len()))),
        ("absentFromHelp".to_string(), Json::Arr(absent.into_iter().map(Json::s).collect())),
    ])
}

/// The top-level subcommand strings in `fn main`'s dispatch, including `|`-joined alternatives.
///
/// Scans only the `Some(...)` head of each arm, so a string appearing in an arm's BODY is never mistaken
/// for a subcommand name.
#[must_use]
pub fn dispatch_arms(main: &str) -> Vec<String> {
    let Some(i) = main.find("fn main() {") else { return Vec::new() };
    let mut out = std::collections::BTreeSet::new();
    let mut rest = &main[i..];
    while let Some(j) = rest.find("Some(") {
        rest = &rest[j + "Some(".len()..];
        let Some(arrow) = rest.find("=>") else { break };
        let head = &rest[..arrow];
        // A long head is not a subcommand arm — it is a match on something else entirely.
        if head.len() > 200 {
            continue;
        }
        for lit in string_literals(head) {
            if !lit.is_empty() && lit.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
                out.insert(lit);
            }
        }
    }
    out.into_iter().collect()
}

/// Every `"…"` literal in a fragment, without regex.
fn string_literals(frag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = frag;
    while let Some(a) = rest.find('"') {
        rest = &rest[a + 1..];
        let Some(b) = rest.find('"') else { break };
        out.push(rest[..b].to_string());
        rest = &rest[b + 1..];
    }
    out
}

/// The help text as the binary prints it: the `CATALOGUE` table `print_usage` iterates.
///
/// TWO WRONG VERSIONS OF THIS FUNCTION, both caught by the lens itself rather than by inspection. The
/// first looked for `fn usage(`, which does not exist - the function is `print_usage` - and so read an
/// EMPTY help text and reported 0 of 75 documented. The second read `print_usage`'s body, which was
/// right until the fix for issue172 moved the lines into a `const CATALOGUE`, at which point it reported
/// 1 of 75. A lens over source has to be re-aimed when the source moves; the value of it being a lens is
/// that a wrong aim shows up as an absurd number instead of a plausible one.
fn usage_text(main: &str) -> String {
    let Some(i) = main.find("const CATALOGUE:") else { return String::new() };
    let body = &main[i..];
    let end = body.find("
];").map_or(body.len(), |e| e + 3);
    body[..end].to_string()
}

/// Does the help text NAME this subcommand? WORD-BOUNDED, because a plain substring test lets
/// `check-engine` satisfy `check` and `activation` satisfy `act` — the exact bug that made the
/// hand-written version of this probe report 0 of 72 commands documented.
fn help_names(help: &str, cmd: &str) -> bool {
    let bytes = help.as_bytes();
    let mut from = 0;
    while let Some(rel) = help[from..].find(cmd) {
        let s = from + rel;
        let e = s + cmd.len();
        let before_ok = s.checked_sub(1).and_then(|i| bytes.get(i)).is_none_or(|b| !is_cmd_char(*b));
        let after_ok = bytes.get(e).is_none_or(|b| !is_cmd_char(*b));
        if before_ok && after_ok {
            return true;
        }
        from = s + 1;
    }
    false
}

const fn is_cmd_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

// ── lens 2: can a process be enforced at all? ────────────────────────────────────────────────────

/// Per top-level process: does its unit assert a guard, i.e. is its constraint machine-checkable?
///
/// AN INDICATOR, NEVER A GATE (invariant 7). Some constraints are genuinely judgments — a definition of
/// done is a judgment, a critique is a judgment — and no guard can be conjured for them. What this
/// reports is how much of the catalogue is enforceable, so the unenforceable part is VISIBLE rather than
/// assumed covered.
fn process_enforcement(root: &Path) -> Json {
    let act = crate::activation::Activation::load(root);
    let declared = enforcement_contract(root);
    let mut enforced: Vec<Json> = Vec::new();
    let mut unenforceable: Vec<Json> = Vec::new();
    let mut undeclared: Vec<Json> = Vec::new();
    for f in crate::collect_sysml(&root.join(".engine/processes")) {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        let unit = f.file_stem().unwrap_or_default().to_string_lossy().to_string();
        let n = act.unit(&unit).map_or(0, |u| u.guards.len());
        for name in top_level_processes(&text) {
            let entry = declared.get(&name);
            let mut row = vec![
                ("process".to_string(), Json::s(name.clone())),
                ("unit".to_string(), Json::s(unit.clone())),
                ("guards".to_string(), count(n)),
            ];
            if n > 0 {
                enforced.push(Json::Obj(row));
                continue;
            }
            match entry {
                // `checkable = false` - a judgment, and no guard can exist for it. Correct, permanent.
                Some((false, reason)) => {
                    row.push(("reason".to_string(), Json::s(reason.clone())));
                    unenforceable.push(Json::Obj(row));
                }
                // `checkable = true` with no guard - an ADMITTED gap. Worse than undeclared, because
                // someone looked at it and agreed a guard could exist.
                Some((true, reason)) => {
                    row.push(("admittedGap".to_string(), Json::Bool(true)));
                    row.push(("reason".to_string(), Json::s(reason.clone())));
                    undeclared.push(Json::Obj(row));
                }
                // No entry at all. Silence reads as a GAP, never as consent.
                None => {
                    row.push(("admittedGap".to_string(), Json::Bool(false)));
                    undeclared.push(Json::Obj(row));
                }
            }
        }
    }
    let total = enforced.len() + unenforceable.len() + undeclared.len();
    let accounted = enforced.len() + unenforceable.len();
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "Three buckets, because the two-bucket version made a judgment look identical to a                  missing guard (issue173). ENFORCED asserts a guard. UNENFORCEABLE is declared in                  `.engine/contracts/process-enforcement.toml` with a reason a reviewer can disagree                  with. UNDECLARED is the honest gap - and an `admittedGap` is one where the contract                  itself says a guard COULD exist. An INDICATOR, never a gate: gating this ratio would                  make the cheapest fix a guard that checks nothing.",
            ),
        ),
        ("processes".to_string(), count(total)),
        ("enforced".to_string(), count(enforced.len())),
        ("declaredUnenforceable".to_string(), count(unenforceable.len())),
        ("accountedPct".to_string(), Json::Int(pct(accounted, total))),
        ("undeclared".to_string(), Json::Arr(undeclared)),
        ("unenforceable".to_string(), Json::Arr(unenforceable)),
    ])
}

/// `process -> (checkable, reason)` from `.engine/contracts/process-enforcement.toml`.
///
/// An absent file yields an empty map, so every unguarded process reports as undeclared - the
/// pre-issue173 behaviour, and strictly less informative rather than wrong.
fn enforcement_contract(root: &Path) -> std::collections::BTreeMap<String, (bool, String)> {
    let mut out = std::collections::BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(root.join(".engine/contracts/process-enforcement.toml"))
    else {
        return out;
    };
    let mut current = String::new();
    let mut checkable = false;
    let mut reason = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            if !current.is_empty() {
                out.insert(current.clone(), (checkable, reason.clone()));
            }
            current = name.to_string();
            checkable = false;
            reason = String::new();
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        match k.trim() {
            "checkable" => checkable = v.trim() == "true",
            "reason" => reason = v.trim().trim_matches('"').to_string(),
            _ => {}
        }
    }
    if !current.is_empty() {
        out.insert(current, (checkable, reason));
    }
    out
}

/// `action NAME : Process {` — and NOT `: ProcessStep`, which is the entire point of this function.
#[must_use]
pub fn top_level_processes(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|raw| {
            let line = raw.trim_start();
            if line.starts_with("//") {
                return None;
            }
            let rest = line.strip_prefix("action ")?;
            let (name, after) = rest.split_once(':')?;
            let tail = after.trim_start().strip_prefix("Process")?;
            tail.trim_start().starts_with('{').then(|| name.trim().to_string())
        })
        .collect()
}

// ── lens 3: was an accepted Decision actually carried out? ───────────────────────────────────────

/// Accepted Decisions that PROMISE a named artifact, and whether that artifact exists (issue174).
///
/// Promises are declared in `.engine/contracts/decision-artifacts.toml` rather than in a Decision field,
/// deliberately: `schema/core` is frozen (invariant 5), and this question does not need a schema change
/// to answer. The contract is authored data, not a view.
///
/// NOT A GATE. A Decision may legitimately be accepted long before it is built — D0161 was accepted the
/// day it was written — and blocking on an unbuilt promise would push the promise out of the record
/// rather than into it. What was missing is that nothing DISTINGUISHED `accepted and needs nothing` from
/// `accepted and abandoned`; 64 of 161 accepted Decisions have no chartered work and most of those are
/// correct, which is why the raw count was never the finding.
fn decision_follow_through(root: &Path) -> Json {
    let path = root.join(".engine/contracts/decision-artifacts.toml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Json::Obj(vec![
            ("declared".to_string(), Json::Int(0)),
            (
                "note".to_string(),
                Json::s(
                    "No `.engine/contracts/decision-artifacts.toml`. A project that never declared a \
                     promise has not broken one, so this reports nothing rather than guessing (D0138).",
                ),
            ),
        ]);
    };
    let mut kept: Vec<Json> = Vec::new();
    let mut broken: Vec<Json> = Vec::new();
    let mut decision = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            decision = name.to_string();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else { continue };
        if key.trim() != "artifact" {
            continue;
        }
        let rel = value.trim().trim_matches('"');
        let row = Json::Obj(vec![
            ("decision".to_string(), Json::s(decision.clone())),
            ("artifact".to_string(), Json::s(rel)),
        ]);
        if root.join(rel).exists() { kept.push(row) } else { broken.push(row) }
    }
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "An accepted Decision whose promised artifact is ABSENT is the mechanism behind \
                 half-baked implementations: acceptance is recorded, the promise is not, and the model \
                 reads as complete. Reported, never gated.",
            ),
        ),
        ("declared".to_string(), count(kept.len() + broken.len())),
        ("kept".to_string(), count(kept.len())),
        ("unbuilt".to_string(), Json::Arr(broken)),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `: ProcessStep` must NOT be counted as a process. This is the single worst number the
    /// hand-written audit produced — 131 where there are 24 — and it is one character of regex.
    #[test]
    fn a_process_step_is_not_a_process() {
        let text = "    action intake : Process {\n    action inRecord : ProcessStep {\n\
                    action other : Process{\n    // action commented : Process {\n";
        assert_eq!(top_level_processes(text), vec!["intake", "other"]);
    }

    /// Word boundaries: `check-engine` in the help must not satisfy `check`, and `activation` must not
    /// satisfy `act`. A plain `contains` here reported 0 of 72 commands undocumented.
    #[test]
    fn help_names_is_word_bounded() {
        let help = "  check-engine ROOT   do a thing\n  activation [ROOT]  another\n  ls  list\n";
        assert!(help_names(help, "check-engine"));
        assert!(help_names(help, "activation"));
        assert!(help_names(help, "ls"));
        assert!(!help_names(help, "check"), "check-engine must not satisfy check");
        assert!(!help_names(help, "act"), "activation must not satisfy act");
        assert!(!help_names(help, "orient"));
    }

    /// THE CONTROL for issue172: every dispatched subcommand must be named in the catalogue.
    ///
    /// Expressed through the lens rather than as a separate probe, so the test and the reported number
    /// can never disagree - if the lens is re-aimed wrongly this test fails too, which is what happened
    /// twice while building it and is exactly the behaviour wanted. A new `Some("x") =>` arm with no
    /// catalogue line now fails the build instead of quietly shipping an undiscoverable command.
    #[test]
    fn every_dispatched_subcommand_is_documented() {
        let main = std::fs::read_to_string("src/main.rs").expect("main.rs is readable");
        let dispatched = dispatch_arms(&main);
        assert!(dispatched.len() > 50, "the dispatch scan found {} arms - the lens is mis-aimed", dispatched.len());
        let help = usage_text(&main);
        assert!(!help.is_empty(), "the CATALOGUE could not be located - the lens is mis-aimed");
        let absent: Vec<&String> = dispatched.iter().filter(|c| !help_names(&help, c)).collect();
        assert!(
            absent.is_empty(),
            "{} subcommand(s) dispatched but absent from the help CATALOGUE: {absent:?}",
            absent.len()
        );
    }

    /// A string inside an arm's BODY is not a subcommand. Without the head-only scan, every literal in
    /// `main.rs` became a phantom command.
    #[test]
    fn dispatch_arms_reads_only_the_match_head() {
        let src = "fn main() {\n    match x {\n        Some(\"orient\") => cmd_orient(rest),\n\
                   Some(v @ (\"activate\" | \"deactivate\")) => go(v),\n\
                   Some(\"land\") => { eprintln!(\"not-a-command\"); }\n    }\n}";
        let arms = dispatch_arms(src);
        assert!(arms.contains(&"orient".to_string()));
        assert!(arms.contains(&"activate".to_string()));
        assert!(arms.contains(&"deactivate".to_string()));
        assert!(arms.contains(&"land".to_string()));
        assert!(!arms.contains(&"not-a-command".to_string()), "an arm BODY literal is not a command");
    }
}
