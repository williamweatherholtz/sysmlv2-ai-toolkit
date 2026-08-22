//! element-content staleness (D0084) - targeted suspicion on element change - extracted from view.rs (sprint 418, dcViewRsRestructure: the panel's
//! god-module finding). Pure move, no behavior change; `view::` paths survive via the
//! `pub use` re-exports in mod.rs.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use crate::json::Json;


#[allow(clippy::wildcard_imports)] // a pure move-only split: the parent's vocabulary IS this file's vocabulary
use super::*;

// ── element-content staleness (D0084 — targeted suspicion: re-verify/re-critique on element change) ──
// A verify/critique of an assurance element goes SUSPECT when the element's SEMANTIC field changed
// since the verification's latest result commit AND the element existed then (so same-sprint
// create+verify isn't falsely flagged). Reuses orient's batched `git cat-file`. Honors D0005's
// material-change intent at the element grain that coverage/critique actually depend on.

/// The semantic field whose change should re-suspect verification of this element type.
fn semantic_field(type_name: &str) -> Option<&'static str> {
    match type_name {
        "Need" | "SystemRequirement" => Some("statement"),
        "Decision" => Some("decision"),
        _ => None,
    }
}

/// The forward-only line for `impossible_evidence_dates` (D0162): a result whose cited COMMIT predates
/// this date is exempt. Keyed on the commit rather than on `judgedAt` so that no future stamp can escape
/// by being attached to an old judgment - which is exactly the defect that motivated the guard.
const GRANDFATHER_BEFORE: &str = "2026-08-18";

/// `TestResult`s whose `judgedAgainst` commit POSTDATES their `judgedAt` (issue144).
///
/// A judgment cannot have been made against a commit that did not yet exist. This is not a pedantic
/// date check — it catches the one mechanical way a recorded attestation becomes a lie without anyone
/// intending it: a bulk stamp that rewrites every `PENDING` SHA in a file, including results whose judge
/// was a HUMAN who never saw that commit. That happened, to a `method=confirmation` result from the day
/// before, and no existing guard could see it because every field was individually well-formed.
///
/// Day granularity, and the commit's own date must be strictly LATER than `judgedAt` to violate: a
/// same-day stamp is the normal case, and clock skew within a day is not evidence of anything.
///
/// Returns `(scanned, violations)`. Unresolvable SHAs are SKIPPED, not flagged — `evidence-resolves`
/// owns that, and a guard that reports two different failures for one field teaches nothing.
///
/// # Errors
/// If the model cannot be built from `root`.
pub fn impossible_evidence_dates(root: &Path) -> Result<(usize, Vec<String>), String> {
    let model = Model::build(root).map_err(|e| e.to_string())?;
    let mut scanned = 0usize;
    let mut out = Vec::new();
    let mut dates: HashMap<String, String> = HashMap::new();
    for (name, info) in &model.items {
        let (Some(sha), Some(at)) = (info.attrs.get("judgedAgainst"), info.attrs.get("judgedAt")) else {
            continue;
        };
        if sha.is_empty() || sha == "PENDING" || at.is_empty() {
            continue;
        }
        scanned += 1;
        // Memoised per DISTINCT sha: 3799 results cite far fewer commits, and each miss is a subprocess.
        let commit_date = dates
            .entry(sha.clone())
            .or_insert_with(|| crate::govern::commit_date(root, sha).unwrap_or_default())
            .clone();
        if commit_date.is_empty() {
            continue; // unresolvable — evidence-resolves owns that failure
        }
        // FORWARD-ONLY (D0162, and issue068's rule that correct-when-written work is never retro-failed).
        // The corpus holds thirteen pre-existing violations, all in two commits and all exactly one day:
        // sessions that crossed midnight, where the judgment was recorded before 00:00 and the commit
        // landed after. Those are benign, and five of them are the human's own attestations, which D0108
        // ownership forbids me from editing. So the line is drawn at this guard's introduction rather than
        // by rewriting anyone's dates.
        // The line is drawn on WHEN THE CITED COMMIT WAS MADE, not on when the judgment claims to be
        // from. Drawing it on `judgedAt` was my first attempt and it exempted the very defect this guard
        // exists for: a result dated YESTERDAY that a bulk stamp pointed at TODAY's commit is old by
        // judgedAt and brand new by evidence. Keying on the commit means every stamp from here on is in
        // scope no matter how old the judgment it is attached to.
        if commit_date.as_str() < GRANDFATHER_BEFORE {
            continue;
        }
        if commit_date.as_str() > at.as_str() {
            let by = info.attrs.get("judgedBy").cloned().unwrap_or_default();
            out.push(format!(
                "{name}: judgedAt {at} but judgedAgainst {sha} was committed {commit_date} - a judgment cannot be made against a commit that did not exist yet (judgedBy {by})"
            ));
        }
    }
    out.sort();
    Ok((scanned, out))
}

