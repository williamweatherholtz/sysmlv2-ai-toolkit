//! `keel sync` and `keel land` — integrate with the remote without ever rewriting history (D0129).
//!
//! # Why the loop is client-side
//!
//! Two contributions that each pass the gate alone can fail together, which is the whole reason
//! merge queues exist. There is no server here, so the queue is executed by each contributor: fetch,
//! merge the trunk, run the FULL enforced gate against the RESULTING tree, push, and repeat on
//! rejection. Gating the merged tree rather than the isolated contribution is the property that
//! matters; gating your own branch in isolation proves nothing about what will land.
//!
//! # Never rebase, never force — this one is existential (issue071)
//!
//! A passing `TestResult` counts as done only while its `judgedAgainst` SHA still RESOLVES. Rebasing
//! rewrites commit identity, so evidence already recorded points at commits that were never pushed:
//! the rewriting machine still finds them as loose objects and reports GREEN, while every other
//! clone reports `invalidEvidence`, re-lists finished work as ready, and a second contributor redoes
//! it. Computed state becomes a function of WHICH CLONE ran the query, which negates the engine's
//! central claim. A merge keeps the original commits reachable, so every anchor stays resolvable
//! everywhere. There is no flag in this module that can force a push, by construction.
//!
//! # One implementation, not two
//!
//! `.githooks/post-commit` implemented this loop in shell first. Leaving it there and adding a Rust
//! copy would be two mechanisms for one fact — the dual-truth defect this engine exists to remove —
//! so the hook now DELEGATES here, and the shell path survives only as a degradation for a machine
//! with no `keel` binary. Same reasoning as D0134 moving the in-loop gates into the binary.

use std::path::Path;
use std::process::Command;

/// How this clone stands relative to its upstream.
pub struct Divergence {
    pub branch: String,
    /// Commits on the remote that this clone does not have.
    pub behind: usize,
    /// Commits here that the remote does not have.
    pub ahead: usize,
    /// No upstream configured, or the remote is unreachable — reported, never assumed to be zero.
    pub unknown: Option<String>,
}

impl Divergence {
    #[must_use]
    pub const fn diverged(&self) -> bool {
        self.behind > 0 && self.ahead > 0
    }
    /// A compact JSON object for `orient`.
    ///
    /// ALWAYS emitted, including the unknown case. "I could not tell" and "you are in sync" are
    /// different answers, and a computed view that renders them identically is the silent-failure
    /// shape this project keeps paying for (issue093, issue096).
    #[must_use]
    pub fn to_json(&self) -> String {
        self.unknown.as_ref().map_or_else(
            || format!(
                "{{\"branch\":\"{}\",\"behind\":{},\"ahead\":{},\"diverged\":{}}}",
                self.branch.replace('"', "\\\""),
                self.behind,
                self.ahead,
                self.diverged()
            ),
            |u| format!("{{\"branch\":\"{}\",\"unknown\":\"{}\"}}", self.branch.replace('"', "\\\""), u.replace('"', "\\\"")),
        )
    }
}

fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git").arg("-C").arg(repo).args(args).output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

