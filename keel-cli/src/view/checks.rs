//! the generic evaluator over DECLARED rules (D0105) - `keel check` - extracted from view.rs (sprint 418, dcViewRsRestructure: the panel's
//! god-module finding). Pure move, no behavior change; `view::` paths survive via the
//! `pub use` re-exports in mod.rs.


use std::path::Path;

use crate::json::Json;


#[allow(clippy::wildcard_imports)] // a pure move-only split: the parent's vocabulary IS this file's vocabulary
use super::*;

// ── `keel check` (D0105 EXPAND step 2): the generic evaluator over DECLARED rules ────────────────

/// Does `info` match an `EdgeRule` `subjectType` — a `#Marker` (marker match) or a bare type name?
fn rc_matches_subject(info: &ItemInfo, subject: &str) -> bool {
    subject.strip_prefix('#').map_or_else(
        || info.type_name == subject,
        |marker| info.marker.as_deref().is_some_and(|m| m.trim_start_matches('#').eq_ignore_ascii_case(marker)),
    )
}

/// `EdgeRule` violations: `subject` instances lacking `edge` (at `cardinality`) to an existing instance
/// of `object` (`"*"` = any target). Sorted subject names. The generic core that subsumes the ~9
/// conformance guards once each rule reaches parity.
/// Repo-relative, forward-slashed files git reports as newly-ADDED in the staged index — the `newlyAdded`
/// scope set (matches the charter/sprint-coverage guards' forward-only semantics). Empty if git fails.
fn staged_added_files(root: &Path) -> std::collections::HashSet<String> {
    crate::gitx::git()
        .arg("-C").arg(root)
        .args(["diff", "--cached", "--name-only", "--diff-filter=A"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().map(|l| l.trim().replace('\\', "/")).collect())
        .unwrap_or_default()
}

pub(super) fn edge_rule_violations(model: &Model, subject: &str, edge: &str, object: &str, direction: &str, cardinality: &str, scope_files: Option<&std::collections::HashSet<String>>) -> Vec<String> {
    let incoming = direction == "incoming";
    let mut out: Vec<String> = Vec::new();
    for (name, info) in &model.items {
        if !rc_matches_subject(info, subject) {
            continue;
        }
        // `newlyAdded` scope: only subjects whose source file is in the staged-added set.
        if scope_files.is_some_and(|files| !files.contains(&info.file)) {
            continue;
        }
        let count = model
            .edges
            .iter()
            .filter(|e| {
                let (near, far) = if incoming { (&e.to, &e.from) } else { (&e.from, &e.to) };
                e.kind == edge
                    && near == name
                    && (object == "*" || model.items.get(far).is_some_and(|t| t.type_name == object))
            })
            .count();
        let ok = if cardinality == "exactlyOne" { count == 1 } else { count >= 1 };
        if !ok {
            out.push(name.clone());
        }
    }
    out.sort();
    out
}

/// `strip_prefix(p)` then `strip_suffix(')')` — pulls `args` out of a `fn(args)` predicate term.
fn predicate_args<'a>(t: &'a str, p: &str) -> Option<&'a str> {
    t.strip_prefix(p).and_then(|r| r.strip_suffix(')'))
}

/// Is `name` within a rule's `appliesWhen` SCOPE? `all` (always), `whereStatus(v)` (the item's `status`
/// attr == v), `whereKind(v)` (the item's `kind` `WorkKind` == v). `Some(bool)`, or `None` if the scope
/// predicate is unsupported (caller marks the rule not-evaluated). Git-temporal scopes (`newlyAdded`)
/// are a later sub-step, reported unsupported here.
fn subject_in_scope(info: &ItemInfo, scope: &str) -> Option<bool> {
    let scope = scope.trim();
    if scope == "all" {
        return Some(true);
    }
    if let Some(v) = predicate_args(scope, "whereStatus(") {
        return Some(info.attrs.get("status").map(String::as_str) == Some(v.trim()));
    }
    // whereKind(v): the item's WorkKind == v (e.g. research) — scopes a rule to a work-kind (issue055
    // researchSpikeCharterRule: only WorkKind::research Stories). The parser stores the enum MEMBER,
    // so `WorkKind::research` reads back as "research".
    if let Some(v) = predicate_args(scope, "whereKind(") {
        return Some(info.attrs.get("kind").map(String::as_str) == Some(v.trim()));
    }
    None
}

