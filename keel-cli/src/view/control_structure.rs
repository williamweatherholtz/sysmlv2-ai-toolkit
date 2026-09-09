//! `keel show control-structure` — STPA step 2 for keel itself, COMPUTED (D0284, st066: "a defined
//! system for keel: what authorities are present, how they interact, what data is sent").
//!
//! Everything derivable is derived from the facts that already wire the authorities:
//!
//! | element | source |
//! |---|---|
//! | the hook boundary's actions | `.claude/settings.json` hook events → `keel hook <sub>` |
//! | the commit gate's actions | `.githooks/*` → the `keel` commands each hook runs |
//! | CI's and the channel's actions | `.github/workflows/*.yml` → step names + `keel` commands run |
//! | the agent's actions / feedback | `.engine/cli/commands.sysml` → `effect = writes\|both` / `reads` |
//! | the human's declared deciders | `.engine/contracts/github-actors.toml` |
//! | the remote's rules | branch protection, fetched LIVE via `gh api` — never copied into the tree |
//!
//! What is NOT derivable is read from the authored residue (`Controller`, `ControlledProcess`,
//! `ProcessModel`, `OtherInputOutput` instances, hazard→process edges) when the project has authored
//! it, and reported as absent WITH THE REASON when it has not: a fresh project computes the same
//! actions with no anchors, and its `stepTwoGate` rows say which clauses that leaves unmet.
//!
//! ROLES are the view's own, stable ids (`hooks`, `commit-gate`, `ci`, `channel`, `agent`, `human`,
//! `remote`, `console`); an authored `Controller` named `ct<Role>` decorates the role with its title,
//! id and process model. The structure never depends on the decoration existing. A role that nothing
//! wires and nothing anchors is not in the structure at all: it is listed under `absentRoles` with the
//! source that would wire it (issue394: `channel` was drawn inert for a week after its workflows went).
//!
//! The handbook's remaining step-2 elements are COMPUTED from the actions (D0363, issue394):
//! an ACTUATOR per action - the mechanism that carries the controller's action to its process
//! (p.26) - and each controller's RESPONSIBILITIES - the hazards on the processes it acts on, one
//! hierarchical level deep, since a controller acting on the agent's turn answers for what the agent
//! can reach. The fifth element type, other inputs and outputs that are neither control nor feedback
//! (p.25), is authored (`OtherInputOutput`): what enters from outside the boundary is a judgment.
//! The SOP's step-2 gate is then read clause by clause into `stepTwoGate`, holds or not, with the
//! evidence that decided it - so "step 2 is complete" is a row set, never a sentence.

use super::{Json, Model, Path, ViewError};
use std::path::PathBuf;
use std::process::Command;

/// One computed control action.
struct Action {
    name: String,
    title: String,
    issued_by: &'static str,
    acts_on: &'static str,
    data: String,
    source: String,
}

/// One computed feedback path.
struct Fb {
    name: String,
    title: String,
    sensed_from: &'static str,
    reports_to: &'static str,
    data: String,
    source: String,
}

/// The view's controller roles, in the order a change travels through them. `anchor` is the authored
/// `Controller` name that decorates the role when present; the last field is what WIRES the role -
/// the source its actions are computed from - reported for a role this project does not have.
const ROLES: [(&str, &str, &str, &str); 8] = [
    ("human", "ctHuman", "the human director", "an intake path (every project has one), or a declared decider in .engine/contracts/github-actors.toml"),
    ("agent", "ctAgent", "the AI agent", "write commands in .engine/cli/commands.sysml"),
    ("hooks", "ctHooks", "keel at the Claude Code hook boundary", "hook events in .claude/settings.json"),
    ("commit-gate", "ctCommitGate", "keel at the git hook boundary", "hooks in .githooks/"),
    ("ci", "ctCI", "GitHub Actions CI", "a workflow in .github/workflows/ not named decision-*"),
    ("channel", "ctChannel", "the decision channel workflows", "a decision-*.yml workflow in .github/workflows/"),
    ("remote", "ctRemote", "the GitHub remote", "branch protection on origin/main, fetched live"),
    ("console", "ctConsole", "keel serve", "the serve command in the CLI facts"),
];

/// The controlled processes, likewise: role id, authored anchor, what it is, and - when the process
/// IS another controller's activity - the controller that enacts it. The agent's turn is the agent
/// acting, so a controller acting on the turn (the hooks) is responsible, one level down, for every
/// hazard the agent's own actions can reach. That is the hierarchy the handbook draws (pp.22-23).
const PROCESSES: [(&str, &str, &str, Option<&str>); 6] = [
    ("model", "cpModel", "the recorded model (.tracking + .engine instances)", None),
    ("main-ref", "cpMainRef", "the shared history (origin/main)", None),
    ("enforcement-surface", "cpEnforcementSurface", "guards, hooks, workflows, processes, skills, contracts", None),
    ("deliverable", "cpDeliverable", "the deliverable source and tests", None),
    ("work", "cpWork", "sprints, ceremonies, the frontier", None),
    ("agent-turn", "cpAgentTurn", "one Claude Code response", Some("agent")),
];

fn role_of_controller_anchor(anchor: &str) -> &'static str {
    ROLES.iter().find(|(_, a, _, _)| *a == anchor).map_or("", |(r, _, _, _)| *r)
}

fn role_of_process_anchor(anchor: &str) -> &'static str {
    PROCESSES.iter().find(|(_, a, _, _)| *a == anchor).map_or("", |(r, _, _, _)| *r)
}

/// One computed ACTUATOR: the mechanism that carries a controller's action to its process (Handbook
/// p.26 - the control path's counterpart to a sensor). Derived from the action the way `actsOn` is: a
/// judgment made once here, visible in the output so it can be argued with, never authored twice.
struct Actuator {
    name: &'static str,
    title: &'static str,
    mechanism: &'static str,
    source: &'static str,
}

const ACT_HARNESS_VERDICT: Actuator = Actuator {
    name: "actHarnessVerdict",
    title: "Claude Code applies the hook's verdict",
    mechanism: "the harness reads the hook's stdout JSON and blocks, denies or allows the tool call or the turn; the hook itself changes nothing",
    source: ".claude/settings.json; .engine/claude-plugin/hooks/hooks.json",
};
const ACT_LAUNCH_SETTINGS: Actuator = Actuator {
    name: "actLaunchSettings",
    title: "the launch passes settings above project scope",
    mechanism: "`keel claude` passes --settings and --plugin-dir; the harness's own precedence carries the pin, so a kill switch on disk is overridden (D0296 run 6)",
    source: "keel claude",
};
const ACT_GIT_HOOK_DISPATCH: Actuator = Actuator {
    name: "actGitHookDispatch",
    title: "git runs the hook and honours its exit code",
    mechanism: "core.hooksPath -> .githooks/<hook>; a non-zero exit refuses the commit or the push - git carries the refusal, keel only exits",
    source: ".githooks/",
};
const ACT_CHECK_CONCLUSION: Actuator = Actuator {
    name: "actCheckConclusion",
    title: "the runner records the job's conclusion on the ref",
    mechanism: "GitHub Actions executes the job; the red or green conclusion is all the action leaves on main - preventive only if branch protection requires it (see remote.reading)",
    source: ".github/workflows/",
};
const ACT_CHANNEL_DELEGATION: Actuator = Actuator {
    name: "actChannelDelegation",
    title: "the channel workflow records on the model by delegation",
    mechanism: "a decision-*.yml workflow parses the decider's comment and runs the write command under the delegated actor",
    source: ".github/workflows/decision-*.yml",
};
const ACT_REF_UPDATE_REFUSAL: Actuator = Actuator {
    name: "actRefUpdateRefusal",
    title: "GitHub refuses the ref update",
    mechanism: "branch protection is evaluated at receive-pack: a force-push or a deletion is rejected before the ref moves",
    source: "branch protection, fetched live",
};
const ACT_SERVE_WRITE_API: Actuator = Actuator {
    name: "actServeWriteApi",
    title: "keel serve calls the write API",
    mechanism: "HTTP on 127.0.0.1:7777 into the same write layer; a tap is signed by the paired device (D0334/D0335) and an unsigned one writes nothing",
    source: "keel serve",
};
const ACT_WRITE_LAYER: Actuator = Actuator {
    name: "actWriteLayer",
    title: "the keel write layer lands the file",
    mechanism: "temp-file-then-rename under .keel-write-lock; actor and date required or the write refuses (D0129, issue184/185)",
    source: "keel-cli/src/write.rs",
};
const ACT_GIT_PUSH: Actuator = Actuator {
    name: "actGitPush",
    title: "git pushes after the gate",
    mechanism: "keel sync / keel land gate every project in the workspace, then git pushes; what the ref accepts is the remote's rule",
    source: "keel land; keel sync",
};
const ACT_SURFACE_REWRITE: Actuator = Actuator {
    name: "actSurfaceRewrite",
    title: "keel rewrites the generated surface in place",
    mechanism: "init, migrate, sync-claude, activate and deactivate write the engine-owned files directly; sync-claude --check reports the drift afterwards",
    source: "keel-cli/src (init, migrate, claude_surface, activation)",
};
const ACT_AGENT_ROUTING: Actuator = Actuator {
    name: "actAgentRouting",
    title: "the agent reads the direction and routes the turn",
    mechanism: "prose enters the agent's context; the intake skill and the agent's own routing translate it - nothing mechanical carries it, and the routing rig investigates whether it arrives (D0382, issue415)",
    source: ".claude/skills/intake/SKILL.md",
};
const ACT_HARNESS_FILE_TOOLS: Actuator = Actuator {
    name: "actHarnessFileTools",
    title: "the harness's Write, Edit and Bash tools change the source",
    mechanism: "keel mediates none of it: pre-write advises on protected surfaces, and manifest drift makes done work suspect afterwards",
    source: "Claude Code tools; .engine/deliverable-manifest.txt",
};

