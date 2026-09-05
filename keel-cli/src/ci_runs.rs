//! `keel audit-ci-runs [ROOT] [--repo OWNER/NAME]` (D0323 / issue374, the external-fact gate).
//!
//! A `// RAN:` receipt (D0232) proves a sentence was typed. It does not prove a run happened: on
//! 2026-09-04 a suite count was written into a receipt while the suite was still running, and only the
//! author's own read-back caught it (stpa-self run 1, UCA-A1). The landscape finding behind this
//! (Beads' `gh:run` gates, D0282) is that a gate which closes only when an EXTERNAL fact exists is a
//! gate the agent cannot talk its way past.
//!
//! This is that class of result: a receipt of the form `// RAN: ci-run id=<run id> workflow=<name>`
//! names a GitHub Actions run. CI - the party the agent cannot write to - queries the run and checks
//! that it exists in THIS repository, concluded `success`, and ran on the result's `judgedAgainst`
//! SHA. A fabricated id, a different SHA, a failed run, or a run from another repository is a
//! violation naming the result. The check runs in `ci.yml` under the keystone lock, not as a local
//! guard: a commit gate must not need the network, and the whole point is that the agent's machine
//! is not the judge. Locally the command runs the same check when `gh` is available.
//!
//! The verdict is a pure function over the receipt and the run's JSON, so every branch is unit-tested
//! without a network; only the fetch is `gh api`.

use std::path::Path;

/// One `ci-run` receipt found in the tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiRunReceipt {
    pub file: String,
    pub line: usize,
    pub result: String,
    pub judged_against: String,
    pub run_id: String,
    pub workflow: String,
}

/// Parse `ci-run id=<digits> workflow=<name>` out of a receipt's text; `None` for any other receipt.
#[must_use]
pub fn parse_receipt(text: &str) -> Option<(String, String)> {
    let rest = text.split("ci-run").nth(1)?;
    let mut id = None;
    let mut workflow = None;
    for tok in rest.split_whitespace() {
        if let Some(v) = tok.strip_prefix("id=") {
            if !v.is_empty() && v.chars().all(|c| c.is_ascii_digit()) {
                id = Some(v.to_string());
            }
        } else if let Some(v) = tok.strip_prefix("workflow=") {
            // a receipt is prose: the name may be followed by the sentence's own punctuation
            let v = v.trim_matches(|c: char| c == '"' || c == ';' || c == ',' || c == ')' || c == '.');
            if !v.is_empty() {
                workflow = Some(v.to_string());
            }
        }
    }
    Some((id?, workflow?))
}

/// The value of `:>> key = "..."` on a result line.
fn field(line: &str, key: &str) -> Option<String> {
    let pattern = format!(":>> {key} = \"");
    let start = line.find(&pattern)? + pattern.len();
    let rest = &line[start..];
    rest.find('"').map(|end| rest[..end].to_string())
}

/// Every `ci-run` receipt under `.tracking`, with the result it attests and that result's SHA.
#[must_use]
pub fn receipts(root: &Path) -> Vec<CiRunReceipt> {
    let mut out = Vec::new();
    for f in crate::collect_sysml(&root.join(".tracking")) {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !line.contains(" : TestResult") {
                continue;
            }
            // the receipt is on the result line or the line above (the shape `evidence-cited` reads)
            let receipt = if line.contains("// RAN:") {
                line.split("// RAN:").nth(1).map(str::to_string)
            } else {
                i.checked_sub(1).and_then(|j| lines.get(j)).and_then(|p| p.trim_start().strip_prefix("// RAN:")).map(str::to_string)
            };
            let Some(receipt) = receipt else { continue };
            let Some((run_id, workflow)) = parse_receipt(&receipt) else { continue };
            let result = line.split(" : TestResult").next().and_then(|s| s.split("part ").nth(1)).unwrap_or("?").trim().to_string();
            let judged_against = field(line, "judgedAgainst").unwrap_or_default();
            let file = f.strip_prefix(root).unwrap_or(&f).to_string_lossy().replace('\\', "/");
            out.push(CiRunReceipt { file, line: i + 1, result, judged_against, run_id, workflow });
        }
    }
    out
}

/// What the run API said, reduced to the four facts the verdict reads. `None` = the run does not exist
/// (a 404, or an id that is not a run).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunFacts {
    pub repository: String,
    pub head_sha: String,
    pub conclusion: String,
    pub workflow_name: String,
}