/// Evaluate one `ElementRule` predicate TERM for item `name`. Closed vocabulary so far: `nonBlank(field)`
/// (trimmed len > 0), `minLength(field,n)` (trimmed len >= n), `hasPassingResult(suffix)` (a sibling
/// item `<name><suffix>R1` exists with `outcome=pass` — the acceptance/DoD naming convention),
/// `resultJudgedByHuman(suffix)`, the `[not]matchesPattern[CI]` substring family, and
/// `charterTargetType(T1,...)` (every outgoing `#CharteredBy` edge targets an allow-listed type).
/// Unknown term returns `None` (the caller reports the rule `evaluated=false` rather than silently passing).
fn eval_predicate_term(model: &Model, name: &str, term: &str) -> Option<bool> {
    let term = term.trim();
    let attrs = &model.items.get(name)?.attrs;
    if let Some(args) = predicate_args(term, "nonBlank(") {
        return Some(attrs.get(args.trim()).is_some_and(|v| !v.trim().is_empty()));
    }
    if let Some(args) = predicate_args(term, "minLength(") {
        let mut parts = args.split(',');
        let field = parts.next()?.trim();
        let n: usize = parts.next()?.trim().parse().ok()?;
        return Some(attrs.get(field).is_some_and(|v| v.trim().chars().count() >= n));
    }
    if let Some(suffix) = predicate_args(term, "hasPassingResult(") {
        let ev = format!("{name}{}R1", suffix.trim());
        return Some(model.items.get(&ev).and_then(|i| i.attrs.get("outcome")).map(String::as_str) == Some("pass"));
    }
    // resultJudgedByHuman(suffix): the sibling result <name><suffix>R1 was judged by a HUMAN actor — its
    // judgedBy names a `Person`-typed item (D0106 confirmation-authenticity: sign-off is never AI-fabricated).
    if let Some(suffix) = predicate_args(term, "resultJudgedByHuman(") {
        let ev = format!("{name}{}R1", suffix.trim());
        let judged_by = model.items.get(&ev).and_then(|i| i.attrs.get("judgedBy"));
        return Some(judged_by.and_then(|jb| model.items.get(jb)).is_some_and(|a| a.type_name == "Person"));
    }
    // acceptQuotesDelegatedWords(cutoff): D0192 OPTION A substance check. The sibling acceptance event
    // `<name>AcceptR1`, when judged on/after the cutoff date, must evidence its channel: the Test's
    // procedureText quotes the human's conversational words (a single-quoted span of >= 10 chars) or
    // cites a human surface gesture (deck/console). Earlier events are grandfathered (issue068
    // forward-only). A MISSING event is acceptance-events' violation, not this rule's — vacuously true.
    // STATED LIMIT (D0192): a fabricated quote defeats this; the protections behind it are the human
    // reading their own queue and the audit trail.
    if let Some(cutoff) = predicate_args(term, "acceptQuotesDelegatedWords(") {
        let Some(r1) = model.items.get(&format!("{name}AcceptR1")) else {
            return Some(true);
        };
        let Some(judged_at) = r1.attrs.get("judgedAt") else {
            return Some(true);
        };
        if judged_at.as_str() < cutoff.trim() {
            return Some(true);
        }
        // issue287: a record the HUMAN made themselves is not delegated - its recorder (`createdBy`,
        // written by every accept path since D0299) is the judge - and demanding they quote themselves
        // points governance at the one party it must never bind. A record with NO recorder stamped
        // (every acceptance before D0299) is read as delegated: absence of provenance is not evidence
        // of self-recording, and those records already carry their quote or gesture.
        let self_recorded = r1.attrs.get("createdBy").is_some_and(|rec| r1.attrs.get("judgedBy") == Some(rec));
        if self_recorded {
            return Some(true);
        }
        let text = model
            .items
            .get(&format!("{name}Accept"))
            .and_then(|i| i.attrs.get("procedureText"))
            .map_or("", String::as_str);
        return Some(quotes_conversational_words(text));
    }
    // confirmationQuotesOrAttested(cutoff): D0198 OPTION A (quote receipts). A method=confirmation
    // Test whose LATEST result is a human-judged pass on/after the cutoff must carry its evidence:
    // its own procedureText quotes the human's words / cites their gesture, OR a companion record
    // `<name>Attest<N>` (method=confirmation, quoting text, passing result) exists. Acceptance
    // events (`*Accept`) keep delegatedAcceptanceSubstanceRule; companion records themselves
    // (`*Attest<N>`) are evidence, not subjects. Pre-cutoff and AI-judged results: vacuously true
    // (issue068 forward-only; an AI-judged confirmation is confirmationAuthenticityRule's business).
    if let Some(cutoff) = predicate_args(term, "confirmationQuotesOrAttested(") {
        if attrs.get("method").map(String::as_str) != Some("confirmation")
            || name.ends_with("Accept")
            || is_attest_companion(name)
        {
            return Some(true);
        }
        let Some((outcome, judged_at, judged_by)) = latest_result_full(model, name) else {
            return Some(true); // unanswered confirmation = pending obligation, not a defect
        };
        let human = model.items.get(&judged_by).is_some_and(|a| a.type_name == "Person");
        if outcome != "pass" || !human || judged_at.as_str() < cutoff.trim() {
            return Some(true);
        }
        let own_text = attrs.get("procedureText").map_or("", String::as_str);
        if quotes_conversational_words(own_text) {
            return Some(true);
        }
        let receipt = model.items.iter().any(|(cn, ci)| {
            cn.starts_with(name)
                && is_attest_companion(cn)
                && ci.attrs.get("method").map(String::as_str) == Some("confirmation")
                && quotes_conversational_words(ci.attrs.get("procedureText").map_or("", String::as_str))
                && latest_result(model, cn).is_some_and(|(o, _)| o == "pass")
        });
        return Some(receipt);
    }
    // charterTargetType(T1,T2,...): every OUTGOING #CharteredBy edge from `name` targets an item whose
    // TYPE is in the allow-list. The enforceable slice of research-spike routing (issue055): once a
    // spike EXISTS, its charter must point at a real Issue or Decision, so the routing convention gains a
    // control on the structurally-visible side (the "did analysis skip the spike?" judgment stays
    // reminder-enforced — a commit gate cannot see a conversation). Vacuously true when `name` has no
    // charter edge — edge EXISTENCE is charterRule's job (D0068), not this rule's.
    if let Some(args) = predicate_args(term, "charterTargetType(") {
        let allow: Vec<&str> = args.split(',').map(str::trim).collect();
        let ok = model
            .edges
            .iter()
            .filter(|e| e.kind == "charteredby" && e.from == name)
            .all(|e| model.items.get(&e.to).is_some_and(|t| allow.contains(&t.type_name.as_str())));
        return Some(ok);
    }
    // matchesPattern(field, needle) / notMatchesPattern(field, needle): case-sensitive substring on an attr.
    // The `CI` variants are case-insensitive. `needle` may contain spaces (after the first comma); no ')'.
    for (prefix, want, ci) in [
        ("matchesPatternCI(", true, true),
        ("notMatchesPatternCI(", false, true),
        ("matchesPattern(", true, false),
        ("notMatchesPattern(", false, false),
    ] {
        if let Some(args) = predicate_args(term, prefix) {
            let (field, needle) = args.split_once(',')?;
            let hit = attrs.get(field.trim()).is_some_and(|v| {
                if ci {
                    v.to_lowercase().contains(&needle.trim().to_lowercase())
                } else {
                    v.contains(needle.trim())
                }
            });
            return Some(hit == want);
        }
    }
    None
}