/// The actuator of one action, or the stated reason it has none. `channel_wired` is whether any
/// decision-*.yml workflow exists: the human's decision on the channel has an actuator only then.
fn actuator_for(a: &Action, channel_wired: bool) -> Result<&'static Actuator, String> {
    if a.name == "launchPin" {
        return Ok(&ACT_LAUNCH_SETTINGS);
    }
    if a.name.starts_with("hook") {
        return Ok(&ACT_HARNESS_VERDICT);
    }
    if a.name.starts_with("githook") {
        return Ok(&ACT_GIT_HOOK_DISPATCH);
    }
    match (a.issued_by, a.name.as_str()) {
        ("ci", _) => Ok(&ACT_CHECK_CONCLUSION),
        ("channel", _) => Ok(&ACT_CHANNEL_DELEGATION),
        ("remote", _) => Ok(&ACT_REF_UPDATE_REFUSAL),
        ("console", _) => Ok(&ACT_SERVE_WRITE_API),
        (_, "humanDirects") => Ok(&ACT_AGENT_ROUTING),
        (_, "humanDecidesOnChannel") if channel_wired => Ok(&ACT_CHANNEL_DELEGATION),
        (_, "humanDecidesOnChannel") => Err("no actuator: the decision channel is disconnected - no .github/workflows/decision-*.yml carries a decider's comment to the model; an acceptance is recorded from their quoted words instead (D0289)".to_string()),
        (_, "agentEditsDeliverable") => Ok(&ACT_HARNESS_FILE_TOOLS),
        (_, n) if n.starts_with("cmd") => Ok(match a.acts_on {
            "main-ref" => &ACT_GIT_PUSH,
            "enforcement-surface" => &ACT_SURFACE_REWRITE,
            _ => &ACT_WRITE_LAYER,
        }),
        _ => Err(format!("no actuator derived for {} - the view knows no mechanism for this action's shape", a.name)),
    }
}

/// Commands whose WRITE the write layer refuses for an AI-kind actor, so the issuing authority is the
/// human even though the agent's shell may type them. The claim is tested: see
/// `human_authority_commands_refuse_an_ai_actor` in the tests module.
const HUMAN_AUTHORITY_COMMANDS: [&str; 1] = ["accept"];

/// Which process a write command acts on, by family. A judgment made once here rather than per
/// command, and visible in the output as `actsOn` so it can be argued with.
fn process_for_family(family: &str, name: &str) -> &'static str {
    match family {
        "integration" => {
            if name == "init" || name == "sync-claude" {
                "enforcement-surface"
            } else {
                "main-ref"
            }
        }
        "distribution" => "enforcement-surface",
        "governance" if name == "claim" || name == "advance" => "work",
        _ => "model",
    }
}

/// Every `keel <token>` invocation in a text, where the token is a dispatched command.
fn keel_commands_in(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(i) = rest.find("keel ") {
            let after = &rest[i + 5..];
            let token: String = after.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-').collect();
            if crate::cli_surface::has_command(&token) && !out.contains(&token) {
                out.push(token.clone());
            }
            rest = after;
        }
    }
    out
}

fn hook_actions(root: &Path, out: &mut Vec<Action>) {
    let path = root.join(".claude").join("settings.json");
    let Ok(text) = std::fs::read_to_string(&path) else { return };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else { return };
    let Some(hooks) = v.get("hooks").and_then(|h| h.as_object()) else { return };
    let mut events: Vec<(&String, String)> = Vec::new();
    for (event, arr) in hooks {
        let mut subs = Vec::new();
        if let Some(items) = arr.as_array() {
            for it in items {
                for h in it.get("hooks").and_then(|x| x.as_array()).into_iter().flatten() {
                    if let Some(cmd) = h.get("command").and_then(|c| c.as_str()) {
                        if let Some(i) = cmd.find("hook ") {
                            let sub: String = cmd[i + 5..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-').collect();
                            if !sub.is_empty() && !subs.contains(&sub) {
                                subs.push(sub);
                            }
                        }
                    }
                }
            }
        }
        events.push((event, subs.join(", ")));
    }
    events.sort();
    // D0296: when the plugin rendering carries the same event, the action has two hosts - and a
    // settings edit cannot remove the second one, since hook lists merge across scopes.
    let plugin_rel = format!("{}/hooks/hooks.json", crate::claude_surface::PLUGIN_DIR);
    let plugin_events: Vec<String> = std::fs::read_to_string(root.join(&plugin_rel))
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v.get("hooks").and_then(|h| h.as_object()).map(|h| h.keys().cloned().collect()))
        .unwrap_or_default();
    for (event, subs) in events {
        // The verdict kind is a property of the event, from the dispatcher's contract: Stop and
        // PostToolUse can BLOCK, PreToolUse can DENY, the rest can only advise.
        let kind = match event.as_str() {
            "Stop" | "PostToolUse" | "SubagentStop" => "blocks",
            "PreToolUse" => "denies or advises",
            "ConfigChange" => "refuses and restores",
            _ => "advises",
        };
        out.push(Action {
            name: format!("hook{event}"),
            title: format!("{event}: {kind}"),
            issued_by: "hooks",
            acts_on: "agent-turn",
            data: format!("keel hook {subs}; verdict JSON the harness enforces"),
            source: if plugin_events.contains(event) { format!(".claude/settings.json; {plugin_rel}") } else { ".claude/settings.json".to_string() },
        });
    }
    // D0296 layer 2: the launch pin is a control action of its own - it acts on the SESSION's
    // settings precedence, not on a turn - present when a keel launch has written it here.
    if root.join(".keel").join("launch-settings.json").is_file() {
        out.push(Action {
            name: "launchPin".to_string(),
            title: "launch: disableAllHooks pinned false above project scope".to_string(),
            issued_by: "hooks",
            acts_on: "agent-turn",
            data: "--settings .keel/launch-settings.json; --plugin-dir the plugin rendering; KEEL_BIN".to_string(),
            source: ".keel/launch-settings.json".to_string(),
        });
    }
}

