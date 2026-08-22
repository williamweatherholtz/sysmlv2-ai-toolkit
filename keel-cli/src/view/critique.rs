//! critique coverage (D0080/D0079) - per-element x required-lens - extracted from view.rs (sprint 418, dcViewRsRestructure: the panel's
//! god-module finding). Pure move, no behavior change; `view::` paths survive via the
//! `pub use` re-exports in mod.rs.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use crate::json::Json;


#[allow(clippy::wildcard_imports)] // a pure move-only split: the parent's vocabulary IS this file's vocabulary
use super::*;

// ── critique coverage (D0080/D0079 — per-element x required-lens critique coverage) ────────────
// An antagonistic critique is a `method=critique` Test with a `lens`, `#Verify`-linked to its
// target (parsed as a "verify" marker-edge), with a result by an INDEPENDENT critic (the result's
// judgedBy must differ from the target's createdBy). An element is critique-COVERED iff every
// REQUIRED lens for its type has such a critique. The required-lens policy is DECLARED, not hardcoded
// (D0097): read from `.engine/contracts/critique-policy.toml` (downstream-overridable), with the
// "Core-3" default (Need/SystemRequirement -> completeness/correctness/testability; Decision ->
// completeness/correctness/feasibility) as the built-in fallback when the file is absent. Honest by
// construction: with no critiques recorded, every element is uncovered. (Git-temporal critique-
// staleness reuses the suspect machinery.)

/// The seven `CritiqueLens` variants (schema/core `element.sysml`) — the requirement-quality canon.
/// A declared policy lens MUST be one of these (fail-loud otherwise).
const CANON_LENSES: [&str; 7] =
    ["completeness", "correctness", "ambiguity", "testability", "feasibility", "consistency", "necessity"];

/// The declared critique policy (D0097): required critique lenses per assurance-element type.
///
/// Read from `.engine/contracts/critique-policy.toml`. A type with a non-empty lens list is a critique
/// TARGET; each listed lens needs an independent `method=critique` verification for an element of that
/// type to be critique-covered. Downstream projects override the file; absent it, the built-in Core-3
/// applies.
pub struct CritiquePolicy {
    lenses: BTreeMap<String, Vec<String>>,
    pub(super) from_file: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CritiquePolicyFile {
    #[serde(default)]
    lenses: BTreeMap<String, Vec<String>>,
}

impl CritiquePolicy {
    /// The built-in Core-3 default (D0080) — identical to the shipped `critique-policy.toml`, used as
    /// the fallback when no policy file is present so behavior is unchanged with or without the file.
    pub(super) fn core3() -> Self {
        let mut lenses = BTreeMap::new();
        let req = || vec!["completeness".to_string(), "correctness".to_string(), "testability".to_string()];
        lenses.insert("Need".to_string(), req());
        lenses.insert("SystemRequirement".to_string(), req());
        lenses.insert(
            "Decision".to_string(),
            vec!["completeness".to_string(), "correctness".to_string(), "feasibility".to_string()],
        );
        Self { lenses, from_file: false }
    }

    /// Load the declared policy from `.engine/contracts/critique-policy.toml`, falling back to the
    /// built-in Core-3 default when the file is absent. Validates every lens name against the canon.
    ///
    /// # Errors
    /// Returns [`ViewError::Toml`] if the file is malformed, or [`ViewError::Policy`] if it lists an
    /// unknown lens name.
    pub fn load(root: &Path) -> Result<Self, ViewError> {
        let path = root.join(".engine").join("contracts").join("critique-policy.toml");
        let Ok(text) = std::fs::read_to_string(&path) else { return Ok(Self::core3()) };
        let parsed: CritiquePolicyFile =
            toml::from_str(&text).map_err(|e| ViewError::Toml(path.display().to_string(), Box::new(e)))?;
        for (ty, lenses) in &parsed.lenses {
            for l in lenses {
                if !CANON_LENSES.contains(&l.as_str()) {
                    return Err(ViewError::Policy(format!(
                        "type '{ty}' lists unknown lens '{l}' (valid: {})",
                        CANON_LENSES.join(" | ")
                    )));
                }
            }
        }
        Ok(Self { lenses: parsed.lenses, from_file: true })
    }

    /// Lenient load for ADVISORY aggregate reports: falls back to the Core-3 default on any error. A
    /// malformed policy is surfaced loudly by `critique-coverage` / `guard critique` (same gate), so the
    /// report cards needn't re-raise it.
    pub(super) fn load_or_core3(root: &Path) -> Self {
        Self::load(root).unwrap_or_else(|_| Self::core3())
    }