/// Is `name` a quote-receipt companion — `<test>Attest<N>` (D0198 OPTION A naming convention)?
fn is_attest_companion(name: &str) -> bool {
    name.rfind("Attest").is_some_and(|i| {
        i > 0 && !name[i + 6..].is_empty() && name[i + 6..].chars().all(|c| c.is_ascii_digit())
    })
}

/// `(outcome, judgedAt, judgedBy)` of the HIGHEST-numbered `<v>R<n>` result — the fields the
/// D0198 quote-receipt predicate needs beside [`latest_result`]'s `(outcome, judgedAgainst)`.
fn latest_result_full(model: &Model, v: &str) -> Option<(String, String, String)> {
    let mut best: Option<(u32, String, String, String)> = None;
    for (name, info) in &model.items {
        let Some(suf) = name.strip_prefix(v) else { continue };
        let Some(digits) = suf.strip_prefix('R') else { continue };
        if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let Ok(n) = digits.parse::<u32>() else { continue };
        if best.as_ref().is_none_or(|(bn, _, _, _)| n > *bn) {
            best = Some((
                n,
                info.attrs.get("outcome").cloned().unwrap_or_default(),
                info.attrs.get("judgedAt").cloned().unwrap_or_default(),
                info.attrs.get("judgedBy").cloned().unwrap_or_default(),
            ));
        }
    }
    best.map(|(_, o, at, by)| (o, at, by))
}

