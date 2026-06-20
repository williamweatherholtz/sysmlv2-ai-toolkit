//! Forward process-discipline GUARDS ported from `.engine/tools/validate/*.py` (D0074 M3).
//!
//! Each guard scans authored facts and returns a [`GuardReport`] (violations → non-zero exit).
//! Parity with the python guards is by VERDICT (pass/fail) + violation SET, not byte-identical
//! report text. M3a ports the three no-git guards: `actors`, `acceptance-events`,
//! `sprint-coverage`. M3b/M3c add ceremony/charter/keystone + a unified runner.

use std::collections::HashSet;
use std::path::Path;

use crate::algo::is_space;

/// A guard's outcome: scanned count, tolerated warnings, and blocking violations.
pub struct GuardReport {
    pub name: &'static str,
    pub scanned: usize,
    pub warnings: Vec<String>,
    pub violations: Vec<String>,
}

impl GuardReport {
    /// True when there are no blocking violations.
    #[must_use]
    pub const fn ok(&self) -> bool {
        self.violations.is_empty()
    }

    /// Print the human report (warnings, then violations, then a summary line).
    pub fn print(&self) {
        for w in &self.warnings {
            println!("  WARN  {w}");
        }
        for v in &self.violations {
            println!("  ERROR {v}");
        }
        println!(
            "[guard:{}] {} — {} scanned, {} warning(s), {} violation(s)",
            self.name,
            if self.ok() { "PASS" } else { "FAIL" },
            self.scanned,
            self.warnings.len(),
            self.violations.len()
        );
    }
}

fn relpath(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

// ── actors guard (authoredBy/createdBy/judgedBy reference a known ProjectActor) ────────────────

/// Pre-convention actor values (2026-06-10/11) + tool names used as judgedBy before the actor
/// convention; reported as WARN, not a violation. Mirrors `validate_actors.LEGACY_ACTORS`.
const LEGACY_ACTORS: &[&str] = &[
    "user", "demo", "inspect", "claudeOpus", "_test_suspect",
    "validate_schema", "validate_workflows", "validate_instances",
    "validate_tracking", "validate_all", "whats_next",
];

const ACTOR_ATTRS: &[&str] = &["authoredBy", "createdBy", "judgedBy"];

fn load_known_actors(root: &Path) -> HashSet<String> {
    let mut known = HashSet::new();
    let Ok(text) = std::fs::read_to_string(root.join(".tracking").join("actors.sysml")) else {
        return known;
    };
    for line in text.lines() {
        // ^\s*part\s+(\w+)\s*:\s*(?:Person|Actor)\b
        let t = line.trim_start_matches(is_space);
        let Some(after) = t.strip_prefix("part") else { continue };
        let after_ws = after.trim_start_matches(is_space);
        if after_ws.len() == after.len() {
            continue;
        }
        let ident: String = after_ws.chars().take_while(|c| crate::algo::is_word(*c)).collect();
        if ident.is_empty() {
            continue;
        }
        let Some(r) = after_ws.strip_prefix(ident.as_str()) else { continue };
        let r = r.trim_start_matches(is_space);
        let Some(r) = r.strip_prefix(':') else { continue };
        let r = r.trim_start_matches(is_space);
        let is_actor = ["Person", "Actor"].iter().any(|kw| {
            r.strip_prefix(kw).is_some_and(|tail| tail.chars().next().is_none_or(|c| !crate::algo::is_word(c)))
        });
        if is_actor {
            known.insert(ident);
        }
    }
    known
}

/// Values of `:>> authoredBy|createdBy|judgedBy = "..."` on a line.
fn scan_actor_refs(line: &str) -> Vec<String> {
    let mut vals = Vec::new();
    for chunk in line.split(":>>").skip(1) {
        let c = chunk.trim_start_matches(is_space);
        for attr in ACTOR_ATTRS {
            if let Some(rest) = c.strip_prefix(attr) {
                let rest = rest.trim_start_matches(is_space);
                if let Some(rest) = rest.strip_prefix('=') {
                    let rest = rest.trim_start_matches(is_space);
                    if let Some(rest) = rest.strip_prefix('"') {
                        let val: String = rest.chars().take_while(|c| *c != '"').collect();
                        if !val.is_empty() {
                            vals.push(val);
                        }
                    }
                }
                break; // the chunk started with this attr name; don't test the others
            }
        }
    }
    vals
}

/// Guard: every `authoredBy`/`createdBy`/`judgedBy` value references a known `ProjectActor`
/// (or a tolerated legacy actor). Mirrors `validate_actors.py`.
#[must_use]
pub fn actors(root: &Path) -> GuardReport {
    let known = load_known_actors(root);
    let legacy: HashSet<&str> = LEGACY_ACTORS.iter().copied().collect();
    let mut warnings = Vec::new();
    let mut violations = Vec::new();
    let files = crate::collect_sysml(&root.join(".tracking"));
    let scanned = files.len();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let rel = relpath(root, path);
        for (i, line) in text.lines().enumerate() {
            for val in scan_actor_refs(line) {
                if known.contains(&val) {
                    continue;
                }
                if legacy.contains(val.as_str()) {
                    warnings.push(format!("{rel}:{}: legacy actor \"{val}\" (pre-convention)", i + 1));
                } else {
                    violations.push(format!("{rel}:{}: unknown actor \"{val}\" not in ProjectActors", i + 1));
                }
            }
        }
    }
    GuardReport { name: "actors", scanned, warnings, violations }
}

