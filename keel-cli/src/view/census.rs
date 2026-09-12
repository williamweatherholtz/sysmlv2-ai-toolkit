//! `keel show control-census` — the empirical view of every control (dcControlCensusByActorKind, D0426).
//!
//! The human's words, 2026-09-10 (st116): *"any guards that have not been abused that gatekeep me should
//! be reconsidered or dissolved outright"*. That is a question about EVIDENCE, and until this lens the
//! evidence was scattered: which act a control binds sat in the code, whose act it fell on sat in the
//! fire-ledger (D0424), and what motivated it sat in the Issues. This view joins them, per control:
//!
//! - **binds** — whose act the control refuses: `human`, `ai` or `either`. A Claude Code hook fires on
//!   the agent's tool call, so a hook rule binds `ai`; a guard runs on whoever commits, so it binds
//!   `either` and the ledger says who; a write-path check binds the actor whose act the verb records -
//!   `keel accept --by <person>` under delegation is the HUMAN's acceptance, `record result` is the
//!   agent's verdict. The write-path table is declared here because the code is the only place the
//!   subject is knowable; every entry names the Decision that governs it.
//! - **blocks by actor kind** — from the ledger's non-allow lines (`control`, `actorKind`, D0424). An
//!   `allow` line carries no actor kind, so FIRES are per hosting event, never per control.
//! - **incidents** — every Issue whose TITLE names the control (a description cites controls by
//!   contrast: issue447's names three it closes out as NOT friction, and a description match credited
//!   all three with an observed incident), classed `observed`
//!   (`discoveredInField = true`: the Issue reports an act that happened) or `code-read` (found by
//!   inspection or an STPA run). An observed incident that the control's DISSOLVING Decision names is
//!   the control's own friction, not a subversion it caught - D0423 names issue397's eight refusals of
//!   correct acceptances, which are evidence AGAINST the read-back check, not for it. A word-match on
//!   "refused" was tried first and misread issue256 (tool output injected; the fix refuses) as friction,
//!   so the act is read from the Decision, never from the Issue's prose.
//! - **evidence class** — `friction` (its only blocks or incidents are refusals of a human act, and no
//!   subversion was ever observed), `hypothetical` (code-read incidents only, zero blocks), `evidenced`
//!   (an observed subversion, or a block on an act it binds). Friction and hypothetical are listed
//!   first: those are the removal candidates, one Decision each.
//!
//! Two limits are stated in the output rather than hidden. `record issue` writes `discoveredInField =
//! false` unless `--in-field` is passed, so `observed` is a LOWER bound on what happened; and a ledger
//! line from before 2026-09-10 names no control, so a block there is counted `unattributed`, never
//! credited to a control it did not name.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::json::Json;

use super::{Model, ViewError};

/// Whose act a control refuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subject {
    Human,
    Ai,
    Either,
}

impl Subject {
    const fn label(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Ai => "ai",
            Self::Either => "either",
        }
    }
}

/// How an Issue came to be: it reports an act that happened, or it was read out of the code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncidentClass {
    Observed,
    CodeRead,
}

/// What the incident's act was: a subversion the control addresses, or the control's own refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncidentAct {
    Subversion,
    Refusal,
}

/// The census verdict per control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceClass {
    Friction,
    Hypothetical,
    Evidenced,
}

impl EvidenceClass {
    const fn label(self) -> &'static str {
        match self {
            Self::Friction => "friction",
            Self::Hypothetical => "hypothetical",
            Self::Evidenced => "evidenced",
        }
    }
}

/// A control as the census enumerates it, before the ledger and the Issues are joined.
#[derive(Clone, Debug)]
pub struct ControlDecl {
    pub name: String,
    /// `guard`, `control` (a ctlXxx constraint), `hook-rule`, `write-path` or `commit-gate`.
    pub kind: &'static str,
    pub binds: Subject,
    /// Why it binds that subject - the rule, so the reader can dispute it.
    pub binds_rule: String,
    /// `active`, or the Decision that dissolved or narrowed it.
    pub state: String,
    /// Extra strings an Issue may use to name this control.
    pub aliases: Vec<String>,
    /// Issues the control's dissolving Decision (named in `state`) cites: its refusals of a correct act.
    pub refusal_issues: Vec<String>,
}

/// The Issue facts the census reads. `text` is the TITLE: the subject an Issue names, as against the
/// controls its description cites for contrast.
#[derive(Clone, Debug)]
pub struct IssueFacts {
    pub name: String,
    pub text: String,
    pub in_field: bool,
}

#[derive(Clone, Debug)]
pub struct Incident {
    pub issue: String,
    pub class: IncidentClass,
    pub act: IncidentAct,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub decl: ControlDecl,
    pub blocks: BTreeMap<String, u64>,
    pub incidents: Vec<Incident>,
    pub class: EvidenceClass,
    pub basis: String,
}

