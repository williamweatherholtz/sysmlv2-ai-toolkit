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
            println!("  {}  {w}", crate::color::warn("WARN"));
        }
        for v in &self.violations {
            println!("  {} {v}", crate::color::fail("ERROR"));
        }
        // SELF-CONTRADICTION CHECK (issue180). A guard cannot find a violation in a population of
        // zero, so `0 scanned, 1 violation(s)` means the guard is not reporting what it examined. Three
        // guards printed exactly that while working correctly, which made `scanned` useless as a
        // liveness signal - the only signal separating a guard whose population is legitimately empty
        // from one that is mis-aimed and can never fire. Surfaced in the RUNNER rather than as a test,
        // so it holds for every guard added after this one without anybody remembering to.
        if self.scanned == 0 && !self.violations.is_empty() {
            println!(
                "  {}  guard `{}` reports {} violation(s) against a scan count of 0 - it is not                  reporting the population it examined (issue180)",
                crate::color::warn("WARN"),
                self.name,
                self.violations.len()
            );
        }
        println!(
            "[guard:{}] {} — {} scanned, {} warning(s), {} violation(s)",
            self.name,
            crate::color::verdict(self.ok()),
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

/// The day the actor convention became binding. Every legacy-actor reference in the corpus is dated
/// before it (newest observed: 2026-06-11), so this starts at zero violations; a legacy name on a
/// record dated on or after it is a VIOLATION (D0261). Deriving the verdict from the RECORD'S OWN
/// DATE is what makes this a ratchet rather than a second hand-maintained baseline that drifts.
const LEGACY_ACTOR_CUTOFF: &str = "2026-06-12";

/// The `judgedAt`/`createdAt` date declared on the same line, if any. Item declarations in this
/// corpus are single-line, so the line carries its own date; a line without one is treated as
/// undatable history rather than assumed recent.
fn record_date(line: &str) -> Option<String> {
    for attr in ["judgedAt", "createdAt", "saidAt", "acceptedAt"] {
        if let Some(rest) = line.split(attr).nth(1) {
            let digits: String =
                rest.trim_start_matches([' ', '=', '"']).chars().take(10).collect();
            if digits.len() == 10 && digits.as_bytes().get(4) == Some(&b'-') {
                return Some(digits);
            }
        }
    }
    None
}

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
    let mut legacy_historic = 0usize;
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
                    // A legacy name in a record dated ON OR AFTER the convention is a VIOLATION,
                    // not tolerated history — the date comes from the record itself, so this needs
                    // no baseline list to drift (D0261). Older ones are COUNTED, not enumerated:
                    // 52 undischargeable lines per run were 54% of the whole warning channel, and
                    // real findings sat unread behind them for four days.
                    if record_date(line).is_some_and(|d| d.as_str() >= LEGACY_ACTOR_CUTOFF) {
                        violations.push(format!(
                            "{rel}:{}: legacy actor \"{val}\" in a record dated on/after {LEGACY_ACTOR_CUTOFF} \
                             — legacy names are tolerated only in history that predates the convention",
                            i + 1
                        ));
                    } else {
                        legacy_historic += 1;
                    }
                } else {
                    violations.push(format!("{rel}:{}: unknown actor \"{val}\" not in ProjectActors", i + 1));
                }
            }
        }
    }
    if legacy_historic > 0 {
        warnings.push(format!(
            "{legacy_historic} legacy actor reference(s) in records predating the {LEGACY_ACTOR_CUTOFF}              convention — immutable history, NOT dischargeable (rewriting a judgedBy would falsify              provenance). Counted, not enumerated: a warning nobody can act on trains blindness to              the ones they can. A legacy name dated on/after the cutoff is a violation above."
        ));
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
/// D0232's cutover. Both attestation guards bind FORWARD only: 983 `method=test` results and 38
/// coverage claims predate the convention, and retro-fitting evidence nobody captured would mean
/// inventing it — which is the very failure these guards exist to prevent. Same shape as D0198's
/// quote-receipt cutover.
const EVIDENCE_ENFORCED_FROM: &str = "2026-08-25";

/// Read a quoted `:>> <name> = "value"` off a single declaration line.
fn field(line: &str, name: &str) -> Option<String> {
    let needle = format!(":>> {name} = \"");
    let rest = line.split(&needle).nth(1)?;
    Some(rest.split('"').next()?.to_string())
}

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

/// The tasks declared inside a delivery file's `action def` block, and the `TestResult` names
/// present in that same file. A task is STAMPED when some result name starts with the task name
/// (`storyFooDoDR1` / `storyFooR1` both stamp `storyFoo` — both spellings are in the corpus).
fn sprint_tasks_and_results(src: &str) -> (Vec<String>, Vec<String>) {
    let tasks = src
        .lines()
        .filter_map(|l| l.trim().strip_prefix("action ")?.strip_suffix(';'))
        .filter(|t| !t.contains(' ') && !t.contains(':'))
        .map(str::to_string)
        .collect();
    let results = src
        .split("part ")
        .skip(1)
        .filter(|seg| seg.starts_with(char::is_alphanumeric))
        .filter_map(|seg| {
            let name = seg.split_whitespace().next()?;
            seg.split_once(" : ")
                .filter(|(_, rest)| rest.starts_with("TestResult"))
                .map(|_| name.to_string())
        })
        .collect();
    (tasks, results)
}

/// Guard: a sprint the work has MOVED ON FROM may not carry an unstamped task (D0260).
///
/// Sprint 483's story was finished and verified, its result never appended, and the frontier
/// therefore served finished work as ready for three weeks — the one miss in 496 sprints, found
/// only when D0258's priority-assessment step first read the frontier's head item by item.
///
/// The check is a RATCHET, not a new burden: all 496 sprint files already stamp every task, so
/// this guard starts at zero violations and exists to keep a perfect record perfect. It is
/// scoped to avoid the issue272 failure — blocking legitimate work to prevent an illegitimate
/// state. The HIGHEST-numbered sprint is exempt, because an in-progress sprint has unstamped
/// tasks by definition and gating it would make the guard a lockout. Opening sprint N+1 is the
/// objective, self-declared event that says N is no longer in progress.
#[must_use]
pub fn sprint_closure(root: &Path) -> GuardReport {
    let files = crate::collect_sysml(&root.join(".tracking").join("delivery"));
    let number = |p: &Path| -> Option<u32> {
        p.file_name()?
            .to_str()?
            .strip_prefix("sprint")?
            .split(|c: char| !c.is_ascii_digit())
            .next()?
            .parse()
            .ok()
    };
    // The in-progress exemption: the single highest sprint number present.
    let newest = files.iter().filter_map(|p| number(p)).max();
    let mut violations = Vec::new();
    let mut scanned = 0usize;
    for path in &files {
        let Ok(src) = std::fs::read_to_string(path) else { continue };
        if !src.contains("action def ") {
            continue;
        }
        if number(path).is_some() && number(path) == newest {
            continue; // in progress — see doc comment
        }
        let (tasks, results) = sprint_tasks_and_results(&src);
        scanned += tasks.len();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
        for t in tasks {
            if !results.iter().any(|r| r.starts_with(&t)) {
                violations.push(format!(
                    "{name}: task `{t}` has no TestResult, but work has moved on to a later sprint \
                     — an unstamped task is served as READY forever, so finished work is \
                     indistinguishable from open work (D0260/sprint483)"
                ));
            }
        }
    }
    violations.sort();
    GuardReport { name: "sprint-closure", scanned, warnings: Vec::new(), violations }
}

/// Guard: work descended from an UNTRUSTED utterance must be routed through a Decision (D0264).
///
/// An issue on a public tracker is an instruction from an unauthenticated stranger. Triaging it is
/// fine — reading is not obeying — but routing it straight to an implementation task means the
/// project acts on a stranger's instruction with nobody having agreed to it. That is prompt
/// injection with a filing form, and the defence is not detection but ROUTING: untrusted input may
/// produce a plan and a proposed Decision, and a human accepts before anything is built.
///
/// SCOPE, deliberately narrow. This fires only on a story that HAS been routed (`#Implicates`) and
/// whose targets contain no Decision. An unrouted story is untriaged, not a violation — the guard
/// must not punish work-in-progress, which is how a control gets bypassed instead of obeyed.
#[must_use]
pub fn untrusted_routing(root: &Path) -> GuardReport {
    let blob = crate::collect_sysml(&root.join(".tracking"))
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .collect::<Vec<_>>()
        .join("\n");
    // Statements whose recorded tier is `untrusted`.
    let untrusted: HashSet<String> = blob
        .split("part ")
        .skip(1)
        .filter(|seg| seg.contains("SourceTrust::untrusted"))
        .filter_map(|seg| seg.split_whitespace().next().map(str::to_string))
        .collect();
    // Stories deriving from one of them.
    let mut tainted: HashSet<String> = HashSet::new();
    for line in blob.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("#DerivedFrom dependency from ") {
            if let Some((story, target)) = rest.split_once(" to ") {
                if untrusted.contains(target.trim_end_matches(';').trim()) {
                    tainted.insert(story.trim().to_string());
                }
            }
        }
    }
    // Of those, the ones ROUTED somewhere, and whether any target is a Decision.
    let mut routed: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for line in blob.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("#Implicates dependency from ") {
            if let Some((story, target)) = rest.split_once(" to ") {
                let story = story.trim().to_string();
                if tainted.contains(&story) {
                    routed.entry(story).or_default().push(target.trim_end_matches(';').trim().to_string());
                }
            }
        }
    }
    let is_decision = |t: &String| {
        t.len() >= 5 && t.starts_with('d') && t[1..5].chars().all(|c| c.is_ascii_digit())
    };
    let violations: Vec<String> = routed
        .iter()
        .filter(|(_, targets)| !targets.iter().any(is_decision))
        .map(|(story, targets)| {
            format!(
                "{story} descends from an UNTRUSTED utterance and is routed to {targets:?} with no \
                 Decision among them — untrusted input may PLAN, and a human accepts before anything \
                 is implemented (D0264). Propose a Decision and route the story to it."
            )
        })
        .collect();
    // SCANNED is the population POLICED - untrusted utterances - not the subset currently in
    // violation. Reporting the tainted-story count instead read 0 on a tree holding four untrusted
    // statements, which the liveness meta-test rightly rejected: a guard whose population is always
    // zero has no signal distinguishing "nothing to police" from "mis-aimed" (issue180).
    GuardReport { name: "untrusted-routing", scanned: untrusted.len(), warnings: Vec::new(), violations }
}