/// Reduce `gh api repos/O/N/actions/runs/<id>` JSON to [`RunFacts`]; `None` when the body is not a run.
#[must_use]
pub fn run_facts(json: &str) -> Option<RunFacts> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let head_sha = v.get("head_sha")?.as_str()?.to_string();
    Some(RunFacts {
        repository: v.get("repository").and_then(|r| r.get("full_name")).and_then(|s| s.as_str()).unwrap_or("").to_string(),
        head_sha,
        conclusion: v.get("conclusion").and_then(|s| s.as_str()).unwrap_or("").to_string(),
        workflow_name: v.get("name").and_then(|s| s.as_str()).unwrap_or("").to_string(),
    })
}

/// The pure verdict: does this run vouch for this result?
///
/// # Errors
/// One sentence naming what does not hold - the run is missing, belongs to another repository, ran on
/// a different tree, did not conclude `success`, or is another workflow.
pub fn verdict(receipt: &CiRunReceipt, slug: &str, facts: Option<&RunFacts>) -> Result<(), String> {
    let Some(f) = facts else {
        return Err(format!("{}:{}: {} cites ci-run id={} which does not exist in {slug} - a fabricated or mistyped run id; the pass has no external fact behind it", receipt.file, receipt.line, receipt.result, receipt.run_id));
    };
    if !f.repository.eq_ignore_ascii_case(slug) {
        return Err(format!("{}:{}: {} cites ci-run id={} which belongs to {} not {slug}", receipt.file, receipt.line, receipt.result, receipt.run_id, f.repository));
    }
    if receipt.judged_against.is_empty() || !f.head_sha.starts_with(&receipt.judged_against) && !receipt.judged_against.starts_with(&f.head_sha) {
        return Err(format!("{}:{}: {} cites ci-run id={} which ran on {} but the result is judgedAgainst {} - a green run for a DIFFERENT tree vouches for nothing", receipt.file, receipt.line, receipt.result, receipt.run_id, &f.head_sha[..f.head_sha.len().min(12)], receipt.judged_against));
    }
    if f.conclusion != "success" {
        return Err(format!("{}:{}: {} cites ci-run id={} whose conclusion is `{}`, not success", receipt.file, receipt.line, receipt.result, receipt.run_id, if f.conclusion.is_empty() { "(none yet)" } else { &f.conclusion }));
    }
    if !receipt.workflow.is_empty() && !f.workflow_name.eq_ignore_ascii_case(&receipt.workflow) {
        return Err(format!("{}:{}: {} cites ci-run id={} as workflow `{}` but the run is `{}`", receipt.file, receipt.line, receipt.result, receipt.run_id, receipt.workflow, f.workflow_name));
    }
    Ok(())
}

fn fetch(slug: &str, run_id: &str) -> Result<Option<RunFacts>, String> {
    let out = std::process::Command::new("gh").args(["api", &format!("repos/{slug}/actions/runs/{run_id}")]).output().map_err(|e| format!("could not run gh: {e}"))?;
    if out.status.success() {
        Ok(run_facts(&String::from_utf8_lossy(&out.stdout)))
    } else {
        let err = String::from_utf8_lossy(&out.stderr);
        if err.contains("404") || err.contains("Not Found") {
            Ok(None)
        } else {
            Err(format!("gh api failed for run {run_id}: {}", err.trim()))
        }
    }
}

fn github_slug(root: &Path) -> Option<String> {
    let url = crate::gitx::git().arg("-C").arg(root).args(["config", "--get", "remote.origin.url"]).output().ok().filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;
    let rest = url.strip_prefix("https://github.com/").or_else(|| url.strip_prefix("git@github.com:"))?;
    Some(rest.trim_end_matches(".git").trim_end_matches('/').to_string())
}

/// `keel audit-ci-runs [ROOT] [--repo OWNER/NAME]`: exit 0 when every ci-run receipt is vouched for
/// (or there are none - stated), 1 on any violation, 2 when the check cannot run.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    let Some(slug) = args.iter().position(|a| a == "--repo").and_then(|i| args.get(i + 1)).cloned().or_else(|| github_slug(root)) else {
        eprintln!("audit-ci-runs: no GitHub repository - pass --repo OWNER/NAME or set remote.origin.url");
        return 2;
    };
    let found = receipts(root);
    if found.is_empty() {
        println!("audit-ci-runs: 0 ci-run receipt(s) in .tracking - nothing cites an external run yet (a `// RAN: ci-run id=<id> workflow=<name>` receipt is checked here against {slug})");
        return 0;
    }
    let mut violations = Vec::new();
    for r in &found {
        match fetch(&slug, &r.run_id) {
            Ok(facts) => {
                if let Err(v) = verdict(r, &slug, facts.as_ref()) {
                    violations.push(v);
                }
            }
            Err(e) => {
                eprintln!("audit-ci-runs: {e}");
                return 2;
            }
        }
    }
    for v in &violations {
        println!("  {} {v}", crate::color::fail("ERROR"));
    }
    println!("[audit-ci-runs] {} — {} receipt(s) checked against {slug}, {} violation(s)", if violations.is_empty() { crate::color::pass("PASS") } else { crate::color::fail("FAIL") }, found.len(), violations.len());
    i32::from(!violations.is_empty())
}

