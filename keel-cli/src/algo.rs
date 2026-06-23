//! Algorithmic views ported from `query.py` (D0074/D0075 M2.2b): `orphans` + `audit`.
//!
//! Unlike the declared-view Model (which spans `.tracking` + `.engine`), these match
//! `query.py`'s scope: both read `.tracking/` ONLY — `orphans` over every tracking file,
//! `audit` over `.tracking/delivery` plus charter-edge / sitting-review scans across all
//! of `.tracking`. Output is byte-identical to `query.py orphans` / `query.py audit` (the
//! migration parity bar) via the Python-`json.dumps(indent=2)`-compatible emitter.

use std::collections::{BTreeSet, HashSet};
use std::path::Path;

use keel_parser::ast::{Item, Package, Part, Value};
use keel_parser::{parse, tokenize};

use crate::json::Json;

/// A failure encountered while reading/parsing a tracking file.
#[derive(Debug, thiserror::Error)]
pub enum AlgoError {
    /// A tracking file could not be read or parsed.
    #[error("reading/parsing tracking file {0}: {1}")]
    Parse(String, String),
}

pub(crate) fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub(crate) const fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn str_value(v: &Value) -> Option<&str> {
    match v {
        Value::Str(s) | Value::Ident(s) => Some(s),
        _ => None,
    }
}

fn jint(n: usize) -> Json {
    Json::Int(i64::try_from(n).unwrap_or(i64::MAX))
}

fn tracking_packages(root: &Path) -> Result<Vec<Package>, AlgoError> {
    let mut pkgs = Vec::new();
    for path in crate::collect_sysml(&root.join(".tracking")) {
        let name = path.display().to_string();
        let src = std::fs::read_to_string(&path).map_err(|e| AlgoError::Parse(name.clone(), e.to_string()))?;
        let tokens = tokenize(&src, &name).map_err(|e| AlgoError::Parse(name.clone(), e.to_string()))?;
        let pkg = parse(tokens, &name).map_err(|e| AlgoError::Parse(name.clone(), e.to_string()))?;
        pkgs.push(pkg);
    }
    Ok(pkgs)
}

// ── orphans (orphansVP, D0056) ────────────────────────────────────────────────
// A task (`action`) with no `{name}DoD` verification, and Issues with no/dangling
// `relatedTask`, are broken traceability. Scope: `.tracking` only.

fn add_dod(name: &str, dods: &mut HashSet<String>) {
    if let Some(prefix) = name.strip_suffix("DoD") {
        if !prefix.is_empty() {
            dods.insert(prefix.to_string());
        }
    }
}

fn add_issue(p: &Part, issues: &mut Vec<(String, String)>) {
    if p.type_name.as_deref() == Some("Issue") {
        let rt = p
            .attributes
            .iter()
            .find(|a| a.name == "relatedTask")
            .and_then(|a| str_value(&a.value))
            .unwrap_or("")
            .to_string();
        issues.push((p.name.clone(), rt));
    }
}

fn collect_orphan_inputs(item: &Item, actions: &mut HashSet<String>, dods: &mut HashSet<String>, issues: &mut Vec<(String, String)>) {
    match item {
        Item::ActionDecl(a) => {
            actions.insert(a.name.clone());
        }
        Item::Verification(v) => add_dod(&v.name, dods),
        Item::Part(p) => add_issue(p, issues),
        Item::ActionDef(ad) => {
            for a in &ad.actions {
                actions.insert(a.name.clone());
            }
            for v in &ad.verifications {
                add_dod(&v.name, dods);
            }
            for p in &ad.parts {
                add_issue(p, issues);
            }
        }
        _ => {}
    }
}

/// Orphaned / dangling elements view (D0056) as JSON, byte-identical to `query.py orphans`.
///
/// # Errors
/// Returns [`AlgoError`] if a `.tracking` file cannot be read or parsed.
pub fn orphans(root: &Path) -> Result<String, AlgoError> {
    let pkgs = tracking_packages(root)?;
    let mut actions: HashSet<String> = HashSet::new();
    let mut dods: HashSet<String> = HashSet::new();
    let mut issues: Vec<(String, String)> = Vec::new();
    for pkg in &pkgs {
        for item in &pkg.items {
            collect_orphan_inputs(item, &mut actions, &mut dods, &mut issues);
        }
    }

    let mut without_dod: Vec<String> = actions.iter().filter(|a| !dods.contains(a.as_str())).cloned().collect();
    without_dod.sort();

    let mut no_rel: Vec<Json> = Vec::new();
    let mut dangling: Vec<Json> = Vec::new();
    for (name, rt) in &issues {
        if rt.is_empty() {
            no_rel.push(Json::s(name.clone()));
        } else if !actions.contains(rt.as_str()) {
            dangling.push(Json::Obj(vec![
                ("issue".to_string(), Json::s(name.clone())),
                ("relatedTask".to_string(), Json::s(rt.clone())),
            ]));
        }
    }

    let out = Json::Obj(vec![
        ("tasks_without_dod".to_string(), Json::Arr(without_dod.into_iter().map(Json::s).collect())),
        ("issues_without_relatedTask".to_string(), Json::Arr(no_rel)),
        ("issues_dangling_relatedTask".to_string(), Json::Arr(dangling)),
    ]);
    Ok(out.dump())
}