/// The INTAKE view (D0166): what was said, what it became, and what nobody acted on.
///
/// Three gaps, none of which was computable before:
///   UNPARSED  - a `Statement` no `UserStory` cites. Direction they gave that nothing translated.
///   UNROUTED  - a `UserStory` whose `implication` is not `none` and which reaches no downstream item.
///               Triaged and then dropped, which is worse than untriaged because it looks handled.
///   UNSOURCED - a `Need` / `SystemRequirement` / `Issue` / `Decision` that no `UserStory` implicates. Work with
///               no recorded human statement behind it. Expected to be large and expected to be
///               uncomfortable: it is the ratio of what was asked for to what I invented.
///
/// UNSOURCED IS A FLOOR, NOT A TOTAL, and the view says so: nothing can force a statement to be
/// recorded, so an item may be genuinely requested and simply have no `Statement` written down. The
/// number measures the RECORD's completeness, never the human's.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub fn intake(root: &Path) -> Result<String, ViewError> {
    // The item types a UserStory can implicate. Declared at the top: an item after statements is
    // confusing because items exist from the start of the scope regardless.
    const DOWNSTREAM: [&str; 4] = ["Need", "SystemRequirement", "Issue", "Decision"];
    // Kinds that owe no downstream item: an acceptance produces nothing, its outcome IS the
    // acknowledgement; a question is answered, a priority applied by reordering, a convention adopted in
    // prose, a correction absorbed by an existing record. Declared with DOWNSTREAM because both are items
    // and an item after statements reads as a surprise.
    const SELF_TERMINATING: [&str; 6] =
        ["none", "attestation", "question", "priority", "convention", "correction"];
    let model = Model::build(root)?;
    let is = |n: &str, ty: &str| model.items.get(n).is_some_and(|i| i.type_name == ty);

    let statements: Vec<&String> =
        model.items.iter().filter(|(_, i)| i.type_name == "Statement").map(|(n, _)| n).collect();
    let stories: Vec<&String> =
        model.items.iter().filter(|(_, i)| i.type_name == "UserStory").map(|(n, _)| n).collect();

    // a story cites its statement with #DerivedFrom; it names its outcome with #Implicates
    let mut cited: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let mut routed: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let mut implicated: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for e in &model.edges {
        if e.kind == "derivedfrom" && is(&e.from, "UserStory") && is(&e.to, "Statement") {
            cited.insert(e.to.as_str());
        }
        if e.kind == "implicates" && is(&e.from, "UserStory") {
            routed.insert(e.from.as_str());
            implicated.insert(e.to.as_str());
        }
    }

    let mut unparsed: Vec<String> =
        statements.iter().filter(|s| !cited.contains(s.as_str())).map(|s| (*s).clone()).collect();
    // ONLY THE PRODUCTIVE KINDS OWE AN OUTCOME. The first version required a downstream item for every
    // implication except `none`, and using it immediately flagged a legitimate `attestation` as unrouted:
    // an acceptance PRODUCES nothing, its outcome IS the acknowledgement. Same for a question answered, a
    // priority applied by reordering, a convention adopted in prose, a correction that edits an existing
    // record. Reporting those as gaps would train the reader to ignore the number - the warning-fatigue
    // failure issue160 records one layer up. They are counted separately as self-terminating, so they stay
    // visible without being defects.
    let kind_of = |s: &str| -> String {
        model
            .items
            .get(s)
            .and_then(|i| i.attrs.get("implication"))
            .map(|k| k.rsplit("::").next().unwrap_or(k).to_string())
            .unwrap_or_default()
    };
    let mut unrouted: Vec<String> = stories
        .iter()
        .filter(|s| {
            !SELF_TERMINATING.contains(&kind_of(s).as_str()) && !routed.contains(s.as_str())
        })
        .map(|s| (*s).clone())
        .collect();
    let self_terminating = stories
        .iter()
        .filter(|s| SELF_TERMINATING.contains(&kind_of(s).as_str()))
        .count();
    // A story with NO #DerivedFrom is an invention wearing a story's clothes - reported separately from
    // unrouted, because the two need different fixes: one needs a source, the other needs an outcome.
    let mut unsourced_stories: Vec<String> = stories
        .iter()
        .filter(|s| !model.edges.iter().any(|e| e.kind == "derivedfrom" && e.from == ***s))
        .map(|s| (*s).clone())
        .collect();

    let mut unsourced: Vec<String> = model
        .items
        .iter()
        .filter(|(n, i)| DOWNSTREAM.contains(&i.type_name.as_str()) && !implicated.contains(n.as_str()))
        .map(|(n, _)| n.clone())
        .collect();
    let downstream_total = model
        .items
        .values()
        .filter(|i| DOWNSTREAM.contains(&i.type_name.as_str()))
        .count();

    for v in [&mut unparsed, &mut unrouted, &mut unsourced_stories, &mut unsourced] {
        v.sort();
    }
    let cap = |v: &[String], n: usize| -> Json {
        Json::Arr(v.iter().take(n).map(|s| Json::s(s.clone())).collect())
    };

    // per-implication tally, so the triage distribution is visible rather than inferred
    let mut by_kind: BTreeMap<String, i64> = BTreeMap::new();
    for s in &stories {
        let k = model.items.get(*s).and_then(|i| i.attrs.get("implication")).cloned().unwrap_or_default();
        *by_kind.entry(k.rsplit("::").next().unwrap_or("unrecorded").to_string()).or_insert(0) += 1;
    }

    Ok(Json::Obj(vec![
        ("statements".to_string(), Json::Int(i64::try_from(statements.len()).unwrap_or(0))),
        ("userStories".to_string(), Json::Int(i64::try_from(stories.len()).unwrap_or(0))),
        ("unparsed".to_string(), Json::Int(i64::try_from(unparsed.len()).unwrap_or(0))),
        ("unparsed_statements".to_string(), cap(&unparsed, 20)),
        ("unrouted".to_string(), Json::Int(i64::try_from(unrouted.len()).unwrap_or(0))),
        ("unrouted_stories".to_string(), cap(&unrouted, 20)),
        ("selfTerminating".to_string(), Json::Int(i64::try_from(self_terminating).unwrap_or(0))),
        ("storiesWithNoStatement".to_string(), Json::Int(i64::try_from(unsourced_stories.len()).unwrap_or(0))),
        ("storiesWithNoStatement_list".to_string(), cap(&unsourced_stories, 20)),
        ("downstreamItems".to_string(), Json::Int(i64::try_from(downstream_total).unwrap_or(0))),
        ("unsourced".to_string(), Json::Int(i64::try_from(unsourced.len()).unwrap_or(0))),
        ("unsourced_sample".to_string(), cap(&unsourced, 20)),
        ("byImplication".to_string(), Json::Obj(by_kind.into_iter().map(|(k, v)| (k, Json::Int(v))).collect())),
        (
            "unsourcedNote".to_string(),
            Json::s(
                "unsourced counts downstream items no UserStory implicates. It is a FLOOR on the                  record's completeness, never a claim about the human: nothing can force a statement to                  be written down, so an item may be genuinely requested and simply unrecorded."
                    .to_string(),
            ),
        ),
    ])
    .dump())
}

