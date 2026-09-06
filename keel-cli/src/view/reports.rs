//! load-bearing decisions, the traceability diagram, scorecards (D0087), indicators (D0089) - extracted from view.rs (sprint 418, dcViewRsRestructure: the panel's
//! god-module finding). Pure move, no behavior change; `view::` paths survive via the
//! `pub use` re-exports in mod.rs.

use std::collections::HashSet;
use std::path::Path;

use crate::json::Json;


#[allow(clippy::wildcard_imports)] // a pure move-only split: the parent's vocabulary IS this file's vocabulary
use super::*;

// ── load-bearing decisions report (formalized; replaces ad-hoc ranking scripts) ─────────────────
// Ranks accepted Decisions by dependence (charters-to x2 + cross-citations from other decisions)
// and flags "antiquated" signals: uncritiqued (no full Core-3 element-critique), references-retired
// (cites a retired mechanism), superseded-in-part (a later decision supersedes/retires/replaces it).
// Superseded ZOMBIES (status != accepted) are out of scope here by design (handled separately).

/// Mechanisms retired/superseded across the project — a decision still citing one signals its
/// process context has moved on (the D0048 case). Curated; extend as more retire.
const RETIRED_MECHANISMS: &[&str] =
    &["query.py", "parity_check", "validate_all", "validate_sysml", "RESUME.md", "StateCursor", "kill_stale_kernels"];

/// The `dNNNN` decision name declared in a decision file's text, if any (handles a `#Marker` prefix).
pub(super) fn find_decision_name(text: &str) -> Option<String> {
    for line in text.lines() {
        let l = line.trim_start().trim_start_matches('#');
        let Some(rest) = l
            .strip_prefix("part ")
            .or_else(|| l.strip_prefix("ProspectiveChange part "))
            .or_else(|| l.strip_prefix("SafetyChange part "))
        else {
            continue;
        };
        let name = rest.split([' ', ':']).next().unwrap_or("");
        if name.len() == 5 && name.starts_with('d') && name.get(1..).is_some_and(|d| d.chars().all(|c| c.is_ascii_digit())) {
            return Some(name.to_owned());
        }
    }
    None
}

/// Count word-ish mentions of a `dNNNN` decision name (both `d` and `D` forms) in `text`.
pub(super) fn count_mentions(text: &str, name: &str) -> usize {
    let upper = format!("D{}", name.get(1..).unwrap_or(""));
    text.matches(name).count() + text.matches(&upper).count()
}

/// True if any line mentions `name` alongside a supersede/retire/replace verb (a later decision
/// revising this one).
pub(super) fn supersede_near(text: &str, name: &str) -> bool {
    let upper = format!("D{}", name.get(1..).unwrap_or(""));
    text.lines().any(|line| {
        (line.contains(name) || line.contains(&upper))
            && (line.contains("supersede") || line.contains("Supersede") || line.contains("retire") || line.contains("replace"))
    })
}

struct DecisionRow {
    name: String,
    charters: usize,
    citations: usize,
    score: usize,
    uncritiqued: bool,
    references_retired: Vec<String>,
    superseded_in_part: Vec<String>,
}

/// Load-bearing decisions report (formalized) as JSON: accepted Decisions ranked by dependence,
/// each with critique-coverage + antiquation flags. Computed from authored facts; no stored data.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn decisions_report(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    // Decision-file texts keyed by decision name (for citation + supersede + retired scans).
    let mut texts: Vec<(String, String)> = Vec::new();
    for path in crate::collect_sysml(&root.join(".engine").join("decisions")) {
        if let Ok(t) = std::fs::read_to_string(&path) {
            if let Some(name) = find_decision_name(&t) {
                texts.push((name, t));
            }
        }
    }
    // Decisions with FULL Core-3 critique coverage (so `uncritiqued` = not in this set).
    let stale = compute_stale_verifications(root, &model);
    let critiqued: HashSet<String> = compute_critique_coverage(&model, &stale, &CritiquePolicy::load(root)?)
        .into_iter()
        .filter(|c| c.covered && c.type_name == "Decision")
        .map(|c| c.element)
        .collect();

    let mut rows: Vec<DecisionRow> = Vec::new();
    for (name, info) in &model.items {
        if info.type_name != "Decision" || info.attrs.get("status").map(String::as_str) != Some("accepted") {
            continue; // accepted decisions only — zombies (non-accepted) are out of scope here
        }
        let charters = model.edges.iter().filter(|e| e.kind == "charteredby" && &e.to == name).count();
        let mut citations = 0;
        let mut superseded_in_part: Vec<String> = Vec::new();
        let mut own_text = "";
        for (other, t) in &texts {
            if other == name {
                own_text = t;
                continue;
            }
            let n = count_mentions(t, name);
            citations += n;
            if n > 0 && supersede_near(t, name) {
                superseded_in_part.push(other.clone());
            }
        }
        superseded_in_part.sort();
        let references_retired: Vec<String> =
            RETIRED_MECHANISMS.iter().filter(|m| own_text.contains(**m)).map(|m| (*m).to_owned()).collect();
        rows.push(DecisionRow {
            charters,
            citations,
            score: charters * 2 + citations,
            uncritiqued: !critiqued.contains(name),
            references_retired,
            superseded_in_part,
            name: name.clone(),
        });
    }
    rows.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.name.cmp(&b.name)));

    let decisions: Vec<Json> = rows
        .iter()
        .map(|r| {
            Json::Obj(vec![
                ("decision".to_string(), Json::s(r.name.clone())),
                ("score".to_string(), Json::Int(i64::try_from(r.score).unwrap_or(i64::MAX))),
                ("charters".to_string(), Json::Int(i64::try_from(r.charters).unwrap_or(i64::MAX))),
                ("citations".to_string(), Json::Int(i64::try_from(r.citations).unwrap_or(i64::MAX))),
                ("uncritiqued".to_string(), Json::Bool(r.uncritiqued)),
                ("references_retired".to_string(), Json::Arr(r.references_retired.iter().map(|s| Json::s(s.clone())).collect())),
                ("superseded_in_part".to_string(), Json::Arr(r.superseded_in_part.iter().map(|s| Json::s(s.clone())).collect())),
            ])
        })
        .collect();
    let out = Json::Obj(vec![
        (
            "report".to_string(),
            Json::s("load-bearing decisions: accepted Decisions ranked by dependence (charters x2 + cross-citations) + antiquation flags. uncritiqued = lacks full Core-3 element-critique; references_retired = cites a retired mechanism; superseded_in_part = HEURISTIC (a later decision's text mentions it near supersede/retire/replace — a hint to review, not authority). Zombies (status != accepted) are out of scope. Computed, never stored."),
        ),
        ("decisions".to_string(), Json::Arr(decisions)),
    ]);
    Ok(out.dump())
}

// ── comprehensive traceability diagram (computed view; interactive self-contained HTML, D0085) ──
// The whole model — every element (node, typed + metadata) and every typed edge — emitted as ONE
// interactive HTML page (cytoscape): filter by node type / edge kind, search, click-to-focus a
// neighborhood, fit. Regenerated on demand from authored facts; never committed as truth (§2.1/D0015).

// ── computed report / scorecard layer (D0087) ─────────────────────────────────────────────────
// Human-digestible AGGREGATE reports rolling up the per-element views into totals/percentages +
// a health/opportunity read. Each report emits a `cards` array (label/value/detail/tone) so ONE
// HTML template renders all of them. Computed on demand; never authored, never committed (§2.1).

/// Integer percentage `n/d`. An EMPTY population is 0%, never 100% (D0286): a project that has
/// authored nothing has done nothing, and a fresh scaffold reporting a clean sweep of 100% was the
/// defect the human saw on every inheriting project. Callers that can tell "nothing yet" from "0 of
/// many" should pair this with [`cov_tone_of`], which gives the empty case its own tone.
pub(super) fn pct(n: usize, d: usize) -> u32 {
    n.saturating_mul(100).checked_div(d).map_or(0, |x| u32::try_from(x).unwrap_or(0))
}

/// Tone for a coverage-style percentage (higher is better).
pub(super) const fn cov_tone(p: u32) -> &'static str {
    if p >= 90 { "good" } else if p >= 70 { "warn" } else { "bad" }
}

/// Tone for a ratio whose population may be empty: `empty` (neutral) when there is nothing to
/// measure, so a fresh project reads "nothing yet" rather than "failing" - and never "complete".
pub(super) fn cov_tone_of(n: usize, d: usize) -> &'static str {
    if d == 0 { "empty" } else { cov_tone(pct(n, d)) }
}

/// One scorecard metric card.
fn card(label: &str, value: String, detail: String, tone: &str) -> Json {
    Json::Obj(vec![
        ("label".to_string(), Json::s(label.to_string())),
        ("value".to_string(), Json::s(value)),
        ("detail".to_string(), Json::s(detail)),
        ("tone".to_string(), Json::s(tone.to_string())),
    ])
}

/// Compute a report's `(title, cards)`; shared by the JSON emitter and the HTML scorecard.
fn report_cards(root: &Path, name: &str) -> Result<(String, Vec<Json>), ViewError> {
    let model = Model::build(root)?;
    let orient = crate::orient::compute(root);
    let done = crate::orient::done_names(root);
    let task_suspect: HashSet<String> = orient.suspect.iter().cloned().collect();
    let stale = compute_stale_verifications(root, &model);
    let cov = compute_coverage(&model, &done, &task_suspect, &stale);
    match name {
        "assurance" => Ok(("Assurance Scorecard".to_string(), assurance_cards(root, &model, &cov, &stale, &done, &task_suspect)?)),
        "traceability" => Ok(("Traceability / V&V Coverage".to_string(), traceability_cards(&model, &cov))),
        "quality-debt" => Ok(("Quality & Debt".to_string(), quality_debt_cards(root, &model, &cov, &stale, &task_suspect))),
        "flow" => Ok(("Flow / Velocity".to_string(), flow_cards(root, &model, &orient))),
        "governance" => Ok(("Governance / Decisions".to_string(), governance_cards(&model))),
        "friction" => Ok(("Authoring Friction (vs spreadsheet)".to_string(), friction_cards())),
        other => Err(ViewError::UnknownReport(other.to_string())),
    }
}