/// Guard (D0278): the control-defect registry names real controls and still-open Issues.
///
/// # Why the registry needs its own guard
///
/// `.engine/contracts/control-defects.toml` makes a defective control announce itself beside its own
/// verdict, which only works while the entries are true. Two ways it rots, both silent:
///
/// - A typo'd or retired CONTROL NAME. The entry then belongs to nothing, so the announcement never
///   prints and a known-broken guard goes back to handing out unqualified greens — the exact state
///   the registry was built to end, restored without anyone noticing.
/// - An Issue that has since been RESOLVED. The announcement then keeps qualifying verdicts that are
///   now sound, which is how a true warning becomes noise and then becomes scrolled past (D0214).
///   The process says the entry is removed in the same commit that resolves the Issue; this is what
///   makes that a rule rather than a hope.
///
/// The direction is checked too, because `note()` picks its wording from it: a mistyped direction
/// would tell the reader a green is untrustworthy when the defect is over-reporting, or worse, the
/// reverse.
#[must_use]
pub fn control_defect_registry(root: &Path) -> GuardReport {
    let entries = crate::control_defects::load(root);
    if entries.is_empty() {
        return GuardReport { name: "control-defect-registry", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    }
    let done = crate::orient::done_names(root);
    let open: std::collections::HashSet<String> = match crate::view::open_issue_names(root, &done) {
        Ok(v) => v.into_iter().collect(),
        Err(e) => {
            return GuardReport {
                name: "control-defect-registry",
                scanned: entries.len(),
                warnings: Vec::new(),
                violations: vec![format!("cannot read issue resolution to check the registry: {e}")],
            };
        }
    };
    // Every Issue this project HAS, open or closed. The registry ships with the engine into every
    // `keel init` project (the defects are the ENGINE's guards, so the note is true downstream), but
    // the Issues it cites live in the engine repository's own .tracking. A downstream project sees
    // the entry and lacks the Issue — found by init_smoke going red on a fresh scaffold. So an id
    // that is ABSENT here is "tracked upstream" and a warning; an id that is PRESENT and CLOSED is
    // the rot the guard exists to catch and stays a violation.
    let known: std::collections::HashSet<String> =
        crate::view::all_issue_names(root, &done).unwrap_or_default().into_iter().collect();
    let mut violations = Vec::new();
    let mut warnings = Vec::new();
    for (control, d) in &entries {
        if !GUARD_NAMES.contains(&control.as_str()) {
            violations.push(format!(
                "control-defects.toml names `{control}`, which is not a guard in this binary — the defect note would never print, so a control known to be broken would silently go back to giving unqualified verdicts"
            ));
        }
        if d.direction != "over" && d.direction != "under" {
            violations.push(format!(
                "`{control}`: direction `{}` is neither `over` nor `under` — the note's wording is chosen from it, so a wrong value tells the reader to distrust the wrong half of the verdict",
                d.direction
            ));
        }
        if !known.contains(&d.issue) {
            warnings.push(format!(
                "`{control}` is registered against {}, which this project does not hold — the defect is the engine's and is tracked in the engine repository; the note still prints, and this entry is removed by the engine's next resync when it resolves upstream",
                d.issue
            ));
        } else if !open.contains(&d.issue) {
            violations.push(format!(
                "`{control}` is registered against {}, which is RESOLVED — the entry should have been removed in that same commit (D0278). A note that outlives its defect is how a true warning becomes noise",
                d.issue
            ));
        }
    }
    GuardReport { name: "control-defect-registry", scanned: entries.len(), warnings, violations }
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

/// Is this repo-relative path under the KEYSTONE LOCK — process definition or enforcement surface?
///
/// Public because the workspace gate has to ask the same question about paths that belong to no
/// project (issue276). One predicate, one answer: a second copy in `workspace.rs` would be a second
/// place for the locked set to be true, and it would drift the first time a path class is added here.
#[must_use]
pub fn is_locked_path(p: &str) -> bool {
    is_process_def(p) || is_enforcement_surface(p)
}

/// Does this commit carry a staged Decision bearing a `#ProspectiveChange`/`#SafetyChange` marker?
///
/// The authorisation half of the keystone lock, exposed for the workspace gate. Additions AND
/// modifications, because the authorising Decision is almost always a NEW file — conflating the two
/// lists is the regression `process_change` documents catching on its own author.
/// A Decision at ANY depth, because in a workspace there is no project at the repository root and so
/// no `.engine/decisions/` there. Requiring the root-relative form would make the shared root hook
/// permanently unchangeable: no Decision could ever authorise a change to it, which is a different
/// failure from the one being fixed but just as wrong. A marked Decision recorded in ANY project in
/// the repository authorises the repository's own locked surface — the human signing it is the same
/// human either way, and the alternative is a surface with no legitimate path to change.
fn is_decision_file_at_any_depth(p: &str) -> bool {
    let s = p.replace('\\', "/");
    is_sysml(p) && (s.starts_with(".engine/decisions/") || s.contains("/.engine/decisions/"))
}

#[must_use]
pub fn staged_marked_decision(root: &Path) -> bool {
    staged_files(root)
        .iter()
        .filter(|p| is_decision_file_at_any_depth(p))
        .any(|p| has_process_marker(&git_stdout(root, &["show", &format!(":{p}")])))
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
    staged_changes(root, "ACMR")
}

/// Staged paths for a given `--diff-filter`, scoped to this project, with BOTH sides of a rename
/// and no C-quoting.
///
/// TWO CORRECTIONS AN ADVERSARIAL PANEL FOUND, both of which let a locked file change unsigned:
///
/// 1. **Renames.** `--name-only` prints only a rename's DESTINATION, and the keystone filter had been
///    narrowed to `MD`, which excludes `R` entirely. So `sed 's/blocking/warning/' rules.sysml >
///    rules2.sysml; rm rules.sysml; git add -A` presented git with `R096` — and the guard saw NOTHING.
///    Every blocking rule in a project could be downgraded to a warning by `git mv`, in a
///    single-project repo, with no Decision. `--name-status -M` is read instead, and BOTH paths of an
///    `R` are returned: the source matters (a locked file left its path) and so does the destination
///    (a locked file arrived carrying modified content).
/// 2. **Non-ASCII paths.** git C-QUOTES a path containing a non-ASCII byte in this output
///    (`"pr\303\264j/..."`) while `rev-parse --show-prefix` does NOT, so the prefix comparison below
///    stripped nothing and a project named `prôj` was invisible to the workspace gate AND to its own
///    guards — a full rules downgrade there passed. `core.quotePath=false` with `-z` records makes
///    both sides of that comparison the same bytes.
fn staged_changes(root: &Path, filter: &str) -> Vec<String> {
    // SCOPED TO THIS PROJECT (D0234/issue271). `git diff --cached` answers for the whole REPOSITORY
    // no matter which subdirectory you ask from, so in a repo holding several keel projects every
    // project's guards saw every other project's staged files — and the workspace-level ones too.
    // Found live: a two-project workspace failed BOTH projects' process-change guard over
    // `.githooks/pre-commit`, a file belonging to neither. Paths are returned repo-relative, so they
    // are re-based onto the project and anything outside it is dropped.
    let flag = format!("--diff-filter={filter}");
    // `-c core.quotePath=false` with `-z`: see (2) above. `--name-status -M` so a rename yields BOTH
    // of its paths: see (1). Records are NUL-separated; an `R`/`C` status is followed by TWO paths.
    let raw = git_stdout(
        root,
        &["-c", "core.quotePath=false", "diff", "--cached", "--name-status", "-M", "-z", &flag],
    );
    let mut fields = raw.split('\0').filter(|f| !f.is_empty());
    let mut repo_relative: Vec<String> = Vec::new();
    while let Some(status) = fields.next() {
        let renamed = status.starts_with('R') || status.starts_with('C');
        let Some(first) = fields.next() else { break };
        repo_relative.push(first.to_string());
        if renamed {
            // BOTH sides. Dropping the source is what let a locked file be edited-and-moved unsigned.
            if let Some(second) = fields.next() {
                repo_relative.push(second.to_string());
            }
        }
    }
    let prefix = git_stdout(root, &["rev-parse", "--show-prefix"]).trim().replace('\\', "/");
    repo_relative
        .into_iter()
        .map(|l| l.trim().replace('\\', "/"))
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            if prefix.is_empty() {
                // The project IS the repo root: every staged file is its own, as it always was.
                Some(l)
            } else {
                // Keep only what lives under this project, re-based to project-relative.
                l.strip_prefix(&prefix).map(str::to_string)
            }
        })
        .collect()
}

/// Guard: a staged process-def change must carry a co-committed marked Decision (D0070).
///
/// A staged change to `.engine/processes|workflows/*.sysml` MUST be co-committed with a
/// `#ProspectiveChange`/`#SafetyChange`-marked Decision (the keystone hard lock). Mirrors
/// `validate_process_change.py`.
#[must_use]
pub fn process_change(root: &Path) -> GuardReport {
    // MODIFIED or DELETED, never merely ADDED (issue272). The keystone lock exists so a control
    // cannot be weakened without a signed Decision. An ADDED locked file is a control ARRIVING, and
    // treating that as a weakening made a freshly scaffolded project unable to make its FIRST
    // commit: `keel init` stages every `.engine/processes/*` file, so the guard demanded a
    // process-change Decision for a scaffold the author did not write. That is the first thing a new
    // user does, and it failed.
    //
    // STATED RESIDUAL: an added process CAN weaken, by asserting a constraint that claims a CORE
    // guard and so makes it switchable (the issue242 capture). That transition is caught by
    // `audit-adherence`'s guard-state monotonicity, which ranks CORE above ACTIVE and fails the
    // build on a downgrade (D0209 clause 1) — so the hole is covered, by the check built for it.
    let changed = staged_changes(root, "MDR");
    // TWO LISTS, and conflating them was a regression this very guard caught on its author within
    // the minute: what TRIGGERS the lock is a locked file being modified or deleted, but the
    // co-committed Decision that AUTHORISES it is almost always a NEW file — so searching for it in
    // the modify-only list found nothing and refused a properly signed change. The Decision is
    // looked for among all staged additions and modifications.
    let decision_texts: Vec<(String, String)> = staged_files(root)
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


// ── acceptance-binds-to-text (issue341 / D0308): the text signed is the text carried ─────────────

/// The three fields an acceptance is a judgment OF.
const ACCEPTED_FIELDS: [&str; 3] = ["decision", "rationale", "consequences"];

/// The VALUE of a string attribute: every quoted segment of `:>> key = "a" + "b";` concatenated.
/// The early Decisions write long fields as `+`-joined segments across lines, and the indentation
/// between segments is source formatting, not signed text - comparing raw source read ten of them as
/// drifted on whitespace alone.
fn field_of(text: &str, key: &str) -> Option<String> {
    let start = text.find(&format!(":>> {key} = "))? + key.len() + 7;
    let rest = &text[start..];
    let mut value = String::new();
    let mut in_str = false;
    let mut saw_any = false;
    for c in rest.chars() {
        if in_str {
            if c == '"' {
                in_str = false;
            } else {
                value.push(c);
            }
        } else if c == '"' {
            in_str = true;
            saw_any = true;
        } else if c == ';' {
            break;
        } else if !(c.is_whitespace() || c == '+') {
            break; // not a string attribute
        }
    }
    saw_any.then_some(value)
}

/// The latest `{dec}AcceptR<n>` result's `judgedAgainst` - the SHA the acceptance currently binds to.
fn latest_acceptance_sha(text: &str, dec: &str) -> Option<String> {
    let prefix = format!("part {dec}AcceptR");
    let mut best: Option<(u32, String)> = None;
    for (i, _) in text.match_indices(&prefix) {
        let after = &text[i + prefix.len()..];
        let n: u32 = after.chars().take_while(char::is_ascii_digit).collect::<String>().parse().ok()?;
        let ja = after.find("judgedAgainst = \"")? + 17;
        let sha: String = after[ja..].chars().take_while(char::is_ascii_hexdigit).collect();
        if best.as_ref().is_none_or(|(bn, _)| n > *bn) {
            best = Some((n, sha));
        }
    }
    best.map(|(_, s)| s)
}

/// Pure core: which of the accepted fields differ between the text at acceptance and the text at HEAD,
/// compared as string VALUES (see `field_of`).
fn drifted_fields(accepted: &str, head: &str) -> Vec<&'static str> {
    ACCEPTED_FIELDS.iter().copied().filter(|k| field_of(accepted, k) != field_of(head, k)).collect()
}

/// Guard (hard, D0308 / issue341): the text a human signed is the text the tree carries.
///
/// An accepted Decision's decision, rationale and consequences at HEAD must equal its text at the
/// SHA its latest acceptance result binds to - the file at `judgedAgainst`, or,
/// when the acceptance was recorded in the same commit as the Decision (every standing-consent
/// auto-accept), the commit that introduced the file. A difference is a violation naming the Decision,
/// both SHAs and the fields; the remedy is `keel accept <d> --rebind` on a human judgment that the
/// words still hold, never an edit of history. Measured before the guard existed: 17 of 301 accepted
/// Decisions had drifted, all editorially (PROPOSED->ACCEPTED wording inside the text, rollout notes,
/// guard-count renumbering) - recorded as issue371, re-bound, not allowlisted.
#[must_use]
pub fn acceptance_binds_to_text(root: &Path) -> GuardReport {
    let dir = root.join(".engine").join("decisions");
    let files = crate::collect_sysml(&dir);
    if files.is_empty() {
        return GuardReport { name: "acceptance-binds-to-text", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    }
    // Every accepted Decision with a result: (rel path, decision, sha, head text).
    let mut accepted: Vec<(String, String, String, String)> = Vec::new();
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        if !text.contains("DecisionStatus::accepted") {
            continue;
        }
        let Some(i) = text.find("part d0") else { continue };
        let dec: String = text[i + 5..].chars().take_while(|c| c.is_alphanumeric()).collect();
        let Some(sha) = latest_acceptance_sha(&text, &dec) else { continue };
        let rel = relpath(root, f);
        accepted.push((rel, dec, sha, text));
    }
    // One batch for the texts at the binding SHAs; one `git log` for every file's introducing commit.
    let keys: Vec<String> = accepted.iter().map(|(rel, _, sha, _)| format!("{sha}:{rel}")).collect();
    let blobs = crate::orient::batch_cat_blobs(root, &keys);
    let intro_log = git_stdout(root, &["log", "--diff-filter=A", "--format=%h", "--name-only", "--", ".engine/decisions"]);
    let mut introducing: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut current = String::new();
    for line in intro_log.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        if std::path::Path::new(l).extension().is_some_and(|e| e.eq_ignore_ascii_case("sysml")) {
            introducing.entry(l.replace('\\', "/")).or_insert_with(|| current.clone());
        } else {
            current = l.to_string();
        }
    }
    let intro_keys: Vec<String> = accepted
        .iter()
        .filter(|(rel, _, sha, _)| blobs.get(&format!("{sha}:{rel}")).cloned().flatten().is_none())
        .filter_map(|(rel, _, _, _)| introducing.get(rel).map(|c| format!("{c}:{rel}")))
        .collect();
    let intro_blobs = crate::orient::batch_cat_blobs(root, &intro_keys);
    let head = git_stdout(root, &["rev-parse", "--short", "HEAD"]).trim().to_string();
    let mut violations = Vec::new();
    let mut warnings = Vec::new();
    let mut scanned = 0usize;
    for (rel, dec, sha, text) in &accepted {
        let at_binding = blobs.get(&format!("{sha}:{rel}")).cloned().flatten();
        let (base_sha, base) = at_binding.map_or_else(
            || {
                introducing.get(rel).map_or_else(
                    || (sha.clone(), None),
                    |c| (format!("{c} (the commit that introduced the file; the acceptance at {sha} predates it)"), intro_blobs.get(&format!("{c}:{rel}")).cloned().flatten()),
                )
            },
            |b| (sha.clone(), Some(b)),
        );
        let Some(base) = base else {
            warnings.push(format!("{dec}: no text found at its acceptance SHA {sha} or at any introducing commit - the binding cannot be checked yet (not yet committed, a shallow clone, or a file renamed since); it will be, at the first commit that holds it"));
            continue;
        };
        scanned += 1;
        let drift = drifted_fields(&base, text);
        if !drift.is_empty() {
            violations.push(format!(
                "{dec}: {} changed since the acceptance bound at {base_sha} - the text at {head} (HEAD) is not the text the human signed (issue341/D0308). If the words still hold, re-bind with `keel accept {dec} --rebind --note \"<what changed, why it still holds>\"`; never edit the acceptance.",
                drift.join(", ")
            ));
        }
    }
    violations.sort();
    GuardReport { name: "acceptance-binds-to-text", scanned, warnings, violations }
}