// ── acceptance-events guard (accepted Decision has a passing acceptance event) ─────────────────

/// Guard: every `status=accepted` Decision carries a passing `dNNNNAcceptR1` event (D0066).
/// Reuses `view::attestation_data` (the enforcement twin of attestation-coverage).
#[must_use]
pub fn acceptance_events(root: &Path) -> GuardReport {
    match crate::view::attestation_data(root) {
        Ok((total, mut missing)) => {
            missing.sort();
            let violations = missing
                .into_iter()
                .map(|d| format!("{d}: accepted but no passing acceptance event (D0066)"))
                .collect();
            GuardReport { name: "acceptance-events", scanned: total, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport {
            name: "acceptance-events",
            scanned: 0,
            warnings: Vec::new(),
            violations: vec![format!("error reading decisions: {e}")],
        },
    }
}

// ── sprint-coverage guard (done work is covered by a sprint) ────────────────────────────────────

/// Done tasks predating the sprint discipline (D0064); accepted as historical (never extend).
const GRANDFATHERED: &[&str] = &["ceremonyGateGuard", "rustS8runtimeParser", "rustS9writeApi", "trackedMetadataReplan"];

/// `<task>` from a `part <task>DoDR<n> : TestResult { ...pass }` part name.
fn strip_dodr(name: &str) -> Option<String> {
    let pos = name.find("DoDR")?;
    let after = &name[pos + "DoDR".len()..];
    if after.is_empty() || !after.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let task = &name[..pos];
    if task.is_empty() {
        None
    } else {
        Some(task.to_string())
    }
}

/// Done tasks declared in the backlog: `part <task>DoDR<n> : TestResult { ...VerdictKind::pass }`.
fn done_tasks(backlog: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    for (idx, _) in backlog.match_indices("part ") {
        let after = &backlog[idx + "part ".len()..];
        let name: String = after.chars().take_while(|c| crate::algo::is_word(*c)).collect();
        if let Some(task) = strip_dodr(&name) {
            let stmt_end = backlog[idx..].find('}').map_or(backlog.len(), |e| idx + e);
            let stmt = &backlog[idx..stmt_end];
            if stmt.contains(": TestResult") && stmt.contains("VerdictKind::pass") {
                out.insert(task);
            }
        }
    }
    out
}

fn delivery_blob(root: &Path) -> String {
    crate::collect_sysml(&root.join(".tracking").join("delivery"))
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Guard: every done backlog task is covered by a sprint (its name appears in a delivery file)
/// or is grandfathered. Mirrors `validate_sprint_coverage.py` (D0064/issue020).
#[must_use]
pub fn sprint_coverage(root: &Path) -> GuardReport {
    let backlog = std::fs::read_to_string(root.join(".tracking").join("backlog.sysml")).unwrap_or_default();
    let done = done_tasks(&backlog);
    let blob = delivery_blob(root);
    let grandfathered: HashSet<&str> = GRANDFATHERED.iter().copied().collect();
    let mut uncovered: Vec<String> = done
        .iter()
        .filter(|t| !blob.contains(t.as_str()) && !grandfathered.contains(t.as_str()))
        .cloned()
        .collect();
    uncovered.sort();
    let violations = uncovered
        .into_iter()
        .map(|t| format!("{t}: done but not covered by any sprint (D0064/issue020)"))
        .collect();
    GuardReport { name: "sprint-coverage", scanned: done.len(), warnings: Vec::new(), violations }
}

// ── ceremony guard (gate ordering + retro-scan evidence) ───────────────────────────────────────

const GATE_ORDER: [&str; 6] = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];
const CEREMONY_GRANDFATHERED: &[&str] = &["sprint11_nativeSpikes"];
const SCAN_EVIDENCE: &[&str] = &["avoidable", "improvement", "retro held", "no avoidable", "process improvement"];

/// Gate names with a `verification <…{G}Gate>` declaration in the text.
fn gates_defined(text: &str) -> HashSet<&'static str> {
    let mut out = HashSet::new();
    for (idx, _) in text.match_indices("verification ") {
        let after = &text[idx + "verification ".len()..];
        let name: String = after.chars().take_while(|c| crate::algo::is_word(*c)).collect();
        for g in GATE_ORDER {
            if name.ends_with(&format!("{g}Gate")) {
                out.insert(g);
            }
        }
    }
    out
}

/// Gate names with a passing `part <…{G}Gate…R\d+> : TestResult` (reuses `orient::gate_passed`).
fn gates_passed(text: &str) -> HashSet<&'static str> {
    GATE_ORDER.into_iter().filter(|g| crate::orient::gate_passed(text, g)).collect()
}