fn githook_actions(root: &Path, out: &mut Vec<Action>) {
    let dir = root.join(".githooks");
    let Ok(rd) = std::fs::read_dir(&dir) else { return };
    let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.is_file()).collect();
    files.sort();
    for f in files {
        let name = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        let cmds = keel_commands_in(&text);
        out.push(Action {
            name: format!("githook{}", camel(&name)),
            title: format!("{name}: refuse unless green"),
            issued_by: "commit-gate",
            acts_on: "main-ref",
            data: if cmds.is_empty() { "no keel command".to_string() } else { format!("keel {}", cmds.join(", keel ")) },
            source: format!(".githooks/{name}"),
        });
    }
}

fn workflow_actions(root: &Path, out: &mut Vec<Action>, fb: &mut Vec<Fb>) {
    let dir = root.join(".github").join("workflows");
    let Ok(rd) = std::fs::read_dir(&dir) else { return };
    let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "yml" || x == "yaml")).collect();
    files.sort();
    for f in files {
        let fname = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        let is_channel = fname.starts_with("decision-");
        let role: &'static str = if is_channel { "channel" } else { "ci" };
        let steps: Vec<String> = text
            .lines()
            .filter_map(|l| l.trim_start().strip_prefix("- name:"))
            .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
            .collect();
        let cmds = keel_commands_in(&text);
        out.push(Action {
            name: format!("workflow{}", camel(fname.trim_end_matches(".yml").trim_end_matches(".yaml"))),
            title: if is_channel { format!("{fname}: record on the model by delegation") } else { format!("{fname}: fail the build") },
            issued_by: role,
            acts_on: if is_channel { "model" } else { "main-ref" },
            data: format!("steps: {}; runs: keel {}", steps.join(" | "), if cmds.is_empty() { "-".to_string() } else { cmds.join(", keel ") }),
            source: format!(".github/workflows/{fname}"),
        });
        fb.push(Fb {
            name: format!("status{}", camel(fname.trim_end_matches(".yml").trim_end_matches(".yaml"))),
            title: format!("{fname}: run status"),
            sensed_from: if is_channel { "model" } else { "main-ref" },
            reports_to: "human",
            data: "red or green per run; email and `gh run list`; nothing makes the agent look (D0266)".to_string(),
            source: format!(".github/workflows/{fname}"),
        });
    }
}

fn cli_actions(root: &Path, out: &mut Vec<Action>, fb: &mut Vec<Fb>) {
    let path = root.join(".engine").join("cli").join("commands.sysml");
    let Ok(text) = std::fs::read_to_string(&path) else { return };
    for f in crate::guards::parse_cli_facts(&text) {
        let src = ".engine/cli/commands.sysml".to_string();
        match f.effect.as_str() {
            "writes" | "both" => {
                let issued_by = if HUMAN_AUTHORITY_COMMANDS.contains(&f.name.as_str()) { "human" } else { "agent" };
                out.push(Action {
                    name: format!("cmd{}", camel(&f.name)),
                    title: format!("keel {}: {}", f.name, f.synopsis),
                    issued_by,
                    acts_on: process_for_family(&f.family, &f.name),
                    data: format!("effect {}; family {}", f.effect, f.family),
                    source: src,
                });
            }
            "reads" => {
                fb.push(Fb {
                    name: format!("read{}", camel(&f.name)),
                    title: format!("keel {}{}: {}", if f.family == "lens" { "show " } else { "" }, f.name, f.synopsis),
                    sensed_from: "model",
                    reports_to: "agent",
                    data: format!("family {}", f.family),
                    source: src,
                });
            }
            _ => {}
        }
    }
}

/// The remote's rules, fetched live. Returns the JSON row and never caches into the tree.
fn remote_rules(root: &Path) -> Json {
    if std::env::var_os("KEEL_OFFLINE").is_some() {
        return Json::Obj(vec![("status".to_string(), Json::s("unverified: KEEL_OFFLINE set"))]);
    }
    let Some(slug) = github_slug(root) else {
        return Json::Obj(vec![("status".to_string(), Json::s("unverified: origin is not a GitHub remote"))]);
    };
    let out = Command::new("gh").args(["api", &format!("repos/{slug}/branches/main/protection")]).output();
    let Ok(o) = out else {
        return Json::Obj(vec![("status".to_string(), Json::s("unverified: gh not available"))]);
    };
    if !o.status.success() {
        return Json::Obj(vec![("status".to_string(), Json::s(format!("unverified: {}", String::from_utf8_lossy(&o.stderr).trim())))]);
    }
    let Ok(v) = serde_json::from_slice::<serde_json::Value>(&o.stdout) else {
        return Json::Obj(vec![("status".to_string(), Json::s("unverified: unparseable response"))]);
    };
    let force = v.pointer("/allow_force_pushes/enabled").and_then(serde_json::Value::as_bool);
    let admins = v.pointer("/enforce_admins/enabled").and_then(serde_json::Value::as_bool);
    let checks = v.get("required_status_checks").is_some_and(|c| !c.is_null());
    let reviews = v.get("required_pull_request_reviews").is_some_and(|c| !c.is_null());
    Json::Obj(vec![
        ("status".to_string(), Json::s(format!("fetched live from {slug}"))),
        ("refusesForcePush".to_string(), force.map_or(Json::Null, |b| Json::Bool(!b))),
        ("enforceAdmins".to_string(), admins.map_or(Json::Null, Json::Bool)),
        ("requiresStatusChecks".to_string(), Json::Bool(checks)),
        ("requiresReviews".to_string(), Json::Bool(reviews)),
        (
            "reading".to_string(),
            Json::s(if checks {
                "CI is preventive: a red build keeps a commit off main"
            } else {
                "CI is DETECTIVE: no status check is required, so a red build reports a commit already on main"
            }),
        ),
    ])
}

fn github_slug(root: &Path) -> Option<String> {
    let url = crate::gitx::git()
        .arg("-C")
        .arg(root)
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;
    let rest = url.strip_prefix("https://github.com/").or_else(|| url.strip_prefix("git@github.com:"))?;
    Some(rest.trim_end_matches(".git").trim_end_matches('/').to_string())
}

fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut up = true;
    for c in s.chars() {
        if c == '-' || c == '_' || c == '.' {
            up = true;
        } else if up {
            out.extend(c.to_uppercase());
            up = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// The computed control-action NAMES that need no network.
///
/// Everything `gather` finds except the remote's branch-protection row, which is fetched live. This
/// is what the `stpa-currency` guard (D0313) compares a run's `ANALYSED:` list against - a commit
/// gate must not call `gh`.
#[must_use]
pub fn local_action_names(root: &Path) -> Vec<String> {
    local_actions(root).into_iter().map(|a| a.name).collect()
}

/// One computed local control action with the edge it sits on: who issues it and what it acts on.
///
/// The `stpa-currency` guard groups its remainder by this edge (D0410): a tranche of the analysis is
/// one controller->process edge walked in full, so the guard names the edges still open rather than
/// one flat list of names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAction {
    pub name: String,
    pub issued_by: &'static str,
    pub acts_on: &'static str,
}

/// The computed local control actions with their edges, in structure order (see `local_action_names`).
#[must_use]
pub fn local_actions(root: &Path) -> Vec<LocalAction> {
    let mut actions = Vec::new();
    let mut feedback = Vec::new();
    gather_local(root, &mut actions, &mut feedback);
    actions.into_iter().map(|a| LocalAction { name: a.name, issued_by: a.issued_by, acts_on: a.acts_on }).collect()
}

/// The structural, derivable half: every action and feedback path, from facts.
fn gather(root: &Path) -> (Vec<Action>, Vec<Fb>, Json) {
    let mut actions = Vec::new();
    let mut feedback = Vec::new();
    gather_local(root, &mut actions, &mut feedback);
    let remote = remote_rules(root);
    if let Json::Obj(fields) = &remote {
        if fields.iter().any(|(k, v)| k == "refusesForcePush" && matches!(v, Json::Bool(true))) {
            actions.push(Action {
                name: "remoteRefusesRewrite".to_string(),
                title: "the remote refuses a history rewrite".to_string(),
                issued_by: "remote",
                acts_on: "main-ref",
                data: "force-push and deletion rejected; requires no status check and no review unless the row says otherwise".to_string(),
                source: "branch protection, fetched live".to_string(),
            });
        }
    }
    (actions, feedback, remote)
}

/// Every control action and feedback path computable from the TREE alone.
/// The propriety lenses that have a finding for `instrument:<name>`, with their verdicts.
///
/// COMPUTED, replacing a stored `assessed` field the manifest carried for one day. The owner's
/// correction: a status field can contradict the findings it summarises, and when it does the field
/// wins because it is what a reader sees. This cannot drift - it IS the findings.
///
/// STATED LIMITATION: `ProprietyFinding::target` is a String, so an assessment naming an instrument
/// that no longer exists still counts here. A typed `#Verify` edge would refuse that, and cannot
/// reach a target declared in a contract file (issue395).
fn propriety_of(model: &Model, name: &str) -> Json {
    let key = format!("instrument:{name}");
    let mut rows: Vec<(String, String)> = model
        .items
        .values()
        .filter(|i| i.type_name == "ProprietyFinding")
        .filter(|i| i.attrs.get("target").is_some_and(|t| t == &key))
        .map(|i| {
            (
                i.attrs.get("lens").cloned().unwrap_or_default(),
                i.attrs.get("verdict").cloned().unwrap_or_default(),
            )
        })
        .collect();
    rows.sort();
    if rows.is_empty() {
        return Json::s("UNASSESSED - no propriety finding names this instrument");
    }
    Json::Arr(
        rows.into_iter()
            .map(|(lens, verdict)| {
                Json::Obj(vec![
                    ("lens".to_string(), Json::s(lens)),
                    ("verdict".to_string(), Json::s(verdict)),
                ])
            })
            .collect(),
    )
}

/// The `mechanism` path of every declared `Sensor` - what guard 65 holds the tree against (D0363).
///
/// An accessor rather than a public `Model`: the guard needs one field of one type, and widening the
/// model's visibility so a guard can walk it would trade encapsulation for one caller.
#[must_use]
pub fn declared_sensor_mechanisms(root: &Path) -> Vec<String> {
    let Ok(model) = Model::build(root) else { return Vec::new() };
    let mut out: Vec<String> = model
        .items
        .values()
        .filter(|i| i.type_name == "Sensor")
        .filter_map(|i| i.attrs.get("mechanism").cloned())
        .collect();
    out.sort();
    out
}

/// The authored measures as FEEDBACK paths and SENSORS (D0363, Route A of issue395).
///
/// WHY THE MODEL AND NOT A FILE. Feedback here is otherwise derived from `CliCommand` facts and
/// workflow statuses, so a measure that is not a CLI command could not appear - measured 2026-09-06,
/// none of twenty did, while six defects in one day landed in exactly those channels. They were first
/// declared in a contract file, which got them into this view and left them unverifiable: a verify
/// edge needs an endpoint and a contract file holds none. They are now `Sensor` and `Feedback` items,
/// so a verification case can point at one and `edge-endpoints` refuses a reference that does not
/// resolve.
///
/// The sensor's DETERMINISM travels with the row, because "one run of this is a measurement" and "one
/// run of this is a sample" are different facts about a number and nothing else says which (issue392).
fn instrument_feedback(model: &Model, feedback: &mut Vec<Fb>) -> Vec<Json> {
    let mut sensors: Vec<(String, Json)> = Vec::new();
    for (name, item) in &model.items {
        if item.type_name != "Sensor" {
            continue;
        }
        let attr = |k: &str| item.attrs.get(k).cloned().unwrap_or_default();
        let produces = attr("producesFeedback");
        let (mut sensed, mut reports) = ("", "");
        if let Some(fb) = model.items.get(&produces) {
            let from = fb.attrs.get("sensedFrom").cloned().unwrap_or_default();
            let to = fb.attrs.get("reportsTo").cloned().unwrap_or_default();
            sensed = role_of_process_anchor(&from);
            reports = role_of_controller_anchor(&to);
        }
        let determinism = attr("determinism").rsplit("::").next().unwrap_or_default().to_string();
        let measures = attr("measures");
        if !sensed.is_empty() && !reports.is_empty() {
            feedback.push(Fb {
                name: item.attrs.get("title").cloned().unwrap_or_else(|| name.clone()),
                title: format!("{}: {measures}", attr("mechanism")),
                sensed_from: sensed,
                reports_to: reports,
                data: format!("{measures}; determinism: {determinism}"),
                source: attr("mechanism"),
            });
        }
        let label = item.attrs.get("title").cloned().unwrap_or_else(|| name.clone());
        sensors.push((
            label.clone(),
            Json::Obj(vec![
                ("name".to_string(), Json::s(label.clone())),
                ("item".to_string(), Json::s(name.clone())),
                ("mechanism".to_string(), Json::s(attr("mechanism"))),
                ("producesFeedback".to_string(), Json::s(produces)),
                ("determinism".to_string(), Json::s(determinism)),
                ("reading".to_string(), Json::s(attr("reading").rsplit("::").next().unwrap_or_default().to_string())),
                ("propriety".to_string(), propriety_of(model, &label)),
            ]),
        ));
    }
    sensors.sort_by(|a, b| a.0.cmp(&b.0));
    sensors.into_iter().map(|(_, row)| row).collect()
}

fn gather_local(root: &Path, actions: &mut Vec<Action>, feedback: &mut Vec<Fb>) {
    hook_actions(root, actions);
    githook_actions(root, actions);
    workflow_actions(root, actions, feedback);
    cli_actions(root, actions, feedback);
    // The human's direction is a control action with no receiving control; it exists whenever a
    // project has an intake path, which every project on this vintage has.
    actions.push(Action {
        name: "humanDirects".to_string(),
        title: "the human gives direction".to_string(),
        issued_by: "human",
        acts_on: "work",
        data: "prose: chat, a Statement recorded verbatim, a direction Decision; nothing parses it - the agent's routing is the only translation (D0166)".to_string(),
        source: "keel record statement (intake)".to_string(),
    });
    let deciders = crate::github::deciders(root);
    if !deciders.is_empty() {
        actions.push(Action {
            name: "humanDecidesOnChannel".to_string(),
            title: "a declared login accepts or rejects on the decision channel".to_string(),
            issued_by: "human",
            acts_on: "model",
            data: format!("logins: {}", deciders.keys().cloned().collect::<Vec<_>>().join(", ")),
            source: ".engine/contracts/github-actors.toml".to_string(),
        });
    }
    // The console's authority exists when `serve` does: the approve queue is where a human authorises
    // an ask-tier write (D0182), and the deck is where they judge.
    if crate::cli_surface::has_command("serve") {
        actions.push(Action {
            name: "consoleApprovesWrite".to_string(),
            title: "a human approves an ask-tier write from the console queue".to_string(),
            issued_by: "console",
            acts_on: "model",
            data: "approval of a queued write: path, requesting run, approver; recorded as an obligation, not a click".to_string(),
            source: "keel serve (approve queue, D0182)".to_string(),
        });
        feedback.push(Fb {
            name: "consoleLenses".to_string(),
            title: "console lenses and approve queue".to_string(),
            sensed_from: "model",
            reports_to: "human",
            data: "#View renders on 127.0.0.1:7777; a pull audit, nothing owed (D0204)".to_string(),
            source: "keel serve".to_string(),
        });
    }
    // The deliverable is acted on by NO keel command - the agent edits source with its own tools - and
    // that is a control action all the same (issue394: for a week the view said so only here, so the
    // deliverable read as a process nobody acts on and EHZ9 had no responsible controller). What keel
    // has of it is feedback: manifest drift.
    if root.join(".engine").join("deliverable-manifest.txt").is_file() {
        actions.push(Action {
            name: "agentEditsDeliverable".to_string(),
            title: "the agent edits the deliverable source with its own tools".to_string(),
            issued_by: "agent",
            acts_on: "deliverable",
            data: "Write, Edit and Bash on the paths the manifest names; keel mediates none of it - pre-write advises on protected surfaces, drift makes done work suspect afterwards".to_string(),
            source: ".engine/deliverable-manifest.txt".to_string(),
        });
        feedback.push(Fb {
            name: "deliverableDrift".to_string(),
            title: "deliverable drift makes done work suspect".to_string(),
            sensed_from: "deliverable",
            reports_to: "agent",
            data: "source listed in the manifest changed since a task's verified commit -> suspect (orient)".to_string(),
            source: ".engine/deliverable-manifest.txt".to_string(),
        });
    }
}

/// One authored process model, with the role of the controller that holds it.
struct Pm {
    name: String,
    role: &'static str,
    row: Json,
}

/// The authored decoration: process models and hazard -> process edges (hazard name, process
/// anchor), from the model.
fn decoration(model: &Model) -> (Vec<Pm>, Vec<(String, String)>) {
    let mut pmodels: Vec<Pm> = Vec::new();
    for (n, i) in &model.items {
        if i.type_name != "ProcessModel" {
            continue;
        }
        let beliefs = i.attrs.get("beliefs").cloned().unwrap_or_default();
        let held = i.attrs.get("heldBy").cloned().unwrap_or_default();
        let cites: Vec<Json> = beliefs
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|w| w.starts_with("issue") && w.len() > 5 && w[5..].chars().all(|c| c.is_ascii_digit()))
            .map(Json::s)
            .collect();
        pmodels.push(Pm {
            name: n.clone(),
            role: role_of_controller_anchor(&held),
            row: Json::Obj(vec![
                ("name".to_string(), Json::s(n.clone())),
                ("heldBy".to_string(), Json::s(held)),
                ("beliefs".to_string(), Json::s(beliefs)),
                ("falseBeliefsCite".to_string(), Json::Arr(cites)),
            ]),
        });
    }
    pmodels.sort_by(|a, b| a.name.cmp(&b.name));
    let mut hazard_rows: Vec<(String, String)> = Vec::new();
    for e in &model.edges {
        if model.items.get(&e.from).is_some_and(|i| i.type_name == "Hazard") && model.items.get(&e.to).is_some_and(|i| i.type_name == "ControlledProcess") {
            hazard_rows.push((e.from.clone(), e.to.clone()));
        }
    }
    hazard_rows.sort();
    hazard_rows.dedup();
    (pmodels, hazard_rows)
}

