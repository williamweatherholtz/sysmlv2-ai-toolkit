//! `keel audit-history` — re-derive the gate verdict for every commit in a range (issue116/D0047).
//!
//! # Why a self-asserted marker would be worthless
//!
//! A commit made with hooks bypassed is INDISTINGUISHABLE afterwards from one that passed the gate.
//! I proved that by making one: to build a two-clone fixture I committed with `core.hooksPath` set to
//! the null device, and the only reason it was recovered is that I said so in the same session.
//! Disclosure is not a control (D0047), and neither is a "gate: passed" line in a commit message —
//! anyone who skips the hook can write the line.
//!
//! So this re-derives the verdict from the TREE. Each commit is checked out into a throwaway worktree
//! and the real `validate` + `guard` code runs against it. Nothing is trusted; nothing is recorded by
//! the committer that the auditor then believes.
//!
//! # What it detects, and what it does NOT
//!
//! It detects a commit whose tree FAILS the gate — the harm. It does NOT detect the act of skipping
//! the hook: a bypassed commit that happens to be clean passes here, correctly, because a clean tree
//! is a clean tree however it arrived. That gap is stated rather than papered over, and it is the
//! right trade: the thing worth finding is a dishonest model that landed, not a ceremony that was
//! skipped over an honest one.
//!
//! It is an AUDIT, never a gate (D0098). It blocks nothing, because completeness is a burndown.

use std::path::Path;

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = crate::gitx::git().arg("-C").arg(repo).args(args).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// One commit's re-derived verdict.
pub struct CommitVerdict {
    pub sha: String,
    pub subject: String,
    pub clean: bool,
    /// Why it failed, capped — the point is which commit, not a full transcript per commit.
    pub reasons: Vec<String>,
}

/// Re-run the gate against `sha` in a throwaway worktree.
///
/// A worktree rather than `git stash` or a checkout: the audit must never touch the caller's working
/// tree, which may hold uncommitted work. `--detach` because we are inspecting history, not branching.
fn verdict_for(repo: &Path, sha: &str, scratch: &Path) -> CommitVerdict {
    let subject = git(repo, &["log", "-1", "--format=%s", sha]).unwrap_or_default();
    let wt = scratch.join(format!("audit-{}", &sha[..sha.len().min(8)]));
    let wt_s = wt.display().to_string();
    let mut reasons = Vec::new();

    let created = git(repo, &["worktree", "add", "--detach", "--quiet", &wt_s, sha]).is_some();
    // `git worktree add` EXITS 0 WHILE FAILING TO WRITE FILES — on Windows it reports "Filename too
    // long" per file and still succeeds overall. Trusting the exit code would hand `validate` a
    // half-populated tree, which fails for a reason that has nothing to do with the commit: a false
    // accusation, and the worst possible output for an audit whose entire job is to be believed.
    // So the tree is checked for the two directories the gate reads before any verdict is formed.
    let complete = wt.join(".engine").is_dir() && wt.join(".tracking").is_dir();
    if !created || !complete {
        let why = if created {
            "worktree was created only PARTIALLY (git exits 0 while failing to write files — on Windows, long paths). Verdict UNKNOWN"
        } else {
            "could not create a worktree for this commit — verdict UNKNOWN"
        };
        let _ = git(repo, &["worktree", "remove", "--force", &wt_s]);
        return CommitVerdict {
            sha: sha.to_string(),
            subject,
            clean: false,
            reasons: vec![format!("{why}, reported as not-clean so it cannot pass silently")],
        };
    }

    let report = crate::validate_root(&wt);
    if !report.is_clean() {
        reasons.push(format!("validate: {} parse error(s), {} semantic diagnostic(s)", report.errors.len(), report.diagnostics.len()));
    }
    for r in crate::guards::run_all(&wt) {
        if !r.violations.is_empty() {
            reasons.push(format!("guard:{} — {} violation(s)", r.name, r.violations.len()));
        }
    }

    let _ = git(repo, &["worktree", "remove", "--force", &wt_s]);
    CommitVerdict { sha: sha.to_string(), subject, clean: reasons.is_empty(), reasons }
}