/// Does an acceptance record carry its channel evidence (D0192 OPTION A)? True on a single-quoted
/// span of at least 10 characters (the human's verbatim conversational words) or a named human
/// surface gesture (deck/console).
pub(super) fn quotes_conversational_words(text: &str) -> bool {
    let lower = text.to_lowercase();
    // Named human-surface gestures that ARE the channel evidence: the localhost deck/console, and a
    // GitHub comment citation (issue235). A GitHub comment is 2FA-authenticated, server-timestamped,
    // and immutably event-logged — the D0205/D0201-B conclusion that it subsumes the device HMAC — so
    // citing one is a STRONGER gesture than the quoted words, and a one-letter fork answer ("A") is a
    // deliberate authenticated choice even though it is no 10-char span.
    if lower.contains("deck") || lower.contains("console") || lower.contains("github comment") || lower.contains("github.com/") {
        return true;
    }
    // A quote span closes at an apostrophe NOT followed by a letter — otherwise every contraction
    // ("let's", "doesn't") truncates the span and an honest verbatim quote fails the check (found
    // live: the D0205 acceptance quoting 'yep let's go' scanned as 7 chars). An apostrophe with a
    // letter right after is part of the words, not the closing quote.
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes.get(i) == Some(&b'\'') {
            let mut j = i + 1;
            while j < bytes.len() {
                if bytes.get(j) == Some(&b'\'') && !bytes.get(j + 1).is_some_and(u8::is_ascii_alphabetic) {
                    break;
                }
                j += 1;
            }
            if j < bytes.len() && text.get(i + 1..j).is_some_and(|s| s.chars().count() >= 10) {
                return true;
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    false
}

/// Evaluate a full `ElementRule` `predicate` (TERMs joined by ` and `) for item `name`. Returns `None`
/// if ANY term is unsupported (so the rule reports `evaluated=false`, never a false pass). Conjunction.
fn eval_predicate(model: &Model, name: &str, predicate: &str) -> Option<bool> {
    let mut all = true;
    for term in predicate.split(" and ") {
        all &= eval_predicate_term(model, name, term)?;
    }
    Some(all)
}

/// `ElementRule` violations: `scope`d `subject` instances whose `predicate` is false. `Some(sorted
/// names)`, or `None` if the scope or predicate uses an unsupported term (caller marks the rule
/// not-evaluated). Subsumes the ~5 structural guards as each predicate becomes expressible.
pub(super) fn element_rule_violations(model: &Model, subject: &str, predicate: &str, scope: &str) -> Option<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    for (name, info) in &model.items {
        if !rc_matches_subject(info, subject) {
            continue;
        }
        if !subject_in_scope(info, scope)? {
            continue;
        }
        if !eval_predicate(model, name, predicate)? {
            out.push(name.clone());
        }
    }
    out.sort();
    Some(out)
}

/// Business-layer view (serveBusinessNeedsView): the Brief, Personas, Needs and use cases.
///
/// The "what/why" layer the `keel serve` console lacked. A computed `#View`; each Need carries a
/// `decomposed` flag (some `SystemRequirement` `satisfy`-links it) so the human sees the trace frontier.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn business(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let by_type = |ty: &str| {
        let mut v: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| i.type_name == ty).collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    };
    let field = |i: &ItemInfo, k: &str| Json::s(i.attrs.get(k).cloned().unwrap_or_default());
    let briefs = by_type("Brief").into_iter().map(|(n, i)| Json::Obj(vec![
        ("name".to_string(), Json::s(n.clone())),
        ("title".to_string(), field(i, "title")),
        ("problem".to_string(), field(i, "problem")),
        ("opportunity".to_string(), field(i, "opportunity")),
        ("constraintsNote".to_string(), field(i, "constraintsNote")),
    ])).collect();
    let personas = by_type("Persona").into_iter().map(|(n, i)| Json::Obj(vec![
        ("name".to_string(), Json::s(n.clone())),
        ("title".to_string(), field(i, "title")),
        ("description".to_string(), field(i, "description")),
        ("goals".to_string(), field(i, "goals")),
    ])).collect();
    let needs = by_type("Need").into_iter().map(|(n, i)| {
        let decomposed = model.edges.iter().any(|e| e.kind == "satisfy" && &e.from == n);
        Json::Obj(vec![
            ("name".to_string(), Json::s(n.clone())),
            ("title".to_string(), field(i, "title")),
            ("statement".to_string(), field(i, "statement")),
            ("priority".to_string(), field(i, "priority")),
            ("source".to_string(), field(i, "source")),
            ("decomposed".to_string(), Json::Bool(decomposed)),
        ])
    }).collect();
    let use_cases = by_type("UseCase").into_iter().map(|(n, i)| Json::Obj(vec![
        ("name".to_string(), Json::s(n.clone())),
        ("title".to_string(), field(i, "title")),
    ])).collect();
    Ok(Json::Obj(vec![
        ("business".to_string(), Json::s("Business layer (Brief -> Personas -> Needs -> UseCases) — the what/why (D0105 serveBusinessNeedsView)")),
        ("briefs".to_string(), Json::Arr(briefs)),
        ("personas".to_string(), Json::Arr(personas)),
        ("needs".to_string(), Json::Arr(needs)),
        ("useCases".to_string(), Json::Arr(use_cases)),
    ]).dump())
}