/// `(outcome, judgedAgainst)` of the HIGHEST-numbered `<v>R<n>` result for verification `v`.
pub(super) fn latest_result(model: &Model, v: &str) -> Option<(String, String)> {
    let mut best: Option<(u32, String, String)> = None;
    for (name, info) in &model.items {
        let Some(suf) = name.strip_prefix(v) else { continue };
        let Some(digits) = suf.strip_prefix('R') else { continue };
        if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let Ok(n) = digits.parse::<u32>() else { continue };
        if best.as_ref().is_none_or(|(bn, _, _)| n > *bn) {
            let outcome = info.attrs.get("outcome").cloned().unwrap_or_default();
            let sha = info.attrs.get("judgedAgainst").cloned().unwrap_or_default();
            best = Some((n, outcome, sha));
        }
    }
    best.map(|(_, o, s)| (o, s))
}

/// Map each assurance element (`requirement <n/sr>` / `part <d> : Decision`) to its repo-relative
/// file (one working-tree pass, no git) — to fetch its historical content for staleness.
fn build_element_files(root: &Path) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    let dirs = [root.join(".tracking"), root.join(".engine").join("decisions")];
    for path in dirs.iter().flat_map(|d| crate::collect_sysml(d)) {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Some(rel) = path.strip_prefix(root).ok().and_then(std::path::Path::to_str).map(|s| s.replace('\\', "/")) else {
            continue;
        };
        for line in text.lines() {
            let t = line.trim_start();
            let name = t.strip_prefix("requirement ").or_else(|| t.strip_prefix("part ")).and_then(|r| r.split([' ', ':']).next());
            if let Some(name) = name {
                if !name.is_empty() {
                    out.entry(name.to_owned()).or_insert_with(|| rel.clone());
                }
            }
        }
    }
    out
}

