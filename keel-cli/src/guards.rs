//! Forward process-discipline GUARDS ported from `.engine/tools/validate/*.py` (D0074 M3).
//!
//! Each guard scans authored facts and returns a [`GuardReport`] (violations → non-zero exit).
//! Parity with the python guards is by VERDICT (pass/fail) + violation SET, not byte-identical
//! report text. M3a ports the three no-git guards: `actors`, `acceptance-events`,
//! `sprint-coverage`. M3b/M3c add ceremony/charter/keystone + a unified runner.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

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

/// Guard: an accepted Decision's acceptance event must be HUMAN-judged (D0106/issue059).
///
/// The enforceable slice of strict process-boundedness (a sign-off is never AI-fabricated). Rule-sourced
/// from `confirmationAuthenticityRule` (the CONTRACT pattern). D0106's conversational parse-first part is
/// inherently un-gatable at commit and stays reminder-enforced.
#[must_use]
pub fn confirmation_authenticity(root: &Path) -> GuardReport {
    match crate::view::rule_violations_opt(root, "confirmationAuthenticityRule") {
        Ok(Some((scanned, bad))) => {
            let violations = bad
                .into_iter()
                .map(|d| format!("{d}: accepted but its acceptance event is not human-judged — a sign-off must be a real human attestation, never AI-fabricated (D0106/D0016)"))
                .collect();
            GuardReport { name: "confirmation-authenticity", scanned, warnings: Vec::new(), violations }
        }
                // D0136/issue090: an ABSENT rule means the project has not ADOPTED this control —
        // it has not violated it. Warn (never silent, so deleting a rule to dodge the gate is
        // visible) and pass; a MALFORMED rule still fails via Err below.
        Ok(None) => GuardReport { name: "confirmation-authenticity", scanned: 0, warnings: vec!["declared rule `confirmationAuthenticityRule` is not present — this control is NOT ADOPTED by this project, so nothing was checked (D0136/issue090)".to_string()], violations: Vec::new() },
Err(e) => GuardReport { name: "confirmation-authenticity", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading confirmation-authenticity rule: {e}")] },
    }
}

/// Guard: every `status=accepted` Decision carries a passing `dNNNNAcceptR1` event (D0066).
#[must_use]
pub fn acceptance_events(root: &Path) -> GuardReport {
    // CONTRACT (D0107): sourced from the declared acceptanceEventRule (single gate source).
    match crate::view::rule_violations_opt(root, "acceptanceEventRule") {
        Ok(Some((total, mut missing))) => {
            missing.sort();
            let violations = missing
                .into_iter()
                .map(|d| format!("{d}: accepted but no passing acceptance event (D0066)"))
                .collect();
            GuardReport { name: "acceptance-events", scanned: total, warnings: Vec::new(), violations }
        }
                // D0136/issue090: an ABSENT rule means the project has not ADOPTED this control —
        // it has not violated it. Warn (never silent, so deleting a rule to dodge the gate is
        // visible) and pass; a MALFORMED rule still fails via Err below.
        Ok(None) => GuardReport { name: "acceptance-events", scanned: 0, warnings: vec!["declared rule `acceptanceEventRule` is not present — this control is NOT ADOPTED by this project, so nothing was checked (D0136/issue090)".to_string()], violations: Vec::new() },
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

/// Guard: every newly-added delivery Story declares its `#CharteredBy` edge (D0068).
///
/// CONTRACT (D0107): sourced from the declared `charterRule` (an `EdgeRule` with `newlyAdded` git scope)
/// — the single gate source; the bespoke charter predicate was retired after parity (sprints 178-183).
#[must_use]
pub fn charter(root: &Path) -> GuardReport {
    match crate::view::rule_violations_opt(root, "charterRule") {
        Ok(Some((scanned, uncharted))) => {
            let violations = uncharted
                .into_iter()
                .map(|s| format!("Story '{s}' has no #CharteredBy edge — a delivery Story must charter to its originating Decision/Need/Requirement, or (a research spike) to an Issue/proposed-Decision (D0068/issue055)"))
                .collect();
            GuardReport { name: "charter", scanned, warnings: Vec::new(), violations }
        }
                // D0136/issue090: an ABSENT rule means the project has not ADOPTED this control —
        // it has not violated it. Warn (never silent, so deleting a rule to dodge the gate is
        // visible) and pass; a MALFORMED rule still fails via Err below.
        Ok(None) => GuardReport { name: "charter", scanned: 0, warnings: vec!["declared rule `charterRule` is not present — this control is NOT ADOPTED by this project, so nothing was checked (D0136/issue090)".to_string()], violations: Vec::new() },
Err(e) => GuardReport { name: "charter", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading charter rule: {e}")] },
    }
}

/// Guard: requirement-rootedness (D0098/D0099, issue047).
///
/// A declared `#Capability` (a user-facing feature) must carry a `#DerivedFrom` edge to a Need. An
/// HONESTY gate: shipping a capability whose driving Need is unstated is a traceability lie-of-omission.
/// UNMARKED work is exempt — decision-driven engine evolution is legitimate (D0064), so this never
/// floods (it binds only what is opted-in via the marker). The full charter-source balance is the
/// non-blocking `keel rootedness` burndown.
#[must_use]
pub fn requirement_rootedness(root: &Path) -> GuardReport {
    // CONTRACT (D0107): sourced from the declared capabilityRootednessRule (single gate source).
    match crate::view::rule_violations_opt(root, "capabilityRootednessRule") {
        Ok(Some((_scanned, gaps))) => {
            let violations = gaps
                .into_iter()
                .map(|c| format!("{c}: #Capability with no #DerivedFrom edge to a Need — state the driving Need (D0099)"))
                .collect();
            GuardReport { name: "requirement-rootedness", scanned: 0, warnings: Vec::new(), violations }
        }
                // D0136/issue090: an ABSENT rule means the project has not ADOPTED this control —
        // it has not violated it. Warn (never silent, so deleting a rule to dodge the gate is
        // visible) and pass; a MALFORMED rule still fails via Err below.
        Ok(None) => GuardReport { name: "requirement-rootedness", scanned: 0, warnings: vec!["declared rule `capabilityRootednessRule` is not present — this control is NOT ADOPTED by this project, so nothing was checked (D0136/issue090)".to_string()], violations: Vec::new() },
Err(e) => GuardReport { name: "requirement-rootedness", scanned: 0, warnings: Vec::new(), violations: vec![format!("error computing rootedness: {e}")] },
    }
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

// ── doc-sync guard (D0113: the doc-sync discipline made a CONTROL — was pure vigilance) ────────────

/// A staged path whose change is DEFINITIONAL and near-always carries doc implications: `.engine`
/// schema / process / workflow definitions. (Tool `.rs` code + skills are deliberately EXCLUDED to keep
/// this low-noise; the scope can widen once proven, D0113.)
fn is_doc_governed_def(p: &str) -> bool {
    p.starts_with(".engine/schema/")
        || p.starts_with(".engine/processes/")
        || p.starts_with(".engine/workflows/")
}

/// A staged path that COUNTS as a doc update (satisfies doc-sync): `CLAUDE.md`, `.engine/docs/`, or any
/// `README.md`.
fn is_doc_file(p: &str) -> bool {
    p == "CLAUDE.md" || p.starts_with(".engine/docs/") || p.ends_with("README.md")
}

/// Pure core: a staged definitional change (schema/process/workflow) with NO co-committed doc update
/// yields one warning naming the offending files. Empty when nothing definitional changed OR a doc did.
fn doc_sync_warnings(changed: &[String]) -> Vec<String> {
    let mut governed: Vec<&str> = changed.iter().map(String::as_str).filter(|p| is_doc_governed_def(p)).collect();
    governed.sort_unstable();
    if governed.is_empty() || changed.iter().any(|p| is_doc_file(p)) {
        return Vec::new();
    }
    vec![format!(
        "definitional change ({}) with NO co-committed doc update (CLAUDE.md / .engine/docs / README) — run the doc-sync skill; fix any doc claim this change invalidates in THIS commit",
        governed.join(", ")
    )]
}

/// Guard (WARNING-level, D0113): a staged schema/process/workflow change co-committed with no doc update.
///
/// Converts the doc-sync discipline from pure vigilance into a visible control — documentation drift was
/// a recorded HIGH critique finding, and doc-sync had no enforcing guard (only the skill). Heuristic +
/// WARNING (the D0102 promote-once-low-noise pattern, like `decision-requirement-link`): a definitional
/// change MIGHT legitimately need no doc, so this NUDGES (never blocks); promote to hard once proven
/// low-noise. Shares the `staged_files` git mechanism with `process_change`.
#[must_use]
pub fn doc_sync(root: &Path) -> GuardReport {
    let changed = staged_files(root);
    let scanned = changed.iter().filter(|p| is_doc_governed_def(p)).count();
    GuardReport { name: "doc-sync", scanned, warnings: doc_sync_warnings(&changed), violations: Vec::new() }
}

/// Guard: every Issue carries a `#Resolves` edge (D0077).
///
/// An untriaged issue (no resolver) is a violation — it has no resolving work/Decision and can
/// never compute as resolved. Enforcement (hook wiring + inclusion in the `guard all` set) is
/// turned on once IRL-d backfill triages the existing issues; until then the guard is runnable
/// but not gating.
#[must_use]
pub fn issues(root: &Path) -> GuardReport {
    // CONTRACT (D0107): sourced from the declared issuesTriagedRule (single gate source), not a bespoke predicate.
    match crate::view::rule_violations_opt(root, "issuesTriagedRule") {
        Ok(Some((total, untriaged))) => {
            let violations = untriaged
                .into_iter()
                .map(|i| format!("{i}: untriaged — no #Resolves edge (D0077; link a resolving action or Decision)"))
                .collect();
            GuardReport { name: "issues", scanned: total, warnings: Vec::new(), violations }
        }
                // D0136/issue090: an ABSENT rule means the project has not ADOPTED this control —
        // it has not violated it. Warn (never silent, so deleting a rule to dodge the gate is
        // visible) and pass; a MALFORMED rule still fails via Err below.
        Ok(None) => GuardReport { name: "issues", scanned: 0, warnings: vec!["declared rule `issuesTriagedRule` is not present — this control is NOT ADOPTED by this project, so nothing was checked (D0136/issue090)".to_string()], violations: Vec::new() },
Err(e) => GuardReport { name: "issues", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading issues: {e}")] },
    }
}

/// Guard: every assurance element carries its required-lens critiques (D0080/D0079).
///
/// An element missing a required-lens critique (per the declared critique policy, D0097 — default
/// Core-3) is reported here. This is critique-COVERAGE — a COMPLETENESS measure, so under the honest-
/// state doctrine (D0098) it is NOT in the enforced `GUARD_NAMES`: it is a non-blocking burndown,
/// RUNNABLE via `keel guard critique` / `keel critique-coverage` and surfaced in orient, never a hard
/// commit gate (an un-critiqued element is honest incomplete state, not a lie). Critique INDEPENDENCE
/// (`critic-independence`) stays enforced — that is honesty, not completeness.
#[must_use]
pub fn critique(root: &Path) -> GuardReport {
    match crate::view::critique_gaps(root) {
        Ok(gaps) => {
            let violations = gaps
                .into_iter()
                .map(|e| format!("{e}: missing a required-lens critique (D0080/D0097 policy; run the element-critique skill)"))
                .collect();
            GuardReport { name: "critique", scanned: 0, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport { name: "critique", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading critique coverage: {e}")] },
    }
}

/// Guard (issue109): every typed-edge endpoint resolves to a declared item.
///
/// HARD, and it is an honest-state gate rather than a completeness one: a dangling edge does not
/// mean work is unfinished, it means the model asserts a relationship that is not there. `issue060`
/// read as triaged by a resolver declared in no commit; a delivery Story read as chartered by an
/// origin that never existed. Both survived every existing check, because `issues` and `charter`
/// each verify that the EDGE is present and neither resolves its endpoints.
///
/// Both were fixed before this guard was added, so it starts at zero — no grandfathering needed and
/// none granted (issue068 forbids retro-failing work that was correct when written; this work was
/// not correct when written, it was undetected).
#[must_use]
pub fn edge_endpoints(root: &Path) -> GuardReport {
    match crate::view::dangling_edge_endpoints(root) {
        Ok(bad) => {
            let violations = bad
                .into_iter()
                .map(|e| format!("{e} — a typed edge must connect two declared items; declare the item or remove the edge, never repoint it at something convenient"))
                .collect();
            GuardReport { name: "edge-endpoints", scanned: 0, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport { name: "edge-endpoints", scanned: 0, warnings: Vec::new(), violations: vec![format!("error resolving edge endpoints: {e}")] },
    }
}

/// Guard: the composite assurance-readiness gate (D0079 c).
///
/// Reports the exact blockers when the deliverable is not assured (coverage/critique gaps, stale
/// verification, undispositioned >= Medium findings, open Critical, invariant violations). This is the
/// SELF-ASSURANCE composite (completeness/readiness), so under the honest-state doctrine (D0098) it is
/// NOT in the enforced `GUARD_NAMES`: a NON-BLOCKING burndown verdict, RUNNABLE via `keel guard
/// assured` / `keel assured` and surfaced in orient, never a hard commit gate. Incompleteness flagged
/// AS incomplete is honest state; suppressing it or blocking on it both destroy the honest picture.
#[must_use]
pub fn assured(root: &Path) -> GuardReport {
    match crate::view::assured_blockers(root) {
        Ok(blockers) => GuardReport { name: "assured", scanned: 0, warnings: Vec::new(), violations: blockers },
        Err(e) => GuardReport { name: "assured", scanned: 0, warnings: Vec::new(), violations: vec![format!("error computing readiness: {e}")] },
    }
}

// ── viewpoint-renderer guard (every declared viewpoint names a real renderer) ──────────────────

/// View-ish `keel` subcommands a viewpoint renderer may legitimately name.
const VIEW_SUBCOMMANDS: &[&str] = &[
    "orient", "whats-next", "view", "diagram", "render", "report", "decisions", "suspect", "orphans",
    "attestation-coverage", "governing-version", "reprocess-candidates", "coverage", "critique-coverage",
    "assured", "open-issues", "audit", "validate", "guard", "indicators", "record-measurement",
    "concern-coverage", "dispositions", "sitting-coverage", "critique-policy", "rootedness", "tier-satisfaction", "recent",
];

/// The quoted value of `:>> {key} = "..."` on a line.
fn quoted_attr(line: &str, key: &str) -> Option<String> {
    let needle = format!(":>> {key} = \"");
    line.split(needle.as_str()).nth(1)?.split('"').next().map(str::to_string)
}

/// Classify a viewpoint renderer string: `"retired"` (query.py/report.py, a violation), `"planned"`
/// (a tolerated warning), `"ok"` (names a real `keel` subcommand), or `"unknown"` (a violation).
fn classify_renderer(r: &str) -> &'static str {
    if r.contains("query.py") || r.contains("report.py") {
        "retired"
    } else if r.starts_with("(planned") {
        "planned"
    } else if r.strip_prefix("keel ").and_then(|s| s.split([' ', '(']).next()).is_some_and(|c| VIEW_SUBCOMMANDS.contains(&c)) {
        "ok"
    } else {
        "unknown"
    }
}

/// Guard (D0056/issue034): every declared Viewpoint's renderer names a real current command
/// (a `keel <subcommand>`), or is explicitly `(planned ...)`.
///
/// A renderer referencing a RETIRED tool (query.py / report.py, D0074) or an unknown command is a
/// violation — it stops the viewpoint registry from drifting to dead renderers (the d0056 finding).
/// A `(planned ...)` renderer is a tolerated WARNING (a declared-but-unbuilt concern).
#[must_use]
pub fn viewpoint_renderer(root: &Path) -> GuardReport {
    let path = root.join(".engine").join("views").join("viewpoint-registry.sysml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return GuardReport { name: "viewpoint-renderer", scanned: 0, warnings: Vec::new(), violations: vec![format!("cannot read {}", relpath(root, &path))] };
    };
    let mut scanned = 0;
    let mut warnings = Vec::new();
    let mut violations = Vec::new();
    let mut title = String::new();
    for line in text.lines() {
        let t = line.trim_start();
        if let Some(v) = quoted_attr(t, "title") {
            title = v;
        } else if let Some(r) = quoted_attr(t, "renderer") {
            scanned += 1;
            match classify_renderer(&r) {
                "retired" => violations.push(format!("{title}: renderer references a RETIRED tool (query.py/report.py, D0074) — '{r}'")),
                "unknown" => violations.push(format!("{title}: renderer names no known keel command — '{r}'")),
                "planned" => warnings.push(format!("{title}: viewpoint declared but renderer is planned/unbuilt — '{r}'")),
                _ => {}
            }
        }
    }
    GuardReport { name: "viewpoint-renderer", scanned, warnings, violations }
}

// ── manifest-coverage guard (deliverable-suspicion manifest stays valid + complete) ────────────

/// Name fragments that mark a task as likely deliverable-source-dependent (a verification whose
/// evidence is the Rust deliverable behaving correctly) — used for the unlisted-task WARNING.
const DELIVERABLE_TASK_HINTS: &[&str] = &["rust", "Parser", "writeApi", "runtimeParser", "specVersion"];

/// All `action <name>;` task names declared in .tracking/{backlog,delivery} (not `action def`).
fn declared_task_names(root: &Path) -> HashSet<String> {
    let mut names = HashSet::new();
    for sub in ["backlog.sysml", "delivery"] {
        let base = root.join(".tracking").join(sub);
        let files = if base.is_dir() { crate::collect_sysml(&base) } else { vec![base] };
        for f in files {
            let Ok(text) = std::fs::read_to_string(&f) else { continue };
            for line in text.lines() {
                let t = line.trim_start();
                if let Some(rest) = t.strip_prefix("action ") {
                    if rest.starts_with("def ") {
                        continue;
                    }
                    let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if !name.is_empty() {
                        names.insert(name);
                    }
                }
            }
        }
    }
    names
}

/// Parse the deliverable manifest into `(task, paths)` entries (`task: NAME | p1 p2`; `#` comments).
fn parse_manifest(text: &str) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let Some(rest) = t.strip_prefix("task:") else { continue };
        let mut parts = rest.splitn(2, '|');
        let Some(name) = parts.next().map(str::trim) else { continue };
        let paths: Vec<String> = parts.next().unwrap_or("").split_whitespace().map(str::to_string).collect();
        if !name.is_empty() {
            out.push((name.to_string(), paths));
        }
    }
    out
}

/// Guard (D0050/issue033): the deliverable-suspicion manifest stays VALID + complete.
///
/// VIOLATION: a manifest entry names a task that no longer exists, or lists a path that no longer
/// exists (a dead entry silently drops deliverable-suspicion coverage — the d0050 finding).
/// WARNING: a declared task whose name looks deliverable-dependent but is not manifest-listed
/// (a possible unguarded verification — the manifest is a hand-maintained allow-list).
#[must_use]
pub fn manifest_coverage(root: &Path) -> GuardReport {
    let path = root.join(".engine").join("deliverable-manifest.txt");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return GuardReport { name: "manifest-coverage", scanned: 0, warnings: Vec::new(), violations: vec![format!("cannot read {}", relpath(root, &path))] };
    };
    let entries = parse_manifest(&text);
    let tasks = declared_task_names(root);
    let listed: HashSet<&str> = entries.iter().map(|(n, _)| n.as_str()).collect();
    let mut warnings = Vec::new();
    let mut violations = Vec::new();
    for (name, paths) in &entries {
        if !tasks.contains(name) {
            violations.push(format!("manifest entry '{name}' names a task that no longer exists (dead entry — deliverable-suspicion coverage silently lost)"));
        }
        for p in paths {
            if !root.join(p).exists() {
                violations.push(format!("manifest entry '{name}' lists path '{p}' which no longer exists"));
            }
        }
    }
    // Exclude sprint-wrapper actions (story*) — the manifest is about BACKLOG deliverable tasks, and
    // a "storyParser*" wrapper matching the "Parser" hint is a false positive, not a manifest gap.
    let mut unlisted: Vec<&String> = tasks
        .iter()
        .filter(|t| !t.starts_with("story") && !listed.contains(t.as_str()) && DELIVERABLE_TASK_HINTS.iter().any(|h| t.contains(h)))
        .collect();
    unlisted.sort();
    for t in unlisted {
        warnings.push(format!("task '{t}' looks deliverable-dependent (name) but is NOT in deliverable-manifest.txt — confirm it needs no source-drift suspicion"));
    }
    GuardReport { name: "manifest-coverage", scanned: entries.len(), warnings, violations }
}

/// Guard (D0080/issue031): a Critical-severity finding's target must carry a non-aiModel critic.
///
/// ENFORCED (vacuous until a Critical finding exists). aiModel-vs-aiModel critique shares blind spots,
/// so the highest-stakes elements require cognition-distinct (human/tool) independence.
#[must_use]
pub fn critic_independence(root: &Path) -> GuardReport {
    match crate::view::critical_independence_gaps(root) {
        Ok(gaps) => {
            let violations = gaps
                .into_iter()
                .map(|e| format!("{e}: target of a Critical-severity finding but has only aiModel critiques — requires a human/tool critic (D0080 independence, issue031)"))
                .collect();
            GuardReport { name: "critic-independence", scanned: 0, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport { name: "critic-independence", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading critique independence: {e}")] },
    }
}

/// Diagnostic (D0080/issue030): low-rigor critiques + affirming-only critics, as WARNINGS.
///
/// RUNNABLE via `keel guard critique-rigor` but NOT in the enforced `GUARD_NAMES` — rigor is a
/// heuristic signal for human attention, not a hard gate (a shallow-but-honest critique is not a
/// commit-blocker). Surfaces critiques lacking adversarial structure / substance and never-find critics.
#[must_use]
pub fn critique_rigor(root: &Path) -> GuardReport {
    match crate::view::critique_rigor(root) {
        Ok(findings) => GuardReport { name: "critique-rigor", scanned: findings.len(), warnings: findings, violations: Vec::new() },
        Err(e) => GuardReport { name: "critique-rigor", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading critique rigor: {e}")] },
    }
}

/// Guard (D0103): every Decision must carry a substantive `context` + `rationale` (the why).
///
/// Not just the schema-present (possibly blank) fields — a recorded decision without its why is ill-formed
/// state. HARD honest-state gate: a Decision whose `context` or `rationale` is blank/trivial (trimmed < 20
/// chars) is a violation. Precise (no false positives), and all current decisions pass — no flood.
#[must_use]
pub fn decision_rationale(root: &Path) -> GuardReport {
    // CONTRACT (D0107): sourced from the declared decisionRationaleRule (single gate source).
    match crate::view::rule_violations_opt(root, "decisionRationaleRule") {
        Ok(Some((total, weak))) => {
            let violations = weak
                .into_iter()
                .map(|d| format!("{d}: blank/trivial context or rationale (D0103 — a Decision must state a substantive why; >=20 chars each)"))
                .collect();
            GuardReport { name: "decision-rationale", scanned: total, warnings: Vec::new(), violations }
        }
                // D0136/issue090: an ABSENT rule means the project has not ADOPTED this control —
        // it has not violated it. Warn (never silent, so deleting a rule to dodge the gate is
        // visible) and pass; a MALFORMED rule still fails via Err below.
        Ok(None) => GuardReport { name: "decision-rationale", scanned: 0, warnings: vec!["declared rule `decisionRationaleRule` is not present — this control is NOT ADOPTED by this project, so nothing was checked (D0136/issue090)".to_string()], violations: Vec::new() },
Err(e) => GuardReport { name: "decision-rationale", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading decision rationale: {e}")] },
    }
}

/// Guard (D0102/issue052): an accepted Decision that names a Need/SystemRequirement in its prose but
/// carries NO typed edge to it — a governance/derivation link that should be typed, not prose.
///
/// WARNING-level: it RUNS in `GUARD_NAMES` (visible on every commit, not ignorable) but emits warnings,
/// never violations, so it does not block (D0102 — warning first; promotable to a hard gate once proven
/// low-noise by moving the warnings to violations). A compute error IS a violation.
#[must_use]
pub fn decision_requirement_link(root: &Path) -> GuardReport {
    match crate::view::decision_requirement_prose_links(root) {
        Ok(pairs) => {
            let warnings = pairs
                .iter()
                .map(|(d, r)| format!("{d} names {r} in prose but has no typed edge to it (D0102 — link via #DependsOn/#Supersede/#DerivedFrom/satisfy/derive)"))
                .collect();
            GuardReport { name: "decision-requirement-link", scanned: pairs.len(), warnings, violations: Vec::new() }
        }
        Err(e) => GuardReport { name: "decision-requirement-link", scanned: 0, warnings: Vec::new(), violations: vec![format!("error computing decision-requirement links: {e}")] },
    }
}

// ── marker-vocabulary guard (an undeclared/misspelled marker silently blinds a control) ───────────

/// Remove `SysML` string literals from a line, so markers QUOTED IN PROSE are not mistaken for edges.
///
/// Essential, not cosmetic: `procedureText` fields legitimately discuss markers (`#Marker dependency
/// from a to b`, `#Kind dependency`, `#Changes dependency`), and a naive scan reports each as an
/// undeclared marker. Those three alone would have produced 9 false violations on a hard guard.
fn strip_string_literals(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_str = false;
    for c in line.chars() {
        if c == '"' {
            in_str = !in_str;
            continue;
        }
        if !in_str {
            out.push(c);
        }
    }
    out
}

/// Syntactic positions in which a `#Marker` names a real edge or a marked item.
const MARKER_FOLLOWERS: [&str; 5] = ["dependency", "part", "item", "verification", "requirement"];

/// The ENGINE's own marker algebra — always valid, known to the binary (D0136 / issue089).
///
/// These are the markers the engine's OWN guards and views consume: `#Verify` drives
/// tier-satisfaction and verification-trace, `#DerivedFrom` drives the hard requirement-rootedness
/// guard, `#Resolves` drives issue triage, and so on. They are part of the engine's CONTRACT, so the
/// binary must know them intrinsically rather than requiring each project to re-declare them.
///
/// Why this exists: D0133 shipped `marker-vocabulary` as a HARD guard in the BINARY whose passing
/// condition was `metadata def` lines in the PROJECT's schema files. `include_dir!` embeds `.engine`
/// at build time, so a v0.2.0 binary carried the declarations — but an existing downstream project
/// keeps its own on-disk `.engine/schema/`, which upgrading the binary never touches. Reproduced: a
/// pre-v0.2.0 schema plus a v0.2.0 binary yields **566 violations and every commit blocked**, on the
/// engine's own shipped files. Worse, the obvious remedy meant editing FROZEN `schema/core`, so the
/// guard forced every downstream project into a frozen-schema sign-off just to keep committing.
pub const ENGINE_MARKERS: [&str; 17] = [
    // edge algebra where the pilot grammar has no native form
    "DependsOn",
    "Supersede",
    "OrderingOnly",
    "CharteredBy",
    "Resolves",
    "Verify",
    "DerivedFrom",
    "Measures",
    "Informs",
    "JustifiedBy",
    "Dispositions",
    "Covers",
    // item-level classifiers the guards read
    "ProspectiveChange",
    "SafetyChange",
    "Capability",
    "ProcessDefect",
    "View",
];

/// Marker names USED in real syntactic positions in `text`, as `(marker, 1-based line)`.
fn markers_used(text: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = strip_string_literals(raw);
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            continue; // a comment may legitimately name a marker
        }
        for (pos, _) in line.match_indices('#') {
            let rest = &line[pos + 1..];
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric()).collect();
            if name.is_empty() {
                continue;
            }
            let after = rest[name.len()..].trim_start();
            if MARKER_FOLLOWERS.iter().any(|f| after.starts_with(f)) {
                out.push((name, i + 1));
            }
        }
    }
    out
}

/// Marker names DECLARED as `metadata def <Name>;` in `texts`.
fn markers_declared(texts: &[String]) -> HashSet<String> {
    // The engine's own algebra is always valid — a project must never have to re-declare it (D0136).
    let mut out: HashSet<String> = ENGINE_MARKERS.iter().map(|m| (*m).to_string()).collect();
    for text in texts {
        for raw in text.lines() {
            let line = raw.trim();
            if line.starts_with("//") {
                continue;
            }
            if let Some(rest) = line.strip_prefix("metadata def ") {
                let name: String = rest.trim().chars().take_while(|c| c.is_alphanumeric()).collect();
                if !name.is_empty() {
                    out.insert(name);
                }
            }
        }
    }
    out
}

/// Guard: every metadata marker used must be DECLARED (D0133 / issue077).
///
/// Markers were never type-checked, so a MISSPELLED marker validated clean and silently removed that
/// item from the depending control's view — and a blind guard reports PASS, not a violation. The
/// exposure was concentrated: `#Verify` carries 456 edges and is what `tier-satisfaction`,
/// `sr_verified_pct` and the `verification-trace` guard all key on, so a single typo would report a
/// DELIVERED requirement as unverified. `#DerivedFrom` (37 edges) is load-bearing for the HARD
/// `requirement-rootedness` guard.
///
/// HARD-blocking: a typo that blinds a control makes the model's computed state a lie, which is
/// ill-formed STATE rather than incomplete work — squarely inside the honest-state gate (D0098).
/// Safe to make hard because the check is exact (a declared-name set membership), not heuristic.
#[must_use]
pub fn marker_vocabulary(root: &Path) -> GuardReport {
    // Project-declared markers may be declared ANYWHERE in .engine or .tracking (D0136): a downstream
    // project must be able to declare its OWN vocabulary in its OWN files, without being forced into a
    // frozen-schema (§2.5) change just to keep committing.
    let mut files = crate::collect_sysml(&root.join(".tracking"));
    files.extend(crate::collect_sysml(&root.join(".engine")));
    let declared_texts: Vec<String> = files.iter().filter_map(|p| std::fs::read_to_string(p).ok()).collect();
    let declared = markers_declared(&declared_texts);
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let rel = relpath(root, path);
        for (marker, line) in markers_used(&text) {
            scanned += 1;
            if !declared.contains(&marker) {
                violations.push(format!(
                    "{rel}:{line}: marker `#{marker}` is NOT declared as a `metadata def` — markers are not type-checked, so an undeclared or MISSPELLED marker validates clean and silently removes this item from whatever guard or view depends on it (issue077/D0133). If it is a TYPO, fix the spelling. If it is your project's own marker, declare `metadata def {marker};` in any of your own .engine or .tracking files — you do NOT need to touch frozen schema/core (D0136). The engine's own markers are always valid without declaration."
                ));
            }
        }
    }
    GuardReport { name: "marker-vocabulary", scanned, warnings: Vec::new(), violations }
}

/// Test-only re-exports of the pure marker scanners.
#[doc(hidden)]
#[must_use]
pub fn markers_used_for_test(text: &str) -> Vec<(String, usize)> {
    markers_used(text)
}
#[doc(hidden)]
#[must_use]
pub fn markers_declared_for_test(texts: &[String]) -> HashSet<String> {
    markers_declared(texts)
}

// ── retro-backlog guard (a retro finding that terminates in prose) ────────────────────────────────

/// Markers a retro uses to name something it learned.
const RETRO_FINDING_MARKERS: &[&str] = &["AVOIDABLE-ISSUE", "AVOIDABLE ISSUE", "LESSON:"];

/// Phrases by which a retro EXPLICITLY justifies raising no tracked item.
///
/// The obligation is not "always create an item" — sometimes a control already exists, and adding a
/// duplicate is noise. The obligation is that the choice is STATED rather than left silent, so a
/// reader can tell a considered decision from an omission.
const RETRO_NO_ITEM_JUSTIFICATIONS: &[&str] = &["no new item", "no item needed", "already tracked", "no further item"];

/// Warnings for staged sprint records whose retro names a finding with nothing tracked and no reason.
fn retro_backlog_warnings(changed: &[String], sprint_texts: &[(String, String)]) -> Vec<String> {
    let tracked_co_staged = changed
        .iter()
        .any(|p| p.ends_with(".tracking/issues.sysml") || p.ends_with(".tracking/backlog.sysml"));
    if tracked_co_staged {
        return Vec::new(); // a tracked item WAS co-recorded in this commit
    }
    let mut out = Vec::new();
    for (path, text) in sprint_texts {
        let upper = text.to_uppercase();
        if !RETRO_FINDING_MARKERS.iter().any(|m| upper.contains(m)) {
            continue;
        }
        let lower = text.to_lowercase();
        if RETRO_NO_ITEM_JUSTIFICATIONS.iter().any(|j| lower.contains(j)) {
            continue; // explicitly justified as needing no item
        }
        out.push(format!(
            "{path}: the retro names a finding (AVOIDABLE-ISSUE / LESSON) but this commit records NO tracked Issue or backlog action, and gives no reason — a retro finding must become a tracked, prioritized item or say explicitly why it needs none (issue085; D0018 — never let a lesson terminate in prose)"
        ));
    }
    out
}

/// Test-only re-export of the pure warning builder (the view self-tests exercise it).
#[doc(hidden)]
#[must_use]
pub fn retro_backlog_warnings_for_test(changed: &[String], sprint_texts: &[(String, String)]) -> Vec<String> {
    retro_backlog_warnings(changed, sprint_texts)
}

/// Guard: a sprint retro's findings must become tracked items, not prose (issue085 / D0130).
///
/// Sprint 247's retro named three avoidable issues; only one had a control, and the other two were
/// written into CLAUDE.md prose and the AI's own memory — OUTSIDE the model, carrying no severity, no
/// priority, no resolver and no id, invisible to orient and the burndown, inside the very ceremony
/// meant to prevent recurrence. That is the prose-shadow-truth D0018 forbids.
///
/// Git-diff-aware, heuristic and WARNING-level — the `doc-sync` (D0113) shape. It is satisfied either
/// by co-recording a tracked item or by SAYING why none is needed, so what it really enforces is that
/// the choice is explicit. Reads the working tree, which equals the index for staged-and-unmodified
/// files (the pre-commit case); a partially-staged sprint file could be misread, which is one more
/// reason this warns rather than blocks.
#[must_use]
pub fn retro_backlog(root: &Path) -> GuardReport {
    let changed = staged_files(root);
    let sprint_texts: Vec<(String, String)> = changed
        .iter()
        .filter(|p| p.contains(".tracking/delivery/sprint") && std::path::Path::new(p).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml")))
        .filter_map(|p| std::fs::read_to_string(root.join(p)).ok().map(|t| (p.clone(), t)))
        .collect();
    let scanned = sprint_texts.len();
    GuardReport { name: "retro-backlog", scanned, warnings: retro_backlog_warnings(&changed, &sprint_texts), violations: Vec::new() }
}

// ── priority-inversion guard (recorded order disagreeing with recorded severity) ──────────────────

/// Guard: a ready item outranks work that resolves a >= High Issue (issue084 / D0130).
///
/// D0052 makes backlog DECLARATION ORDER the priority and requires the AI to auto-follow the ranked
/// frontier, but nothing compared recorded ORDER against recorded SEVERITY — so a mis-ordered backlog
/// looked exactly like a curated one. It was mis-ordered: `keelArchViews` (issue069, Low) ranked FIRST
/// because an earlier session appended it to the end of a COMPLETED block, while
/// `dcStaleKernelInstanceGate` (issue081, High) ranked 14th.
///
/// WARNING-level and never blocking: priority IS a human judgment and deferring a High item behind an
/// enabler can be entirely correct. The point is to make the trade-off visible rather than leave it to
/// whoever last appended to the file. A compute error IS a violation.
#[must_use]
pub fn priority_inversion(root: &Path) -> GuardReport {
    match crate::view::priority_inversions(root) {
        Ok(pairs) => {
            let warnings = pairs
                .iter()
                .map(|(lower, high, sev)| {
                    format!("{lower} outranks {high}, which resolves a {sev} issue — if that is deliberate say so, otherwise reorder the backlog (D0052: declaration order IS priority; reordering is how you reprioritize)")
                })
                .collect();
            GuardReport { name: "priority-inversion", scanned: pairs.len(), warnings, violations: Vec::new() }
        }
        Err(e) => GuardReport { name: "priority-inversion", scanned: 0, warnings: Vec::new(), violations: vec![format!("error computing priority inversions: {e}")] },
    }
}

// ── attestation-substance guard (a confirmation that attests nothing) ─────────────────────────────

/// Attestations already thin when this guard landed (2026-08-13), grandfathered to WARNING.
///
/// FORWARD-ONLY per the issue068 lesson: a new guard must never retroactively fail items authored
/// under the process in force at the time. Nine of 234 `method=confirmation` verifications — seven bare
/// "accepted", one empty (`d0129Accept`), one bare actor name (`d0128Accept`). They warn
/// on every run so the debt stays visible; anything NEW is a hard violation. Do not extend this list:
/// a new contentless attestation is a defect to fix, not to grandfather.
const GRANDFATHERED_THIN_ATTESTATIONS: [&str; 9] = [
    "d0118Accept",
    "d0119Accept",
    "d0120Accept",
    "d0121Accept",
    "d0124Accept",
    "d0125Accept",
    "d0127Accept",
    "d0128Accept",
    "d0129Accept",
];

/// Guard: a passing `method=confirmation` must actually attest something (issue083 / D0130).
///
/// `d0129Accept` was authored with an EMPTY `procedureText` and passed every enforced guard — because
/// `acceptance-events` and `confirmation-authenticity` verify that an acceptance EXISTS and is
/// HUMAN-judged, never that it says anything. For a confirmation the attestation text IS the evidence
/// (D0016), so a contentless acceptance is an unsupported claim in the shape of a complete record, on
/// the record type that governs everything downstream.
///
/// HARD-blocking, matching `decision-rationale` (D0103), which applies the same substantive-field test
/// to a Decision's *why*: a contentless attestation is ill-formed STATE, not incomplete work, so it is
/// squarely inside the honest-state gate (D0098) rather than the burndown.
#[must_use]
pub fn attestation_substance(root: &Path) -> GuardReport {
    let grandfathered: HashSet<&str> = GRANDFATHERED_THIN_ATTESTATIONS.iter().copied().collect();
    match crate::view::thin_attestations(root) {
        Ok(found) => {
            let mut warnings = Vec::new();
            let mut violations = Vec::new();
            for (name, reason) in &found {
                if grandfathered.contains(name.as_str()) {
                    warnings.push(format!("{name}: {reason} — GRANDFATHERED (pre-issue083); state what was attested when this record is next touched"));
                } else {
                    violations.push(format!("{name}: {reason} — a confirmation records a HUMAN's word, so it must say what was attested and to what (D0016/issue083)"));
                }
            }
            GuardReport { name: "attestation-substance", scanned: found.len(), warnings, violations }
        }
        Err(e) => GuardReport { name: "attestation-substance", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading attestations: {e}")] },
    }
}

// ── verification-trace guard (delivered work whose requirement carries no verification) ───────────

/// Guard: a DELIVERED verification names a `SystemRequirement` in prose but never `#Verify`-links it.
///
/// Closes issue082 (D0130). Sprint 247 delivered six SRs with passing `DoD` `TestResult`s and CI green,
/// yet `tier-satisfaction` reported all six UNVERIFIED — because an SR is verified only when a Test
/// `#Verify`-links TO IT, and the `DoD` Tests linked to the backlog ACTION instead. So the model could
/// not distinguish *requirement not yet delivered* from *requirement delivered but its verification was
/// never traced upward*, and the AI then reported `sr_verified_pct` to the human as though it meant
/// functional verification. This makes that specific, previously-invisible state visible.
///
/// WARNING-level and non-blocking, on two independent grounds: completeness is honest-state burndown
/// that must never gate a commit (D0098), and prose-name matching is a heuristic, so it follows the
/// D0102 promote-once-low-noise pattern. A compute error IS a violation.
#[must_use]
pub fn verification_trace(root: &Path) -> GuardReport {
    match crate::view::untraced_verification_links(root) {
        Ok(pairs) => {
            let warnings = pairs
                .iter()
                .map(|(v, sr)| {
                    format!("{v} PASSED and names {sr} in its procedure, but no #Verify edge reaches {sr} — the work is verified, the REQUIREMENT is not (issue082); author `#Verify dependency from {v} to {sr};`")
                })
                .collect();
            GuardReport { name: "verification-trace", scanned: pairs.len(), warnings, violations: Vec::new() }
        }
        Err(e) => GuardReport { name: "verification-trace", scanned: 0, warnings: Vec::new(), violations: vec![format!("error computing verification trace: {e}")] },
    }
}

// ── process-skill guard (D0059/issue036: no inert process — every process has a deploying skill) ──

/// Every `.engine/processes/<file>.sysml` path referenced anywhere in the skills-registry text.
fn referenced_processes(reg: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    for tok in reg.split(|c: char| c.is_whitespace() || c == '"') {
        if let Some(rest) = tok.strip_prefix(".engine/processes/") {
            if let Some(idx) = rest.find(".sysml") {
                out.insert(rest[..idx + ".sysml".len()].to_string());
            }
        }
    }
    out
}

/// Coverage logic for the process-skill guard (pure, for self-test): every process file must be
/// referenced by ≥1 skill, and every referenced path must name an existing process.
fn process_skill_violations(processes: &[String], reg: &str) -> Vec<String> {
    let referenced = referenced_processes(reg);
    let mut violations = Vec::new();
    for p in processes {
        if !referenced.contains(p) {
            violations.push(format!("process '.engine/processes/{p}' has NO deploying skill (inert process — D0059; a deploying skill's purpose must name the process .sysml it deploys)"));
        }
    }
    let proc_set: HashSet<&str> = processes.iter().map(String::as_str).collect();
    let mut dangling: Vec<&String> = referenced.iter().filter(|r| !proc_set.contains(r.as_str())).collect();
    dangling.sort();
    for r in dangling {
        violations.push(format!("skill registry references '.engine/processes/{r}' which does not exist (dangling deploying claim — orphan skill edge)"));
    }
    violations
}

/// Guard (D0059/issue036): every process definition has a DEPLOYING skill ("no inert process").
///
/// D0059 establishes that a process with no deploying skill is applied by inconsistent vigilance (a
/// HIGH finding that recurred); the d0059 critique found the claimed coverage audit never existed.
/// The correspondence is a uniform CONVENTION — a deploying skill's `purpose` names the
/// `.engine/processes/<name>.sysml` it deploys — and this guard makes it machine-checkable.
///
/// VIOLATION: a process file referenced by NO skill (inert), or a skill referencing a process that
/// does not exist (a dangling deploying claim). A view-only skill that deploys no process is fine
/// (the audit is process→skill, not the reverse).
#[must_use]
pub fn process_skill(root: &Path) -> GuardReport {
    let proc_dir = root.join(".engine").join("processes");
    let processes: Vec<String> = crate::collect_sysml(&proc_dir)
        .iter()
        .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
        .collect();
    let reg_path = root.join(".engine").join("skills").join("skills-registry.sysml");
    let Ok(reg) = std::fs::read_to_string(&reg_path) else {
        return GuardReport { name: "process-skill", scanned: 0, warnings: Vec::new(), violations: vec![format!("cannot read {}", relpath(root, &reg_path))] };
    };
    let violations = process_skill_violations(&processes, &reg);
    GuardReport { name: "process-skill", scanned: processes.len(), warnings: Vec::new(), violations }
}

/// Diagnostic (D0047/issue039): a `#ProcessDefect` finding must resolve to a guard-producing action.
///
/// RUNNABLE via `keel guard defect-guard-coverage` but NOT in the enforced `GUARD_NAMES` — whether
/// a defect class "needs a guard" is judgment-bound (a shallow heuristic on the resolver name), so it
/// is a WARN for human attention, not a commit gate. Closes issue039: the "corrections become guards"
/// rule (D0047) now has an audit instead of relying purely on vigilance.
#[must_use]
pub fn defect_guard_coverage(root: &Path) -> GuardReport {
    match crate::view::defect_guard_coverage(root) {
        Ok((examined, warnings)) => GuardReport { name: "defect-guard-coverage", scanned: examined, warnings, violations: Vec::new() },
        Err(e) => GuardReport { name: "defect-guard-coverage", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading defect-guard coverage: {e}")] },
    }
}

// ── duplicate-identity guard (concurrent allocation otherwise lands GREEN) ─────────────────────

/// Extract the value of a `:>> <attr> = "<value>";` assignment appearing anywhere in `line`.
///
/// Attributes are frequently written inline (`part x : TestResult { :>> id = "…"; :>> outcome = …; }`),
/// so this searches the whole line rather than anchoring at the start.
fn inline_attr(line: &str, attr: &str) -> Option<String> {
    let mut rest = line;
    while let Some(pos) = rest.find(":>>") {
        let after = &rest[pos + 3..];
        let trimmed = after.trim_start();
        if let Some(tail) = trimmed.strip_prefix(attr) {
            let tail = tail.trim_start();
            if let Some(tail) = tail.strip_prefix('=') {
                let tail = tail.trim_start();
                if let Some(tail) = tail.strip_prefix('"') {
                    if let Some(end) = tail.find('"') {
                        return Some(tail[..end].to_owned());
                    }
                }
            }
        }
        rest = after;
    }
    None
}

/// Declaration keywords whose following token is an instance/definition NAME.
const DECL_KEYWORDS: [&str; 6] = ["part", "action", "verification", "requirement", "item", "use case"];

/// Name declared by `line`, if it is an instance declaration (not a reference or an edge).
fn declared_name(line: &str) -> Option<String> {
    // Strip a leading metadata marker prefix (`#ProspectiveChange part d0129 : Decision {`).
    let line = if line.starts_with('#') {
        line.split_once(' ').map_or("", |(_, r)| r).trim_start()
    } else {
        line
    };
    // Edges and successions mention names but declare none.
    for skip in ["first ", "flow ", "satisfy ", "verify ", "allocate ", "then ", "private ", "import ", "doc "] {
        if line.starts_with(skip) {
            return None;
        }
    }
    for kw in DECL_KEYWORDS {
        let Some(rest) = line.strip_prefix(kw) else { continue };
        let rest = rest.trim_start();
        // `part def Foo` / `action def Bar` declare a TYPE — still a name in the package scope.
        let rest = rest.strip_prefix("def ").map_or(rest, str::trim_start);
        let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if name.is_empty() {
            continue;
        }
        // Must be followed by a declaration token, not more words (which would make it a phrase).
        let after = rest[name.len()..].trim_start();
        if after.starts_with(':') || after.starts_with(';') || after.starts_with('{') {
            return Some(name);
        }
    }
    None
}

/// Element ids that were ALREADY duplicated when this guard was introduced (D0129, 2026-08-12).
///
/// Grandfathered to WARNING, never suppressed. These are hand-authored bootstrap ids that were
/// copy-pasted between files (`a1b2c3d4-…` appears four times), so the identity invariant (§2.3) was
/// already violated 26 times and nothing detected it. They are reported every run so the debt stays
/// visible, and they are tracked by issue080 for a proper migration (D0067) — repairing them means
/// rewriting ids that other records may reference, which is a migration, not a guard's job.
///
/// FORWARD-ONLY, per the issue068 lesson: a new guard must never retroactively fail historical items
/// governed by the process in force when they were authored. Anything NOT on this list is an ERROR.
/// Do not extend this list — a new duplicate is a defect to fix, not to grandfather.
const GRANDFATHERED_DUPLICATE_IDS: [&str; 18] = [
    "63940516-2c7d-4e8f-ed03-5162738e0305",
    "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d",
    "a7b8c9d0-e1f2-4a3b-4c5d-6e7f8a9b0c1d",
    "a7b8c9d0-e1f2-4a3b-9c4d-5e6f7a8b9c0d",
    "ad3e4f5a-b6c7-4d8e-a19c-3b4c5d6e7f8a",
    "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e",
    "b8c9d0e1-f2a3-4b4c-5d6e-7f8a9b0c1d2e",
    "b8c9d0e1-f2a3-4b4c-8d5e-6f7a8b9c0d1e",
    "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f",
    "c9d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f",
    "d0e1f2a3-b4c5-4d6e-7f8a-9b0c1d2e3f4a",
    "d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a",
    "df6b7c8d-e9f0-4a1b-cd2e-3f4a5b6c7d8e",
    "e1f2a3b4-c5d6-4e7f-8a9b-0c1d2e3f4a5b",
    "e5f6a7b8-c9d0-4e1f-2a3b-4c5d6e7f8a9b",
    "ef7c8d9e-f0a1-4b2c-de3f-4a5b6c7d8e9f",
    "f2a3b4c5-d6e7-4f8a-9b0c-1d2e3f4a5b6c",
    "f6a7b8c9-d0e1-4f2a-3b4c-5d6e7f8a9b0c",
];

/// Guard: no repeated identity anywhere in the model (issue074 / D0129).
///
/// This is the failure class where the **absence** of a git conflict is the danger. Git protects
/// against concurrent edits to the same LINES; it does not protect against concurrent allocation of
/// the same NAME. Two contributors working offline both mint the next decision number by directory
/// scan (`write.rs::next_decision_number`); because their slugs differ the FILENAMES differ, so git
/// reports no conflict, both land, and the resulting duplicate `package DecisionNNNN` declarations
/// are silently merged by the registry (`keel-parser/src/registry.rs`, `or_default()`). The same shape
/// applies to two `sprintNNN_*` files (both counted by `in_progress_sprints`) and to hand-appended
/// `issueNNN` names. Corruption therefore lands GREEN and is undetectable afterwards.
///
/// Four classes are detected:
/// 1. repeated element `id` — identity itself (CLAUDE.md §2.3), also the backstop for a UUID collision
/// 2. repeated declared item name within one package
/// 3. repeated `package` name across files — the silently-merged case
/// 4. repeated allocated sequence number (decision file `NNNN-`, sprint file `sprintNNN_`)
///
/// Per D0047 a recurrable defect class gets a permanent automated control, not vigilance — and this
/// one bit the engine's own authors during D0129 (two workstreams both allocated `issue071`; nothing
/// warned, because the two claims lived in different files).
#[must_use]
pub fn duplicate_identity(root: &Path) -> GuardReport {
    let mut paths = crate::collect_sysml(&root.join(".tracking"));
    paths.extend(crate::collect_sysml(&root.join(".engine")));
    let scanned = paths.len();
    let files: Vec<(String, String)> = paths
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok().map(|t| (relpath(root, p), t)))
        .collect();

    let (warnings, mut violations) = duplicate_scan(&files);

    // Class 4 — allocated sequence numbers embedded in FILENAMES (no git conflict when slugs differ).
    violations.extend(duplicate_sequence(root, &root.join(".engine").join("decisions"), "", 4));
    violations.extend(duplicate_sequence(root, &root.join(".tracking").join("delivery"), "sprint", 0));

    GuardReport { name: "duplicate-identity", scanned, warnings, violations }
}

/// Pure core of `duplicate_identity`: scan `(relpath, text)` pairs for repeated ids, item names and
/// package names. Returns `(warnings, violations)` — grandfathered id duplicates warn, the rest fail.
fn duplicate_scan(files: &[(String, String)]) -> (Vec<String>, Vec<String>) {
    let mut violations = Vec::new();
    let mut warnings = Vec::new();
    let grandfathered: HashSet<&str> = GRANDFATHERED_DUPLICATE_IDS.iter().copied().collect();

    let mut ids: HashMap<String, String> = HashMap::new();
    let mut pkgs: HashMap<String, String> = HashMap::new();
    let mut items: HashMap<(String, String), String> = HashMap::new();

    for (rel, text) in files {
        let mut cur_pkg = String::new();

        for (i, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('*') || line.starts_with("/*") {
                continue;
            }
            let loc = format!("{rel}:{}", i + 1);

            if let Some(rest) = line.strip_prefix("package ") {
                let name = rest.split_whitespace().next().unwrap_or("").trim_end_matches('{').trim();
                if !name.is_empty() {
                    if cur_pkg.is_empty() {
                        name.clone_into(&mut cur_pkg);
                    }
                    if let Some(prev) = pkgs.insert(name.to_owned(), loc.clone()) {
                        violations.push(format!(
                            "{loc}: duplicate package name `{name}` (also declared at {prev}) — the registry SILENTLY MERGES same-named packages, so this corruption would land green (issue074)"
                        ));
                    }
                }
                continue;
            }

            if let Some(id) = inline_attr(line, "id") {
                if let Some(prev) = ids.insert(id.clone(), loc.clone()) {
                    if grandfathered.contains(id.as_str()) {
                        warnings.push(format!(
                            "{loc}: duplicate element id \"{id}\" (also at {prev}) — GRANDFATHERED bootstrap duplicate, pre-issue074; tracked by issue080 for migration"
                        ));
                    } else {
                        violations.push(format!(
                            "{loc}: duplicate element id \"{id}\" (also at {prev}) — identity is the invariant that lets items share a name (§2.3); a collision corrupts it"
                        ));
                    }
                }
            }

            if let Some(name) = declared_name(line) {
                let key = (cur_pkg.clone(), name.clone());
                if let Some(prev) = items.insert(key, loc.clone()) {
                    violations.push(format!(
                        "{loc}: duplicate declared name `{name}` in package `{cur_pkg}` (also at {prev}) — concurrent allocation produces no git conflict, so nothing else would warn"
                    ));
                }
            }
        }
    }

    (warnings, violations)
}

/// Detect two files in `dir` that claim the same allocated sequence number.
///
/// `prefix` is stripped before reading digits (`sprint163_x` -> `163`); `width` > 0 requires exactly
/// that many leading digits (decision files are zero-padded `0129-`).
fn duplicate_sequence(root: &Path, dir: &Path, prefix: &str, width: usize) -> Vec<String> {
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.extension().and_then(|e| e.to_str()) != Some("sysml") {
            continue;
        }
        let Some(stem) = path.file_name().and_then(|n| n.to_str()) else { continue };
        let Some(rest) = stem.strip_prefix(prefix) else { continue };
        let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
        if digits.is_empty() || (width > 0 && digits.len() != width) {
            continue;
        }
        let rel = relpath(root, &path);
        if let Some(prev) = seen.insert(digits.clone(), rel.clone()) {
            out.push(format!(
                "{rel}: sequence number {prefix}{digits} already allocated by {prev} — two contributors allocated it independently; different slugs mean git reported NO conflict (issue074)"
            ));
        }
    }
    out
}

/// The ENFORCED forward guards, in CLI/runner order.
///
/// `issues` joined the enforced set at IRL-d (D0077). HONEST-STATE doctrine (D0098): the enforced set
/// holds only INTEGRITY guards — the recorded model must not lie, be malformed, or be untraceable.
/// COMPLETENESS / self-assurance (`assured` composite readiness + `critique`-COVERAGE) was DEMOTED
/// from this set: it is computed as a NON-BLOCKING burndown (`keel guard assured` / `keel critique-
/// coverage` stay runnable, surfaced in orient), never a hard commit gate — incomplete implementation
/// flagged AS incomplete is honest state, not a failure. NOTE: critique INDEPENDENCE stays enforced
/// (critic-independence — honesty); only critique COVERAGE demoted. The requirement-rootedness hard
/// guard (D0098 honesty: a chartered capability with no driving Need) joins next (requirementRootednessGuard).
pub const GUARD_NAMES: [&str; 29] =
    ["actors", "acceptance-events", "sprint-coverage", "ceremony", "charter", "process-change", "issues", "viewpoint-renderer", "manifest-coverage", "critic-independence", "process-skill", "requirement-rootedness", "decision-rationale", "attestation-substance", "marker-vocabulary", "duplicate-identity", "decision-requirement-link", "verification-trace", "priority-inversion", "retro-backlog", "confirmation-authenticity", "engine-lint", "doc-sync", "hook-config-integrity", "activation-manifest", "sequence-multiplicity", "parser-coverage", "base-first-justification", "edge-endpoints"];

/// Script extensions a hook command may invoke. Deliberately EXCLUDES `.exe` and extensionless
/// binaries: a not-yet-built `target/release/keel.exe` is a legitimate transient state that the hook
/// commands already probe for, whereas a script is committed source that must exist to be referenced.
const HOOK_SCRIPT_EXTS: [&str; 6] = ["py", "sh", "ps1", "js", "mjs", "rb"];

/// WARNING-level: a hook command referencing a script that DOES NOT EXIST (issue093).
///
/// Why this guard exists, and why it is a guard rather than a reminder (D0047): migrating the in-loop
/// gates into the binary (D0134) deleted `.engine/tools/stop_gate.py`, and `.claude/settings.json` got
/// the replacement — but `.claude/settings.local.json` ALSO declared a Stop hook pointing at the
/// deleted script. Claude Code MERGES hooks across settings files, so both fired and the stale one
/// failed on every single turn end. It survived because `settings.local.json` is GITIGNORED: it never
/// appeared in `git status`, never in a diff, and the doc-sync sweep covers the tracked surface only.
/// So the delete-completely discipline had a blind spot exactly where hook wiring lives.
///
/// WARNING, not hard-blocking, and the level is the point: this config is machine-local and partly
/// gitignored, so CI cannot see it and one contributor's personal hook must never block another's
/// commit. A warning still surfaces on every `keel guard` — including inside the Stop hook itself,
/// which is what makes a sibling hook's breakage self-reporting rather than something the human has to
/// notice in scrollback.
fn hook_config_integrity(root: &Path) -> GuardReport {
    let mut warnings = Vec::new();
    let mut scanned = 0usize;

    for rel in [".claude/settings.json", ".claude/settings.local.json"] {
        let path = root.join(rel);
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue; // absent is fine — neither file is required
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
            warnings.push(format!("{rel}: not valid JSON — every hook declared here is silently inert"));
            continue;
        };
        let Some(events) = json.get("hooks").and_then(serde_json::Value::as_object) else {
            continue;
        };
        for (event, groups) in events {
            for group in groups.as_array().into_iter().flatten() {
                for hook in group.get("hooks").and_then(serde_json::Value::as_array).into_iter().flatten() {
                    let Some(cmd) = hook.get("command").and_then(serde_json::Value::as_str) else {
                        continue;
                    };
                    scanned += 1;
                    for tok in cmd.split([' ', '"', '\'', '\t', ';', '|', '&', '(', ')']) {
                        let t = tok.trim();
                        if !Path::new(t).extension().is_some_and(|e| {
                            HOOK_SCRIPT_EXTS.iter().any(|x| e.eq_ignore_ascii_case(x))
                        }) {
                            continue;
                        }
                        // Only repo-relative references are checkable; an absolute path may live on a
                        // machine we are not inspecting, so claiming it is missing would be wrong.
                        if Path::new(t).is_absolute() {
                            continue;
                        }
                        if !root.join(t).exists() {
                            warnings.push(format!(
                                "{rel}: {event} hook references `{t}`, which does not exist — that hook FAILS every time it fires (issue093)"
                            ));
                        }
                    }
                }
            }
        }
    }

    GuardReport { name: "hook-config-integrity", scanned, warnings, violations: Vec::new() }
}

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
        "assured" => Some(assured(root)),
        "viewpoint-renderer" => Some(viewpoint_renderer(root)),
        "manifest-coverage" => Some(manifest_coverage(root)),
        "critic-independence" => Some(critic_independence(root)),
        "process-skill" => Some(process_skill(root)),
        "requirement-rootedness" => Some(requirement_rootedness(root)),
        "decision-rationale" => Some(decision_rationale(root)), // hard (D0103)
        "marker-vocabulary" => Some(marker_vocabulary(root)), // hard (D0133/issue077) — an undeclared marker silently blinds a control
        "attestation-substance" => Some(attestation_substance(root)), // hard (D0130/issue083) — a confirmation must attest something
        "duplicate-identity" => Some(duplicate_identity(root)), // hard (D0129/issue074) — concurrent allocation lands green without it
        "decision-requirement-link" => Some(decision_requirement_link(root)), // warning-only member of GUARD_NAMES (D0102)
        "verification-trace" => Some(verification_trace(root)), // warning-only (D0130/issue082) — delivered work whose requirement is untraced
        "priority-inversion" => Some(priority_inversion(root)), // warning-only (D0130/issue084) — recorded order vs recorded severity
        "retro-backlog" => Some(retro_backlog(root)), // warning-only (D0130/issue085) — a retro finding must not terminate in prose
        "base-first-justification" => Some(base_first_justification(root)), // warning-only (D0139(B))
        "edge-endpoints" => Some(edge_endpoints(root)), // hard (issue109) — an edge asserting a relationship to nothing
        "parser-coverage" => Some(parser_coverage(root)), // warning-only (issue102) — what the engine cannot read
        "sequence-multiplicity" => Some(sequence_multiplicity(root)), // warning-only (issue101) — sequences are newly enabled
        "activation-manifest" => Some(activation_manifest(root)), // hard (D0138) — a typo silently disables a control
        "hook-config-integrity" => Some(hook_config_integrity(root)), // warning-only (D0047/issue093) — a hook pointing at a deleted script
        "confirmation-authenticity" => Some(confirmation_authenticity(root)), // hard (D0106/issue059) — rule-sourced
        "engine-lint" => Some(engine_lint(root)), // hard import-check + warn missing-id (D0112 phase 1, kernel-free)
        "doc-sync" => Some(doc_sync(root)), // WARNING-level member of GUARD_NAMES (D0113) — definitional change w/o doc update

        "critique-rigor" => Some(critique_rigor(root)), // runnable-only (not in GUARD_NAMES)
        "defect-guard-coverage" => Some(defect_guard_coverage(root)), // runnable-only (D0047/issue039)
        _ => None,
    }
}

/// Run all enforced guards over `root`, returning their reports in `GUARD_NAMES` order.
#[must_use]
pub fn run_all(root: &Path) -> Vec<GuardReport> {
    let act = crate::activation::Activation::load(root);
    GUARD_NAMES
        .iter()
        .filter_map(|n| match act.guard_state(n) {
            // A process the project has not adopted: SKIP the check, but say so. Silence here would be
            // the issue090 defect inverted — instead of failing a project for a control it never
            // adopted, we would be passing it while hiding that the control is off (D0138).
            crate::activation::GuardState::Inactive(p) => Some(GuardReport {
                name: n,
                scanned: 0,
                warnings: vec![format!(
                    "NOT ACTIVE — process `{p}` is not in this project's active set, so this control was NOT checked (D0138; `keel activate {p}` to adopt it)"
                )],
                violations: Vec::new(),
            }),
            _ => run_one(n, root),
        })
        .collect()
}

/// Every `(name, attributes)` pair in a package, including those nested inside an `action def` body —
/// which is where the backlog and every sprint record actually live, so a top-level-only walk would
/// inspect almost nothing.
fn named_attr_bearers(pkg: &keel_parser::ast::Package) -> Vec<(&str, &[keel_parser::ast::Attribute])> {
    use keel_parser::ast::Item;
    let mut out: Vec<(&str, &[keel_parser::ast::Attribute])> = Vec::new();
    for item in &pkg.items {
        match item {
            Item::Part(p) => out.push((p.name.as_str(), &p.attributes)),
            Item::Verification(v) => out.push((v.name.as_str(), &v.attributes)),
            Item::ActionDef(d) => {
                for p in &d.parts {
                    out.push((p.name.as_str(), &p.attributes));
                }
                for v in &d.verifications {
                    out.push((v.name.as_str(), &v.attributes));
                }
            }
            _ => {}
        }
    }
    out
}

/// WARNING-level: a PROJECT-declared marker with no recorded justification (D0139(B)).
///
/// D0139 requires that a custom `metadata def` edge be a last resort carrying a recorded justification
/// naming the base constructs considered and why each fails. That rule exists because a documented
/// PREFERENCE demonstrably does not work here: `sysmlv2-syntax-notes.md:16` already recorded `:>` as the
/// idiomatic derivation form, and `#DerivedFrom` was used 37 times anyway. D0047 is explicit that manual
/// vigilance is not a control.
///
/// Scope is deliberately narrow. The engine's own 17 markers are GRANDFATHERED — forward-only, the
/// issue068 rule — and D0140 has since supplied kernel-verified justifications for the two that needed
/// them. So this fires only on markers a PROJECT declares from here on, of which there are currently
/// zero. A guard that reports nothing today and blocks a bad habit tomorrow is the intended shape.
///
/// TEXT-BASED, and the reason is worth stating rather than hiding: the AST does not capture `doc`
/// clauses or comments, so there is nothing structural to inspect. Capturing doc text is a separate
/// parser increment; until it lands, a text scan is the honest option, and it is scoped to the lines
/// immediately around the declaration rather than the whole file.
fn base_first_justification(root: &Path) -> GuardReport {
    let engine: HashSet<&str> = ENGINE_MARKERS.iter().copied().collect();
    let mut warnings = Vec::new();
    let mut scanned = 0usize;
    for dir in [root.join(".tracking"), root.join(".engine")] {
        if !dir.is_dir() {
            continue;
        }
        for path in crate::collect_sysml(&dir) {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let Some(rest) = line.trim_start().strip_prefix("metadata def ") else { continue };
                // Strip a trailing `// comment` BEFORE taking the name. Without this,
                // `metadata def View;   // marks a computed artifact` yields a name containing the whole
                // comment, so an ENGINE marker fails the grandfather check and is reported as a project
                // marker — which is exactly what happened on the first run.
                let decl = rest.split("//").next().unwrap_or(rest);
                let name = decl.trim_end_matches([';', ' ', '{']).trim();
                if name.is_empty() || engine.contains(name) {
                    continue; // engine markers are grandfathered (issue068)
                }
                scanned += 1;
                // A justification is a `doc` clause on the declaration, or comment lines directly above
                // it. Both are how this repo actually documents its markers today.
                let has_doc = line.contains("doc ")
                    || lines
                        .get(i.saturating_sub(3)..i)
                        .unwrap_or_default()
                        .iter()
                        .any(|l| {
                            let t = l.trim_start();
                            t.starts_with("//") && t.len() > 8
                        });
                if !has_doc {
                    warnings.push(format!(
                        "{rel}:{}: project marker `#{name}` has no recorded justification — D0139(B) requires naming the base SysML v2 constructs considered and why each fails, because a custom edge is a last resort and an undocumented one is dialect nobody can audit",
                        i + 1
                    ));
                }
            }
        }
    }
    GuardReport { name: "base-first-justification", scanned, warnings, violations: Vec::new() }
}

/// WARNING-level: statements the parser could not read, grouped by leading token (issue102).
///
/// The parser recognises a fixed statement set and skips the rest — silently, until now. Measured with
/// an undeclared target, `ref e : NoSuchType`, `port p : NoSuchPortDef`, `assert constraint c :
/// NoSuchConstraint` and `connect ghostA.p to ghostB.p` ALL validate clean, while the control
/// (`part x : NoSuchType`) correctly produces a diagnostic. So those statements are not merely
/// unresolved — they are invisible.
///
/// This is the safety property that makes the rest of the base-first pass survivable. Every construct
/// D0139 converts toward is currently in that invisible set, so a conversion landing before the reader
/// would make its edges parse clean and vanish while every guard reported green — the failure mode this
/// project exists to prevent, and the one issue027 already fixed for items dropped outside a package.
///
/// Reports a per-lead-token count rather than one line per statement: the engine's own schema skips 29
/// statements today, and a 29-line warning block every run would train its reader to ignore the guard.
/// The counts are what shows a conversion going wrong — a lead token appearing where it did not before.
fn parser_coverage(root: &Path) -> GuardReport {
    let mut by_lead: BTreeMap<String, usize> = BTreeMap::new();
    let mut scanned = 0usize;
    for dir in [root.join(".tracking"), root.join(".engine")] {
        if !dir.is_dir() {
            continue;
        }
        for path in crate::collect_sysml(&dir) {
            let Ok(pkg) = crate::parse_pkg(&path) else { continue };
            scanned += 1;
            for sk in &pkg.skipped {
                *by_lead.entry(sk.lead.clone()).or_default() += 1;
            }
        }
    }
    let total: usize = by_lead.values().sum();
    let mut warnings = Vec::new();
    if total > 0 {
        // Rank by count and show only the head. The tail is dominated by element NAMES — a skipped
        // `use case <name> : T` reports its own name as the lead token — which produces dozens of
        // singletons that bury the kinds worth acting on. Collapsing them keeps the guard readable,
        // which is the difference between a control someone reads and one they learn to scroll past.
        let mut ranked: Vec<(&String, &usize)> = by_lead.iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        let head: Vec<String> = ranked.iter().take(8).map(|(l, n)| format!("{l}×{n}")).collect();
        let tail: usize = ranked.iter().skip(8).map(|(_, n)| **n).sum();
        let tail_note =
            if tail > 0 { format!(", and {tail} more across {} kind(s)", ranked.len() - 8) } else { String::new() };
        warnings.push(format!(
            "{total} statement(s) across {scanned} file(s) are SKIPPED by keel-parser and therefore invisible to every guard and view: {}{tail_note}. A base-first conversion onto any of these would parse clean and lose its edges (issue102/D0139)",
            head.join(", ")
        ));
    }
    GuardReport { name: "parser-coverage", scanned, warnings, violations: Vec::new() }
}

/// WARNING-level: every multi-valued feature assignment `:>> f = (a, b, c)` in the model (issue101).
///
/// Enabling the sequence form (issue095) removed a crude safety property: `(` used to be a parse error
/// EVERYWHERE, so a sequence could not be written into a single-valued attribute. Now it can, and it is
/// accepted silently — `createdBy = ("you", "ghost")` parses clean and the `actors` guard passes, so an
/// unregistered actor slips through. The precise check — reject a sequence where the schema declares no
/// `[*]` multiplicity — is not yet possible: neither the AST nor the registry captures multiplicity.
///
/// So this reports every sequence instead. That is genuinely useful rather than a placeholder, because
/// the model contains ZERO sequences today: any hit is new, and reviewing it is exactly the check that
/// multiplicity metadata will later automate. Deliberately AST-based, not a text scan — grepping for
/// `= (` matches the prose in Decisions and definition-of-done text that discusses the sequence form, which is the
/// self-inflating-census error of issue099 in miniature.
///
/// Retire this guard when multiplicity lands and the exact check replaces it.
fn sequence_multiplicity(root: &Path) -> GuardReport {
    use keel_parser::ast::Value;
    let mut warnings = Vec::new();
    let mut scanned = 0usize;
    for dir in [root.join(".tracking"), root.join(".engine")] {
        if !dir.is_dir() {
            continue;
        }
        for path in crate::collect_sysml(&dir) {
            let Ok(pkg) = crate::parse_pkg(&path) else {
                continue; // parse errors are validate's business, not this guard's
            };
            let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
            for (item_name, attrs) in named_attr_bearers(&pkg) {
                for a in attrs {
                    if let Value::Seq(items) = &a.value {
                        scanned += 1;
                        warnings.push(format!(
                            "{rel}:{}: {item_name}.{} is a {}-element sequence — confirm the schema declares this feature multi-valued ([*]); a sequence in a single-valued attribute is accepted silently today (issue101)",
                            a.line,
                            a.name,
                            items.len()
                        ));
                    }
                }
            }
        }
    }
    GuardReport { name: "sequence-multiplicity", scanned, warnings, violations: Vec::new() }
}

/// HARD: the activation manifest itself must be well-formed (D0138).
///
/// Hard-blocking is safe here because every check is EXACT set-membership against `GUARD_NAMES` and the
/// processes on disk — no heuristic. And it must be hard: a typo in either contract file silently
/// disables a control, which is strictly worse than the control failing loudly. Absence of either file
/// is NOT a violation (the issue090 lesson: a project that never adopted a control has not violated it).
fn activation_manifest(root: &Path) -> GuardReport {
    let act = crate::activation::Activation::load(root);
    let scanned = act.unit_names().len();
    let mut warnings = Vec::new();
    if act.is_declared() {
        let inactive = act.inactive_processes();
        if !inactive.is_empty() {
            warnings.push(format!(
                "this project has NOT activated: {} — their guards are skipped (visible above, never silent)",
                inactive.join(", ")
            ));
        }
    }
    GuardReport { name: "activation-manifest", scanned, warnings, violations: act.errors }
}

// ── engine-lint guard (D0112 phase 1: the mechanical .engine instance lints, ported kernel-free) ──

/// Instance types that carry an `:>> id` (identity invariant §2.3). Mirrors `validate_instances._ID_TYPES`.
const ENGINE_ID_TYPES: &[&str] = &[
    "Decision", "AISkill", "Agent", "Process", "ProcessStep", "TestResult", "Brief", "Persona", "Need",
    "Issue", "Story", "Release", "ChangeRequest", "Component", "DesignElement", "Test", "Viewpoint",
    "Indicator", "Measurement",
];

/// Count `part|verification|requirement <name> : <IdType>` declarations in `text`.
///
/// Line-based mirror of `validate_instances.warn_missing_ids`.
fn count_tracked_instances(text: &str) -> usize {
    text.lines()
        .filter(|line| {
            let t = line.trim_start();
            ["part ", "verification ", "requirement "].iter().any(|kw| {
                t.strip_prefix(kw)
                    .and_then(|rest| rest.split_once(':'))
                    .map(|(_, after)| after.trim_start().split(|c: char| !c.is_alphanumeric()).next().unwrap_or(""))
                    .is_some_and(|ty| ENGINE_ID_TYPES.contains(&ty))
            })
        })
        .count()
}

/// The two mechanical `.engine`-instance lints, ported kernel-free (D0112 phase 1).
///
/// (1) HARD — every `.engine/decisions/*.sysml` must `import EngineWork` (the `Decision` type lives
/// there). (2) WARN — every tracked instance (`part|verification|requirement <name> : <IdType>`) should
/// carry an `:>> id` (§2.3). The first kernel-free step of retiring the JVM from the `.engine` path;
/// parity with the python lints by VERDICT, not byte-identical text.
#[must_use]
pub fn engine_lint(root: &Path) -> GuardReport {
    let mut warnings = Vec::new();
    let mut violations = Vec::new();
    let decisions_dir = root.join(".engine").join("decisions");
    let decision_files = crate::collect_sysml(&decisions_dir);
    // (1) HARD: import-EngineWork on every decision file.
    for path in &decision_files {
        if let Ok(text) = std::fs::read_to_string(path) {
            if !text.contains("import EngineWork") {
                violations.push(format!(
                    "{}: Decision file missing 'import EngineWork' — the Decision type lives in EngineWork (D0112)",
                    relpath(root, path)
                ));
            }
        }
    }
    // (2) WARN: missing-id across the .engine instance set (decisions/processes/views + registry + template).
    let mut inst_files: Vec<PathBuf> = Vec::new();
    for sub in ["decisions", "processes", "views"] {
        inst_files.extend(crate::collect_sysml(&root.join(".engine").join(sub)));
    }
    inst_files.push(root.join(".engine").join("skills").join("skills-registry.sysml"));
    inst_files.push(root.join(".engine").join("docs").join("tracking-template.sysml"));
    for path in &inst_files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let inst = count_tracked_instances(&text);
        let ids = text.matches(":>> id =").count();
        if inst > ids {
            warnings.push(format!("{}: {} tracked instance(s) missing :>> id (§2.3)", relpath(root, path), inst - ids));
        }
    }
    GuardReport { name: "engine-lint", scanned: decision_files.len() + inst_files.len(), warnings, violations }
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
    fn doc_sync_flags_undocumented_definitional_change() {
        // D0113: a schema/process/workflow change with no co-committed doc = warn; with a doc = clean.
        let warn = doc_sync_warnings(&[".engine/processes/foo.sysml".to_string(), "keel-cli/src/x.rs".to_string()]);
        assert_eq!(warn.len(), 1, "definitional change + no doc must warn: {warn:?}");
        assert!(doc_sync_warnings(&[".engine/processes/foo.sysml".to_string(), "CLAUDE.md".to_string()]).is_empty());
        assert!(doc_sync_warnings(&[".engine/schema/core.sysml".to_string(), ".engine/docs/guide.md".to_string()]).is_empty());
        assert!(doc_sync_warnings(&[".tracking/backlog.sysml".to_string()]).is_empty()); // nothing definitional
    }

    #[test]
    fn engine_lint_counts_tracked_instances() {
        // D0112 phase 1: only `part|verification|requirement <name> : <IdType>` count toward the
        // missing-id check; non-ID types (SystemRequirement) and non-instance lines don't.
        let text = "package P {\n    part d1 : Decision { :>> id = \"x\"; }\n    verification t1 : Test { :>> id = \"y\"; }\n    requirement r1 : SystemRequirement {}\n    part note : SomeOtherType {}\n    action a1;\n}";
        assert_eq!(count_tracked_instances(text), 2); // Decision + Test only
    }

    #[test]
    fn viewpoint_renderer_classification() {
        // D0056/issue034: retired-tool refs + unknown commands are violations; planned is a warning;
        // a real keel subcommand is ok.
        assert_eq!(classify_renderer("query.py governing-version <item>"), "retired");
        assert_eq!(classify_renderer("report.py:tab_decisions"), "retired");
        assert_eq!(classify_renderer("(planned) baselines view — not yet rendered"), "planned");
        assert_eq!(classify_renderer("keel diagram (interactive HTML #View)"), "ok");
        assert_eq!(classify_renderer("keel report <assurance|...> [--html]"), "ok");
        assert_eq!(classify_renderer("keel frobnicate"), "unknown");
        assert_eq!(classify_renderer("some hand-wave"), "unknown");
        assert_eq!(quoted_attr("    :>> renderer = \"keel orient\";", "renderer").as_deref(), Some("keel orient"));
    }

    #[test]
    fn manifest_parses_per_task_entries() {
        // D0050/issue033: `task: NAME | p1 p2` lines parse to (name, paths); comments/blanks skipped.
        let text = "# header comment\n\ntask: rustS1Lexer | keel-parser/src/lexer.rs keel-parser/src/token.rs\ntask: writeApi | keel-cli/src/write.rs\n";
        let entries = parse_manifest(text);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].0, "rustS1Lexer");
        assert_eq!(entries[0].1, vec!["keel-parser/src/lexer.rs".to_string(), "keel-parser/src/token.rs".to_string()]);
        assert_eq!(entries[1], ("writeApi".to_string(), vec!["keel-cli/src/write.rs".to_string()]));
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

    // charter_selftest retired (D0107 CONTRACT): the charter guard now sources from charterRule; its
    // logic + parity are covered by view::tests::edge_rule_newly_added_scope_restricts_to_staged_files.

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

    #[test]
    fn process_skill_flags_inert_and_dangling() {
        let procs = vec!["doc-sync.sysml".to_string(), "lonely.sysml".to_string()];
        let reg = "purpose = \"deploying skill for .engine/processes/doc-sync.sysml.\"\npurpose = \"for .engine/processes/ghost.sysml (dangling)\"";
        let v = process_skill_violations(&procs, reg);
        assert!(v.iter().any(|m| m.contains("lonely.sysml") && m.contains("NO deploying skill")), "inert process flagged");
        assert!(v.iter().any(|m| m.contains("ghost.sysml") && m.contains("dangling")), "dangling claim flagged");
        // doc-sync.sysml is referenced -> not flagged as inert.
        assert!(!v.iter().any(|m| m.contains("doc-sync.sysml") && m.contains("NO deploying skill")));
        // All real -> clean.
        let clean = process_skill_violations(&["doc-sync.sysml".to_string()], "x .engine/processes/doc-sync.sysml, y");
        assert!(clean.is_empty(), "every process referenced -> clean");
    }

    #[test]
    fn duplicate_scan_catches_each_identity_class() {
        // issue074/D0129. The danger is that NONE of these produce a git conflict when the two
        // claims live in different files, so without this guard the corruption lands green.
        let a = (
            "a.sysml".to_string(),
            "package P {\n    part x : Need { :>> id = \"11111111-1111-4111-9111-111111111111\"; }\n}".to_string(),
        );
        // Same id in a different file -> collision of identity itself.
        let dup_id = (
            "b.sysml".to_string(),
            "package Q {\n    part y : Need { :>> id = \"11111111-1111-4111-9111-111111111111\"; }\n}".to_string(),
        );
        // Same package name in a different file -> the registry silently MERGES these.
        let dup_pkg = (
            "c.sysml".to_string(),
            "package P {\n    part z : Need { :>> id = \"22222222-2222-4222-9222-222222222222\"; }\n}".to_string(),
        );
        // Same declared name inside one package.
        let dup_name = (
            "d.sysml".to_string(),
            "package R {\n    part dup : Need { :>> id = \"33333333-3333-4333-9333-333333333333\"; }\n    part dup : Need { :>> id = \"44444444-4444-4444-9444-444444444444\"; }\n}".to_string(),
        );

        let (_, v_id) = duplicate_scan(&[a.clone(), dup_id]);
        assert!(v_id.iter().any(|m| m.contains("duplicate element id")), "duplicate id must fail: {v_id:?}");

        let (_, v_pkg) = duplicate_scan(&[a.clone(), dup_pkg]);
        assert!(v_pkg.iter().any(|m| m.contains("duplicate package name")), "duplicate package must fail: {v_pkg:?}");

        let (_, v_name) = duplicate_scan(&[dup_name]);
        assert!(v_name.iter().any(|m| m.contains("duplicate declared name")), "duplicate name must fail: {v_name:?}");

        // A clean pair must stay silent — no false positives on distinct ids/names/packages.
        let (w, v) = duplicate_scan(&[a, ("e.sysml".to_string(),
            "package S {\n    part other : Need { :>> id = \"55555555-5555-4555-9555-555555555555\"; }\n}".to_string())]);
        assert!(v.is_empty() && w.is_empty(), "distinct identities -> clean: {v:?} {w:?}");
    }

    #[test]
    fn duplicate_scan_grandfathers_known_bootstrap_ids_as_warnings() {
        // FORWARD-ONLY (the issue068 lesson): a new guard must not retroactively FAIL historical
        // items. The 18 bootstrap ids already duplicated when the guard landed warn instead — the
        // debt stays visible (issue080) without blocking every commit.
        let gf = GRANDFATHERED_DUPLICATE_IDS[0];
        let files = vec![
            ("a.sysml".to_string(), format!("package P {{\n    part x : Need {{ :>> id = \"{gf}\"; }}\n}}")),
            ("b.sysml".to_string(), format!("package Q {{\n    part y : Need {{ :>> id = \"{gf}\"; }}\n}}")),
        ];
        let (warnings, violations) = duplicate_scan(&files);
        assert!(violations.is_empty(), "grandfathered id must NOT fail: {violations:?}");
        assert!(
            warnings.iter().any(|m| m.contains("GRANDFATHERED") && m.contains(gf)),
            "grandfathered id must warn visibly: {warnings:?}"
        );
    }

    #[test]
    fn declared_name_ignores_edges_and_references() {
        // Edges and successions MENTION names without declaring any; treating them as declarations
        // would make the guard unusable through false positives.
        assert_eq!(declared_name("part foo : Need {"), Some("foo".to_string()));
        assert_eq!(declared_name("action bar;"), Some("bar".to_string()));
        assert_eq!(declared_name("verification gate : Test {"), Some("gate".to_string()));
        assert_eq!(declared_name("#ProspectiveChange part d0129 : Decision {"), Some("d0129".to_string()));
        assert_eq!(declared_name("part def Thing :> Element {"), Some("Thing".to_string()));
        assert_eq!(declared_name("first brief then personas;"), None);
        assert_eq!(declared_name("satisfy nNeed by srReq;"), None);
        assert_eq!(declared_name("flow from a.o to b.i;"), None);
        assert_eq!(declared_name("private import EngineElement::*;"), None);
        assert_eq!(declared_name(":>> id = \"x\";"), None);
    }

    #[test]
    fn inline_attr_reads_attributes_written_mid_line() {
        // Records are routinely written as one-liners, so anchoring at line start would miss them.
        let line = "part r : TestResult { :>> id = \"abc-123\"; :>> outcome = VerdictKind::pass; }";
        assert_eq!(inline_attr(line, "id"), Some("abc-123".to_string()));
        assert_eq!(inline_attr(line, "title"), None);
        assert_eq!(inline_attr("    :>> id  =  \"spaced\";", "id"), Some("spaced".to_string()));
    }
}