/// Launchable-set view (srServeModelDrivenRegistry, Tier 1a): the processes + skills keel serve may launch.
///
/// Computed from the DECLARED model (no separately-authored list — nServeReuseModel). Each entry carries
/// its name/title/kind. A computed `#View`; finer per-launchable output schemas are a later increment.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn launchables(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let of_kind = |ty: &str| -> Vec<Json> {
        let mut v: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(_, i)| i.type_name == ty).collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v.into_iter()
            .map(|(n, i)| Json::Obj(vec![
                ("name".to_string(), Json::s(n.clone())),
                ("title".to_string(), Json::s(i.attrs.get("title").cloned().unwrap_or_default())),
                ("kind".to_string(), Json::s(ty)),
            ]))
            .collect()
    };
    let skills = of_kind("AISkill");
    let processes = of_kind("Process");
    let total = skills.len() + processes.len();
    Ok(Json::Obj(vec![
        ("launchables".to_string(), Json::s("keel serve launchable set — declared skills + processes (srServeModelDrivenRegistry, D0109). Only these may be launched; no freeform path.")),
        ("skills".to_string(), Json::Arr(skills)),
        ("processes".to_string(), Json::Arr(processes)),
        ("total".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
    ])
    .dump())
}

/// Whether `target` is a declared launchable (a `Process` or `AISkill` in the model) — the guardrail
/// behind srServeLauncherDefinedOnly (no freeform launch). Tier 1a helper.
///
/// # Errors
/// Returns [`ViewError`] on a parse failure.
pub fn is_launchable(root: &Path, target: &str) -> Result<bool, ViewError> {
    let model = Model::build(root)?;
    Ok(model.items.get(target).is_some_and(|i| matches!(i.type_name.as_str(), "Process" | "AISkill")))
}