    /// Required critique lenses for an element type (empty slice for non-targets).
    pub(super) fn required_lenses(&self, type_name: &str) -> &[String] {
        self.lenses.get(type_name).map_or(&[], Vec::as_slice)
    }

    /// Whether an element TYPE is a critique target (has >= 1 required lens declared).
    pub(super) fn is_target_type(&self, type_name: &str) -> bool {
        self.lenses.get(type_name).is_some_and(|v| !v.is_empty())
    }

    /// The declared target types, sorted (the `BTreeMap` keeps them ordered).
    pub(super) fn target_types(&self) -> impl Iterator<Item = &String> {
        self.lenses.iter().filter(|(_, v)| !v.is_empty()).map(|(k, _)| k)
    }
}

pub(super) struct LensStatus {
    pub(super) lens: String,
    pub(super) critiqued: bool,
    critic: Option<String>,  // result judgedBy (independent of the target author)
    outcome: Option<String>, // pass = survived the lens; fail = a finding was raised
}

pub(super) struct CritiqueCoverage {
    pub(super) element: String,
    pub(super) type_name: String,
    pub(super) lenses: Vec<LensStatus>,
    pub(super) covered: bool, // every required lens critiqued
}

pub(super) fn compute_critique_coverage<S: std::hash::BuildHasher>(
    model: &Model,
    stale: &HashSet<String, S>,
    policy: &CritiquePolicy,
) -> Vec<CritiqueCoverage> {
    // Targets = the policy's declared types (D0097). Decisions are critiqued only once accepted (an
    // accepted Decision is a final commitment) — that accepted-only rule is intrinsic, not config.
    let is_target = |i: &ItemInfo| {
        if !policy.is_target_type(&i.type_name) {
            return false;
        }
        if i.type_name == "Decision" {
            return i.attrs.get("status").map(String::as_str) == Some("accepted");
        }
        true
    };
    let mut targets: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| is_target(i)).collect();
    targets.sort_by(|a, b| a.0.cmp(b.0));
    targets
        .into_iter()
        .map(|(name, info)| {
            let author = info.attrs.get("createdBy").map_or("", String::as_str);
            let lenses: Vec<LensStatus> = policy
                .required_lenses(&info.type_name)
                .iter()
                .map(|lens| {
                    let lens = lens.as_str();
                    // A critique of this element via this lens: a verify-edge (critique -> element)
                    // whose source is a method=critique Test with this lens, having an independent result.
                    let mut critiqued = false;
                    let mut critic = None;
                    let mut outcome = None;
                    for e in model.edges.iter().filter(|e| e.kind == "verify" && e.to == *name) {
                        let Some(c) = model.items.get(&e.from) else { continue };
                        if c.attrs.get("method").map(String::as_str) != Some("critique") {
                            continue;
                        }
                        if c.attrs.get("lens").map(String::as_str) != Some(lens) {
                            continue;
                        }
                        // D0084: a critique whose target's content drifted since it ran is STALE —
                        // it no longer covers the lens (re-critique needed).
                        if stale.contains(&e.from) {
                            continue;
                        }
                        let res = model.items.get(&format!("{}R1", e.from));
                        let by = res.and_then(|r| r.attrs.get("judgedBy")).map(String::as_str);
                        let out = res.and_then(|r| r.attrs.get("outcome")).map(String::as_str);
                        // Independence: the critic must differ from the target's author.
                        if let (Some(by), Some(out)) = (by, out) {
                            if by != author {
                                critiqued = true;
                                critic = Some(by.to_string());
                                outcome = Some(out.to_string());
                                break;
                            }
                        }
                    }
                    LensStatus { lens: lens.to_string(), critiqued, critic, outcome }
                })
                .collect();
            let covered = !lenses.is_empty() && lenses.iter().all(|l| l.critiqued);
            CritiqueCoverage { element: name.clone(), type_name: info.type_name.clone(), lenses, covered }
        })
        .collect()
}

// Charter-time governance (D0068/D0081): the assurance requirements are PROSPECTIVE — they bind only
// elements created after the governing decision landed. coverage(C) is governed by D0079; the
// critique requirement by D0080. Pre-decision elements are grandfathered (out of the GATE's gap set,
// though still shown in the VIEW with `governed=false` for transparency).
pub(super) const COVERAGE_DECISION: &str = "d0079";
pub(super) const CRITIQUE_DECISION: &str = "d0080";
/// The sitting-review grandfather line (D0155): sittings present at this Decision's introduction commit
/// are accepted-unreviewed; everything after it is a live obligation.
pub(super) const SITTING_DECISION: &str = "d0155";

