//! `keel hardening` — the architectural-critique process's own questions, COMPUTED (issue171/D0169).
//!
//! WHY THIS MODULE EXISTS, and it is the least flattering reason in this codebase. D0046's critique
//! process asks a stable, recurring set of questions, and every pass re-derived them as throwaway python.
//! In the pass that produced this module, FOUR probes were wrong before they were right:
//!
//!   - a regex matching `: Process` also matched `: ProcessStep` — 131 processes reported, 24 exist;
//!   - help extraction reported `keel orient` and `keel assured` as nonexistent, minutes after both had
//!     been run in the same session;
//!   - a second attempt at the same question reported 0 of 72 subcommands documented;
//!   - a registry probe reported 0 registered skills against 35 real declarations, which would have
//!     become a phantom-drift finding of exactly the kind CR-10 already fixed once in 2026-06.
//!
//! Each wrong number was ONE EDIT from becoming a recorded finding, and a recorded finding directs the
//! next session. D0040 says a recurring task without a skill leaks process knowledge into conversation
//! history; this leaked worse than that — it leaked WRONG ANSWERS toward the model.
//!
//! Everything here reads SOURCE rather than running the binary. A view that shells out to itself to
//! report on itself cannot be trusted, and the help question in particular is about what the source
//! claims, not about what one invocation happened to print.

use crate::json::Json;
use std::path::Path;

/// The hardening lens set.
///
/// # Errors
/// Never returns `Err` today; the signature matches the other view functions so it can be served by
/// `keel serve`'s cache, which is typed over `Result`.
pub fn hardening(root: &Path) -> Result<String, crate::view::ViewError> {
    Ok(Json::Obj(vec![
        (
            "hardening".to_string(),
            Json::s(
                "the architectural-critique process's own questions, computed rather than re-probed \
                 (issue171). Every number here was once a hand-written regex that got it wrong.",
            ),
        ),
        ("helpCoverage".to_string(), help_coverage(root)),
        ("processEnforcement".to_string(), process_enforcement(root)),
        ("decisionFollowThrough".to_string(), decision_follow_through(root)),
        ("apiSurface".to_string(), api_surface(root)),
        ("enforcementPoints".to_string(), enforcement_points(root)),
    ])
    .dump())
}

/// A percentage, or `None` when there is no population to take one OF (issue183).
///
/// This returned 100 for an empty population - a reasonable-looking divide-by-zero guard that turns
/// "nothing was measured" into "everything passed". Against a tree with no `keel-cli/src`, which is what
/// every downstream project is, the help lens reported 0 of 0 dispatched at 100%. A FALSE GREEN IS
/// WORSE THAN A WRONG NUMBER, because nobody investigates a pass.
fn pct(part: usize, whole: usize) -> Option<i64> {
    (whole > 0).then(|| i64::try_from(part * 100 / whole).unwrap_or(0))
}

/// The value, or an explicit `unavailable` marker naming why nothing could be measured.
fn measured(v: Option<i64>, why: &str) -> Json {
    v.map_or_else(|| Json::s(format!("unavailable: {why}")), Json::Int)
}

fn count(n: usize) -> Json {
    Json::Int(i64::try_from(n).unwrap_or(0))
}

// ── lens 1: does the CLI describe itself? ────────────────────────────────────────────────────────

/// Which top-level subcommands `keel --help` names, and which it does not (issue172).
///
/// The CLI is the authority and the automation substrate (D0093). A subcommand absent from help is
/// reachable only by reading `main.rs` or CLAUDE.md, which is not discoverability.
fn help_coverage(root: &Path) -> Json {
    // A LENS THAT CANNOT READ ITS INPUT SAYS SO (issue183). `unwrap_or_default` turned an unreadable
    // file into an empty one, an empty one into an empty population, and an empty population into 100%.
    let Ok(main) = std::fs::read_to_string(root.join("keel-cli/src/main.rs")) else {
        return Json::Obj(vec![
            ("available".to_string(), Json::Bool(false)),
            (
                "reason".to_string(),
                Json::s(
                    "keel-cli/src/main.rs is not readable from this root, so the CLI surface cannot be                      audited. This lens reads SOURCE - it answers about a source tree, never about an                      installed binary (D0169).",
                ),
            ),
        ]);
    };
    let dispatched = dispatch_arms(&main);
    let help = usage_text(&main);
    let (named, absent): (Vec<String>, Vec<String>) =
        dispatched.iter().cloned().partition(|c| help_names(&help, c));
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "Help is a hand-maintained string beside a hand-maintained match, so the two drift \
                 silently and only ever one way: a new command gets dispatched and never described.",
            ),
        ),
        ("dispatched".to_string(), count(dispatched.len())),
        ("namedInHelp".to_string(), count(named.len())),
        ("coveragePct".to_string(), measured(pct(named.len(), dispatched.len()), "nothing dispatched was found - the lens is mis-aimed")),
        ("available".to_string(), Json::Bool(true)),
        ("absentFromHelp".to_string(), Json::Arr(absent.into_iter().map(Json::s).collect())),
    ])
}

