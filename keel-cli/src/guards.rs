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
/// An element missing a required-lens critique (per the declared critique policy, D0097 — default
/// Core-3) is a violation. RUNNABLE via
/// `keel guard critique` but NOT yet in the enforced `GUARD_NAMES` set: with zero critiques
/// recorded it would block every commit. It joins the enforced set (a hard pre-commit gate, the
/// human-accepted choice) once a genuine critique pass brings the model to required-lens coverage.
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

/// Guard: the composite assurance-readiness gate (D0079 c).
///
/// Fails with the exact blockers when the deliverable is not assured (coverage/critique gaps, stale
/// verification, undispositioned >= Medium findings, open Critical, invariant violations). RUNNABLE
/// via `keel guard assured` but NOT in the enforced `GUARD_NAMES` (it subsumes the per-commit
/// guards and the not-yet-enforced critique gate — it is a readiness verdict, not a per-commit lock).
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
    "concern-coverage", "dispositions", "sitting-coverage", "critique-policy",
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

/// The ENFORCED forward guards, in CLI/runner order.
///
/// `issues` joined the enforced set at IRL-d (D0077). `critique` + `assured` joined at D0081 once
/// CHARTER-TIME scoping (D0068 freeze) made them safe to enforce: they bind only assurance elements
/// created after the governing decision (D0079/D0080), so pre-decision work is grandfathered and the
/// gates pass vacuously while holding all FUTURE requirements/needs/decisions to full rigor.
pub const GUARD_NAMES: [&str; 13] =
    ["actors", "acceptance-events", "sprint-coverage", "ceremony", "charter", "process-change", "issues", "critique", "assured", "viewpoint-renderer", "manifest-coverage", "critic-independence", "process-skill"];

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
        "critique-rigor" => Some(critique_rigor(root)), // runnable-only (not in GUARD_NAMES)
        "defect-guard-coverage" => Some(defect_guard_coverage(root)), // runnable-only (D0047/issue039)
        _ => None,
    }
}

/// Run all enforced guards over `root`, returning their reports in `GUARD_NAMES` order.
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
}