/// Whether `name` is GOVERNED (in scope) given a grandfather set: in scope iff present and not
/// grandfathered. A `None` set (git unavailable) yields `false` — conservative: the gate never
/// spuriously blocks when charter history can't be read.
pub(super) fn governed(grandfathered: Option<&HashSet<String>>, name: &str) -> bool {
    grandfathered.is_some_and(|gf| !gf.contains(name))
}

/// Names of GOVERNED elements (created after D0080) missing >= 1 required-lens critique — the
/// `guard critique` gap set (charter-time scoped, D0081), sorted.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_gaps(root: &Path) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    let policy = CritiquePolicy::load(root)?;
    let gf = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let stale = compute_stale_verifications(root, &model);
    Ok(compute_critique_coverage(&model, &stale, &policy)
        .into_iter()
        .filter(|c| !c.covered && governed(gf.as_ref(), &c.element))
        .map(|c| c.element)
        .collect())
}

/// Elements rendered SUSPECT by an unresolved failing critique (D0086).
///
/// An element with a `method=critique` Test (`#Verify`-linked to it) whose latest result is `fail`
/// "induces suspicion" — computed from the authored critique, nothing stored; re-clear by appending
/// a passing result to that critique (or a later passing critique). Returns the sorted element set.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_suspect(root: &Path) -> Result<Vec<String>, ViewError> {
    Ok(critique_suspect_set(&Model::build(root)?))
}

/// Pure core of [`critique_suspect`]: the sorted set of elements with an unresolved failing critique.
pub(super) fn critique_suspect_set(model: &Model) -> Vec<String> {
    // D0102: a fail critique whose finding Issue is dispositioned ACCEPT-RISK/DISMISS no longer induces
    // suspicion — the verdict consciously resolved it. The finding->critique link is the typed `#DependsOn`
    // edge from the Issue to the critique Test (so the computation has a typed path, not prose).
    let mut accepted: HashSet<String> = HashSet::new(); // critique Tests whose finding is accept-risk/dismiss
    for (iname, iinfo) in &model.items {
        if iinfo.type_name != "Issue" {
            continue;
        }
        if matches!(issue_disposition(model, iname).as_deref(), Some("acceptRisk" | "dismiss")) {
            for e in &model.edges {
                if e.kind == "dependson" && &e.from == iname {
                    accepted.insert(e.to.clone());
                }
            }
        }
    }
    let mut suspect: HashSet<String> = HashSet::new();
    for e in &model.edges {
        if e.kind != "verify" {
            continue;
        }
        let Some(src) = model.items.get(&e.from) else { continue };
        if src.attrs.get("method").map(String::as_str) != Some("critique") {
            continue;
        }
        if accepted.contains(&e.from) {
            continue; // D0102: this fail critique's finding was accept-risk'd / dismissed
        }
        if matches!(latest_result(model, &e.from), Some((ref o, _)) if o == "fail") {
            suspect.insert(e.to.clone());
        }
    }
    let mut out: Vec<String> = suspect.into_iter().collect();
    out.sort();
    out
}

/// True if `token` occurs in `haystack` as a whole identifier (not a substring of a longer one) — so
/// `sr1` does not match inside `sr15ServeIntrospect`. Used by [`decision_requirement_prose_links`].
pub(super) fn contains_token(haystack: &str, token: &str) -> bool {
    let bytes = haystack.as_bytes();
    haystack.match_indices(token).any(|(i, _)| {
        let before_ok = i == 0 || bytes.get(i - 1).is_none_or(|b| !b.is_ascii_alphanumeric());
        let after_ok = bytes.get(i + token.len()).is_none_or(|b| !b.is_ascii_alphanumeric());
        before_ok && after_ok
    })
}

/// Decisions whose `context` OR `rationale` is blank/trivial (D0103): trimmed length < 20 chars.
///
/// Returns `(total decisions, weak names)`. A recorded decision without a substantive why is ill-formed
/// state — the `decision-rationale` hard guard reads this. (`decision`/`consequences` stay schema-required.)
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn decisions_weak_rationale(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    let decisions: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| i.type_name == "Decision").collect();
    let mut weak: Vec<String> = decisions
        .iter()
        .filter(|(_, info)| {
            let blank = |f: &str| info.attrs.get(f).is_none_or(|v| v.trim().len() < 20);
            blank("context") || blank("rationale")
        })
        .map(|(n, _)| (*n).clone())
        .collect();
    weak.sort();
    Ok((decisions.len(), weak))
}