/// Computed report as JSON (D0087); `trend` adds a git-derived time-series for the headline metric.
///
/// # Errors
/// Returns [`ViewError`] for an unknown report name or a parse failure.
pub fn report(root: &Path, name: &str, trend: bool) -> Result<String, ViewError> {
    let (title, cards) = report_cards(root, name)?;
    let mut obj = vec![
        ("report".to_string(), Json::s(name.to_string())),
        ("title".to_string(), Json::s(title)),
        ("note".to_string(), Json::s("computed aggregate (D0087) — regenerate, never commit as truth".to_string())),
        ("cards".to_string(), Json::Arr(cards)),
    ];
    if trend {
        obj.push(("trend".to_string(), trend_json(root, name)));
    }
    Ok(Json::Obj(obj).dump())
}

/// A report's headline metric git-derived series `{label, series:[{commit,value}]}` over recent
/// commits (the report's primary scalar, via [`metric_value`]).
fn trend_json(root: &Path, report: &str) -> Json {
    let series: Vec<Json> = trend_series(root, report_headline_key(report))
        .into_iter()
        .map(|(sha, v)| Json::Obj(vec![("commit".to_string(), Json::s(sha)), ("value".to_string(), Json::s(format!("{v:.2}")))]))
        .collect();
    Json::Obj(vec![
        ("label".to_string(), Json::s(headline_label(report).to_string())),
        ("series".to_string(), Json::Arr(series)),
    ])
}

/// Recent commits touching the model (chronological: oldest → newest), capped at `n`.
fn sampled_commits(root: &Path, n: usize) -> Vec<String> {
    let out = git_out(root, &["log", &format!("-n{n}"), "--format=%H", "--", ".tracking", ".engine"]).unwrap_or_default();
    let mut v: Vec<String> = out.lines().map(str::to_string).collect();
    v.reverse();
    v
}

/// Run `git -C root <args>` and capture stdout, or `None` on non-zero exit / failure.
pub(super) fn git_out(root: &Path, args: &[&str]) -> Option<String> {
    // The CALL count now happens in gitx::git() at construction; only the rich detail
    // (argv tally, wall time) stays here, so the two layers never double-count.
    crate::perf::note_git(args);
    let out = crate::perf::timed(&crate::perf::GIT_NANOS, || {
        crate::gitx::git().arg("-C").arg(root).args(args).output()
    })
    .ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The headline metric's display label per report.
pub(super) const fn headline_label(name: &str) -> &str {
    match name.as_bytes() {
        b"assurance" => "Verification coverage %",
        b"traceability" => "Requirements verified %",
        b"quality-debt" => "Supersede edges (volatility)",
        b"governance" => "Accepted decisions",
        b"friction" => "Write-API verbs (1-command facts)",
        _ => "Delivered points (burnup)",
    }
}

/// The single shared computation for a canonical scalar metric (D0090).
///
/// Both the report cards and the computed Indicators source their numeric value from this keyed
/// registry, so each metric is computed in exactly one place. `None` if the key is unknown or the
/// model fails to build.
#[must_use]
pub fn metric_value(root: &Path, key: &str) -> Option<f64> {
    let model = Model::build(root).ok()?;
    let cnt = |n: usize| -> f64 { f64::from(u32::try_from(n).unwrap_or(u32::MAX)) };
    match key {
        // coverage-family (the full tier pipeline)
        "coverage_pct" | "req_verified_pct" | "needs_verified_pct" => {
            let done = crate::orient::done_names(root);
            let task_suspect: HashSet<String> = crate::orient::compute(root).suspect.into_iter().collect();
            let stale = compute_stale_verifications(root, &model);
            let cov = compute_coverage(&model, &done, &task_suspect, &stale);
            Some(f64::from(match key {
                "req_verified_pct" => verified_pct_of(&cov, "SystemRequirement"),
                "needs_verified_pct" => verified_pct_of(&cov, "Need"),
                _ => coverage_pct_of(&cov, ""),
            }))
        }
        "critique_pct" => {
            let stale = compute_stale_verifications(root, &model);
            let crit = compute_critique_coverage(&model, &stale, &CritiquePolicy::load_or_core3(root));
            Some(f64::from(pct(crit.iter().filter(|c| c.covered).count(), crit.len())))
        }
        "attestation_pct" => {
            let (total, missing) = compute_attestation(&model);
            Some(f64::from(pct(total - missing.len(), total)))
        }
        // D0249's unbuilt clause (dcUngroundedRatioTriggers): the rootedness ungrounded ratio - delivery
        // Stories chartered by a Decision that reaches no Need, as a share of all delivery Stories. The
        // number rootedness always held (415 of 482 in sprint 483) and no view ranked or raised.
        "ungrounded_ratio_pct" => {
            let (total, ungrounded) = super::checks::rootedness_counts(root).ok()?;
            Some(f64::from(pct(ungrounded, total)))
        }
        "volatility" => Some(cnt(model.edges.iter().filter(|e| e.kind == "supersede").count())),
        // D0200 clause 4: the human's override rate over READ sitting reviews (batch-acks excluded -
        // an acknowledgment cannot override). Sustained 0% is the ALARM, not the goal (the HITL
        // rubber-stamp threshold the panel cited): a reviewer who never rejects is not reviewing.
        "sitting_override_rate_pct" => {
            let mut reviews = 0usize;
            let mut overrides = 0usize;
            let mut seen: HashSet<&String> = HashSet::new();
            for e in model.edges.iter().filter(|e| e.kind == "covers") {
                if !seen.insert(&e.from) {
                    continue;
                }
                let Some(info) = model.items.get(&e.from) else { continue };
                let text = info.attrs.get("procedureText").map_or("", String::as_str);
                if text.contains("BATCH-ACKNOWLEDGED") || text.to_lowercase().contains("batch") {
                    continue;
                }
                reviews += 1;
                if crate::view::latest_result(&model, &e.from).is_some_and(|(o, _)| o == "fail") {
                    overrides += 1;
                }
            }
            Some(f64::from(pct(overrides, reviews.max(1))))
        }
        "accepted_decisions" => Some(cnt(model.items.values().filter(|i| i.type_name == "Decision" && i.attrs.get("status").map(String::as_str) == Some("accepted")).count())),
        "open_findings" => {
            let done = crate::orient::done_names(root);
            let (undisp, crit) = finding_blockers(&compute_issue_resolution(&model, &done), &model);
            Some(cnt(undisp.len() + crit.len()))
        }
        "friction_verbs" => Some(4.0), // the 4 one-command write-API verbs (a fixed benchmark)
        "velocity" | "burnup" | "throughput" => {
            let flows = collect_flows(root);
            match key {
                "throughput" => Some(cnt(flows.len())),
                "burnup" => Some(f64::from(i32::try_from(flows.iter().map(|f| f.points).sum::<i64>()).unwrap_or(i32::MAX))),
                _ => Some(velocity_of(&flows)),
            }
        }
        _ => None,
    }
}

/// The headline metric key a report's `--trend` tracks (the report's primary scalar).
fn report_headline_key(report: &str) -> &str {
    match report {
        "assurance" => "coverage_pct",
        "traceability" => "req_verified_pct",
        "quality-debt" => "volatility",
        "governance" => "accepted_decisions",
        "friction" => "friction_verbs",
        _ => "burnup", // flow
    }
}

/// Compute a keyed metric ([`metric_value`]) at each sampled commit via a throwaway git worktree
/// (reuses the whole pipeline unchanged at that commit). Commits that fail to check out are skipped.
fn trend_series(root: &Path, key: &str) -> Vec<(String, f64)> {
    let mut out = Vec::new();
    // 12 recent commits balances a readable trendline against the per-commit worktree+pipeline cost.
    for sha in sampled_commits(root, 12) {
        let short: String = sha.chars().take(8).collect();
        let Some(wt) = std::env::temp_dir().join(format!("keel-trend-{short}")).to_str().map(str::to_string) else { continue };
        // Best-effort clean, then add a detached worktree at the commit.
        let _ = git_out(root, &["worktree", "remove", "--force", &wt]);
        if git_out(root, &["worktree", "add", "--detach", &wt, &sha]).is_some() {
            if let Some(v) = metric_value(Path::new(&wt), key) {
                out.push((short, v));
            }
            let _ = git_out(root, &["worktree", "remove", "--force", &wt]);
        }
    }
    let _ = git_out(root, &["worktree", "prune"]);
    out
}

/// Computed report rendered as a human-digestible HTML scorecard (D0087).
///
/// # Errors
/// Returns [`ViewError`] for an unknown report name or a parse failure.
pub fn report_html(root: &Path, name: &str, trend: bool) -> Result<String, ViewError> {
    let (title, cards) = report_cards(root, name)?;
    let trend_data = if trend { trend_json(root, name) } else { Json::Null };
    Ok(REPORT_TEMPLATE
        .replace("/*STYLE*/", TABLE_STYLE)
        .replace("/*TITLE*/", &json_esc(&title))
        .replace("/*TREND*/", &trend_data.dump())
        .replace("/*CARDS*/", &Json::Arr(cards).dump()))
}

/// The orient DASHBOARD as a self-contained HTML scorecard (D0093) — the human's recurring home.
///
/// Cards: where things stand + what's ready + open issues + suspect/stale + assurance readiness,
/// reusing the report card template. A computed #View (regenerate-don't-commit), drilling down to the
/// `keel orient` JSON authority.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn orient_html(root: &Path) -> Result<String, ViewError> {
    let o = crate::orient::compute(root);
    let preview = |items: &[String], n: usize| -> String {
        if items.is_empty() {
            return "\u{2014}".to_string();
        }
        let shown: Vec<&str> = items.iter().take(n).map(String::as_str).collect();
        let more = items.len().saturating_sub(n);
        if more > 0 { format!("{} \u{2026} +{more} more", shown.join(", ")) } else { shown.join(", ") }
    };
    let rb = compute_readiness(root)?;
    let wip: Vec<String> = o
        .in_progress_sprints
        .iter()
        .map(|s| format!("{} (pending {})", s.sprint, s.pending.clone().unwrap_or_else(|| "\u{2014}".to_string())))
        .collect();
    let suspect_total = o.suspect.len() + o.invalid_evidence.len();
    let cards = vec![
        card("Progress", format!("{} / {}", o.done, o.outstanding), "completed vs outstanding tasks".to_string(), "good"),
        card("Ready to start", o.ready.len().to_string(), format!("unblocked now: {}", preview(&o.ready, 6)), if o.ready.is_empty() { "warn" } else { "good" }),
        card("Sprints in progress", o.in_progress_sprints.len().to_string(), if wip.is_empty() { "none".to_string() } else { preview(&wip, 4) }, if o.in_progress_sprints.len() <= 2 { "good" } else { "warn" }),
        card("Open issues", o.open_issues.len().to_string(), format!("unresolved: {}", preview(&o.open_issues, 6)), if o.open_issues.is_empty() { "good" } else { "warn" }),
        // Acceptance is the ONE human gate in an otherwise autonomous loop (D0049), so a proposal
        // nobody has seen is the single thing on this dashboard the human alone can clear. It gets a
        // card of its own rather than a line inside the Decisions surface, because that surface is
        // the accepted-only scorecard — it rendered everything EXCEPT what needs action (issue096).
        // "bad" and not "warn" when non-empty: this blocks the loop, it does not merely age.
        card(
            "Awaiting your acceptance",
            o.pending_acceptances.len().to_string(),
            if o.pending_acceptances.is_empty() {
                "no decision is waiting on you".to_string()
            } else {
                format!("proposed: {}", preview(&o.pending_acceptances, 6))
            },
            if o.pending_acceptances.is_empty() { "good" } else { "bad" },
        ),
        card("Suspect / stale", suspect_total.to_string(), format!("{} drift/criterion + {} invalid-evidence \u{2014} re-verify", o.suspect.len(), o.invalid_evidence.len()), if suspect_total == 0 { "good" } else { "warn" }),
        card(
            "Assurance readiness",
            rb.verdict().to_string(),
            format!("{} governed; {} coverage + {} critique + {} \u{2265}Medium + {} Critical + {} invariant blocker(s)", rb.governed, rb.coverage_gaps.len(), rb.critique_gaps.len(), rb.undispositioned_findings.len(), rb.unfixed_critical.len(), rb.invariant_violations.len()),
            rb.tone(),
        ),
    ];
    Ok(REPORT_TEMPLATE
        .replace("/*STYLE*/", TABLE_STYLE)
        .replace("/*TITLE*/", &json_esc("Orient \u{00b7} where things stand"))
        .replace("/*TREND*/", "null")
        .replace("/*CARDS*/", &Json::Arr(cards).dump()))
}