/// Read behind/ahead WITHOUT contacting the remote — i.e. against the last fetch.
///
/// Separated from fetching on purpose: `orient` must be able to report the sync state on every
/// invocation, and a view that silently performs network I/O is a view you stop running. `keel sync`
/// fetches first and then calls this; `orient` calls it alone and reports what the last fetch knew.
#[must_use]
pub fn divergence(repo: &Path) -> Divergence {
    let branch = git(repo, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_else(|_| "HEAD".to_string());
    // `@{u}` is git's upstream shorthand; the braces are git syntax, not a format placeholder.
    let upstream_ref = concat!("@", "{u}");
    let Ok(upstream) = git(repo, &["rev-parse", "--abbrev-ref", "--symbolic-full-name", upstream_ref]) else {
        return Divergence { branch, behind: 0, ahead: 0, unknown: Some("no upstream configured for this branch".to_string()) };
    };
    match git(repo, &["rev-list", "--left-right", "--count", &format!("{upstream}...HEAD")]) {
        Ok(counts) => {
            let mut it = counts.split_whitespace();
            let behind = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let ahead = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            Divergence { branch, behind, ahead, unknown: None }
        }
        Err(e) => Divergence { branch, behind: 0, ahead: 0, unknown: Some(e) },
    }
}

/// Does the full enforced gate pass against the tree as it stands right now?
///
/// Deliberately the SAME entry point the commit hook uses, rather than a parallel definition: the
/// point of gating the merged tree is that it is held to the identical standard, and two gate
/// definitions that can drift apart would quietly reintroduce the problem.
fn gate_passes(repo: &Path) -> Result<(), Vec<String>> {
    let mut problems = Vec::new();
    let report = crate::validate_root(repo);
    for (p, d) in &report.diagnostics {
        problems.push(format!("{}:{} — {}", p.display(), d.line, d.message));
    }
    for e in &report.errors {
        problems.push(format!("{} — {}", e.file.display(), e.message));
    }
    for g in crate::guards::run_all(repo) {
        for v in &g.violations {
            problems.push(format!("[{}] {v}", g.name));
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}

/// `keel sync [ROOT]` — fetch, report divergence, integrate ancestry-preservingly, then orient.
#[must_use]
pub fn cmd_sync(repo: &Path) -> i32 {
    let before = divergence(repo);
    println!("keel sync — branch {}", before.branch);
    match git(repo, &["fetch", "origin"]) {
        Ok(_) => {}
        Err(e) => {
            println!("  fetch FAILED (offline?): {e}");
            println!("  reporting the sync state as of the last successful fetch, and saying so:");
        }
    }
    let d = divergence(repo);
    if let Some(u) = &d.unknown {
        println!("  sync state UNKNOWN: {u}");
        println!("  (unknown is not the same as in-sync, and is reported as itself.)");
        return 1;
    }
    println!("  behind {} · ahead {}{}", d.behind, d.ahead, if d.diverged() { " · DIVERGED" } else { "" });
    if d.behind == 0 {
        println!("  nothing to integrate.");
    } else {
        // Ancestry-preserving integrate. `git merge` is used rather than `git pull`, and that choice
        // IS the safety property: `pull` honours a contributor's `pull.rebase=true` and would then
        // silently orphan every evidence anchor in the repo (issue071). There is no `--no-rebase` on
        // `merge` — it is a `pull` flag, and passing it here made `land` fail on its first real merge.
        println!("  integrating {} commit(s) by MERGE (never rebase — issue071)", d.behind);
        match git(repo, &["merge", "--no-edit", &format!("origin/{}", d.branch)]) {
            Ok(out) => println!("  {}", out.lines().next().unwrap_or("merged")),
            Err(e) => {
                eprintln!("keel sync: merge could not complete automatically:\n{e}");
                eprintln!("  Resolve keeping BOTH facts, then `keel validate . && keel guard .` and commit.");
                eprintln!("  For two conclusions that cannot both be true, record an Issue for HUMAN adjudication (D0108) —");
                eprintln!("  never settle it by precedence. NEVER `git pull --rebase` here: it orphans evidence anchors.");
                return 1;
            }
        }
    }
    // Gate parity: the merged tree is held to the same standard everyone else is.
    match gate_passes(repo) {
        Ok(()) => println!("  gate: PASSES on the integrated tree"),
        Err(problems) => {
            eprintln!("keel sync: the integrated tree does NOT pass the gate ({} problem(s)):", problems.len());
            for p in problems.iter().take(10) {
                eprintln!("    {p}");
            }
            eprintln!("  Both sides may have been green alone. Fix before landing — that is what this check is for.");
            return 1;
        }
    }
    // Now that a fetch has actually happened, an anchor that still does not resolve is genuinely
    // dangling rather than merely unfetched. This is the ONLY place that distinction can be drawn
    // (issue113): `orient` never fetches, so from there the honest answer is always "unverifiable
    // from here".
    let o = crate::orient::compute_after_fetch(repo, true);
    if o.invalid_evidence.is_empty() {
        println!("  evidence: every anchor resolves.");
    } else {
        println!("  evidence: {} task(s) anchored to a commit NOBODY has — genuinely dangling, not unfetched:", o.invalid_evidence.len());
        for t in o.invalid_evidence.iter().take(10) {
            println!("      {t}");
        }
        println!("      These re-enter the frontier as outstanding: a passing result whose anchor is gone is not evidence (issue071).");
    }
    println!();
    println!("  now run `keel orient .` — its answers are computed against this tree.");
    0
}

/// `keel land [ROOT]` — push; on rejection integrate and retry, bounded. Never rewrites history.
#[must_use]
// @audit-hash ceLandGate
pub fn cmd_land(repo: &Path, max_attempts: u32) -> i32 {
    let branch = git(repo, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_else(|_| "HEAD".to_string());
    for attempt in 1..=max_attempts {
        println!("keel land: pushing {branch} -> origin (attempt {attempt}/{max_attempts})");
        if git(repo, &["push", "origin", &branch]).is_ok() {
            println!("keel land: landed.");
            return 0;
        }
        println!("  rejected — another contributor landed first. Integrating (merge, never rebase).");
        if let Err(e) = git(repo, &["fetch", "origin", &branch]) {
            eprintln!("keel land: fetch failed (offline?): {e}");
            eprintln!("  Run `keel sync` then `keel land` when able. Nothing was rewritten.");
            return 1;
        }
        if let Err(e) = git(repo, &["merge", "--no-edit", &format!("origin/{branch}")]) {
            eprintln!("keel land: MERGE COULD NOT BE COMPLETED AUTOMATICALLY.");
            eprintln!("  Either a textual conflict needs resolving, or the gate refused the merged tree");
            eprintln!("  (a semantic conflict: both sides green alone, red together).");
            eprintln!("  DO: resolve keeping BOTH facts, run `keel validate . && keel guard .`, commit, and re-run.");
            eprintln!("  For two conclusions that cannot both be true, record an Issue for HUMAN adjudication (D0108).");
            eprintln!("  NEVER `git pull --rebase` here: it orphans evidence anchors (issue071).\n{e}");
            let _ = git(repo, &["merge", "--abort"]);
            return 1;
        }
        // THE POINT OF THE WHOLE LOOP: gate the tree that will actually LAND, not the contribution
        // in isolation. Both sides were green alone or neither would have been committed; only the
        // merged tree can show that they are red together. Relying on `.githooks/pre-merge-commit`
        // for this was wrong — a contributor who has not run `git config core.hooksPath .githooks`
        // has no hook, and `land` would push a broken trunk while reporting success.
        if let Err(problems) = gate_passes(repo) {
            eprintln!("keel land: the MERGED tree does not pass the gate ({} problem(s)):", problems.len());
            for p in problems.iter().take(10) {
                eprintln!("    {p}");
            }
            eprintln!("  Both sides were green ALONE. This is the semantic conflict a merge queue exists to catch,");
            eprintln!("  and it is caught here rather than on the trunk. The merge is left in the working tree:");
            eprintln!("  resolve keeping BOTH facts, run `keel validate . && keel guard .`, commit, and re-run.");
            eprintln!("  Nothing was pushed and nothing was rewritten.");
            return 1;
        }
    }
    // Exhaustion is an honest failure, never an escalation. There is deliberately no force path.
    eprintln!("keel land: still rejected after {max_attempts} attempts — heavy contention. Backing off.");
    eprintln!("  Re-run shortly. Never force-push: it would orphan every evidence anchor behind it (issue071).");
    1
}

#[cfg(test)]
mod tests {
    use super::Divergence;

    #[test]
    fn divergence_reports_unknown_as_itself_not_as_in_sync() {
        // "I could not tell" and "you are in sync" are different answers. A view that renders them
        // identically is the silent-failure shape this project keeps paying for.
        let unknown = Divergence { branch: "main".into(), behind: 0, ahead: 0, unknown: Some("no upstream configured".into()) };
        let j = unknown.to_json();
        assert!(j.contains("\"unknown\""), "{j}");
        assert!(!j.contains("\"behind\":0"), "an unknown state must not render as zeros: {j}");

        let synced = Divergence { branch: "main".into(), behind: 0, ahead: 0, unknown: None };
        let j2 = synced.to_json();
        assert!(j2.contains("\"behind\":0") && j2.contains("\"ahead\":0"), "{j2}");
        assert!(!j2.contains("unknown"), "{j2}");
    }

    #[test]
    fn diverged_means_both_directions() {
        let d = |b, a| Divergence { branch: "main".into(), behind: b, ahead: a, unknown: None };
        assert!(!d(0, 0).diverged());
        assert!(!d(3, 0).diverged(), "only behind is not diverged — a fast-forward integrates cleanly");
        assert!(!d(0, 3).diverged(), "only ahead is not diverged — that is just unpushed work");
        assert!(d(2, 3).diverged(), "both directions is the case that needs a merge");
        assert!(d(2, 3).to_json().contains("\"diverged\":true"));
    }
}