/// The write-path checks, with the subject each binds. `(name, subject, state, rule, aliases)`.
///
/// Declared in code because the subject is knowable nowhere else: the verb tells whose act is being
/// recorded. A dissolved check stays in the table so its history classes - the human asked which
/// controls gatekept them, and a control that was dissolved for doing so is the answer's exemplar.
const WRITE_PATH_CHECKS: &[(&str, Subject, &str, &str, &[&str])] = &[
    (
        "accept:delegated-words",
        Subject::Human,
        "dissolved to a WARN line by D0423 (2026-09-10)",
        "the read-back of a Decision by the human's quoted words: the act recorded is the HUMAN's acceptance (D0198/D0375)",
        &["read-back check", "delegated-words", "quoted words do not name"],
    ),
    (
        "accept:short-words",
        Subject::Human,
        "dissolved to a WARN line by D0423 (2026-09-10)",
        "quoted words shorter than ten characters: the act recorded is the human's acceptance",
        &["ten characters", "short-words", "shorter than ten"],
    ),
    (
        "accept:gesture-word-typed",
        Subject::Ai,
        "kept by D0427 (proposed): the human never types a gesture citation",
        "a delegated note whose only evidence is a gesture word: the act refused is the agent typing a citation it did not observe (D0411/issue426)",
        &["gesture word", "gesture citation", "gesture-word", "approved at the console"],
    ),
    (
        "accept:no-quote",
        Subject::Ai,
        "active",
        "an unquoted delegated note: the act refused is the agent paraphrasing the human INTO an acceptance (D0198)",
        &["unquoted note", "no-quote", "paraphrasing them into"],
    ),
    (
        "accept:no-delegation",
        Subject::Ai,
        "active",
        "a `--by <human>` from a session with no delegatedRecording grant: the act refused is the agent recording for a person who did not delegate",
        &["no-delegation", "delegatedRecording"],
    ),
    (
        "judge-set:no-quote",
        Subject::Ai,
        "active",
        "an unquoted delegated note on a judged set of proposed results (D0443): the act refused is the agent paraphrasing the human INTO a per-item confirmation (D0198)",
        &["judge-set no-quote", "judge-set:no-quote"],
    ),
    (
        "judge-set:no-delegation",
        Subject::Ai,
        "active",
        "a `--by <human>` judge-set from a session with no confirmationRecord delegatedRecording grant: the act refused is the agent judging proposals for a person who did not delegate (D0443)",
        &["judge-set no-delegation", "judge-set:no-delegation"],
    ),
    (
        "tap:unsigned",
        Subject::Human,
        "dissolved to a WARN line by D0426 (2026-09-10, issue447)",
        "a console or deck tap without a paired device's HMAC: the act is the human's own tap at their own console (D0334/D0335)",
        &["unsigned tap", "unpaired tap", "tap:unsigned", "tap signature"],
    ),
    (
        "append-result:ran-receipt",
        Subject::Ai,
        "active",
        "an AI-judged method=test result with no `// RAN:` receipt: the act refused is the agent's verdict without what produced it (D0232/issue266; refused at the write since issue448)",
        &["// ran:", "ran: receipt", "ran receipt", "recorded what produced it", "append-result receipt"],
    ),
    (
        "append-gate-result:ran-receipt",
        Subject::Ai,
        "active",
        "an AI-judged method=test ceremony gate result with no `// RAN:` receipt: the act refused is the agent's Implement-gate verdict without what produced it (issue448 - the line that landed)",
        &["append-gate-result", "gate result with no receipt", "receiptless gate"],
    ),
    (
        "record:tool-output-prose",
        Subject::Ai,
        "active",
        "prose that reads as captured tool output in a Decision or Issue: the act refused is the agent's shell executing backticks into a record (D0224/issue256)",
        &["captured tool output", "tool output", "backtick"],
    ),
];

/// Hook events whose refusals fall on the agent: a Claude Code hook fires on the agent's tool call or
/// turn boundary; the human's shell never passes through one.
const HOOK_EVENTS: &[&str] = &["pre-bash", "pre-write", "post-edit", "stop", "subagent-stop", "config-change", "user-prompt"];

fn control_kind_for_event(event: &str) -> Option<&'static str> {
    match event {
        "commit-gate-guard" => Some("guard"),
        "commit-gate-validate" | "commit-gate-check-engine" | "commit-gate" => Some("commit-gate"),
        "refused" => Some("write-path"),
        e if HOOK_EVENTS.contains(&e) => Some("hook-rule"),
        _ => None,
    }
}

/// A short guard name ("issues", "actors", "charter") appears in ordinary prose; it names the control
/// only next to the word guard or in backticks. A hyphenated or long name is matched as a substring.
fn names_control(text: &str, name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    if n.contains('-') || n.contains(':') || n.len() >= 8 {
        return text.contains(&n);
    }
    text.contains(&format!("guard `{n}`"))
        || text.contains(&format!("guard: {n}"))
        || text.contains(&format!("guard {n}"))
        || text.contains(&format!("`{n}` guard"))
        || text.contains(&format!("{n} guard"))
        || text.contains(&format!("keel guard {n}"))
}