/// Minimum characters for an attestation to be treated as stating anything (issue083).
///
/// The corpus motivates the number rather than taste: of 234 `method=confirmation` verifications, the
/// non-substantive ones cluster at 0-20 chars (empty, the bare token "accepted", a bare actor name at
/// exactly 20) and the next-shortest genuine attestation is 33 ("schema types aligned with natives").
/// 25 sits in that gap, so the threshold separates the two populations instead of splitting either.
const MIN_ATTESTATION_CHARS: usize = 25;

/// Stock affirmations that assert agreement without recording WHAT was agreed.
const STOCK_AFFIRMATIONS: &[&str] = &[
    "accepted", "approved", "ok", "okay", "yes", "confirmed", "agreed", "signed off", "signoff",
    "lgtm", "done", "acknowledged", "ack", "fine", "good", "proceed", "accept",
];

/// Normalize an attestation for comparison: lowercase, punctuation stripped, whitespace collapsed.
fn normalize_attestation(s: &str) -> String {
    let cleaned: String = s.chars().map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { ' ' }).collect();
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `method=confirmation` verifications with a PASSING result whose attestation says nothing.
///
/// Closes issue083 (D0130). `d0129Accept` was authored with an EMPTY `procedureText` and passed every
/// enforced guard, because `acceptance-events` and `confirmation-authenticity` check that an acceptance
/// EXISTS and is HUMAN-judged — never that it says anything. For `method=confirmation` the attestation
/// text IS the evidence (D0016: test/analysis/inspection carry their own evidence; a confirmation's
/// evidence is the attestation itself), so a contentless acceptance is an unsupported claim wearing the
/// shape of a complete record — on the highest-consequence record type in the engine, since accepted
/// Decisions govern everything downstream. `decision-rationale` (D0103) already applies exactly this
/// substantive-field test to a Decision's *why*; it was never applied to the event that makes the
/// Decision binding.
///
/// Three INDEPENDENT reasons, because length alone is the wrong test — a bare actor name is not
/// evidence at any length, and `d0128Accept` ("william weatherholtz") is exactly 20 characters.
/// Only verifications with a passing result are considered: an unanswered confirmation is a pending
/// human obligation, not a defect.
///
/// Returns `(verification name, reason)`.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn thin_attestations(root: &Path) -> Result<Vec<(String, String)>, ViewError> {
    Ok(thin_attestation_list(&Model::build(root)?))
}

/// Pure core of [`thin_attestations`], for self-test.
pub(super) fn thin_attestation_list(model: &Model) -> Vec<(String, String)> {
    // Actor ids AND display names — a bare attribution restates `judgedBy`, it does not evidence it.
    let mut actor_forms: HashSet<String> = HashSet::new();
    for (name, info) in &model.items {
        if info.type_name == "Person" || info.type_name == "Actor" {
            actor_forms.insert(normalize_attestation(name));
            if let Some(display) = info.attrs.get("name") {
                actor_forms.insert(normalize_attestation(display));
            }
        }
    }

    let mut out: Vec<(String, String)> = Vec::new();
    for (vname, vinfo) in &model.items {
        if vinfo.attrs.get("method").map(String::as_str) != Some("confirmation") {
            continue;
        }
        if latest_result(model, vname).as_ref().map(|(o, _)| o.as_str()) != Some("pass") {
            continue; // unanswered confirmation = pending human obligation, not a defect
        }
        let text = vinfo.attrs.get("procedureText").map_or("", String::as_str).trim();
        let norm = normalize_attestation(text);
        let reason = if text.is_empty() {
            Some("empty — a confirmation's evidence IS its attestation text (D0016)".to_string())
        } else if actor_forms.contains(&norm) {
            Some(format!("only names an actor (\"{text}\") — that restates judgedBy, it does not evidence the attestation"))
        } else if STOCK_AFFIRMATIONS.contains(&norm.as_str()) {
            Some(format!("stock affirmation (\"{text}\") — records agreement without recording WHAT was agreed"))
        } else if text.chars().count() < MIN_ATTESTATION_CHARS {
            Some(format!("too thin to state what was attested ({} chars, minimum {MIN_ATTESTATION_CHARS}): \"{text}\"", text.chars().count()))
        } else {
            None
        };
        if let Some(r) = reason {
            out.push((vname.clone(), r));
        }
    }
    out.sort();
    out
}

/// Governance verbs (D0104): a Decision GOVERNS a requirement (vs merely mentioning it) when one of these
/// sits near the requirement name. Matched as a lowercase substring so inflections count (amended/descoped).
const GOV_VERBS: &[&str] = &["amend", "supersede", "descope", "revise", "cancel", "replace", "retire", "rescope", "moot"];

