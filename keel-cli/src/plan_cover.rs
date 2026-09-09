//! A signed plan covers the steps it names (D0375 option C, the human's word 2026-09-08; D0396).
//!
//! A process-change or safety-change Decision is outside standing consent (D0337) and normally waits
//! for the human's own signature. But when its item is one of the steps ENUMERATED in a Decision the
//! human accepted THEMSELVES - the plan - the human already signed for it: once, on the plan. Such a
//! Decision, recorded with `plan: dNNNN` and `step: <name>`, is accepted at record time under the
//! plan's authority, and its acceptance note carries a PLAN-COVERED token naming the plan and the step
//! so a guard can re-check the cover never outlives the plan it cites.
//!
//! Three clauses, all required; any one failing leaves the Decision proposed and names which:
//!   (a) the plan Decision exists;
//!   (b) it was accepted by a HUMAN'S OWN word - its acceptance note carries no AUTO-ACCEPTED token and
//!       no PLAN-COVERED token (one level: a plan cannot itself be plan-covered), and its judge is a
//!       Person named in github-actors.toml as a decider;
//!   (c) it NAMES the step in its own decision or consequences text, matched on the step name exactly.
//! A plan cannot cover itself.

use std::path::{Path, PathBuf};

/// The literal a plan-covered acceptance note opens with. The guard reads the plan id and step from it.
pub const TOKEN: &str = "PLAN-COVERED";
/// The token a standing-consent acceptance carries; a plan carrying it was not signed by a human's own word.
pub const AUTO_TOKEN: &str = "AUTO-ACCEPTED";

/// The outcome of assessing whether a plan covers a step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cover {
    /// All three clauses hold; the acceptance is recorded under the plan's judge.
    Covered { judge: String },
    /// A clause failed; `clause` is "a" | "b" | "c" | "self", `detail` says what was searched.
    Refused { clause: &'static str, detail: String },
}

/// The plan's file under `.engine/decisions/`, if any Decision by that name is declared.
fn decision_file(root: &Path, dname: &str) -> Option<PathBuf> {
    crate::collect_sysml(&root.join(".engine").join("decisions"))
        .into_iter()
        .find(|p| std::fs::read_to_string(p).is_ok_and(|t| t.contains(&format!("part {dname} : Decision"))))
}

/// The value of one `:>> key = "..."` attribute in a block of text (first occurrence).
fn attr<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!(":>> {key} = \"");
    let i = text.find(&needle)? + needle.len();
    text.get(i..).and_then(|r| r.split('"').next())
}

/// The judge (`judgedBy`) of a Decision's acceptance result, and its acceptance note - read from the
/// `{dname}AcceptR` result and the `{dname}Accept` Test. `None` when the Decision is not accepted.
fn acceptance(text: &str, dname: &str) -> Option<(String, String)> {
    // the note lives on the Accept Test's procedureText; the judge on the AcceptR result's judgedBy.
    let accept_test = text.split(&format!("verification {dname}Accept ")).nth(1)?;
    let note = attr(accept_test, "procedureText")?.to_string();
    let result = text.split(&format!("part {dname}AcceptR")).nth(1)?;
    let judge = attr(result, "judgedBy")?.to_string();
    Some((judge, note))
}

/// Assess whether `plan_id` covers `step` for a new Decision named `covered` (which cannot be its own
/// plan). Pure over the tree: reads only the plan's file and the deciders table.
#[must_use]
pub fn assess(root: &Path, covered: &str, plan_id: &str, step: &str) -> Cover {
    if plan_id == covered {
        return Cover::Refused { clause: "self", detail: "a Decision cannot be its own plan".to_string() };
    }
    // (a) the plan exists
    let Some(path) = decision_file(root, plan_id) else {
        return Cover::Refused { clause: "a", detail: format!("no Decision '{plan_id}' under .engine/decisions/") };
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Cover::Refused { clause: "a", detail: format!("{plan_id}'s file could not be read") };
    };
    // (b) the plan was accepted by a human's own word
    let Some((judge, note)) = acceptance(&text, plan_id) else {
        return Cover::Refused { clause: "b", detail: format!("{plan_id} is not accepted - a plan must carry a human's own acceptance before it can cover a step") };
    };
    if note.contains(AUTO_TOKEN) {
        return Cover::Refused { clause: "b", detail: format!("{plan_id} was AUTO-ACCEPTED under standing consent, not signed by a human's own word - standing consent does not reach the enforcement surface (D0337)") };
    }
    if note.contains(TOKEN) {
        return Cover::Refused { clause: "b", detail: format!("{plan_id} is itself PLAN-COVERED - a plan-covered Decision cannot be a plan (one level: the human's word, then the steps it names, never a chain)") };
    }
    let deciders: std::collections::BTreeSet<String> = crate::github::deciders(root).into_values().collect();
    if !deciders.contains(&judge) {
        return Cover::Refused { clause: "b", detail: format!("{plan_id}'s judge '{judge}' is not a decider named in github-actors.toml - the plan's signer must be a declared human decider") };
    }
    // (c) the plan names the step in its own decision or consequences text
    let decision_text = attr(&text, "decision").unwrap_or("");
    let consequences_text = attr(&text, "consequences").unwrap_or("");
    if !decision_text.contains(step) && !consequences_text.contains(step) {
        return Cover::Refused {
            clause: "c",
            detail: format!(
                "{plan_id} does not name the step '{step}' in its decision ({} chars) or consequences ({} chars) text - the plan must enumerate the step it covers, matched exactly",
                decision_text.len(),
                consequences_text.len()
            ),
        };
    }
    Cover::Covered { judge }
}

/// The acceptance note a covered Decision carries. The guard parses `plan_id` and `step` back out of it.
#[must_use]
pub fn note(plan_id: &str, step: &str, judge: &str) -> String {
    format!(
        "{TOKEN} under {plan_id} step '{step}': the human signed the plan itself and this enumerated step does not re-ask (D0375 option C / D0396). The plan's judge: {judge}."
    )
}

/// Parse `(plan_id, step)` back out of a `PLAN-COVERED` note, for the guard.
#[must_use]
pub fn parse_note(note: &str) -> Option<(String, String)> {
    let rest = note.strip_prefix(&format!("{TOKEN} under "))?;
    let (plan_id, rest) = rest.split_once(" step '")?;
    let step = rest.split('\'').next()?;
    Some((plan_id.trim().to_string(), step.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{note, parse_note};

    #[test]
    fn a_note_round_trips_its_plan_and_step() {
        let n = note("d0375", "plan-covers-step", "wweatherholtz");
        assert_eq!(parse_note(&n), Some(("d0375".to_string(), "plan-covers-step".to_string())));
        assert!(n.contains("PLAN-COVERED"));
        assert_eq!(parse_note("some other note"), None);
    }
}
