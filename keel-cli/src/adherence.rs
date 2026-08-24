//! `keel audit-adherence [--since REF]` (D0209 clause 1, the process-adherence audit).
//!
//! The sound floor the manager-inversion panel endorsed unanimously: re-derive from the git tree
//! ALONE — on CI infra the agent cannot touch — that the AGENT FOLLOWED THE PROCESS, not merely that
//! the current state is honest. `audit-history` already re-runs the full gate per commit (gate-of-
//! record); this adds the one thing it does not check: **guard-set / rule-severity monotonicity**.
//!
//! The control evaluation (issue236) proved an agent could weaken a blocking rule to a warning. The
//! commit-time keystone now requires a signed Decision for `.engine/rules/` edits — but a bypassed
//! hook leaves no trace at commit time, so this audit re-derives it independently over the whole
//! pushed range: across every commit, a rule's severity must never WEAKEN and a declared rule/
//! guard-constraint must never VANISH unless that same commit co-commits a marked Decision (the
//! keystone, re-checked from the tree). Unlike `audit-history`, this one is a GATE: it exits non-zero
//! on an unsigned weakening, because a silently-disarmed control is not honest state, it is a lie the
//! tree can prove (D0209).

use std::collections::BTreeMap;
use std::path::Path;

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = crate::gitx::git().arg("-C").arg(repo).args(args).output().ok()?;
    if out.status.success() { Some(String::from_utf8_lossy(&out.stdout).into_owned()) } else { None }
}

/// Severity rank: higher binds harder. A DROP between commits is a weakening.
fn rank(sev: &str) -> u8 {
    match sev {
        "blocking" | "block" => 3,
        "warning" | "warn" => 2,
        _ => 1, // any other declared severity — still present, just not ranked above
    }
}

/// The enforcement SIGNATURE at one commit: rule/constraint name -> severity rank. Reads the rule
/// files and the guard-constraint declarations from THAT commit's tree via `git show` (no checkout —
/// cheap enough to gate on). A `constraint def` with no severity is a guard binding: present = rank 3
/// (its removal is a weakening exactly as a severity drop is).
fn signature(repo: &Path, sha: &str) -> BTreeMap<String, u8> {
    let mut sig = BTreeMap::new();
    // rule files: `part <name> : ElementRule|EdgeRule { ... :>> severity = RuleSeverity::<sev>; }`
    let files = git(repo, &["ls-tree", "-r", "--name-only", sha, ".engine/rules/"]).unwrap_or_default();
    for f in files.lines().filter(|f| std::path::Path::new(f).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml"))) {
        let Some(text) = git(repo, &["show", &format!("{sha}:{f}")]) else { continue };
        let mut current: Option<String> = None;
        for line in text.lines() {
            let t = line.trim_start().trim_start_matches('#').trim_start();
            if let Some(rest) = t.strip_prefix("part ") {
                if let Some((name, tail)) = rest.split_once(':') {
                    if tail.contains("ElementRule") || tail.contains("EdgeRule") {
                        current = Some(name.trim().to_string());
                    }
                }
            }
            if let Some(name) = &current {
                if let Some(i) = line.find("RuleSeverity::") {
                    let sev: String = line[i + "RuleSeverity::".len()..]
                        .chars()
                        .take_while(char::is_ascii_alphabetic)
                        .collect();
                    sig.insert(format!("rule:{name}"), rank(&sev));
                    current = None;
                }
            }
            // a guard-constraint binding: `constraint def <name>;` — present is rank 3.
            if let Some(rest) = t.strip_prefix("constraint def ") {
                let name = rest.split([';', ' ', '/']).next().unwrap_or("").trim();
                if !name.is_empty() {
                    sig.insert(format!("guard:{name}"), 3);
                }
            }
        }
    }
    sig
}

