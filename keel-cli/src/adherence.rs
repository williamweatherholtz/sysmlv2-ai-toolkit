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

/// `camelCase` -> `kebab-case`, matching `activation::camel_to_kebab`: a process asserts a guard by
/// its constraint name (`staleGateProse`) and the guard is named `stale-gate-prose`.
fn camel_to_kebab(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// The `[processes] active = [...]` list from a commit's `activation.toml`, plus whether the section
/// was declared at all. An ABSENT file means everything is active (D0138/issue090), so the two cases
/// must stay distinguishable — treating absent as "nothing active" would report every guard as
/// disarmed on any project that never adopted a manifest.
fn active_processes(repo: &Path, sha: &str) -> (Vec<String>, bool) {
    let Some(text) = git(repo, &["show", &format!("{sha}:.engine/contracts/activation.toml")]) else {
        return (Vec::new(), false);
    };
    let mut in_processes = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_processes = t == "[processes]";
            continue;
        }
        if in_processes && t.starts_with("active") {
            if let Some(list) = t.split_once('[').and_then(|(_, r)| r.rsplit_once(']')).map(|(l, _)| l) {
                let names = list
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                return (names, true);
            }
        }
    }
    (Vec::new(), false)
}

/// Which guard each process CLAIMS at this commit: `assert constraint <x> : <guardName>;` inside a
/// `.engine/processes/*.sysml` part/action. Claiming is what converts a guard from CORE
/// (never-deactivatable) into that process's switchable property — the issue242 capture.
fn guard_claims(repo: &Path, sha: &str) -> BTreeMap<String, String> {
    let mut claims = BTreeMap::new();
    let files = git(repo, &["ls-tree", "-r", "--name-only", sha, ".engine/processes/"]).unwrap_or_default();
    for f in files.lines().filter(|f| Path::new(f).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml"))) {
        let Some(text) = git(repo, &["show", &format!("{sha}:{f}")]) else { continue };
        let proc_name = Path::new(f).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        for line in text.lines() {
            let t = line.trim();
            if !t.starts_with("assert constraint") {
                continue;
            }
            // `assert constraint <local> : <GuardName>;`
            if let Some((_, tail)) = t.split_once(':') {
                let g = tail.trim().trim_end_matches(';').trim();
                if !g.is_empty() && g.chars().all(|c| c.is_ascii_alphanumeric()) {
                    claims.insert(camel_to_kebab(g), proc_name.clone());
                }
            }
        }
    }
    claims
}

/// Effective guard state rank at a commit (issue242). CORE binds hardest because it cannot be
/// switched off at all; ACTIVE enforces but is switchable; INACTIVE does not enforce.
///
/// This is the dimension `audit-adherence` was missing. The D0208 control evaluation proved a rule
/// severity could be flipped, which clause 1 closed — but two other routes disarm a control without
/// touching `.engine/rules/` at all: (a) adding one `assert constraint` line to a guard-less process
/// CAPTURES a core guard and makes it that process's switchable property (verified live: core
/// `stale-gate-prose` became `[INACTIVE] report` and stopped running), and (b) narrowing
/// `activation.toml`'s active list disarms every guard the removed process owns. Both are covered by
/// the commit keystone, but clause 1 exists precisely because a bypassed hook leaves no trace at
/// commit time — and the hook WAS bypassed here (issue240).
const fn state_rank(state: &GuardEffective) -> u8 {
    match state {
        GuardEffective::Core => 3,
        GuardEffective::Active => 2,
        GuardEffective::Inactive => 1,
    }
}

enum GuardEffective {
    Core,
    Active,
    Inactive,
}

/// The enforcement SIGNATURE at one commit: rule/constraint name -> severity rank. Reads the rule
/// files and the guard-constraint declarations from THAT commit's tree via `git show` (no checkout —
/// cheap enough to gate on). A `constraint def` with no severity is a guard binding: present = rank 3
/// (its removal is a weakening exactly as a severity drop is).
fn signature(repo: &Path, sha: &str) -> BTreeMap<String, u8> {
    let mut sig = BTreeMap::new();
    let mut declared_guards: Vec<String> = Vec::new();
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
                    declared_guards.push(name.to_string());
                }
            }
        }
    }
    // issue242: the EFFECTIVE state of each declared guard, so a capture (Core -> Active) or a
    // deactivation (Active -> Inactive) ranks as a weakening. Without this the signature saw only
    // `.engine/rules/`, and both routes to disarming a control were invisible to the gate built for
    // exactly that class.
    let (active, declared_manifest) = active_processes(repo, sha);
    let claims = guard_claims(repo, sha);
    for g in declared_guards {
        let kebab = camel_to_kebab(&g);
        // Unclaimed by any process => CORE: no activation switch reaches it. Claimed => Active
        // unless a DECLARED manifest omits the claiming process; with no manifest everything is
        // active (D0138), so absent must never read as off.
        let state = claims.get(&kebab).map_or(GuardEffective::Core, |p| {
            if !declared_manifest || active.iter().any(|a| a == p) {
                GuardEffective::Active
            } else {
                GuardEffective::Inactive
            }
        });
        sig.insert(format!("guardstate:{kebab}"), state_rank(&state));
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
        3 => "blocking/bound/core",
        2 => "warning/active",
        0 => "absent",
        _ => "present/inactive",
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

    #[test]
    fn core_binds_harder_than_active_which_binds_harder_than_inactive() {
        // issue242: the two disarm routes are Core -> Active/Inactive (a process CAPTURES a core
        // guard) and Active -> Inactive (activation.toml narrows). Both must rank as weakenings.
        assert!(state_rank(&GuardEffective::Core) > state_rank(&GuardEffective::Active));
        assert!(state_rank(&GuardEffective::Active) > state_rank(&GuardEffective::Inactive));
        assert!(state_rank(&GuardEffective::Inactive) > 0, "inactive is still DECLARED - absence is worse");
    }

    #[test]
    fn camel_to_kebab_matches_the_activation_convention() {
        assert_eq!(camel_to_kebab("staleGateProse"), "stale-gate-prose");
        assert_eq!(camel_to_kebab("docSync"), "doc-sync");
        assert_eq!(camel_to_kebab("actors"), "actors");
    }
}
