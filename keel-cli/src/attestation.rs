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
    /// Verdicts recorded as `proposed` (D0312 B): an AI-examined claim no human has judged yet -
    /// done for nothing, never folded into pass or fail.
    pub proposed: usize,
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
            if line.contains("VerdictKind::proposed") {
                e.proposed += 1;
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

/// Every `TestResult` under `.tracking` recorded `proposed` (D0312 B) that NO human has yet judged.
///
/// The burndown's count of what awaits a human's judgment (D0443: a proposal a later human pass or
/// fail answers is judged, not awaiting). Textual, like the census: a proposal is a line, not a
/// computed state.
#[must_use]
pub fn proposed_count(root: &Path) -> usize {
    proposal_counts(root).awaiting()
}

// ── D0443: the proposals, the sample and the human's judgment of it ─────────────────────────────

/// One `proposed` result awaiting a human's judgment (D0312 B), as one file holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    /// The Test whose result is proposed (`sRefineGate`).
    pub test: String,
    /// The result part (`sRefineGateR1`).
    pub result: String,
    /// The result's `id`. A v4 uuid is a uniform random order nothing authored, so ordering the
    /// proposals by it makes the sample deterministic for the reader and unchoosable by the writer.
    pub uuid: String,
    /// A later `<test>R<m>` result judged pass or fail by a registered human exists.
    pub judged: bool,
}

/// How many of a file's proposals a human is asked to judge: attestation-policy.toml
/// `[proposedJudgment] sampling`, a share (`"50%"`) or a count (`3`). No rule samples everything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingRule {
    /// `"50%"`: ceil(share x n).
    Share(usize),
    /// `3`: min(count, n).
    Count(usize),
}

impl SamplingRule {
    /// `sampling = "50%"` or `sampling = 3`; anything else is no rule.
    #[must_use]
    pub fn parse(v: &toml::Value) -> Option<Self> {
        match v {
            toml::Value::Integer(n) => usize::try_from(*n).ok().map(Self::Count),
            toml::Value::String(s) => s.trim().strip_suffix('%').and_then(|p| p.trim().parse::<usize>().ok()).filter(|p| *p <= 100).map(Self::Share),
            _ => None,
        }
    }

    /// The quota over `n` proposals.
    #[must_use]
    pub fn quota(self, n: usize) -> usize {
        match self {
            Self::Share(pct) => (n * pct).div_ceil(100),
            Self::Count(c) => c.min(n),
        }
    }
}

/// The project's declared sampling rule, or `None` (sample everything).
#[must_use]
pub fn sampling_rule(root: &Path) -> Option<SamplingRule> {
    let text = std::fs::read_to_string(root.join(".engine").join("contracts").join("attestation-policy.toml")).ok()?;
    let v = text.parse::<toml::Value>().ok()?;
    SamplingRule::parse(v.get("proposedJudgment")?.get("sampling")?)
}

/// Every proposal in one file's text, ordered by result uuid. `human` says whether a judge's name is a
/// registered human - the caller binds it to the registry; a unit test passes its own.
#[must_use]
pub fn proposals_in_text(text: &str, human: &dyn Fn(&str) -> bool) -> Vec<Proposal> {
    // every result line: (test, n, outcome, judgedBy, id)
    let mut results: Vec<(String, u32, String, String, String)> = Vec::new();
    for line in text.lines() {
        if !line.contains(" : TestResult {") {
            continue;
        }
        let Some(part) = line.split(" : TestResult").next().and_then(|s| s.split("part ").nth(1)) else { continue };
        let Some((base, n)) = part.trim().rsplit_once('R') else { continue };
        let Ok(n) = n.parse::<u32>() else { continue };
        let outcome = line.split("VerdictKind::").nth(1).and_then(|s| s.split([';', ' ', '}']).next()).unwrap_or("").to_string();
        results.push((base.to_string(), n, outcome, quoted(line, "judgedBy").unwrap_or_default(), quoted(line, "id").unwrap_or_default()));
    }
    let mut out: Vec<Proposal> = results
        .iter()
        .filter(|r| r.2 == "proposed")
        .map(|(base, n, _, _, uuid)| {
            let judged = results.iter().any(|(b, m, o, by, _)| b == base && m > n && (o == "pass" || o == "fail") && human(by));
            Proposal { test: base.clone(), result: format!("{base}R{n}"), uuid: uuid.clone(), judged }
        })
        .collect();
    out.sort_by(|a, b| a.uuid.cmp(&b.uuid).then_with(|| a.result.cmp(&b.result)));
    out
}