#[cfg(test)]
mod binds_to_text_tests {
    use super::{drifted_fields, latest_acceptance_sha};

    /// D0308: the LATEST acceptance result is the binding; the three accepted fields are compared;
    /// a re-binding to the current text clears the drift while the first acceptance stays in place.
    #[test]
    fn the_latest_acceptance_binds_and_drift_is_per_field() {
        let at_accept = r#"part d1 : Decision { :>> decision = "do X"; :>> rationale = "because"; :>> consequences = "Y"; }
    part d1AcceptR1 : TestResult { :>> judgedAgainst = "aaa1111"; }"#;
        let head = r#"part d1 : Decision { :>> decision = "do X"; :>> rationale = "because"; :>> consequences = "Y. ROLLOUT: landed."; }
    part d1AcceptR1 : TestResult { :>> judgedAgainst = "aaa1111"; }"#;
        assert_eq!(drifted_fields(at_accept, head), vec!["consequences"]);
        assert_eq!(latest_acceptance_sha(head, "d1"), Some("aaa1111".to_string()));
        let rebound = format!("{head}
    part d1AcceptR2 : TestResult {{ :>> judgedAgainst = \"bbb2222\"; :>> notes = \"REBOUND\"; }}");
        assert_eq!(latest_acceptance_sha(&rebound, "d1"), Some("bbb2222".to_string()), "the re-binding is the new baseline");
        assert!(drifted_fields(head, head).is_empty());
        // formatting between `+`-joined segments is not signed text
        let joined_a = ":>> consequences = \"Pros: a, \"\n        + \"b.\";";
        let joined_b = ":>> consequences = \"Pros: a, \"\r\n            + \"b.\";";
        assert!(drifted_fields(joined_a, joined_b).is_empty(), "indentation and line endings between segments are formatting");
        assert_eq!(super::field_of(joined_a, "consequences").as_deref(), Some("Pros: a, b."));
    }
}

// ── unit-extras-present (issue290 / D0300): a unit's declared mechanism is in the tree ──────────

/// Every `[unit]` section of `unit-extras.toml` with its declared `files`, in file order.
fn declared_extras(root: &Path) -> Vec<(String, Vec<String>)> {
    let Ok(text) = std::fs::read_to_string(root.join(".engine/contracts/unit-extras.toml")) else {
        return Vec::new();
    };
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    let mut in_files = false;
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('#') {
            continue;
        }
        if let Some(name) = l.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
            out.push((name.to_string(), Vec::new()));
            in_files = false;
            continue;
        }
        // A key line may carry its array INLINE (`files = ["a", "b"]`) or open a multi-line one; both
        // are TOML, and the first draft of this parser (like `process_cmd::unit_extras`) read only the
        // second - a one-line declaration scanned as zero files, silently.
        if let Some(rest) = l.strip_prefix("files") {
            let rest = rest.trim_start_matches([' ', '=']).trim();
            if let Some(inline) = rest.strip_prefix('[').filter(|r| r.contains(']')) {
                if let Some(body) = inline.split(']').next() {
                    if let Some((_, files)) = out.last_mut() {
                        files.extend(body.split(',').map(|v| v.trim().trim_matches('"').to_string()).filter(|v| !v.is_empty()));
                    }
                }
                in_files = false;
            } else {
                in_files = true;
            }
            continue;
        }
        if l.starts_with("requires") || l == "]" {
            in_files = false;
            continue;
        }
        if in_files {
            let v = l.trim_end_matches(',').trim().trim_matches('"');
            if let (false, Some((_, files))) = (v.is_empty(), out.last_mut()) {
                files.push(v.to_string());
            }
        }
    }
    out
}

/// The process names `installed-units.toml` records as installed here.
fn installed_unit_names(root: &Path) -> Vec<String> {
    std::fs::read_to_string(root.join(".engine/contracts/installed-units.toml"))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.trim().strip_prefix("process = "))
        .map(|v| v.trim().trim_matches('"').to_string())
        .collect()
}

/// Pure core (issue290): for every INSTALLED unit that declares extras, each declared file must exist;
/// a missing one is a violation naming unit and path. A unit not installed here is not judged - its
/// extras are somebody else's mechanism.
fn extras_violations(declared: &[(String, Vec<String>)], installed: &[String], exists: &dyn Fn(&str) -> bool) -> (usize, Vec<String>) {
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for (unit, files) in declared {
        if !installed.iter().any(|u| u == unit) {
            continue;
        }
        for f in files {
            scanned += 1;
            if !exists(f) {
                violations.push(format!(
                    "unit `{unit}` declares `{f}` as part of its mechanism (unit-extras.toml) and the file is not in this tree - the process definition and its skill reference machinery that does not exist here (issue290: penumbra adopted a unit whose four mechanism files were hand-staged out). Import the unit with `keel process import` rather than staging files by hand, or remove the declaration if the unit no longer carries it."
                ));
            }
        }
    }
    (scanned, violations)
}

/// Guard: a declared unit extra exists in every project that installed the unit (D0300).
///
/// The tool-reference guard already refuses a doc naming a deleted `.engine/tools/` file; this is the
/// same predicate for a unit's declared MECHANISM - workflows, scripts, contracts a process needs to
/// RUN. issue290: a project adopted a unit whose definition and skill were present and whose four
/// mechanism files were not, because they were hand-staged out of the PR; nothing caught it, since
/// adoption-check gates what EXPORT produces and never what a target received.
#[must_use]
pub fn unit_extras_present(root: &Path) -> GuardReport {
    let declared = declared_extras(root);
    let installed = installed_unit_names(root);
    let (scanned, violations) = extras_violations(&declared, &installed, &|f| root.join(f).exists());
    GuardReport { name: "unit-extras-present", scanned, warnings: Vec::new(), violations }
}

#[cfg(test)]
mod extras_tests {
    use super::extras_violations;

    fn decl() -> Vec<(String, Vec<String>)> {
        vec![("channel".to_string(), vec![".github/workflows/decision-issue.yml".to_string(), ".github/scripts/decide.py".to_string()])]
    }

    /// Both TOML array shapes are read - inline and multi-line - because a one-line declaration used to
    /// scan as zero files, which is a guard passing over the thing it was built to see.
    #[test]
    fn declared_extras_reads_inline_and_multiline_arrays() {
        let root = std::env::temp_dir().join(format!("keel-extras-parse-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".engine").join("contracts")).expect("mkdir");
        std::fs::write(
            root.join(".engine").join("contracts").join("unit-extras.toml"),
            "# header\n[one]\nfiles = [\"a.yml\", \"b.py\"]\nrequires = [\"x\"]\n[two]\nfiles = [\n  \"c.yml\",\n]\n",
        )
        .expect("toml");
        let d = super::declared_extras(&root);
        assert_eq!(d, vec![("one".to_string(), vec!["a.yml".to_string(), "b.py".to_string()]), ("two".to_string(), vec!["c.yml".to_string()])]);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// issue290, the penumbra shape: the unit is installed, two of its mechanism files are absent - two
    /// violations naming unit and path; present -> none; not installed here -> not judged.
    #[test]
    fn an_installed_unit_with_absent_mechanism_fails_and_present_passes() {
        let installed = vec!["channel".to_string()];
        let (scanned, v) = extras_violations(&decl(), &installed, &|_| false);
        assert_eq!((scanned, v.len()), (2, 2), "{v:?}");
        assert!(v[0].contains("channel") && v[0].contains("decision-issue.yml") && v[0].contains("keel process import"), "{}", v[0]);
        let (_, v) = extras_violations(&decl(), &installed, &|_| true);
        assert!(v.is_empty(), "present files are not a finding");
        let (scanned, v) = extras_violations(&decl(), &[], &|_| false);
        assert_eq!((scanned, v.len()), (0, 0), "a unit not installed here is not judged");
    }
}

// ── decision-amends-process (issue298 / D0244): an amendment reaches the definition ──────────────

/// One process definition's lexical anchors: its file, and the identifiers a Decision would use to
/// speak about it - the `Process` action name, the file stem, and every `ProcessStep` name.
struct ProcessAnchors {
    file: String,
    names: Vec<String>,
}

fn process_anchors(root: &Path) -> Vec<ProcessAnchors> {
    let dir = root.join(".engine").join("processes");
    let Ok(rd) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut out = Vec::new();
    for entry in rd.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "sysml") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let stem = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let mut names = vec![stem];
        for line in text.lines() {
            let l = line.trim();
            if let Some(rest) = l.strip_prefix("action ") {
                if rest.contains(": Process ") || rest.contains(": Process{") || rest.contains(": ProcessStep") {
                    if let Some(name) = rest.split(|c: char| c == ':' || c.is_whitespace()).next() {
                        if !name.is_empty() {
                            names.push(name.to_string());
                        }
                    }
                }
            }
        }
        out.push(ProcessAnchors { file: format!(".engine/processes/{}", path.file_name().unwrap_or_default().to_string_lossy()), names });
    }
    out.sort_by(|a, b| a.file.cmp(&b.file));
    out
}

/// Does `text` contain `ident` as a whole identifier (not as a substring of a longer name)?
fn names_identifier(text: &str, ident: &str) -> bool {
    text.match_indices(ident).any(|(i, _)| {
        let before = text[..i].chars().next_back();
        let after = text[i + ident.len()..].chars().next();
        !before.is_some_and(|c| c.is_alphanumeric() || c == '_') && !after.is_some_and(|c| c.is_alphanumeric() || c == '_')
    })
}

/// Pure core (issue298): for each staged Decision, every process it names by identifier whose
/// definition file is NOT also staged yields one warning naming the Decision, the identifier and the
/// file it should have reached.
fn amendment_warnings(decision_texts: &[(String, String)], staged: &[String], anchors: &[ProcessAnchors]) -> Vec<String> {
    let mut out = Vec::new();
    for (path, text) in decision_texts {
        for a in anchors {
            if staged.iter().any(|s| s == &a.file) {
                continue;
            }
            // An anchor must LOOK like an identifier - camelCase or hyphenated. A process whose name is a
            // plain English word (`migration`, `dor`) would fire on every sentence using the word: the
            // first commit carrying this guard warned on "step 5 of the migration plan".
            let mut hit: Vec<&str> = a
                .names
                .iter()
                .filter(|n| n.len() > 3 && (n.contains('-') || n.chars().any(char::is_uppercase)) && names_identifier(text, n))
                .map(String::as_str)
                .collect();
            hit.sort_unstable();
            hit.dedup();
            if hit.is_empty() {
                continue;
            }
            out.push(format!(
                "{path} names `{}` of {} and this commit does not touch that definition - if the Decision AMENDS the step, edit the process file in the same commit so the definition carries it (D0244: a design may amend a process, never quietly disagree with one; issue298 is D0243 doing exactly this); if it only CITES the step, this is noise to read past",
                hit.join("`, `"),
                a.file
            ));
        }
    }
    out
}

/// Guard (WARNING-tier, D0102 promote-once-low-noise): a staged Decision that names a process or one
/// of its steps by identifier while that process definition is unchanged in the same commit.
///
/// THE CLASS (issue298): D0243 changed what knowledge-graph-memory's steps 1 and 2 MEAN - `.knowledge/`
/// optional, seeding corpus-derived - and never touched the process file, so the keystone lock never
/// fired: the guarded artifact was not edited, the change was made AROUND the control. D0244 wrote the
/// rule ("a design may amend a process; it may not quietly disagree with one") and carried D0243's
/// amendment into the definition by hand. This guard watches for the next one.
///
/// WHAT IT CAN AND CANNOT SEE, measured over the 296 Decisions in this tree before it was written:
/// 17 name a process step by identifier and 5 of those did not touch the file (D0242 among them - a
/// real amendment). 24 mention steps by NUMBER ("steps 1-2", "step 5 of the plan", "STPA step 2") and
/// two thirds of those are not amendments at all, so numbers are not an anchor. D0243 itself says
/// "the source process's steps 1-2" and names nothing - the identifier rule would NOT have caught it.
/// That residual is stated here rather than hidden behind a passing check: the durable fix for the
/// unnamed shape is an `#Amends` edge a Decision must carry, which is a schema change and a fork.
#[must_use]
pub fn decision_amends_process(root: &Path) -> GuardReport {
    let staged = staged_files(root);
    let decision_texts: Vec<(String, String)> = staged
        .iter()
        .filter(|p| is_decision_file(p))
        .map(|p| (p.clone(), git_stdout(root, &["show", &format!(":{p}")])))
        // A staged Decision whose PROSE is unchanged from HEAD - an appended acceptance result, a
        // re-binding (D0308) - amends nothing; the first re-bind of seventeen Decisions produced
        // twenty-five citation warnings, all noise. Only text that moved can amend.
        .filter(|(p, staged_text)| {
            let at_head = git_stdout(root, &["show", &format!("HEAD:{p}")]);
            at_head.is_empty() || ["decision", "rationale", "consequences", "context"].iter().any(|k| field_of(&at_head, k) != field_of(staged_text, k))
        })
        .collect();
    let anchors = process_anchors(root);
    let warnings = amendment_warnings(&decision_texts, &staged, &anchors);
    GuardReport { name: "decision-amends-process", scanned: decision_texts.len(), warnings, violations: Vec::new() }
}

#[cfg(test)]
mod amendment_tests {
    use super::{amendment_warnings, names_identifier, ProcessAnchors};

