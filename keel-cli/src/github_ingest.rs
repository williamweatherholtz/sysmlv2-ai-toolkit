//! `keel github-ingest` — a GitHub issue becomes a recorded utterance (D0263).
//!
//! # Why this records a Statement and not an Issue
//!
//! A GitHub issue is SOMEONE'S WORDS. What those words implicate — a bug, a need, a process change,
//! nothing at all — is a judgment, and D0216 puts the judgment in a separate item that CITES the
//! words rather than replacing them. Ingesting straight to an `Issue` would fuse the two and throw
//! away the only artifact a later reader can check my reading against.
//!
//! There is a second, harder reason. `record issue` requires a RESOLVER, because an untriaged issue
//! is how a tracker fills with things nobody owns. Ingestion cannot know the resolver — so an
//! ingest-to-Issue path would have to invent one, which is precisely the fabrication the intake
//! process exists to prevent. Recording the utterance and leaving triage to the operator keeps the
//! refusal honest: nothing is written that nobody chose.
//!
//! # Idempotency
//!
//! The `sourceUrl` is the issue's durable address. `record_statement` refuses when an utterance from
//! that URL is already recorded, checked INSIDE the write lock, so two concurrent ingests cannot
//! both find it absent (issue185). A re-ingest REFUSES rather than silently deduplicating: the
//! caller asked to record something already recorded, and a write that reports success while doing
//! nothing is the failure `keel deactivate` once had.
//!
//! # The network boundary
//!
//! Fetching is `gh api`, and it is separated from parsing so the parse is testable without a
//! network (pf44: a suite whose only real-world contact is skipped and reported green proves
//! nothing). `--from FILE` reads the same JSON shape from disk — the fixture path used by tests and
//! by anyone working offline.

use std::path::Path;
use std::process::Command;

/// The fields of a GitHub issue this command records, and nothing more.
#[derive(Debug)]
pub struct IngestedIssue {
    pub number: u64,
    pub title: String,
    /// The issue body VERBATIM — never trimmed, never tidied (D0216).
    pub body: String,
    /// The reporter's GitHub login. Deliberately NOT an enrolled `ProjectActor`: an outside
    /// reporter has no actor id here, and inventing one would misattribute their words.
    pub login: String,
    /// `YYYY-MM-DD`, taken from the issue's `created_at`.
    pub created_on: String,
    pub url: String,
}

/// Parse the `gh api` issue JSON shape. Returns `Err` with the missing field named, rather than
/// substituting a default — a Statement with a defaulted author or date is a fabricated provenance.
///
/// # Errors
/// When the payload is not an object, or a required field is absent or of the wrong type.
pub fn parse_issue(json: &str) -> Result<IngestedIssue, String> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("not JSON: {e}"))?;
    let need_str = |k: &str| -> Result<String, String> {
        v.get(k)
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| format!("issue JSON has no string `{k}` — refusing rather than defaulting it"))
    };
    let number = v
        .get("number")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "issue JSON has no numeric `number`".to_string())?;
    let login = v
        .get("user")
        .and_then(|u| u.get("login"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "issue JSON has no `user.login` — whose words these are is not defaultable".to_string())?
        .to_string();
    let created_at = need_str("created_at")?;
    let created_on = created_at.get(..10).unwrap_or(&created_at).to_string();
    // An issue opened with an empty body is a real thing on GitHub and it records NOTHING, so it is
    // refused here rather than stored as an utterance with no content.
    let body = v.get("body").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
    if body.trim().is_empty() {
        return Err(format!("issue #{number} has an empty body — there is no utterance to record"));
    }
    Ok(IngestedIssue { number, title: need_str("title")?, body, login, created_on, url: need_str("html_url")? })
}

