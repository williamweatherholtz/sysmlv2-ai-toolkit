//! Known defects in the controls themselves (D0278).
//!
//! # Why a control defect is its own class
//!
//! An ordinary defect affects what it touched. A defect in a GUARD invalidates its own history:
//! every verdict it gave while broken is unverified, and nobody knows which. That blast radius is
//! retroactive and unbounded until somebody assesses it, which is the whole reason these are triaged
//! on the spot rather than at a retro.
//!
//! # What this module is for
//!
//! `keel guard` prints a registered defect BESIDE the verdict of the control it belongs to, so a
//! green from a control known to under-report arrives already qualified — at the moment someone is
//! reading it, without anyone having to remember. That is the hook the process rests on, and it is
//! the reason the registry is an authored contract rather than a note in a retro: three true control
//! findings on 2026-09-01 went into sprint prose and were unfindable, uncounted and unresolved.

use std::collections::BTreeMap;
use std::path::Path;

/// One registered defect, as authored in `.engine/contracts/control-defects.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Defect {
    /// The tracked Issue. Must exist and be open, or `control-defect-registry` fails.
    pub issue: String,
    /// `"over"` (flags what is fine) or `"under"` (passes what is broken — silent, always worse).
    pub direction: String,
    /// One line: what the wrong verdict IS, in the reader's terms.
    pub effect: String,
    /// What this control passed while broken, and whether any of it was re-checked. The step no
    /// ordinary defect has.
    pub invalidates: String,
}

impl Defect {
    /// The line printed beside the control's own verdict.
    #[must_use]
    pub fn note(&self, control: &str) -> String {
        let arrow = if self.direction == "under" {
            "UNDER-REPORTS — it PASSES things it should catch, so a green here is not evidence"
        } else {
            "OVER-REPORTS — it flags things that are fine, so a violation here may not be one"
        };
        format!("  DEFECT  `{control}` {arrow} ({}). {}", self.issue, self.effect)
    }
}

/// Every registered control defect, keyed by control name.
///
/// A missing or unreadable file means NO registered defects, which is the honest default: the
/// registry records what is known to be broken, and its absence is not a claim that nothing is.
#[must_use]
pub fn load(root: &Path) -> BTreeMap<String, Defect> {
    let path = root.join(".engine").join("contracts").join("control-defects.toml");
    let Ok(text) = std::fs::read_to_string(path) else { return BTreeMap::new() };
    parse(&text)
}

/// Pure parser, so the shape is testable without a tree.
#[must_use]
pub fn parse(text: &str) -> BTreeMap<String, Defect> {
    let mut out = BTreeMap::new();
    let mut section: Option<String> = None;
    let mut cur: BTreeMap<String, String> = BTreeMap::new();
    let flush = |out: &mut BTreeMap<String, Defect>, s: &Option<String>, c: &BTreeMap<String, String>| {
        let (Some(name), Some(issue), Some(direction)) = (s.as_ref(), c.get("issue"), c.get("direction")) else {
            return;
        };
        out.insert(
            name.clone(),
            Defect {
                issue: issue.clone(),
                direction: direction.clone(),
                effect: c.get("effect").cloned().unwrap_or_default(),
                invalidates: c.get("invalidates").cloned().unwrap_or_default(),
            },
        );
    };
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('#') || l.is_empty() {
            continue;
        }
        if let Some(name) = l.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            flush(&mut out, &section, &cur);
            section = Some(name.to_string());
            cur = BTreeMap::new();
        } else if let Some((k, v)) = l.split_once(" = ") {
            cur.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
        }
    }
    flush(&mut out, &section, &cur);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_registered_defect_parses_and_states_its_direction() {
        let d = parse(
            "# a comment\n[issues]\nissue = \"issue331\"\ndirection = \"under\"\n\
             effect = \"a mis-triage reads as triaged\"\ninvalidates = \"every prior PASS\"\n",
        );
        let e = d.get("issues").expect("the section parses");
        assert_eq!(e.issue, "issue331");
        assert_eq!(e.direction, "under");
        // The note must make the DIRECTION legible without the reader knowing the vocabulary: an
        // under-reporting control's green is the thing they must not trust.
        let note = e.note("issues");
        assert!(note.contains("UNDER-REPORTS"), "{note}");
        assert!(note.contains("a green here is not evidence"), "{note}");
        assert!(note.contains("issue331"), "the note names the tracked Issue: {note}");
    }

    #[test]
    fn an_over_reporting_defect_qualifies_the_violation_not_the_pass() {
        let d = parse("[ownership]\nissue = \"issue333\"\ndirection = \"over\"\neffect = \"attributes to the machine\"\n");
        let note = d.get("ownership").expect("parses").note("ownership");
        assert!(note.contains("OVER-REPORTS"), "{note}");
        assert!(
            note.contains("may not be one"),
            "an over-reporting control's VIOLATION is the doubtful part, not its pass: {note}"
        );
    }

    #[test]
    fn an_absent_registry_claims_nothing() {
        // The honest default. An empty registry says "nothing is KNOWN to be broken", never
        // "nothing is broken" — the same distinction `keel status` draws between OK and UNKNOWN.
        assert!(parse("").is_empty());
        assert!(load(Path::new("this-directory-does-not-exist")).is_empty());
    }
}