/// True if `window` cites a FOREIGN decision id (`dNNNN`/`DNNNN` whose digits != `own_digits`).
fn has_foreign_decision_id(window: &str, own_digits: &str) -> bool {
    let b = window.as_bytes();
    (0..b.len()).any(|i| {
        let id_start = matches!(b.get(i), Some(b'd' | b'D'))
            && b.get(i + 1..i + 5).is_some_and(|d| d.iter().all(u8::is_ascii_digit))
            && b.get(i + 5).is_none_or(|c| !c.is_ascii_digit())
            && (i == 0 || b.get(i - 1).is_none_or(|c| !c.is_ascii_alphanumeric()));
        id_start && b.get(i + 1..i + 5).is_some_and(|d| d.iter().map(|&c| c as char).collect::<String>() != own_digits)
    })
}

/// True if `window` (text around a requirement mention) reads as GOVERNANCE by the decision `own_digits`
/// (D0104): a governance verb is present AND no FOREIGN decision id is cited (a foreign id means the verb is
/// attributed to ANOTHER decision — a citation, not this decision's action).
pub(super) fn is_governance_mention(window: &str, own_digits: &str) -> bool {
    let lower = window.to_lowercase();
    GOV_VERBS.iter().any(|v| lower.contains(v)) && !has_foreign_decision_id(window, own_digits)
}

/// Decision→requirement GOVERNANCE references that exist only in PROSE (D0102/D0104, the issue052 class).
///
/// For each accepted Decision, the Needs/SystemRequirements its text GOVERNS (a governance verb near the
/// exact name, no foreign decision id) but to which it carries NO typed edge — a governance link that
/// should be typed, not prose. Contextual mentions/examples are excluded (D0104). A computed `#View`.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn decision_requirement_prose_links(root: &Path) -> Result<Vec<(String, String)>, ViewError> {
    let model = Model::build(root)?;
    let reqs: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Need" || i.type_name == "SystemRequirement").map(|(n, _)| n).collect();
    let mut out: Vec<(String, String)> = Vec::new();
    for (dname, dinfo) in &model.items {
        if dinfo.type_name != "Decision" || dinfo.attrs.get("status").map(String::as_str) != Some("accepted") {
            continue;
        }
        let own_digits: String = dname.chars().filter(char::is_ascii_digit).take(4).collect();
        let text: String = ["context", "decision", "rationale", "consequences"].iter().filter_map(|f| dinfo.attrs.get(*f)).cloned().collect::<Vec<_>>().join(" ");
        for r in &reqs {
            if !contains_token(&text, r) {
                continue;
            }
            if model.edges.iter().any(|e| (&e.from == dname && e.to == **r) || (e.from == **r && &e.to == dname)) {
                continue; // already typed-linked
            }
            // D0104: flag only a GOVERNANCE mention (governance verb near R, no foreign decision id) — not a
            // contextual example or a description of another decision's action.
            let governs = text.match_indices(r.as_str()).any(|(i, _)| {
                let bytes = text.as_bytes();
                let boundary = (i == 0 || bytes.get(i - 1).is_none_or(|b| !b.is_ascii_alphanumeric())) && bytes.get(i + r.len()).is_none_or(|b| !b.is_ascii_alphanumeric());
                if !boundary {
                    return false;
                }
                let mut lo = i.saturating_sub(60);
                let mut hi = (i + r.len() + 60).min(text.len());
                while lo > 0 && !text.is_char_boundary(lo) {
                    lo -= 1;
                }
                while hi < text.len() && !text.is_char_boundary(hi) {
                    hi += 1;
                }
                is_governance_mention(text.get(lo..hi).unwrap_or(&text), &own_digits)
            });
            if governs {
                out.push((dname.clone(), (*r).clone()));
            }
        }
    }
    out.sort();
    Ok(out)
}

/// `SystemRequirement`s a DELIVERED verification names in prose but does not `#Verify`-link to.
///
/// Closes issue082 (D0130): `work done` and `requirement verified` were DISCONNECTED. Sprint 247
/// delivered six SRs with passing `DoD` `TestResult`s and CI green, yet `tier-satisfaction` correctly
/// reported all six UNVERIFIED — because an SR counts as verified only when a Test `#Verify`-links TO
/// IT, and the `DoD` Tests link to the backlog ACTION instead. The model therefore could not tell
/// `requirement not yet delivered` from `requirement delivered but its verification was never traced
/// upward`, and the gap was narrated to the human as though 34% meant functional verification.
///
/// Detection reuses the `decision-requirement-link` shape (D0102): prose names it, no typed edge.
/// A verification counts as DELIVERED when its highest-numbered result passed — so this flags only
/// requirements whose work is actually done, never merely planned ones (those are honest burndown).
///
/// Returns `(verification name, SystemRequirement name)` pairs.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn untraced_verification_links(root: &Path) -> Result<Vec<(String, String)>, ViewError> {
    let model = Model::build(root)?;
    let phases = declared_workflow_phases(root);
    Ok(untraced_links(&model, &phases))
}