    fn kg() -> Vec<ProcessAnchors> {
        vec![ProcessAnchors {
            file: ".engine/processes/knowledge-graph-memory.sysml".to_string(),
            names: vec!["knowledge-graph-memory".to_string(), "knowledgeGraphMemory".to_string(), "kgInjection".to_string(), "kgQuestions".to_string()],
        }]
    }

    /// issue298, the D0242 shape: a Decision names `kgInjection`; the process file is not staged; one
    /// warning names the Decision, the identifier and the file.
    #[test]
    fn a_decision_naming_a_step_without_the_definition_warns() {
        let d = (".engine/decisions/0242-x.sysml".to_string(), "The kgInjection step now pushes before the model thinks.".to_string());
        let w = amendment_warnings(std::slice::from_ref(&d), std::slice::from_ref(&d.0), &kg());
        assert_eq!(w.len(), 1, "{w:?}");
        assert!(w[0].contains("kgInjection") && w[0].contains("knowledge-graph-memory.sysml") && w[0].contains("D0244"), "{}", w[0]);
        // the same commit touching the definition is the amendment reaching it: silent
        let staged = vec![d.0.clone(), ".engine/processes/knowledge-graph-memory.sysml".to_string()];
        assert!(amendment_warnings(&[d], &staged, &kg()).is_empty());
    }

    /// The stated residual, armed so it cannot be forgotten: D0243's own sentence names NOTHING and is
    /// not caught. If this test ever fails, the residual has been closed and the doc must say so.
    #[test]
    fn the_unnamed_shape_is_a_known_miss() {
        let d = (".engine/decisions/0243-x.sysml".to_string(), "as the source process's steps 1-2 imply - rejected; .knowledge/ becomes an optional refinement".to_string());
        let staged = vec![d.0.clone()];
        assert!(amendment_warnings(std::slice::from_ref(&d), &staged, &kg()).is_empty(), "the identifier rule does not see an unnamed process");
    }

    /// A process named by a plain English word is not an anchor: `migration` in "step 5 of the
    /// migration plan" is prose, and the first commit carrying this guard warned on exactly that.
    #[test]
    fn a_plain_word_process_name_is_not_an_anchor() {
        let anchors = vec![ProcessAnchors { file: ".engine/processes/migration.sysml".to_string(), names: vec!["migration".to_string(), "migration".to_string(), "mgExpand".to_string()] }];
        let d = (".engine/decisions/0281-x.sysml".to_string(), "This is step 5 of the migration plan.".to_string());
        assert!(amendment_warnings(std::slice::from_ref(&d), std::slice::from_ref(&d.0), &anchors).is_empty());
        let d = (".engine/decisions/0281-y.sysml".to_string(), "The mgExpand step now also copies the pin.".to_string());
        let w = amendment_warnings(std::slice::from_ref(&d), std::slice::from_ref(&d.0), &anchors);
        assert_eq!(w.len(), 1, "a camelCase step name still anchors: {w:?}");
    }