/// Evaluate ONE declared rule by name → `(subjects_scanned, sorted violations)`.
///
/// The CONTRACT single source (D0107): the 5 migrated guards source their violations here instead of a
/// bespoke Rust predicate.
///
/// # Errors
/// [`ViewError`] on a parse failure, an unknown rule name, or an unsupported predicate/scope.
/// Like [`rule_violations`], but `Ok(None)` when the named rule is simply NOT DECLARED.
///
/// Closes issue090 (D0136's class, second instance). Six hard guards are rule-sourced, and a missing
/// rule made `rule_violations` return `Err`, which those guards reported as a VIOLATION — so a project
/// whose `.engine/rules/` is absent, older, or authored fresh got six hard failures and every commit
/// blocked, with messages naming internal rule names that mean nothing to it. A project that never
/// adopted a control has not violated it.
///
/// The distinction matters in both directions: an ABSENT rule means the control is not adopted (the
/// caller should pass, but say so visibly, so deleting a rule to dodge a gate is never silent), while a
/// MALFORMED rule is a real error and must still fail.
///
/// # Errors
/// Returns [`ViewError`] for a genuine parse/compute failure — never for mere absence.
pub fn rule_violations_opt(root: &Path, rule_name: &str) -> Result<Option<(usize, Vec<String>)>, ViewError> {
    if !Model::build(root)?.items.contains_key(rule_name) {
        return Ok(None);
    }
    rule_violations(root, rule_name).map(Some)
}

/// Violations of a single declared rule, as `(scanned, violating element names)`.
///
/// # Errors
/// Returns [`ViewError`] if the rule is not declared, is malformed, or a tracking file fails to parse.
/// Prefer [`rule_violations_opt`] in a GUARD: absence means the control is not adopted, not violated.
pub fn rule_violations(root: &Path, rule_name: &str) -> Result<(usize, Vec<String>), ViewError> {
    let model = Model::build(root)?;
    let Some(info) = model.items.get(rule_name) else {
        return Err(ViewError::Track(rule_name.to_string(), format!("declared rule '{rule_name}' not found")));
    };
    let a = |k: &str| info.attrs.get(k).cloned().unwrap_or_default();
    let scope = {
        let s = a("appliesWhen");
        if s.is_empty() { "all".to_string() } else { s }
    };
    let subject = a("subjectType");
    match info.type_name.as_str() {
        "EdgeRule" => {
            let scope_files = if scope == "newlyAdded" { Some(staged_added_files(root)) } else { None };
            let scanned = model.items.values().filter(|i| rc_matches_subject(i, &subject) && scope_files.as_ref().is_none_or(|f| f.contains(&i.file))).count();
            let v = edge_rule_violations(&model, &subject, &a("requiredEdge").to_lowercase(), &a("objectType"), &a("edgeDirection"), &a("cardinality"), scope_files.as_ref());
            Ok((scanned, v))
        }
        "ElementRule" => {
            let scanned = model.items.values().filter(|i| rc_matches_subject(i, &subject) && subject_in_scope(i, &scope).unwrap_or(false)).count();
            let v = element_rule_violations(&model, &subject, &a("predicate"), &scope)
                .ok_or_else(|| ViewError::Track(rule_name.to_string(), "unsupported predicate/scope term".to_string()))?;
            Ok((scanned, v))
        }
        other => Err(ViewError::Track(rule_name.to_string(), format!("unknown rule kind '{other}'"))),
    }
}