/// The top-level subcommand strings in `fn main`'s dispatch, including `|`-joined alternatives.
///
/// Scans only the `Some(...)` head of each arm, so a string appearing in an arm's BODY is never mistaken
/// for a subcommand name.
#[must_use]
pub fn dispatch_arms(main: &str) -> Vec<String> {
    let Some(i) = main.find("fn main() {") else { return Vec::new() };
    let mut out = std::collections::BTreeSet::new();
    let mut rest = &main[i..];
    while let Some(j) = rest.find("Some(") {
        rest = &rest[j + "Some(".len()..];
        let Some(arrow) = rest.find("=>") else { break };
        let head = &rest[..arrow];
        // A long head is not a subcommand arm — it is a match on something else entirely.
        if head.len() > 200 {
            continue;
        }
        for lit in string_literals(head) {
            if !lit.is_empty() && lit.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
                out.insert(lit);
            }
        }
    }
    out.into_iter().collect()
}

/// Every `"…"` literal in a fragment, without regex.
fn string_literals(frag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = frag;
    while let Some(a) = rest.find('"') {
        rest = &rest[a + 1..];
        let Some(b) = rest.find('"') else { break };
        out.push(rest[..b].to_string());
        rest = &rest[b + 1..];
    }
    out
}

/// The help text as the binary prints it: the `CATALOGUE` table `print_usage` iterates.
///
/// TWO WRONG VERSIONS OF THIS FUNCTION, both caught by the lens itself rather than by inspection. The
/// first looked for `fn usage(`, which does not exist - the function is `print_usage` - and so read an
/// EMPTY help text and reported 0 of 75 documented. The second read `print_usage`'s body, which was
/// right until the fix for issue172 moved the lines into a `const CATALOGUE`, at which point it reported
/// 1 of 75. A lens over source has to be re-aimed when the source moves; the value of it being a lens is
/// that a wrong aim shows up as an absurd number instead of a plausible one.
fn usage_text(main: &str) -> String {
    let Some(i) = main.find("const CATALOGUE:") else { return String::new() };
    let body = &main[i..];
    let end = body.find("
];").map_or(body.len(), |e| e + 3);
    body[..end].to_string()
}

/// Does the help text NAME this subcommand? WORD-BOUNDED, because a plain substring test lets
/// `check-engine` satisfy `check` and `activation` satisfy `act` — the exact bug that made the
/// hand-written version of this probe report 0 of 72 commands documented.
fn help_names(help: &str, cmd: &str) -> bool {
    let bytes = help.as_bytes();
    let mut from = 0;
    while let Some(rel) = help[from..].find(cmd) {
        let s = from + rel;
        let e = s + cmd.len();
        let before_ok = s.checked_sub(1).and_then(|i| bytes.get(i)).is_none_or(|b| !is_cmd_char(*b));
        let after_ok = bytes.get(e).is_none_or(|b| !is_cmd_char(*b));
        if before_ok && after_ok {
            return true;
        }
        from = s + 1;
    }
    false
}

const fn is_cmd_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

// ── lens 2: can a process be enforced at all? ────────────────────────────────────────────────────