/// Did this commit co-commit a MARKED Decision (the keystone, re-derived from the tree)?
fn commit_is_signed_change(repo: &Path, sha: &str) -> bool {
    let touched = git(repo, &["show", "--name-only", "--format=", sha]).unwrap_or_default();
    touched
        .lines()
        .filter(|f| f.starts_with(".engine/decisions/") && std::path::Path::new(f).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml")))
        .any(|f| {
            git(repo, &["show", &format!("{sha}:{f}")])
                .is_some_and(|t| t.lines().any(|l| {
                    let s = l.trim_start();
                    s.starts_with("#ProspectiveChange") || s.starts_with("#SafetyChange")
                }))
        })
}

/// `keel audit-adherence [--since REF] [--max N]` — the guard-set monotonicity gate.
///
/// # Returns
/// Exit code: 0 if the enforcement signature never weakened unsigned across the range; 1 on any
/// unsigned weakening (this is a GATE, not a report — that is the D0209 difference from audit-history).
pub fn cmd(args: &[String], repo: &Path) -> i32 {
    let since = args.iter().position(|a| a == "--since").and_then(|i| args.get(i + 1)).cloned();
    let max: usize = args
        .iter()
        .position(|a| a == "--max")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(200);
    let range = since.as_ref().map_or_else(|| "HEAD".to_string(), |s| format!("{s}..HEAD"));
    // OLDEST-first so each commit is compared against its predecessor in the range.
    let Some(list) = git(repo, &["rev-list", "--reverse", "--no-merges", &format!("--max-count={max}"), &range])
    else {
        eprintln!("audit-adherence: cannot list commits (is {} a git repo?)", repo.display());
        return 1;
    };
    let shas: Vec<String> = list.lines().map(str::to_owned).filter(|s| !s.is_empty()).collect();
    if shas.len() < 2 {
        println!("audit-adherence: fewer than 2 commits in range — nothing to compare. Clean.");
        return 0;
    }
    println!("audit-adherence: re-deriving the enforcement signature across {} commit(s) from the tree", shas.len());
    println!("  (guard-set + rule severities; a weakening needs a co-committed signed Decision — D0209).");
    let Some(first) = shas.first() else { return 0 };
    let mut prev = signature(repo, first);
    let mut violations = 0u32;
    for w in shas.windows(2) {
        let [a, b] = w else { continue };
        let cur = signature(repo, b);
        let mut weakened: Vec<String> = Vec::new();
        for (name, &old_rank) in &prev {
            let new_rank = cur.get(name).copied().unwrap_or(0); // 0 = vanished
            if new_rank < old_rank {
                weakened.push(format!(
                    "{name}: {} -> {}",
                    rank_name(old_rank),
                    rank_name(new_rank)
                ));
            }
        }
        if !weakened.is_empty() && !commit_is_signed_change(repo, b) {
            violations += 1;
            let short = b.get(..7).unwrap_or(b.as_str());
            println!("  UNSIGNED WEAKENING at {short} (parent {}):", a.get(..7).unwrap_or(a.as_str()));
            for wk in &weakened {
                println!("      {wk}");
            }
            println!("      no co-committed #ProspectiveChange/#SafetyChange Decision — the keystone was bypassed (issue236 class).");
        }
        prev = cur;
    }
    if violations == 0 {
        println!("audit-adherence: PASS — the enforcement signature never weakened unsigned across the range.");
        0
    } else {
        println!("audit-adherence: FAIL — {violations} unsigned weakening(s). A control was disarmed with no signed Decision (D0209).");
        1
    }
}

const fn rank_name(r: u8) -> &'static str {
    match r {
        3 => "blocking/bound",
        2 => "warning",
        0 => "absent",
        _ => "present",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_orders_blocking_above_warning_above_absent() {
        assert!(rank("blocking") > rank("warning"));
        assert!(rank("block") > rank("warn"));
        assert_eq!(rank("blocking"), rank("block"));
    }

    #[test]
    fn a_severity_drop_is_a_weakening_and_absent_is_the_lowest() {
        // The exact issue236 shape: blocking(3) -> warning(2) is a weakening; vanished(0) is worse.
        assert!(rank("warning") < rank("blocking"));
        assert!(0 < rank("warning"));
    }
}