/// `keel rules` (D0105 EXPAND step 2): evaluate DECLARED rules over the model.
///
/// The generic evaluator that replaces the bespoke guards once each reaches PARITY
/// (guardsToRulesMigration). This walking skeleton evaluates `EdgeRule` with `appliesWhen="all"`;
/// `ElementRule`/`OrderingRule` and the full scope sub-language are later EXPAND steps (reported
/// `evaluated=false` meanwhile). Runs ALONGSIDE `keel guard` — nothing is retired here.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance/rule file fails to parse.
pub fn check(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut rule_names: Vec<&String> =
        model.items.iter().filter(|(_, i)| matches!(i.type_name.as_str(), "EdgeRule" | "ElementRule")).map(|(n, _)| n).collect();
    rule_names.sort();
    let mut rules_json: Vec<Json> = Vec::new();
    let mut total = 0usize;
    for rname in rule_names {
        let Some(info) = model.items.get(rname) else { continue };
        let kind = info.type_name.clone();
        let a = |k: &str| info.attrs.get(k).cloned().unwrap_or_default();
        let scope = {
            let s = a("appliesWhen");
            if s.is_empty() { "all".to_string() } else { s }
        };
        let (violations, evaluated) = if kind == "EdgeRule" {
            // EdgeRule scope: `all` (whole model) or `newlyAdded` (git staged-added files); else unsupported.
            let scope_files = if scope == "newlyAdded" { Some(staged_added_files(root)) } else { None };
            if scope == "all" || scope == "newlyAdded" {
                (edge_rule_violations(&model, &a("subjectType"), &a("requiredEdge").to_lowercase(), &a("objectType"), &a("edgeDirection"), &a("cardinality"), scope_files.as_ref()), true)
            } else {
                (Vec::new(), false)
            }
        } else {
            // ElementRule handles scope (all / whereStatus) itself; None => unsupported scope/predicate.
            element_rule_violations(&model, &a("subjectType"), &a("predicate"), &scope).map_or((Vec::new(), false), |v| (v, true))
        };
        total += violations.len();
        rules_json.push(Json::Obj(vec![
            ("rule".to_string(), Json::s(rname.clone())),
            ("kind".to_string(), Json::s(kind)),
            ("severity".to_string(), Json::s(a("severity"))),
            ("scope".to_string(), Json::s(scope)),
            ("evaluated".to_string(), Json::Bool(evaluated)),
            ("violations".to_string(), Json::Arr(violations.into_iter().map(Json::s).collect())),
        ]));
    }
    Ok(Json::Obj(vec![
        ("check".to_string(), Json::s("declared-rule evaluation (D0105; EdgeRule + ElementRule, appliesWhen=all)")),
        ("rules".to_string(), Json::Arr(rules_json)),
        ("total_violations".to_string(), Json::Int(i64::try_from(total).unwrap_or(i64::MAX))),
    ])
    .dump())
}

/// Requirement-rootedness view (D0098/D0099, issue047): the charter-source BURNDOWN (need-rooted vs
/// decision-driven vs orphan) over all delivery Stories, plus the `#Capability` gate set.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn rootedness(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let mut stories: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Story").map(|(n, _)| n).collect();
    stories.sort();
    let n = |c: usize| Json::Int(i64::try_from(c).unwrap_or(i64::MAX));
    let class_of: Vec<(&String, &str)> = stories.iter().map(|s| (*s, rd_charter_class(&model, s))).collect();
    let count = |k: &str| class_of.iter().filter(|(_, c)| *c == k).count();
    let orphans: Vec<Json> = class_of.iter().filter(|(_, c)| *c == "orphan").map(|(s, _)| Json::s((*s).clone())).collect();
    let gate: Vec<Json> = capability_root_violations(&model).into_iter().map(Json::s).collect();
    let out = Json::Obj(vec![
        ("rootedness".to_string(), Json::s("requirement rootedness (D0098/D0099, issue047): charter-source burndown over delivery Stories — `need` reaches a Need, `decision` is legitimate decision-driven engine evolution (D0064), `orphan` has no charter. The HARD gate (`guard requirement-rootedness`) fires only on a #Capability item with no #DerivedFrom->Need.")),
        ("total".to_string(), n(stories.len())),
        ("need_rooted".to_string(), n(count("need"))),
        // Split per issue176: one number used to cover both legitimate engine evolution and work
        // nobody asked for. `decision_chartered` is retained as their SUM so no existing reader breaks.
        ("decision_chartered".to_string(), n(count("decision_rooted") + count("decision_ungrounded"))),
        ("decision_rooted".to_string(), n(count("decision_rooted"))),
        ("decision_ungrounded".to_string(), n(count("decision_ungrounded"))),
        ("orphan".to_string(), n(count("orphan"))),
        ("orphans".to_string(), Json::Arr(orphans)),
        ("capability_violations".to_string(), Json::Arr(gate)),
    ]);
    Ok(out.dump())
}