    /// Whole identifiers only: `kgInjectionProvenOnTheTrap` (a backlog task) must not read as the
    /// step `kgInjection`, or every sprint that delivered the step would warn.
    #[test]
    fn a_longer_name_containing_the_step_is_not_the_step() {
        assert!(!names_identifier("dcKgInjectionProvenOnTheTrap kgInjectionProvenOnTheTrap", "kgInjection"));
        assert!(names_identifier("the kgInjection step", "kgInjection"));
        assert!(names_identifier("(kgInjection)", "kgInjection"));
    }
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

/// D0304's date: an Issue created on or after it must be NAMED by its resolver's text.
const ISSUE_NAMING_CUTOFF: &str = "2026-09-04";

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
            let mut violations: Vec<String> = untriaged
                .into_iter()
                .map(|i| format!("{i}: untriaged — no #Resolves edge (D0077; link a resolving action or Decision)"))
                .collect();
            let mut warnings = Vec::new();
            // issue333 / D0304: an edge that EXISTS is not a triage - the resolver must NAME the issue.
            // Forward-only from D0304's date (the issue068 pattern): 144 of 387 historical resolutions
            // never name their issue and re-triaging them is a sitting review the human may choose
            // (D0204), never owed; they are counted once so the number cannot hide. NOTE issue352: this
            // cutoff is the ENGINE's adoption date and is retroactive for a downstream project that
            // migrates into it - dcForwardOnlyCutoffIsTheProjectsOwn is the open fix for that class.
            match crate::view::unnamed_resolutions(root, ISSUE_NAMING_CUTOFF) {
                Ok((_, forward, historical)) => {
                    for (issue, resolver) in forward {
                        violations.push(format!(
                            "{issue}: its resolver `{resolver}` never names it (title, DoD text or decision) - an edge that exists is not a triage; a resolver that will close the issue says so (issue333/D0304). Name the issue in the resolver's DoD, or re-triage to the item that resolves it."
                        ));
                    }
                    if historical > 0 {
                        warnings.push(format!(
                            "{historical} resolution(s) recorded before {ISSUE_NAMING_CUTOFF} do not name their issue - history, forward-only from D0304; re-triaging them is a pull-audit the human may choose (D0204), never owed"
                        ));
                    }
                }
                Err(e) => violations.push(format!("error reading resolutions: {e}")),
            }
            GuardReport { name: "issues", scanned: total, warnings, violations }
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


/// Classify a viewpoint renderer string: `"retired"` (query.py/report.py, a violation), `"planned"`
/// (a tolerated warning), `"ok"` (names a real `keel` subcommand), or `"unknown"` (a violation).
fn classify_renderer(r: &str) -> &'static str {
    if r.contains("query.py") || r.contains("report.py") {
        "retired"
    } else if r.starts_with("(planned") {
        "planned"
    } else if crate::cli_surface::renderer_command(r).is_some_and(|(verb, lens)| {
        // D0273: the lens family collapsed into ONE router, so a renderer now reads
        // `keel show <lens>`. Accepting only the verb would let `keel show frobnicate` pass, which
        // is the same hole with an extra word in it — so when the verb is the router, the LENS is
        // what must resolve.
        if verb == "show" {
            lens.is_some_and(crate::cli_surface::has_lens)
        } else {
            crate::cli_surface::has_command(verb)
        }
    }) {
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

/// Phrases by which a retro EXPLICITLY justifies raising no tracked item.
///
/// The obligation is not "always create an item" — sometimes a control already exists, and adding a
/// duplicate is noise. The obligation is that the choice is STATED rather than left silent, so a
/// reader can tell a considered decision from an omission.
pub(crate) const RETRO_NO_ITEM_JUSTIFICATIONS: &[&str] = &["no new item", "no item needed", "already tracked", "no further item"];

/// Every tracked-item NAME this retro's own text mentions — `dcCamelCase` and `issueNNN` tokens.
///
/// The RETRO's text, not the whole sprint file: a sprint file legitimately names the task it
/// delivered in its `DoD` line, and that name satisfied the old check for every retro ever written.
pub(crate) fn named_items(text: &str) -> Vec<String> {
    /// `dc` is followed by an uppercase letter; `issue` by a digit.
    type NextOk = fn(char) -> bool;
    let bytes = text.as_bytes();
    let boundary =
        |i: usize| i.checked_sub(1).and_then(|j| bytes.get(j)).is_none_or(|b| !b.is_ascii_alphanumeric());
    let mut out = Vec::new();
    // A Decision (`d0289`) tracks a finding as legitimately as a task or an Issue does - a "won't do"
    // IS a Decision (Invariant 4) - so a retro may name one to justify raising nothing else.
    let pairs: [(&str, NextOk); 3] = [("dc", |c| c.is_ascii_uppercase()), ("issue", |c| c.is_ascii_digit()), ("d0", |c| c.is_ascii_digit())];
    for (needle, ok_next) in pairs {
        let mut from = 0;
        while let Some(rel) = text[from..].find(needle) {
            let st = from + rel;
            let rest = &text[st + needle.len()..];
            if boundary(st) && rest.starts_with(ok_next) {
                let name: String = text[st..].chars().take_while(char::is_ascii_alphanumeric).collect();
                out.push(name);
            }
            from = st + needle.len();
        }
    }
    out
}

/// The `procedureText` of every RETRO gate in a sprint file — the `method = analyze` verifications
/// whose title says retro. Returns the texts; a file with no retro gate yields none.
///
/// Blocks start at a LINE that begins with `verification ` — never at the word inside a string. The
/// first version split the whole file on the word, so a retro whose text mentioned "verification"
/// (the commonest word in this repository) was cut in half and silently not examined: the guard passed
/// a retro it had not read. Found by this guard's own arming test on 2026-09-03 (issue364, second shape).
pub(crate) fn retro_texts(sprint_file: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut blocks: Vec<String> = Vec::new();
    for line in sprint_file.lines() {
        if line.trim_start().starts_with("verification ") {
            blocks.push(String::new());
        }
        if let Some(b) = blocks.last_mut() {
            b.push_str(line);
            b.push('\n');
        }
    }
    for block in blocks {
        let head = block.split('{').next().unwrap_or("");
        let is_retro = block.contains("VerificationMethod::analyze") && head.to_ascii_lowercase().contains("retro");
        if !is_retro {
            continue;
        }
        if let Some(i) = block.find("procedureText = \"") {
            let rest = &block[i + 17..];
            if let Some(j) = rest.find("\";") {
                out.push(rest[..j].to_string());
            }
        }
    }
    out
}

/// Violations for staged sprint records whose RETRO names findings that this commit does not track.
///
/// # Why this is the third shape of the check, and why the first two were both wrong
///
/// The first version was satisfied when the commit CO-STAGED any tracked file — and every commit
/// stages issues.sysml for something, so five retros reached zero items (issue189). The second
/// version, the fix for that, required the retro's text to NAME an item — and every sprint file names
/// the task it delivered, so the check was satisfied by construction; it also examined a retro only
/// if the text contained the literal tokens AVOIDABLE-ISSUE or LESSON:, so a retro that said FINDING
/// was never examined at all. Six findings on 2026-09-01 went through it that way (issue335).
///
/// This version asks the question the guard was always for: does THIS COMMIT add a tracked item that
/// THIS RETRO names? `staged_added_items` is the set of `part issueNNN` / `action dcX;` declarations
/// in the staged diff's added lines; a retro is clean when its own text names one of them, or when it
/// carries an explicit no-item justification. Every retro gate is examined — a `method = analyze`
/// gate titled retro IS a findings record, whatever words it uses.
///
/// # The fourth shape (issue364, D0293): a justification must name what tracks the finding
///
/// The third shape let a retro carry "no new item - already tracked" and pass with nothing checked -
/// the phrase was a substring match consulting no item. Two retros in two days said a finding was
/// tracked elsewhere when nothing tracked it, and both passed; the human's question found them. Now a
/// justified retro must NAME an item that exists (added by this commit, or already in the tree) - an
/// `issueNNN`, a `dcTask`, or a `dNNNN` Decision. Whether the named item actually covers the finding
/// stays the reader's judgment; that the claim points at something real is the guard's.
fn retro_backlog_violations(added_items: &[String], known_items: &[String], sprint_texts: &[(String, String)]) -> Vec<String> {
    let mut out = Vec::new();
    for (path, text) in sprint_texts {
        for retro in retro_texts(text) {
            let lower = retro.to_lowercase();
            let named = named_items(&retro);
            let exists = |n: &String| added_items.iter().any(|a| a == n) || known_items.iter().any(|k| k == n);
            if let Some(phrase) = RETRO_NO_ITEM_JUSTIFICATIONS.iter().find(|j| lower.contains(*j)) {
                if named.iter().any(exists) {
                    continue;
                }
                out.push(format!(
                    "{path}: the retro says '{phrase}' but names no existing item that tracks the finding ({}) — a justification must point at something real: the issueNNN, dcTask or dNNNN that carries it (D0131/D0293; issue364: two retros claimed 'already tracked' about untracked findings and passed)",
                    if named.is_empty() { "it names no item at all".to_string() } else { format!("it names {}, none of which exists", named.join(", ")) }
                ));
                continue;
            }
            if named.iter().any(|n| added_items.iter().any(|a| a == n)) {
                continue;
            }
            out.push(format!(
                "{path}: the retro records findings but this commit ADDS no tracked item the retro names ({}) — a finding must become a tracked, prioritized item in the SAME commit, or the retro must say why none is needed (D0131; issue335: naming an existing task is what every retro does, and is not tracking a finding)",
                if named.is_empty() { "it names no item at all".to_string() } else { format!("it names {}, none of which this commit adds", named.join(", ")) }
            ));
        }
    }
    out
}

/// `part issueNNN` and `action dcX;` declarations ADDED by the staged diff.
fn staged_added_items(root: &Path) -> Vec<String> {
    let out = crate::gitx::git().arg("-C").arg(root).args(["diff", "--cached", "-U0", "--", ".tracking"]).output();
    let Ok(out) = out else { return Vec::new() };
    let text = String::from_utf8_lossy(&out.stdout);
    let mut items = Vec::new();
    for line in text.lines().filter(|l| l.starts_with('+') && !l.starts_with("+++")) {
        let l = line[1..].trim_start();
        if let Some(rest) = l.strip_prefix("part ") {
            if rest.starts_with("issue") {
                items.push(rest.chars().take_while(char::is_ascii_alphanumeric).collect());
            }
        } else if let Some(rest) = l.strip_prefix("action ") {
            if rest.starts_with("dc") {
                items.push(rest.chars().take_while(char::is_ascii_alphanumeric).collect());
            }
        }
    }
    items
}

/// Test-only re-export of the pure warning builder (the view self-tests exercise it).
#[doc(hidden)]
#[must_use]
pub fn retro_backlog_violations_for_test(added_items: &[String], known_items: &[String], sprint_texts: &[(String, String)]) -> Vec<String> {
    retro_backlog_violations(added_items, known_items, sprint_texts)
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
    let added = staged_added_items(root);
    // What a justification may point at: every declared task, every Issue (open or done), every Decision.
    let mut known: Vec<String> = declared_task_names(root).into_iter().collect();
    known.extend(crate::view::all_issue_names(root, &HashSet::new()).unwrap_or_default());
    if let Ok(rd) = std::fs::read_dir(root.join(".engine").join("decisions")) {
        known.extend(rd.flatten().filter_map(|e| e.file_name().to_str().and_then(|f| f.get(..4)).filter(|n| n.chars().all(|c| c.is_ascii_digit())).map(|n| format!("d{n}"))));
    }
    GuardReport { name: "retro-backlog", scanned, warnings: Vec::new(), violations: retro_backlog_violations(&added, &known, &sprint_texts) }
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
                    format!("{lower} outranks {high}, whose computed class is {sev} (a resolved Issue's severity, or a finding retros keep naming as already tracked - D0311; `keel show priority` shows which) — if that is deliberate say so, otherwise reorder the backlog (D0052: declaration order IS priority; reordering is how you reprioritize)")
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
    // COUNTED, not enumerated (D0261). The allowlist is FIXED and anything outside it already
    // violates, so nothing can hide in this number - unlike the per-instance lines, which added no
    // information after the first run and crowded out findings a reader could act on.
    let mut grandfathered_thin = 0usize;
    match crate::view::thin_attestations(root) {
        Ok(found) => {
            let mut warnings = Vec::new();
            let mut violations = Vec::new();
            for (name, reason) in &found {
                if grandfathered.contains(name.as_str()) {
                    grandfathered_thin += 1;
                } else {
                    violations.push(format!("{name}: {reason} — a confirmation records a HUMAN's word, so it must say what was attested and to what (D0016/issue083)"));
                }
            }
            if grandfathered_thin > 0 {
                warnings.push(format!(
                    "{grandfathered_thin} grandfathered thin attestation(s) (pre-issue083) — counted,                      not enumerated (D0261): the allowlist is fixed, so anything outside it is a                      violation above and nothing can hide in this number"
                ));
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
pub const GUARD_NAMES: [&str; 61] =
    ["evidence-cited", "gating-workflow-history", "process-applicability", "doc-guard-count", "actors", "acceptance-events", "sprint-coverage", "ceremony", "charter", "process-change", "issues", "viewpoint-renderer", "manifest-coverage", "critic-independence", "process-skill", "requirement-rootedness", "decision-rationale", "attestation-substance", "marker-vocabulary", "duplicate-identity", "decision-requirement-link", "verification-trace", "priority-inversion", "retro-backlog", "confirmation-authenticity", "engine-lint", "doc-sync", "hook-config-integrity", "activation-manifest", "sequence-multiplicity", "parser-coverage", "base-first-justification", "edge-endpoints", "ownership", "attestation-authority", "type-collision", "attribute-vocabulary", "resolver-kind", "stale-gate-prose", "impossible-evidence-date", "identity-present", "identity-well-formed", "tool-reference", "scaffold-placeholder", "claude-surface-drift", "decision-scaffolding", "release-recorded", "enrollment-binding", "control-event-coverage", "question-coverage", "claim-ancestry", "judgment-request-quality", "manifest-key-portability", "control-map-reconciled", "sprint-closure", "untrusted-routing", "control-defect-registry", "cli-surface-declared", "decision-amends-process", "unit-extras-present", "acceptance-binds-to-text"];


// ── control-map-reconciled guard (issue304, chartered by D0255) ──────────────────────────────────

/// Every control event names a DECLARED control, or says why it is instrumentation instead.
///
/// # The third failure class, and why only a check closes it
///
/// D0217 named declared-but-never-fired. D0253 named declared-and-unprobeable. Both are visible to a
/// reader who looks. This closes the one that is visible to NOBODY: a control that is IMPLEMENTED and
/// FIRING while the control map does not know it exists. `keel controls` computes the hazard/control
/// diff over DECLARED controls and DECLARED hazards, so an undeclared control cannot appear as a gap —
/// and neither can a hazard only that control covers.
///
/// The perverse property is what makes a check mandatory rather than a habit: the coverage measure
/// IMPROVES as the map gets less complete, because fewer declared controls with no gaps reads better
/// than more declared controls with gaps. Reconciling by hand once would leave that incentive intact.
///
/// Found this way, not by reasoning: the map declared nine controls while `control-events.toml`
/// declared fourteen events, two of which — the post-edit fast tier and the turn-boundary stop gate —
/// BLOCK, and neither was in the map.
///
/// # Why events are the anchor
///
/// A control that can fire leaves a counted record, and `control-event-coverage` (D0193) already
/// cross-checks that declaration against the event names the binary actually emits. Anchoring here
/// chains binary → events → controls, so the map is reconciled against something already tied to the
/// code rather than against a second hand-maintained list — which would be one more thing to drift.
fn control_map_reconciled(root: &Path) -> GuardReport {
    let path = root.join(".engine").join("contracts").join("control-events.toml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        // D0136: absence is a state, stated. A project that never adopted control events has nothing
        // to reconcile, and reporting a violation would fire on a project that opted out.
        return GuardReport { name: "control-map-reconciled", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    };
    let declared: std::collections::HashSet<String> = crate::collect_sysml(&root.join(".tracking"))
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .flat_map(|t| {
            t.match_indices("part ctl")
                .filter_map(|(i, _)| {
                    t.get(i + 5..).and_then(|rest| {
                        rest.split_whitespace().next().map(std::string::ToString::to_string)
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // SCOPED TO ADOPTION (D0231/issue090, caught by the adoption-check test on a fresh scaffold).
    // `keel init` ships this contract because it is engine vocabulary, but the control MAP is a
    // project's own instance data and is not shipped — so a fresh project inherits events naming
    // controls it has never declared, and an unscoped check fires on every downstream tree for
    // controls that are none of its business. A project with no control map has nothing to
    // reconcile: report the zero rather than a violation, so out-of-scope reads as out-of-scope.
    //
    // RESIDUAL, stated because it is real: a downstream project that builds its OWN control map
    // still inherits the shipped `control` bindings, which name this project's controls. Those
    // bindings are instance knowledge riding in an engine file, and until they are separated the
    // guard would mis-fire there too. Recorded rather than hidden behind a passing check.
    if declared.is_empty() {
        return GuardReport { name: "control-map-reconciled", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    }
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    let mut event = String::new();
    for line in text.lines() {
        let l = line.trim();
        if let Some(name) = l.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
            event = name.to_string();
            scanned += 1;
        } else if let Some(v) = l.strip_prefix("control = ") {
            let ctl = v.trim().trim_matches('"');
            if ctl != "none" && !declared.contains(ctl) {
                violations.push(format!(
                    "control-events.toml [{event}] names control `{ctl}`, which no .tracking/ file declares — an event that fires for an UNDECLARED control is the third failure class (D0254): `keel controls` computes over declared controls, so this one cannot appear as a gap and its absence makes coverage read cleaner rather than worse"
                ));
            }
        }
    }
    // An event with NO control line at all is the silent case this guard exists for: it neither
    // claims a control nor states that it is instrumentation.
    for block in text.split('[').skip(1) {
        let Some((name, body)) = block.split_once(']') else { continue };
        if !body.contains("control = ") {
            violations.push(format!(
                "control-events.toml [{name}] declares no `control` — say which declared control it is the firing of, or `control = \"none\"` with a `controlNote` saying why it is instrumentation. Silence must read as a gap, never as consent (the process-enforcement.toml convention)"
            ));
        }
    }
    // issue308 (propriety panel, pf33): a `provenBy` in control-arming.toml naming a file the tree
    // does not hold is a receipt-shaped TESTIMONY — worse than no claim, in the very contract that
    // exists to separate the two (D0253). Existence is the objective half and is checked here; that
    // the named test EXERCISES the control stays with the panel, being judgment (pf35).
    if let Ok(arming) = std::fs::read_to_string(root.join(".engine").join("contracts").join("control-arming.toml")) {
        for line in arming.lines() {
            let Some(v) = line.trim().strip_prefix("provenBy = ") else { continue };
            let rel = v.trim().trim_matches('"');
            scanned += 1;
            if !rel.is_empty() && !root.join(rel).exists() {
                violations.push(format!(
                    "control-arming.toml names provenBy `{rel}`, which does not exist in the tree — a dangling proof pointer reads as a receipt while being a testimony (issue308/D0253). Fix the path, or remove the claim"
                ));
            }
        }
    }
    GuardReport { name: "control-map-reconciled", scanned, warnings: Vec::new(), violations }
}

// ── manifest-key-portability guard (issue301, chartered by D0250) ────────────────────────────────

/// Every key in `installed-units.toml` must be repository-relative.
///
/// # Why a guard and not a one-time cleanup
///
/// Four keys under the `decision-channel` unit named this machine's home directory
/// (`file.C:__SL__Users__SL__<user>__SL__...`), because `unit_files` puts a unit's declared EXTRAS at
/// `root/<extra>` while the key builder stripped only the `.engine` prefix and fell through to the
/// absolute path. Fixing the builder and rewriting the four keys leaves nothing to stop the fifth:
/// the next extra added outside `.engine` would reintroduce it silently. D0047 — a defect that can
/// recur becomes a control, never a corrected file.
///
/// # Why this is now blocking rather than advisory
///
/// D0250 makes the library a git repository that other machines clone. A key naming the exporting
/// machine resolves to nothing on the importing one, so the three-way base that `--update` merges
/// against stops being found — silently, in the one file whose entire purpose is portability. The
/// failure surfaces on the SECOND machine, days later, as content mysteriously not updating.
///
/// Detection is by shape, not by this machine's paths: a Windows drive letter, a POSIX absolute
/// path, or a home-directory prefix. A guard that looked for `WilliamWeatherholtz` would pass on
/// every machine except the one that already got it right.
fn manifest_key_portability(root: &Path) -> GuardReport {
    let path = root.join(".engine").join("contracts").join("installed-units.toml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        // D0136: absence is a state, stated. A project with no installed units has no manifest.
        return GuardReport { name: "manifest-key-portability", scanned: 0, warnings: Vec::new(), violations: Vec::new() };
    };
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.trim().strip_prefix("file.") else { continue };
        let Some((key, _)) = rest.split_once(" = ") else { continue };
        scanned += 1;
        let decoded = key.replace("__SL__", "/");
        let absolute = decoded.starts_with('/')
            || decoded.starts_with('~')
            || decoded.as_bytes().get(1).is_some_and(|c| *c == b':');
        // issue307 (propriety panel, pf02): the class is "resolves outside the receiving project",
        // and absolute keys are only its loudest members. A RELATIVE key with traversal segments
        // (`../../elsewhere`) escapes the project root identically — the mutation the original
        // predicate did not kill. Segment-wise, not substring: a filename containing ".." is legal.
        let traverses = decoded.split('/').any(|seg| seg == "..");
        if absolute || traverses {
            let kind = if absolute { "an ABSOLUTE path" } else { "a TRAVERSAL path (contains a `..` segment)" };
            violations.push(format!(
                "{}: unit-manifest key `{decoded}` is {kind} — it resolves outside the receiving project, so a clone of this library cannot reconstruct it and the three-way `--update` base is silently lost (issue301/issue307/D0250). Keys are repository-relative and stay inside the root; a unit file outside it is refused at export, never absolutised",
                relpath(root, &path)
            ));
        }
    }
    GuardReport { name: "manifest-key-portability", scanned, warnings: Vec::new(), violations }
}

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

/// Guard 52: an AI-judged `method=test` result records WHAT WAS RUN (D0232/issue266).
///
/// MEASURED before it was built: of 5,135 recorded `TestResult`s, exactly ONE recorded what produced
/// it. A result carries outcome, judgedBy, judgedAt and judgedAgainst — so every `pass` in this model
/// is a TESTIMONY, and nothing in the record lets a third party re-derive it. That is the mechanism
/// behind a 3.9% fail rate over 5,120 results: the actor doing the work also authors the verdict.
///
/// AI-JUDGED ONLY, and that is the design rather than an exemption. Governance binds the AI; a
/// HUMAN's word IS the evidence, and demanding a receipt from them would point the control at the
/// wrong party. `method=confirmation` is human by construction (D0106) and never in scope here.
///
/// FORWARD-ONLY from [`EVIDENCE_ENFORCED_FROM`], on the D0198 precedent: 983 `method=test` results
/// predate the convention, and retro-fitting evidence nobody captured would mean inventing it.
fn evidence_cited(root: &Path) -> GuardReport {
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    // The Test declares the METHOD, the result declares the JUDGE — both are needed, so map first.
    let files = crate::collect_sysml(&root.join(".tracking"));
    let mut method_of: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        for cap in text.split("verification ").skip(1) {
            let Some(name) = cap.split([' ', ':']).next() else { continue };
            if let Some(m) = cap.split(":>> method = VerificationMethod::").nth(1) {
                if let Some(kind) = m.split([';', ' ']).next() {
                    method_of.insert(name.to_string(), kind.to_string());
                }
            }
        }
    }
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !line.contains(" : TestResult {") {
                continue;
            }
            let Some(judged_at) = field(line, "judgedAt") else { continue };
            if judged_at.as_str() < EVIDENCE_ENFORCED_FROM {
                continue;
            }
            let Some(judged_by) = field(line, "judgedBy") else { continue };
            // A human's attestation needs no receipt.
            if crate::actor::kind_of(root, &judged_by).as_deref() == Some("human") {
                continue;
            }
            let Some(part) =
                line.split(" : TestResult").next().and_then(|s| s.split("part ").nth(1))
            else {
                continue;
            };
            let base = part.trim().rsplit_once('R').map_or_else(|| part.trim(), |(b, _)| b);
            if method_of.get(base).map(String::as_str) != Some("test") {
                continue; // only an EXERCISED claim owes a re-runnable receipt
            }
            scanned += 1;
            // The receipt is a `// RAN:` comment on the result line or immediately above it.
            let has = line.contains("// RAN:")
                || i.checked_sub(1)
                    .and_then(|j| lines.get(j))
                    .is_some_and(|p| p.trim_start().starts_with("// RAN:"));
            if !has {
                violations.push(format!(
                    "{}:{}: {} is an AI-judged method=test result with no `// RAN:` receipt - a pass nobody else can re-derive is a testimony, not a test. Pass --evidence to append-result (D0232)",
                    relpath(root, f),
                    i + 1,
                    part.trim()
                ));
            }
        }
    }
    GuardReport { name: "evidence-cited", scanned, warnings: Vec::new(), violations }
}

/// Guard 51: a workflow that RUNS the gate must check out full history (D0229/issue260).
///
/// `claim-ancestry` and `audit-history` derive their verdicts from git history and SKIP LOUDLY on a
/// shallow clone - correctly, because a depth-dependent verdict is the machine-dependence K15
/// forbids. The consequence is that a gating workflow which forgets `fetch-depth: 0` silently runs
/// with those guards disabled. That is exactly what happened: `ci.yml` carried the fix (issue229),
/// `release.yml` never got it, and the release gate had been RED since guard 48 landed - undetected
/// for weeks because releases are rare enough that nobody ran it.
///
/// The same-fix-in-N-places class, which is the most-repeated finding in this project's retros. The
/// control is cheap: the fix is a literal, so its absence is decidable.
///
/// STATED LIMITATION: the check is per-FILE, not per-JOB, because there is no YAML parser here. A
/// workflow with two jobs where only the non-gating one sets `fetch-depth: 0` would pass - which is
/// the shape `release.yml` itself has (its build job checks out shallow, legitimately). It catches
/// the real failure class, a gating workflow with no full-history checkout anywhere, and nothing
/// finer. Recorded rather than left for someone to discover.
fn gating_workflow_history(root: &Path) -> GuardReport {
    let dir = root.join(".github").join("workflows");
    let mut scanned = 0usize;
    let mut violations = Vec::new();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return GuardReport { name: "gating-workflow-history", scanned, warnings: Vec::new(), violations };
    };
    let mut files: Vec<std::path::PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()).is_some_and(|e| e == "yml" || e == "yaml"))
        .collect();
    files.sort();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        // "Runs the gate" is judged by what the workflow actually invokes, not by its name.
        let gates = ["cargo test", "keel guard", "keel validate", "audit-history", "audit-adherence"]
            .iter()
            .any(|needle| text.contains(needle));
        if !gates || !text.contains("actions/checkout") {
            continue;
        }
        scanned += 1;
        if !text.contains("fetch-depth: 0") {
            violations.push(format!(
                "{}: runs the gate but checks out SHALLOW - every history-derived guard (claim-ancestry, audit-history) silently skips, so this workflow reports a pass it did not verify. Add `fetch-depth: 0` to the checkout (issue229/issue260)",
                relpath(root, path)
            ));
        }
    }
    GuardReport { name: "gating-workflow-history", scanned, warnings: Vec::new(), violations }
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
    // SCOPED TO ADOPTION (issue259, D0164). The APPLIES-WHEN fact exists to serve `project-onboarding`;
    // a project that has not adopted that process has not violated anything by lacking it, and gating
    // it anyway is the failure D0164 names - a control a project never adopted, enforced against it.
    // Found the hard way: this guard, hours after landing, failed all 22 processes of the first
    // project keel was adopted onto. Reported as 0 scanned rather than silently skipped, so an
    // out-of-scope guard is visible rather than a vacuous pass.
    if !dir.join("project-onboarding.sysml").exists() {
        return GuardReport { name: "process-applicability", scanned, warnings: Vec::new(), violations };
    }
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
    let mut warnings: Vec<String> = bare
        .into_iter()
        .map(|(d, at)| {
            format!(
                "{d} (accepted {at}): an accepted #ProspectiveChange Decision with NO inbound tracked-item edge — it promises process change but charters no work (D0188). Add a #CharteredBy/#DerivedFrom/#Resolves edge from the item that delivers it, or record why none is needed."
            )
        })
        .collect();
    // D0303 OPTION C (issue331): a Decision is ONE clause. The guard can see whether a Decision is
    // chartered but not which clause an edge covers - so a compound Decision half-delivered read as
    // covered (d0252, nine days). The human chose the convention over a schema change: from the day
    // after acceptance, a Decision whose text enumerates clauses is a violation naming the count and
    // the fix (one Decision per clause, edges between them); the ones before are counted once.
    let (forward, grandfathered) = compound_decisions(&texts, COMPOUND_DECISION_CUTOFF);
    let mut violations: Vec<String> = forward
        .into_iter()
        .map(|(d, n)| format!("{d}: a COMPOUND Decision - its text enumerates {n} clauses - and decision-scaffolding cannot see which clause an edge charters, so partial delivery would be invisible (issue331). D0303 option C: record one Decision per clause, with #DependsOn edges between them, and let each be chartered on its own."))
        .collect();
    violations.sort();
    if grandfathered > 0 {
        warnings.push(format!(
            "{grandfathered} compound Decision(s) recorded before {COMPOUND_DECISION_CUTOFF} enumerate several clauses - grandfathered under D0303 option C; their partial delivery is not visible to this guard, and re-splitting history is the D0129 class, so they are counted, not reported"
        ));
    }
    GuardReport { name: "decision-scaffolding", scanned, warnings, violations }
}

/// D0303 option C's date: a Decision created on or after it is one clause.
const COMPOUND_DECISION_CUTOFF: &str = "2026-09-05";

/// Pure core (issue331 / D0303 C): every Decision whose `decision` text enumerates two or more clauses
/// (`(1) ... (2) ...`, `(a) ... (b) ...`, or `clause A ... clause B`), split by its `createdAt`
/// against the cutoff into `(forward violators as (name, clause count), grandfathered count)`.
fn compound_decisions(texts: &[(String, String)], cutoff: &str) -> (Vec<(String, usize)>, usize) {
    let mut forward = Vec::new();
    let mut grandfathered = 0usize;
    for (_, t) in texts {
        let mut rest = t.as_str();
        while let Some(i) = rest.find("part d0") {
            let block = &rest[i..];
            let name: String = block[5..].chars().take_while(|c| c.is_alphanumeric()).collect();
            let end = block.find("\n    }").map_or(block.len(), |e| e + 6);
            let body = &block[..end];
            rest = &rest[i + 5 + name.len()..];
            if !body.contains(": Decision") {
                continue;
            }
            let field = |key: &str| -> &str {
                body.find(&format!("{key} = \"")).map_or("", |s| {
                    let v = &body[s + key.len() + 4..];
                    v.find('"').map_or(v, |e| &v[..e])
                })
            };
            let decision = field("decision");
            let clauses = clause_count(decision);
            if clauses < 2 {
                continue;
            }
            if field("createdAt") >= cutoff {
                forward.push((name, clauses));
            } else {
                grandfathered += 1;
            }
        }
    }
    (forward, grandfathered)
}

/// How many enumerated clauses a decision text carries: the largest of `(N)` numerals, `(x)` letters,
/// and `clause X` markers that appear in sequence from the first.
fn clause_count(text: &str) -> usize {
    let numeric = (1..=9).take_while(|n| text.contains(&format!("({n})"))).count();
    let lettered = ('a'..='i').take_while(|c| text.contains(&format!("({c})"))).count();
    let clause = ('A'..='I').take_while(|c| text.contains(&format!("clause {c}"))).count();
    numeric.max(lettered).max(clause)
}

#[cfg(test)]
mod compound_decision_tests {
    use super::{clause_count, compound_decisions};

    /// D0303 C armed against the broken predicate (D0253): a two-clause Decision created after the
    /// cutoff is reported with its count; one before is counted; a single-clause one is neither.
    #[test]
    fn a_compound_decision_is_reported_forward_only() {
        assert_eq!(clause_count("(1) first. (2) second. (3) third."), 3);
        assert_eq!(clause_count("clause A says x; clause B says y"), 2);
        assert_eq!(clause_count("one thing, said once"), 0);
        assert_eq!(clause_count("(2) alone does not enumerate"), 0);
        let mk = |name: &str, created: &str, decision: &str| format!(
            "package X {{\n    part {name} : Decision {{\n        :>> id = \"x\";\n        :>> title = \"t\";\n        :>> createdAt = \"{created}\";\n        :>> decision = \"{decision}\";\n    }}\n}}\n"
        );
        let texts = vec![
            ("a".to_string(), mk("d0901", "2026-09-10", "(1) build the thing; (2) also the other thing")),
            ("b".to_string(), mk("d0902", "2026-01-01", "(1) old; (2) compound; grandfathered")),
            ("c".to_string(), mk("d0903", "2026-09-10", "one clause, one Decision")),
        ];
        let (forward, grandfathered) = compound_decisions(&texts, "2026-09-05");
        assert_eq!(forward, vec![("d0901".to_string(), 2)]);
        assert_eq!(grandfathered, 1);
    }
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
        "sprint-closure" => Some(sprint_closure(root)),
        "untrusted-routing" => Some(untrusted_routing(root)),
        "control-defect-registry" => Some(control_defect_registry(root)),
        "cli-surface-declared" => Some(cli_surface_declared(root)),
        "decision-amends-process" => Some(decision_amends_process(root)), // WARNING-tier (issue298/D0244)
        "unit-extras-present" => Some(unit_extras_present(root)), // hard (issue290/D0300) - a unit's declared mechanism is in the tree
        "acceptance-binds-to-text" => Some(acceptance_binds_to_text(root)), // hard (issue341/D0308) - the text signed is the text carried
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
        // D0229: a workflow that RUNS the gate but checks out shallow silently disables every
        // history-derived guard in it. Found by the release gate being red since guard 48 landed.
        // D0232, both AI-ONLY: governance binds the AI, never the human. A human's word IS the
        // evidence; demanding a receipt from them would point the control at the wrong party.
        "evidence-cited" => Some(evidence_cited(root)),
        "gating-workflow-history" => Some(gating_workflow_history(root)),
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
        "manifest-key-portability" => Some(manifest_key_portability(root)), // issue301/D0250 — a unit manifest key naming one machine
        "control-map-reconciled" => Some(control_map_reconciled(root)), // issue304/D0255 — a firing control absent from the map

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

/// One CLI fact as authored in `.engine/cli/commands.sysml`, reduced to the fields the guard compares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredCliFact {
    pub name: String,
    pub family: String,
    pub effect: String,
    pub stability: String,
    pub synopsis: String,
}

/// Read one `:>> key = "value"` or `:>> key = Enum::member` attribute out of a single-line part.
fn cli_attr(line: &str, key: &str) -> Option<String> {
    let needle = format!(":>> {key} = ");
    let i = line.find(&needle)? + needle.len();
    let rest = &line[i..];
    if let Some(r) = rest.strip_prefix('"') {
        return r.find('"').map(|e| r[..e].to_string());
    }
    let end = rest.find(';')?;
    Some(rest[..end].rsplit("::").next().unwrap_or("").to_string())
}

/// Parse the `CliCommand` facts out of the file text.
///
/// Pure, so the comparison is unit-testable on a fixture; the model loader is not used because this guard must also run in a tree whose `.tracking`
/// is unrelated to the engine (a downstream project).
#[must_use]
pub fn parse_cli_facts(text: &str) -> Vec<AuthoredCliFact> {
    text.lines()
        .filter(|l| l.contains(": CliCommand {"))
        .filter_map(|l| {
            Some(AuthoredCliFact {
                name: cli_attr(l, "name")?,
                family: cli_attr(l, "family")?,
                effect: cli_attr(l, "effect")?,
                stability: cli_attr(l, "stability")?,
                synopsis: cli_attr(l, "synopsis")?,
            })
        })
        .collect()
}

/// The comparison, pure: authored facts vs the Rust mirror vs the dispatch inventory.
///
/// BOTH WAYS on every edge. Returns violations only - there is no advisory shape here, because any disagreement
/// means `keel --help` describes a surface that does not exist or hides one that does.
#[must_use]
pub fn cli_surface_violations(
    authored: &[AuthoredCliFact],
    mirror: &[crate::cli_facts::CliFact],
    commands: &[&str],
    lenses: &[&str],
) -> Vec<String> {
    use std::collections::BTreeMap;
    let mut out = Vec::new();
    let by_name: BTreeMap<&str, &AuthoredCliFact> = authored.iter().map(|f| (f.name.as_str(), f)).collect();
    let mirror_by: BTreeMap<&str, &crate::cli_facts::CliFact> = mirror.iter().map(|f| (f.name, f)).collect();
    // authored <-> mirror
    for f in authored {
        match mirror_by.get(f.name.as_str()) {
            None => out.push(format!("`{}` is an authored CliCommand fact with no entry in cli_facts.rs - the help cannot describe it", f.name)),
            Some(m) => {
                for (what, a, b) in [("family", f.family.as_str(), m.family), ("effect", f.effect.as_str(), m.effect), ("stability", f.stability.as_str(), m.stability), ("synopsis", f.synopsis.as_str(), m.synopsis)] {
                    if a != b {
                        out.push(format!("`{}` {what} differs: facts say `{a}`, cli_facts.rs says `{b}` - one home, the .sysml; regenerate the mirror", f.name));
                    }
                }
            }
        }
    }
    for m in mirror {
        if !by_name.contains_key(m.name) {
            out.push(format!("`{}` is in cli_facts.rs but has no authored CliCommand fact - the help describes a command the model does not declare", m.name));
        }
    }
    // authored <-> dispatch, by kind
    for f in authored {
        let is_lens = f.family == "lens";
        let dispatched = if is_lens { lenses.contains(&f.name.as_str()) } else { commands.contains(&f.name.as_str()) };
        if !dispatched {
            out.push(format!("`{}` is declared as a {} but nothing dispatches it", f.name, if is_lens { "show lens" } else { "command" }));
        }
    }
    for c in commands {
        if by_name.get(c).is_none_or(|f| f.family == "lens") {
            out.push(format!("`{c}` is dispatched as a command but has no CliCommand fact (D0271: every command is an authored fact)"));
        }
    }
    for l in lenses {
        if by_name.get(l).is_none_or(|f| f.family != "lens") {
            out.push(format!("`{l}` is dispatched as a show lens but has no CliCommand fact with family `lens`"));
        }
    }
    out
}

/// Guard: the CLI surface is an authored fact, held equal to the dispatch and to the help.
///
/// (D0271, issue344.) `.engine/cli/commands.sysml` is the home; `cli_facts::CLI_FACTS` mirrors it and renders
/// `keel --help`; `cli_surface::COMMAND_NAMES` / `LENS_NAMES` are the dispatch. Any of the three
/// disagreeing is a violation. An absent facts file is a violation too: a project on this engine
/// vintage ships the file, and a tree without it is a tree whose help describes nothing.
#[must_use]
pub fn cli_surface_declared(root: &Path) -> GuardReport {
    let path = root.join(".engine").join("cli").join("commands.sysml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return GuardReport {
            name: "cli-surface-declared",
            scanned: 0,
            warnings: Vec::new(),
            violations: vec![format!("{} is absent - the CLI facts (D0271) are not in this tree; `keel migrate` resyncs it", path.display())],
        };
    };
    let authored = parse_cli_facts(&text);
    let violations = cli_surface_violations(&authored, &crate::cli_facts::CLI_FACTS, &crate::cli_surface::COMMAND_NAMES, &crate::cli_surface::LENS_NAMES);
    GuardReport { name: "cli-surface-declared", scanned: authored.len(), warnings: Vec::new(), violations }
}

#[cfg(test)]
mod issue_naming_tests {
    /// issue333 / D0304: a resolver that never names its issue - and whose issue never names it - is
    /// a violation for an issue created on/after the cutoff, a counted history line before it; either
    /// direction of naming satisfies the check; the shape issue323 had (carried-over resolver, neither
    /// text mentioning the other) is what fires.
    #[test]
    fn a_resolver_that_names_neither_way_is_a_mistriage_forward_only() {
        let root = std::env::temp_dir().join(format!("keel-issuename-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".tracking")).expect("mkdir");
        let iss = |n: &str, created: &str, desc: &str| {
            format!("    part {n} : Issue {{ :>> id = \"00000000-0000-4000-8000-0000000{}\"; :>> title = \"t\"; :>> createdAt = \"{created}\"; :>> createdBy = \"bot\"; :>> description = \"{desc}\"; :>> severity = Severity::Low; }}
", &n[5..])
        };
        let text = format!(
            "package Fx {{
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    action def Build {{
        action dcNamesIt;
        verification dcNamesItDoD : Test {{ :>> id = \"00000000-0000-4000-8000-000000000001\"; :>> method = VerificationMethod::test; :>> procedureText = \"resolves issue901 by doing the thing\"; }}
        action dcCarriedOver;
        verification dcCarriedOverDoD : Test {{ :>> id = \"00000000-0000-4000-8000-000000000002\"; :>> method = VerificationMethod::test; :>> procedureText = \"an unrelated task\"; }}
        action dcNamedByIssue;
        verification dcNamedByIssueDoD : Test {{ :>> id = \"00000000-0000-4000-8000-000000000003\"; :>> method = VerificationMethod::test; :>> procedureText = \"another task\"; }}
    }}
{}{}{}{}    #Resolves dependency from dcNamesIt to issue901;
    #Resolves dependency from dcCarriedOver to issue902;
    #Resolves dependency from dcNamedByIssue to issue903;
    #Resolves dependency from dcCarriedOver to issue904;
}}
",
            iss("issue901", "2026-09-10", "the resolver names me"),
            iss("issue902", "2026-09-10", "nothing here names the resolver - the issue323 shape"),
            iss("issue903", "2026-09-10", "PREVENTING CHANGE: dcNamedByIssue"),
            iss("issue904", "2026-01-01", "old, unnamed both ways - history"),
        );
        std::fs::write(root.join(".tracking").join("fx.sysml"), text).expect("fixture");
        let (scanned, forward, historical) = crate::view::unnamed_resolutions(&root, "2026-09-04").expect("model");
        assert_eq!(scanned, 4);
        assert_eq!(forward, vec![("issue902".to_string(), "dcCarriedOver".to_string())], "only the issue323 shape, forward of the cutoff, is a violation");
        assert_eq!(historical, 1, "the pre-cutoff miss is counted, not reported");
        let _ = std::fs::remove_dir_all(&root);
    }
}

#[cfg(test)]
mod guard_catalogue_tests {
    /// D0195 clause 1 / issue370: EVERY enforced guard has a constraint-def identity in
    /// `.engine/rules/guard-constraints.sysml` (its camelCased name). Twelve did not, for weeks, while
    /// the file said all did - because nothing computed the two lists against each other.
    #[test]
    fn every_guard_has_a_constraint_def_identity() {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../.engine/rules/guard-constraints.sysml")).expect("guard-constraints.sysml");
        let declared: std::collections::HashSet<String> = text
            .lines()
            .filter_map(|l| l.trim().strip_prefix("constraint def "))
            .map(|r| r.split(|c: char| c == ';' || c.is_whitespace()).next().unwrap_or_default().to_string())
            .collect();
        let camel = |k: &str| {
            let mut out = String::new();
            for (i, part) in k.split('-').enumerate() {
                if i == 0 {
                    out.push_str(part);
                } else if let Some(c) = part.chars().next() {
                    out.push(c.to_ascii_uppercase());
                    out.push_str(&part[c.len_utf8()..]);
                }
            }
            out
        };
        let missing: Vec<&str> = super::GUARD_NAMES.iter().copied().filter(|g| !declared.contains(&camel(g))).collect();
        assert!(missing.is_empty(), "guards with no constraint-def identity (D0195 clause 1): {missing:?}");
    }

    /// THE CONTROL for the catalogue: every enforced guard has a row in `.engine/docs/guards.md`. Six
    /// guards had none on 2026-09-02 - five of them older than a week - because `doc-guard-count` polices
    /// the COUNT claim and nothing policed the rows. A guard nobody can look up is a refusal nobody can
    /// act on.
    #[test]
    fn every_guard_has_a_catalogue_row() {
        let md = std::fs::read_to_string("../.engine/docs/guards.md").expect("guards.md ships with the engine");
        let missing: Vec<&str> = super::GUARD_NAMES.iter().copied().filter(|n| !md.contains(&format!("| `{n}` |"))).collect();
        assert!(missing.is_empty(), "guards with no row in .engine/docs/guards.md: {missing:?}");
    }
}

#[cfg(test)]
mod cli_surface_declared_tests {
    use super::*;

    fn fact(name: &str, family: &str, effect: &str) -> String {
        format!(":>> name = \"{name}\"; :>> family = \"{family}\"; :>> effect = CliEffect::{effect}; :>> stability = CliStability::stable; :>> synopsis = \"s\";")
    }
    fn line(name: &str, family: &str, effect: &str) -> String {
        format!("    part x : CliCommand {{ :>> id = \"i\"; {} }}", fact(name, family, effect))
    }
    fn mirror(name: &'static str, family: &'static str, effect: &'static str) -> crate::cli_facts::CliFact {
        crate::cli_facts::CliFact { name, family, effect, stability: "stable", invocation: "", synopsis: "s" }
    }

    #[test]
    fn a_dispatched_command_with_no_fact_is_a_violation() {
        let authored = parse_cli_facts(&line("orient", "orientation", "reads"));
        let m = [mirror("orient", "orientation", "reads")];
        let v = cli_surface_violations(&authored, &m, &["orient", "ghost"], &[]);
        assert_eq!(v.len(), 1, "{v:?}");
        assert!(v[0].contains("`ghost` is dispatched"), "{v:?}");
    }

    #[test]
    fn a_fact_nothing_dispatches_is_a_violation_and_so_is_a_lens_declared_as_a_command() {
        let text = format!("{}\n{}", line("orient", "orientation", "reads"), line("suspect", "orientation", "reads"));
        let authored = parse_cli_facts(&text);
        let m = [mirror("orient", "orientation", "reads"), mirror("suspect", "orientation", "reads")];
        let v = cli_surface_violations(&authored, &m, &["orient"], &["suspect"]);
        // `suspect` is a lens in the dispatch but declared as a command: undispatched as a command AND
        // the lens has no `lens` fact - two violations, both true.
        assert_eq!(v.len(), 2, "{v:?}");
    }

    #[test]
    fn a_mirror_that_drifts_from_the_facts_is_caught_field_by_field() {
        let authored = parse_cli_facts(&line("status", "orientation", "reads"));
        let m = [mirror("status", "orientation", "writes")];
        let v = cli_surface_violations(&authored, &m, &["status"], &[]);
        assert_eq!(v.len(), 1, "{v:?}");
        assert!(v[0].contains("effect differs") && v[0].contains("`reads`") && v[0].contains("`writes`"), "{v:?}");
    }

    #[test]
    fn the_live_facts_mirror_and_dispatch_agree() {
        let text = std::fs::read_to_string("../.engine/cli/commands.sysml").expect("the facts ship with the engine");
        let authored = parse_cli_facts(&text);
        assert_eq!(authored.len(), crate::cli_facts::CLI_FACTS.len(), "every fact parsed");
        let v = cli_surface_violations(&authored, &crate::cli_facts::CLI_FACTS, &crate::cli_surface::COMMAND_NAMES, &crate::cli_surface::LENS_NAMES);
        assert!(v.is_empty(), "{v:#?}");
    }
}

#[cfg(test)]
mod retro_tie_tests {
    use super::{named_items, retro_backlog_violations_for_test, retro_texts};

    /// A sprint file whose RETRO gate carries `finding`. The `DoD` line names the delivered task — as
    /// every real sprint file does — which is exactly what defeated the previous shape of this guard.
    fn sprint_with_retro(finding: &str) -> String {
        format!(
            "package S {{
             verification storyXDoD : Test {{ :>> method = VerificationMethod::test; :>> procedureText = \"DELIVERED BACKLOG ITEMS: dcDeliveredThing.\"; }}
             verification xRetroGate : Test {{ :>> title = \"Sprint X retro gate\"; :>> method = VerificationMethod::analyze; :>> procedureText = \"{finding}\"; }}
             }}
"
        )
    }

    /// THE HOLE THAT LET SIX FINDINGS THROUGH IN ONE DAY (issue335). The retro says FINDING, not
    /// LESSON — so the old vocabulary gate never examined it — and the file names the delivered task,
    /// so the old naming check was satisfied by construction. Both were wrong; this must FAIL.
    #[test]
    fn a_finding_in_any_words_with_no_new_item_is_a_violation() {
        let text = sprint_with_retro("TWO FINDINGS. (1) the guard checks that an edge EXISTS, not that the resolver fits. (2) the same per-instance repair recurred.");
        let v = retro_backlog_violations_for_test(&[], &[], &[("s.sysml".to_string(), text)]);
        assert_eq!(v.len(), 1, "a retro with findings and no NEW tracked item must be a violation: {v:?}");
        assert!(v[0].contains("names no item at all"), "{v:?}");
    }

    /// issue189's hole, kept as a regression: naming the DELIVERED task is what every retro does and
    /// tracks nothing. Only an item this commit ADDS counts.
    #[test]
    fn naming_the_delivered_task_does_not_track_a_finding() {
        let text = sprint_with_retro("FINDING: the counter was wrong. This sprint delivered dcDeliveredThing, which is unrelated.");
        let v = retro_backlog_violations_for_test(&[], &[], &[("s.sysml".to_string(), text)]);
        assert_eq!(v.len(), 1, "an EXISTING task's name must not satisfy the check: {v:?}");
        assert!(v[0].contains("dcDeliveredThing") && v[0].contains("none of which this commit adds"), "{v:?}");
    }

    /// The satisfying condition: the retro names an item and THIS COMMIT adds it.
    #[test]
    fn a_finding_whose_named_item_this_commit_adds_is_clean() {
        let text = sprint_with_retro("FINDING: the counter was wrong; recorded as issue188 with a resolver.");
        let added = vec!["issue188".to_string()];
        assert!(retro_backlog_violations_for_test(&added, &[], &[("s.sysml".to_string(), text)]).is_empty());
        let text = sprint_with_retro("LESSON: shell mangling again - now tracked as dcAuthorViaWriteTool.");
        let added = vec!["dcAuthorViaWriteTool".to_string()];
        assert!(retro_backlog_violations_for_test(&added, &[], &[("s.sysml".to_string(), text)]).is_empty());
    }

    /// The obligation is a STATED choice, not always-an-item: an explicit justification is clean.
    #[test]
    fn an_explicit_no_item_justification_is_clean_when_it_names_the_item_that_tracks_it() {
        // Naming an EXISTING item (here a known task) beside the justification is what makes the claim
        // checkable; the phrase alone is no longer enough (issue364, D0293).
        let text = sprint_with_retro("FINDING: a one-off typo; no new item - dcPostEditGate already catches this class.");
        let known = vec!["dcPostEditGate".to_string()];
        assert!(retro_backlog_violations_for_test(&[], &known, &[("s.sysml".to_string(), text)]).is_empty());
        // ...and a Decision counts as the tracking item too.
        let text = sprint_with_retro("no new item - already tracked: the stale help is recorded in d0283.");
        let known = vec!["d0283".to_string()];
        assert!(retro_backlog_violations_for_test(&[], &known, &[("s.sysml".to_string(), text)]).is_empty());
    }

    /// issue364, second shape: a retro whose text contains the word "verification" was cut in half by the
    /// old extraction and never examined - the guard passed a retro it had not read.
    #[test]
    fn a_retro_that_mentions_verification_is_still_examined() {
        let text = sprint_with_retro("FINDING: the verification of X was skipped; no item named anywhere here.");
        assert_eq!(retro_texts(&text).len(), 1, "the retro must be extracted whole: {:?}", retro_texts(&text));
        let v = retro_backlog_violations_for_test(&[], &[], &[("s.sysml".to_string(), text)]);
        assert_eq!(v.len(), 1, "and examined: {v:?}");
    }

    /// THE CONTROL for issue364: 'already tracked' must point at something real.
    #[test]
    fn already_tracked_with_no_named_item_is_a_violation() {
        let text = sprint_with_retro("no new item - already tracked: the finding belongs to the verification revamp.");
        let v = retro_backlog_violations_for_test(&[], &["dcVerificationByAuthority".to_string()], &[("s.sysml".to_string(), text)]);
        assert_eq!(v.len(), 1, "{v:?}");
        assert!(v[0].contains("'already tracked'") && v[0].contains("names no item at all"), "{v:?}");
    }

    /// ...and naming an item that does NOT exist is the same violation, said differently.
    #[test]
    fn already_tracked_naming_a_nonexistent_item_is_a_violation() {
        let text = sprint_with_retro("no new item - already tracked in issue999, which covers it.");
        let v = retro_backlog_violations_for_test(&[], &["issue001".to_string()], &[("s.sysml".to_string(), text)]);
        assert_eq!(v.len(), 1, "{v:?}");
        assert!(v[0].contains("issue999") && v[0].contains("none of which exists"), "{v:?}");
    }

    /// The two retros that motivated the fix, re-scanned: sprint 526's named a real item (issue336) and
    /// so passes the FORM check - the guard cannot judge that issue336 does not cover the finding, and
    /// says so in its doc; sprint 527's named dcVerificationByAuthority and passes likewise. What both
    /// would have FAILED is the shape they took in the human's reading: a claim with no item at all.
    #[test]
    fn a_justification_that_names_a_real_item_passes_form_and_leaves_relevance_to_the_reader() {
        let text = sprint_with_retro("no new item - already tracked: that gap belongs to issue336.");
        assert!(retro_backlog_violations_for_test(&[], &["issue336".to_string()], &[("s.sysml".to_string(), text)]).is_empty());
    }

    /// Only the RETRO gate is examined — a `DoD` or review gate mentioning a finding-like word is not a
    /// findings record, and a sprint with no retro gate yields nothing to check.
    #[test]
    fn only_retro_gates_are_examined() {
        let no_retro = "package S {
verification storyXDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = \"FINDING: not a retro.\"; }
}
";
        assert!(retro_texts(no_retro).is_empty());
        assert!(retro_backlog_violations_for_test(&[], &[], &[("s.sysml".to_string(), no_retro.to_string())]).is_empty());
    }

    /// Word boundaries on the item tokens, as before.
    #[test]
    fn prose_lookalikes_do_not_count_as_items() {
        assert!(named_items("the dc motor issue was discussed at length").is_empty());
        assert!(named_items("reproduced changes").is_empty());
        assert_eq!(named_items("tracked as dcFooBar"), vec!["dcFooBar"]);
        assert_eq!(named_items("see issue123"), vec!["issue123"]);
        assert!(named_items("tissue42 is not an item").is_empty());
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
        const LEGITIMATELY_EMPTY: [&str; 15] = [
            // scans priority-ordering pairs on the ready frontier; D0189's scope closure emptied
            // the frontier down to a handful of same-rank resolvers, so there is nothing to order.
            // The population returns the moment the backlog holds ranked work again.
            "priority-inversion",
            // SHALLOW-CLONE dependent: claim-ancestry skips loudly when history is unavailable, which
            // is correct (a depth-dependent verdict is the K15 machine-dependence it exists to
            // prevent) but means its population is zero in any job that checks out shallow. Guard 51
            // now refuses a GATING workflow that does so; this test runs in jobs that may not be one.
            "claim-ancestry",
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
            "decision-amends-process",   // scans STAGED Decisions - zero between commits
            "unit-extras-present",       // scans declared unit extras; no installed unit declares any since the channel left (D0292)
            "control-defect-registry",   // scans registered control defects - EMPTY since 2026-09-04, when the last of D0278's three left with D0303 C; a registered defect is the unhealthy state
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
    fn a_receipt_is_demanded_of_the_ai_and_never_of_the_human() {
        // D0232. The exemption is the DESIGN, not a courtesy: governance binds the AI, and a human's
        // word IS the evidence. Tested in four directions, because a guard that only ever passes is
        // indistinguishable from one aimed at nothing, and one that fires on the human would point
        // the whole control at the wrong party.
        let root = std::env::temp_dir().join("keel-evidence-cited-test");
        let _ = std::fs::remove_dir_all(&root);
        let tr = root.join(".tracking");
        std::fs::create_dir_all(&tr).unwrap();
        std::fs::write(
            tr.join("actors.sysml"),
            "package A {\n    part wweatherholtz : Person { :>> name = \"W\"; }\n    part bot : Actor { :>> kind = ActorKind::ai; }\n}\n",
        )
        .unwrap();

        let res = |name: &str, by: &str, at: &str, ran: bool| {
            let receipt = if ran { "        // RAN: cargo test -> 3 passed\n" } else { "" };
            format!("{receipt}        part {name}R1 : TestResult {{ :>> id = \"x\"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = \"abc\"; :>> judgedAt = \"{at}\"; :>> judgedBy = \"{by}\"; }}\n")
        };
        let write = |body: &str| {
            std::fs::write(
                tr.join("s.sysml"),
                format!(
                    "package S {{\n    action def R {{\n        action t;\n        verification tDoD : Test {{ :>> id = \"y\"; :>> method = VerificationMethod::test; }}\n{body}    }}\n}}\n"
                ),
            )
            .unwrap();
        };

        // 1. AI, method=test, NO receipt -> refused.
        write(&res("tDoD", "bot", "2026-08-25", false));
        let r = process_and_report(&root);
        assert_eq!((r.scanned, r.violations.len()), (1, 1), "an AI test-claim with no receipt must be refused");

        // 2. AI, WITH a receipt -> clean.
        write(&res("tDoD", "bot", "2026-08-25", true));
        let r = process_and_report(&root);
        assert_eq!((r.scanned, r.violations.len()), (1, 0), "a receipt satisfies it");

        // 3. HUMAN, no receipt -> NOT EVEN SCANNED. Their word is the evidence.
        write(&res("tDoD", "wweatherholtz", "2026-08-25", false));
        let r = process_and_report(&root);
        assert_eq!((r.scanned, r.violations.len()), (0, 0), "a human's attestation is out of scope entirely");

        // 4. AI, but dated BEFORE the cutover -> out of scope; retro-fitting evidence nobody
        //    captured would mean inventing it, which is the failure this guard exists to prevent.
        write(&res("tDoD", "bot", "2026-08-01", false));
        let r = process_and_report(&root);
        assert_eq!((r.scanned, r.violations.len()), (0, 0), "the guard binds forward only");
        let _ = std::fs::remove_dir_all(&root);
    }

    fn process_and_report(root: &std::path::Path) -> GuardReport {
        super::evidence_cited(root)
    }

    #[test]
    fn a_process_that_cannot_say_when_it_applies_is_refused() {
        // D0225. Both directions, because a guard that only ever passes is indistinguishable from a
        // guard that does nothing - and this codebase has already shipped two checks that passed on
        // an empty population (issue250, claude-surface-drift on zero skills).
        let root = std::env::temp_dir().join("keel-guard-applicability");
        let _ = std::fs::remove_dir_all(&root);
        let dir = root.join(".engine").join("processes");
        std::fs::create_dir_all(&dir).unwrap();

        // Out of scope until the project adopts project-onboarding (issue259): the fact serves that
        // process, and a project that never adopted it has violated nothing (D0164).
        std::fs::write(dir.join("other.sysml"), "package O {
    action o : Process {
        :>> id = \"y\";
    }
}
").unwrap();
        assert_eq!(process_applicability(&root).scanned, 0, "not adopted -> out of scope, and it says 0 scanned");
        std::fs::write(dir.join("project-onboarding.sysml"), "package PO {
    action po : Process {
        :>> id = \"z\";
        // APPLIES-WHEN: adopted
    }
}
").unwrap();
        std::fs::remove_file(dir.join("other.sysml")).unwrap();

        let declares = "package P {
    action p : Process {
        :>> id = \"x\";
";
        std::fs::write(dir.join("with.sysml"), format!("{declares}        // APPLIES-WHEN: a stated situation
    }}
}}
")).unwrap();
        let r = process_applicability(&root);
        assert_eq!((r.scanned, r.violations.len()), (2, 0), "both declared conditions pass");

        std::fs::write(dir.join("without.sysml"), format!("{declares}    }}
}}
")).unwrap();
        let r = process_applicability(&root);
        assert_eq!(r.scanned, 3, "every process file is in scope once adopted");
        assert_eq!(r.violations.len(), 1, "the one lacking a condition is refused");
        assert!(r.violations[0].contains("without.sysml"), "{:?}", r.violations);

        // A file that declares no Process at all is out of scope, not a violation.
        std::fs::write(dir.join("helper.sysml"), "package H {
    private import X::*;
}
").unwrap();
        assert_eq!(process_applicability(&root).scanned, 3, "a non-Process file is not scanned");
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

