//! `keel attestation` — how much of this model's "pass" is a receipt, and how much is testimony.
//!
//! WHY A REPORT AND NOT A GATE (D0232). Three attempts were made to gate the WORDING of a verdict,
//! and all three failed calibration against the real corpus:
//!
//! | attempt | rule | false positives |
//! |---|---|---|
//! | 1 | a verdict word with no number in the field | 111 — mostly terse old backlog prose |
//! | 2 | an unqualified universal ("perfect", "cannot fail") | 35 — nearly all CRITIQUES using the phrase to criticise vacuity |
//! | 3 | "would have caught X" must cite something re-runnable | 2, then 1 after narrowing — all of them meta-discussion of the claim pattern itself |
//!
//! So language policing does not calibrate here, and a blocking guard at that precision teaches the
//! author to route around the write path — which is where every other check lives. D0214 also argues
//! the fix must be subtractive: this engine already detects more than it triages, and an unread
//! warning is worse than no warning.
//!
//! What IS decidable is the STRUCTURE of an attestation: its method, who judged it, and whether it
//! records what produced it. So the over-claim rate becomes a measured number instead of a policed
//! sentence — an indicator, per D0088: when a defensible threshold cannot be set, monitoring beats
//! gating, and promoting it later needs a justified boundary rather than a hunch.
//!
//! `evidence-cited` (guard 52) is the gate that DOES calibrate, because it checks structure: an
//! AI-judged `method=test` result must record what it ran. Human judgments are never in scope —
//! governance binds the AI, and a human's word IS the evidence.
use std::collections::BTreeMap;
use std::path::Path;

/// One row of the attestation census.
#[derive(Default)]
pub struct Census {
    /// Results whose Test declares `method=test` (an EXERCISED claim).
    pub exercised: usize,
    /// …of those, how many record what produced them.
    pub exercised_with_receipt: usize,
    /// Results whose Test declares an examining method (inspect/analyze/critique/confirmation).
    pub examined: usize,
    /// Verdicts recorded as `fail` — a population that never fails is not being tested.
    pub failed: usize,
    /// Total results counted.
    pub total: usize,
}

/// Count attestations by judge kind: `"human"`, `"ai"`, or `"unregistered"`.
#[must_use]
pub fn census(root: &Path) -> BTreeMap<String, Census> {
    let files = crate::collect_sysml(&root.join(".tracking"));
    // The Test declares the method; the result declares the judge.
    let mut method_of: BTreeMap<String, String> = BTreeMap::new();
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        for cap in text.split("verification ").skip(1) {
            let Some(name) = cap.split([' ', ':']).next() else { continue };
            if let Some(m) = cap.split(":>> method = VerificationMethod::").nth(1) {
                if let Some(kind) = m.split([';', ' ']).next() {
                    method_of.insert(name.to_string(), kind.to_string());
                }
            }
        }
    }
    let mut out: BTreeMap<String, Census> = BTreeMap::new();
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !line.contains(" : TestResult {") {
                continue;
            }
            let by = quoted(line, "judgedBy").unwrap_or_default();
            let kind = crate::actor::kind_of(root, &by).unwrap_or_else(|| "unregistered".to_string());
            let e = out.entry(kind).or_default();
            e.total += 1;
            if line.contains("VerdictKind::fail") {
                e.failed += 1;
            }
            let Some(part) = line.split(" : TestResult").next().and_then(|s| s.split("part ").nth(1)) else { continue };
            let base = part.trim().rsplit_once('R').map_or_else(|| part.trim(), |(b, _)| b);
            if method_of.get(base).map(String::as_str) == Some("test") {
                e.exercised += 1;
                let receipt = line.contains("// RAN:")
                    || i.checked_sub(1)
                        .and_then(|j| lines.get(j))
                        .is_some_and(|p| p.trim_start().starts_with("// RAN:"));
                if receipt {
                    e.exercised_with_receipt += 1;
                }
            } else {
                e.examined += 1;
            }
        }
    }
    out
}

fn quoted(line: &str, name: &str) -> Option<String> {
    let needle = format!(":>> {name} = \"");
    Some(line.split(&needle).nth(1)?.split('"').next()?.to_string())
}