/// Extract `element`'s `:>> <field> = "..."` value from a file blob (the FIRST occurrence inside the
/// element's block). `None` if the element/field isn't present (e.g. it didn't exist at that commit).
pub(super) fn extract_field(blob: &str, element: &str, field: &str) -> Option<String> {
    let decl_r = format!("requirement {element} ");
    let decl_p = format!("part {element} ");
    let fieldpat = format!(":>> {field} = \"");
    let mut in_elem = false;
    for line in blob.lines() {
        let t = line.trim_start();
        if !in_elem {
            if t.starts_with(&decl_r) || t.starts_with(&decl_p) {
                in_elem = true;
            }
            continue;
        }
        if t.starts_with("part ") || t.starts_with("requirement ") || t.starts_with("verification ") || t.starts_with("action ") {
            break; // next top-level item — left the element's block without finding the field
        }
        if let Some(idx) = t.find(&fieldpat) {
            let rest = t.get(idx + fieldpat.len()..)?;
            return Some(decode_string_body(rest));
        }
    }
    None
}

/// Decode a string-literal body (the text after the opening quote) up to the first unescaped quote,
/// applying the SAME escape rules as the lexer's `lex_string` (backslash-backslash, backslash-quote,
/// `\n`, `\t`). Without this, a raw git-blob read of an escaped field (e.g. a regex containing a
/// backslash) compares unequal to the parsed model value and the element's critiques are falsely
/// flagged stale (issue044) — undercounting critique coverage. Keeps blob-extract == model-attr.
fn decode_string_body(rest: &str) -> String {
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('"') => out.push('"'),
                Some('\\') | None => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
            },
            other => out.push(other),
        }
    }
    out
}