#[cfg(test)]
mod tests {
    use super::{parse_receipt, run_facts, verdict, CiRunReceipt, RunFacts};

    fn receipt(sha: &str) -> CiRunReceipt {
        CiRunReceipt { file: ".tracking/backlog.sysml".into(), line: 7, result: "dcXDoDR1".into(), judged_against: sha.into(), run_id: "1234".into(), workflow: "rust".into() }
    }
    fn green(sha: &str) -> RunFacts {
        RunFacts { repository: "o/n".into(), head_sha: sha.into(), conclusion: "success".into(), workflow_name: "rust".into() }
    }

    /// Found by the first LIVE run, not by a unit test: an off-by-one read judgedAgainst as empty and the
    /// real receipt was refused as a run for a different tree. The reader is now pinned.
    #[test]
    fn a_result_lines_judged_against_is_read_exactly() {
        let line = "        part xDoDR1 : TestResult { :>> id = \"1\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"50175b68d8fb\"; :>> judgedAt = \"2026-09-05\"; }";
        assert_eq!(super::field(line, "judgedAgainst").as_deref(), Some("50175b68d8fb"));
        assert_eq!(super::field(line, "judgedAt").as_deref(), Some("2026-09-05"));
        assert_eq!(super::field(line, "judgedBy"), None);
    }

    #[test]
    fn the_receipt_grammar_is_strict_and_other_receipts_are_not_ci_runs() {
        assert_eq!(parse_receipt(" ci-run id=1234 workflow=rust"), Some(("1234".into(), "rust".into())));
        assert_eq!(parse_receipt("cargo test --release: 512 passed"), None, "an ordinary receipt is not a ci-run");
        assert_eq!(parse_receipt("ci-run id=abc workflow=rust"), None, "a non-numeric id is not an id");
        assert_eq!(parse_receipt("ci-run id=1234"), None, "the workflow name is required");
        assert_eq!(parse_receipt("ci-run id=1234 workflow=ci; suite 521/0"), Some(("1234".into(), "ci".into())), "found live: the sentence's semicolon is not part of the name");
    }

    /// The `DoD` three cases: a real green run for the judged SHA passes; a fabricated id fails naming
    /// the result; a real run for a DIFFERENT sha fails.
    #[test]
    fn a_green_run_for_the_judged_sha_passes_and_the_two_fabrications_fail_by_name() {
        let full = "50175b6a1b2c3d4e5f60718293a4b5c6d7e8f901";
        assert_eq!(verdict(&receipt("50175b6"), "o/n", Some(&green(full))), Ok(()), "short judgedAgainst matches the run's full sha");
        let missing = verdict(&receipt("50175b6"), "o/n", None).unwrap_err();
        assert!(missing.contains("dcXDoDR1") && missing.contains("does not exist"), "{missing}");
        let other = verdict(&receipt("deadbee"), "o/n", Some(&green(full))).unwrap_err();
        assert!(other.contains("DIFFERENT tree") && other.contains("deadbee"), "{other}");
    }

    #[test]
    fn a_failed_run_another_repository_or_another_workflow_do_not_vouch() {
        let full = "50175b6a1b2c3d4e5f60718293a4b5c6d7e8f901";
        let mut f = green(full);
        f.conclusion = "failure".into();
        assert!(verdict(&receipt("50175b6"), "o/n", Some(&f)).unwrap_err().contains("failure"));
        let mut f = green(full);
        f.repository = "someone/else".into();
        assert!(verdict(&receipt("50175b6"), "o/n", Some(&f)).unwrap_err().contains("someone/else"));
        let mut f = green(full);
        f.workflow_name = "release".into();
        assert!(verdict(&receipt("50175b6"), "o/n", Some(&f)).unwrap_err().contains("release"));
    }

    #[test]
    fn the_api_shape_reduces_to_facts_and_a_non_run_body_is_none() {
        let json = r#"{"id":1234,"name":"rust","head_sha":"50175b6a1b2c3d4e5f60718293a4b5c6d7e8f901","conclusion":"success","repository":{"full_name":"o/n"}}"#;
        let f = run_facts(json).expect("a run");
        assert_eq!(f.repository, "o/n");
        assert_eq!(f.conclusion, "success");
        assert!(run_facts(r#"{"message":"Not Found"}"#).is_none());
    }
}
