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
/// Anchors on the `verification …RetroGate… : Test` declaration (not any `RetroGate` substring,
/// which can appear in other gates' prose) — mirrors `_RETRO_TEXT`.
fn retro_scan_missing(text: &str, passed: &HashSet<&'static str>) -> bool {
    if !passed.contains("Retro") {
        return false;
    }
    for (idx, _) in text.match_indices("verification ") {
        let after = &text[idx + "verification ".len()..];
        let name: String = after.chars().take_while(|c| crate::algo::is_word(*c)).collect();
        if !name.contains("RetroGate") {
            continue;
        }
        let Some(rest) = after.strip_prefix(name.as_str()) else { return false };
        let Some(pt) = rest.find("procedureText") else { return false };
        let after_pt = &rest[pt..];
        let Some(q) = after_pt.find('"') else { return false };
        let body: String = after_pt[q + 1..].chars().take_while(|c| *c != '"').collect();
        let b = body.to_lowercase();
        return !SCAN_EVIDENCE.iter().any(|k| b.contains(k));
    }
    false // no retro verification declaration found
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

// ── process-change keystone guard (D0070 hard lock) ────────────────────────────────────────────

fn is_sysml(p: &str) -> bool {
    std::path::Path::new(p).extension().is_some_and(|e| e == "sysml")
}

fn is_process_def(p: &str) -> bool {
    is_sysml(p) && (p.starts_with(".engine/processes/") || p.starts_with(".engine/workflows/"))
}

fn is_decision_file(p: &str) -> bool {
    is_sysml(p) && p.starts_with(".engine/decisions/")
}

/// True if a line-anchored `#ProspectiveChange`/`#SafetyChange` marker is present (prose mentions
/// inside string literals start with `:>>`/`//`, so they never match). Mirrors `_MARKER`.
fn has_process_marker(text: &str) -> bool {
    text.lines().any(|line| {
        let t = line.trim_start_matches(is_space);
        ["#ProspectiveChange", "#SafetyChange"]
            .iter()
            .any(|kw| t.strip_prefix(kw).is_some_and(|rest| rest.chars().next().is_none_or(|c| !crate::algo::is_word(c))))
    })
}

/// Pure core: a staged process-def change must be co-committed with a marked Decision.
fn keystone_violations(changed: &[String], decision_texts: &[(String, String)]) -> Vec<String> {
    let mut procdefs: Vec<&str> = changed.iter().map(String::as_str).filter(|p| is_process_def(p)).collect();
    procdefs.sort_unstable();
    if procdefs.is_empty() {
        return Vec::new(); // no process-def changed — guard is silent
    }
    let marked = decision_texts.iter().any(|(p, t)| is_decision_file(p) && has_process_marker(t));
    if marked {
        return Vec::new();
    }
    vec![format!(
        "process-def file(s) changed ({}) with NO co-committed process-change Decision (a #ProspectiveChange/#SafetyChange-marked .engine/decisions/*.sysml). D0070 hard lock: every process-def change — typos included — must record a process-change Decision.",
        procdefs.join(", ")
    )]
}

fn staged_files(root: &Path) -> Vec<String> {
    git_stdout(root, &["diff", "--cached", "--name-only", "--diff-filter=ACMR"])
        .lines()
        .map(|l| l.trim().replace('\\', "/"))
        .filter(|l| !l.is_empty())
        .collect()
}

/// Guard: a staged process-def change must carry a co-committed marked Decision (D0070).
///
/// A staged change to `.engine/processes|workflows/*.sysml` MUST be co-committed with a
/// `#ProspectiveChange`/`#SafetyChange`-marked Decision (the keystone hard lock). Mirrors
/// `validate_process_change.py`.
#[must_use]
pub fn process_change(root: &Path) -> GuardReport {
    let changed = staged_files(root);
    let decision_texts: Vec<(String, String)> = changed
        .iter()
        .filter(|p| is_decision_file(p))
        .map(|p| (p.clone(), git_stdout(root, &["show", &format!(":{p}")])))
        .collect();
    let violations = keystone_violations(&changed, &decision_texts);
    let scanned = changed.iter().filter(|p| is_process_def(p)).count();
    GuardReport { name: "process-change", scanned, warnings: Vec::new(), violations }
}

/// Guard: every Issue carries a `#Resolves` edge (D0077).
///
/// An untriaged issue (no resolver) is a violation — it has no resolving work/Decision and can
/// never compute as resolved. Enforcement (hook wiring + inclusion in the `guard all` set) is
/// turned on once IRL-d backfill triages the existing issues; until then the guard is runnable
/// but not gating.
#[must_use]
pub fn issues(root: &Path) -> GuardReport {
    match crate::view::untriaged_issues(root) {
        Ok((total, untriaged)) => {
            let violations = untriaged
                .into_iter()
                .map(|i| format!("{i}: untriaged — no #Resolves edge (D0077; link a resolving action or Decision)"))
                .collect();
            GuardReport { name: "issues", scanned: total, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport { name: "issues", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading issues: {e}")] },
    }
}

/// Guard: every assurance element carries its required-lens critiques (D0080/D0079).
///
/// An element missing a required-lens critique (Core-3 policy) is a violation. RUNNABLE via
/// `sysmlv2 guard critique` but NOT yet in the enforced `GUARD_NAMES` set: with zero critiques
/// recorded it would block every commit. It joins the enforced set (a hard pre-commit gate, the
/// human-accepted choice) once a genuine critique pass brings the model to required-lens coverage.
#[must_use]
pub fn critique(root: &Path) -> GuardReport {
    match crate::view::critique_gaps(root) {
        Ok(gaps) => {
            let violations = gaps
                .into_iter()
                .map(|e| format!("{e}: missing a required-lens critique (D0080 Core-3; run the element-critique skill)"))
                .collect();
            GuardReport { name: "critique", scanned: 0, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport { name: "critique", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading critique coverage: {e}")] },
    }
}

/// The ENFORCED forward guards, in CLI/runner order.
///
/// `issues` joined the enforced set at IRL-d (D0077) once the existing issues were triaged with
/// `#Resolves` edges. `critique` (D0080) is runnable via `run_one` but joins the enforced set only
/// after a genuine critique pass (else it would block all commits with zero critiques recorded).
pub const GUARD_NAMES: [&str; 7] = ["actors", "acceptance-events", "sprint-coverage", "ceremony", "charter", "process-change", "issues"];

/// Run a single guard by name, or `None` if the name is unknown.
#[must_use]
pub fn run_one(name: &str, root: &Path) -> Option<GuardReport> {
    match name {
        "actors" => Some(actors(root)),
        "acceptance-events" => Some(acceptance_events(root)),
        "sprint-coverage" => Some(sprint_coverage(root)),
        "ceremony" => Some(ceremony(root)),
        "charter" => Some(charter(root)),
        "process-change" => Some(process_change(root)),
        "issues" => Some(issues(root)),
        "critique" => Some(critique(root)),
        _ => None,
    }
}

/// Run all six guards over `root`, returning their reports in `GUARD_NAMES` order.
#[must_use]
pub fn run_all(root: &Path) -> Vec<GuardReport> {
    GUARD_NAMES.iter().filter_map(|n| run_one(n, root)).collect()
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

        // Regression: "RetroGate" mentioned in an EARLIER gate's prose must not be mistaken for
        // the retro verification (the bug the unified runner caught on sprint56).
        let prose_then_real = "verification xStandupGate : Test { :>> procedureText = \"approach: retro_scan_missing (RetroGate prose)\"; }\nverification xRetroGate : Test { :>> procedureText = \"no avoidable issue\"; }";
        assert!(!retro_scan_missing(prose_then_real, &passed));
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

    #[test]
    fn keystone_selftest() {
        // Mirrors validate_process_change.selftest (incl. prose-marker-does-not-count).
        let marked = "package D {\n    #ProspectiveChange part d99 : Decision { :>> id = \"x\"; }\n}";
        let plain = "package D {\n    part d98 : Decision { :>> id = \"y\"; }\n}";
        let prose = "package D {\n    part d97 : Decision {\n        :>> decision = \"example: #ProspectiveChange part dNNNN : Decision { ... }\";\n    }\n}";

        let pos = keystone_violations(
            &[".engine/workflows/delivery.sysml".to_string(), ".engine/decisions/0099-x.sysml".to_string()],
            &[(".engine/decisions/0099-x.sysml".to_string(), marked.to_string())],
        );
        let neg = keystone_violations(
            &[".engine/processes/agile-workflow.sysml".to_string(), ".engine/decisions/0098-y.sysml".to_string()],
            &[(".engine/decisions/0098-y.sysml".to_string(), plain.to_string())],
        );
        let neg2 = keystone_violations(&[".engine/processes/agile-workflow.sysml".to_string()], &[]);
        let neutral = keystone_violations(&[".tracking/backlog.sysml".to_string()], &[]);
        let prose_only = keystone_violations(
            &[".engine/workflows/delivery.sysml".to_string(), ".engine/decisions/0097-z.sysml".to_string()],
            &[(".engine/decisions/0097-z.sysml".to_string(), prose.to_string())],
        );

        assert!(pos.is_empty(), "marked Decision co-committed -> pass");
        assert_eq!(neg.len(), 1, "unmarked Decision -> fail");
        assert_eq!(neg2.len(), 1, "no Decision -> fail");
        assert!(neutral.is_empty(), "no process-def -> silent");
        assert_eq!(prose_only.len(), 1, "prose marker does NOT count");
    }
}
