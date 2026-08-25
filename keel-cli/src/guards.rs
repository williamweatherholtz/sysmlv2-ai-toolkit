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
        // SELF-CONTRADICTION CHECK (issue180). A guard cannot find a violation in a population of
        // zero, so `0 scanned, 1 violation(s)` means the guard is not reporting what it examined. Three
        // guards printed exactly that while working correctly, which made `scanned` useless as a
        // liveness signal - the only signal separating a guard whose population is legitimately empty
        // from one that is mis-aimed and can never fire. Surfaced in the RUNNER rather than as a test,
        // so it holds for every guard added after this one without anybody remembering to.
        if self.scanned == 0 && !self.violations.is_empty() {
            println!(
                "  WARN  guard `{}` reports {} violation(s) against a scan count of 0 - it is not                  reporting the population it examined (issue180)",
                self.name,
                self.violations.len()
            );
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
    let mut report = match crate::view::rule_violations_opt(root, "confirmationAuthenticityRule") {
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
    };
    // D0192 OPTION A substance half: when the attestation policy DECLARES a recording delegation for
    // acceptances, a delegated record must actually quote the human's conversational words. Sourced
    // from `delegatedAcceptanceSubstanceRule` (CONTRACT pattern, forward-only per the rule's cutoff).
    // A declared delegation with no substance rule is warned — the policy promised a check that is
    // not adopted — and no declared delegation means nothing is demanded here.
    if let Some(delegation) = crate::activation::recording_delegation(root, "decisionAcceptance") {
        match crate::view::rule_violations_opt(root, "delegatedAcceptanceSubstanceRule") {
            Ok(Some((_, bad))) => {
                for d in bad {
                    report.violations.push(format!(
                        "{d}: accepted on/after the delegation cutoff but its acceptance event neither quotes the human's words (a single-quoted span) nor cites their gesture — a record made under the {delegation} recording delegation must carry its channel evidence"
                    ));
                }
            }
            Ok(None) => report.warnings.push(format!(
                "attestation-policy declares recording delegation {delegation} for decisionAcceptance but `delegatedAcceptanceSubstanceRule` is not declared — the delegation's substance check is NOT ADOPTED (D0192)"
            )),
            Err(e) => report.violations.push(format!("error reading delegatedAcceptanceSubstanceRule: {e}")),
        }
    }
    // D0198 OPTION A (quote receipts): the same contract pattern for confirmation FLIPS — when the
    // policy declares the confirmationRecord recording delegation, a human-judged flip after the
    // cutoff must quote the human itself or carry a companion `<test>Attest<N>` record that does.
    if let Some(delegation) = crate::activation::recording_delegation(root, "confirmationRecord") {
        match crate::view::rule_violations_opt(root, "delegatedConfirmationSubstanceRule") {
            Ok(Some((_, bad))) => {
                for d in bad {
                    report.violations.push(format!(
                        "{d}: a human-judged confirmation flip after the delegation cutoff carries no quote receipt — neither its own text nor a companion <test>Attest<N> record quotes the human's words (D0198 {delegation})"
                    ));
                }
            }
            Ok(None) => report.warnings.push(format!(
                "attestation-policy declares recording delegation {delegation} for confirmationRecord but `delegatedConfirmationSubstanceRule` is not declared — the delegation's substance check is NOT ADOPTED (D0198)"
            )),
            Err(e) => report.violations.push(format!("error reading delegatedConfirmationSubstanceRule: {e}")),
        }
    }
    report
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
    crate::gitx::git()
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
        Ok(Some((scanned, gaps))) => {
            let violations = gaps
                .into_iter()
                .map(|c| format!("{c}: #Capability with no #DerivedFrom edge to a Need — state the driving Need (D0099)"))
                .collect();
            GuardReport { name: "requirement-rootedness", scanned, warnings: Vec::new(), violations }
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
    // D0184/p3aKeystoneExtension: a skill body or an activation/attestation policy IS process
    // definition — the keystone lock covers them exactly as it covers processes and workflows.
    // Contracts that are MACHINE STATE registries (unit-ids, installed-units) are import/export
    // bookkeeping, not process definition, and stay outside the lock.
    if p.starts_with(".engine/skills/")
        && (is_sysml(p) || std::path::Path::new(p).extension().is_some_and(|e| e.eq_ignore_ascii_case("md")))
    {
        return true;
    }
    if p.starts_with(".engine/contracts/")
        && !p.ends_with("unit-ids.toml")
        && !p.ends_with("installed-units.toml")
        && !p.ends_with("deck-inbox.toml")
        && !p.ends_with("engine-version.toml") // D0190: machine-stamped by init/migrate, instance data
        && !p.ends_with("adoption-profile.toml")
    {
        return true;
    }
    // issue236 (D0208 control evaluation, HIGH): `.engine/rules/` holds the DECLARED ElementRule/
    // EdgeRule instances the guards enforce — downgrading a rule's severity from blocking to warning
    // silently disarms a control, and the evaluation proved that edit passed with no guard firing
    // (the monitor was modifiable by the monitored). Rules are enforcement DEFINITION, so the
    // keystone lock covers them: any `.engine/rules/*.sysml` edit needs a co-committed marked
    // Decision, i.e. a human-signed act — the panel's self-modification gap, closed.
    is_sysml(p)
        && (p.starts_with(".engine/processes/")
            || p.starts_with(".engine/workflows/")
            || p.starts_with(".engine/rules/"))
}

/// The files that DEFINE the enforcement LOGIC (every `-> GuardReport` guard plus the audit-adherence
/// gate). Kept identical to the real set by `enforcement_surface_covers_every_guard_source` — that
/// test fails CI if a new guard-defining file appears outside this list, which is the D0209-clause-2
/// "diff `is_process_def` against the actual guard-definition paths" audit made executable.
const GUARD_SOURCE_FILES: &[&str] = &["keel-cli/src/guards.rs", "keel-cli/src/adherence.rs"];

/// The ENFORCEMENT SURFACE (D0209 clause 2, dcFreezeEnforcementSurface): the paths a guard reads its
/// own DEFINITION or CONFIG from. issue236 proved a control could be silently disarmed by editing its
/// definition; `.engine/rules/` (in `is_process_def`) closed the DECLARED-rule leg, and this closes
/// the rest — the guard SOURCE, the local hook CONFIG, and the CI WORKFLOW files that run the gates on
/// infra the agent cannot touch. A change to any of them needs a co-committed human-signed marked
/// Decision, exactly like a process definition. Kept SEPARATE from `is_process_def` so the two intents
/// stay legible and the coverage audit can diff this set against the real guard sources.
fn is_enforcement_surface(p: &str) -> bool {
    // CI workflow files — where audit-adherence / audit-history / the keel gates actually run.
    if p.starts_with(".github/workflows/")
        && std::path::Path::new(p)
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("yml") || e.eq_ignore_ascii_case("yaml"))
    {
        return true;
    }
    // Local git hooks — the per-commit / per-merge / per-push gate wiring.
    if p.starts_with(".githooks/") {
        return true;
    }
    // Guard SOURCE — the enforcement logic itself.
    GUARD_SOURCE_FILES.contains(&p)
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
    // The keystone covers process DEFINITION (D0070) AND the ENFORCEMENT SURFACE (D0209 clause 2):
    // guard source, hook config, CI workflows. A change to either needs a co-committed marked Decision.
    let mut locked: Vec<&str> = changed
        .iter()
        .map(String::as_str)
        .filter(|p| is_process_def(p) || is_enforcement_surface(p))
        .collect();
    locked.sort_unstable();
    if locked.is_empty() {
        return Vec::new(); // nothing under the lock changed — guard is silent
    }
    let marked = decision_texts.iter().any(|(p, t)| is_decision_file(p) && has_process_marker(t));
    if marked {
        return Vec::new();
    }
    vec![format!(
        "locked file(s) changed ({}) with NO co-committed process-change Decision (a #ProspectiveChange/#SafetyChange-marked .engine/decisions/*.sysml). HARD LOCK: process definitions (D0070) AND the enforcement surface — guard source, hook config, CI workflows (D0209 clause 2) — may change only with a human-signed Decision, because a silently self-modified control is the issue236 self-modification class. Record one with `keel record decision --process-change ...` (the flag emits the marker; issue213).",
        locked.join(", ")
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
    let scanned = changed.iter().filter(|p| is_process_def(p) || is_enforcement_surface(p)).count();
    // D0204 (pullOversight): the short-lived D0200 clause-5 throttle - refusing process-def commits
    // while the human's review queue aged - is deliberately ABSENT. Nothing gates the AI's work on
    // the human's attention cadence; the records stay auditable, the ask is gone.
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
    match crate::view::dangling_edge_endpoints_scanned(root) {
        Ok((scanned, bad)) => {
            let violations = bad
                .into_iter()
                .map(|e| format!("{e} — a typed edge must connect two declared items; declare the item or remove the edge, never repoint it at something convenient"))
                .collect();
            GuardReport { name: "edge-endpoints", scanned, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport { name: "edge-endpoints", scanned: 0, warnings: Vec::new(), violations: vec![format!("error resolving edge endpoints: {e}")] },
    }
}

/// Guard: no comment may claim a gate is PENDING when its acceptance result PASSES (issue140).
///
/// Prose frozen at authoring time contradicting the record beside it is the D0018 defect inside the model
/// files, and it is not harmless: `keel-viewer.sysml` carried `PENDING human acceptance of N-18 — NOT yet
/// accepted` on the line ABOVE N-18's passing acceptance result, and I believed the comment over the
/// record and published a false claim in a critique. A reader has no reason to distrust a comment sitting
/// next to the thing it describes.
///
/// PRECISE, not heuristic: fires only when a comment within three lines of a PASSING `*Accept*R*`
/// `TestResult` claims the opposite. A comment saying PENDING beside a gate that has NOT been signed is
/// correct and is left alone, which is what keeps this from firing on honest work-in-progress.
#[must_use]
pub fn stale_gate_prose(root: &Path) -> GuardReport {
    const CLAIMS: [&str; 3] = ["PENDING", "NOT yet accepted", "proposed/unaccepted"];
    let mut violations = Vec::new();
    let mut scanned = 0;
    for dir in [".tracking", ".engine", ".knowledge"] {
        for f in crate::collect_sysml(&root.join(dir)) {
            let Ok(text) = std::fs::read_to_string(&f) else { continue };
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                // A passing acceptance RESULT: the record a nearby comment must not contradict.
                let is_pass_result = line.contains(": TestResult") && line.contains("VerdictKind::pass") && line.contains("Accept");
                if !is_pass_result {
                    continue;
                }
                scanned += 1;
                let lo = i.saturating_sub(3);
                let hi = (i + 4).min(lines.len());
                // `.get` rather than a slice: a panic inside a GUARD would take the whole gate down and
                // report nothing, which is the worst possible failure mode for a control.
                for probe in lines.get(lo..hi).unwrap_or(&[]) {
                    let t = probe.trim_start();
                    if !t.starts_with("//") {
                        continue;
                    }
                    if let Some(claim) = CLAIMS.iter().find(|c| t.contains(**c)) {
                        violations.push(format!(
                            "{}:{}: a comment says `{claim}` within three lines of a PASSING acceptance result — the prose contradicts the record it sits beside, and a reader has no reason to distrust it (issue140). State what IS; delete the stale note rather than annotating it",
                            relpath(root, &f),
                            i + 1
                        ));
                        break;
                    }
                }
            }
        }
    }
    violations.sort();
    violations.dedup();
    GuardReport { name: "stale-gate-prose", scanned, warnings: Vec::new(), violations }
}

/// Guard: a `#Resolves` resolver must be WORK or a mooting `Decision` (D0077/issue136).
///
/// `guard issues` checks only that an Issue HAS a resolving edge, and its own message already says the
/// resolver should be "a resolving action or Decision" — unchecked, so triage passed on an edge pasted
/// from the line above it, pointing an unrelated `SystemRequirement` at an Issue about a flag parser.
/// A nominally-triaged Issue is worse than an untriaged one: it reports as handled.
///
/// Valid resolvers: a declared `action` (work that will close it), or a `Decision` (which moots it —
/// "we won't do X" is a first-class resolution, §1.4). A requirement, Need, Test or Story cannot resolve
/// anything: none of them is an act, so none can ever compute as complete against the Issue.
///
/// A BESPOKE PREDICATE, and D0107 is the precedent that makes that the right call rather than a
/// shortcut: this cannot be an `EdgeRule`, because `objectType` filters by declared item TYPE and an
/// `action` is not a typed element — the 127 legitimate action resolvers would all fail. Extending the
/// rule language to express "action OR Decision" is the larger change; the constraint is checked here
/// meanwhile, and the rule keeps `objectType = "*"` because that is honestly all it can say.
#[must_use]
pub fn resolver_kind(root: &Path) -> GuardReport {
    let actions = declared_task_names(root);
    match crate::view::resolves_edges(root) {
        Ok(edges) => {
            let scanned = edges.len();
            let violations = edges
                .into_iter()
                .filter(|(from, _, ty)| !actions.contains(from) && ty != "Decision")
                .map(|(from, to, ty)| {
                    let what = if ty.is_empty() { "not a declared action and not a typed item".to_string() } else { format!("a {ty}") };
                    format!(
                        "{to}: #Resolves comes from {from}, which is {what} — a resolver must be a declared action (work that closes it) or a Decision (which moots it); \
                         until it is, the Issue reports as TRIAGED while nothing is on the hook for it (issue136)"
                    )
                })
                .collect();
            GuardReport { name: "resolver-kind", scanned, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport { name: "resolver-kind", scanned: 0, warnings: Vec::new(), violations: vec![format!("error reading #Resolves edges: {e}")] },
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
    "concern-coverage", "dispositions", "sitting-coverage", "critique-policy", "rootedness", "tier-satisfaction", "recent", "verification",
    "authority-queue", // real since D0129 sync work; never added here, so the first viewpoint naming it failed
    "arch", // D0148 — the six `arch` views register as viewpoints; the group name is what `keel <cmd>` matches
];

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
    // EVERY declared Viewpoint, not just the registry file's (issue139). This guard read one hardcoded
    // filename while the model saw them all, so a viewpoint declared elsewhere never had its renderer
    // checked — proven with a probe whose renderer named no command and which the guard did not see.
    let vps = match crate::view::declared_viewpoints(root) {
        Ok(v) => v,
        Err(e) => return GuardReport { name: "viewpoint-renderer", scanned: 0, warnings: Vec::new(), violations: vec![format!("cannot enumerate viewpoints: {e}")] },
    };
    let mut scanned = 0;
    let mut warnings = Vec::new();
    let mut violations = Vec::new();
    // A DEACTIVATED viewpoint's renderer is not required to resolve (D0164), the same way a deactivated
    // process's guards are skipped: a project that declared it does not look through this lens has not
    // violated a rule about the lens. Reported in `keel activation`, never silent.
    let act = crate::activation::Activation::load(root);
    for vp in vps {
        if !act.is_viewpoint_active(&vp.name) {
            continue;
        }
        // A Viewpoint with NO renderer is a declared lens nothing can render, so it is judged rather
        // than skipped — the previous text scan only ever saw viewpoints that had the line at all.
        let r = vp.renderer;
        let label = if vp.title.is_empty() { vp.name } else { vp.title };
        scanned += 1;
        if r.trim().is_empty() {
            violations.push(format!("{label}: viewpoint declares NO renderer — a lens nothing can render is a concern claimed and not served"));
            continue;
        }
        match classify_renderer(&r) {
            "retired" => violations.push(format!("{label}: renderer references a RETIRED tool (query.py/report.py, D0074) — '{r}'")),
            "unknown" => violations.push(format!("{label}: renderer names no known keel command — '{r}'")),
            "planned" => warnings.push(format!("{label}: viewpoint declared but renderer is planned/unbuilt — '{r}'")),
            _ => {}
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
    match crate::view::critical_independence_gaps_scanned(root) {
        Ok((scanned, gaps)) => {
            let violations = gaps
                .into_iter()
                .map(|e| format!("{e}: target of a Critical-severity finding but has only aiModel critiques — requires a human/tool critic (D0080 independence, issue031)"))
                .collect();
            GuardReport { name: "critic-independence", scanned, warnings: Vec::new(), violations }
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
/// The engine's own marker vocabulary, DERIVED from the `metadata def`s in the schema baked into
/// this binary — never restated as a literal list.
///
/// It was a hardcoded 17-entry list and had already fallen behind: `Controls`, `Feedback` and
/// `Restructure` shipped with the codeaudit module and were never added (issue120). Deriving keeps
/// the property this list exists for — the vocabulary travels WITH the binary, so upgrading the
/// binary against an older on-disk `.engine/` cannot produce the issue090 lockout — while removing
/// the second place that had to be remembered.
#[must_use]
pub fn engine_markers() -> &'static HashSet<String> {
    static M: std::sync::LazyLock<HashSet<String>> =
        std::sync::LazyLock::new(|| crate::schema::VOCAB.markers.clone());
    &M
}

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
    let mut out: HashSet<String> = engine_markers().clone();
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
    // THE CO-STAGED ITEM MUST BE TIED TO THE FINDING, NOT TO THE COMMIT (issue189 / D0172 step 4).
    // This function used to return empty whenever the commit staged issues.sysml or backlog.sysml AT
    // ALL - so a sprint that recorded its own unrelated findings satisfied the guard while its retro's
    // AVOIDABLE-ISSUE went untracked. Measured consequence: one failure class reached FIVE retros and
    // zero items, because every commit co-staged something. The exemption is now PER RETRO: the retro's
    // own text must NAME a tracked item (dcCamelCase or issueNNN) or carry a no-item justification.
    let _ = changed; // retained in the signature so existing callers and tests keep their shape
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
        // The retro text NAMES the item it produced: `dcXxx` (a backlog action) or `issueNNN`.
        if names_tracked_item(text) {
            continue;
        }
        out.push(format!(
            "{path}: the retro names a finding (AVOIDABLE-ISSUE / LESSON) but this commit records NO tracked Issue or backlog action, and gives no reason — a retro finding must become a tracked, prioritized item or say explicitly why it needs none (issue085; D0018 — never let a lesson terminate in prose)"
        ));
    }
    out
}

/// Does this retro text NAME a tracked item — `dcCamelCase` or `issueNNN`?
///
/// Word-boundary checked on both sides, because `producedX` must not satisfy `produced` and prose like
/// `reduced` must not match `dc`. No regex: the guard path stays dependency-light (D0048).
fn names_tracked_item(text: &str) -> bool {
    let bytes = text.as_bytes();
    let boundary =
        |i: usize| i.checked_sub(1).and_then(|j| bytes.get(j)).is_none_or(|b| !b.is_ascii_alphanumeric());
    let mut from = 0;
    while let Some(rel) = text[from..].find("dc") {
        let s = from + rel;
        // dc + UpperCamel, at a word boundary: a named backlog action.
        if boundary(s) && text[s + 2..].starts_with(|c: char| c.is_ascii_uppercase()) {
            return true;
        }
        from = s + 2;
    }
    let mut from = 0;
    while let Some(rel) = text[from..].find("issue") {
        let s = from + rel;
        if boundary(s) && text[s + 5..].starts_with(|c: char| c.is_ascii_digit()) {
            return true;
        }
        from = s + 5;
    }
    false
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
    let Ok(central) = std::fs::read_to_string(&reg_path) else {
        return GuardReport { name: "process-skill", scanned: 0, warnings: Vec::new(), violations: vec![format!("cannot read {}", relpath(root, &reg_path))] };
    };
    // D0220: a skill may declare its deployment BESIDE ITSELF, in any `.sysml` under
    // `.engine/skills/`, not only in the central registry. Adopting decision-channel on penumbra
    // proved why: the unit carried the SKILL.md but the registry ENTRY that binds skill->process
    // stayed home, so `process-skill` failed in the receiving project on the very first run - a new
    // project's first experience of an adopted unit was a red gate. Reading the whole directory lets
    // a unit ship its own registration as a file, so nothing has to be text-merged into a shared
    // registry (the hazard the rules layer already avoids by carrying rules BY NAME).
    let mut reg = central;
    for f in crate::collect_sysml(&root.join(".engine").join("skills")) {
        if f == reg_path {
            continue;
        }
        if let Ok(extra) = std::fs::read_to_string(&f) {
            reg.push('\n');
            reg.push_str(&extra);
        }
    }
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
/// package names. Returns `(warnings, violations)`.
///
/// The warnings channel is retained but now always empty for ids: issue080's 18 bootstrap duplicates
/// were re-identified by a D0067 migration, so there is no exemption path left and every duplicate
/// fails. The tuple shape is kept because the item-name and package-name scans share this function.
fn duplicate_scan(files: &[(String, String)]) -> (Vec<String>, Vec<String>) {
    let mut violations = Vec::new();
    let warnings = Vec::new();
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
                    // No grandfather list any more (issue080 RESOLVED): the 18 bootstrap duplicates
                    // across 26 records were re-identified by a D0067 migration, so every duplicate
                    // from here is a live corruption and fails. Keeping an empty exemption list
                    // around would be an invitation to refill it.
                    violations.push(format!(
                        "{loc}: duplicate element id \"{id}\" (also at {prev}) — identity is the invariant that lets items share a name (§2.3); a collision corrupts it"
                    ));
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

// ── doc-guard-count guard (issue246: a count typed into prose beside a computable one) ──────────

/// Guard: the doc surface must not hardcode the TOTAL guard count.
///
/// issue246. `CLAUDE.md` said "all 45 forward guards" and `.engine/docs/guards.md` said "runs **45**
/// forward guards" while 48 ran — and guards.md's very next sentence said `keel version` reports the
/// split "so the number has one home". A number typed into prose beside a number the engine can
/// compute is the highest-frequency drift mechanism in this repository, and the 2026-08-24 panel found
/// the brief that DIAGNOSED report-vs-truth drift committing it twice itself.
///
/// The rule is DELETION, not reconciliation: D0105 gives every fact one canonical home, and the home
/// of this one is `keel version`. Syncing 45 to 48 would drift again on guard 49; forbidding the
/// literal cannot. Narrow by construction — it fires only on a digit immediately preceding
/// "forward guards" or on "all N guards", so a SUBSET count ("5 guards are rule-sourced") and an
/// ordinal reference ("Guard 37 checks...") are untouched.
#[must_use]
pub fn doc_guard_count(root: &Path) -> GuardReport {
    let mut files: Vec<PathBuf> = vec![root.join("CLAUDE.md")];
    if let Ok(rd) = std::fs::read_dir(root.join(".engine").join("docs")) {
        files.extend(rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "md")));
    }
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    let actual = GUARD_NAMES.len();
    for f in files {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        scanned += 1;
        let rel = f.strip_prefix(root).unwrap_or(&f).to_string_lossy().replace('\\', "/");
        for (n, line) in text.lines().enumerate() {
            if let Some(claim) = total_guard_count_claim(line) {
                violations.push(format!(
                    "{rel}:{}: states a TOTAL guard count (`{claim}`) while {actual} guards are enforced - the count has ONE home, `keel version` (D0105/issue246). Delete the number; point at the computed source.",
                    n + 1
                ));
            }
        }
    }
    GuardReport { name: "doc-guard-count", scanned, warnings: Vec::new(), violations }
}

/// The hardcoded TOTAL-count phrase in a line, if any. `None` for subset counts and ordinals.
fn total_guard_count_claim(line: &str) -> Option<String> {
    // "<digits>[**] forward guards" — the digits may be wrapped in markdown emphasis.
    for pat in ["forward guards", "guards, kernel-free"] {
        if let Some(i) = line.find(pat) {
            let before: String = line[..i].chars().rev().take(12).collect::<String>().chars().rev().collect();
            let stripped: String = before.chars().filter(|c| *c != '*' && *c != '_').collect();
            if stripped.trim_end().chars().last().is_some_and(|c| c.is_ascii_digit()) {
                return Some(format!("{}{pat}", before.trim_start()));
            }
        }
    }
    // The "all <N> guards" branch was REMOVED after a probe against the real corpus: guards.md
    // legitimately narrates history ("passed validate and all 37 guards"), a TRUE statement about the
    // past that must not be forbidden. Only the canonical CURRENT-total phrasing is checked, which is
    // what both drifted sites actually used. A guard that fires on true prose gets bypassed.
    None
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
pub const GUARD_NAMES: [&str; 50] =
    ["process-applicability", "doc-guard-count", "actors", "acceptance-events", "sprint-coverage", "ceremony", "charter", "process-change", "issues", "viewpoint-renderer", "manifest-coverage", "critic-independence", "process-skill", "requirement-rootedness", "decision-rationale", "attestation-substance", "marker-vocabulary", "duplicate-identity", "decision-requirement-link", "verification-trace", "priority-inversion", "retro-backlog", "confirmation-authenticity", "engine-lint", "doc-sync", "hook-config-integrity", "activation-manifest", "sequence-multiplicity", "parser-coverage", "base-first-justification", "edge-endpoints", "ownership", "attestation-authority", "type-collision", "attribute-vocabulary", "resolver-kind", "stale-gate-prose", "impossible-evidence-date", "identity-present", "identity-well-formed", "tool-reference", "scaffold-placeholder", "claude-surface-drift", "decision-scaffolding", "release-recorded", "enrollment-binding", "control-event-coverage", "question-coverage", "claim-ancestry", "judgment-request-quality"];


// ── type-collision guard (userDefinedTypedefs, D0128) ────────────────────────

/// A `<kind> def <Name>` declaration line's name.
fn declared_type_name(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let t = trimmed.strip_prefix("abstract ").unwrap_or(trimmed);
    let mut it = t.split_whitespace();
    let first = it.next()?;
    if !first.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    let mut next = it.next()?;
    if next == "case" {
        next = it.next()?; // `use case def X`
    }
    if next != "def" {
        return None;
    }
    let name = it.next()?;
    let name = name.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
    (!name.is_empty() && name.chars().next().is_some_and(char::is_alphabetic)).then_some(name)
}

/// Guard: a PROJECT type must not shadow an ENGINE type (D0128 userDefinedTypedefs).
///
/// A project declaring its own domain types in `.tracking/` is supported and wanted — that is the
/// whole point of userDefinedTypedefs, and it already resolves. What is NOT safe is a project
/// declaring a name the engine already defines. Measured before building this: a project
/// `part def Story :> Element` validates CLEAN today and passes every guard, while `Story` is the
/// type `orient` counts work by. Whichever definition wins, the other is silently ignored, and a
/// computed view starts counting something other than what the reader believes — with no diagnostic
/// anywhere. Silence is the defect; the collision itself is easy to fix once seen.
///
/// HARD, and it starts at zero: 91 engine defs against 305 project defs in this repo produce no
/// collision today, so nothing is grandfathered and nothing needs to be (issue068 protects work that
/// was correct when written; there is none to protect here).
///
/// Names only, deliberately. Whether the project MEANT to extend or to replace the engine type is
/// not decidable from the text, and a guard that guessed would be wrong in one direction or the
/// other. Reporting the shadow and letting the author rename is exact.
#[must_use]
pub fn type_collision(root: &Path) -> GuardReport {
    let mut engine: HashMap<String, String> = HashMap::new();
    for path in crate::collect_sysml(&root.join(".engine").join("schema")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
        for (i, line) in text.lines().enumerate() {
            if let Some(n) = declared_type_name(line) {
                engine.entry(n.to_owned()).or_insert_with(|| format!("{rel}:{}", i + 1));
            }
        }
    }
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for path in crate::collect_sysml(&root.join(".tracking")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
        for (i, line) in text.lines().enumerate() {
            let Some(n) = declared_type_name(line) else { continue };
            scanned += 1;
            if let Some(where_engine) = engine.get(n) {
                violations.push(format!(
                    "{rel}:{}: project type `{n}` SHADOWS the engine type declared at {where_engine} — whichever definition wins, the other is silently ignored and a computed view starts counting something other than what the reader believes. Rename the project type (D0128: projects extend the model, they do not redefine it)",
                    i + 1
                ));
            }
        }
    }
    GuardReport { name: "type-collision", scanned, warnings: Vec::new(), violations }
}

// ── ownership + attestation authority (D0129 srDcAuthorityFromRegistry; mechanizes D0108) ────────

/// Item name -> (`createdBy`, its attribute assignments) parsed from one `.sysml` source.
///
/// Deliberately AST-based rather than diff-line based: a diff hunk does not know which item a
/// changed line belongs to, and guessing from indentation would misattribute an edit — the one
/// thing an ownership check must never do.
/// Is a non-owner diff exactly the sanctioned ACCEPT TRANSFORM (D0205) — the same attribute set
/// with only `status` moving from proposed to accepted?
fn is_accept_transform(old_attrs: &[String], new_attrs: &[String]) -> bool {
    if old_attrs.len() != new_attrs.len() {
        return false;
    }
    let mut status_flip = false;
    let old_set: std::collections::HashSet<&String> = old_attrs.iter().collect();
    let new_set: std::collections::HashSet<&String> = new_attrs.iter().collect();
    for gone in old_set.difference(&new_set) {
        if gone.starts_with("status=") && gone.contains("proposed") {
            status_flip = true;
        } else {
            return false; // some other attribute changed — not the sanctioned transform
        }
    }
    for came in new_set.difference(&old_set) {
        if !(came.starts_with("status=") && came.contains("accepted")) {
            return false;
        }
    }
    status_flip
}

fn items_with_attrs(src: &str, filename: &str) -> HashMap<String, (String, Vec<String>)> {
    let mut out = HashMap::new();
    let Ok(tokens) = keel_parser::tokenize(src, filename) else { return out };
    let Ok(pkg) = keel_parser::parse(tokens, filename) else { return out };
    let mut note = |name: &str, attrs: &[keel_parser::ast::Attribute]| {
        let mut pairs: Vec<String> = attrs
            .iter()
            .map(|a| format!("{}={}", a.name, crate::view::attr_value_string(&a.value)))
            .collect();
        pairs.sort();
        let created_by = attrs
            .iter()
            .find(|a| a.name == "createdBy")
            .map(|a| crate::view::attr_value_string(&a.value))
            .unwrap_or_default();
        out.insert(name.to_owned(), (created_by, pairs));
    };
    for item in &pkg.items {
        match item {
            keel_parser::ast::Item::Part(p) => note(&p.name, &p.attributes),
            keel_parser::ast::Item::Verification(v) => note(&v.name, &v.attributes),
            keel_parser::ast::Item::UseCase(u) => note(&u.name, &u.attributes),
            keel_parser::ast::Item::ActionUsage(a) => note(&a.name, &a.attributes),
            // Items nested inside a delivery `action def` — where every sprint's gates live, and so
            // the densest concentration of owned fields in the model.
            keel_parser::ast::Item::ActionDef(d) => {
                for p in &d.parts {
                    note(&p.name, &p.attributes);
                }
                for v in &d.verifications {
                    note(&v.name, &v.attributes);
                }
            }
            _ => {}
        }
    }
    out
}

/// The file's content at HEAD, or `None` if it is newly added.
fn head_blob(root: &Path, path: &str) -> Option<String> {
    let out = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["show", &format!("HEAD:{path}")])
        .output()
        .ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Guard (D0108/D0129): only an item's OWNER edits its fields.
///
/// D0108's coordination contract — owner-of-record edits fields, a non-owner may ADD items and typed
/// edges or SUPERSEDE, never overwrite in place — was CONVENTION ONLY: absent from every guard and
/// every declared rule, enforced by prose plus a reminder hook. D0047 is explicit that manual
/// vigilance is not a control, and in-place write paths exist that can clobber another actor's item.
///
/// WHAT PASSES, deliberately: adding a new item, adding a typed edge, and superseding all leave every
/// existing item's fields untouched, so they are invisible to this check by construction rather than
/// by an exemption list that could drift. Only a CHANGED attribute on an item someone else created
/// is a violation.
///
/// Compares the staged file against its HEAD blob AST-to-AST. A diff hunk does not know which item a
/// line belongs to; attributing an edit to the wrong owner would be worse than not checking.
#[must_use]
pub fn ownership(root: &Path) -> GuardReport {
    let actor = crate::actor::resolve(root, None).ok();
    let all_staged = staged_files(root);
    // A GOVERNED MIGRATION is the sanctioned exception, and it needs one because these two controls
    // genuinely collide: D0108 forbids a non-owner editing another actor's fields, while D0067
    // REQUIRES bulk transforms that cross every ownership boundary at once (repairing 26 duplicated
    // ids is exactly that). Blocking a migration would have made D0067 unexecutable; exempting on a
    // flag would have made D0108 optional.
    //
    // The exemption is therefore the same keystone shape `process-change` already uses: the change
    // is permitted only when a transform under `.engine/tools/migrations/` is CO-COMMITTED, which is
    // what D0067 demands anyway (a committed transform, a dry run, reconciled control totals). It
    // cannot be claimed — it has to be in the commit, where a reviewer can read it.
    let migration_co_committed = all_staged.iter().any(|p| p.starts_with(".engine/tools/migrations/"));
    if migration_co_committed {
        return GuardReport {
            name: "ownership",
            scanned: 0,
            warnings: vec![
                "cross-owner edits ALLOWED: a migration transform under .engine/tools/migrations/ is co-committed (D0067). Ownership (D0108) is suspended for this commit and the transform is the record of why.".to_owned(),
            ],
            violations: Vec::new(),
        };
    }
    let staged: Vec<String> = all_staged.into_iter()
        .filter(|p| std::path::Path::new(p).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml")))
        .collect();
    let mut violations = Vec::new();
    let mut scanned = 0usize;
    for path in &staged {
        let Some(before) = head_blob(root, path) else { continue }; // newly added file — all additions
        let Ok(after) = std::fs::read_to_string(root.join(path)) else { continue };
        let old = items_with_attrs(&before, path);
        let new = items_with_attrs(&after, path);
        for (name, (owner, new_attrs)) in &new {
            let Some((old_owner, old_attrs)) = old.get(name) else { continue }; // added item
            scanned += 1;
            if old_attrs == new_attrs {
                continue;
            }
            // D0205 (githubChannel): the ACCEPT TRANSFORM is the one sanctioned non-owner edit — a
            // recording channel (the GitHub Action, the serve endpoint) flips a Decision's status
            // from proposed to accepted on the human's authenticated gesture. Recognized MECHANICALLY:
            // the ONLY attribute that changed is `status`, exactly proposed -> accepted. Anything
            // else a non-owner touches (title, rationale, a second attr riding along) still violates.
            // The acceptance EVENT items the same write appends are ADDS and were always permitted.
            if is_accept_transform(old_attrs, new_attrs) {
                continue;
            }
            let owner = if old_owner.is_empty() { owner } else { old_owner };
            if owner.is_empty() {
                continue; // no recorded owner — nothing to enforce against, and inventing one is worse
            }
            match &actor {
                Some(a) if a == owner => {}
                Some(a) => violations.push(format!(
                    "{path}: '{name}' is owned by '{owner}' and its fields were edited by '{a}' — D0108: a non-owner ADDS items and typed edges or SUPERSEDES, never overwrites in place. Author a superseding item, or have the owner make the change."
                )),
                None => violations.push(format!(
                    "{path}: '{name}' (owned by '{owner}') had fields edited, but this machine has no bound actor, so the edit cannot be attributed. Run `keel actor set <id>` — provenance is never defaulted (D0129)."
                )),
            }
        }
    }
    GuardReport { name: "ownership", scanned, warnings: Vec::new(), violations }
}

/// Guard (D0092/D0106/D0129): a human-only attestation must be judged by a HUMAN.
///
/// `confirmation-authenticity` already enforces this for Decision ACCEPTANCE. This extends the same
/// rule to the other authority D0129 names: DISPOSITION of a finding at or above the threshold.
///
/// THE THRESHOLD IS LOAD-BEARING AND WAS ALMOST GOT WRONG. D0080 explicitly permits an AI to
/// disposition a LOW finding — this repo contains exactly such a case, `issue043Disp1R1` judged by
/// `claudeOpus`, whose own text says "Low doc-accuracy finding, AI-dispositioned (no human gate for
/// Low, D0080)". A guard requiring a human on every disposition would have failed a correct,
/// documented judgement and forced either a false attestation or a bypass. Medium and above only.
/// The threshold stays in the guard rather than in the policy file because it is a property of the
/// FINDING, not of the actor — the contract answers "who may attest", not "what needs attesting".
///
/// D0146: the check is now against the DECLARED policy (kind AND role) rather than a hardcoded
/// `is a Person`, which is what srDcAuthorityFromRegistry asks for. An absent contract falls back to
/// human-only, so deleting the file cannot disable the check.
#[must_use]
pub fn attestation_authority(root: &Path) -> GuardReport {
    match crate::view::ai_judged_high_dispositions(root) {
        Ok((scanned, bad)) => {
            let violations = bad
                .into_iter()
                .map(|(disp, issue, gap)| format!(
                    "{disp}: dispositions '{issue}' (>= Medium) but its judge does not satisfy the declared authority policy — {gap}. See .engine/contracts/attestation-policy.toml [findingDisposition] (D0092/D0146)."
                ))
                .collect();
            GuardReport { name: "attestation-authority", scanned, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport {
            name: "attestation-authority",
            scanned: 0,
            warnings: Vec::new(),
            violations: vec![format!("error reading dispositions: {e}")],
        },
    }
}


/// Guard (issue144): a judgment may not cite a commit that postdates it.
///
/// HARD, and it is squarely an honest-state gate (D0098): it does not ask whether work is finished, it
/// asks whether a recorded judgment is POSSIBLE. It exists because I stamped a human's
/// `method=confirmation` result — given the day before — against a commit created today, by running a
/// blanket replace of every `PENDING` SHA in a file. Every field remained well-formed, `attestation-*`
/// and `confirmation-authenticity` both passed, and the record silently claimed the human had attested
/// something at a commit they had never seen. §4 forbids fabricating an attestation; this is the control
/// for fabricating one MECHANICALLY, which no amount of care about the original recording prevents.
#[must_use]
pub fn impossible_evidence_dates(root: &Path) -> GuardReport {
    match crate::view::impossible_evidence_dates(root) {
        Ok((scanned, violations)) => {
            GuardReport { name: "impossible-evidence-date", scanned, warnings: Vec::new(), violations }
        }
        Err(e) => GuardReport {
            name: "impossible-evidence-date",
            scanned: 0,
            warnings: Vec::new(),
            violations: vec![format!("error reading results: {e}")],
        },
    }
}

/// Guard (issue166): every id-bearing declaration actually carries an `:>> id`.
///
/// HARD, and it closes an invariant that was unguarded. §1.3 makes identity an immutable UUID so items
/// never collide on name — and `keel validate` passed with an `Issue` missing its `id` entirely. The only
/// existing coverage was `engine-lint`, which is `.engine`-scoped by design, and the demoted python
/// tracking validator (D0132), which fails correct files and so cannot be relied on. `duplicate-identity`
/// catches two items SHARING an id and says nothing about an item having none.
///
/// A TEXT SCAN, not a model walk: an item with no identity is exactly the thing the model layer cannot
/// see clearly, and this needs to run at commit speed. Measured at ~60ms over 8738 declarations, against
/// a 156ms model build it does not perform.
///
/// STARTS AT ZERO with no grandfather line, because the corpus was measured first: 8738 id-bearing
/// declarations across `.tracking` and `.engine`, none missing an id. A forward-only exemption would have
/// been ceremony over an empty set.
#[must_use]
pub fn identity_present(root: &Path) -> GuardReport {
    let mut files = crate::collect_sysml(&root.join(".tracking"));
    files.extend(crate::collect_sysml(&root.join(".engine")));
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let rel = relpath(root, path);
        let decls = id_bearing_decls(&text);
        for (name, ty, line, body) in decls {
            scanned += 1;
            if !body.contains(":>> id") {
                violations.push(format!(
                    "{rel}:{line}: {name} : {ty} carries NO `:>> id` - identity is an immutable UUID (section 1.3); an item without one cannot be referenced, superseded or attested against"
                ));
            }
        }
    }
    GuardReport { name: "identity-present", scanned, warnings: Vec::new(), violations }
}

#[must_use]
/// Guard 38: an `id` must be SHAPED like a UUID — 8-4-4-4-12 of `[0-9a-z]` (issue170/D0168).
///
/// Guard 37 checks an id is PRESENT and `duplicate-identity` checks two items do not SHARE one. The
/// middle property — that the string is an identifier at all — was enforced by nothing, and an id of
/// literally `not-a-uuid-at-all` passed `keel validate` and all 37 guards. A malformed id is still
/// UNIQUE, so it collides with nothing and every view resolves it happily; the damage is silent and
/// surfaces only when something outside this repo tries to join on it.
///
/// SHAPE, NOT STRICT HEX, and the corpus is why: 78 ids deliberately carry a mnemonic suffix
/// (`…-000000000i01` for the intake process steps), which is UUID-shaped but not hexadecimal. Those are
/// intentional and readable, and a guard that failed them would be demanding a migration nobody asked
/// for. Shape catches every real defect — the two mangled ids that prompted this, and the 15 historical
/// ones below — while leaving a deliberate convention alone.
///
/// THE GRANDFATHER SET IS AN EXPLICIT LIST, not a date. Guard 36's first version keyed its exemption on
/// a date and thereby exempted the very defect it existed for. Fifteen named strings cannot absorb a
/// sixteenth: a new malformed id fails, no matter when it is written. They are not REWRITTEN because
/// section 1.3 makes identity immutable — an id is wrong here, and changing it would be a second wrong.
pub fn identity_well_formed(root: &Path) -> GuardReport {
    /// Ids that predate the guard. Malformed (7- and 9-character first groups) and immutable.
    const GRANDFATHERED: [&str; 15] = [
        "be4dae8-5f6a-4b7c-def8-9a0b1c2d3e4f",
        "cf5ebl9-7b8c-4d9e-efa0-1c2d3e4f5a6b",
        "d0105r001-0001-4001-9001-516273841001",
        "d0105r002-0002-4002-9002-516273841002",
        "d0105r003-0003-4003-9003-516273841003",
        "d0105r004-0004-4004-9004-516273841004",
        "d0105r005-0005-4005-9005-516273841005",
        "d0105r006-0006-4006-9006-516273841006",
        "d0105r007-0007-4007-9007-516273841007",
        "d0105r008-0008-4008-9008-516273841008",
        "da6fcm0-8c9d-4e0f-fab1-2d3e4f5a6b7c",
        "da7gdp2-0e1f-4a2b-bcd3-4f5a6b7c8d9e",
        "eb5ebf9-6a7b-4c8d-efa9-0b1c2d3e4f5a",
        "eb8heq3-1f2a-4b3c-cde4-5a6b7c8d9e0f",
        "fc6fcn1-9d0e-4f1a-abc2-3e4f5a6b7c8d",
    ];
    let mut files = crate::collect_sysml(&root.join(".tracking"));
    files.extend(crate::collect_sysml(&root.join(".engine")));
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let rel = relpath(root, path);
        for (n, raw) in text.lines().enumerate() {
            let line = raw.trim_start();
            if line.starts_with("//") {
                continue;
            }
            for value in id_values(line) {
                scanned += 1;
                if uuid_shaped(&value) || GRANDFATHERED.contains(&value.as_str()) {
                    continue;
                }
                violations.push(format!(
                    "{rel}:{}: id \"{value}\" is not shaped like a UUID (8-4-4-4-12 of [0-9a-z]) - section 1.3 makes identity an immutable UUID, and a malformed id is still UNIQUE, so nothing else in the model will ever notice",
                    n + 1
                ));
            }
        }
    }
    GuardReport { name: "identity-well-formed", scanned, warnings: Vec::new(), violations }
}

/// Guard 39: a tool the LIVING doc surface references must EXIST (issue196).
///
/// Sprint 377's closeOut recorded the python deck generator as deleted while the file still sat in
/// `.engine/tools/` with two live references — a claimed deletion nobody ran `ls` against
/// (verify-the-wrong-surface, filesystem edition). Its first dry run found a SECOND stale reference
/// (a retired hook script still named in a skill). The checkable half is mechanical: every
/// `.engine/tools/<file>` mentioned in processes, skills, docs, or CLAUDE.md must resolve on disk.
///
/// SCOPE IS THE LIVING SURFACE ONLY — decisions and `.tracking` are historical records and may name
/// tools that no longer exist, truthfully. The no-tombstones rule applies to what this guard scans;
/// immutability applies to what it does not.
#[must_use]
pub fn tool_reference(root: &Path) -> GuardReport {
    fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(rd) = std::fs::read_dir(dir) else { return };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| {
                x.eq_ignore_ascii_case("md") || x.eq_ignore_ascii_case("sysml") || x.eq_ignore_ascii_case("toml")
            }) {
                out.push(p);
            }
        }
    }
    let mut files = Vec::new();
    for base in ["processes", "skills", "docs", "contracts", "workflows", "rules"] {
        walk(&root.join(".engine").join(base), &mut files);
    }
    let claude = root.join("CLAUDE.md");
    if claude.exists() {
        files.push(claude);
    }
    let needle = ".engine/tools/";
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    let mut reported = std::collections::BTreeSet::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let rel = relpath(root, path);
        for (n, line) in text.lines().enumerate() {
            let mut rest = line;
            while let Some(i) = rest.find(needle) {
                let tail = &rest[i..];
                let end = tail
                    .find(|c: char| !(c.is_ascii_alphanumeric() || "._/-".contains(c)))
                    .unwrap_or(tail.len());
                let mut tok = &tail[..end];
                while tok.ends_with('.') || tok.ends_with('-') {
                    tok = &tok[..tok.len() - 1];
                }
                rest = &tail[needle.len()..];
                // a bare directory mention carries no filename; only a file reference is checkable
                if !tok.rsplit('/').next().is_some_and(|f| f.contains('.')) {
                    continue;
                }
                scanned += 1;
                if !root.join(tok).exists() && reported.insert(tok.to_string()) {
                    violations.push(format!(
                        "{rel}:{}: references `{tok}`, which does not exist - a follower hits a dead path, and a claimed deletion that left references is a claim nobody verified",
                        n + 1
                    ));
                }
            }
        }
    }
    GuardReport { name: "tool-reference", scanned, warnings: Vec::new(), violations }
}

/// Guard 50: every declared process states the SITUATION in which a project needs it (D0225).
///
/// Onboarding recommends a process set by matching a project's elicited facts against each process's
/// `// APPLIES-WHEN:` condition. A process that declares none is invisible to that match — it can be
/// recommended neither for nor against — so the author's chartered set silently omits it and nobody
/// can tell the omission from a decision. That is the honest-state class (D0098): the guard does not
/// require the set to be COMPLETE, only that a process which exists can be reasoned about.
///
/// Beside the process rather than in a central table, because a central table cannot travel with one
/// unit — the defect that made 23 of 24 units land red on adoption (issue253/D0222).
fn process_applicability(root: &Path) -> GuardReport {
    let dir = root.join(".engine").join("processes");
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        // No processes directory at all is not a violation: a project may hold no process definitions.
        return GuardReport { name: "process-applicability", scanned, warnings: Vec::new(), violations };
    };
    let mut files: Vec<std::path::PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("sysml"))
        .collect();
    files.sort();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        // Only files that actually DECLARE a process are in scope - a helper or an include is not.
        if !text.contains(": Process {") {
            continue;
        }
        scanned += 1;
        if !text.lines().any(|l| l.trim().starts_with("// APPLIES-WHEN:")) {
            violations.push(format!(
                "{}: declares a Process but no `// APPLIES-WHEN:` condition - onboarding cannot recommend it for OR against, so a chartered set would omit it silently (D0225)",
                relpath(root, path)
            ));
        }
    }
    GuardReport { name: "process-applicability", scanned, warnings: Vec::new(), violations }
}

/// Guard 40: no `.sysml` in the model carries the scaffold's FILL-ME token (dcSprintScaffold).
///
/// `keel new sprint` writes every judgment-bearing text as [`crate::scaffold::PLACEHOLDER`] so the
/// skeleton is honest about being unfilled — and THIS guard is what makes that honesty enforceable:
/// an unfilled scaffold cannot pass a gate or be committed, by construction rather than diligence.
/// Also in the fast per-edit tier (`keel gate --fast`), so the rejection lands at edit time.
#[must_use]
pub fn scaffold_placeholder(root: &Path) -> GuardReport {
    let mut files = crate::collect_sysml(&root.join(".tracking"));
    files.extend(crate::collect_sysml(&root.join(".engine")));
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        scanned += 1;
        let rel = relpath(root, path);
        for (n, line) in text.lines().enumerate() {
            if line.contains(crate::scaffold::PLACEHOLDER) {
                violations.push(format!(
                    "{rel}:{}: unfilled scaffold text — fill it in; a placeholder is not a recorded judgment",
                    n + 1
                ));
            }
        }
    }
    GuardReport { name: "scaffold-placeholder", scanned, warnings: Vec::new(), violations }
}

/// Guard 41: the keel-owned `.claude/` enforcement surface matches this binary's generator
/// (D0174/P0.2). The check IS `keel sync-claude --check` — one implementation, one surface.
///
/// A project with NO `.claude/` directory has not adopted the in-loop surface and passes with a
/// note (CLI + commit/CI gates remain its enforcement; the D0186 harness-support matrix states
/// this). Version skew is a WARNING ("regenerate"), never a violation — the entries may be
/// semantically current under an older stamp. Drift in the keel-owned subset is a violation:
/// a mutated hook command is a silently weakened control (K7).
#[must_use]
pub fn claude_surface_drift(root: &Path) -> GuardReport {
    if !root.join(".claude").exists() {
        return GuardReport { name: "claude-surface-drift", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    }
    match crate::claude_surface::sync_claude(root, true) {
        Ok(r) => {
            let warnings = r
                .version_skew
                .map(|(old, new)| vec![format!("surface stamped by generator {old}, binary is {new} — run `keel sync-claude` (regenerate obligation)")])
                .unwrap_or_default();
            GuardReport {
                name: "claude-surface-drift",
                scanned: r.registry_count + 2, // settings.json + output style + the skills
                warnings,
                violations: r.drift,
            }
        }
        Err(e) => GuardReport {
            name: "claude-surface-drift",
            scanned: 0,
            warnings: Vec::new(),
            violations: vec![format!("cannot evaluate the surface: {e}")],
        },
    }
}

/// Guard 42: an accepted `#ProspectiveChange` Decision is reachable by a tracked-item edge (D0188).
///
/// WARNING-TIER by D0188's composed rule with D0180: promotion to hard is a recorded review citing
/// the fire-ledger evidence window, never a default. FORWARD-ONLY: the boundary is D0188's own
/// recorded acceptance date, read from the model (never hardcoded); the 64 historical gaps are not
/// retro-failed. THE LANDING-SPRINT GRACE: the most recently accepted violator is exempt — in this
/// repo's practice acceptance and the chartered work land together, but a multi-contributor project
/// may accept in one integration and charter in the next.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn decision_scaffolding(root: &Path) -> GuardReport {
    let mut files = crate::collect_sysml(&root.join(".tracking"));
    files.extend(crate::collect_sysml(&root.join(".engine")));
    let mut texts: Vec<(String, String)> = Vec::new();
    for f in &files {
        if let Ok(t) = std::fs::read_to_string(f) {
            texts.push((relpath(root, f), t));
        }
    }
    // The forward-only boundary: d0188's own acceptance date. Absent (downstream trees without the
    // decision) → the guard has no boundary and passes with zero scanned — adoption is by decision.
    let boundary = texts
        .iter()
        .find_map(|(_, t)| {
            let i = t.find("part d0188AcceptR")?;
            let j = t[i..].find("judgedAt = \"")? + i + "judgedAt = \"".len();
            t.get(j..j + 10).map(str::to_string)
        });
    let Some(boundary) = boundary else {
        return GuardReport { name: "decision-scaffolding", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    };
    // Accepted #ProspectiveChange decisions with their own acceptance dates.
    let mut candidates: Vec<(String, String)> = Vec::new(); // (decision, acceptedAt)
    for (_, t) in &texts {
        for line in t.lines() {
            let l = line.trim_start();
            let Some(rest) = l.strip_prefix("#ProspectiveChange part ") else { continue };
            let Some((name, tail)) = rest.split_once(':') else { continue };
            if !tail.trim_start().starts_with("Decision") {
                continue;
            }
            let name = name.trim().to_string();
            if !t.contains("DecisionStatus::accepted") {
                continue;
            }
            let accepted_at = t
                .find(&format!("part {name}AcceptR"))
                .and_then(|i| {
                    let j = t[i..].find("judgedAt = \"")? + i + "judgedAt = \"".len();
                    t.get(j..j + 10).map(str::to_string)
                })
                .unwrap_or_default();
            if !accepted_at.is_empty() && accepted_at.as_str() >= boundary.as_str() {
                candidates.push((name, accepted_at));
            }
        }
    }
    let scanned = candidates.len();
    // Reachability: any inbound tracked-item edge (charteredby/derivedfrom/resolves) or satisfy.
    let reachable = |d: &str| -> bool {
        let charter = "#CharteredBy dependency from ";
        texts.iter().any(|(_, t)| {
            t.lines().any(|l| {
                let l = l.trim_start();
                ((l.starts_with(charter) || l.starts_with("#DerivedFrom dependency from ") || l.starts_with("#Resolves dependency from "))
                    && l.trim_end().trim_end_matches(';').ends_with(&format!(" to {d}")))
                    || l.starts_with(&format!("satisfy {d} by "))
            })
        })
    };
    let mut bare: Vec<(String, String)> = candidates.into_iter().filter(|(d, _)| !reachable(d)).collect();
    // Landing-sprint grace: the newest violator by acceptance date is exempt.
    bare.sort_by(|a, b| a.1.cmp(&b.1));
    if !bare.is_empty() {
        bare.pop();
    }
    let warnings = bare
        .into_iter()
        .map(|(d, at)| {
            format!(
                "{d} (accepted {at}): an accepted #ProspectiveChange Decision with NO inbound tracked-item edge — it promises process change but charters no work (D0188). Add a #CharteredBy/#DerivedFrom/#Resolves edge from the item that delivers it, or record why none is needed."
            )
        })
        .collect();
    GuardReport { name: "decision-scaffolding", scanned, warnings, violations: Vec::new() }
}

/// Guard 43 (D0191, WARNING tier, owned by the `deploy` unit).
///
/// Every local version tag has a `Release` item naming it whose recorded commit matches the tag's commit — "a version was
/// recorded" and "the reconciled version matches the tag" were process-enforcement.toml's own
/// admitted checkable claims, unguarded until now. Zero tags scans zero and passes, so a project
/// without releases (or without the deploy unit) is untouched.
#[must_use]
pub fn release_recorded(root: &Path) -> GuardReport {
    let tags: Vec<String> = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .arg("tag")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with('v') && l[1..].starts_with(|c: char| c.is_ascii_digit()))
        .map(str::to_string)
        .collect();
    // issue244/D0214: a Release carrying an inbound `#Supersede` edge is RETIRED, exactly as the
    // sibling views already treat superseded Needs (issue088), requirements (issue127) and tasks
    // (issue100). Without this, a correction made through the engine's OWN sanctioned mechanism -
    // and the only one available to a non-owner (D0108) - could never clear the warning, so the
    // warning was unresolvable by construction and therefore permanent noise.
    let mut superseded: std::collections::HashSet<String> = std::collections::HashSet::new();
    for f in crate::collect_sysml(&root.join(".tracking")) {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        for line in text.lines() {
            let l = line.trim_start();
            if let Some(rest) = l.strip_prefix("#Supersede dependency from ") {
                if let Some((_, to)) = rest.split_once(" to ") {
                    superseded.insert(to.trim().trim_end_matches(';').trim().to_string());
                }
            }
        }
    }
    // Every authored Release block: (name, title, commit).
    let mut releases: Vec<(String, String, String)> = Vec::new(); // (name, title, commit)
    for f in crate::collect_sysml(&root.join(".tracking")) {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        let mut name = String::new();
        let mut title = String::new();
        let mut commit = String::new();
        let mut in_release = false;
        for line in text.lines() {
            let l = line.trim_start();
            if l.starts_with("part ") && l.contains(": Release {") {
                in_release = true;
                name = l.trim_start_matches("part ").split_whitespace().next().unwrap_or("").to_string();
                title.clear();
                commit.clear();
            }
            if in_release {
                if let Some(v) = l.strip_prefix(":>> title = \"") {
                    title = v.split('"').next().unwrap_or("").to_string();
                }
                if let Some(v) = l.strip_prefix(":>> commit = \"") {
                    commit = v.split('"').next().unwrap_or("").to_string();
                }
                if l.trim_end() == "}" {
                    in_release = false;
                    releases.push((name.clone(), title.clone(), commit.clone()));
                }
            }
        }
    }
    let mut warnings = Vec::new();
    for tag in &tags {
        let Some((_, _, recorded)) =
            releases.iter().filter(|(n, _, _)| !superseded.contains(n)).find(|(_, title, _)| title.contains(tag.as_str()))
        else {
            warnings.push(format!(
                "tag `{tag}` has NO Release item naming it — what shipped is not an authored fact (D0191; record it in .tracking/baselines.sysml)"
            ));
            continue;
        };
        let tag_commit = crate::gitx::git()
            .arg("-C")
            .arg(root)
            .args(["rev-parse", "--short", &format!("{tag}^{{commit}}")])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
        if !tag_commit.is_empty() && !recorded.is_empty() && !tag_commit.starts_with(recorded.as_str()) && !recorded.starts_with(tag_commit.as_str()) {
            warnings.push(format!(
                "tag `{tag}` points at {tag_commit} but its Release item records commit {recorded} — the recorded release and the shipped tag disagree (D0191)"
            ));
        }
    }
    GuardReport { name: "release-recorded", scanned: tags.len(), warnings, violations: Vec::new() }
}

/// Guard 44 (D0191, WARNING tier, owned by the `actor-enrollment` unit).
///
/// When a machine binding (`.keel/actor`) exists, its name resolves to a registered `Person`, or to
/// an `Actor` carrying a declared kind. Until now NOTHING validated the binding file — a name that is unregistered or
/// kindless surfaced only when some downstream write refused. An absent binding scans zero: binding
/// is per-machine and optional until a write needs an actor.
#[must_use]
pub fn enrollment_binding(root: &Path) -> GuardReport {
    let Some(bound) = std::fs::read_to_string(root.join(crate::actor::BINDING_PATH))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    else {
        return GuardReport { name: "enrollment-binding", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    };
    let actors = std::fs::read_to_string(root.join(".tracking").join("actors.sysml")).unwrap_or_default();
    let mut warnings = Vec::new();
    let decl = actors.lines().find_map(|line| {
        let l = line.trim_start();
        l.strip_prefix("part ")
            .and_then(|r| r.split_once(':'))
            .filter(|(n, _)| n.trim() == bound)
            .map(|(_, after)| after.trim_start().to_string())
    });
    match decl {
        None => warnings.push(format!(
            "machine binding `.keel/actor` names `{bound}`, which is NOT a registered actor — enroll it (`keel enroll`) or rebind (`keel actor set`) before it strands a write (D0191)"
        )),
        Some(after) if after.starts_with("Person") => {}
        Some(after) => {
            // An Actor must carry a kind; single-line and block forms both keep the kind within
            // the declaring region, so scan from the declaration to the next `part `.
            let region = actors
                .split_once(&format!("part {bound}"))
                .map(|(_, rest)| rest.split("\npart ").next().unwrap_or(rest).to_string())
                .unwrap_or_default();
            if !region.contains("ActorKind::") {
                warnings.push(format!(
                    "machine binding `.keel/actor` names `{bound}` ({}), which declares NO kind — an actor whose kind is unstated defeats the human/AI attestation distinction (D0106/D0191)",
                    after.split_whitespace().next().unwrap_or("?")
                ));
            }
        }
    }
    GuardReport { name: "enrollment-binding", scanned: 1, warnings, violations: Vec::new() }
}


// ── judgment-request-quality guard (a fork must earn the ask, D0207 clause 3) ────────────────────

/// Guard: a PROPOSED fork Decision carries everything a human needs to judge it.
///
/// Their words (D0207): "there's not a strong shortname, not good rationale, not good alternatives
/// or implications (i.e. the 'why'), there's no statement of research. all these should be provided
/// if you're reaching out for my judgment." A fork (a decision enumerating OPTIONs) is the one shape
/// that still reaches out — so before it may even be proposed it must have: a short name leading the
/// title (one word before the colon), a substantive rationale, a RESEARCH statement grounding the
/// choice, and per-option implications (a COST per OPTION). Non-fork decisions auto-accept under the
/// standing consent and are not scanned here. Accepted history is out of scope (status filter).
#[must_use]
pub fn judgment_request_quality(root: &Path) -> GuardReport {
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for path in crate::collect_sysml(&root.join(".engine").join("decisions")) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        if !text.contains("status = DecisionStatus::proposed") {
            continue;
        }
        let rel = relpath(root, &path);
        let options = text.matches("OPTION ").count();
        let distinct_options = {
            let mut toks: Vec<char> = Vec::new();
            let mut rest = text.as_str();
            while let Some(i) = rest.find("OPTION ") {
                rest = &rest[i + 7..];
                if let Some(c) = rest.chars().next().filter(char::is_ascii_uppercase) {
                    if !toks.contains(&c) {
                        toks.push(c);
                    }
                }
            }
            toks.len()
        };
        if distinct_options < 2 {
            continue; // not a fork — auto-accepts under D0207, never reaches out
        }
        scanned += 1;
        let field = |name: &str| -> String {
            let key = format!("{name} = \"");
            text.find(&key).and_then(|i| {
                let s = i + key.len();
                text[s..].find('"').map(|j| text[s..s + j].to_string())
            }).unwrap_or_default()
        };
        let title = field("title");
        let short = title.split(':').next().unwrap_or("").trim();
        if short.is_empty() || short.contains(' ') || short.chars().count() > 28 {
            violations.push(format!(
                "{rel}: fork decision's title does not lead with a strong short name (one word before the colon, <= 28 chars) — got \"{short}\" (D0207: the ask must be recognizable at a glance)"
            ));
        }
        if field("rationale").chars().count() < 200 {
            violations.push(format!(
                "{rel}: fork decision's rationale is under 200 chars — a request for judgment must carry its why (D0207)"
            ));
        }
        let research = text
            .lines()
            .find_map(|l| l.trim_start().strip_prefix("// RESEARCH:"))
            .map_or("", str::trim);
        if research.chars().count() < 40 {
            violations.push(format!(
                "{rel}: fork decision carries no substantive RESEARCH statement (a `// RESEARCH:` line, >= 40 chars) — what was looked at before asking: panel precedent, literature, prior art, or 'none found, and here is where I looked' (D0207; `keel record decision --research \"...\"`)"
            ));
        }
        let costs = text.matches("COST").count();
        if costs < distinct_options {
            violations.push(format!(
                "{rel}: {distinct_options} option(s) but only {costs} COST statement(s) — every alternative carries its implications or the choice is not informed (D0207)"
            ));
        }
        let _ = options;
    }
    GuardReport { name: "judgment-request-quality", scanned, warnings: Vec::new(), violations }
}

// ── claim-ancestry guard (a claim's date is bounded by the commit that introduced it, issue229) ───

/// Guard: `claimedAt` cannot precede its own introducing commit by more than the expiry window.
///
/// issue229 (process-value panel, multi-agent lens): holdership = earliest un-expired `claimedAt`,
/// and `claimedAt` is authored by the claimer — a backdated claim steals holdership deterministically
/// on every clone. This applies the repo's own doctrine (D0013: git ancestry is the clock) to claims:
/// the introducing commit's author date bounds how early the claim may say it was made. The bound is
/// [`crate::claim::CLAIM_EXPIRY_DAYS`], because a claim older than the window is stale on arrival —
/// backdating WITHIN the window remains possible and is stated here rather than hidden: the guard
/// narrows the theft window from unbounded to the expiry span. An uncommitted claim has no
/// introducing commit yet and is skipped — it cannot influence another clone until it lands.
#[must_use]
pub fn claim_ancestry(root: &Path) -> GuardReport {
    let claims = match crate::claim::claims(root) {
        Ok(c) => c,
        Err(e) => {
            return GuardReport {
                name: "claim-ancestry",
                scanned: 0,
                warnings: Vec::new(),
                violations: vec![format!("error reading claims: {e}")],
            }
        }
    };
    // A SHALLOW clone cannot resolve introduction commits — the oldest visible commit is the clone
    // boundary, not the claim's birth, and judging against it flags honest claims (this exact guard
    // went CI-red on its first push because checkout@v4 defaults to depth 1). Depth-dependent
    // verdicts are the machine-dependence K15 forbids: on a shallow repo this guard SKIPS LOUDLY.
    let shallow = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--is-shallow-repository"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .is_some_and(|o| String::from_utf8_lossy(&o.stdout).trim() == "true");
    if shallow {
        return GuardReport {
            name: "claim-ancestry",
            scanned: 0,
            warnings: vec![
                "SHALLOW clone: introduction commits cannot be resolved, so claim dates were NOT checked here (a depth-dependent verdict would be the K15 machine-dependence this guard exists to prevent). CI checks out full history for this reason.".to_string(),
            ],
            violations: Vec::new(),
        };
    }
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for c in &claims {
        if c.at.is_empty() {
            continue;
        }
        let intro = crate::gitx::git()
            .arg("-C")
            .arg(root)
            .args(["log", "--reverse", "--format=%ad", "--date=short", "-S", &c.name, "--", ".tracking/claims"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8_lossy(&o.stdout).lines().next().map(str::to_owned));
        let Some(intro_date) = intro.filter(|d| !d.is_empty()) else { continue }; // uncommitted claim
        scanned += 1;
        let lead = crate::view::days_between_pub(&c.at, &intro_date);
        if lead > crate::claim::CLAIM_EXPIRY_DAYS {
            violations.push(format!(
                "{}: claimedAt {} predates its introducing commit ({intro_date}) by {lead} day(s) — more than the {}-day expiry window. Git ancestry is the clock (D0013); a claim cannot say it was made before it could have influenced any clone (issue229 backdating).",
                c.name, c.at, crate::claim::CLAIM_EXPIRY_DAYS
            ));
        }
    }
    GuardReport { name: "claim-ancestry", scanned, warnings: Vec::new(), violations }
}

// ── question-coverage guard (declared knowledge facts are well-formed, D0161 part 3ii) ────────────

/// Guard: declared knowledge facts are well-formed (D0161 part 3ii).
///
/// A `Question` carries its text; an `Alias` carries its term and maps to an existing element.
/// WELL-FORMEDNESS only — coverage itself stays `keel knowledge question-coverage`,
/// because gating on coverage would make the cheapest fix deleting the question (D0098). Zero
/// declared facts = zero scanned, green: an absent `.knowledge/` is the feature unplugged (D0161
/// part 3i), never a violation. Unit-owned by `knowledge-graph-memory`, so deactivating that process
/// drops exactly this check.
#[must_use]
pub fn question_coverage(root: &Path) -> GuardReport {
    match crate::view::knowledge_wellformedness(root) {
        Ok((scanned, violations)) => GuardReport { name: "question-coverage", scanned, warnings: Vec::new(), violations },
        Err(e) => GuardReport {
            name: "question-coverage",
            scanned: 0,
            warnings: Vec::new(),
            violations: vec![format!("error reading knowledge facts: {e}")],
        },
    }
}

/// Guard 45 (D0193, WARNING tier): every control-relevant event is DECLARED with its required
/// record, and the declaration matches what the binary emits.
///
/// The family it closes (issues 203/205/207 + the sr13 sentinel): a control-relevant event with no
/// counted record stays invisible until a verification campaign trips over it. The check is a
/// two-way diff between `.engine/contracts/control-events.toml`'s ledger-record sections and the
/// event names the binary's emitters use - a declared event nothing emits warns (dead declaration),
/// an emitted event nothing declares warns (uncounted event). Inventory-record events are checked
/// against the hardening lens's point list by name. Absent contract = not adopted, reported (D0136).
#[must_use]
pub fn control_event_coverage(root: &Path) -> GuardReport {
    /// Every ledger event name the binary emits. A NEW `ledger_emit` call site must add its event
    /// here AND to the contract - this constant going stale is exactly what the two-way diff warns on.
    const EMITTED_LEDGER: [&str; 14] = [
        "post-edit", "stop", "user-prompt", "pre-bash", "pre-write", "subagent-stop",
        "launch-dirty-refusal", "override-consumed", "override-obligation-UNSYNCED",
        "red-yield-obligation-UNSYNCED", "actor-rebind", "hook-watchdog-timeout",
        "advisory-issued", "advisory-repeated", // issue230: spoken vs silent, and the ignore signal
    ];
    const INVENTORY_POINTS: [(&str, &str); 2] = [("spec-pin-check", "build-time spec pin"), ("pre-push-behind", "pre-push .githooks")];
    let path = root.join(".engine").join("contracts").join("control-events.toml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return GuardReport {
            name: "control-event-coverage",
            scanned: EMITTED_LEDGER.len(),
            warnings: vec!["control-events.toml is ABSENT - this control is NOT ADOPTED by this project (D0136: absence is a state, never a violation); the binary's control events go uncounted-by-declaration".to_string()],
            violations: Vec::new(),
        };
    };
    let mut declared_ledger: Vec<String> = Vec::new();
    let mut declared_inventory: Vec<String> = Vec::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('[') && l.ends_with(']') {
            current = Some(l[1..l.len() - 1].to_string());
        } else if let (Some(name), Some(rest)) = (&current, l.strip_prefix("record")) {
            let value = rest.trim_start_matches(['=', ' ']).trim_matches('"');
            match value {
                v if v.starts_with("ledger") => declared_ledger.push(name.clone()),
                v if v.starts_with("inventory") => declared_inventory.push(name.clone()),
                _ => {}
            }
        }
    }
    let mut warnings = Vec::new();
    for d in &declared_ledger {
        if !EMITTED_LEDGER.contains(&d.as_str()) {
            warnings.push(format!("declared control event `{d}` (record=ledger) has NO emitter in the binary - a dead declaration reads as coverage that does not exist (D0193)"));
        }
    }
    for e in EMITTED_LEDGER {
        if !declared_ledger.iter().any(|d| d == e) {
            warnings.push(format!("the binary emits ledger event `{e}` that control-events.toml does not declare - an uncounted-by-declaration control event, the issue203/205/207 family (D0193)"));
        }
    }
    // Inventory-record events: the named point must exist in the enforcementPoints inventory text.
    let inventory = crate::hardening::hardening(root).unwrap_or_default(); // the lens is the inventory's one authority; a compute failure reads as absent points, which warns rather than passes
    for (event, point_needle) in INVENTORY_POINTS {
        if declared_inventory.iter().any(|d| d == event) && !inventory.contains(point_needle) {
            warnings.push(format!("declared control event `{event}` (record=inventory) names no matching enforcement point (`{point_needle}`) in the hardening lens (D0193/issue203)"));
        }
    }
    let scanned = declared_ledger.len() + declared_inventory.len();
    GuardReport { name: "control-event-coverage", scanned, warnings, violations: Vec::new() }
}