fn fetch(repo: &str, number: &str) -> Result<String, String> {
    let out = Command::new("gh")
        .args(["api", &format!("repos/{repo}/issues/{number}")])
        .output()
        .map_err(|e| format!("could not run `gh` ({e}) — install the GitHub CLI or pass --from FILE"))?;
    if !out.status.success() {
        return Err(format!("gh api failed: {}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == &format!("--{name}")).and_then(|i| args.get(i + 1)).cloned()
}

/// `keel github-ingest --repo OWNER/NAME --issue N [--from FILE] --by ACTOR --at DATE [ROOT]`
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    let json = match (flag(args, "from"), flag(args, "repo"), flag(args, "issue")) {
        (Some(f), _, _) => match std::fs::read_to_string(&f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("github-ingest: cannot read {f}: {e}");
                return 2;
            }
        },
        (None, Some(repo), Some(n)) => match fetch(&repo, &n) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("github-ingest: {e}");
                return 1;
            }
        },
        _ => {
            eprintln!(
                "usage: keel github-ingest --repo OWNER/NAME --issue N [--from FILE] --by ACTOR --at YYYY-MM-DD [ROOT]"
            );
            return 2;
        }
    };
    let issue = match parse_issue(&json) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("github-ingest: {e}");
            return 1;
        }
    };
    // The RECORDER and the recording date are keel's own provenance, never defaulted (D0129).
    let Some(author) = flag(args, "by").or_else(|| std::env::var("KEEL_ACTOR").ok()) else {
        eprintln!("github-ingest: --by ACTOR required (or KEEL_ACTOR) — who recorded this is its own fact");
        return 2;
    };
    let Some(at) = flag(args, "at") else {
        eprintln!("github-ingest: --at YYYY-MM-DD required — when it was recorded is its own fact");
        return 2;
    };
    let title = format!("GH#{} {}", issue.number, issue.title);
    match crate::intake_write::record_statement(
        root,
        &crate::intake_write::NewStatement {
            text: &issue.body,
            said_by: &issue.login,
            said_at: &issue.created_on,
            channel: "github",
            source_url: Some(&issue.url),
            title: &title,
            author: &author,
            created_at: &at,
        },
    ) {
        Ok((name, file)) => {
            println!("ingested GH#{} -> {name} in {file}", issue.number);
            println!("  their words are stored VERBATIM, attributed to `{}` (a GitHub login, not an", issue.login);
            println!("  enrolled actor — an outside reporter has no actor id here).");
            println!("  NOT YET TRIAGED. What this implicates is a JUDGMENT and is yours to make:");
            println!("    keel record story --from-statement {name} --implication <kind> --triage-note \"...\"");
            0
        }
        Err(e) => {
            eprintln!("github-ingest: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_issue;

    const OK: &str = r#"{"number":42,"title":"gate refuses on a fresh clone","body":"Steps:\n1. clone\n2. run keel gate","user":{"login":"someone"},"created_at":"2026-08-30T11:22:33Z","html_url":"https://github.com/o/r/issues/42"}"#;

    #[test]
    fn a_well_formed_issue_parses_with_every_field_kept() {
        let i = parse_issue(OK).expect("parses");
        assert_eq!(i.number, 42);
        assert_eq!(i.login, "someone");
        assert_eq!(i.created_on, "2026-08-30", "the DATE is taken from created_at, not from today");
        assert!(i.body.contains("1. clone"), "the body is kept verbatim, newlines and all");
    }

    #[test]
    fn a_missing_reporter_is_refused_rather_than_defaulted() {
        let no_user = OK.replace(r#""user":{"login":"someone"},"#, "");
        let e = parse_issue(&no_user).expect_err("must refuse");
        assert!(e.contains("user.login"), "and it says WHICH field: {e}");
    }

    #[test]
    fn an_empty_body_records_nothing_and_says_so() {
        let empty = OK.replace(r#""body":"Steps:\n1. clone\n2. run keel gate""#, r#""body":"""#);
        let e = parse_issue(&empty).expect_err("must refuse");
        assert!(e.contains("no utterance"), "{e}");
    }
}