/// Names of verify/critique Tests that are STALE: their target assurance element's semantic field
/// changed since the Test's latest result commit, and the element existed at that commit (D0084).
pub(super) fn compute_stale_verifications(root: &Path, model: &Model) -> HashSet<String> {
    let elem_files = build_element_files(root);
    let mut work: Vec<(String, String, &'static str, String)> = Vec::new(); // (test, element, field, sha)
    let mut keys: HashSet<String> = HashSet::new();
    for e in model.edges.iter().filter(|e| e.kind == "verify") {
        let Some(info) = model.items.get(&e.to) else { continue };
        if info.type_name == "Decision" && info.attrs.get("status").map(String::as_str) != Some("accepted") {
            continue;
        }
        let Some(field) = semantic_field(&info.type_name) else { continue };
        let Some((_, sha)) = latest_result(model, &e.from) else { continue };
        if sha.is_empty() {
            continue;
        }
        let Some(rel) = elem_files.get(&e.to) else { continue };
        keys.insert(format!("{sha}:{rel}"));
        work.push((e.from.clone(), e.to.clone(), field, sha));
    }
    let blobs = crate::orient::batch_cat_blobs(root, &keys.into_iter().collect::<Vec<_>>());
    let mut stale: HashSet<String> = HashSet::new();
    for (test, element, field, sha) in work {
        let Some(rel) = elem_files.get(&element) else { continue };
        let Some(Some(blob)) = blobs.get(&format!("{sha}:{rel}")) else { continue }; // file absent at sha
        let Some(old) = extract_field(blob, &element, field) else { continue }; // element absent then -> not stale
        let cur = model.items.get(&element).and_then(|i| i.attrs.get(field)).map_or("", String::as_str);
        if old != cur {
            stale.insert(test);
        }
    }
    stale
}

/// Direct verifiers of `target`, strongest first: explicit-test edge → accept-event (Decisions
/// only) → charter-dod. Needs add a transitive `satisfy` verifier separately (it depends on
/// requirement coverage computed first). `stale` (D0084) marks verify-edge Tests whose target's
/// content drifted since they were judged.
fn direct_verifiers<S: std::hash::BuildHasher>(
    model: &Model,
    target: &str,
    is_decision: bool,
    done: &HashSet<String, S>,
    task_suspect: &HashSet<String, S>,
    stale: &HashSet<String, S>,
) -> Vec<Verifier> {
    let mut vs: Vec<Verifier> = Vec::new();
    for e in model.edges.iter().filter(|e| e.kind == "verify" && e.to == target) {
        // A verify-edge Test is complete iff its LATEST TestResult passed — read from the Model
        // (a standalone `verification`/`part <name>R<n> : TestResult`), NOT the action-task `done`
        // set (these Tests aren't action DoDs). Mirrors accept-event + critique-coverage.
        // EXCLUDE method=critique Tests: critiques are also #Verify-linked but belong to
        // critique-coverage (an adversarial lens), not objective assurance coverage (D0082).
        let src = model.items.get(&e.from);
        if src.and_then(|i| i.attrs.get("method")).map(String::as_str) == Some("critique") {
            continue;
        }
        let pass = latest_result(model, &e.from).is_some_and(|(o, _)| o == "pass");
        vs.push(Verifier {
            complete: pass,
            suspect: stale.contains(&e.from), // D0084: element-content drift since the verifying commit
            name: e.from.clone(),
            kind: "explicit-test",
        });
    }
    // A Decision's canonical assurance is its recorded human acceptance event (D0066): a passing
    // `<decision>AcceptR1` TestResult. (Attestation-staleness is a future refinement.)
    if is_decision {
        let ev = format!("{target}AcceptR1");
        if model.items.get(&ev).and_then(|i| i.attrs.get("outcome")).map(String::as_str) == Some("pass") {
            vs.push(Verifier { name: ev, kind: "accept-event", complete: true, suspect: false });
        }
    }
    for e in model.edges.iter().filter(|e| e.kind == "charteredby" && e.to == target) {
        let forms = charter_forms(&e.from);
        vs.push(Verifier {
            complete: forms.iter().any(|f| done.contains(f)),
            suspect: forms.iter().any(|f| task_suspect.contains(f)),
            name: e.from.clone(),
            kind: "charter-dod",
        });
    }
    vs
}

pub(super) fn compute_coverage<S: std::hash::BuildHasher>(
    model: &Model,
    done: &HashSet<String, S>,
    task_suspect: &HashSet<String, S>,
    stale: &HashSet<String, S>,
) -> Vec<Coverage> {
    // Pass 1: requirements + decisions (their coverage is direct).
    let mut req_tier: HashMap<String, &'static str> = HashMap::new();
    let mut out: Vec<Coverage> = Vec::new();
    // Assurance targets: Needs, SystemRequirements, and ACCEPTED Decisions only. Superseded /
    // rejected / proposed Decisions are not active commitments (mirrors the attestation guard's
    // accepted-only scope, D0066) — including them would report false gaps.
    let is_target = |i: &ItemInfo| match i.type_name.as_str() {
        "Need" | "SystemRequirement" => true,
        "Decision" => i.attrs.get("status").map(String::as_str) == Some("accepted"),
        _ => false,
    };
    // issue088, second half: a Need or SystemRequirement carrying an incoming `#Supersede` edge was
    // DESCOPED by a Decision (§2.4), and is no more an active commitment than a superseded Decision
    // is — the exact reasoning the comment above already applies one type over. Found by checking
    // whether the tier-satisfaction blind spot was shared; it was, in this view but not in
    // `rootedness` (which measures charter over Stories, not decomposition).
    let descoped: HashSet<&str> =
        model.edges.iter().filter(|e| e.kind == "supersede").map(|e| e.to.as_str()).collect();
    let mut targets: Vec<(&String, &ItemInfo)> =
        model.items.iter().filter(|(n, i)| is_target(i) && !descoped.contains(n.as_str())).collect();
    targets.sort_by(|a, b| a.0.cmp(b.0));
    for (name, info) in &targets {
        if info.type_name == "Need" {
            continue; // pass 2
        }
        let verifiers = direct_verifiers(model, name, info.type_name == "Decision", done, task_suspect, stale);
        let (tier, basis) = tier_of(&verifiers);
        if info.type_name == "SystemRequirement" {
            req_tier.insert((*name).clone(), tier);
        }
        out.push(Coverage { element: (*name).clone(), type_name: info.type_name.clone(), tier, basis, verifiers });
    }
    // Pass 2: needs — direct verifiers plus a transitive `satisfy` verifier that confers `verified`
    // ONLY when the satisfied requirement is itself verified (transitive satisfaction, the contract).
    for (name, info) in &targets {
        if info.type_name != "Need" {
            continue;
        }
        let mut verifiers = direct_verifiers(model, name, false, done, task_suspect, stale);
        for e in model.edges.iter().filter(|e| e.kind == "satisfy" && &e.from == *name) {
            let req_verified = req_tier.get(&e.to).copied() == Some("verified");
            verifiers.push(Verifier { name: e.to.clone(), kind: "satisfy", complete: req_verified, suspect: false });
        }
        let (tier, basis) = tier_of(&verifiers);
        out.push(Coverage { element: (*name).clone(), type_name: info.type_name.clone(), tier, basis, verifiers });
    }
    out.sort_by_key(|c| (c.type_name.clone(), c.element.clone()));
    out
}

/// Assurance-coverage view (D0079 C) as JSON.
///
/// Emits per-element coverage state + basis + verifiers, a per-type summary, and the flat gap set
/// (uncovered + suspect). Reuses the orient done/suspect authorities — never stores a verdict.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let done = crate::orient::done_names(root);
    let task_suspect: HashSet<String> = crate::orient::compute(root).suspect.into_iter().collect();
    let stale = crate::perf::phase("staleVerifications", || compute_stale_verifications(root, &model));
    let cov = compute_coverage(&model, &done, &task_suspect, &stale);
    let gf = crate::govern::grandfathered_under(root, COVERAGE_DECISION);

    // Per-type summary, counted by TIER (D0082).
    let mut summary: Vec<Json> = Vec::new();
    for ty in ASSURANCE_TYPES {
        let rows: Vec<&Coverage> = cov.iter().filter(|c| c.type_name == ty).collect();
        if rows.is_empty() {
            continue;
        }
        let count = |t: &str| i64::try_from(rows.iter().filter(|c| c.tier == t).count()).unwrap_or(i64::MAX);
        summary.push(Json::Obj(vec![
            ("type".to_string(), Json::s(ty)),
            ("total".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX))),
            ("verified".to_string(), Json::Int(count("verified"))),
            ("attested".to_string(), Json::Int(count("attested"))),
            ("addressed".to_string(), Json::Int(count("addressed"))),
            ("suspect".to_string(), Json::Int(count("suspect"))),
            ("uncovered".to_string(), Json::Int(count("uncovered"))),
        ]));
    }

    let elements: Vec<Json> = cov
        .iter()
        .map(|c| {
            let verifiers: Vec<Json> = c
                .verifiers
                .iter()
                .map(|v| {
                    Json::Obj(vec![
                        ("name".to_string(), Json::s(v.name.clone())),
                        ("kind".to_string(), Json::s(v.kind)),
                        ("complete".to_string(), Json::Bool(v.complete)),
                        ("suspect".to_string(), Json::Bool(v.suspect)),
                    ])
                })
                .collect();
            Json::Obj(vec![
                ("element".to_string(), Json::s(c.element.clone())),
                ("type".to_string(), Json::s(c.type_name.clone())),
                ("tier".to_string(), Json::s(c.tier)),
                ("basis".to_string(), c.basis.map_or(Json::Null, Json::s)),
                ("governed".to_string(), Json::Bool(governed(gf.as_ref(), &c.element))),
                ("verifiers".to_string(), Json::Arr(verifiers)),
            ])
        })
        .collect();

    // A tier counts as covered for the GATE iff verified or attested (D0082). addressed/suspect/
    // uncovered are gaps. Full gap set (honest measurement) + the GOVERNED subset the gate uses.
    let gaps: Vec<Json> =
        cov.iter().filter(|c| !is_covered_tier(c.tier)).map(|c| Json::s(c.element.clone())).collect();
    let governed_gaps: Vec<Json> =
        cov.iter().filter(|c| !is_covered_tier(c.tier) && governed(gf.as_ref(), &c.element)).map(|c| Json::s(c.element.clone())).collect();

    let out = Json::Obj(vec![
        (
            "assurance".to_string(),
            Json::s("coverage tiers (D0082): verified (reproducible verify-edge evidence; needs transitively via a verified requirement) > attested (decision acceptance event) > addressed (charter-dod work only — a claim) > uncovered. Gate-covered = verified|attested. (D0079 C)"),
        ),
        ("summary".to_string(), Json::Arr(summary)),
        ("gaps".to_string(), Json::Arr(gaps)),
        ("governed_gaps".to_string(), Json::Arr(governed_gaps)),
        ("elements".to_string(), Json::Arr(elements)),
    ]);
    Ok(out.dump())
}