// ── audit (sprint-process adherence, D0046) ───────────────────────────────────
// Aggregates the dimensions the forward guards do not: charter coverage, ceremony
// completeness, estimation discipline, sitting-review currency — ACTIONABLE vs
// grandfathered. Text-based (matches query.py's regex semantics) for byte parity.

const CEREMONY_GATES: [&str; 6] = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"];
const CHARTER_SINCE: u32 = 38;

fn is_ceremony_grandfathered(fname: &str) -> bool {
    fname == "delivery.sysml" || fname == "sprint11_nativeSpikes.sysml"
}

/// The `part`-declared identifier at the start of a line (after optional indent and a
/// required space), or `None` if the line is not a `part <ident>` declaration.
fn part_ident(line: &str) -> Option<String> {
    let t = line.trim_start_matches(is_space);
    let after = t.strip_prefix("part")?;
    let after_ws = after.trim_start_matches(is_space);
    if after_ws.len() == after.len() {
        return None; // no whitespace after `part` (e.g. `partition`)
    }
    let ident: String = after_ws.chars().take_while(|c| is_word(*c)).collect();
    if ident.is_empty() {
        None
    } else {
        Some(ident)
    }
}

/// Story names declared in a file: `^[ \t]*part <ident> : Story\b`.
pub(crate) fn story_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(ident) = part_ident(line) else { continue };
        let t = line.trim_start_matches(is_space);
        let Some(rest) = t.strip_prefix("part").map(|r| r.trim_start_matches(is_space)) else { continue };
        let Some(rest) = rest.strip_prefix(ident.as_str()) else { continue };
        let rest = rest.trim_start_matches(is_space);
        let Some(rest) = rest.strip_prefix(':') else { continue };
        let rest = rest.trim_start_matches(is_space);
        let Some(tail) = rest.strip_prefix("Story") else { continue };
        if tail.chars().next().is_none_or(|c| !is_word(c)) {
            out.push(ident);
        }
    }
    out
}

/// `sprint(\d+)_` prefix of a delivery filename → the sprint number.
fn sprint_num(fname: &str) -> Option<u32> {
    let rest = fname.strip_prefix("sprint")?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    let after = rest.strip_prefix(digits.as_str())?;
    if digits.is_empty() || !after.starts_with('_') {
        return None;
    }
    digits.parse().ok()
}

/// `#CharteredBy dependency from <work> to <_>` — collect the chartered work names.
fn collect_charter_work(text: &str, out: &mut HashSet<String>) {
    for chunk in text.split("#CharteredBy").skip(1) {
        let mut it = chunk.split_whitespace();
        if it.next() != Some("dependency") || it.next() != Some("from") {
            continue;
        }
        let Some(work_tok) = it.next() else { continue };
        let work: String = work_tok.chars().take_while(|c| is_word(*c)).collect();
        if !work.is_empty() && it.next() == Some("to") {
            out.insert(work);
        }
    }
}

/// A name matches `sitting\w*R\d+`: starts with `sitting` and ends with `R` + digits.
fn is_sitting_review_name(name: &str) -> bool {
    if !name.starts_with("sitting") {
        return false;
    }
    let mut rev = name.chars().rev();
    let mut saw_digit = false;
    let mut next = rev.next();
    while let Some(c) = next {
        if c.is_ascii_digit() {
            saw_digit = true;
            next = rev.next();
        } else {
            break;
        }
    }
    saw_digit && next == Some('R')
}