/// Ordering violations: a passed gate while an earlier DEFINED gate is unpassed.
fn ordering_violations(defined: &HashSet<&'static str>, passed: &HashSet<&'static str>) -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    for (i, g) in GATE_ORDER.into_iter().enumerate() {
        if !passed.contains(g) {
            continue;
        }
        for earlier in GATE_ORDER.into_iter().take(i) {
            if defined.contains(earlier) && !passed.contains(earlier) {
                out.push((g, earlier));
            }
        }
    }
    out
}

/// True if Retro passed but its gate text records no avoidable-issue scan evidence (issue011).
fn retro_scan_missing(text: &str, passed: &HashSet<&'static str>) -> bool {
    if !passed.contains("Retro") {
        return false;
    }
    let Some(pos) = text.find("RetroGate") else { return false };
    let after = &text[pos..];
    let Some(pt) = after.find("procedureText") else { return false };
    let after_pt = &after[pt..];
    let Some(q) = after_pt.find('"') else { return false };
    let body: String = after_pt[q + 1..].chars().take_while(|c| *c != '"').collect();
    let b = body.to_lowercase();
    !SCAN_EVIDENCE.iter().any(|k| b.contains(k))
}

/// Guard: within a delivery file, no ceremony gate passes while an earlier DEFINED gate is
/// unpassed; a passing Retro records avoidable-issue scan evidence. Mirrors `validate_ceremony.py`.
#[must_use]
pub fn ceremony(root: &Path) -> GuardReport {
    let files = crate::collect_sysml(&root.join(".tracking").join("delivery"));
    let mut warnings = Vec::new();
    let mut violations = Vec::new();
    let grandfathered: HashSet<&str> = CEREMONY_GRANDFATHERED.iter().copied().collect();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let passed = gates_passed(&text);
        let mut defined = gates_defined(&text);
        defined.extend(passed.iter().copied());
        let viols = ordering_violations(&defined, &passed);
        if !viols.is_empty() {
            let detail = viols.iter().map(|(g, e)| format!("{g} passed but {e} (earlier) unpassed")).collect::<Vec<_>>().join("; ");
            if grandfathered.contains(stem.as_str()) {
                warnings.push(format!("{stem}: {detail} (grandfathered, pre-issue010)"));
            } else {
                violations.push(format!("{stem}: {detail}"));
            }
        }
        if retro_scan_missing(&text, &passed) && !grandfathered.contains(stem.as_str()) {
            violations.push(format!("{stem}: Retro gate recorded without avoidable-issue scan evidence (issue011)"));
        }
    }
    GuardReport { name: "ceremony", scanned: files.len(), warnings, violations }
}

// ── charter guard (newly-added delivery Story declares its #CharteredBy edge) ───────────────────