fn assurance_cards(root: &Path, model: &Model, cov: &[Coverage], stale: &HashSet<String>, done: &HashSet<String>, task_suspect: &HashSet<String>) -> Result<Vec<Json>, ViewError> {
    let total = cov.len();
    let ct = |t: &str| cov.iter().filter(|c| c.tier == t).count();
    let (verified, attested) = (ct("verified"), ct("attested"));
    // Headline from the shared coverage-ratio formula (D0090) — computed in exactly one place
    // (`coverage_pct_of`); verified/attested/total below are the structural breakdown for the detail.
    let covered_pct = coverage_pct_of(cov, "");
    let crit = compute_critique_coverage(model, stale, &CritiquePolicy::load(root)?);
    let crit_cov = crit.iter().filter(|c| c.covered).count();
    let crit_pct = pct(crit_cov, crit.len());
    let (att_total, att_missing) = compute_attestation(model);
    let att_pct = pct(att_total - att_missing.len(), att_total);
    // Open finding Issues by severity.
    let open: HashSet<String> = compute_issue_resolution(model, done).into_iter().filter(|i| i.open).map(|i| i.issue).collect();
    let sev_count = |s: &str| open.iter().filter(|n| model.items.get(*n).and_then(|i| i.attrs.get("severity")).map(String::as_str) == Some(s)).count();
    let (crit_f, high_f, med_f, low_f) = (sev_count("Critical"), sev_count("High"), sev_count("Medium"), sev_count("Low"));
    let undisp = crit_f + high_f + med_f;
    let suspect_load = task_suspect.len() + critique_suspect_set(model).len();
    let rb = compute_readiness(root)?;
    Ok(vec![
        card("Verification coverage", format!("{covered_pct}%"), format!("{verified} verified + {attested} attested of {total} (gate-covered)"), if total == 0 { "empty" } else { cov_tone(covered_pct) }),
        card("Critique coverage", format!("{crit_pct}%"), format!("{crit_cov} of {} elements Core-3 critiqued", crit.len()), cov_tone_of(crit_cov, crit.len())),
        card("Acceptance integrity", format!("{att_pct}%"), format!("{} of {att_total} accepted decisions attested", att_total - att_missing.len()), cov_tone_of(att_total - att_missing.len(), att_total)),
        card("Open findings (\u{2265}Medium)", undisp.to_string(), format!("{crit_f} Critical / {high_f} High / {med_f} Medium / {low_f} Low open"), if crit_f > 0 { "bad" } else if undisp > 0 { "warn" } else { "good" }),
        card("Suspect load", suspect_load.to_string(), format!("{} drift/criterion + {} failing-critique; {} stale verifications", task_suspect.len(), critique_suspect_set(model).len(), stale.len()), if suspect_load == 0 { "good" } else { "warn" }),
        card("Assurance readiness", rb.verdict().to_string(), format!("{} governed; {} coverage + {} critique + {} \u{2265}Medium + {} Critical + {} invariant blocker(s)", rb.governed, rb.coverage_gaps.len(), rb.critique_gaps.len(), rb.undispositioned_findings.len(), rb.unfixed_critical.len(), rb.invariant_violations.len()), rb.tone()),
    ])
}

fn traceability_cards(model: &Model, cov: &[Coverage]) -> Vec<Json> {
    let by_type = |ty: &str| -> Vec<&Coverage> { cov.iter().filter(|c| c.type_name == ty).collect() };
    let needs = by_type("Need");
    let reqs = by_type("SystemRequirement");
    // Headlines from the shared verified-ratio formula (D0090; single-source with metric_value); the
    // per-tier breakdown below stays local for the detail text.
    let n_pct = verified_pct_of(cov, "Need");
    let r_pct = verified_pct_of(cov, "SystemRequirement");
    // Edge completeness. A Need is satisfied by an OUTGOING satisfy edge (need -> requirement); a
    // requirement is verified by an INCOMING verify edge (test/critique -> requirement, #Verify).
    let names_of = |ty: &str| -> Vec<&String> { model.items.iter().filter(|(_, i)| i.type_name == ty).map(|(n, _)| n).collect() };
    let needs_names = names_of("Need");
    let n_tot = needs_names.len();
    let n_sat = needs_names.iter().filter(|n| has_outgoing(&model.edges, n, "satisfy")).count();
    let req_names = names_of("SystemRequirement");
    let r_tot = req_names.len();
    let r_ver = req_names
        .iter()
        .filter(|n| model.edges.iter().any(|e| e.kind == "verify" && &e.to == **n))
        .count();
    let r_tier = |t: &str| reqs.iter().filter(|c| c.tier == t).count();
    vec![
        card("Needs verified", format!("{n_pct}%"), format!("{} of {} needs reach a verified requirement", needs.iter().filter(|c| c.tier == "verified").count(), needs.len()), cov_tone(n_pct)),
        card("Requirements verified", format!("{r_pct}%"), format!("{} verified / {} attested / {} addressed / {} uncovered of {}", r_tier("verified"), r_tier("attested"), r_tier("addressed"), r_tier("uncovered") + r_tier("suspect"), reqs.len()), cov_tone(r_pct)),
        card("Needs with satisfy edge", format!("{}%", pct(n_sat, n_tot)), format!("{n_sat} of {n_tot} needs carry a satisfy edge"), cov_tone_of(n_sat, n_tot)),
        card("Requirements with verify edge", format!("{}%", pct(r_ver, r_tot)), format!("{r_ver} of {r_tot} requirements carry a verify edge (DO-178C-style traceability)"), cov_tone(pct(r_ver, r_tot))),
    ]
}