/// Coverage claims that name a tracked item but cite nothing re-runnable — REPORTED, never gated,
/// because the gating attempt ran at 0% precision on live data (see the module note).
#[must_use]
pub fn uncited_coverage_claims(root: &Path) -> usize {
    let mut files = crate::collect_sysml(&root.join(".tracking"));
    files.extend(crate::collect_sysml(&root.join(".engine").join("decisions")));
    let mut n = 0usize;
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        for line in text.lines() {
            let lower = line.to_ascii_lowercase();
            for phrase in ["would have caught", "would have prevented", "would have stopped"] {
                if let Some(at) = lower.find(phrase) {
                    // Land on a char boundary: this prose is full of em dashes, and slicing through
                    // one panics. Fixed the same bug in the injection detector and repeated it here.
                    let mut end = lower.len().min(at + 400);
                    while end > at && !lower.is_char_boundary(end) {
                        end -= 1;
                    }
                    let w = &lower[at..end];
                    if !(w.contains(".rs") || w.contains(".py") || w.contains("tests/") || w.contains("`keel ") || w.contains("guard ")) {
                        n += 1;
                    }
                    break;
                }
            }
        }
    }
    n
}

/// `keel attestation [ROOT] [--json]`.
#[must_use]
pub fn cmd(args: &[String]) -> i32 {
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map_or_else(|| std::path::PathBuf::from("."), std::path::PathBuf::from);
    // issue281: refuse rather than answer over nothing. At a workspace root this printed a census of
    // ZERO attestations and exited 0 - the same false green the issue269 refusal closed for
    // `validate` alone.
    if let Err(code) = crate::workspace::require_project(&root, "keel attestation [ROOT] [--json]") {
        return code;
    }
    let c = census(&root);
    let claims = uncited_coverage_claims(&root);
    let pct = |a: usize, b: usize| (a * 100).checked_div(b).unwrap_or(0);

    if args.iter().any(|a| a == "--json") {
        let rows: Vec<String> = c
            .iter()
            .map(|(k, v)| {
                format!(
                    "{{\"judge\":\"{k}\",\"total\":{},\"exercised\":{},\"exercisedWithReceipt\":{},\"examined\":{},\"failed\":{},\"receiptPct\":{},\"failPct\":{}}}",
                    v.total, v.exercised, v.exercised_with_receipt, v.examined, v.failed,
                    pct(v.exercised_with_receipt, v.exercised), pct(v.failed, v.total)
                )
            })
            .collect();
        println!("{{\"byJudge\":[{}],\"uncitedCoverageClaims\":{claims}}}", rows.join(","));
        return 0;
    }

    println!("attestation census — is a `pass` a RECEIPT or a TESTIMONY?");
    println!();
    println!("  {:<14} {:>7} {:>10} {:>9} {:>9} {:>8}", "JUDGE", "results", "exercised", "w/receipt", "examined", "failed");
    for (k, v) in &c {
        println!(
            "  {:<14} {:>7} {:>10} {:>8}% {:>9} {:>7}%",
            k, v.total, v.exercised, pct(v.exercised_with_receipt, v.exercised), v.examined, pct(v.failed, v.total)
        );
    }
    println!();
    println!("  w/receipt is the honest number: an EXERCISED claim that records what produced it, so a");
    println!("  third party can re-derive the verdict instead of taking the judge's word (guard 52,");
    println!("  AI-judged results only - a human's word IS the evidence).");
    println!();
    println!("  failed% is the other one worth watching: a population that never fails is not being");
    println!("  tested, it is being recorded.");
    println!();
    println!("  uncited coverage claims ('would have caught X' naming nothing re-runnable): {claims}");
    println!("  REPORTED, not gated - three attempts to gate verdict WORDING ran at 111, 35 and 1");
    println!("  false positives, so language policing does not calibrate here (D0232).");
    0
}

#[cfg(test)]
mod tests {
    use super::census;

    #[test]
    fn the_census_separates_human_testimony_from_ai_claims() {
        // The whole point of the split: a human's `pass` needs no receipt, an AI's does. If the
        // census stopped distinguishing them, the number would blame the wrong party.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let c = census(&root);
        assert!(!c.is_empty(), "population must be non-empty or this passes vacuously");
        let total: usize = c.values().map(|v| v.total).sum();
        assert!(total > 1000, "expected this project's full result corpus, saw {total}");
        assert!(c.contains_key("human"), "human judgments must be counted separately: {:?}", c.keys().collect::<Vec<_>>());
        assert!(c.contains_key("ai"), "AI judgments must be counted separately: {:?}", c.keys().collect::<Vec<_>>());
    }
}