/// Per top-level process: does its unit assert a guard, i.e. is its constraint machine-checkable?
///
/// AN INDICATOR, NEVER A GATE (invariant 7). Some constraints are genuinely judgments — a definition of
/// done is a judgment, a critique is a judgment — and no guard can be conjured for them. What this
/// reports is how much of the catalogue is enforceable, so the unenforceable part is VISIBLE rather than
/// assumed covered.
fn process_enforcement(root: &Path) -> Json {
    let act = crate::activation::Activation::load(root);
    let declared = enforcement_contract(root);
    let mut enforced: Vec<Json> = Vec::new();
    let mut unenforceable: Vec<Json> = Vec::new();
    let mut undeclared: Vec<Json> = Vec::new();
    for f in crate::collect_sysml(&root.join(".engine/processes")) {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        let unit = f.file_stem().unwrap_or_default().to_string_lossy().to_string();
        let n = act.unit(&unit).map_or(0, |u| u.guards.len());
        for name in top_level_processes(&text) {
            let entry = declared.get(&name);
            let mut row = vec![
                ("process".to_string(), Json::s(name.clone())),
                ("unit".to_string(), Json::s(unit.clone())),
                ("guards".to_string(), count(n)),
            ];
            if n > 0 {
                enforced.push(Json::Obj(row));
                continue;
            }
            match entry {
                // `checkable = false` - a judgment, and no guard can exist for it. Correct, permanent.
                Some((false, reason)) => {
                    row.push(("reason".to_string(), Json::s(reason.clone())));
                    unenforceable.push(Json::Obj(row));
                }
                // `checkable = true` with no guard - an ADMITTED gap. Worse than undeclared, because
                // someone looked at it and agreed a guard could exist.
                Some((true, reason)) => {
                    row.push(("admittedGap".to_string(), Json::Bool(true)));
                    row.push(("reason".to_string(), Json::s(reason.clone())));
                    undeclared.push(Json::Obj(row));
                }
                // No entry at all. Silence reads as a GAP, never as consent.
                None => {
                    row.push(("admittedGap".to_string(), Json::Bool(false)));
                    undeclared.push(Json::Obj(row));
                }
            }
        }
    }
    let total = enforced.len() + unenforceable.len() + undeclared.len();
    let accounted = enforced.len() + unenforceable.len();
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "Three buckets, because the two-bucket version made a judgment look identical to a                  missing guard (issue173). ENFORCED asserts a guard. UNENFORCEABLE is declared in                  `.engine/contracts/process-enforcement.toml` with a reason a reviewer can disagree                  with. UNDECLARED is the honest gap - and an `admittedGap` is one where the contract                  itself says a guard COULD exist. An INDICATOR, never a gate: gating this ratio would                  make the cheapest fix a guard that checks nothing.",
            ),
        ),
        ("processes".to_string(), count(total)),
        ("enforced".to_string(), count(enforced.len())),
        ("declaredUnenforceable".to_string(), count(unenforceable.len())),
        ("accountedPct".to_string(), measured(pct(accounted, total), "no processes declared")),
        ("undeclared".to_string(), Json::Arr(undeclared)),
        ("unenforceable".to_string(), Json::Arr(unenforceable)),
    ])
}

/// `process -> (checkable, reason)` from `.engine/contracts/process-enforcement.toml`.
///
/// An absent file yields an empty map, so every unguarded process reports as undeclared - the
/// pre-issue173 behaviour, and strictly less informative rather than wrong.
fn enforcement_contract(root: &Path) -> std::collections::BTreeMap<String, (bool, String)> {
    let mut out = std::collections::BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(root.join(".engine/contracts/process-enforcement.toml"))
    else {
        return out;
    };
    let mut current = String::new();
    let mut checkable = false;
    let mut reason = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            if !current.is_empty() {
                out.insert(current.clone(), (checkable, reason.clone()));
            }
            current = name.to_string();
            checkable = false;
            reason = String::new();
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        match k.trim() {
            "checkable" => checkable = v.trim() == "true",
            "reason" => reason = v.trim().trim_matches('"').to_string(),
            _ => {}
        }
    }
    if !current.is_empty() {
        out.insert(current, (checkable, reason));
    }
    out
}

/// `action NAME : Process {` — and NOT `: ProcessStep`, which is the entire point of this function.
#[must_use]
pub fn top_level_processes(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|raw| {
            let line = raw.trim_start();
            if line.starts_with("//") {
                return None;
            }
            let rest = line.strip_prefix("action ")?;
            let (name, after) = rest.split_once(':')?;
            let tail = after.trim_start().strip_prefix("Process")?;
            tail.trim_start().starts_with('{').then(|| name.trim().to_string())
        })
        .collect()
}