/// Sitting-review pass records: `part <sitting…R\d+> : TestResult { …[^}]* VerdictKind::pass`.
/// Statement-scoped (matches query.py's `[^}]*`, which spans newlines) so multi-line
/// `TestResult` blocks are caught — mirrors [`crate::orient::gate_passed`].
fn collect_sitting_reviews(text: &str, out: &mut BTreeSet<String>) {
    for (idx, _) in text.match_indices("part ") {
        let after = &text[idx + "part ".len()..];
        let name: String = after.chars().take_while(|c| is_word(*c)).collect();
        if !is_sitting_review_name(&name) {
            continue;
        }
        let stmt_end = text[idx..].find('}').map_or(text.len(), |e| idx + e);
        let stmt = &text[idx..stmt_end];
        if stmt.contains(": TestResult") && stmt.contains("VerdictKind::pass") {
            out.insert(name);
        }
    }
}

/// Per-delivery-file scan accumulators (counts kept as `usize`; lists as ready-to-emit `Json`).
#[derive(Default)]
struct AuditScan {
    uncharted_actionable: Vec<Json>,
    uncharted_gf: usize,
    gates_actionable: Vec<Json>,
    gates_gf: Vec<Json>,
    missing_points: Vec<Json>,
    missing_hours: usize,
    sprint_files: usize,
}

/// Scan `.tracking/delivery` (basename order; `collect_sysml` is path-sorted = basename-sorted
/// here) for charter coverage, ceremony completeness, and estimation discipline.
fn scan_delivery_files(tracking: &Path, chartered: &HashSet<String>) -> AuditScan {
    let mut scan = AuditScan::default();
    for path in crate::collect_sysml(&tracking.join("delivery")) {
        let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let stories = story_names(&text);
        if stories.is_empty() {
            continue;
        }
        scan.sprint_files += 1;
        let charter_expected = sprint_num(&fname).is_some_and(|n| n >= CHARTER_SINCE);
        for s in &stories {
            if !chartered.contains(s.as_str()) {
                if charter_expected {
                    scan.uncharted_actionable.push(Json::Obj(vec![
                        ("file".to_string(), Json::s(fname.clone())),
                        ("story".to_string(), Json::s(s.clone())),
                    ]));
                } else {
                    scan.uncharted_gf += 1;
                }
            }
        }
        let absent: Vec<&str> = CEREMONY_GATES.iter().copied().filter(|g| !crate::orient::gate_passed(&text, g)).collect();
        if !absent.is_empty() {
            let entry = Json::Obj(vec![
                ("file".to_string(), Json::s(fname.clone())),
                ("missing".to_string(), Json::Arr(absent.into_iter().map(|g| Json::s(g.to_string())).collect())),
            ]);
            if is_ceremony_grandfathered(&fname) {
                scan.gates_gf.push(entry);
            } else {
                scan.gates_actionable.push(entry);
            }
        }
        if !text.contains("estimatedPoints") {
            scan.missing_points.push(Json::s(fname.clone()));
        }
        if !text.contains("actualHours") {
            scan.missing_hours += 1;
        }
    }
    scan
}

/// Build the findings list — ORDER IS SIGNIFICANT (mirrors query.py exactly).
fn build_findings(scan: &AuditScan, sitting_ok: bool) -> Vec<Json> {
    let charter_ok = scan.uncharted_actionable.is_empty();
    let ceremony_ok = scan.gates_actionable.is_empty();
    let mut findings: Vec<Json> = Vec::new();
    if !charter_ok {
        findings.push(Json::s(format!(
            "ACTIONABLE: {} post-sprint{CHARTER_SINCE} Story(ies) without a #CharteredBy edge",
            scan.uncharted_actionable.len()
        )));
    }
    if !ceremony_ok {
        findings.push(Json::s(format!("ACTIONABLE: {} non-grandfathered sprint(s) missing a passed gate", scan.gates_actionable.len())));
    }
    if scan.missing_hours > 0 {
        findings.push(Json::s(format!(
            "{}/{} sprint(s) record no actualHours — D0038 estimation-feedback discipline is dormant project-wide (decide: retire for AI-autonomous sprints, or revive) [tracked: issue022]",
            scan.missing_hours, scan.sprint_files
        )));
    }
    if !sitting_ok {
        findings.push(Json::s(
            "no per-sitting sprint-review record (D0049/D0073 human gate) — no sittingNNReviewRn pass artifact found",
        ));
    }
    if charter_ok && ceremony_ok {
        findings.insert(
            0,
            Json::s(format!(
                "PASS: every sprint that should comply (charter since sprint{CHARTER_SINCE}; full ceremony) does — recent sprints hold to ceremony + charter."
            )),
        );
    }
    findings
}