fn git_stdout(root: &Path, args: &[&str]) -> String {
    std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

fn added_delivery_files(root: &Path) -> Vec<String> {
    git_stdout(root, &["diff", "--cached", "--name-only", "--diff-filter=A"])
        .lines()
        .map(|l| l.trim().replace('\\', "/"))
        .filter(|l| l.starts_with(".tracking/delivery/") && std::path::Path::new(l).extension().is_some_and(|e| e == "sysml"))
        .collect()
}

/// `#CharteredBy dependency from <work> to <_>` → the chartered work names.
fn chartered_work(text: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    for chunk in text.split("#CharteredBy").skip(1) {
        let mut it = chunk.split_whitespace();
        if it.next() != Some("dependency") || it.next() != Some("from") {
            continue;
        }
        let Some(tok) = it.next() else { continue };
        let work: String = tok.chars().take_while(|c| crate::algo::is_word(*c)).collect();
        if !work.is_empty() && it.next() == Some("to") {
            out.insert(work);
        }
    }
    out
}

/// Pure core: violations for newly-added delivery files `(path, staged_text)`.
fn charter_violations(added: &[(String, String)]) -> Vec<String> {
    let mut sorted: Vec<&(String, String)> = added.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut violations = Vec::new();
    for (path, text) in sorted {
        let chartered = chartered_work(text);
        let mut uncharted: Vec<String> = crate::algo::story_names(text).into_iter().filter(|s| !chartered.contains(s)).collect();
        uncharted.sort();
        uncharted.dedup();
        for s in uncharted {
            violations.push(format!(
                "{path}: Story '{s}' has no #CharteredBy edge — a delivery Story must charter to its originating Decision/Need/Requirement (D0068)"
            ));
        }
    }
    violations
}

/// Guard: every newly-added delivery Story declares its `#CharteredBy` edge (D0068).
///
/// Newly-added = `git diff --cached --diff-filter=A`. Forward-only — existing files are never
/// re-checked. Mirrors `validate_charter.py`.
#[must_use]
pub fn charter(root: &Path) -> GuardReport {
    let added = added_delivery_files(root);
    let texts: Vec<(String, String)> = added
        .iter()
        .map(|p| (p.clone(), git_stdout(root, &["show", &format!(":{p}")])))
        .collect();
    let violations = charter_violations(&texts);
    GuardReport { name: "charter", scanned: added.len(), warnings: Vec::new(), violations }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_refs_extracted() {
        let line = "    part x { :>> authoredBy = \"ana\"; :>> judgedBy = \"bob\"; :>> title = \"z\"; }";
        assert_eq!(scan_actor_refs(line), vec!["ana".to_string(), "bob".to_string()]);
    }

    #[test]
    fn dodr_task_stripped() {
        assert_eq!(strip_dodr("portOrphansAuditDoDR1"), Some("portOrphansAudit".to_string()));
        assert_eq!(strip_dodr("portOrphansAuditDoDR12"), Some("portOrphansAudit".to_string()));
        assert_eq!(strip_dodr("fooRefineGateR1"), None);
        assert_eq!(strip_dodr("DoDR1"), None);
    }

    #[test]
    fn sprint_coverage_selftest() {
        // Mirrors validate_sprint_coverage.selftest: covered passes, orphan flagged.
        let backlog = "action fakeCovered;\npart fakeCoveredDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }\naction fakeOrphan;\npart fakeOrphanDoDR1 : TestResult { :>> outcome = VerdictKind::pass; }\n";
        let done = done_tasks(backlog);
        assert!(done.contains("fakeCovered") && done.contains("fakeOrphan"));
        let blob = "package ProjectDeliveryX { part s : Story { :>> title = \"delivers fakeCovered\"; } }";
        let grandfathered: HashSet<&str> = HashSet::new();
        let uncovered: Vec<String> = done.iter().filter(|t| !blob.contains(t.as_str()) && !grandfathered.contains(t.as_str())).cloned().collect();
        assert_eq!(uncovered, vec!["fakeOrphan".to_string()]);
    }

    #[test]
    fn ceremony_ordering_violation_detected() {
        // Implement passed while Standup (defined) is unpassed -> violation.
        let mut defined: HashSet<&'static str> = HashSet::new();
        defined.extend(["Refine", "Standup", "Implement"]);
        let mut passed: HashSet<&'static str> = HashSet::new();
        passed.extend(["Refine", "Implement"]); // Standup skipped
        let v = ordering_violations(&defined, &passed);
        assert_eq!(v, vec![("Implement", "Standup")]);
    }

    #[test]
    fn retro_scan_evidence_required() {
        let mut passed: HashSet<&'static str> = HashSet::new();
        passed.insert("Retro");
        let with = "verification xRetroGate : Test { :>> procedureText = \"no avoidable issue found\"; }";
        let without = "verification xRetroGate : Test { :>> procedureText = \"rubber stamp\"; }";
        assert!(!retro_scan_missing(with, &passed));
        assert!(retro_scan_missing(without, &passed));
    }

    #[test]
    fn charter_selftest() {
        // Mirrors validate_charter.selftest: chartered passes, uncharted flagged, none passes.
        let good = "package S {\n    part s42 : Story { :>> id = \"x\"; }\n    #CharteredBy dependency from s42 to d0070;\n}";
        let bad = "package S {\n    part s99 : Story { :>> id = \"y\"; }\n}";
        assert!(charter_violations(&[(".tracking/delivery/g.sysml".to_string(), good.to_string())]).is_empty());
        let neg = charter_violations(&[(".tracking/delivery/b.sysml".to_string(), bad.to_string())]);
        assert_eq!(neg.len(), 1);
        assert!(neg[0].contains("s99"));
        assert!(charter_violations(&[]).is_empty());
    }
}
