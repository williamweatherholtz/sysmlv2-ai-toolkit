//! `keel currency [ROOT] --repo OWNER/NAME --by ACTOR [--at DATE] [--skip-pull]` (D0338, dcScheduledCurrency).
//!
//! The UNATTENDED currency pass, in one place: pull the repository's open issues as verbatim
//! Statements (idempotent on the issue URL, D0264's trust tier recorded on each, NEVER triaged), sync
//! the unit library (an unreachable remote is STATED as stale, never treated as up to date), and read
//! the library drift back. One report, one exit code: 0 when every pass ran and nothing failed, 1 when
//! a pass failed, 2 when the pass could not start.
//!
//! WHY ONE COMMAND. A schedule that runs three commands has three places to be silently wrong; a
//! schedule that runs one command whose output is the report has one. The schedule itself is the
//! declared, removable file `.github/workflows/currency.yml` (D0163: delete the file, the schedule is
//! gone) and it runs this command - it holds no logic of its own.
//!
//! AUTONOMY STAYS BOUND BY D0264: a pull records WORDS. What an issue implicates is a judgment, and an
//! unattended judgment on untrusted input is the thing the trust tier exists to prevent, so nothing here
//! triages, routes or records an Issue.

use std::path::Path;

/// One pass's outcome for the summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassOutcome {
    pub name: &'static str,
    pub code: i32,
    pub note: String,
}

/// The one-place summary and the exit code it implies (pure, unit-tested).
#[must_use]
pub fn summary(passes: &[PassOutcome]) -> (String, i32) {
    use std::fmt::Write as _;
    let mut out = String::from("currency pass:\n");
    let mut worst = 0;
    for p in passes {
        let verdict = match p.code {
            0 => "ok",
            2 => "could not start",
            _ => "FAILED",
        };
        let _ = writeln!(out, "  {:<8} {verdict}{}{}", p.name, if p.note.is_empty() { "" } else { " - " }, p.note);
        worst = worst.max(if p.code == 2 { 2 } else { i32::from(p.code != 0) });
    }
    out.push_str(if worst == 0 { "  nothing failed; what was found is above, and nothing was triaged (D0264)." } else { "  a pass did not complete - read it above; nothing was triaged (D0264)." });
    (out, worst)
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == &format!("--{name}")).and_then(|i| args.get(i + 1)).cloned()
}

/// Run the pass. Prints each stage under its own header, then the summary.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    let repo = flag(args, "repo").or_else(|| github_slug(root));
    let by = flag(args, "by");
    let at = flag(args, "at").unwrap_or_else(crate::scaffold::today);
    let skip_pull = args.iter().any(|a| a == "--skip-pull");
    let mut passes = Vec::new();

    println!("== 1/3 github-pull (verbatim Statements, never triaged; D0264) ==");
    if skip_pull {
        passes.push(PassOutcome { name: "pull", code: 0, note: "skipped by --skip-pull".to_string() });
    } else {
        match (repo.as_deref(), by.as_deref()) {
            (Some(r), Some(b)) => {
                let pull_args: Vec<String> = ["--repo", r, "--by", b, "--at", &at].iter().map(|s| (*s).to_string()).collect();
                let code = crate::github_ingest::pull_cmd(&pull_args, root);
                passes.push(PassOutcome { name: "pull", code, note: format!("{r} as {b} on {at}") });
            }
            (None, _) => {
                eprintln!("currency: no GitHub repository - pass --repo OWNER/NAME or set remote.origin.url");
                passes.push(PassOutcome { name: "pull", code: 2, note: "no repository".to_string() });
            }
            (_, None) => {
                eprintln!("currency: --by ACTOR is required - the recorder of an utterance is never defaulted (D0129)");
                passes.push(PassOutcome { name: "pull", code: 2, note: "no --by actor".to_string() });
            }
        }
    }

    println!("== 2/3 library sync (unreachable is STATED stale, never up to date; D0250) ==");
    let lib = crate::library::cmd_sync();
    passes.push(PassOutcome { name: "library", code: lib, note: match lib { 0 => "synced or stated".to_string(), 2 => "not initialised on this machine".to_string(), _ => "diverged - a defect, not a merge".to_string() } });

    println!("== 3/3 drift (this project against the library) ==");
    let drift = crate::status::library_lines(root);
    for l in &drift {
        println!("  {l}");
    }
    passes.push(PassOutcome { name: "drift", code: 0, note: format!("{} line(s) read back", drift.len()) });

    let (text, code) = summary(&passes);
    println!("{text}");
    code
}

fn github_slug(root: &Path) -> Option<String> {
    let url = crate::gitx::git().arg("-C").arg(root).args(["config", "--get", "remote.origin.url"]).output().ok().filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;
    let rest = url.strip_prefix("https://github.com/").or_else(|| url.strip_prefix("git@github.com:"))?;
    Some(rest.trim_end_matches(".git").trim_end_matches('/').to_string())
}

#[cfg(test)]
mod tests {
    use super::{summary, PassOutcome};

    /// One report, one exit code: a failed pass is named and makes the code 1; a pass that could not
    /// start makes it 2; all green is 0 and says nothing was triaged.
    #[test]
    fn the_summary_names_every_pass_and_the_worst_code_wins() {
        let ok = vec![
            PassOutcome { name: "pull", code: 0, note: "o/n as githubRecorder on 2026-09-05".into() },
            PassOutcome { name: "library", code: 0, note: "synced or stated".into() },
            PassOutcome { name: "drift", code: 0, note: "3 line(s) read back".into() },
        ];
        let (text, code) = summary(&ok);
        assert_eq!(code, 0);
        assert!(text.contains("pull     ok") && text.contains("nothing was triaged"), "{text}");
        let mut failed = ok.clone();
        failed[1].code = 1;
        assert_eq!(summary(&failed).1, 1);
        assert!(summary(&failed).0.contains("library  FAILED"));
        let mut unstarted = ok;
        unstarted[0].code = 2;
        assert_eq!(summary(&unstarted).1, 2);
        assert!(summary(&unstarted).0.contains("could not start"));
    }
}