#[cfg(test)]
mod tests {
    use super::quotes_conversational_words;

    fn item(type_name: &str, attrs: &[(&str, &str)]) -> super::ItemInfo {
        super::ItemInfo {
            type_name: type_name.to_string(),
            attrs: attrs.iter().map(|(k, v)| ((*k).to_string(), (*v).to_string())).collect(),
            marker: None,
            file: String::new(),
        }
    }

    /// issue287: the substance rule binds DELEGATED records only. A record whose recorder IS the judge
    /// (the human in the console, or at their own terminal) passes without quoting themselves; a record
    /// an AI made on their behalf with no quote still FAILS; a record with no recorder stamped (every
    /// acceptance before D0299) is read as delegated, so provenance absent is never provenance assumed.
    #[test]
    fn the_substance_rule_binds_only_delegated_acceptances() {
        let eval = |recorder: Option<&str>, note: &str| {
            let mut model = super::Model { items: std::collections::HashMap::new(), edges: Vec::new() };
            model.items.insert("d1".to_string(), item("Decision", &[("status", "accepted")]));
            model.items.insert("d1Accept".to_string(), item("Test", &[("method", "confirmation"), ("procedureText", note)]));
            let mut r1 = vec![("outcome", "pass"), ("judgedAt", "2026-09-04"), ("judgedBy", "hum")];
            if let Some(r) = recorder {
                r1.push(("createdBy", r));
            }
            model.items.insert("d1AcceptR1".to_string(), item("TestResult", &r1));
            super::eval_predicate_term(&model, "d1", "acceptQuotesDelegatedWords(2026-08-22)")
        };
        assert_eq!(eval(Some("hum"), "agreed with panel's decisions"), Some(true), "self-recorded: the human need not quote themselves");
        assert_eq!(eval(Some("bot"), "agreed with panel's decisions"), Some(false), "AI-delegated without a quote still FAILS");
        assert_eq!(eval(Some("bot"), "their words: 'yes, accept it as written'"), Some(true), "AI-delegated with the quote passes");
        assert_eq!(eval(None, "agreed with panel's decisions"), Some(false), "no recorder stamped = delegated, not self-recorded");
        assert_eq!(eval(None, "accepted in the keel console"), Some(true), "a cited human gesture still passes unstamped records");
    }

    /// D0192 OPTION A: the substance check's boundary. A delegated record passes on a verbatim
    /// single-quoted span (>= 10 chars) or a named human gesture; a bare assertion fails.
    #[test]
    fn delegated_records_must_quote_or_cite_a_gesture() {
        // The real D0192 acceptance shape: quoted verbatim words.
        assert!(quotes_conversational_words(
            "OPTION A chosen. Their words, verbatim: 'option A is fine. this hasn't been a real issue to date' (chat, 2026-08-22)."
        ));
        // A deck tap is a human gesture — no quote needed.
        assert!(quotes_conversational_words("signed via deck"));
        assert!(quotes_conversational_words("accepted at the console review queue"));
        // issue235: a GitHub comment citation is the authenticated gesture — a bare option letter is enough.
        assert!(quotes_conversational_words("OPTION A - their words, verbatim: 'A' (GitHub comment https://github.com/o/r/issues/3#issuecomment-1, authenticated login williamweatherholtz)"));
        // A bare assertion carries no channel evidence.
        assert!(!quotes_conversational_words("the human approved this decision in chat"));
        // Contractions inside the quote must not terminate the span (the 'yep let's go' incident).
        assert!(quotes_conversational_words("Their words, verbatim: 'yep let's go' (chat)"));
        // A short quoted fragment is an apostrophe artifact, not conversational words.
        assert!(!quotes_conversational_words("they said 'ok' and moved on"));
        assert!(!quotes_conversational_words(""));
    }
}