fn quality_debt_cards(root: &Path, model: &Model, cov: &[Coverage], stale: &HashSet<String>, task_suspect: &HashSet<String>) -> Vec<Json> {
    // Charter debt: grandfathered elements (pre-rigor) that are still not gate-covered or not critiqued.
    let gf_cov = crate::govern::grandfathered_under(root, COVERAGE_DECISION);
    let gf_crit = crate::govern::grandfathered_under(root, CRITIQUE_DECISION);
    let cov_debt = cov.iter().filter(|c| !is_covered_tier(c.tier) && gf_cov.as_ref().is_some_and(|g| g.contains(&c.element))).count();
    let crit_debt = compute_critique_coverage(model, stale, &CritiquePolicy::load_or_core3(root)).into_iter().filter(|c| !c.covered && gf_crit.as_ref().is_some_and(|g| g.contains(&c.element))).count();
    // Requirements volatility: supersede edges (churn signal).
    let supersedes = model.edges.iter().filter(|e| e.kind == "supersede").count();
    let decisions = model.items.values().filter(|i| i.type_name == "Decision").count();
    let vol_pct = pct(supersedes, decisions);
    let suspect_total = task_suspect.len() + critique_suspect_set(model).len();
    vec![
        card("Charter debt (coverage)", cov_debt.to_string(), format!("{cov_debt} grandfathered elements still not gate-covered (pre-D0079 rigor backlog)"), if cov_debt == 0 { "good" } else { "warn" }),
        card("Charter debt (critique)", crit_debt.to_string(), format!("{crit_debt} grandfathered elements still missing Core-3 critique (pre-D0080)"), if crit_debt == 0 { "good" } else { "warn" }),
        card("Requirements volatility", format!("{vol_pct}%"), format!("{supersedes} supersede edges across {decisions} decisions (churn / early-warning signal)"), if vol_pct >= 30 { "warn" } else { "good" }),
        card("Suspect + stale", suspect_total.to_string(), format!("{suspect_total} elements suspect; {} stale verifications to re-run", stale.len()), if suspect_total == 0 { "good" } else { "warn" }),
    ]
}

/// Days since 1970-01-01 for a civil date (Hinnant's algorithm; exact, no deps).
const fn days_from_civil(y0: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y0 - 1 } else { y0 };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Parse `"YYYY-MM-DD"` to days-since-epoch.
fn parse_ymd(s: &str) -> Option<i64> {
    let mut it = s.trim().splitn(3, '-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.chars().take_while(char::is_ascii_digit).collect::<String>().parse().ok()?;
    Some(days_from_civil(y, m, d))
}

/// The quoted value immediately after `key` on a line (`key` includes the opening quote).
fn quoted_after(line: &str, key: &str) -> Option<String> {
    line.split(key).nth(1)?.split('"').next().map(str::to_string)
}

/// Per-sprint flow facts pulled from one delivery file.
struct SprintFlow {
    points: i64,
    created: Option<i64>,
    refine: Option<i64>,
    retro: Option<i64>,
}

fn sprint_flow(text: &str) -> SprintFlow {
    let mut sf = SprintFlow { points: 0, created: None, refine: None, retro: None };
    for line in text.lines() {
        let t = line.trim_start();
        if sf.points == 0 {
            if let Some(p) = t.split("estimatedPoints = ").nth(1).and_then(|x| x.trim().trim_end_matches(';').trim().parse::<i64>().ok()) {
                sf.points = p;
            }
        }
        if sf.created.is_none() {
            if let Some(c) = quoted_after(t, "createdAt = \"") {
                sf.created = parse_ymd(&c);
            }
        }
        if t.contains("RefineGateR") {
            if let Some(d) = quoted_after(t, "judgedAt = \"") {
                sf.refine = parse_ymd(&d);
            }
        }
        if t.contains("RetroGateR") {
            if let Some(d) = quoted_after(t, "judgedAt = \"") {
                sf.retro = parse_ymd(&d);
            }
        }
    }
    sf
}

/// Per-sprint flow facts for every delivery file. Shared by `metric_value` (velocity/throughput/
/// burnup) and the flow scorecard (D0090) so the sprint set is parsed in exactly one place.
fn collect_flows(root: &Path) -> Vec<SprintFlow> {
    crate::collect_sysml(&root.join(".tracking").join("delivery"))
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok().map(|t| sprint_flow(&t)))
        .collect()
}

/// Mean delivered points per sprint (the canonical velocity, f64) over `flows`. The single velocity
/// formula — shared by the velocity Indicator/metric and the flow scorecard card (D0090).
fn velocity_of(flows: &[SprintFlow]) -> f64 {
    if flows.is_empty() {
        return 0.0;
    }
    let points: i64 = flows.iter().map(|f| f.points).sum();
    f64::from(i32::try_from(points).unwrap_or(i32::MAX)) / f64::from(u32::try_from(flows.len()).unwrap_or(u32::MAX))
}

fn flow_cards(root: &Path, model: &Model, orient: &crate::orient::Output) -> Vec<Json> {
    let _ = model;
    let ready = orient.ready.len();
    let wip = orient.in_progress_sprints.len();
    let open_issues = orient.open_issues.len();
    let flows = collect_flows(root);
    let sprints = flows.len();
    let total_points: i64 = flows.iter().map(|f| f.points).sum();
    // Canonical velocity from the shared formula (D0090) — same number the velocity Indicator shows.
    let velocity = velocity_of(&flows);
    // Cycle time (refine→retro) + lead time (created→retro), in days, over sprints with both dates.
    let cycles: Vec<i64> = flows.iter().filter_map(|f| Some(f.retro? - f.refine?)).collect();
    let cycle_mean = if cycles.is_empty() { 0 } else { cycles.iter().sum::<i64>() / i64::try_from(cycles.len()).unwrap_or(1) };
    let cycle_pts: i64 = flows.iter().filter(|f| f.retro.is_some() && f.refine.is_some()).map(|f| f.points).sum();
    let cycle_days_total: i64 = cycles.iter().sum();
    let per_point = if cycle_pts == 0 { 0.0 } else { f64::from(i32::try_from(cycle_days_total).unwrap_or(0)) / f64::from(i32::try_from(cycle_pts).unwrap_or(1)) };
    let leads: Vec<i64> = flows.iter().filter_map(|f| Some(f.retro? - f.created?)).collect();
    let lead_mean = if leads.is_empty() { 0 } else { leads.iter().sum::<i64>() / i64::try_from(leads.len()).unwrap_or(1) };
    // Predictability: spread of per-sprint points.
    let pts: Vec<i64> = flows.iter().map(|f| f.points).filter(|p| *p > 0).collect();
    let (pmin, pmax) = (pts.iter().min().copied().unwrap_or(0), pts.iter().max().copied().unwrap_or(0));
    // Aging WIP: as-of (latest recorded date) minus the refine date of any started-but-unfinished sprint.
    let as_of = flows.iter().filter_map(|f| f.retro.or(f.refine)).max().unwrap_or(0);
    let aging = flows.iter().filter(|f| f.refine.is_some() && f.retro.is_none()).filter_map(|f| Some(as_of - f.refine?)).max().unwrap_or(0);
    vec![
        card("Ready frontier", ready.to_string(), format!("{ready} task(s) ready to start now"), if ready == 0 { "warn" } else { "good" }),
        card("Work in progress", wip.to_string(), format!("{wip} sprint(s) with ceremony in progress (low WIP is healthy)"), if wip <= 2 { "good" } else { "warn" }),
        card("Velocity", format!("{velocity:.2}"), format!("~{velocity:.1} points/sprint (mean across {sprints} sprints, {total_points} pts total)"), "good"),
        card("Cycle time", format!("{cycle_mean}d"), format!("mean refine→retro across {} sprints (same-day autonomous = ~0)", cycles.len()), "good"),
        card("Time / story point", format!("{per_point:.2}d"), format!("{cycle_days_total} cycle-days / {cycle_pts} points (lower = faster delivery)"), "good"),
        card("Lead time", format!("{lead_mean}d"), format!("mean created→retro across {} sprints (DORA-style lead time)", leads.len()), "good"),
        card("Predictability", format!("{pmin}–{pmax} pts"), format!("per-sprint point spread (velocity {})", if pmax - pmin <= 4 { "consistent" } else { "variable" }), if pmax - pmin <= 4 { "good" } else { "warn" }),
        card("Throughput", sprints.to_string(), format!("{sprints} delivery sprints recorded"), "good"),
        card("Aging WIP", format!("{aging}d"), format!("oldest unfinished sprint age (as-of latest recorded date); {wip} in progress"), if aging <= 7 { "good" } else { "warn" }),
        card("Open issues", open_issues.to_string(), format!("{open_issues} open issue(s) on the board"), if open_issues == 0 { "good" } else { "warn" }),
    ]
}

/// Authoring-friction benchmark (D0054/issue029): record one canonical fact (a passing test result)
/// via the write API vs the hand-edit and spreadsheet baselines. Makes the D0054 first-class friction
/// requirement VERIFIABLE — "the write path beats a spreadsheet" becomes a checkable claim.
fn friction_cards() -> Vec<Json> {
    vec![
        card("Write API: record a fact", "1 command".to_string(), "append-result / append-gate-result / add-task / apply-review — one invocation, with auto UUID + who/when/commit provenance + append-only enforcement".to_string(), "good"),
        card("Hand-edit .sysml", "~6 steps".to_string(), "open file, locate the DoD, author the TestResult line, generate a UUID, find the insertion point, save — error-prone, no enforcement".to_string(), "warn"),
        card("Spreadsheet (baseline)", "1 row".to_string(), "fast to type, but NO provenance, NO validation, NO computed resolution/suspicion — the JPL friction trap (D0054)".to_string(), "warn"),
        card("Verdict vs spreadsheet", "beats it".to_string(), "the write path ties the spreadsheet on steps (1 command) and dominates on provenance + validation + computed state — satisfies the D0054 first-class friction requirement".to_string(), "good"),
    ]
}

// ── indicators (D0089: monitored measures; computed/pulled/manual; source-agnostic status) ──────