// ── lens 3: was an accepted Decision actually carried out? ───────────────────────────────────────

/// Accepted Decisions that PROMISE a named artifact, and whether that artifact exists (issue174).
///
/// Promises are declared in `.engine/contracts/decision-artifacts.toml` rather than in a Decision field,
/// deliberately: `schema/core` is frozen (invariant 5), and this question does not need a schema change
/// to answer. The contract is authored data, not a view.
///
/// NOT A GATE. A Decision may legitimately be accepted long before it is built — D0161 was accepted the
/// day it was written — and blocking on an unbuilt promise would push the promise out of the record
/// rather than into it. What was missing is that nothing DISTINGUISHED `accepted and needs nothing` from
/// `accepted and abandoned`; 64 of 161 accepted Decisions have no chartered work and most of those are
/// correct, which is why the raw count was never the finding.
fn decision_follow_through(root: &Path) -> Json {
    let path = root.join(".engine/contracts/decision-artifacts.toml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Json::Obj(vec![
            ("declared".to_string(), Json::Int(0)),
            (
                "note".to_string(),
                Json::s(
                    "No `.engine/contracts/decision-artifacts.toml`. A project that never declared a \
                     promise has not broken one, so this reports nothing rather than guessing (D0138).",
                ),
            ),
        ]);
    };
    let mut kept: Vec<Json> = Vec::new();
    let mut broken: Vec<Json> = Vec::new();
    let mut decision = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            decision = name.to_string();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else { continue };
        if key.trim() != "artifact" {
            continue;
        }
        let rel = value.trim().trim_matches('"');
        let row = Json::Obj(vec![
            ("decision".to_string(), Json::s(decision.clone())),
            ("artifact".to_string(), Json::s(rel)),
        ]);
        if root.join(rel).exists() { kept.push(row) } else { broken.push(row) }
    }
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "An accepted Decision whose promised artifact is ABSENT is the mechanism behind \
                 half-baked implementations: acceptance is recorded, the promise is not, and the model \
                 reads as complete. Reported, never gated.",
            ),
        ),
        ("declared".to_string(), count(kept.len() + broken.len())),
        ("kept".to_string(), count(kept.len())),
        ("unbuilt".to_string(), Json::Arr(broken)),
    ])
}

// ── lens 4: does the HTTP surface declare itself? ────────────────────────────────────────────────

/// Registered `/api` routes vs what `/api/version` advertises and what the console calls (issue178).
///
/// ISSUE172 ONE LAYER OUT, and it exists as a lens for the same reason: the FIFTH hand probe of this
/// session to produce a wrong number was this exact question, asked one pass after the instrument was
/// built to stop that happening. My probe read `fn api_version`'s body, which names the endpoint
/// CONSTANTS rather than containing the endpoint strings, and so reported 13 unaccounted routes where
/// there were 3.
///
/// UNACCOUNTED is the number that matters, not `unadvertised`: D0114 commits a versioned read API for a
/// separate viewer, so a route the console never calls is entirely legitimate. A route NOTHING declares
/// is different - a consumer cannot discover it and a maintainer cannot tell it from a leftover.
fn api_surface(root: &Path) -> Json {
    let (Ok(serve), Ok(html)) = (
        std::fs::read_to_string(root.join("keel-cli/src/serve.rs")),
        std::fs::read_to_string(root.join("keel-cli/assets/console.html")),
    ) else {
        return Json::Obj(vec![
            ("available".to_string(), Json::Bool(false)),
            (
                "reason".to_string(),
                Json::s(
                    "keel-cli/src/serve.rs or assets/console.html is not readable from this root, so                      the HTTP surface cannot be audited (issue183).",
                ),
            ),
        ]);
    };
    let routes = registered_routes(&serve);
    let advertised = advertised_endpoints(&serve);
    let mut unaccounted = Vec::new();
    for r in &routes {
        if advertised.contains(r) {
            continue;
        }
        let base = r.split("/:").next().unwrap_or(r).trim_end_matches('/');
        if !base.is_empty() && html.contains(base) {
            continue;
        }
        unaccounted.push(Json::s(r.clone()));
    }
    // The reverse direction: advertising a route that does not exist is a promise to a consumer that
    // will 404. Cheap to check once the two sets are in hand, and it has never been checked.
    let phantom: Vec<Json> =
        advertised.iter().filter(|a| !routes.contains(*a)).map(|a| Json::s(a.clone())).collect();
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "UNACCOUNTED, not unadvertised: D0114 commits a read API for a separate viewer, so a                  route the console never calls is legitimate. A route NOTHING declares cannot be                  discovered by a consumer or distinguished from a leftover by a maintainer.",
            ),
        ),
        ("available".to_string(), Json::Bool(true)),
        ("routes".to_string(), count(routes.len())),
        ("advertised".to_string(), count(advertised.len())),
        ("unaccounted".to_string(), Json::Arr(unaccounted)),
        ("advertisedButNotRegistered".to_string(), Json::Arr(phantom)),
    ])
}