fn incident_for(decl: &ControlDecl, issue: &IssueFacts) -> Option<Incident> {
    let text = issue.text.to_ascii_lowercase();
    let named = names_control(&text, &decl.name)
        || decl.aliases.iter().any(|a| {
            let a = a.to_ascii_lowercase();
            a.len() >= 6 && text.contains(&a)
        });
    if !named {
        return None;
    }
    let class = if issue.in_field { IncidentClass::Observed } else { IncidentClass::CodeRead };
    let act = if decl.refusal_issues.iter().any(|r| r == &issue.name) { IncidentAct::Refusal } else { IncidentAct::Subversion };
    Some(Incident { issue: issue.name.clone(), class, act })
}

/// The census verdict, with its basis in one sentence. Pure, so the probe pair runs against it.
#[must_use]
pub fn classify(binds: Subject, blocks: &BTreeMap<String, u64>, incidents: &[Incident]) -> (EvidenceClass, String) {
    let on_human = blocks.get("human").copied().unwrap_or(0);
    let on_agent: u64 = blocks.iter().filter(|(k, _)| k.as_str() != "human").map(|(_, v)| v).sum();
    let observed_subversion: Vec<&str> = incidents
        .iter()
        .filter(|i| i.class == IncidentClass::Observed && i.act == IncidentAct::Subversion)
        .map(|i| i.issue.as_str())
        .collect();
    let refusals: Vec<&str> = incidents.iter().filter(|i| i.act == IncidentAct::Refusal).map(|i| i.issue.as_str()).collect();
    if observed_subversion.is_empty() && on_agent == 0 && (on_human > 0 || !refusals.is_empty()) {
        let mut why = Vec::new();
        if on_human > 0 {
            why.push(format!("{on_human} block(s) fell on a human act"));
        }
        if !refusals.is_empty() {
            why.push(format!("refused a correct act in {}", refusals.join(", ")));
        }
        return (EvidenceClass::Friction, format!("{}; no subversion observed, no block on an agent act", why.join("; ")));
    }
    if !observed_subversion.is_empty() {
        return (EvidenceClass::Evidenced, format!("observed incident(s): {}", observed_subversion.join(", ")));
    }
    if on_agent > 0 || on_human > 0 {
        let who = if on_agent > 0 { "an agent act" } else { "a human act" };
        return (EvidenceClass::Evidenced, format!("{} block(s) on {who} - the kind it binds ({})", on_agent + on_human, binds.label()));
    }
    let code_read = incidents.iter().filter(|i| i.class == IncidentClass::CodeRead).count();
    (
        EvidenceClass::Hypothetical,
        if code_read == 0 { "no incident names it and no block is recorded".to_string() } else { format!("{code_read} code-read incident(s), zero blocks") },
    )
}

/// One census: controls x ledger x issues -> rows, friction and hypothetical first.
///
/// Returns the rows and the count of ledger blocks that named no control (pre-D0424 lines).
#[must_use]
pub fn census_rows(mut decls: Vec<ControlDecl>, ledger_text: &str, issues: &[IssueFacts]) -> (Vec<Row>, u64, BTreeMap<String, u64>) {
    let mut blocks: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    let mut event_fires: BTreeMap<String, u64> = BTreeMap::new();
    let mut unattributed = 0u64;
    for line in ledger_text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
        let event = v.get("event").and_then(serde_json::Value::as_str).unwrap_or("?").to_string();
        *event_fires.entry(event.clone()).or_insert(0) += 1;
        let decision = v.get("decision").and_then(serde_json::Value::as_str).unwrap_or("");
        let block = matches!(decision, "block" | "deny" | "refused") || v.get("exit").and_then(serde_json::Value::as_i64).unwrap_or(0) != 0;
        if !block {
            continue;
        }
        let Some(control) = v.get("control").and_then(serde_json::Value::as_str) else {
            unattributed += 1;
            continue;
        };
        let kind = v.get("actorKind").and_then(serde_json::Value::as_str).unwrap_or("unrecorded").to_string();
        // a commit-gate-guard block names every failing guard, comma-joined
        for name in control.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            *blocks.entry(name.to_string()).or_default().entry(kind.clone()).or_insert(0) += 1;
            if !decls.iter().any(|d| d.name == name) {
                let ck = control_kind_for_event(&event).unwrap_or("hook-rule");
                let binds = if ck == "hook-rule" { Subject::Ai } else { Subject::Either };
                decls.push(ControlDecl {
                    name: name.to_string(),
                    kind: ck,
                    binds,
                    binds_rule: format!("named only by the ledger ({event} lines); subject inferred from the event, unclassified in the control map"),
                    state: "active (undeclared in control-map.sysml)".to_string(),
                    aliases: Vec::new(),
                    refusal_issues: Vec::new(),
                });
            }
        }
    }
    let mut rows: Vec<Row> = decls
        .into_iter()
        .map(|decl| {
            let incidents: Vec<Incident> = issues.iter().filter_map(|i| incident_for(&decl, i)).collect();
            let b = blocks.get(&decl.name).cloned().unwrap_or_default();
            let (class, basis) = classify(decl.binds, &b, &incidents);
            Row { decl, blocks: b, incidents, class, basis }
        })
        .collect();
    rows.sort_by(|a, b| a.class.cmp(&b.class).then_with(|| a.decl.name.cmp(&b.decl.name)));
    (rows, unattributed, event_fires)
}