/// Pure core of [`untraced_verification_links`], for self-test.
pub(super) fn untraced_links(model: &Model, phases: &[String]) -> Vec<(String, String)> {
    // SRs carrying NO incoming verify edge — the same predicate tier-satisfaction reports as a gap.
    let unverified: Vec<&String> = model
        .items
        .iter()
        .filter(|(n, i)| i.type_name == "SystemRequirement" && !model.edges.iter().any(|e| e.kind == "verify" && &e.to == *n))
        .map(|(n, _)| n)
        .collect();
    if unverified.is_empty() {
        return Vec::new();
    }

    // PHASE gates (refine/standup/implement/review/closeOut/retro) verify the sprint PROCESS, not the
    // requirement, so a gate merely mentioning an SR is not a claim to have discharged it — including
    // them produced 37 of 103 findings, i.e. more than a third pure noise. Derived from the project's
    // DECLARED phases, never a hardcoded list, so it adapts to a downstream's own workflow. DoD tests
    // are deliberately KEPT: a passing DoD *is* the claim that the work is delivered.
    let is_phase_gate = |name: &str| {
        let lower = name.to_ascii_lowercase();
        phases.iter().any(|p| lower.ends_with(&format!("{}gate", p.to_ascii_lowercase())))
    };

    let mut out: Vec<(String, String)> = Vec::new();
    for (vname, vinfo) in &model.items {
        // Keyed on the SHAPE (carries a procedure, has a passing result) rather than a type name, so
        // this cannot silently stop matching if verification typing changes.
        let Some(text) = vinfo.attrs.get("procedureText") else { continue };
        if is_phase_gate(vname) {
            continue;
        }
        if latest_result(model, vname).as_ref().map(|(o, _)| o.as_str()) != Some("pass") {
            continue; // not delivered -> an unverified SR here is honest incompleteness, not a gap
        }
        for sr in &unverified {
            if contains_token(text, sr) && !model.edges.iter().any(|e| &e.from == vname && e.to == **sr) {
                out.push((vname.clone(), (*sr).clone()));
            }
        }
    }
    out.sort();
    out
}

/// How each VERIFIED `SystemRequirement` is verified, as `(method, count)` pairs, plus the verified total.
///
/// Without this, `sr_verified_pct` reads as "requirements with passing tests" when in this repo the
/// verified set is overwhelmingly `method=critique` from the D0080 backfill — the ambiguity that led to
/// the metric being mis-narrated (issue082). Surfacing the mix makes the number self-describing.
pub(super) fn verified_method_mix(model: &Model) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for e in &model.edges {
        if e.kind != "verify" {
            continue;
        }
        if model.items.get(&e.to).is_none_or(|i| i.type_name != "SystemRequirement") {
            continue;
        }
        let method = model
            .items
            .get(&e.from)
            .and_then(|i| i.attrs.get("method"))
            .cloned()
            .unwrap_or_else(|| "unstated".to_string());
        *counts.entry(method).or_default() += 1;
    }
    let mut out: Vec<(String, usize)> = counts.into_iter().collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    out
}

/// Critical-finding targets lacking a non-aiModel critic (D0080/issue031 independence).
///
/// An element verified by a `method=critique` Test with `severity=Critical` whose latest result is
/// `fail` (a Critical finding) MUST also carry a critique by a human/tool critic — aiModel-vs-aiModel
/// shares blind spots, so the highest-stakes findings require cognition-distinct independence. Returns
/// the gap set (vacuous until a Critical finding exists).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
/// The population `critical_independence_gaps` examines, so its guard can report a scan count (issue180).
///
/// The guard used to hardcode `scanned: 0` while finding real violations, printing
/// `0 scanned, 1 violation(s)` - output that contradicts itself, since a violation cannot be found in an
/// empty population. `scanned` is the only signal separating a guard whose population is legitimately
/// empty from one that is mis-aimed and can never fire.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub fn critical_independence_gaps_scanned(root: &Path) -> Result<(usize, Vec<String>), ViewError> {
    let n = Model::build(root)?.items.len();
    Ok((n, critical_independence_gaps(root)?))
}