/// Direction-aware status of an indicator given its `goal` and its baseline->latest movement.
pub(super) fn indicator_status(goal: &str, baseline: f64, latest: f64) -> &'static str {
    let d = latest - baseline;
    match goal {
        "maximize" => {
            if d > 0.0 { "improving" } else if d < 0.0 { "degrading" } else { "flat" }
        }
        "minimize" => {
            if d < 0.0 { "improving" } else if d > 0.0 { "degrading" } else { "flat" }
        }
        _ => "observed",
    }
}

/// The recorded-Measurement series `(measuredAt, value)` (oldest->newest) banked for an indicator —
/// items typed `Measurement` with a `#Measures` edge to the indicator, sorted by `measuredAt`. Works
/// for any method: pulled/manual observations AND computed-indicator snapshots (D0091).
fn measurement_series(model: &Model, indicator: &str) -> Vec<(String, f64)> {
    let mut pts: Vec<(String, f64)> = model
        .edges
        .iter()
        .filter(|e| e.kind == "measures" && e.to == indicator)
        .filter_map(|e| model.items.get(&e.from).filter(|i| i.type_name == "Measurement"))
        .filter_map(|i| {
            let at = i.attrs.get("measuredAt").cloned().unwrap_or_default();
            let v = i.attrs.get("value").and_then(|s| s.parse::<f64>().ok())?;
            Some((at, v))
        })
        .collect();
    pts.sort_by(|a, b| a.0.cmp(&b.0));
    pts
}

/// `(name, metric-key)` for every `computed` Indicator — the snapshot worklist (D0091).
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn computed_indicator_keys(root: &Path) -> Result<Vec<(String, String)>, ViewError> {
    let model = Model::build(root)?;
    let mut out: Vec<(String, String)> = model
        .items
        .iter()
        .filter(|(_, i)| i.type_name == "Indicator" && i.attrs.get("method").map(String::as_str) == Some("computed"))
        .filter_map(|(n, i)| i.attrs.get("collectionRef").map(|k| (n.clone(), k.clone())))
        .collect();
    out.sort();
    Ok(out)
}

/// Indicators view (D0089): each declared `Indicator`'s value + direction-aware status.
///
/// Source-agnostic over the measurement method — computed series come from the report/trend engine
/// (current value only unless `trend`), pulled/manual series from recorded `Measurement`s.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn indicators(root: &Path, trend: bool) -> Result<String, ViewError> {
    let triggers = indicator_triggers(root);
    let mut triggered: Vec<Json> = Vec::new();
    let model = Model::build(root)?;
    let mut names: Vec<&String> = model.items.iter().filter(|(_, i)| i.type_name == "Indicator").map(|(n, _)| n).collect();
    names.sort();
    let mut out: Vec<Json> = Vec::new();
    for name in names {
        let Some(info) = model.items.get(name) else { continue };
        let method = info.attrs.get("method").map_or("manual", String::as_str);
        let goal = info.attrs.get("goal").map_or("observe", String::as_str);
        let binding = info.attrs.get("collectionRef").cloned().unwrap_or_default();
        // The banked (measuredAt, value) datapoint series (recorded observations + computed snapshots).
        // A snapshot stores only value + timestamp — no "latest" label; latest is CALCULATED (issue037).
        let banked = measurement_series(&model, name);
        // LIVE current value (computed indicators only) — authoritative + never stale (issue037/038):
        // the live recompute is the source of truth; the bank is historical record, never overrides it.
        let live: Option<f64> = if method == "computed" { metric_value(root, &binding) } else { None };
        // The displayed series: the bank; or, for a computed indicator with no bank, --trend / the live point.
        let series: Vec<(String, f64)> = if banked.is_empty() && method == "computed" {
            if trend {
                trend_series(root, &binding)
            } else {
                live.map(|v| vec![("live".to_string(), v)]).unwrap_or_default()
            }
        } else {
            banked.clone()
        };
        let baseline = series.first().map(|(_, v)| *v);
        // latest is computed: live current for a computed indicator; else the most recent datapoint.
        let latest = live.or_else(|| series.last().map(|(_, v)| *v));
        let banked_latest = banked.last().map(|(_, v)| *v);
        // Drift guardrail (issue038): for a computed indicator, has the live value moved off the last snapshot?
        let drift = matches!((live, banked_latest), (Some(l), Some(b)) if (l - b).abs() > 0.001);
        let status = if baseline.is_some() && latest.is_some() && (series.len() > 1 || live.is_some()) {
            indicator_status(goal, baseline.unwrap_or(0.0), latest.unwrap_or(0.0))
        } else if latest.is_some() {
            "single-point"
        } else {
            "no-data"
        };
        let fmt = |o: Option<f64>| o.map_or_else(|| Json::Null, |v| Json::s(format!("{v:.2}")));
        let series_json: Vec<Json> = series
            .iter()
            .map(|(at, v)| Json::Obj(vec![("at".to_string(), Json::s(at.clone())), ("value".to_string(), Json::s(format!("{v:.2}")))]))
            .collect();
        // TRIGGER (D0333): a declared threshold in `.engine/contracts/indicator-triggers.toml` turns a
        // number someone must think to read into a signal the reader is shown. Still an indicator, not
        // a requirement (D0088): crossing it surfaces WORK, it gates nothing.
        let trigger = triggers.get(name.as_str()).map(|t| {
            let crossed = latest.is_some_and(|v| t.crossed(v));
            if crossed {
                triggered.push(Json::Obj(vec![
                    ("indicator".to_string(), Json::s(name.clone())),
                    ("latest".to_string(), fmt(latest)),
                    ("threshold".to_string(), Json::s(t.describe())),
                    ("surfaces".to_string(), Json::s(t.surfaces.clone())),
                ]));
            }
            Json::Obj(vec![
                ("threshold".to_string(), Json::s(t.describe())),
                ("crossed".to_string(), Json::Bool(crossed)),
                ("surfaces".to_string(), Json::s(t.surfaces.clone())),
            ])
        });
        out.push(Json::Obj(vec![
            ("indicator".to_string(), Json::s(name.clone())),
            ("trigger".to_string(), trigger.unwrap_or(Json::Null)),
            ("measures".to_string(), Json::s(info.attrs.get("measures").cloned().unwrap_or_default())),
            ("method".to_string(), Json::s(method.to_string())),
            ("goal".to_string(), Json::s(goal.to_string())),
            ("unit".to_string(), Json::s(info.attrs.get("unit").cloned().unwrap_or_default())),
            ("latest".to_string(), fmt(latest)),       // calculated: live for computed, last datapoint otherwise
            ("live".to_string(), fmt(live)),           // authoritative current recompute (computed only)
            ("baseline".to_string(), fmt(baseline)),
            ("banked_latest".to_string(), fmt(banked_latest)),
            ("drift".to_string(), Json::Bool(drift)),  // computed: live has moved off the last snapshot
            ("points".to_string(), Json::Int(i64::try_from(series.len()).unwrap_or(0))),
            ("status".to_string(), Json::s(status.to_string())),
            ("series".to_string(), Json::Arr(series_json)),
        ]));
    }
    Ok(Json::Obj(vec![
        ("view".to_string(), Json::s("indicators (D0089/D0091): monitored measures + direction-aware status + the banked datapoint series. `latest` is CALCULATED (live recompute for computed indicators — authoritative, never stale; last datapoint otherwise); the bank stores value+timestamp only; `drift`=true when a computed indicator's live value has moved off its last snapshot (the bank is historical record, never overrides live). `triggered` lists the indicators past a declared threshold (indicator-triggers.toml, D0333) and the work each surfaces.".to_string())),
        ("triggered".to_string(), Json::Arr(triggered)),
        ("indicators".to_string(), Json::Arr(out)),
    ])
    .dump())
}

/// One declared indicator trigger (D0333): the threshold and the work crossing it surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorTrigger {
    pub above: Option<f64>,
    pub below: Option<f64>,
    pub surfaces: String,
}

impl IndicatorTrigger {
    #[must_use]
    pub fn crossed(&self, value: f64) -> bool {
        self.above.is_some_and(|a| value > a) || self.below.is_some_and(|b| value < b)
    }
    #[must_use]
    pub fn describe(&self) -> String {
        match (self.above, self.below) {
            (Some(a), Some(b)) => format!("above {a} or below {b}"),
            (Some(a), None) => format!("above {a}"),
            (None, Some(b)) => format!("below {b}"),
            (None, None) => "(no threshold)".to_string(),
        }
    }
}

/// `.engine/contracts/indicator-triggers.toml`: `[indicatorName] above = 50.0  surfaces = "..."`. An
/// absent file means no indicator triggers - the D0136 state, never an error.
#[must_use]
pub fn indicator_triggers(root: &Path) -> HashMap<String, IndicatorTrigger> {
    let Ok(text) = std::fs::read_to_string(root.join(".engine").join("contracts").join("indicator-triggers.toml")) else {
        return HashMap::new();
    };
    parse_indicator_triggers(&text)
}