/// Retrospective sprint-process adherence audit (D0046) as JSON, byte-identical to
/// `query.py audit`.
///
/// # Errors
/// Returns [`AlgoError`] — never in practice (file reads that fail are skipped, matching
/// query.py); the `Result` keeps the signature uniform with [`orphans`].
pub fn audit(root: &Path) -> Result<String, AlgoError> {
    let tracking = root.join(".tracking");

    // Charter work names + sitting reviews, scanned across ALL tracking files.
    let mut chartered: HashSet<String> = HashSet::new();
    let mut sitting: BTreeSet<String> = BTreeSet::new();
    for path in crate::collect_sysml(&tracking) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        collect_charter_work(&text, &mut chartered);
        collect_sitting_reviews(&text, &mut sitting);
    }

    let scan = scan_delivery_files(&tracking, &chartered);
    let charter_ok = scan.uncharted_actionable.is_empty();
    let ceremony_ok = scan.gates_actionable.is_empty();
    let sitting_ok = !sitting.is_empty();
    let findings = build_findings(&scan, sitting_ok);

    let out = Json::Obj(vec![
        ("sprint_files".to_string(), jint(scan.sprint_files)),
        (
            "charter_coverage".to_string(),
            Json::Obj(vec![
                ("actionable_uncharted".to_string(), Json::Arr(scan.uncharted_actionable)),
                ("grandfathered_uncharted_count".to_string(), jint(scan.uncharted_gf)),
                ("ok".to_string(), Json::Bool(charter_ok)),
            ]),
        ),
        (
            "ceremony_completeness".to_string(),
            Json::Obj(vec![
                ("actionable_incomplete".to_string(), Json::Arr(scan.gates_actionable)),
                ("grandfathered_incomplete".to_string(), Json::Arr(scan.gates_gf)),
                ("ok".to_string(), Json::Bool(ceremony_ok)),
            ]),
        ),
        (
            "estimation_discipline".to_string(),
            Json::Obj(vec![
                ("missing_estimatedPoints".to_string(), Json::Arr(scan.missing_points)),
                ("missing_actualHours_count".to_string(), jint(scan.missing_hours)),
            ]),
        ),
        (
            "sitting_review".to_string(),
            Json::Obj(vec![
                ("records".to_string(), Json::Arr(sitting.iter().map(|s| Json::s(s.clone())).collect())),
                ("ok".to_string(), Json::Bool(sitting_ok)),
            ]),
        ),
        ("findings".to_string(), Json::Arr(findings)),
        (
            "note".to_string(),
            Json::s("Operationalizes the architectural-critique GQM adherence metrics (D0046); pairs with the per-commit guards. Findings should be filed as tracked Issues, not prose."),
        ),
    ]);
    Ok(out.dump())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dod_prefix_stripped() {
        let mut dods = HashSet::new();
        add_dod("portOrphansAuditDoD", &mut dods);
        add_dod("portOrphansAuditRefineGate", &mut dods);
        add_dod("DoD", &mut dods);
        assert!(dods.contains("portOrphansAudit"));
        assert!(!dods.contains("portOrphansAuditRefineGate"));
        assert_eq!(dods.len(), 1);
    }

    #[test]
    fn story_names_detected_and_filtered() {
        let text = "package P {\n    part fooStory : Story {\n    part barDoD : Test {\n  partition x\n    part bazStory:Story{\n}";
        assert_eq!(story_names(text), vec!["fooStory".to_string(), "bazStory".to_string()]);
    }

    #[test]
    fn sprint_num_parses_numbered_files_only() {
        assert_eq!(sprint_num("sprint53_portOrphansAudit.sysml"), Some(53));
        assert_eq!(sprint_num("sprint11_nativeSpikes.sysml"), Some(11));
        assert_eq!(sprint_num("delivery.sysml"), None);
        assert_eq!(sprint_num("sprintX_foo.sysml"), None);
    }

    #[test]
    fn sitting_review_name_recognized() {
        assert!(is_sitting_review_name("sitting07ReviewR1"));
        assert!(is_sitting_review_name("sittingR2"));
        assert!(!is_sitting_review_name("sitting07Review"));
        assert!(!is_sitting_review_name("fooR1"));
    }

    #[test]
    fn charter_work_collected_not_prose() {
        let mut out = HashSet::new();
        collect_charter_work("#CharteredBy dependency from portStory to d0075;", &mut out);
        // A prose mention ("a #CharteredBy edge") must NOT be collected.
        collect_charter_work("text without a #CharteredBy edge here", &mut out);
        assert!(out.contains("portStory"));
        assert_eq!(out.len(), 1);
    }
}