/// Elements targeted by a Critical-severity finding whose only critiques are `aiModel` — the
/// `critic-independence` guard's finding set.
///
/// # Errors
/// Returns [`ViewError`] if the model cannot be built.
pub fn critical_independence_gaps(root: &Path) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    let mut critical_targets: HashSet<String> = HashSet::new();
    let mut non_ai_covered: HashSet<String> = HashSet::new();
    for e in &model.edges {
        if e.kind != "verify" {
            continue;
        }
        let Some(src) = model.items.get(&e.from) else { continue };
        if src.attrs.get("method").map(String::as_str) != Some("critique") {
            continue;
        }
        if src.attrs.get("severity").map(String::as_str) == Some("Critical") && matches!(latest_result(&model, &e.from), Some((ref o, _)) if o == "fail") {
            critical_targets.insert(e.to.clone());
        }
        if matches!(src.attrs.get("critiquedBy").map(String::as_str), Some("human" | "tool")) {
            non_ai_covered.insert(e.to.clone());
        }
    }
    let mut gaps: Vec<String> = critical_targets.into_iter().filter(|t| !non_ai_covered.contains(t)).collect();
    gaps.sort();
    Ok(gaps)
}

/// Why a critique's `procedureText` reads as low-rigor (D0080/issue030), or `None` if it passes.
pub(super) fn low_rigor_reason(pt: &str) -> Option<&'static str> {
    if pt.chars().count() < 120 {
        return Some("below the 120-char substance floor");
    }
    let up = pt.to_uppercase();
    if up.contains("ATTACK") || up.contains("FINDING") || up.contains("SURVIVED") {
        None
    } else {
        Some("no ATTACK/FINDING/SURVIVED adversarial structure")
    }
}

/// Critique-rigor diagnostics (D0080/issue030): low-rigor critiques + affirming-only critics.
///
/// A critique is low-rigor if its `procedureText` lacks adversarial structure (no ATTACK/FINDING/
/// SURVIVED reasoning) or is below a substance floor (120 chars). A critic (result `judgedBy`) with
/// many critiques and zero findings is flagged as suspiciously affirming. A diagnostic, not a gate.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_rigor(root: &Path) -> Result<Vec<String>, ViewError> {
    let model = Model::build(root)?;
    let mut out = Vec::new();
    let mut tally: HashMap<String, (u32, u32)> = HashMap::new();
    for (name, info) in &model.items {
        if info.attrs.get("method").map(String::as_str) != Some("critique") {
            continue;
        }
        if let Some(why) = low_rigor_reason(info.attrs.get("procedureText").map_or("", String::as_str)) {
            out.push(format!("low-rigor critique '{name}': {why}"));
        }
        if let Some(res) = model.items.get(&format!("{name}R1")) {
            let by = res.attrs.get("judgedBy").cloned().unwrap_or_default();
            let entry = tally.entry(by).or_insert((0, 0));
            entry.0 += 1;
            if res.attrs.get("outcome").map(String::as_str) == Some("fail") {
                entry.1 += 1;
            }
        }
    }
    let mut critics: Vec<(&String, &(u32, u32))> = tally.iter().collect();
    critics.sort();
    for (by, (total, fails)) in critics {
        if *total >= 5 && *fails == 0 {
            out.push(format!("affirming-only critic '{by}': {total} critiques, 0 findings — verify rigor (D0080)"));
        }
    }
    out.sort();
    Ok(out)
}