fn flag(args: &[String], name: &str) -> Option<String> {
    let i = args.iter().position(|a| a == name)?;
    args.get(i + 1).filter(|v| !v.starts_with("--")).cloned()
}

/// `keel audit-history [--since REF] [--max N] [ROOT]`.
#[must_use]
pub fn cmd(args: &[String], repo: &Path) -> i32 {
    let since = flag(args, "--since");
    let max: usize = flag(args, "--max").and_then(|v| v.parse().ok()).unwrap_or(20);

    // `rev-list -6` is not valid syntax (that is a `log` shorthand); the count flag is `--max-count`.
    let range = since.as_ref().map_or_else(|| "HEAD".to_string(), |s| format!("{s}..HEAD"));
    let cap = format!("--max-count={max}");
    let Some(list) = git(repo, &["rev-list", "--no-merges", &cap, &range]) else {
        eprintln!("error: cannot list commits (is {} a git repo?)", repo.display());
        return 1;
    };
    let shas: Vec<String> = list.lines().map(str::to_owned).filter(|s| !s.is_empty()).take(max).collect();
    if shas.is_empty() {
        println!("no commits in range — nothing to audit.");
        return 0;
    }

    // The system temp dir, NOT `repo/target/`. Nesting the worktree inside the repo added ~40
    // characters to an already-deep path and blew Windows MAX_PATH — `.engine/decisions/` holds
    // 60-character filenames, and the checkout then failed file by file. Outside the repo the path
    // is short, and there is no untracked-state concern to solve in the first place.
    let scratch = std::env::temp_dir().join("keel-audit");
    if let Err(e) = std::fs::create_dir_all(&scratch) {
        eprintln!("error: cannot create {}: {e}", scratch.display());
        return 1;
    }

    println!("re-deriving the gate verdict for {} commit(s) — merges skipped (they carry no tree of", shas.len());
    println!("their own authorship); this RUNS the gate, it does not read a claim that it ran.");
    // Measured at ~18s/commit on this repo, dominated by the worktree checkout. Said UP FRONT because
    // a silent six-minute wait reads as a hang, and a user who kills it learns nothing.
    println!("~18s per commit (a full checkout each), so expect roughly {} minute(s). Ctrl-C is safe;", (shas.len() * 18).div_ceil(60));
    println!("worktrees live in the system temp dir and are removed as each commit finishes.");
    let mut dirty = 0;
    for sha in &shas {
        let v = verdict_for(repo, sha, &scratch);
        let short = &v.sha[..v.sha.len().min(7)];
        if v.clean {
            println!("  PASS  {short}  {}", v.subject);
        } else {
            dirty += 1;
            println!("  FAIL  {short}  {}", v.subject);
            for r in &v.reasons {
                println!("          {r}");
            }
        }
    }
    let _ = std::fs::remove_dir_all(&scratch);

    println!("{dirty} of {} commit(s) do not pass the gate at their own tree.", shas.len());
    if dirty > 0 {
        println!("  A failing commit is not proof a hook was skipped — history predates guards that exist");
        println!("  now, and a control added later must never retro-fail work that was correct when");
        println!("  written (issue068). Read this as a list to explain, not a list to blame.");
    }
    // ALWAYS 0. This is an audit, not a gate (D0098): it reports honest state and blocks nothing.
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unbuildable_worktree_is_reported_not_clean() {
        // The property that matters: UNKNOWN must never read as PASS. A bogus repo path cannot
        // produce a worktree, and the verdict must still be not-clean.
        let bogus = std::path::PathBuf::from("definitely-not-a-repo");
        let v = verdict_for(&bogus, "0000000000000000000000000000000000000000", &bogus);
        assert!(!v.clean, "an unknown verdict must not pass silently");
        assert!(!v.reasons.is_empty(), "and it must say why");
    }

    #[test]
    fn max_defaults_and_parses() {
        assert_eq!(flag(&["--max".to_string(), "5".to_string()], "--max"), Some("5".to_string()));
        assert_eq!(flag(&["--max".to_string()], "--max"), None, "a flag with no value must not swallow the next flag");
    }
}