/// The proposals of one file under the registry's notion of a human.
#[must_use]
pub fn proposals_in(root: &Path, file: &Path) -> Vec<Proposal> {
    let Ok(text) = std::fs::read_to_string(file) else { return Vec::new() };
    proposals_in_text(&text, &|by| crate::actor::kind_of(root, by).as_deref() == Some("human"))
}

/// The SAMPLE of one file's proposals: the ones a human is asked to judge. Every proposal already
/// judged is in it first (sampled is never below judged), then the uuid order up to the quota.
#[must_use]
pub fn sample(proposals: &[Proposal], rule: Option<SamplingRule>) -> Vec<&Proposal> {
    let quota = rule.map_or(proposals.len(), |r| r.quota(proposals.len()));
    let mut out: Vec<&Proposal> = proposals.iter().filter(|p| p.judged).collect();
    for p in proposals.iter().filter(|p| !p.judged) {
        if out.len() >= quota {
            break;
        }
        out.push(p);
    }
    out
}

/// proposed / sampled / judged across every `.tracking` file, each file sampled on its own.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct ProposalCounts {
    pub proposed: usize,
    pub sampled: usize,
    pub judged: usize,
}

impl ProposalCounts {
    /// Proposals no human has judged.
    #[must_use]
    pub const fn awaiting(&self) -> usize {
        self.proposed.saturating_sub(self.judged)
    }
}