/// Critique-coverage view (D0080) as JSON.
///
/// Per-element required-lens matrix + per-type summary + the gap set (elements missing a required
/// lens). Honest by construction; never stored.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn critique_coverage(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let policy = CritiquePolicy::load(root)?;
    let stale = compute_stale_verifications(root, &model);
    let cov = compute_critique_coverage(&model, &stale, &policy);
    let gf = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    // SUPERSEDED ELEMENTS ARE OUT OF SCOPE (issue127, dispositioned ACT by wweatherholtz).
    // `compute_coverage` and `compute_tier_satisfaction` both build this same descoped set and filter
    // by it; this view did not, so a RETIRED element stayed in the denominator permanently and its
    // critique had to be maintained to stop the number falling. Three computed views disagreeing
    // about what is in scope is dual truth about scope itself.
    //
    // Landing this moves the numbers, which is why it waited: Decision 70/24 -> 69/24, Need 39/3 ->
    // 34/3, SystemRequirement 70/10 -> 65/5, gaps 137 -> 136. The SR `covered` count halves because
    // the five elements it counted are the serve-discipline originals that sprint 306 superseded —
    // their critiques remain readable and #Verify-linked, but a retired requirement being "covered"
    // was never the question the view is asked.
    let descoped: HashSet<&str> =
        model.edges.iter().filter(|e| e.kind == "supersede").map(|e| e.to.as_str()).collect();
    let in_scope =
        |c: &CritiqueCoverage| governed(gf.as_ref(), &c.element) && !descoped.contains(c.element.as_str());

    // Summary over GOVERNED elements only (the grandfathered ones aren't required); per the policy's
    // DECLARED target types (D0097), so a downstream-added target type is summarized too.
    let mut summary: Vec<Json> = Vec::new();
    for ty in policy.target_types() {
        let rows: Vec<&CritiqueCoverage> = cov.iter().filter(|c| &c.type_name == ty && in_scope(c)).collect();
        if rows.is_empty() {
            continue;
        }
        let covered = i64::try_from(rows.iter().filter(|c| c.covered).count()).unwrap_or(i64::MAX);
        summary.push(Json::Obj(vec![
            ("type".to_string(), Json::s(ty.clone())),
            ("governed".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX))),
            ("covered".to_string(), Json::Int(covered)),
            ("uncovered".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX) - covered)),
        ]));
    }

    let elements: Vec<Json> = cov
        .iter()
        .map(|c| {
            let lenses: Vec<Json> = c
                .lenses
                .iter()
                .map(|l| {
                    Json::Obj(vec![
                        ("lens".to_string(), Json::s(l.lens.clone())),
                        ("critiqued".to_string(), Json::Bool(l.critiqued)),
                        ("critic".to_string(), l.critic.clone().map_or(Json::Null, Json::s)),
                        ("outcome".to_string(), l.outcome.clone().map_or(Json::Null, Json::s)),
                    ])
                })
                .collect();
            Json::Obj(vec![
                ("element".to_string(), Json::s(c.element.clone())),
                ("type".to_string(), Json::s(c.type_name.clone())),
                ("governed".to_string(), Json::Bool(in_scope(c))),
                ("covered".to_string(), Json::Bool(c.covered)),
                ("lenses".to_string(), Json::Arr(lenses)),
            ])
        })
        .collect();

    // The gap set is the GATE's view: governed + uncovered (grandfathered elements never gate).
    let gaps: Vec<Json> = cov.iter().filter(|c| !c.covered && in_scope(c)).map(|c| Json::s(c.element.clone())).collect();

    let out = Json::Obj(vec![
        (
            "critique".to_string(),
            Json::s("critique-coverage: GOVERNED elements (created after D0080, charter-time D0081) of each declared target type (critique-policy.toml, D0097) x required lens -> an independent method=critique verification #Verify-linked to the element"),
        ),
        ("summary".to_string(), Json::Arr(summary)),
        ("gaps".to_string(), Json::Arr(gaps)),
        ("elements".to_string(), Json::Arr(elements)),
    ]);
    Ok(out.dump())
}

/// The ACTIVE critique policy (D0097) as JSON: the source (declared file vs built-in default) + the
/// required lenses per target type. Lets a project confirm an override took effect. Honest, computed.
///
/// # Errors
/// Returns [`ViewError::Toml`]/[`ViewError::Policy`] if the policy file is malformed or names an
/// unknown lens.
pub fn critique_policy(root: &Path) -> Result<String, ViewError> {
    let policy = CritiquePolicy::load(root)?;
    let types: Vec<Json> = policy
        .lenses
        .iter()
        .map(|(ty, lenses)| {
            Json::Obj(vec![
                ("type".to_string(), Json::s(ty.clone())),
                ("lenses".to_string(), Json::Arr(lenses.iter().map(|l| Json::s(l.clone())).collect())),
                ("target".to_string(), Json::Bool(!lenses.is_empty())),
            ])
        })
        .collect();
    let out = Json::Obj(vec![
        (
            "critique_policy".to_string(),
            Json::s("required antagonistic critique lenses per assurance-element type (D0097); a type with >=1 lens is a critique target"),
        ),
        (
            "source".to_string(),
            Json::s(if policy.from_file { ".engine/contracts/critique-policy.toml" } else { "built-in Core-3 default (no policy file)" }),
        ),
        ("canon".to_string(), Json::Arr(CANON_LENSES.iter().map(|l| Json::s(*l)).collect())),
        ("types".to_string(), Json::Arr(types)),
    ]);
    Ok(out.dump())
}