/// Every `.route("/api/...")` path registered on the router.
#[must_use]
pub fn registered_routes(serve: &str) -> std::collections::BTreeSet<String> {
    let mut out = std::collections::BTreeSet::new();
    let mut rest = serve;
    while let Some(i) = rest.find(".route(\"") {
        rest = &rest[i + ".route(\"".len()..];
        let Some(e) = rest.find('"') else { break };
        let path = &rest[..e];
        if path.starts_with("/api/") {
            out.insert(path.to_string());
        }
    }
    out
}

/// Every endpoint string inside the two `KEEL_API_*_ENDPOINTS` constants.
///
/// Reads the CONSTANTS, not `api_version`'s body — the distinction the hand probe missed.
#[must_use]
pub fn advertised_endpoints(serve: &str) -> std::collections::BTreeSet<String> {
    let mut out = std::collections::BTreeSet::new();
    for name in ["const KEEL_API_READ_ENDPOINTS", "const KEEL_API_WRITE_ENDPOINTS"] {
        let Some(i) = serve.find(name) else { continue };
        let tail = &serve[i..];
        let end = tail.find("];").unwrap_or(tail.len());
        for lit in string_literals(&tail[..end]) {
            if lit.starts_with("/api/") {
                out.insert(lit);
            }
        }
    }
    out
}

// ── lens 6: the enforcement-point inventory (K2, D0174/P0.7) ─────────────────────────────────────