#[must_use]
pub fn proposal_counts(root: &Path) -> ProposalCounts {
    let rule = sampling_rule(root);
    let mut c = ProposalCounts::default();
    for f in crate::collect_sysml(&root.join(".tracking")) {
        let ps = proposals_in(root, &f);
        c.proposed += ps.len();
        c.judged += ps.iter().filter(|p| p.judged).count();
        c.sampled += sample(&ps, rule).len();
    }
    c
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
    let pc = proposal_counts(&root);
    let rule = sampling_rule(&root);
    let pct = |a: usize, b: usize| (a * 100).checked_div(b).unwrap_or(0);

    if args.iter().any(|a| a == "--json") {
        let rows: Vec<String> = c
            .iter()
            .map(|(k, v)| {
                format!(
                    "{{\"judge\":\"{k}\",\"total\":{},\"exercised\":{},\"exercisedWithReceipt\":{},\"examined\":{},\"failed\":{},\"proposed\":{},\"receiptPct\":{},\"failPct\":{}}}",
                    v.total, v.exercised, v.exercised_with_receipt, v.examined, v.failed, v.proposed,
                    pct(v.exercised_with_receipt, v.exercised), pct(v.failed, v.total)
                )
            })
            .collect();
        let rule_json = match rule {
            Some(SamplingRule::Share(p)) => format!("\"{p}%\""),
            Some(SamplingRule::Count(n)) => n.to_string(),
            None => "null".to_owned(),
        };
        println!(
            "{{\"byJudge\":[{}],\"uncitedCoverageClaims\":{claims},\"proposals\":{{\"proposed\":{},\"sampled\":{},\"judged\":{},\"awaiting\":{},\"sampling\":{rule_json}}}}}",
            rows.join(","),
            pc.proposed,
            pc.sampled,
            pc.judged,
            pc.awaiting()
        );
        return 0;
    }

    println!("attestation census — is a `pass` a RECEIPT or a TESTIMONY?");
    println!();
    println!("  {:<14} {:>7} {:>10} {:>9} {:>9} {:>8} {:>8}", "JUDGE", "results", "exercised", "w/receipt", "examined", "failed", "proposed");
    for (k, v) in &c {
        println!(
            "  {:<14} {:>7} {:>10} {:>8}% {:>9} {:>7}% {:>8}",
            k, v.total, v.exercised, pct(v.exercised_with_receipt, v.exercised), v.examined, pct(v.failed, v.total), v.proposed
        );
    }
    println!();
    println!("  proposed is the third tier (D0312 B): an AI-examined pass no human has judged - done for");
    println!("  nothing until one does, and never folded into pass or fail.");
    println!();
    let rule_text = match rule {
        Some(SamplingRule::Share(p)) => format!("{p}% of each file's proposals"),
        Some(SamplingRule::Count(n)) => format!("{n} per file"),
        None => "no rule - every proposal".to_owned(),
    };
    println!(
        "  proposals (D0443): {} proposed, {} sampled for a human's judgment ({rule_text}), {} judged, {} awaiting.",
        pc.proposed,
        pc.sampled,
        pc.judged,
        pc.awaiting()
    );
    println!("  `keel judge-set <file> --words ... --by <human> --date ...` records one result and one quote receipt per item.");
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

    /// D0312 B: `proposed_count` is the burndown's number. Positive: a tree holding one `TestResult`
    /// with `VerdictKind::proposed` counts 1. Negative: the same tree with that line at pass counts 0.
    /// And orient's `gate_passed` - the reader that decides done-ness - does not read a proposed gate
    /// as passed, so a proposed sprint is not done by construction (no reader was taught the word).
    #[test]
    fn a_proposed_result_is_counted_and_is_not_a_passed_gate() {
        let root = std::env::temp_dir().join("keel-attest-proposed");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".tracking").join("delivery")).expect("mkdir");
        let f = root.join(".tracking").join("delivery").join("s.sysml");
        let proposed = "package S {\n    verification sRefineGate : Test { :>> id = \"e2e00000-0000-4000-8000-00000000f101\"; :>> method = VerificationMethod::inspect; }\n    part sRefineGateR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-00000000f102\"; :>> outcome = VerdictKind::proposed; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-10\"; :>> judgedBy = \"bot\"; }\n}\n";
        std::fs::write(&f, proposed).expect("write");
        assert_eq!(super::proposed_count(&root), 1, "one proposed result is counted");
        assert!(!crate::orient::gate_passed(proposed, "sRefine"), "a proposed gate is NOT passed");
        let passed = proposed.replace("VerdictKind::proposed", "VerdictKind::pass");
        std::fs::write(&f, &passed).expect("write");
        assert_eq!(super::proposed_count(&root), 0, "a pass is not proposed");
        assert!(crate::orient::gate_passed(&passed, "sRefine"), "the same gate at pass IS passed");
    }

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

    /// Six proposals in one file, ordered by their result uuid. Nine `TestResult` lines in all: six
    /// proposed, two passes on other tests (never proposals), one fail (not a proposal either).
    fn six_proposals() -> String {
        let mut s = String::from("package S {\n");
        for (i, uuid_tail) in ["f", "3", "a", "0", "c", "7"].iter().enumerate() {
            let _ = std::fmt::Write::write_fmt(&mut s, format_args!(
                "    verification t{i}Gate : Test {{ :>> id = \"e2e00000-0000-4000-8000-0000000000{i}1\"; :>> method = VerificationMethod::inspect; }}\n    part t{i}GateR1 : TestResult {{ :>> id = \"{uuid_tail}2e00000-0000-4000-8000-0000000000{i}2\"; :>> outcome = VerdictKind::proposed; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-10\"; :>> judgedBy = \"bot\"; }}\n"
            ));
        }
        s.push_str("    verification pDoD : Test { :>> id = \"e2e00000-0000-4000-8000-0000000000a1\"; :>> method = VerificationMethod::test; }\n");
        s.push_str("    part pDoDR1 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000a2\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1234\"; :>> judgedAt = \"2026-09-10\"; :>> judgedBy = \"bot\"; }\n");
        s.push_str("    part pDoDR2 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000a3\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc1235\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n");
        s.push_str("    part pDoDR3 : TestResult { :>> id = \"e2e00000-0000-4000-8000-0000000000a4\"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = \"abc1236\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"bot\"; }\n");
        s.push_str("}\n");
        s
    }

    /// D0443 / D0388 known-positive: six proposals under a 50% rule sample THREE - the three whose
    /// result uuid sorts first (t3, t1, t5: tails 0, 3, 7) - and judge none. The order is the uuid's,
    /// not the declaration's, so an agent cannot steer which of its proposals a human reads.
    #[test]
    fn six_proposals_under_a_half_rule_sample_three_by_uuid_and_judge_none() {
        let human = |by: &str| by == "you";
        let ps = super::proposals_in_text(&six_proposals(), &human);
        assert_eq!(ps.len(), 6, "six proposed results, and the passes and the fail are not proposals");
        assert!(ps.iter().all(|p| !p.judged));
        let order: Vec<&str> = ps.iter().map(|p| p.test.as_str()).collect();
        assert_eq!(order, ["t3Gate", "t1Gate", "t5Gate", "t2Gate", "t4Gate", "t0Gate"], "ordered by result uuid");
        let s = super::sample(&ps, Some(super::SamplingRule::Share(50)));
        assert_eq!(s.iter().map(|p| p.test.as_str()).collect::<Vec<_>>(), ["t3Gate", "t1Gate", "t5Gate"]);
        assert_eq!(super::sample(&ps, Some(super::SamplingRule::Count(2))).len(), 2, "a count rule is min(count, n)");
        assert_eq!(super::sample(&ps, None).len(), 6, "no rule samples everything");
        assert_eq!(super::SamplingRule::Share(50).quota(5), 3, "ceil, never floor: half of five is three");
    }

    /// D0443 / D0388 known-negative pair: the same tree with ONE human result on a proposal samples three
    /// and judges one - the judged proposal is in the sample first even when its uuid sorts last - and a
    /// tree whose results are all passes proposes nothing.
    #[test]
    fn a_human_result_is_judged_and_counted_into_the_sample_first_and_passes_propose_nothing() {
        let human = |by: &str| by == "you";
        // t0Gate's result uuid sorts LAST (tail f); a human judges it.
        let judged = format!(
            "{}    part t0GateR2 : TestResult {{ :>> id = \"e2e00000-0000-4000-8000-0000000000b2\"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = \"abc1237\"; :>> judgedAt = \"2026-09-11\"; :>> judgedBy = \"you\"; :>> createdBy = \"bot\"; }}\n}}\n",
            six_proposals().as_str().strip_suffix("}\n").expect("the fixture closes its package")
        );
        let ps = super::proposals_in_text(&judged, &human);
        assert_eq!(ps.len(), 6);
        assert_eq!(ps.iter().filter(|p| p.judged).count(), 1);
        let s = super::sample(&ps, Some(super::SamplingRule::Share(50)));
        assert_eq!(s.len(), 3, "sampled is never below judged and the quota still holds");
        assert_eq!(s[0].test, "t0Gate", "the judged proposal is in the sample first");
        // an AI's later result on a proposal is NOT a judgment
        let ai_later = judged.replace(":>> judgedBy = \"you\"; :>> createdBy = \"bot\";", ":>> judgedBy = \"bot\"; :>> createdBy = \"bot\";");
        assert_eq!(super::proposals_in_text(&ai_later, &human).iter().filter(|p| p.judged).count(), 0);
        // a tree of passes proposes nothing
        let passes = six_proposals().replace("VerdictKind::proposed", "VerdictKind::pass");
        assert!(super::proposals_in_text(&passes, &human).is_empty());
        // the rule parses a share and a count, and refuses a share past 100
        assert_eq!(super::SamplingRule::parse(&toml::Value::String("50%".into())), Some(super::SamplingRule::Share(50)));
        assert_eq!(super::SamplingRule::parse(&toml::Value::Integer(3)), Some(super::SamplingRule::Count(3)));
        assert_eq!(super::SamplingRule::parse(&toml::Value::String("150%".into())), None);
    }
}