/// Every `:>> id = "…"` value on one line. A line may carry several: the sprint records declare an item
/// and its result on one line each, and a per-line regex-free scan must not stop at the first.
fn id_values(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(i) = rest.find(":>> id") {
        rest = &rest[i + ":>> id".len()..];
        let Some(after_eq) = rest.split_once('=') else { break };
        let mut chars = after_eq.1.trim_start().chars();
        if chars.next() != Some('"') {
            continue;
        }
        let tail = chars.as_str();
        if let Some(end) = tail.find('"') {
            out.push(tail[..end].to_string());
            rest = &tail[end..];
        } else {
            break;
        }
    }
    out
}

/// 8-4-4-4-12 groups of `[0-9a-z]`, exactly. Written out rather than regexed because the guard path
/// stays dependency-light, and because the group lengths ARE the specification.
pub(crate) fn uuid_shaped(v: &str) -> bool {
    let groups: Vec<&str> = v.split('-').collect();
    groups.len() == 5
        && [8usize, 4, 4, 4, 12].iter().zip(&groups).all(|(want, g)| {
            g.len() == *want && g.chars().all(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
        })
}

/// `(name, type, 1-based line, body-up-to-the-next-declaration)` for each id-bearing declaration.
///
/// The body stops at the NEXT declaration so a member can never borrow its sibling's id — the bug that
/// would make this guard pass a file where one item has two ids and its neighbour none.
fn id_bearing_decls(text: &str) -> Vec<(String, String, usize, String)> {
    let starts: Vec<(usize, usize, String, String)> = text
        .lines()
        .enumerate()
        .filter_map(|(i, raw)| {
            let line = raw.trim_start();
            if line.starts_with("//") {
                return None;
            }
            let after_marker = line.strip_prefix('#').map_or(line, |r| {
                r.split_once(char::is_whitespace).map_or("", |(_, rest)| rest.trim_start())
            });
            for kw in ["part ", "verification ", "requirement ", "use case "] {
                if let Some(rest) = after_marker.strip_prefix(kw) {
                    let (name, rest) = rest.split_once(':')?;
                    let ty: String =
                        rest.trim_start().chars().take_while(char::is_ascii_alphanumeric).collect();
                    if ENGINE_ID_TYPES.contains(&ty.as_str()) && rest.contains('{') {
                        return Some((i, i + 1, name.trim().to_string(), ty));
                    }
                }
            }
            None
        })
        .collect();
    let lines: Vec<&str> = text.lines().collect();
    starts
        .iter()
        .enumerate()
        .map(|(k, (idx, line_no, name, ty))| {
            let end = starts.get(k + 1).map_or(lines.len(), |n| n.0);
            let body = lines.get(*idx..end).unwrap_or_default().join("\n");
            (name.clone(), ty.clone(), *line_no, body)
        })
        .collect()
}

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
        "doc-guard-count" => Some(doc_guard_count(root)),
        "actors" => Some(actors(root)),
        "acceptance-events" => Some(acceptance_events(root)),
        "sprint-coverage" => Some(sprint_coverage(root)),
        "ceremony" => Some(ceremony(root)),
        "charter" => Some(charter(root)),
        "process-change" => Some(process_change(root)),
        "issues" => Some(issues(root)),
        "resolver-kind" => Some(resolver_kind(root)),
        "stale-gate-prose" => Some(stale_gate_prose(root)),
        "impossible-evidence-date" => Some(impossible_evidence_dates(root)),
        "identity-present" => Some(identity_present(root)),
        "identity-well-formed" => Some(identity_well_formed(root)),
        // D0225: a process that cannot say WHEN it applies cannot be recommended for or against,
        // so onboarding silently stops seeing it. Honest-state, not completeness (D0098): the
        // claim "this project chose its processes" is false if a process was invisible to the choice.
        "process-applicability" => Some(process_applicability(root)),
        "tool-reference" => Some(tool_reference(root)), // hard (issue196) — a doc naming a deleted tool strands its follower
        "scaffold-placeholder" => Some(scaffold_placeholder(root)), // hard (dcSprintScaffold) — an unfilled skeleton is not a record
        "claude-surface-drift" => Some(claude_surface_drift(root)), // hard (D0174/K7) — a mutated hook command is a silently weakened control
        "decision-scaffolding" => Some(decision_scaffolding(root)), // WARNING-tier (D0188, composed with D0180) — an accepted promise chartering no work
        "release-recorded" => Some(release_recorded(root)), // WARNING-tier (D0191, deploy unit) — a shipped tag with no authored Release item
        "enrollment-binding" => Some(enrollment_binding(root)), // WARNING-tier (D0191, actor-enrollment unit) — a machine binding naming an unregistered or kindless actor
        "control-event-coverage" => Some(control_event_coverage(root)), // WARNING-tier (D0193) — a control-relevant event with no counted record
        "question-coverage" => Some(question_coverage(root)), // D0161: declared knowledge facts are well-formed; coverage itself stays a view
        "claim-ancestry" => Some(claim_ancestry(root)), // issue229: claimedAt bounded by the introducing commit (D0013 applied to claims)
        "judgment-request-quality" => Some(judgment_request_quality(root)), // D0207: a fork must earn the ask

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
        "attribute-vocabulary" => Some(attribute_vocabulary(root)), // hard (issue118) — an undeclared attribute is silently LOST
        "type-collision" => Some(type_collision(root)), // hard (D0128) — a project type shadowing an engine type, silently
        "ownership" => Some(ownership(root)), // hard (D0108/D0129) — a non-owner overwriting another actor's fields
        "attestation-authority" => Some(attestation_authority(root)), // hard (D0092) — a human-only verdict judged by an AI
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
    let engine: HashSet<&str> = engine_markers().iter().map(String::as_str).collect();
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
    // issue231 (process-value panel, formal-methods lens): the RATCHET. The parser is the TCB of
    // every guard and view, and a warning whose count can grow silently is a control people learn
    // to scroll past. With a committed baseline declared, EXCEEDING it is a violation — never mere
    // presence (the D0132 all-or-nothing lesson): the 7 legacy successions stay a warning, a NEW
    // skipped class goes red. Shrinking below baseline warns to ratchet the baseline DOWN, so the
    // bound only ever tightens. Absent contract = ratchet not adopted, stated (D0136).
    let mut violations = Vec::new();
    let baseline_path = root.join(".engine").join("contracts").join("parser-coverage-baseline.toml");
    match std::fs::read_to_string(&baseline_path) {
        Ok(text) => match text.parse::<toml::Value>().ok().and_then(|v| v.get("skipped").and_then(toml::Value::as_integer)) {
            Some(baseline) => {
                let baseline = usize::try_from(baseline).unwrap_or(0);
                if total > baseline {
                    violations.push(format!(
                        "skipped-statement count {total} EXCEEDS the committed baseline {baseline} — a new statement class became invisible to every guard and view (issue231 ratchet). Either teach keel-parser the construct, or raise the baseline IN THE SAME COMMIT with the reason (a visible diff, never silent growth)."
                    ));
                } else if total < baseline {
                    warnings.push(format!(
                        "skipped-statement count {total} is BELOW the baseline {baseline} — ratchet it down in parser-coverage-baseline.toml so the bound keeps what the parser gained (issue231)."
                    ));
                }
            }
            None => violations.push("parser-coverage-baseline.toml exists but has no integer `skipped` key — a malformed ratchet must fail loud, not silently un-adopt".to_string()),
        },
        Err(_) => warnings.push("no parser-coverage baseline declared (.engine/contracts/parser-coverage-baseline.toml) — the skipped count can grow without a red gate (issue231 ratchet not adopted; D0136: absence is a state, stated)".to_string()),
    }
    GuardReport { name: "parser-coverage", scanned, warnings, violations }
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
    // issue241: scanned counted only the guard-bearing units, so the guard reported "11 scanned"
    // against a project declaring 23 processes — a scan count that understates its own population is
    // the same class of misreport as the catalogue that denied those 12 existed.
    let scanned = crate::activation::declared_processes(root).len().max(act.unit_names().len());
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
    // Intake (D0166). A type absent from this list has its IDENTITY UNCHECKED - engine-lint never asks
    // whether it carries an `:>> id` - so a new item type is only half-registered until it is here.
    "Statement", "UserStory",
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
mod retro_tie_tests {
    use super::{names_tracked_item, retro_backlog_warnings_for_test};

    /// THE CONTROL for issue189: co-staging a tracked file no longer satisfies the guard - the retro's
    /// own text must name the item its finding produced. The old behaviour let one failure class reach
    /// five retros and zero items, because every commit staged issues.sysml for something else.
    #[test]
    fn a_co_staged_file_no_longer_excuses_an_unnamed_finding() {
        let changed = vec![".tracking/issues.sysml".to_string()];
        let sprints = vec![(
            "sprintX.sysml".to_string(),
            "retro: AVOIDABLE-ISSUE: the same mistake again, described in prose.".to_string(),
        )];
        let w = retro_backlog_warnings_for_test(&changed, &sprints);
        assert_eq!(w.len(), 1, "an unnamed finding must warn even when tracked files are co-staged");
    }

    /// A retro that NAMES its item, or justifies having none, is clean.
    #[test]
    fn naming_the_item_or_justifying_none_satisfies_the_guard() {
        let changed: Vec<String> = Vec::new();
        for text in [
            "AVOIDABLE-ISSUE: shell mangling, occurrence six - now tracked as dcAuthorViaWriteTool.",
            "AVOIDABLE-ISSUE: the counter was wrong; recorded as issue188 with a resolver.",
            "AVOIDABLE-ISSUE: a one-off typo; no new item - the edit gate already catches this class.",
        ] {
            let w = retro_backlog_warnings_for_test(&changed, &[("s.sysml".to_string(), text.to_string())]);
            assert!(w.is_empty(), "should be clean: {text}");
        }
    }

    /// Word boundaries: prose containing `dc` or `issue` must not satisfy the naming check.
    #[test]
    fn prose_lookalikes_do_not_count_as_items() {
        assert!(!names_tracked_item("the dc motor issue was discussed at length"));
        assert!(!names_tracked_item("reproduced changes"));
        assert!(names_tracked_item("tracked as dcFooBar"));
        assert!(names_tracked_item("see issue123"));
        assert!(!names_tracked_item("tissue42 is not an item"));
    }
}

#[cfg(test)]
mod gate_order_tests {
    use super::GATE_ORDER;

    /// Panel R2 (robotics finding 2): `GATE_ORDER` is a compiled constant that hand-duplicates the
    /// succession chain DECLARED in .engine/workflows/delivery.sysml - two homes for one fact.
    /// This test binds them: a CR that edits the declared order breaks the build until the
    /// constant follows, so the guard can never silently enforce a superseded ceremony order.
    #[test]
    fn gate_order_matches_the_declared_successions() {
        let declared = include_str!("../../.engine/workflows/delivery.sysml");
        let mut chain: Vec<(String, String)> = Vec::new();
        for line in declared.lines() {
            let l = line.trim();
            if let Some(rest) = l.strip_prefix("first ") {
                if let Some((a, b)) = rest.trim_end_matches(';').split_once(" then ") {
                    chain.push((a.trim().to_lowercase(), b.trim().to_lowercase()));
                }
            }
        }
        assert_eq!(chain.len(), GATE_ORDER.len() - 1, "the declared chain must cover every adjacent GATE_ORDER pair");
        for (i, pair) in GATE_ORDER.windows(2).enumerate() {
            assert_eq!(chain[i].0, pair[0].to_lowercase(), "declared succession {i} disagrees with GATE_ORDER");
            assert_eq!(chain[i].1, pair[1].to_lowercase(), "declared succession {i} disagrees with GATE_ORDER");
        }
    }
}

#[cfg(test)]
mod scan_count_tests {
    use super::{GuardReport, GUARD_NAMES};

    /// No guard may be written to report violations against a zero scan count (issue180). The RUNNER
    /// surfaces it at print time; this pins that the check exists, since a silent regression here makes
    /// `scanned` untrustworthy again and untrustworthy numbers get used.
    #[test]
    fn the_runner_flags_a_violation_against_an_empty_scan() {
        let src = std::fs::read_to_string("src/guards.rs").expect("guards.rs is readable");
        assert!(
            src.contains("self.scanned == 0 && !self.violations.is_empty()"),
            "the self-contradiction check must survive in GuardReport::print"
        );
    }

    /// Every guard reporting a real population today keeps reporting one. Guards whose population is
    /// legitimately empty at this commit are excluded BY NAME rather than by a blanket allowance, so a
    /// guard silently going quiet cannot hide behind them.
    #[test]
    fn the_guards_that_report_a_population_still_do() {
        const LEGITIMATELY_EMPTY: [&str; 11] = [
            // scans priority-ordering pairs on the ready frontier; D0189's scope closure emptied
            // the frontier down to a handful of same-rank resolvers, so there is nothing to order.
            // The population returns the moment the backlog holds ranked work again.
            "priority-inversion",
            // both D0191 guards scan ENVIRONMENT-DEPENDENT populations: release-recorded scans git
            // tags (absent in CI's tag-less clone) and enrollment-binding scans the machine-local
            // .keel/actor binding (gitignored, absent on CI) - locally both scan real populations.
            "release-recorded",
            "enrollment-binding",
            // scans done-work -> SR pairs lacking a #Verify edge; sprints 393/394 verified every
            // live SR (verification lens: neither = 0), so the population emptied by completion.
            "verification-trace",
            "charter",                   // scans CHANGED files
            "process-change",            // scans CHANGED process definitions
            "judgment-request-quality",  // scans PROPOSED fork decisions - zero between forks is the healthy state (D0207: most decisions auto-accept and never ask)
            "retro-backlog",             // scans retro findings needing a backlog item
            "doc-sync",                  // scans CHANGED doc surface
            "base-first-justification",  // scans new base-construct adoptions
            "ownership",                 // scans cross-owner edits
        ];
        let root = std::path::Path::new("..");
        for name in GUARD_NAMES {
            if LEGITIMATELY_EMPTY.contains(&name) {
                continue;
            }
            let Some(r): Option<GuardReport> = super::run_one(name, root) else { continue };
            assert!(r.scanned > 0, "guard `{name}` reports a zero population - mis-aimed, or newly empty and needing an entry in LEGITIMATELY_EMPTY");
        }
    }
}

#[cfg(test)]
mod identity_form_tests {
    use super::{id_values, tool_reference, uuid_shaped};

    /// The shape predicate, on the two ids I actually mangled and the deliberate mnemonic convention it
    /// must NOT break. Written as a table because the interesting cases are the near-misses.
    #[test]
    fn the_shape_predicate_accepts_the_convention_and_rejects_the_real_defects() {
        for good in [
            "3a86f04d-2c19-4b57-9e80-64bc17ea5d38",
            "4b78e0c1-3f52-4d96-a814-000000000i01", // the intake process-step convention: shaped, not hex
            "0d800620-0814-6620-9b3e-000000300620",
        ] {
            assert!(uuid_shaped(good), "{good} is well shaped and must pass");
        }
        for bad in [
            "not-a-uuid-at-all",
            "3a86f04d-2c19-4b57-9e80-64bc17ea5device", // 15 chars in the last group
            "4b07d2f6-9e35-4c81-a period-placeholder", // a SPACE, which is how this was found
            "eb5ebf9-6a7b-4c8d-efa9-0b1c2d3e4f5a",     // 7 in the first group - the historical class
            "3a86f04d-2c19-4b57-9e80-64BC17EA5D38",    // uppercase: one id, two spellings, is not identity
            "",
        ] {
            assert!(!uuid_shaped(bad), "{bad:?} is malformed and must fail");
        }
    }

    /// A line may declare an item AND its result, so the scan must not stop at the first id. Missing
    /// this would make the guard blind to exactly the sprint records where most ids live.
    #[test]
    fn every_id_on_a_line_is_read_not_just_the_first() {
        let line = r#"part a : X { :>> id = "aaaaaaaa-1111-2222-3333-444444444444"; } part b : Y { :>> id = "bad"; }"#;
        assert_eq!(id_values(line), vec!["aaaaaaaa-1111-2222-3333-444444444444", "bad"]);
    }

    /// The grandfather set is an explicit LIST of 15, and its size is asserted so growing it is a
    /// deliberate edit to a failing test rather than a quiet accommodation. Guard 36's first version
    /// keyed its exemption on a DATE and thereby exempted the defect it existed to catch.
    #[test]
    fn the_grandfather_set_cannot_grow_quietly() {
        let src = std::fs::read_to_string("src/guards.rs").expect("guards.rs is readable");
        let body = src
            .split_once("const GRANDFATHERED: [&str; 15]")
            .expect("the set is declared with its size, so a 16th entry does not compile")
            .1;
        let listed = body[..body.find("];").expect("the list is closed")].matches('"').count() / 2;
        assert_eq!(listed, 15, "the declared size and the actual entries must agree");
    }

    /// Guard 39: a living-surface reference to a missing tool is a violation; a reference to an
    /// existing tool and a bare directory mention are not; `.tracking` (history) is out of scope.
    #[test]
    fn tool_reference_flags_only_missing_files_on_the_living_surface() {
        let root = std::env::temp_dir().join("keel-toolref-guard");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".engine").join("skills")).expect("mkdir");
        std::fs::create_dir_all(root.join(".engine").join("tools")).expect("mkdir");
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        std::fs::write(root.join(".engine").join("tools").join("real.py"), "# real").expect("write");
        std::fs::write(
            root.join(".engine").join("skills").join("s.md"),
            "run `.engine/tools/real.py` then .engine/tools/gone.py. See .engine/tools/ for more.\n",
        )
        .expect("write");
        std::fs::write(
            root.join(".tracking").join("h.sysml"),
            "// history may truthfully say .engine/tools/retired.py existed\n",
        )
        .expect("write");
        let report = tool_reference(&root);
        assert_eq!(report.scanned, 2, "two file references scanned (the bare directory mention is not one)");
        assert_eq!(report.violations.len(), 1, "{:?}", report.violations);
        assert!(report.violations[0].contains(".engine/tools/gone.py"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_process_that_cannot_say_when_it_applies_is_refused() {
        // D0225. Both directions, because a guard that only ever passes is indistinguishable from a
        // guard that does nothing - and this codebase has already shipped two checks that passed on
        // an empty population (issue250, claude-surface-drift on zero skills).
        let root = std::env::temp_dir().join("keel-guard-applicability");
        let _ = std::fs::remove_dir_all(&root);
        let dir = root.join(".engine").join("processes");
        std::fs::create_dir_all(&dir).unwrap();

        let declares = "package P {
    action p : Process {
        :>> id = \"x\";
";
        std::fs::write(dir.join("with.sysml"), format!("{declares}        // APPLIES-WHEN: a stated situation
    }}
}}
")).unwrap();
        let r = process_applicability(&root);
        assert_eq!((r.scanned, r.violations.len()), (1, 0), "a declared condition passes");

        std::fs::write(dir.join("without.sysml"), format!("{declares}    }}
}}
")).unwrap();
        let r = process_applicability(&root);
        assert_eq!(r.scanned, 2, "both process files are in scope");
        assert_eq!(r.violations.len(), 1, "the one lacking a condition is refused");
        assert!(r.violations[0].contains("without.sysml"), "{:?}", r.violations);

        // A file that declares no Process at all is out of scope, not a violation.
        std::fs::write(dir.join("helper.sysml"), "package H {
    private import X::*;
}
").unwrap();
        assert_eq!(process_applicability(&root).scanned, 2, "a non-Process file is not scanned");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// D0209 clause 2 coverage audit, made executable: every file that DEFINES a guard (`-> GuardReport`)
    /// must sit inside the enforcement-surface lock, so a new guard file cannot be added OUTSIDE it and
    /// thereby be editable without a signed Decision. Scans the real `src/` tree rather than trusting the
    /// hand-list in `GUARD_SOURCE_FILES` -- if the two diverge, this fails CI, which is the point.
    #[test]
    fn enforcement_surface_covers_every_guard_source() {
        fn collect(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else { return };
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    collect(&p, out);
                } else if p.extension().is_some_and(|x| x == "rs") {
                    out.push(p);
                }
            }
        }
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        collect(&src, &mut files);
        let mut uncovered = Vec::new();
        for f in files {
            let Ok(text) = std::fs::read_to_string(&f) else { continue };
            if !text.contains("-> GuardReport") {
                continue;
            }
            let rel = f.strip_prefix(&src).unwrap().to_string_lossy().replace('\\', "/");
            let repo_rel = format!("keel-cli/src/{rel}");
            if !is_enforcement_surface(&repo_rel) {
                uncovered.push(repo_rel);
            }
        }
        assert!(
            uncovered.is_empty(),
            "guard-defining file(s) OUTSIDE the enforcement-surface lock (add to GUARD_SOURCE_FILES): {uncovered:?}"
        );
    }

    #[test]
    fn enforcement_surface_locks_workflows_hooks_and_guard_source() {
        assert!(is_enforcement_surface(".github/workflows/ci.yml"));
        assert!(is_enforcement_surface(".githooks/pre-commit"));
        assert!(is_enforcement_surface("keel-cli/src/guards.rs"));
        assert!(is_enforcement_surface("keel-cli/src/adherence.rs"));
        // NOT locked: ordinary source, docs, a workflow-shaped path outside the dir.
        assert!(!is_enforcement_surface("keel-cli/src/main.rs"));
        assert!(!is_enforcement_surface(".engine/docs/guards.md"));
        assert!(!is_enforcement_surface("README.md"));
    }

    /// Every enforced guard must actually DISPATCH. `run_one` is a hand-written match, so a name can sit
    /// in `GUARD_NAMES` -- counted in the control inventory, listed in `--help`, documented in guards.md --
    /// while `run_one` returns `None` for it and `run_all` silently runs 35 of 36. Text-level against this
    /// file's own source, so it costs no model build and cannot drift from the match it checks.
    #[test]
    fn every_enforced_guard_dispatches() {
        const GUARDS_RS: &str = include_str!("guards.rs");
        let dispatch = GUARDS_RS
            .split_once("pub fn run_one(")
            .expect("run_one must exist")
            .1;
        for name in GUARD_NAMES {
            let arm = format!("\"{name}\" =>");
            assert!(
                dispatch.contains(&arm),
                "guard `{name}` is in GUARD_NAMES but has no arm in run_one -- it would be counted in the                  control inventory and never actually run"
            );
        }
    }

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
    fn a_duplicate_id_now_fails_with_no_exemption_list() {
        // issue080 RESOLVED. The 18 bootstrap duplicates across 26 records were re-identified by a
        // D0067 migration (control totals reconciled: 7135 records before and after, distinct ids
        // 7109 -> 7135), so the grandfather list is GONE rather than emptied — an empty exemption
        // list is an invitation to refill it. Every duplicate from here is a live corruption.
        let id = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d"; // one of the 18, now unique in the model
        let files = vec![
            ("a.sysml".to_string(), format!("package P {{
    part x : Need {{ :>> id = \"{id}\"; }}
}}")),
            ("b.sysml".to_string(), format!("package Q {{
    part y : Need {{ :>> id = \"{id}\"; }}
}}")),
        ];
        let (warnings, violations) = duplicate_scan(&files);
        assert_eq!(violations.len(), 1, "a formerly-grandfathered id must now FAIL: {violations:?}");
        assert!(!warnings.iter().any(|m| m.contains("GRANDFATHERED")), "no exemption path remains: {warnings:?}");
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

// ── attribute-vocabulary guard (issue118) ────────────────────────────────────

/// Nearest declared attribute to `name`, for the "did you mean" hint.
///
/// Edit distance capped at 2 relative to length: a typo is a slip of a character or two, and
/// suggesting a name three edits away is noise that teaches the reader to skim the message.
fn nearest_attr<'a>(name: &str, declared: &'a HashSet<String>) -> Option<&'a str> {
    let budget = if name.len() <= 4 { 1 } else { 2 };
    declared
        .iter()
        .map(|d| (edit_distance(name, d), d.as_str()))
        .filter(|(d, _)| *d <= budget)
        .min_by_key(|(d, s)| (*d, s.len()))
        .map(|(_, s)| s)
}

/// Levenshtein distance, two rows. Index-free so a panic is not reachable by construction — this
/// runs over authored text, where an unexpected shape must never abort the gate.
fn edit_distance(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        if let Some(first) = cur.first_mut() {
            *first = i + 1;
        }
        for (j, cb) in b.iter().enumerate() {
            let sub = prev.get(j).copied().unwrap_or(0) + usize::from(ca != cb);
            let del = prev.get(j + 1).copied().unwrap_or(0) + 1;
            let ins = cur.get(j).copied().unwrap_or(0) + 1;
            if let Some(slot) = cur.get_mut(j + 1) {
                *slot = sub.min(del).min(ins);
            }
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev.last().copied().unwrap_or(0)
}

/// Guard (issue118): every `:>> name =` on an ENGINE-typed element names an attribute that type
/// actually declares.
///
/// # Why this is not covered by anything else
///
/// The engine checked MARKERS (D0133) and ENUM MEMBERS, but never attribute NAMES. Proven by probe:
/// a `CodeElement` authored with `codeHsah` and `riskClas` passed `validate`, all 32 guards and the
/// fast edit gate — 329 files reported clean — while silently losing its risk classification, so the
/// element sat in the audit frontier as `correctness` when its author had written `dataLoss`. That is
/// the whole "new schema elements are not being picked up" surprise: the author believes a fact was
/// recorded, every gate agrees the model is honest, and the fact is not there.
///
/// # What it deliberately does NOT judge
///
/// A type the engine schema does not declare is a PROJECT type (D0136/sprint 298), whose attributes
/// the engine cannot know. Those are skipped entirely — judging them would recreate the issue090
/// lockout, where a binary's opinion about a project's own vocabulary blocked every commit.
#[must_use]
pub fn attribute_vocabulary(root: &Path) -> GuardReport {
    let mut files = crate::collect_sysml(&root.join(".tracking"));
    files.extend(crate::collect_sysml(&root.join(".engine")));
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let rel = relpath(root, path);
        // The type of the element whose braces we are inside, with the depth it opened at.
        let mut stack: Vec<(String, String, i32)> = Vec::new();
        let mut depth: i32 = 0;
        for (i, raw) in text.lines().enumerate() {
            let line = strip_string_literals(raw);
            let t = line.trim_start();
            if t.starts_with("//") {
                continue;
            }
            // THE ELEMENT DECLARED ON THIS LINE, if any. Records in this repo are overwhelmingly
            // authored on ONE line — `verification x : Test { :>> id = ...; :>> method = ...; }` —
            // so a scanner that only credits `:>>` to a previously-pushed stack frame misses them
            // all. The first cut did exactly that: 8088 of 41036 assignments scanned, and it
            // reported PASS over the 80% it never looked at, which is precisely the silent-coverage
            // failure this guard exists to prevent.
            let mut line_owner: Option<(String, String)> = None;
            if let Some((decl, rest)) = t.split_once(':') {
                let decl_t = decl.trim().trim_start_matches('#');
                let is_usage = ["part", "verification", "occurrence", "requirement", "action", "item", "use case"]
                    .iter()
                    .any(|k| decl_t.starts_with(k) && !decl_t.contains(" def "));
                if is_usage && !rest.starts_with('>') {
                    let name: String = decl_t.split_whitespace().nth(1).unwrap_or("").to_string();
                    let ty: String =
                        rest.trim().chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if !ty.is_empty() && line.contains('{') {
                        line_owner = Some((name.clone(), ty.clone()));
                        // Push only if the block stays OPEN past this line; a single-line record
                        // opens and closes here and must not linger on the stack.
                        if line.matches('{').count() > line.matches('}').count() {
                            stack.push((name, ty, depth));
                        }
                    }
                }
            }
            // EVERY `:>>` on the line, not just a line-leading one.
            for (pos, _) in line.match_indices(":>>") {
                let attr: String = line[pos + 3..]
                    .trim_start()
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                let owner = line_owner.as_ref().map(|(n, ty)| (n, ty)).or_else(|| stack.last().map(|(n, ty, _)| (n, ty)));
                let Some((owner, ty)) = owner else { continue };
                let Some(declared) = crate::schema::declared_attrs_in(root, ty) else { continue };
                scanned += 1;
                if !attr.is_empty() && !declared.contains(&attr) {
                    let hint = nearest_attr(&attr, &declared).map_or_else(
                        || format!("`{ty}` declares: {}", sorted_list(&declared)),
                        |n| format!("did you mean `{n}`?"),
                    );
                    violations.push(format!(
                        "{rel}:{}: `{owner} : {ty}` sets `{attr}`, which `{ty}` does not declare — {hint} An undeclared attribute is accepted silently and the value is simply LOST: the element computes as though it were never authored (issue118).",
                        i + 1
                    ));
                }
            }
            depth += i32::try_from(line.matches('{').count()).unwrap_or(0)
                - i32::try_from(line.matches('}').count()).unwrap_or(0);
            while stack.last().is_some_and(|(_, _, d)| depth <= *d) {
                stack.pop();
            }
        }
    }
    GuardReport { name: "attribute-vocabulary", scanned, warnings: Vec::new(), violations }
}

fn sorted_list(s: &HashSet<String>) -> String {
    let mut v: Vec<&str> = s.iter().map(String::as_str).collect();
    v.sort_unstable();
    v.join(", ")
}

#[cfg(test)]
mod attribute_vocabulary_tests {
    use super::*;

    fn probe(body: &str) -> GuardReport {
        let dir = std::env::temp_dir().join(format!("keel-attrvocab-{}", body.len()));
        let tracking = dir.join(".tracking");
        std::fs::create_dir_all(&tracking).expect("scratch dir");
        std::fs::write(tracking.join("probe.sysml"), body).expect("write probe");
        let r = attribute_vocabulary(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        r
    }

    #[test]
    fn a_misspelled_attribute_is_caught_with_the_nearest_name() {
        let r = probe("part p : Claim {\n    :>> claimedItm = \"x\";\n}\n");
        assert_eq!(r.violations.len(), 1, "{:?}", r.violations);
        assert!(r.violations[0].contains("did you mean `claimedItem`?"), "{}", r.violations[0]);
    }

    #[test]
    fn correctly_spelled_attributes_pass_including_inherited_ones() {
        let r = probe("part p : Claim {\n    :>> id = \"x\";\n    :>> claimedItem = \"y\";\n}\n");
        assert!(r.violations.is_empty(), "id is inherited from Element: {:?}", r.violations);
    }

    #[test]
    fn single_line_records_are_scanned() {
        // THE REGRESSION THAT MATTERS. The first cut only credited `:>>` to a stack frame pushed on
        // an EARLIER line, so single-line records — the dominant form in this repo — were invisible:
        // 8088 of 41036 assignments scanned, reporting PASS over the 80% it never read.
        let r = probe("part p : Claim { :>> id = \"x\"; :>> claimedItm = \"y\"; }\n");
        assert_eq!(r.scanned, 2, "both assignments on the one line must be scanned");
        assert_eq!(r.violations.len(), 1, "{:?}", r.violations);
    }

    #[test]
    fn a_project_declared_type_is_never_judged() {
        // Judging a vocabulary the engine cannot know is how issue090 blocked every commit.
        let r = probe("part p : SomeProjectType {\n    :>> whateverTheyLike = \"x\";\n}\n");
        assert!(r.violations.is_empty(), "{:?}", r.violations);
        assert_eq!(r.scanned, 0, "an unknown type is skipped, not scanned");
    }

    #[test]
    fn prose_naming_an_attribute_is_not_an_assignment() {
        let r = probe("// :>> notReal = \"in a comment\";\npart p : Claim { :>> id = \"x\"; }\n");
        assert!(r.violations.is_empty(), "{:?}", r.violations);
    }
}

#[cfg(test)]
mod viewpoint_enumeration_tests {
    use super::*;

    /// issue139: the guard must judge EVERY declared Viewpoint, not the ones in one hardcoded filename.
    /// Before the fix a viewpoint in any other `.engine/views` file was invisible — the guard reported
    /// "32 scanned, 0 violations" with a probe in the tree whose renderer named no command at all. This
    /// asserts the FAILING direction, because a guard only ever tested green is a guard nobody tested.
    #[test]
    fn a_viewpoint_outside_the_registry_file_is_still_judged() {
        let dir = std::env::temp_dir().join("keel_vp_enum_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".engine/views")).unwrap();
        std::fs::create_dir_all(dir.join(".tracking")).unwrap();
        let vp = |name: &str, id: &str, title: &str, renderer: &str| {
            format!(
                "package P{name} {{
    part {name} : Viewpoint {{
        :>> id = \"{id}\";
        :>> title = \"{title}\";
        :>> renderer = \"{renderer}\";
    }}
}}
"
            )
        };
        // one in the registry file with a real renderer, one in ANOTHER file with a command that does
        // not exist — the exact shape that used to pass.
        std::fs::write(dir.join(".engine/views/viewpoint-registry.sysml"), vp("goodVP", "aaaaaaaa-1111-4111-8111-111111111111", "good", "keel orient")).unwrap();
        std::fs::write(dir.join(".engine/views/other.sysml"), vp("strayVP", "bbbbbbbb-2222-4222-8222-222222222222", "stray", "keel no-such-command")).unwrap();

        let r = viewpoint_renderer(&dir);
        assert_eq!(r.scanned, 2, "both viewpoints must be scanned, wherever they are declared");
        assert_eq!(r.violations.len(), 1, "the stray viewpoint's unknown renderer must be a violation; got {:?}", r.violations);
        assert!(r.violations[0].contains("stray"), "the violation must name the stray viewpoint; got {:?}", r.violations);

        // and the enumeration both readers share agrees
        let rows = crate::view::declared_viewpoints(&dir).unwrap();
        assert_eq!(rows.len(), 2, "one answer to what viewpoints exist");
        let _ = std::fs::remove_dir_all(&dir);
    }
}

#[cfg(test)]
mod claim_ancestry_tests {
    /// issue229 pinned: a claim whose `claimedAt` predates its own introducing commit by more than
    /// the expiry window turns the guard RED; a same-day claim stays green. Scratch git repo so the
    /// intro-commit lookup is real, not mocked.
    #[test]
    fn backdated_claim_is_refused_and_honest_claim_passes() {
        let dir = std::env::temp_dir().join("keel-claim-ancestry-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".tracking").join("claims")).expect("mkdir");
        let run = |args: &[&str]| {
            let out = crate::gitx::git().arg("-C").arg(&dir).args(args).output().expect("git runs");
            assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "t@t"]);
        run(&["config", "user.name", "t"]);
        let claim_file = |claimed_at: &str| {
            format!(
                "package ProjectClaimsT {{\n    part claimT001 : Claim {{\n        :>> id = \"aaaaaaaa-6666-4666-9666-aaaaaaaaaaaa\";\n        :>> claimedItem = \"someItem\";\n        :>> claimedBy = \"tester\";\n        :>> claimedAt = \"{claimed_at}\";\n    }}\n}}\n"
            )
        };
        // Backdated far beyond the window relative to the commit date (today).
        std::fs::write(dir.join(".tracking/claims/tester.sysml"), claim_file("2020-01-01")).expect("write");
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "claim"]);
        crate::fingerprint::new_epoch();
        let red = super::claim_ancestry(&dir);
        assert_eq!(red.violations.len(), 1, "backdated claim must violate: {:?}", red.warnings);
        assert!(red.violations[0].contains("predates its introducing commit"), "{}", red.violations[0]);
        // An honest claim dated the day it was committed passes.
        let today = {
            let out = crate::gitx::git().arg("-C").arg(&dir).args(["log", "-1", "--format=%ad", "--date=short"]).output().expect("git");
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        };
        std::fs::write(dir.join(".tracking/claims/tester.sysml"), claim_file(&today)).expect("write");
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "honest claim"]);
        crate::fingerprint::new_epoch();
        let green = super::claim_ancestry(&dir);
        assert!(green.violations.is_empty(), "{:?}", green.violations);
    }
}

#[cfg(test)]
mod accept_transform_tests {
    use super::is_accept_transform;

    /// D0205: the sanctioned non-owner edit is EXACTLY status proposed->accepted; anything else —
    /// the reverse flip, a rider attribute, a different field — still violates ownership.
    #[test]
    fn only_the_forward_status_flip_is_sanctioned() {
        let old = vec!["status=proposed".to_string(), "title=x".to_string()];
        let fwd = vec!["status=accepted".to_string(), "title=x".to_string()];
        assert!(is_accept_transform(&old, &fwd));
        let rev_old = vec!["status=accepted".to_string(), "title=x".to_string()];
        let rev_new = vec!["status=proposed".to_string(), "title=x".to_string()];
        assert!(!is_accept_transform(&rev_old, &rev_new), "reverse flip is not sanctioned");
        let rider = vec!["status=accepted".to_string(), "title=CHANGED".to_string()];
        assert!(!is_accept_transform(&old, &rider), "a rider edit is not sanctioned");
        assert!(!is_accept_transform(&old, &old), "no change is not a transform");
    }
}