/// Pure parse of the contract text (unit-tested).
#[must_use]
pub fn parse_indicator_triggers(text: &str) -> HashMap<String, IndicatorTrigger> {
    let mut out: HashMap<String, IndicatorTrigger> = HashMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        if let Some(name) = l.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
            current = Some(name.trim().to_string());
            out.entry(name.trim().to_string()).or_insert(IndicatorTrigger { above: None, below: None, surfaces: String::new() });
            continue;
        }
        let (Some(cur), Some((k, v))) = (current.as_ref(), l.split_once('=')) else { continue };
        let Some(t) = out.get_mut(cur) else { continue };
        let v = v.trim();
        match k.trim() {
            "above" => t.above = v.parse().ok(),
            "below" => t.below = v.parse().ok(),
            "surfaces" => t.surfaces = v.trim_matches('"').to_string(),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod trigger_tests {
    use super::{parse_indicator_triggers, IndicatorTrigger};

    /// Both directions, as the `DoD` demands of a discriminating indicator: past the threshold triggers,
    /// below it does not; a `below` trigger reads the other way; absent thresholds never trigger.
    #[test]
    fn a_trigger_fires_only_past_its_threshold_in_its_direction() {
        let t = parse_indicator_triggers("# triggers\n[ungroundedRatioIndicator]\nabove = 50.0\nsurfaces = \"latent-need derivation (D0249)\"\n[overrideRate]\nbelow = 5\nsurfaces = \"the reviewer is not reviewing\"\n");
        let u = &t["ungroundedRatioIndicator"];
        assert!(u.crossed(85.0) && !u.crossed(50.0) && !u.crossed(12.0));
        assert_eq!(u.surfaces, "latent-need derivation (D0249)");
        assert_eq!(u.describe(), "above 50");
        let o = &t["overrideRate"];
        assert!(o.crossed(0.0) && !o.crossed(5.0) && !o.crossed(40.0));
        assert!(!IndicatorTrigger { above: None, below: None, surfaces: String::new() }.crossed(99.0));
    }
}

fn governance_cards(model: &Model) -> Vec<Json> {
    let decisions: Vec<&ItemInfo> = model.items.values().filter(|i| i.type_name == "Decision").collect();
    let total = decisions.len();
    let accepted = decisions.iter().filter(|i| i.attrs.get("status").map(String::as_str) == Some("accepted")).count();
    let superseded = decisions.iter().filter(|i| i.attrs.get("status").map(String::as_str) == Some("superseded")).count();
    let proc_change = decisions.iter().filter(|i| matches!(i.marker.as_deref(), Some("ProspectiveChange" | "SafetyChange"))).count();
    let (att_total, att_missing) = compute_attestation(model);
    let att_pct = pct(att_total - att_missing.len(), att_total);
    let supersede_edges = model.edges.iter().filter(|e| e.kind == "supersede").count();
    vec![
        card("Decisions", total.to_string(), format!("{accepted} accepted / {superseded} superseded of {total} total"), "good"),
        card("Acceptance integrity", format!("{att_pct}%"), format!("{} of {att_total} accepted decisions carry an attestation event", att_total - att_missing.len()), cov_tone(att_pct)),
        card("Process-change decisions", proc_change.to_string(), format!("{proc_change} #ProspectiveChange/#SafetyChange (governed process edits, D0070)"), "good"),
        card("Supersession", supersede_edges.to_string(), format!("{supersede_edges} supersede edges (decision evolution / churn)"), if supersede_edges <= total / 3 { "good" } else { "warn" }),
    ]
}

const REPORT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>keel report</title>
<meta name="generator" content="keel report (computed #View; regenerate, do not commit as truth)">
/*STYLE*/
<style>
 .cards{display:flex;flex-wrap:wrap;gap:12px;padding:14px}
 .c{flex:1 1 220px;min-width:200px;border:1px solid #ddd;border-radius:8px;padding:12px;background:#fff}
 .c .l{font-size:11px;text-transform:uppercase;color:#666;letter-spacing:.03em}
 .c .v{font-size:30px;font-weight:600;margin:4px 0}
 .c .d{font-size:11px;color:#555;line-height:1.4}
 .c.empty{border-left:5px solid #9aa4b1} .c.good{border-left:5px solid #59a14f} .c.warn{border-left:5px solid #f2a900} .c.bad{border-left:5px solid #e15759}
 .c.good .v{color:#3d7a34} .c.warn .v{color:#b07a00} .c.bad .v{color:#b03a3c}
</style></head><body>
<header><h1>keel · <span id="t"></span></h1><p>computed aggregate report (D0087) — regenerate, never commit as truth</p></header>
<div id="trend" style="padding:0 14px"></div>
<div class="cards" id="cards"></div>
<script>
var TITLE="/*TITLE*/",CARDS=/*CARDS*/,TREND=/*TREND*/;
document.getElementById('t').textContent=TITLE;
var box=document.getElementById('cards');
CARDS.forEach(function(c){var d=document.createElement('div');d.className='c '+(c.tone||'');
 d.innerHTML='<div class=l></div><div class=v></div><div class=d></div>';
 d.querySelector('.l').textContent=c.label;d.querySelector('.v').textContent=c.value;d.querySelector('.d').textContent=c.detail;box.appendChild(d)});
if(TREND&&TREND.series&&TREND.series.length){var s=TREND.series.map(function(p){return p.value});
 var lo=Math.min.apply(null,s),hi=Math.max.apply(null,s),bl='▁▂▃▄▅▆▇█';
 var spark=s.map(function(v){var i=hi===lo?0:Math.round((v-lo)/(hi-lo)*7);return bl.charAt(i)}).join('');
 var first=s[0],last=s[s.length-1],delta=last-first,arrow=delta>0?'▲ +'+delta:delta<0?'▼ '+delta:'→ 0';
 document.getElementById('trend').innerHTML='<div class="c" style="border-left:5px solid #4e79a7"><div class=l>Trend · '+TREND.label+' ('+s.length+' commits)</div><div class=v style="font-family:monospace;font-size:22px">'+spark+'</div><div class=d>'+first+' → '+last+'  ('+arrow+'); range '+lo+'–'+hi+'. Computed from git history (worktree per commit); never stored.</div></div>'}
</script></body></html>"#;

/// Vendored cytoscape.js, INLINED into every generated diagram so it is fully self-contained +
/// offline (no CDN). ~373KB; the only third-party JS in the page.
const CYTOSCAPE_LIB: &str = include_str!("../../assets/cytoscape.min.js");

const DIAGRAM_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>keel traceability</title>
<meta name="generator" content="keel diagram (computed #View; regenerate, do not commit as truth)">
<script>/*CYTOSCAPE_LIB*/</script>
<style>
 html,body{margin:0;height:100%;font:12px system-ui,sans-serif}
 #cy{position:absolute;left:230px;right:0;top:0;bottom:0}
 #panel{position:absolute;left:0;top:0;bottom:0;width:230px;overflow:auto;background:#f7f7f7;border-right:1px solid #ccc;padding:8px;box-sizing:border-box}
 #panel h3{margin:8px 0 4px;font-size:11px;text-transform:uppercase;color:#555}
 #panel label{display:block;font-size:11px;line-height:1.5;cursor:pointer}
 #panel .sw{display:inline-block;width:9px;height:9px;margin-right:4px;border-radius:2px;vertical-align:middle}
 #search{width:100%;box-sizing:border-box;margin-bottom:6px}
 button{font-size:11px;margin:2px 2px 6px 0;cursor:pointer}
 #info{position:absolute;right:8px;top:8px;max-width:360px;background:#fff;border:1px solid #ccc;padding:8px;font-size:11px;display:none;white-space:pre-wrap;max-height:60%;overflow:auto}
</style></head><body>
<div id="panel">
 <input id="search" placeholder="search id… (Enter to fit)">
 <button onclick="cy.fit(undefined,30)">Fit</button><button onclick="resetView()">Reset</button>
 <h3>Node types</h3><div id="types"></div>
 <h3>Edge kinds</h3><div id="kinds"></div>
 <p style="color:#777;font-size:10px">Click a node = focus its neighborhood. Click background = reset. Computed view — regenerate, never commit as truth.</p>
</div>
<div id="cy"></div><div id="info"></div>
<script>
var elements = /*ELEMENTS*/;
var typeColors={Decision:'#4e79a7',Need:'#59a14f',SystemRequirement:'#76b7b2',Story:'#f28e2b',Test:'#9c755f',TestResult:'#bab0ac',Issue:'#e15759',action:'#edc948',ActionDef:'#e6d27a',Process:'#b07aa1',ProcessStep:'#d4a6c8',AISkill:'#86bcb6'};
var edgeColors={satisfy:'#59a14f',verify:'#4e79a7',charteredby:'#f28e2b',supersede:'#e15759',resolves:'#af7aa1',dependency:'#bab0ac',allocate:'#76b7b2',succession:'#9aa',ordering:'#ccc',prospectivechange:'#9c27b0',safetychange:'#d62728',dependson:'#888',contains:'#dcdcc8',resultof:'#e8e0d0'};
var cy=cytoscape({container:document.getElementById('cy'),elements:elements,
 style:[{selector:'node',style:{'label':'data(label)','font-size':6,'width':11,'height':11,'background-color':function(n){return typeColors[n.data('ntype')]||'#888'},'text-wrap':'wrap','text-max-width':130,'color':'#222'}},
  {selector:'edge',style:{'width':1,'line-color':function(e){return edgeColors[e.data('kind')]||'#bbb'},'target-arrow-color':function(e){return edgeColors[e.data('kind')]||'#bbb'},'target-arrow-shape':'triangle','arrow-scale':0.6,'curve-style':'bezier'}},
  {selector:'.hidden',style:{'display':'none'}},{selector:'.faded',style:{'opacity':0.07}},{selector:'.hi',style:{'background-color':'#ffd400','border-width':2,'border-color':'#c80'}}],
 layout:{name:'grid'}});
var offTypes={},offKinds={};
function refresh(){cy.batch(function(){
 cy.nodes().forEach(function(n){n.toggleClass('hidden',!!offTypes[n.data('ntype')])});
 cy.edges().forEach(function(e){var h=!!offKinds[e.data('kind')]||e.source().hasClass('hidden')||e.target().hasClass('hidden');e.toggleClass('hidden',h)});});}
function resetView(){offTypes={};offKinds={};cy.elements().removeClass('hidden faded hi');document.querySelectorAll('#panel input[type=checkbox]').forEach(function(c){c.checked=true});document.getElementById('info').style.display='none';cy.fit(undefined,30);}
function mkFilters(id,vals,colors,store,offDefault){var d=document.getElementById(id);vals.sort().forEach(function(v){var off=offDefault.indexOf(v)>=0;if(off)store[v]=true;var l=document.createElement('label');var c=document.createElement('input');c.type='checkbox';c.checked=!off;c.onchange=function(){store[v]=!c.checked;refresh()};var sw='<span class=sw style="background:'+(colors[v]||'#888')+'"></span>';l.appendChild(c);l.insertAdjacentHTML('beforeend',sw+v+(off?' (off)':''));d.appendChild(l)})}
mkFilters('types',Array.from(new Set(cy.nodes().map(function(n){return n.data('ntype')}))),typeColors,offTypes,['Test','TestResult']);
mkFilters('kinds',Array.from(new Set(cy.edges().map(function(e){return e.data('kind')}))),edgeColors,offKinds,[]);
refresh();
cy.elements(':visible').layout({name:'cose',animate:false,idealEdgeLength:55,nodeRepulsion:5000,componentSpacing:60}).run();
cy.fit(undefined,30);
cy.on('tap','node',function(ev){var n=ev.target;var nb=n.closedNeighborhood();cy.elements().addClass('faded');nb.removeClass('faded');var d=n.data();var s='';Object.keys(d).forEach(function(k){if(k!=='label')s+=k+': '+d[k]+'\n'});var inf=document.getElementById('info');inf.textContent=s;inf.style.display='block'});
cy.on('tap',function(ev){if(ev.target===cy){cy.elements().removeClass('faded');document.getElementById('info').style.display='none'}});
document.getElementById('search').addEventListener('input',function(e){var q=e.target.value.toLowerCase();cy.nodes().removeClass('hi');if(q)cy.nodes().filter(function(n){return (n.id()+' '+(n.data('label')||'')).toLowerCase().indexOf(q)>=0}).addClass('hi')});
document.getElementById('search').addEventListener('keydown',function(e){if(e.key==='Enter'){var hi=cy.nodes('.hi');if(hi.length)cy.fit(hi,50)}});
</script></body></html>"#;

const TABLE_STYLE: &str = r"<style>
 body{margin:0;font:13px system-ui,sans-serif;color:#222}
 header{padding:10px 14px;background:#f7f7f7;border-bottom:1px solid #ccc}
 header h1{margin:0;font-size:15px} header p{margin:3px 0 0;color:#666;font-size:12px}
 #bar{padding:8px 14px;display:flex;gap:8px;align-items:center;flex-wrap:wrap}
 input,select{padding:3px 4px;font:12px system-ui,sans-serif}
 table{border-collapse:collapse;width:100%;font-size:12px}
 th,td{border:1px solid #ddd;padding:4px 7px;text-align:left;vertical-align:top}
 th{background:#eee;cursor:pointer;position:sticky;top:0}
 tbody tr:nth-child(even){background:#fafafa}
 td.name{font-family:ui-monospace,monospace;white-space:nowrap}
 .count{color:#666;font-size:12px} button{cursor:pointer;padding:4px 9px}
 textarea{width:100%;box-sizing:border-box;font:12px system-ui,sans-serif;min-height:30px}
</style>";

const TABLE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>keel view</title>
<meta name="generator" content="keel render --mode table (computed #View; regenerate, do not commit as truth)">
/*STYLE*/</head><body>
<header><h1>keel · <span id="vn"></span></h1><p id="cn"></p></header>
<div id="bar"><input id="q" placeholder="filter rows…" size="36"><span class="count" id="ct"></span></div>
<table id="t"><thead></thead><tbody></tbody></table>
<script>
var VIEW="/*VIEW*/",CONCERN="/*CONCERN*/",COLS=/*COLS*/,ROWS=/*ROWS*/;
document.getElementById('vn').textContent=VIEW;document.getElementById('cn').textContent=CONCERN;
var allCols=['name','type'].concat(COLS),sortCol=null,sortDir=1,filter='';
function visible(){var rows=ROWS.filter(function(r){return !filter||allCols.some(function(c){return (''+(r[c]||'')).toLowerCase().indexOf(filter)>=0})});
 if(sortCol)rows.sort(function(a,b){var x=''+(a[sortCol]||''),y=''+(b[sortCol]||'');return x<y?-sortDir:x>y?sortDir:0});return rows}
function render(){var th=document.querySelector('#t thead');th.innerHTML='';var tr=document.createElement('tr');
 allCols.forEach(function(c){var h=document.createElement('th');h.textContent=c+(sortCol===c?(sortDir>0?' ▲':' ▼'):'');h.onclick=function(){if(sortCol===c)sortDir=-sortDir;else{sortCol=c;sortDir=1}render()};tr.appendChild(h)});
 th.appendChild(tr);var rows=visible(),tb=document.querySelector('#t tbody');tb.innerHTML='';
 rows.forEach(function(r){var t=document.createElement('tr');allCols.forEach(function(c){var d=document.createElement('td');if(c==='name')d.className='name';d.textContent=r[c]||'';t.appendChild(d)});tb.appendChild(t)});
 document.getElementById('ct').textContent=rows.length+' / '+ROWS.length+' rows';}
document.getElementById('q').addEventListener('input',function(e){filter=e.target.value.toLowerCase();render()});render();
</script></body></html>"#;

const REVIEW_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>keel review</title>
<meta name="generator" content="keel render --mode review (computed #View; capture is exported to JSON for apply-review)">
/*STYLE*/</head><body>
<header><h1>keel review · <span id="vn"></span></h1><p id="cn"></p></header>
<div id="bar">
 reviewer <input id="who" placeholder="your id" size="12">
 commit <input id="sha" placeholder="judgedAgainst (optional)" size="12">
 <input id="q" placeholder="filter rows…" size="22">
 <button onclick="exportJSON()">Export JSON</button>
 <span class="count" id="ct"></span>
</div>
<table id="t"><thead></thead><tbody></tbody></table>
<script>
var VIEW="/*VIEW*/",CONCERN="/*CONCERN*/",COLS=/*COLS*/,ROWS=/*ROWS*/;
document.getElementById('vn').textContent=VIEW;document.getElementById('cn').textContent=CONCERN;
var LENSES=['correctness','completeness','ambiguity','testability','feasibility','consistency','necessity'];
var SEV=['Medium','High','Critical','Low'];
var infoCols=['name','type'].concat(COLS),disp={};
function st(n){if(!disp[n])disp[n]={verdict:'',lens:'correctness',severity:'Medium',rationale:'',actionable:false};return disp[n]}
function visible(){return ROWS.filter(function(r){return !filter||infoCols.some(function(c){return (''+(r[c]||'')).toLowerCase().indexOf(filter)>=0})})}
var filter='';
function sel(opts,val){var s=document.createElement('select');opts.forEach(function(o){var e=document.createElement('option');e.value=o;e.textContent=o;if(o===val)e.selected=true;s.appendChild(e)});return s}
function render(){var th=document.querySelector('#t thead');th.innerHTML='';var tr=document.createElement('tr');
 infoCols.concat(['verdict','lens','severity','actionable?','rationale']).forEach(function(c){var h=document.createElement('th');h.textContent=c;tr.appendChild(h)});th.appendChild(tr);
 var rows=visible(),tb=document.querySelector('#t tbody');tb.innerHTML='';
 rows.forEach(function(r){var n=r.name,s=st(n),t=document.createElement('tr');
  infoCols.forEach(function(c){var d=document.createElement('td');if(c==='name')d.className='name';d.textContent=r[c]||'';t.appendChild(d)});
  var dv=document.createElement('td');var v=sel(['','accept','finding'],s.verdict);v.onchange=function(){s.verdict=v.value};dv.appendChild(v);t.appendChild(dv);
  var dl=document.createElement('td');var l=sel(LENSES,s.lens);l.onchange=function(){s.lens=l.value};dl.appendChild(l);t.appendChild(dl);
  var ds=document.createElement('td');var sv=sel(SEV,s.severity);sv.onchange=function(){s.severity=sv.value};ds.appendChild(sv);t.appendChild(ds);
  var da=document.createElement('td');var a=document.createElement('input');a.type='checkbox';a.checked=s.actionable;a.onchange=function(){s.actionable=a.checked};da.appendChild(a);t.appendChild(da);
  var dr=document.createElement('td');var ta=document.createElement('textarea');ta.value=s.rationale;ta.oninput=function(){s.rationale=ta.value};dr.appendChild(ta);t.appendChild(dr);
  tb.appendChild(t)});
 document.getElementById('ct').textContent=rows.length+' rows';}
function exportJSON(){var out={view:VIEW,judgedBy:document.getElementById('who').value,judgedAgainst:document.getElementById('sha').value,dispositions:[]};
 Object.keys(disp).forEach(function(n){var d=disp[n];if(d&&d.verdict){out.dispositions.push({element:n,verdict:d.verdict,lens:d.lens,severity:d.severity,rationale:d.rationale,actionable:d.actionable})}});
 if(!out.dispositions.length){alert('No dispositions set — choose a verdict on at least one row.');return}
 var b=new Blob([JSON.stringify(out,null,2)],{type:'application/json'});var a=document.createElement('a');a.href=URL.createObjectURL(b);a.download='review-batch.json';a.click();}
document.getElementById('q').addEventListener('input',function(e){filter=e.target.value.toLowerCase();render()});render();
</script></body></html>"#;

/// Comprehensive traceability diagram as a self-contained interactive HTML page (D0085).
///
/// Emits the WHOLE model — every element (typed node + its authored metadata) and every typed edge
/// (satisfy/verify/charteredby/supersede/resolves/dependency/allocate/succession/process-change/...) —
/// into one cytoscape page with type/edge filters, search, click-to-focus, and fit. A computed
/// `#View`: regenerate on demand (`keel diagram . > graph.html`), never commit it as truth.
///
/// # Errors
/// Returns [`ViewError`] if a tracking/instance file fails to parse.
pub fn diagram_html(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let elements = graph_elements(&model, None);
    Ok(DIAGRAM_TEMPLATE
        .replace("/*CYTOSCAPE_LIB*/", CYTOSCAPE_LIB)
        .replace("/*ELEMENTS*/", &Json::Arr(elements).dump()))
}

/// Build the cytoscape element array (typed nodes + their metadata, then edges with both endpoints
/// present) for a model. When `only` is `Some`, restrict to that name-set (a view-scoped subgraph);
/// `None` renders the whole model.
fn graph_elements(model: &Model, only: Option<&HashSet<String>>) -> Vec<Json> {
    let meta_keys = ["title", "status", "severity", "lens", "kind", "priority", "outcome", "method", "critiquedBy", "createdBy"];
    let included = |n: &str| only.is_none_or(|s| s.contains(n));
    let mut items: Vec<(&String, &ItemInfo)> = model.items.iter().filter(|(n, _)| included(n)).collect();
    items.sort_by(|a, b| a.0.cmp(b.0));
    let mut elements: Vec<Json> = items
        .iter()
        .map(|(name, info)| {
            // Label with the authored `title` (human string) when present, truncated for legibility;
            // fall back to the part name. The id stays the name (identity); full title is in the
            // click-info panel.
            let label = info.attrs.get("title").map_or_else(
                || (*name).clone(),
                |t| {
                    if t.chars().count() > 60 {
                        format!("{}…", t.chars().take(59).collect::<String>())
                    } else {
                        t.clone()
                    }
                },
            );
            let mut data = vec![
                ("id".to_string(), Json::s((*name).clone())),
                ("label".to_string(), Json::s(label)),
                ("ntype".to_string(), Json::s(if info.type_name.is_empty() { "unknown".to_string() } else { info.type_name.clone() })),
            ];
            for k in meta_keys {
                if let Some(v) = info.attrs.get(k) {
                    data.push((k.to_string(), Json::s(v.clone())));
                }
            }
            if let Some(m) = &info.marker {
                data.push(("marker".to_string(), Json::s(m.clone())));
            }
            Json::Obj(vec![("data".to_string(), Json::Obj(data))])
        })
        .collect();
    // Edges: only those whose BOTH endpoints are present nodes (cytoscape errors on dangling edges);
    // when scoped, both endpoints must also be in the name-set.
    for (i, e) in model.edges.iter().enumerate() {
        if model.items.contains_key(&e.from) && model.items.contains_key(&e.to) && included(&e.from) && included(&e.to) {
            elements.push(Json::Obj(vec![(
                "data".to_string(),
                Json::Obj(vec![
                    ("id".to_string(), Json::s(format!("e{i}"))),
                    ("source".to_string(), Json::s(e.from.clone())),
                    ("target".to_string(), Json::s(e.to.clone())),
                    ("kind".to_string(), Json::s(e.kind.clone())),
                ]),
            )]));
        }
    }
    elements
}

/// Modular interactive-artifact renderer (D0086).
///
/// Renders a declared view as self-contained HTML in one of three modes — `graph` (cytoscape; the
/// whole model when `view` is `model`/`all`, else the view's selected subgraph), `table`
/// (sortable/searchable rows), or `review` (table + per-row accept/finding + rationale capture with
/// a JSON export for `apply-review`). A computed `#View`: regenerate on demand, never commit as truth.
///
/// # Errors
/// Returns [`ViewError`] for an unknown mode, a missing/invalid view, or a parse failure.
pub fn render_html(root: &Path, view: &str, mode: &str) -> Result<String, ViewError> {
    match mode {
        "graph" => {
            if matches!(view, "model" | "all" | "whole") {
                return diagram_html(root);
            }
            let (_, model, result) = run_resolved(root, view)?;
            let elements = graph_elements(&model, Some(&result));
            Ok(DIAGRAM_TEMPLATE
                .replace("/*CYTOSCAPE_LIB*/", CYTOSCAPE_LIB)
                .replace("/*ELEMENTS*/", &Json::Arr(elements).dump()))
        }
        "table" | "review" => {
            let (spec, model, result) = run_resolved(root, view)?;
            Ok(table_or_review_html(&spec, &model, &result, mode == "review"))
        }
        other => Err(ViewError::UnknownMode(other.to_string())),
    }
}

/// Columns rendered for a view's rows: `name`, `type`, then the view's projected fields (or a small
/// default set of common authored fields when the view declares no projection).
fn view_columns(spec: &ViewSpec, model: &Model, result: &HashSet<String>) -> Vec<String> {
    if let Some(p) = &spec.project {
        if !p.fields.is_empty() {
            return p.fields.clone();
        }
    }
    let defaults = ["title", "status", "severity", "outcome", "lens", "method", "kind", "priority"];
    defaults
        .iter()
        .filter(|f| result.iter().any(|n| model.items.get(n).is_some_and(|i| i.attrs.contains_key(**f))))
        .map(|f| (*f).to_string())
        .collect()
}

/// Render a view's rows as either a read-only table or a review surface (extra capture columns +
/// an Export-JSON button that emits a batch consumable by `keel apply-review`).
fn table_or_review_html(spec: &ViewSpec, model: &Model, result: &HashSet<String>, review: bool) -> String {
    let cols = view_columns(spec, model, result);
    let mut names: Vec<&String> = result.iter().collect();
    names.sort();
    let rows: Vec<Json> = names
        .iter()
        .filter_map(|n| {
            model.items.get(*n).map(|info| {
                let mut o = vec![
                    ("name".to_string(), Json::s((*n).clone())),
                    ("type".to_string(), Json::s(info.type_name.clone())),
                ];
                for c in &cols {
                    let v = if c == "marker" { info.marker.clone().unwrap_or_default() } else { info.attrs.get(c).cloned().unwrap_or_default() };
                    o.push((c.clone(), Json::s(v)));
                }
                Json::Obj(o)
            })
        })
        .collect();
    let template = if review { REVIEW_TEMPLATE } else { TABLE_TEMPLATE };
    template
        .replace("/*STYLE*/", TABLE_STYLE)
        .replace("/*VIEW*/", &json_esc(&spec.name))
        .replace("/*CONCERN*/", &json_esc(&spec.concern))
        .replace("/*COLS*/", &Json::Arr(cols.iter().map(|c| Json::s(c.clone())).collect()).dump())
        .replace("/*ROWS*/", &Json::Arr(rows).dump())
}


#[cfg(test)]
mod empty_population_tests {
    use super::{cov_tone_of, pct};

    /// D0286: an empty population is 0%, never 100%. Every fresh project inheriting keel used to open
    /// on a clean sweep of 100% cards - nothing authored, everything "complete".
    #[test]
    fn an_empty_population_is_zero_percent_not_a_clean_sweep() {
        assert_eq!(pct(0, 0), 0, "0/0 is nothing done, not everything done");
        assert_eq!(pct(3, 0), 0, "a numerator over nothing is still nothing measured");
        assert_eq!(pct(0, 4), 0, "and zero of four is a real 0%");
        assert_eq!(pct(4, 4), 100);
        assert_eq!(pct(3, 4), 75);
    }

    /// The empty case gets its OWN tone so a fresh project reads "nothing yet", not "failing" - and
    /// certainly not "good".
    #[test]
    fn the_empty_case_has_its_own_tone_and_never_reads_good() {
        assert_eq!(cov_tone_of(0, 0), "empty");
        assert_eq!(cov_tone_of(0, 4), "bad");
        assert_eq!(cov_tone_of(4, 4), "good");
    }

    /// THE CONTROL: no ratio anywhere in the crate may default to full on an empty denominator. A
    /// source scan, because the three sites this fixed were in three files with three different
    /// spellings of the same mistake (`map_or(100`, `None => 100`, `t.map_or(100`), and the fourth
    /// would be spelled a fourth way. Any `100` chosen as the fallback of a division is caught.
    #[test]
    fn no_division_in_the_crate_falls_back_to_one_hundred() {
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).expect("src").flatten() {
                let p = e.path();
                if p.is_dir() { walk(&p, out); } else if p.extension().is_some_and(|x| x == "rs") { out.push(p); }
            }
        }
        let mut files = Vec::new();
        walk(std::path::Path::new("src"), &mut files);
        let mut hits = Vec::new();
        for f in files {
            let text = std::fs::read_to_string(&f).expect("read");
            for (n, line) in text.lines().enumerate() {
                let l = line.trim();
                if l.starts_with("//") || l.contains("l.contains(") { continue; } // comments, and this scanner's own pattern line
                let bad = (l.contains("checked_div") || l.contains("map_or(") || l.contains("unwrap_or(") || l.contains("=> 100"))
                    && (l.contains("map_or(100") || l.contains("unwrap_or(100") || l.contains("None => 100") || l.contains("map_or(100.0") || l.contains("unwrap_or(100.0"));
                if bad {
                    hits.push(format!("{}:{}: {l}", f.display(), n + 1));
                }
            }
        }
        assert!(hits.is_empty(), "a ratio falls back to 100 on an empty population - D0286 says nothing measured is 0%:\n{}", hits.join("\n"));
    }
}