/// One authored other-input-or-output (the handbook's fifth element type, D0363), with the role it
/// enters or leaves at.
struct Oio {
    role: &'static str,
    inbound: bool,
    row: Json,
}

fn other_inputs_outputs(model: &Model) -> Vec<Oio> {
    let mut out: Vec<(String, Oio)> = Vec::new();
    for (n, i) in &model.items {
        if i.type_name != "OtherInputOutput" {
            continue;
        }
        let attr = |k: &str| i.attrs.get(k).cloned().unwrap_or_default();
        let direction = attr("direction").rsplit("::").next().unwrap_or_default().to_string();
        let at_controller = attr("atController");
        let at_process = attr("atProcess");
        let role = role_of_controller_anchor(&at_controller);
        out.push((
            n.clone(),
            Oio {
                role,
                inbound: direction == "inbound",
                row: Json::Obj(vec![
                    ("name".to_string(), Json::s(n.clone())),
                    ("title".to_string(), Json::s(attr("title"))),
                    ("direction".to_string(), Json::s(direction)),
                    ("externalParty".to_string(), Json::s(attr("externalParty"))),
                    ("atController".to_string(), if role.is_empty() { Json::Null } else { Json::s(role) }),
                    ("atProcess".to_string(), if at_process.is_empty() { Json::Null } else { Json::s(role_of_process_anchor(&at_process)) }),
                ]),
            },
        ));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out.into_iter().map(|(_, o)| o).collect()
}

/// One computed responsibility: a controller answers for a hazard because it acts on the process the
/// hazard is a state of - directly, or through a process another controller enacts.
struct Responsibility {
    controller: &'static str,
    process: &'static str,
    hazard: String,
    via: String,
}

fn responsibilities(actions: &[Action], hazard_rows: &[(String, String)]) -> Vec<Responsibility> {
    let hazards_on = |process: &str| -> Vec<String> { hazard_rows.iter().filter(|(_, p)| role_of_process_anchor(p) == process).map(|(h, _)| h.clone()).collect() };
    let mut out: Vec<Responsibility> = Vec::new();
    let mut push = |controller: &'static str, process: &'static str, hazard: String, via: String| {
        if !out.iter().any(|r| r.controller == controller && r.process == process && r.hazard == hazard) {
            out.push(Responsibility { controller, process, hazard, via });
        }
    };
    for a in actions {
        for h in hazards_on(a.acts_on) {
            push(a.issued_by, a.acts_on, h, format!("acts on {}", a.acts_on));
        }
        if let Some(enactor) = PROCESSES.iter().find(|(r, _, _, _)| *r == a.acts_on).and_then(|(_, _, _, e)| *e) {
            for b in actions.iter().filter(|b| b.issued_by == enactor) {
                for h in hazards_on(b.acts_on) {
                    push(a.issued_by, b.acts_on, h, format!("acts on {}, which the {enactor} enacts", a.acts_on));
                }
            }
        }
    }
    out.sort_by(|a, b| (a.controller, a.process, &a.hazard).cmp(&(b.controller, b.process, &b.hazard)));
    out
}

fn gate_row(clause: &str, holds: bool, evidence: String) -> Json {
    Json::Obj(vec![("clause".to_string(), Json::s(clause)), ("holds".to_string(), Json::Bool(holds)), ("evidence".to_string(), Json::s(evidence))])
}

fn join_or_none(v: &[String]) -> String {
    if v.is_empty() {
        "none".to_string()
    } else {
        v.join(", ")
    }
}

/// The SOP's step-2 gate (`.engine/skills/stpa/references/sop.md`, STEP 2), one row per clause,
/// each decided from the computed structure - never from prose. `present` is the roles in this
/// project's structure; `wired` those with at least one action.
#[allow(clippy::too_many_arguments)]
fn step_two_gate(
    present: &[&'static str],
    wired: &[&'static str],
    actions: &[Action],
    feedback: &[Fb],
    pmodels: &[Pm],
    oios: &[Oio],
    resp: &[Responsibility],
    hazard_rows: &[(String, String)],
    sensors: usize,
    actuators: usize,
) -> Vec<Json> {
    let mut rows = Vec::new();
    let processes_acted: Vec<&str> = PROCESSES.iter().filter(|(r, _, _, _)| actions.iter().any(|a| a.acts_on == *r)).map(|(r, _, _, _)| *r).collect();
    rows.push(gate_row(
        "at least one controller, one control action and one controlled process",
        !wired.is_empty() && !actions.is_empty() && !processes_acted.is_empty(),
        format!("{} controllers issue {} actions on {} processes", wired.len(), actions.len(), processes_acted.len()),
    ));
    let generic = ["command", "status", "computer", "data", "signal"];
    let bare: Vec<String> = actions
        .iter()
        .map(|a| (&a.name, &a.title, &a.data))
        .chain(feedback.iter().map(|f| (&f.name, &f.title, &f.data)))
        .filter(|(_, t, d)| d.trim().is_empty() || t.split_whitespace().count() < 2 || generic.contains(&t.trim().to_ascii_lowercase().as_str()))
        .map(|(n, _, _)| n.clone())
        .collect();
    rows.push(gate_row(
        "every arrow is labelled functionally with what passes (no bare Command / Status / Computer)",
        bare.is_empty(),
        format!("{} actions and {} feedback paths carry a title and data; bare: {}", actions.len(), feedback.len(), join_or_none(&bare)),
    ));
    let unacted: Vec<String> = PROCESSES.iter().filter(|(r, _, _, _)| !processes_acted.contains(r)).map(|(r, _, _, _)| (*r).to_string()).collect();
    rows.push(gate_row(
        "every controlled process has at least one controller acting on it",
        unacted.is_empty(),
        format!("acted on: {}; with no controller: {}", processes_acted.join(", "), join_or_none(&unacted)),
    ));
    let without_resp: Vec<String> = wired.iter().filter(|r| !resp.iter().any(|x| x.controller == **r)).map(|r| (*r).to_string()).collect();
    let orphan_hazards: Vec<String> = hazard_rows.iter().filter(|(h, _)| !resp.iter().any(|x| x.hazard == *h)).map(|(h, _)| h.clone()).collect();
    rows.push(gate_row(
        "responsibilities are traced: every acting controller answers for a hazard on a process it reaches, and every hazard has a controller that answers for it",
        without_resp.is_empty() && orphan_hazards.is_empty(),
        format!(
            "{} responsibilities over {} hazards; controllers with none: {}; hazards nobody answers for: {}",
            resp.len(),
            hazard_rows.iter().map(|(h, _)| h).collect::<std::collections::BTreeSet<_>>().len(),
            join_or_none(&without_resp),
            join_or_none(&orphan_hazards)
        ),
    ));
    let without_pm: Vec<String> = present.iter().filter(|r| !pmodels.iter().any(|p| p.role == **r)).map(|r| (*r).to_string()).collect();
    rows.push(gate_row(
        "every controller has a process model, or its absence is justified",
        without_pm.is_empty(),
        format!("{} process models for {} present controllers; without one: {}", pmodels.len(), present.len(), join_or_none(&without_pm)),
    ));
    let unmaintained: Vec<String> = pmodels
        .iter()
        .filter(|p| !feedback.iter().any(|f| f.reports_to == p.role) && !oios.iter().any(|o| o.inbound && o.role == p.role))
        .map(|p| p.name.clone())
        .collect();
    rows.push(gate_row(
        "each process model is maintained by feedback, or by an other-input, reaching its controller",
        unmaintained.is_empty(),
        format!("maintained by nothing: {}", join_or_none(&unmaintained)),
    ));
    let (inbound, outbound) = (oios.iter().filter(|o| o.inbound).count(), oios.iter().filter(|o| !o.inbound).count());
    rows.push(gate_row(
        "the fifth element type - other inputs and outputs, neither control nor feedback - is enumerated or declared empty with a reason",
        !oios.is_empty(),
        if oios.is_empty() {
            "none authored: no OtherInputOutput item exists, and nothing declares the set empty".to_string()
        } else {
            format!("{inbound} inbound, {outbound} outbound, each with its external party")
        },
    ));
    rows.push(gate_row(
        "sensors and actuators are present or deliberately deferred to step 4",
        true,
        format!("{sensors} sensors authored, {actuators} actuators computed from the actions"),
    ));
    rows
}

/// The actuators: one per action (or the stated reason there is none), deduplicated by name into an
/// array that names the controllers and processes each carries between.
fn actuator_rows(actions: &[Action], channel_wired: bool) -> Vec<Json> {
    let mut out: Vec<Json> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for a in actions {
        let Ok(act) = actuator_for(a, channel_wired) else { continue };
        if seen.contains(&act.name) {
            continue;
        }
        seen.push(act.name);
        let carried: Vec<&Action> = actions.iter().filter(|b| actuator_for(b, channel_wired).is_ok_and(|x| x.name == act.name)).collect();
        let mut ctls: Vec<&str> = carried.iter().map(|b| b.issued_by).collect();
        ctls.sort_unstable();
        ctls.dedup();
        let mut procs: Vec<&str> = carried.iter().map(|b| b.acts_on).collect();
        procs.sort_unstable();
        procs.dedup();
        out.push(Json::Obj(vec![
            ("name".to_string(), Json::s(act.name)),
            ("title".to_string(), Json::s(act.title)),
            ("mechanism".to_string(), Json::s(act.mechanism)),
            ("source".to_string(), Json::s(act.source)),
            ("controllers".to_string(), Json::Arr(ctls.into_iter().map(Json::s).collect())),
            ("processes".to_string(), Json::Arr(procs.into_iter().map(Json::s).collect())),
            ("carries".to_string(), Json::Arr(carried.iter().map(|b| Json::s(b.name.clone())).collect())),
        ]));
    }
    out
}

/// One present controller's fields, minus the anchor decoration and the responsibilities the caller
/// joins on. `processModelAbsent` carries the reason - the anchor a `ProcessModel.heldBy` would name.
fn controller_row(role: &'static str, anchor_name: &str, what: &str, actions: &[Action], feedback: &[Fb], pmodels: &[Pm], channel_wired: bool) -> Vec<(String, Json)> {
    let acts: Vec<Json> = actions.iter().filter(|a| a.issued_by == role).map(|a| Json::s(a.name.clone())).collect();
    let fbs: Vec<Json> = feedback.iter().filter(|f| f.reports_to == role).map(|f| Json::s(f.name.clone())).collect();
    let mut acts_of: Vec<&str> = actions.iter().filter(|a| a.issued_by == role).filter_map(|a| actuator_for(a, channel_wired).ok().map(|x| x.name)).collect();
    acts_of.sort_unstable();
    acts_of.dedup();
    let pm = pmodels.iter().find(|p| p.role == role);
    vec![
        ("role".to_string(), Json::s(role)),
        ("what".to_string(), Json::s(what)),
        ("inert".to_string(), Json::Bool(acts.is_empty())),
        ("actions".to_string(), Json::Arr(acts)),
        ("feedback".to_string(), Json::Arr(fbs)),
        ("actuators".to_string(), Json::Arr(acts_of.into_iter().map(Json::s).collect())),
        ("processModel".to_string(), pm.map_or(Json::Null, |p| Json::s(p.name.clone()))),
        ("processModelAbsent".to_string(), pm.map_or_else(|| Json::s(format!("no ProcessModel with heldBy = {anchor_name} is authored")), |_| Json::Null)),
    ]
}

fn anchor_of(model: &Model, name: &str) -> (Json, Json) {
    model
        .items
        .get(name)
        .filter(|i| i.type_name == "Controller" || i.type_name == "ControlledProcess")
        .map_or((Json::Null, Json::Null), |i| (Json::s(name), i.attrs.get("title").map_or(Json::Null, |t| Json::s(t.clone()))))
}

fn action_row(a: &Action) -> Json {
    Json::Obj(vec![
        ("name".to_string(), Json::s(a.name.clone())),
        ("title".to_string(), Json::s(a.title.clone())),
        ("issuedBy".to_string(), Json::s(a.issued_by)),
        ("actsOn".to_string(), Json::s(a.acts_on)),
        ("data".to_string(), Json::s(a.data.clone())),
        ("source".to_string(), Json::s(a.source.clone())),
    ])
}

fn fb_row(f: &Fb) -> Json {
    Json::Obj(vec![
        ("name".to_string(), Json::s(f.name.clone())),
        ("title".to_string(), Json::s(f.title.clone())),
        ("sensedFrom".to_string(), Json::s(f.sensed_from)),
        ("reportsTo".to_string(), Json::s(f.reports_to)),
        ("data".to_string(), Json::s(f.data.clone())),
        ("source".to_string(), Json::s(f.source.clone())),
    ])
}

/// The whole structure as JSON.
///
/// # Errors
/// Returns the model's own error when the tree does not parse; the derivable half never fails - an
/// absent source (no hooks, no workflows, no CLI facts) contributes nothing and the role reads inert.
pub fn control_structure(root: &Path) -> Result<String, ViewError> {
    let model = Model::build(root)?;
    let (actions, mut feedback, remote) = gather(root);
    // The instruments: feedback paths the CLI facts cannot express, plus the sensors that produce
    // them - the first Sensor items this model has ever had (D0361, STPA Handbook pp.25-26).
    let sensors = instrument_feedback(&model, &mut feedback);
    let (pmodels, hazard_rows) = decoration(&model);
    let oios = other_inputs_outputs(&model);
    let resp = responsibilities(&actions, &hazard_rows);
    let channel_wired = actions.iter().any(|a| a.issued_by == "channel");
    // A role is PRESENT when something wires it: an action it issues, a feedback path it receives, or
    // an authored anchor. Anything else is not an inert controller - it is a controller this project
    // does not have, reported once with what would wire it (issue394: `channel` sat inert for a week).
    let present: Vec<&'static str> = ROLES
        .iter()
        .filter(|(role, anchor, _, _)| {
            actions.iter().any(|a| a.issued_by == *role) || feedback.iter().any(|f| f.reports_to == *role) || model.items.get(*anchor).is_some_and(|i| i.type_name == "Controller")
        })
        .map(|(role, _, _, _)| *role)
        .collect();
    let wired: Vec<&'static str> = present.iter().copied().filter(|r| actions.iter().any(|a| a.issued_by == *r)).collect();
    let absent: Vec<Json> = ROLES
        .iter()
        .filter(|(role, _, _, _)| !present.contains(role))
        .map(|(role, _, what, wired_by)| Json::Obj(vec![("role".to_string(), Json::s(*role)), ("what".to_string(), Json::s(*what)), ("wiredBy".to_string(), Json::s(*wired_by))]))
        .collect();
    let actuators = actuator_rows(&actions, channel_wired);
    let resp_row = |r: &Responsibility| {
        Json::Obj(vec![
            ("controller".to_string(), Json::s(r.controller)),
            ("process".to_string(), Json::s(r.process)),
            ("hazard".to_string(), Json::s(r.hazard.clone())),
            ("hazardTitle".to_string(), model.items.get(&r.hazard).and_then(|i| i.attrs.get("title")).map_or(Json::Null, |t| Json::s(t.clone()))),
            ("via".to_string(), Json::s(r.via.clone())),
        ])
    };
    let controllers: Vec<Json> = ROLES
        .iter()
        .filter(|(role, _, _, _)| present.contains(role))
        .map(|(role, anchor_name, what, _)| {
            let (anchor, title) = anchor_of(&model, anchor_name);
            let mut fields = controller_row(role, anchor_name, what, &actions, &feedback, &pmodels, channel_wired);
            fields.insert(2, ("anchor".to_string(), anchor));
            fields.insert(3, ("anchorTitle".to_string(), title));
            fields.push(("responsibilities".to_string(), Json::Arr(resp.iter().filter(|r| r.controller == *role).map(resp_row).collect())));
            Json::Obj(fields)
        })
        .collect();
    let processes: Vec<Json> = PROCESSES
        .iter()
        .map(|(role, anchor_name, what, enacted_by)| {
            let (anchor, title) = anchor_of(&model, anchor_name);
            let acted_on = actions.iter().filter(|a| a.acts_on == *role).count();
            Json::Obj(vec![
                ("role".to_string(), Json::s(*role)),
                ("what".to_string(), Json::s(*what)),
                ("anchor".to_string(), anchor),
                ("anchorTitle".to_string(), title),
                ("actionsOnIt".to_string(), Json::Int(i64::try_from(acted_on).unwrap_or(0))),
                ("enactedBy".to_string(), enacted_by.map_or(Json::Null, Json::s)),
            ])
        })
        .collect();
    let action_rows: Vec<Json> = actions
        .iter()
        .map(|a| {
            let Json::Obj(mut fields) = action_row(a) else { unreachable!("action_row is an object") };
            match actuator_for(a, channel_wired) {
                Ok(x) => fields.push(("actuator".to_string(), Json::s(x.name))),
                Err(why) => {
                    fields.push(("actuator".to_string(), Json::Null));
                    fields.push(("actuatorAbsent".to_string(), Json::s(why)));
                }
            }
            Json::Obj(fields)
        })
        .collect();
    let inert: Vec<Json> = present.iter().filter(|r| !wired.contains(r)).map(|r| Json::s(*r)).collect();
    let gate = step_two_gate(&present, &wired, &actions, &feedback, &pmodels, &oios, &resp, &hazard_rows, sensors.len(), actuators.len());
    let hazard_json: Vec<Json> = hazard_rows
        .iter()
        .map(|(h, p)| Json::Obj(vec![("hazard".to_string(), Json::s(h.clone())), ("process".to_string(), Json::s(p.clone()))]))
        .collect();
    Ok(Json::Obj(vec![
        ("control-structure".to_string(), Json::s("STPA step 2 for this project's own workflow, COMPUTED from the authored measures (Sensor and Feedback items - the mechanism, its determinism and the loop it closes), the hook config, git hooks, workflow files, CLI facts and declared deciders (D0284). Actuators and responsibilities are derived from the actions; process models and the other inputs/outputs (the handbook's fifth element type, D0363) are authored and joined onto the roles; the step-2 gate is decided clause by clause from this structure; the remote's rules are fetched live and never copied.")),
        ("controllers".to_string(), Json::Arr(controllers)),
        ("processes".to_string(), Json::Arr(processes)),
        ("actions".to_string(), Json::Arr(action_rows)),
        ("feedback".to_string(), Json::Arr(feedback.iter().map(fb_row).collect())),
        ("sensors".to_string(), Json::Arr(sensors)),
        ("actuators".to_string(), Json::Arr(actuators)),
        ("processModels".to_string(), Json::Arr(pmodels.into_iter().map(|p| p.row).collect())),
        ("otherInputsOutputs".to_string(), Json::Arr(oios.into_iter().map(|o| o.row).collect())),
        ("responsibilities".to_string(), Json::Arr(resp.iter().map(resp_row).collect())),
        ("hazardsByProcess".to_string(), Json::Arr(hazard_json)),
        ("remote".to_string(), remote),
        ("inertControllers".to_string(), Json::Arr(inert)),
        ("absentRoles".to_string(), Json::Arr(absent)),
        ("stepTwoGate".to_string(), Json::Arr(gate)),
    ])
    .dump())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keel_commands_are_found_in_a_hook_body_and_unknown_tokens_are_not() {
        let text = "#!/bin/sh\nkeel validate . && keel guard\nkeel frobnicate\necho keel";
        assert_eq!(keel_commands_in(text), vec!["validate".to_string(), "guard".to_string()]);
    }

    fn act(name: &str, issued_by: &'static str, acts_on: &'static str) -> Action {
        Action { name: name.to_string(), title: format!("{name} does a thing"), issued_by, acts_on, data: "what passes".to_string(), source: "test".to_string() }
    }

    /// The actuator is a judgment made once, by the shape of the action: hooks act through the harness's
    /// verdict, git hooks through git's dispatch, a write command through the write layer unless it
    /// pushes or rewrites a surface, and the human's channel decision has NO actuator while the channel
    /// is disconnected - the reason travels with the row rather than a null.
    #[test]
    fn actuators_follow_the_action_shape_and_a_disconnected_channel_has_none() {
        let a = |n, by, on| actuator_for(&act(n, by, on), false).map(|x| x.name);
        assert_eq!(a("hookStop", "hooks", "agent-turn"), Ok("actHarnessVerdict"));
        assert_eq!(a("launchPin", "hooks", "agent-turn"), Ok("actLaunchSettings"));
        assert_eq!(a("githookPreCommit", "commit-gate", "main-ref"), Ok("actGitHookDispatch"));
        assert_eq!(a("workflowCi", "ci", "main-ref"), Ok("actCheckConclusion"));
        assert_eq!(a("remoteRefusesRewrite", "remote", "main-ref"), Ok("actRefUpdateRefusal"));
        assert_eq!(a("consoleApprovesWrite", "console", "model"), Ok("actServeWriteApi"));
        assert_eq!(a("humanDirects", "human", "work"), Ok("actAgentRouting"));
        assert_eq!(a("cmdAddTask", "agent", "model"), Ok("actWriteLayer"));
        assert_eq!(a("cmdLand", "agent", "main-ref"), Ok("actGitPush"));
        assert_eq!(a("cmdSyncClaude", "agent", "enforcement-surface"), Ok("actSurfaceRewrite"));
        assert_eq!(a("agentEditsDeliverable", "agent", "deliverable"), Ok("actHarnessFileTools"));
        let why = a("humanDecidesOnChannel", "human", "model").expect_err("disconnected channel");
        assert!(why.contains("disconnected") && why.contains("D0289"), "{why}");
        assert_eq!(actuator_for(&act("humanDecidesOnChannel", "human", "model"), true).map(|x| x.name), Ok("actChannelDelegation"));
        assert!(a("mystery", "agent", "model").is_err());
    }

    /// Responsibilities are computed one hierarchical level deep: a controller acting on a process
    /// answers for its hazards; a controller acting on the AGENT'S TURN answers for what the agent's
    /// own actions can reach, because the turn is the agent acting.
    #[test]
    fn responsibilities_descend_one_level_through_the_agent_turn() {
        let actions = vec![act("hookStop", "hooks", "agent-turn"), act("cmdAddTask", "agent", "model"), act("humanDirects", "human", "work")];
        let hazards = vec![("ehzModel".to_string(), "cpModel".to_string()), ("ehzWork".to_string(), "cpWork".to_string())];
        let r = responsibilities(&actions, &hazards);
        let has = |c: &str, h: &str| r.iter().any(|x| x.controller == c && x.hazard == h);
        assert!(has("agent", "ehzModel") && has("hooks", "ehzModel") && has("human", "ehzWork"));
        assert!(!has("hooks", "ehzWork"), "the hooks do not reach what the human acts on");
        assert!(r.iter().find(|x| x.controller == "hooks").expect("hooks row").via.contains("agent enacts"));
    }

    fn gate_holds(rows: &[Json]) -> Vec<(String, bool)> {
        rows.iter()
            .map(|r| {
                let Json::Obj(f) = r else { panic!("row") };
                let clause = f.iter().find(|(k, _)| k == "clause").map(|(_, v)| v.dump()).unwrap_or_default();
                let holds = f.iter().any(|(k, v)| k == "holds" && matches!(v, Json::Bool(true)));
                (clause, holds)
            })
            .collect()
    }

    /// The step-2 gate decides each clause from the structure. A controller with no process model, a
    /// process nobody acts on, a hazard nobody answers for, and an empty fifth element type each fail
    /// exactly their own clause - and the evidence names the offender.
    #[test]
    fn the_step_two_gate_fails_the_clause_the_structure_breaks() {
        let actions = vec![act("cmdAddTask", "agent", "model"), act("humanDirects", "human", "work")];
        let feedback = vec![Fb { name: "readOrient".to_string(), title: "keel orient: where things stand".to_string(), sensed_from: "model", reports_to: "agent", data: "d".to_string(), source: "s".to_string() }];
        let pm = |n: &str, role: &'static str| Pm { name: n.to_string(), role, row: Json::Null };
        let pmodels = vec![pm("pmAgent", "agent"), pm("pmHuman", "human")];
        let oio = |role: &'static str, inbound: bool| Oio { role, inbound, row: Json::Null };
        let oios = vec![oio("human", true)];
        let hazards = vec![("ehzModel".to_string(), "cpModel".to_string())];
        let resp = responsibilities(&actions, &hazards);
        let present = ["human", "agent"];
        let rows = step_two_gate(&present, &present, &actions, &feedback, &pmodels, &oios, &resp, &hazards, 0, 1);
        assert_eq!(rows.len(), 8);
        let holds = gate_holds(&rows);
        // the human acts on `work`, which carries no hazard, so the human answers for nothing
        let failing: Vec<&str> = holds.iter().filter(|(_, h)| !h).map(|(c, _)| c.as_str()).collect();
        assert_eq!(failing.len(), 2, "{failing:?}");
        assert!(failing[0].contains("every controlled process has at least one controller"), "{failing:?}");
        assert!(failing[1].contains("responsibilities are traced"), "{failing:?}");
        let text = Json::Arr(rows).dump();
        assert!(text.contains("controllers with none: human"), "{text}");
        assert!(text.contains("with no controller: main-ref, enforcement-surface, deliverable, agent-turn"), "{text}");
        // now break the process-model clauses and the fifth type
        let rows = step_two_gate(&present, &present, &actions, &feedback, &pmodels[..1], &[], &resp, &hazards, 0, 1);
        let text = Json::Arr(rows).dump();
        assert!(text.contains("without one: human"), "{text}");
        assert!(text.contains("maintained by nothing: none"), "the agent's model is fed by readOrient: {text}");
        assert!(text.contains("none authored: no OtherInputOutput item exists"), "{text}");
        let rows = step_two_gate(&present, &present, &actions, &[], &pmodels, &oios, &resp, &hazards, 0, 1);
        assert!(Json::Arr(rows).dump().contains("maintained by nothing: pmAgent"), "no feedback and no inbound input reach the agent");
    }

    #[test]
    fn camel_case_joins_on_dash_dot_underscore() {
        assert_eq!(camel("pre-commit"), "PreCommit");
        assert_eq!(camel("decision-issue.yml"), "DecisionIssueYml");
    }

    #[test]
    fn a_write_command_is_an_action_and_a_read_command_is_feedback() {
        let dir = std::env::temp_dir().join(format!("keel-cs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".engine/cli")).expect("mkdir");
        std::fs::write(
            dir.join(".engine/cli/commands.sysml"),
            "part a : CliCommand { :>> name = \"add-task\"; :>> family = \"authoring\"; :>> effect = CliEffect::writes; :>> stability = CliStability::stable; :>> synopsis = \"s\"; }\n\
             part b : CliCommand { :>> name = \"orient\"; :>> family = \"orientation\"; :>> effect = CliEffect::reads; :>> stability = CliStability::stable; :>> synopsis = \"o\"; }\n\
             part c : CliCommand { :>> name = \"accept\"; :>> family = \"governance\"; :>> effect = CliEffect::writes; :>> stability = CliStability::stable; :>> synopsis = \"h\"; }\n",
        )
        .expect("write");
        let (mut acts, mut fbs) = (Vec::new(), Vec::new());
        cli_actions(&dir, &mut acts, &mut fbs);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(acts.len(), 2);
        assert_eq!(fbs.len(), 1);
        let accept = acts.iter().find(|a| a.name == "cmdAccept").expect("accept");
        assert_eq!(accept.issued_by, "human", "accept is the human's authority even from the agent's shell");
        assert_eq!(acts.iter().find(|a| a.name == "cmdAddTask").expect("add-task").issued_by, "agent");
        assert_eq!(fbs[0].reports_to, "agent");
    }

    #[test]
    fn a_hook_event_becomes_an_action_on_the_agent_turn() {
        let dir = std::env::temp_dir().join(format!("keel-cs-h-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".claude")).expect("mkdir");
        std::fs::write(
            dir.join(".claude/settings.json"),
            r#"{"hooks":{"Stop":[{"hooks":[{"command":"K=x; $K hook stop"}]}],"PreToolUse":[{"hooks":[{"command":"$K hook pre-write"},{"command":"$K hook pre-bash"}]}]}}"#,
        )
        .expect("write");
        let mut acts = Vec::new();
        hook_actions(&dir, &mut acts);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(acts.len(), 2);
        let stop = acts.iter().find(|a| a.name == "hookStop").expect("stop");
        assert!(stop.title.contains("blocks") && stop.data.contains("hook stop"), "{}", stop.data);
        let pre = acts.iter().find(|a| a.name == "hookPreToolUse").expect("pre");
        assert!(pre.data.contains("pre-write") && pre.data.contains("pre-bash"), "{}", pre.data);
        assert!(acts.iter().all(|a| a.acts_on == "agent-turn" && a.issued_by == "hooks"));
    }

    /// The `HUMAN_AUTHORITY_COMMANDS` claim, tested against the write layer rather than asserted: every
    /// command listed there refuses an AI-kind actor at the write layer, so calling it the human's
    /// action is a fact about the code and not a label.
    #[test]
    fn human_authority_commands_refuse_an_ai_actor() {
        for c in HUMAN_AUTHORITY_COMMANDS {
            assert!(
                crate::write::HUMAN_ONLY_WRITE_COMMANDS.contains(&c),
                "`{c}` is listed as the human's authority but the write layer does not refuse an AI actor for it"
            );
        }
    }
}