/// Every enforcement point with its absent/error/timeout behavior — enumerated and reported, so no
/// point is silently open without appearing here (kernel invariant K2).
///
/// The TIMEOUT column matters most: the harness resolves a hook timeout to ALLOW, which nothing at
/// this layer can change — so every timeout-resolves-to-allow point is listed as a recorded
/// residual whose visibility is the fire-ledger (an expected fire with no ledger line is the
/// detection signal, read by PM's analysis and the P5 post-run gate).
fn enforcement_points(root: &Path) -> Json {
    const HOOK_TIMEOUT: &str = "harness resolves to ALLOW - recorded residual; detection = expected fire with no fire-ledger line";
    let settings: serde_json::Value = std::fs::read_to_string(root.join(".claude").join("settings.json"))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(serde_json::Value::Null);
    let has_event = |ev: &str| settings.pointer(&format!("/hooks/{ev}")).is_some();
    let hooks_wired = root.join(".githooks").join("pre-commit").exists()
        && crate::gitx::git()
            .arg("-C")
            .arg(root)
            .args(["config", "core.hooksPath"])
            .output()
            .is_ok_and(|o| o.status.success() && !String::from_utf8_lossy(&o.stdout).trim().is_empty());
    let ci = root.join(".github").join("workflows").exists()
        && std::fs::read_dir(root.join(".github").join("workflows")).is_ok_and(|rd| {
            rd.flatten().any(|e| {
                std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains("keel validate") || t.contains("keel guard"))
            })
        });
    let row = |point: &str, present: bool, absent: &str, error: &str, timeout: &str| {
        Json::Obj(vec![
            ("point".to_string(), Json::s(point)),
            ("present".to_string(), Json::Bool(present)),
            ("absent".to_string(), Json::s(absent)),
            ("error".to_string(), Json::s(error)),
            ("timeout".to_string(), Json::s(timeout)),
        ])
    };
    let rows = vec![
        row("hook UserPromptSubmit (route-first)", has_event("UserPromptSubmit"), "no routing reminder - advisory only, nothing gates", "prints and allows (D0134)", HOOK_TIMEOUT),
        row("hook PostToolUse post-edit (fast tier)", has_event("PostToolUse"), "per-edit gate lost; turn/commit tiers still gate (K15)", "loud warning, allows", HOOK_TIMEOUT),
        row("hook Stop (turn gate)", has_event("Stop"), "turn gate lost; commit/CI tiers still gate (K15)", "loud warning, allows", HOOK_TIMEOUT),
        row("hook PreToolUse pre-bash (shell advisory)", has_event("PreToolUse"), "advisory only, nothing gates", "prints and allows", HOOK_TIMEOUT),
        row("hook PreToolUse pre-write (protected paths)", has_event("PreToolUse"), "PURE-SHELL fallback DENIES without the binary, loudly (P0.3)", "deny message malformed -> harness asks", HOOK_TIMEOUT),
        row("hook SubagentStop (subagent tree gate)", has_event("SubagentStop"), "subagent writes reach the turn gate instead", "loud warning, allows", HOOK_TIMEOUT),
        row("pre-push .githooks (behind check)", root.join(".githooks").join("pre-push").exists(), "raw pushes land isolation-gated; CI still re-derives post-hoc (issue209's pre-fix state)", "fetch failure REFUSES the push (fail-loud, K2)", "n/a - synchronous, no harness deadline"),
        row("pre-commit .githooks (commit gate)", hooks_wired, "UNSET hooksPath = never runs - orient warns loudly; binary absent = exit 1 with the install path (K2, fail-loud)", "exit 1, commit blocked", "n/a - synchronous, no harness deadline"),
        row("CI keel gate (layer 3, hook-independent)", ci, "no remote verification; audit-history still re-derives locally (K15)", "CI red", "CI runner timeout -> red, visible"),
        // issue203 (critique pass 1): this point existed since the parser crate's birth and was
        // never in this inventory - which is how its all-zeros sentinel stayed dead for its whole
        // life. Its network-error path resolves to ALLOW with a warning: the recorded residual.
        row("build-time spec pin (keel-parser build.rs)", root.join("keel-parser").join("build.rs").exists(), "no build-time check on trees without the parser source (every downstream project) - the shipped binary was built against the pinned spec upstream", "SHA mismatch fails the build loudly (sr13SpecPin)", "network error or SYSML_V2_SPEC_OFFLINE resolves to ALLOW with a warning - RECORDED RESIDUAL (issue203)"),
        row(&format!("guards at gate tiers ({} enforced)", crate::guards::GUARD_NAMES.len()), true, "guards run inside validate/gate/commit paths - absent only if the binary is absent (see pre-commit row)", "guard error = violation, fails loud (issue183 rule)", "n/a - in-process"),
        row("declared rules", true, "report-only until P1.5 wires them into hook stop, pre-commit, and CI - RECORDED RESIDUAL (d0177)", "rule parse error reported", "n/a - in-process"),
    ];
    Json::Obj(vec![
        (
            "note".to_string(),
            Json::s(
                "Kernel invariant K2: no enforcement point is silently open without appearing here. \
                 Rows where `present` is false are open points on THIS tree; timeout-resolves-to-allow \
                 rows are residuals whose visibility is the fire-ledger.",
            ),
        ),
        ("points".to_string(), Json::Arr(rows)),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `: ProcessStep` must NOT be counted as a process. This is the single worst number the
    /// hand-written audit produced — 131 where there are 24 — and it is one character of regex.
    #[test]
    fn a_process_step_is_not_a_process() {
        let text = "    action intake : Process {\n    action inRecord : ProcessStep {\n\
                    action other : Process{\n    // action commented : Process {\n";
        assert_eq!(top_level_processes(text), vec!["intake", "other"]);
    }

    /// Word boundaries: `check-engine` in the help must not satisfy `check`, and `activation` must not
    /// satisfy `act`. A plain `contains` here reported 0 of 72 commands undocumented.
    #[test]
    fn help_names_is_word_bounded() {
        let help = "  check-engine ROOT   do a thing\n  activation [ROOT]  another\n  ls  list\n";
        assert!(help_names(help, "check-engine"));
        assert!(help_names(help, "activation"));
        assert!(help_names(help, "ls"));
        assert!(!help_names(help, "check"), "check-engine must not satisfy check");
        assert!(!help_names(help, "act"), "activation must not satisfy act");
        assert!(!help_names(help, "orient"));
    }

    /// THE CONTROL for issue172: every dispatched subcommand must be named in the catalogue.
    ///
    /// Expressed through the lens rather than as a separate probe, so the test and the reported number
    /// can never disagree - if the lens is re-aimed wrongly this test fails too, which is what happened
    /// twice while building it and is exactly the behaviour wanted. A new `Some("x") =>` arm with no
    /// catalogue line now fails the build instead of quietly shipping an undiscoverable command.
    #[test]
    fn every_dispatched_subcommand_is_documented() {
        let main = std::fs::read_to_string("src/main.rs").expect("main.rs is readable");
        let dispatched = dispatch_arms(&main);
        assert!(dispatched.len() > 50, "the dispatch scan found {} arms - the lens is mis-aimed", dispatched.len());
        let help = usage_text(&main);
        assert!(!help.is_empty(), "the CATALOGUE could not be located - the lens is mis-aimed");
        let absent: Vec<&String> = dispatched.iter().filter(|c| !help_names(&help, c)).collect();
        assert!(
            absent.is_empty(),
            "{} subcommand(s) dispatched but absent from the help CATALOGUE: {absent:?}",
            absent.len()
        );
    }

    /// THE CONTROL for issue178: no registered route may be undeclared, and nothing may be advertised
    /// that is not registered. The second half has never been checked and would 404 a real consumer.
    #[test]
    fn every_registered_route_is_accounted_for() {
        let serve = std::fs::read_to_string("src/serve.rs").expect("serve.rs is readable");
        let html = std::fs::read_to_string("assets/console.html").expect("console.html is readable");
        let routes = registered_routes(&serve);
        let advertised = advertised_endpoints(&serve);
        assert!(routes.len() > 30, "route scan found {} - the lens is mis-aimed", routes.len());
        assert!(advertised.len() > 20, "advertisement scan found {} - mis-aimed", advertised.len());
        let unaccounted: Vec<&String> = routes
            .iter()
            .filter(|r| {
                !advertised.contains(*r)
                    && !html.contains(r.split("/:").next().unwrap_or(r).trim_end_matches('/'))
            })
            .collect();
        assert!(unaccounted.is_empty(), "route(s) declared by nothing: {unaccounted:?}");
        let phantom: Vec<&String> = advertised.iter().filter(|a| !routes.contains(*a)).collect();
        assert!(phantom.is_empty(), "advertised but not registered - would 404: {phantom:?}");
    }

    /// THE CONTROL for issue179: every positional read is either flag-guarded or a keyword match.
    ///
    /// A TEXT CHECK over `main.rs` rather than a behavioural test, deliberately: the defect was a
    /// BYPASS - `cmd_init` read `args.first()` directly and so never reached the helper that had been
    /// rejecting unknown flags all along - so the property to pin is "nothing bypasses the guard", which
    /// no single invocation can demonstrate. `keel init --help` scaffolded an engine into a directory
    /// named `--help` and 277 files were committed before the CRLF warnings gave it away.
    ///
    /// A read is ACCEPTABLE when the line filters leading dashes, or when the value is immediately
    /// matched/compared against known keywords (a flag then falls through to the unknown-keyword error).
    /// The first version of this test counted every read and reported 11 violations, most of them
    /// keyword dispatches - blunt, but it found four genuine ones I had missed.
    #[test]
    fn every_positional_read_is_flag_guarded() {
        let main = std::fs::read_to_string("src/main.rs").expect("main.rs is readable");
        let mut offenders = Vec::new();
        for (i, line) in main.lines().enumerate() {
            let trimmed = line.trim_start();
            // A doc comment MENTIONING the pattern is not a read of it. Missing this made the test
            // report its own explanatory comment as a violation.
            if !line.contains("args.first()") || trimmed.starts_with("//") {
                continue;
            }
            let guarded = line.contains("starts_with('-')")
                || line.contains("let Some(first) = args.first()") // the helper itself
                || line.contains("match args.first()")
                || line.contains("== Some(")
                || line.contains("!= Some(");
            // `cmd_activation` and `cmd_hook` guard on a PRECEDING line, so accept a nearby guard too.
            let nearby = main
                .lines()
                .skip(i.saturating_sub(6))
                .take(7)
                .any(|l| l.contains("starts_with('-')"));
            if !guarded && !nearby {
                offenders.push(format!("{}: {}", i + 1, line.trim()));
            }
        }
        assert!(
            offenders.is_empty(),
            "positional read(s) that would accept a flag as a path or name: {offenders:#?}"
        );
    }

    /// THE CONTROL for issue182: no write path may DEFAULT a provenance date.
    ///
    /// Five did, to the literal `2026-01-01`, so a result written without a date claimed it happened in
    /// January. Latent - no corpus item ever carried it - but guard 36 exists precisely to catch evidence
    /// citing a date it could not have had, and this was the write path fabricating one. Mentioning the
    /// literal in a comment or an error message is fine; USING it as a fallback is not.
    #[test]
    fn no_write_path_defaults_a_provenance_date() {
        let main = std::fs::read_to_string("src/main.rs").expect("main.rs is readable");
        let offenders: Vec<String> = main
            .lines()
            .enumerate()
            .filter(|(_, l)| {
                let s = l.trim_start();
                !s.starts_with("//")
                    && (s.contains("unwrap_or_else(|| \"20") || s.contains("unwrap_or(\"20"))
            })
            .map(|(i, l)| format!("{}: {}", i + 1, l.trim()))
            .collect();
        assert!(
            offenders.is_empty(),
            "date literal(s) used as a default in a write path: {offenders:#?}"
        );
    }

    /// THE CONTROL for issue183: a lens that cannot read its input must NOT report a percentage.
    ///
    /// `pct` returned 100 for an empty population, so against a tree with no `keel-cli/src` - which is
    /// what every downstream project is - the help lens reported `0/0 dispatched -> 100%`. A false green
    /// is worse than a wrong number, because nobody investigates a pass. D0169 already DOCUMENTED that
    /// this lens reads a source tree; documenting a limitation is not enforcing it.
    #[test]
    fn a_percentage_is_never_reported_for_an_empty_population() {
        assert_eq!(pct(0, 0), None, "an empty population has no percentage - not 100");
        assert_eq!(pct(1, 0), None);
        assert_eq!(pct(3, 4), Some(75));
        assert_eq!(pct(0, 4), Some(0), "zero of four is a real 0%, not unavailable");
        let unavailable = measured(None, "nothing to measure").dump();
        assert!(
            unavailable.contains("unavailable:"),
            "an unmeasurable value must be a labelled string, got {unavailable}"
        );
    }

    /// The lenses report `available: false` against a tree with no source, and never a green number.
    #[test]
    fn the_source_lenses_refuse_a_tree_with_no_source() {
        // A directory that exists and contains no `keel-cli/` - the downstream shape.
        let empty = std::path::Path::new("..").join("target");
        let out = help_coverage(&empty);
        let Json::Obj(fields) = &out else { panic!("expected an object") };
        let keys: Vec<&str> = fields.iter().map(|(k, _)| k.as_str()).collect();
        assert!(keys.contains(&"available"), "must say whether it could measure: {keys:?}");
        assert!(!keys.contains(&"coveragePct"), "must NOT report a percentage it could not compute");
    }

    /// A string inside an arm's BODY is not a subcommand. Without the head-only scan, every literal in
    /// `main.rs` became a phantom command.
    #[test]
    fn dispatch_arms_reads_only_the_match_head() {
        let src = "fn main() {\n    match x {\n        Some(\"orient\") => cmd_orient(rest),\n\
                   Some(v @ (\"activate\" | \"deactivate\")) => go(v),\n\
                   Some(\"land\") => { eprintln!(\"not-a-command\"); }\n    }\n}";
        let arms = dispatch_arms(src);
        assert!(arms.contains(&"orient".to_string()));
        assert!(arms.contains(&"activate".to_string()));
        assert!(arms.contains(&"deactivate".to_string()));
        assert!(arms.contains(&"land".to_string()));
        assert!(!arms.contains(&"not-a-command".to_string()), "an arm BODY literal is not a command");
    }
}