/// The controls this tree declares: the control map's constraints plus the write-path table.
fn declared_controls(model: &Model) -> Vec<ControlDecl> {
    let mut out: Vec<ControlDecl> = Vec::new();
    for (name, info) in &model.items {
        if info.type_name != "SystemSafetyConstraint" {
            continue;
        }
        let title = info.attrs.get("title").cloned().unwrap_or_default();
        if let Some(g) = title.strip_prefix("guard: ") {
            out.push(ControlDecl {
                name: g.trim().to_string(),
                kind: "guard",
                binds: Subject::Either,
                binds_rule: "a guard runs at every commit and turn boundary on whoever wrote the tree; the ledger names whose act each block fell on".to_string(),
                state: "active".to_string(),
                aliases: vec![name.clone()],
                refusal_issues: Vec::new(),
            });
        } else if let Some(c) = title.strip_prefix("control: ") {
            out.push(ControlDecl {
                name: c.trim().to_string(),
                kind: "control",
                binds: Subject::Either,
                binds_rule: "a commit/push/launch control falls on whoever commits, pushes or launches; arming is `keel show controls`".to_string(),
                state: "active".to_string(),
                aliases: vec![name.clone()],
                refusal_issues: Vec::new(),
            });
        } else if let Some(h) = title.strip_prefix("hook-rule: ") {
            // a harness hook fires on the agent's tool call and on nobody else's act (D0426)
            out.push(ControlDecl {
                name: h.trim().to_string(),
                kind: "hook-rule",
                binds: Subject::Ai,
                binds_rule: "a harness hook rule fires on the agent's own tool call; a human never issues one".to_string(),
                state: "active".to_string(),
                aliases: vec![name.clone()],
                refusal_issues: Vec::new(),
            });
        }
    }
    for (name, binds, state, rule, aliases) in WRITE_PATH_CHECKS {
        out.push(ControlDecl {
            name: (*name).to_string(),
            kind: "write-path",
            binds: *binds,
            binds_rule: (*rule).to_string(),
            state: (*state).to_string(),
            aliases: aliases.iter().map(|a| (*a).to_string()).collect(),
            refusal_issues: if state.starts_with("dissolved") { issues_named_by_decisions_in(model, state) } else { Vec::new() },
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out.dedup_by(|a, b| a.name == b.name);
    out
}

/// Every `issueNNN` the Decisions named in `state` (as `D0423` / `d0423`) cite in any field. A
/// dissolving Decision names the refusals it dissolves the control for; that citation is the fact the
/// incident's act is read from.
fn issues_named_by_decisions_in(model: &Model, state: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for token in state.split(|c: char| !c.is_ascii_alphanumeric()) {
        if let Some(digits) = token.strip_prefix("issue") {
            if digits.len() >= 3 && digits.chars().all(|c| c.is_ascii_digit()) {
                out.push(token.to_string());
                continue;
            }
        }
        let lower = token.to_ascii_lowercase();
        if !(lower.len() == 5 && lower.starts_with('d') && lower[1..].chars().all(|c| c.is_ascii_digit())) {
            continue;
        }
        let Some(decision) = model.items.get(&lower) else { continue };
        if decision.type_name != "Decision" {
            continue;
        }
        let text: String = decision.attrs.values().cloned().collect::<Vec<_>>().join(" ");
        let mut rest = text.as_str();
        while let Some(pos) = rest.find("issue") {
            let after = &rest[pos + 5..];
            let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
            if digits.len() >= 3 {
                out.push(format!("issue{digits}"));
            }
            rest = &rest[pos + 5..];
        }
    }
    out.sort();
    out.dedup();
    out
}

fn issue_facts(model: &Model) -> Vec<IssueFacts> {
    let mut v: Vec<IssueFacts> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Issue")
        .map(|(n, i)| IssueFacts {
            name: n.clone(),
            text: i.attrs.get("title").cloned().unwrap_or_default(),
            in_field: i.attrs.get("discoveredInField").is_some_and(|f| f.trim() == "true"),
        })
        .collect();
    v.sort_by(|a, b| a.name.cmp(&b.name));
    v
}

fn row_json(r: &Row) -> Json {
    Json::Obj(vec![
        ("control".to_string(), Json::s(r.decl.name.clone())),
        ("kind".to_string(), Json::s(r.decl.kind.to_string())),
        ("evidenceClass".to_string(), Json::s(r.class.label().to_string())),
        ("basis".to_string(), Json::s(r.basis.clone())),
        ("binds".to_string(), Json::s(r.decl.binds.label().to_string())),
        ("bindsRule".to_string(), Json::s(r.decl.binds_rule.clone())),
        ("state".to_string(), Json::s(r.decl.state.clone())),
        (
            "blocksByActorKind".to_string(),
            Json::Obj(r.blocks.iter().map(|(k, v)| (k.clone(), Json::Int(i64::try_from(*v).unwrap_or(i64::MAX)))).collect()),
        ),
        (
            "incidents".to_string(),
            Json::Arr(
                r.incidents
                    .iter()
                    .map(|i| {
                        Json::Obj(vec![
                            ("issue".to_string(), Json::s(i.issue.clone())),
                            ("class".to_string(), Json::s(match i.class { IncidentClass::Observed => "observed", IncidentClass::CodeRead => "code-read" }.to_string())),
                            ("act".to_string(), Json::s(match i.act { IncidentAct::Subversion => "subversion", IncidentAct::Refusal => "refusal-of-a-correct-act" }.to_string())),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

// ---- the STPA join (dcUcaCarriesItsEvidenceClass, D0424) -------------------------------------------
//
// An UnsafeControlAction's evidence class is computed from the edges, never asserted in its text: it is
// `observed` when a dependency edge from the UCA, or from a ControllerConstraint bound to it, reaches an
// Issue whose incident happened (`discoveredInField = true`), and `code-read` otherwise. A constraint's
// bound control (a control-map part) is looked up in the census rows, so a constraint standing on a
// `friction` or `hypothetical` control is a REMOVAL CANDIDATE the stpa-self run record lists (st115).

/// The evidence class of an `UnsafeControlAction`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UcaEvidence {
    Observed,
    CodeRead,
}

impl UcaEvidence {
    const fn label(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::CodeRead => "code-read",
        }
    }
}

/// What the join reads about one UCA: the constraints bound to it, every Issue reached from it or
/// from those constraints (with the Issue's in-field flag), and the control-map parts the constraints
/// bind.
#[derive(Clone, Debug, Default)]
pub struct UcaFacts {
    pub name: String,
    pub title: String,
    pub constraints: Vec<String>,
    pub issues: Vec<(String, bool)>,
    pub controls: Vec<String>,
}

/// One UCA with its computed class and the census class of every control its constraints bind.
#[derive(Clone, Debug)]
pub struct UcaRow {
    pub facts: UcaFacts,
    pub class: UcaEvidence,
    /// (control-map part, census control name, census class) for each bound control the census knows.
    pub bound: Vec<(String, String, EvidenceClass)>,
    /// bound controls the census does not know - a constraint edged to a part no census row names
    pub unknown: Vec<String>,
}

/// Pure: classify each UCA and join its bound controls to the census rows.
#[must_use]
pub fn uca_rows(ucas: &[UcaFacts], rows: &[Row]) -> Vec<UcaRow> {
    let mut out: Vec<UcaRow> = ucas
        .iter()
        .map(|u| {
            let class = if u.issues.iter().any(|(_, in_field)| *in_field) { UcaEvidence::Observed } else { UcaEvidence::CodeRead };
            let mut bound = Vec::new();
            let mut unknown = Vec::new();
            for part in &u.controls {
                match rows.iter().find(|r| r.decl.name == *part || r.decl.aliases.iter().any(|a| a == part)) {
                    Some(r) => bound.push((part.clone(), r.decl.name.clone(), r.class)),
                    None => unknown.push(part.clone()),
                }
            }
            UcaRow { facts: u.clone(), class, bound, unknown }
        })
        .collect();
    out.sort_by(|a, b| a.facts.name.cmp(&b.facts.name));
    out
}

/// The constraints standing on a control the census classes friction or hypothetical: the run record's
/// REMOVAL CANDIDATE list, one line per (constraint, control).
#[must_use]
pub fn uca_removal_candidates(ucas: &[UcaRow], rows: &[Row]) -> Vec<String> {
    let mut out = Vec::new();
    let mut bound: BTreeSet<&str> = BTreeSet::new();
    for u in ucas {
        for (part, control, class) in &u.bound {
            bound.insert(control.as_str());
            if *class != EvidenceClass::Evidenced {
                out.push(format!("{} -> {part} = {control} ({}) via {}", u.facts.name, class.label(), u.facts.constraints.join(",")));
            }
        }
    }
    // A friction control (its only record is gatekeeping a person, D0426) is a candidate whether or not
    // an analysis ever stood a constraint on it - the human's ask names the control, not the UCA.
    for r in rows.iter().filter(|r| r.class == EvidenceClass::Friction && !bound.contains(r.decl.name.as_str())) {
        out.push(format!("(no constraint) {} (friction; {})", r.decl.name, r.basis));
    }
    out.sort();
    out.dedup();
    out
}

/// The UCAs and their constraint/Issue/control edges as the tree records them.
fn uca_facts(model: &Model) -> Vec<UcaFacts> {
    let in_field = |issue: &str| model.items.get(issue).is_some_and(|i| i.type_name == "Issue" && i.attrs.get("discoveredInField").is_some_and(|f| f.trim() == "true"));
    let is_issue = |n: &str| model.items.get(n).is_some_and(|i| i.type_name == "Issue");
    let is_control = |n: &str| model.items.get(n).is_some_and(|i| i.type_name == "SystemSafetyConstraint");
    let mut out = Vec::new();
    for (name, info) in &model.items {
        if info.type_name != "UnsafeControlAction" {
            continue;
        }
        let mut facts = UcaFacts { name: name.clone(), title: info.attrs.get("title").cloned().unwrap_or_default(), ..UcaFacts::default() };
        for e in model.edges.iter().filter(|e| e.kind == "dependency" && e.from == *name) {
            if is_issue(&e.to) {
                facts.issues.push((e.to.clone(), in_field(&e.to)));
            }
        }
        for cc in model.edges.iter().filter(|e| e.kind == "dependency" && e.to == *name && model.items.get(&e.from).is_some_and(|i| i.type_name == "ControllerConstraint")) {
            facts.constraints.push(cc.from.clone());
            for e in model.edges.iter().filter(|e| e.kind == "dependency" && e.from == cc.from) {
                if is_issue(&e.to) {
                    facts.issues.push((e.to.clone(), in_field(&e.to)));
                } else if is_control(&e.to) {
                    facts.controls.push(e.to.clone());
                }
            }
        }
        facts.constraints.sort();
        facts.issues.sort();
        facts.issues.dedup();
        facts.controls.sort();
        facts.controls.dedup();
        out.push(facts);
    }
    out
}

fn uca_json(u: &UcaRow) -> Json {
    Json::Obj(vec![
        ("uca".to_string(), Json::s(u.facts.name.clone())),
        ("title".to_string(), Json::s(u.facts.title.clone())),
        ("evidenceClass".to_string(), Json::s(u.class.label().to_string())),
        ("constraints".to_string(), Json::Arr(u.facts.constraints.iter().map(|c| Json::s(c.clone())).collect())),
        ("issues".to_string(), Json::Arr(u.facts.issues.iter().map(|(i, f)| Json::Obj(vec![("issue".to_string(), Json::s(i.clone())), ("inField".to_string(), Json::Bool(*f))])).collect())),
        (
            "boundControls".to_string(),
            Json::Arr(
                u.bound
                    .iter()
                    .map(|(part, control, class)| Json::Obj(vec![("part".to_string(), Json::s(part.clone())), ("control".to_string(), Json::s(control.clone())), ("evidenceClass".to_string(), Json::s(class.label().to_string()))]))
                    .collect(),
            ),
        ),
        ("unknownControls".to_string(), Json::Arr(u.unknown.iter().map(|c| Json::s(c.clone())).collect())),
    ])
}

/// `keel show control-census [ROOT]`.
///
/// # Errors
/// Propagates model-build failures.
pub fn control_census(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let decls = declared_controls(&model);
    let issues = issue_facts(&model);
    let ledger = std::fs::read_to_string(root.join(".keel").join("metrics").join("hooks.jsonl")).unwrap_or_default();
    let (rows, unattributed, event_fires) = census_rows(decls, &ledger, &issues);
    let ucas = uca_rows(&uca_facts(&model), &rows);
    let uca_count = |c: UcaEvidence| ucas.iter().filter(|u| u.class == c).count();
    // an evidenced control no ControllerConstraint stands on: the UCA nobody has analysed yet
    let bound_parts: BTreeSet<&str> = ucas.iter().flat_map(|u| u.bound.iter().map(|(_, c, _)| c.as_str())).collect();
    let evidenced_without_uca: Vec<Json> = rows.iter().filter(|r| r.class == EvidenceClass::Evidenced && !bound_parts.contains(r.decl.name.as_str())).map(|r| Json::s(r.decl.name.clone())).collect();
    let count = |c: EvidenceClass| rows.iter().filter(|r| r.class == c).count();
    let observed_total = issues.iter().filter(|i| i.in_field).count();
    let candidates: Vec<Json> = rows
        .iter()
        .filter(|r| r.class != EvidenceClass::Evidenced)
        .map(|r| Json::s(format!("{} ({}, binds {}, {})", r.decl.name, r.class.label(), r.decl.binds.label(), r.decl.state)))
        .collect();
    Ok(Json::Obj(vec![
        ("controlCensus".to_string(), Json::s("every control by WHOSE ACT IT BINDS, the ledger's blocks by actor kind (D0424), the Issues that motivated it (observed vs code-read) and its evidence class (D0426): friction and hypothetical first - those are the removal candidates, one Decision each. `observed` := discoveredInField, which `record issue` writes false unless --in-field is passed, so it is a LOWER bound; a pre-2026-09-10 ledger block names no control and is counted unattributed; an incident is matched by the control's name or declared aliases in the Issue's TITLE (a description cites controls by contrast), and an incident the control's dissolving Decision names is the control's own refusal of a correct act, not a subversion it caught.".to_string())),
        ("summary".to_string(), Json::Obj(vec![
            ("controls".to_string(), Json::Int(i64::try_from(rows.len()).unwrap_or(i64::MAX))),
            ("friction".to_string(), Json::Int(i64::try_from(count(EvidenceClass::Friction)).unwrap_or(i64::MAX))),
            ("hypothetical".to_string(), Json::Int(i64::try_from(count(EvidenceClass::Hypothetical)).unwrap_or(i64::MAX))),
            ("evidenced".to_string(), Json::Int(i64::try_from(count(EvidenceClass::Evidenced)).unwrap_or(i64::MAX))),
            ("unattributedBlocks".to_string(), Json::Int(i64::try_from(unattributed).unwrap_or(i64::MAX))),
            ("issuesInField".to_string(), Json::Int(i64::try_from(observed_total).unwrap_or(i64::MAX))),
            ("issues".to_string(), Json::Int(i64::try_from(issues.len()).unwrap_or(i64::MAX))),
        ])),
        ("removalCandidates".to_string(), Json::Arr(candidates)),
        ("controls".to_string(), Json::Arr(rows.iter().map(row_json).collect())),
        ("ucaSummary".to_string(), Json::Obj(vec![
            ("note".to_string(), Json::s("every UnsafeControlAction's evidence class COMPUTED from its edges (dcUcaCarriesItsEvidenceClass, D0424): observed when a dependency edge from the UCA or from a ControllerConstraint bound to it reaches an Issue with discoveredInField = true, code-read otherwise; a constraint standing on a friction or hypothetical control is a removal candidate the stpa-self run record lists (st115); an evidenced control no constraint stands on is the UCA not yet analysed".to_string())),
            ("ucas".to_string(), Json::Int(i64::try_from(ucas.len()).unwrap_or(i64::MAX))),
            ("observed".to_string(), Json::Int(i64::try_from(uca_count(UcaEvidence::Observed)).unwrap_or(i64::MAX))),
            ("codeRead".to_string(), Json::Int(i64::try_from(uca_count(UcaEvidence::CodeRead)).unwrap_or(i64::MAX))),
            ("removalCandidates".to_string(), Json::Arr(uca_removal_candidates(&ucas, &rows).into_iter().map(Json::s).collect())),
            ("evidencedWithoutUca".to_string(), Json::Arr(evidenced_without_uca)),
        ])),
        ("ucas".to_string(), Json::Arr(ucas.iter().map(uca_json).collect())),
        ("eventFires".to_string(), Json::Obj(event_fires.iter().map(|(k, v)| (k.clone(), Json::Int(i64::try_from(*v).unwrap_or(i64::MAX)))).collect())),
    ])
    .dump())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uca(name: &str, issues: &[(&str, bool)], controls: &[&str]) -> UcaFacts {
        UcaFacts {
            name: name.into(),
            title: String::new(),
            constraints: vec![format!("cc{name}")],
            issues: issues.iter().map(|(i, f)| ((*i).to_string(), *f)).collect(),
            controls: controls.iter().map(|c| (*c).to_string()).collect(),
        }
    }

    /// Probe pair (D0388): a UCA whose constraint reaches an in-field Issue classes observed; one that
    /// reaches no Issue classes code-read - and a code-read Issue does not make it observed.
    #[test]
    fn a_uca_reaching_an_in_field_issue_is_observed_and_one_reaching_none_is_code_read() {
        let rows = uca_rows(&[uca("ucaA", &[("issue9", true)], &[]), uca("ucaB", &[], &[]), uca("ucaC", &[("issue8", false)], &[])], &[]);
        let class = |n: &str| rows.iter().find(|u| u.facts.name == n).map(|u| u.class).expect("row");
        assert_eq!(class("ucaA"), UcaEvidence::Observed);
        assert_eq!(class("ucaB"), UcaEvidence::CodeRead);
        assert_eq!(class("ucaC"), UcaEvidence::CodeRead);
    }

    /// A constraint standing on a hypothetical control is a removal candidate; one on an evidenced
    /// control is not; a control the census does not know is reported unknown, never silently dropped.
    #[test]
    fn a_constraint_on_a_hypothetical_control_is_a_removal_candidate() {
        let issues = [
            IssueFacts { name: "issue1".into(), text: "the acceptance-binds-to-text guard could miss a rebinding".into(), in_field: false },
            IssueFacts { name: "issue2".into(), text: "a script stamped a verdict past the ceremony guard".into(), in_field: true },
        ];
        let mut hyp = guard("acceptance-binds-to-text");
        hyp.aliases.push("gAcceptanceBindsToText".into());
        let mut evd = guard("ceremony");
        evd.aliases.push("gCeremony".into());
        let (rows, _, _) = census_rows(vec![hyp, evd], "", &issues);
        let ucas = uca_rows(&[uca("ucaX", &[], &["gAcceptanceBindsToText"]), uca("ucaY", &[], &["gCeremony", "gNobody"])], &rows);
        let cands = uca_removal_candidates(&ucas, &rows);
        assert_eq!(cands.len(), 1, "{cands:?}");
        assert!(cands[0].starts_with("ucaX -> gAcceptanceBindsToText = acceptance-binds-to-text (hypothetical)"), "{}", cands[0]);
        let y = ucas.iter().find(|u| u.facts.name == "ucaY").expect("ucaY");
        assert_eq!(y.unknown, vec!["gNobody".to_string()]);
        assert_eq!(y.bound.len(), 1);
    }

    fn guard(name: &str) -> ControlDecl {
        ControlDecl { name: name.to_string(), kind: "guard", binds: Subject::Either, binds_rule: String::new(), state: "active".to_string(), aliases: Vec::new(), refusal_issues: Vec::new() }
    }

    /// Probe pair (D0388): a friction control no constraint binds IS a candidate (the human's ask names
    /// the control); an evidenced control no constraint binds is NOT.
    #[test]
    fn a_friction_control_no_constraint_binds_is_still_a_removal_candidate() {
        let issues = [IssueFacts { name: "issue9".into(), text: "a script stamped a verdict past the ceremony guard".into(), in_field: true }];
        let mut fric = guard("accept:delegated-words");
        fric.binds = Subject::Human;
        let mut evd = guard("ceremony");
        evd.aliases.push("gCeremony".into());
        let ledger = r#"{"event":"refused","decision":"block","control":"accept:delegated-words","actorKind":"human"}"#;
        let (rows, _, _) = census_rows(vec![fric, evd], ledger, &issues);
        let cands = uca_removal_candidates(&uca_rows(&[], &rows), &rows);
        assert_eq!(cands.len(), 1, "{cands:?}");
        assert!(cands[0].starts_with("(no constraint) accept:delegated-words (friction;"), "{}", cands[0]);
    }

    /// Probe pair, chosen before any tree was read (D0388): a guard whose only Issue is code-read and
    /// zero blocks classes hypothetical; the same guard with an observed subversion classes evidenced.
    #[test]
    fn code_read_only_and_zero_blocks_is_hypothetical() {
        let issues = [IssueFacts { name: "issue1".into(), text: "the acceptance-binds-to-text guard could miss a rebinding".into(), in_field: false }];
        let (rows, unattributed, _) = census_rows(vec![guard("acceptance-binds-to-text")], "", &issues);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].class, EvidenceClass::Hypothetical, "{}", rows[0].basis);
        assert_eq!(rows[0].incidents.len(), 1);
        assert_eq!(rows[0].incidents[0].class, IncidentClass::CodeRead);
        assert_eq!(unattributed, 0);
    }

    #[test]
    fn an_observed_subversion_is_evidenced() {
        let issues = [IssueFacts { name: "issue2".into(), text: "a script stamped a verdict past the acceptance-binds-to-text guard".into(), in_field: true }];
        let (rows, _, _) = census_rows(vec![guard("acceptance-binds-to-text")], "", &issues);
        assert_eq!(rows[0].class, EvidenceClass::Evidenced, "{}", rows[0].basis);
        assert!(rows[0].basis.contains("issue2"));
    }

    /// The friction case as the `DoD` names it: eight refusals of a human's correct act, no subversion.
    #[test]
    fn refusals_of_a_human_act_with_no_subversion_is_friction() {
        let decl = ControlDecl {
            name: "accept:delegated-words".into(),
            kind: "write-path",
            binds: Subject::Human,
            binds_rule: String::new(),
            state: "dissolved".into(),
            aliases: vec!["read-back check".into()],
            refusal_issues: vec!["issue397".into()],
        };
        let issues = [IssueFacts { name: "issue397".into(), text: "the read-back check split on the apostrophe, refusing eight correct acceptances".into(), in_field: true }];
        let ledger = r#"{"event":"refused","decision":"refused","control":"accept:delegated-words","actorKind":"human"}"#;
        let (rows, _, _) = census_rows(vec![decl], ledger, &issues);
        assert_eq!(rows[0].class, EvidenceClass::Friction, "{}", rows[0].basis);
        assert_eq!(rows[0].incidents[0].act, IncidentAct::Refusal);
        assert!(rows[0].basis.contains("issue397"), "{}", rows[0].basis);
        assert_eq!(rows[0].blocks.get("human"), Some(&1));
    }

    /// A block on an agent act evidences the control even with no Issue; a ledger block that names
    /// no control is unattributed, and a guard the ledger names but the map does not is still a row.
    #[test]
    fn ledger_blocks_evidence_and_unattributed_lines_are_counted_apart() {
        let ledger = [
            r#"{"event":"commit-gate-guard","decision":"block","control":"issues, ceremony","actorKind":"ai"}"#,
            r#"{"event":"stop","decision":"block","exit":2}"#,
            r#"{"event":"pre-bash","decision":"deny","control":"heredoc-backslash","actorKind":"ai"}"#,
        ]
        .join("\n");
        let (rows, unattributed, fires) = census_rows(vec![guard("issues")], &ledger, &[]);
        assert_eq!(unattributed, 1);
        assert_eq!(fires.get("stop"), Some(&1));
        let by = |n: &str| rows.iter().find(|r| r.decl.name == n).expect(n);
        assert_eq!(by("issues").class, EvidenceClass::Evidenced);
        assert_eq!(by("ceremony").decl.kind, "guard");
        assert_eq!(by("heredoc-backslash").decl.binds, Subject::Ai);
        assert_eq!(rows[0].class, EvidenceClass::Evidenced, "every row here is evidenced; ordering is by name then");
    }

    #[test]
    fn a_short_guard_name_needs_the_word_guard_beside_it() {
        assert!(!names_control("there were issues with the build", "issues"));
        assert!(names_control("the issues guard failed", "issues"));
        assert!(names_control("guard `issues` refused", "issues"));
        assert!(names_control("marker-vocabulary read it as undeclared", "marker-vocabulary"));
    }
}
