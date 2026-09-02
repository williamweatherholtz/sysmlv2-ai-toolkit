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
//! `ProcessModel` instances, hazard→process edges) when the project has authored it, and reported
//! as absent when it has not: a fresh project computes the same actions with no anchors.
//!
//! ROLES are the view's own, stable ids (`hooks`, `commit-gate`, `ci`, `channel`, `agent`, `human`,
//! `remote`, `console`); an authored `Controller` named `ct<Role>` decorates the role with its title,
//! id and process model. The structure never depends on the decoration existing.

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
/// `Controller` name that decorates the role when present.
const ROLES: [(&str, &str, &str); 8] = [
    ("human", "ctHuman", "the human director"),
    ("agent", "ctAgent", "the AI agent"),
    ("hooks", "ctHooks", "keel at the Claude Code hook boundary"),
    ("commit-gate", "ctCommitGate", "keel at the git hook boundary"),
    ("ci", "ctCI", "GitHub Actions CI"),
    ("channel", "ctChannel", "the decision channel workflows"),
    ("remote", "ctRemote", "the GitHub remote"),
    ("console", "ctConsole", "keel serve"),
];

/// The controlled processes, likewise: role id, authored anchor, what it is.
const PROCESSES: [(&str, &str, &str); 6] = [
    ("model", "cpModel", "the recorded model (.tracking + .engine instances)"),
    ("main-ref", "cpMainRef", "the shared history (origin/main)"),
    ("enforcement-surface", "cpEnforcementSurface", "guards, hooks, workflows, processes, skills, contracts"),
    ("deliverable", "cpDeliverable", "the deliverable source and tests"),
    ("work", "cpWork", "sprints, ceremonies, the frontier"),
    ("agent-turn", "cpAgentTurn", "one Claude Code response"),
];

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
    for (event, subs) in events {
        // The verdict kind is a property of the event, from the dispatcher's contract: Stop and
        // PostToolUse can BLOCK, PreToolUse can DENY, the rest can only advise.
        let kind = match event.as_str() {
            "Stop" | "PostToolUse" | "SubagentStop" => "blocks",
            "PreToolUse" => "denies or advises",
            _ => "advises",
        };
        out.push(Action {
            name: format!("hook{event}"),
            title: format!("{event}: {kind}"),
            issued_by: "hooks",
            acts_on: "agent-turn",
            data: format!("keel hook {subs}; verdict JSON the harness enforces"),
            source: ".claude/settings.json".to_string(),
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

/// The structural, derivable half: every action and feedback path, from facts.
fn gather(root: &Path) -> (Vec<Action>, Vec<Fb>, Json) {
    let mut actions = Vec::new();
    let mut feedback = Vec::new();
    hook_actions(root, &mut actions);
    githook_actions(root, &mut actions);
    workflow_actions(root, &mut actions, &mut feedback);
    cli_actions(root, &mut actions, &mut feedback);
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
    // that is a fact the view states rather than papers over. What keel has is feedback: manifest drift.
    if root.join(".engine").join("deliverable-manifest.txt").is_file() {
        feedback.push(Fb {
            name: "deliverableDrift".to_string(),
            title: "deliverable drift makes done work suspect".to_string(),
            sensed_from: "deliverable",
            reports_to: "agent",
            data: "source listed in the manifest changed since a task's verified commit -> suspect (orient)".to_string(),
            source: ".engine/deliverable-manifest.txt".to_string(),
        });
    }
    (actions, feedback, remote)
}

/// The authored decoration: process models and hazard -> process edges, from the model.
fn decoration(model: &Model) -> (Vec<Json>, Vec<Json>) {
    let mut pmodels: Vec<Json> = Vec::new();
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
        pmodels.push(Json::Obj(vec![
            ("name".to_string(), Json::s(n.clone())),
            ("heldBy".to_string(), Json::s(held)),
            ("beliefs".to_string(), Json::s(beliefs)),
            ("falseBeliefsCite".to_string(), Json::Arr(cites)),
        ]));
    }
    pmodels.sort_by_key(Json::dump);
    let mut hazard_rows: Vec<Json> = Vec::new();
    for e in &model.edges {
        if model.items.get(&e.from).is_some_and(|i| i.type_name == "Hazard") && model.items.get(&e.to).is_some_and(|i| i.type_name == "ControlledProcess") {
            hazard_rows.push(Json::Obj(vec![("hazard".to_string(), Json::s(e.from.clone())), ("process".to_string(), Json::s(e.to.clone()))]));
        }
    }
    hazard_rows.sort_by_key(Json::dump);
    (pmodels, hazard_rows)
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
    let (actions, feedback, remote) = gather(root);
    let (pmodels, hazard_rows) = decoration(&model);
    let controllers: Vec<Json> = ROLES
        .iter()
        .map(|(role, anchor_name, what)| {
            let acts: Vec<Json> = actions.iter().filter(|a| a.issued_by == *role).map(|a| Json::s(a.name.clone())).collect();
            let fbs: Vec<Json> = feedback.iter().filter(|f| f.reports_to == *role).map(|f| Json::s(f.name.clone())).collect();
            let (anchor, title) = anchor_of(&model, anchor_name);
            Json::Obj(vec![
                ("role".to_string(), Json::s(*role)),
                ("what".to_string(), Json::s(*what)),
                ("anchor".to_string(), anchor),
                ("anchorTitle".to_string(), title),
                ("inert".to_string(), Json::Bool(acts.is_empty())),
                ("actions".to_string(), Json::Arr(acts)),
                ("feedback".to_string(), Json::Arr(fbs)),
            ])
        })
        .collect();
    let processes: Vec<Json> = PROCESSES
        .iter()
        .map(|(role, anchor_name, what)| {
            let (anchor, title) = anchor_of(&model, anchor_name);
            let acted_on = actions.iter().filter(|a| a.acts_on == *role).count();
            Json::Obj(vec![
                ("role".to_string(), Json::s(*role)),
                ("what".to_string(), Json::s(*what)),
                ("anchor".to_string(), anchor),
                ("anchorTitle".to_string(), title),
                ("actionsOnIt".to_string(), Json::Int(i64::try_from(acted_on).unwrap_or(0))),
            ])
        })
        .collect();
    let inert: Vec<Json> = ROLES.iter().filter(|(r, _, _)| !actions.iter().any(|a| a.issued_by == *r)).map(|(r, _, _)| Json::s(*r)).collect();
    Ok(Json::Obj(vec![
        ("control-structure".to_string(), Json::s("STPA step 2 for this project's own workflow, COMPUTED from the hook config, git hooks, workflow files, CLI facts and declared deciders (D0284). Authored anchors and process models decorate the roles when the project has authored them; the remote's rules are fetched live and never copied.")),
        ("controllers".to_string(), Json::Arr(controllers)),
        ("processes".to_string(), Json::Arr(processes)),
        ("actions".to_string(), Json::Arr(actions.iter().map(action_row).collect())),
        ("feedback".to_string(), Json::Arr(feedback.iter().map(fb_row).collect())),
        ("processModels".to_string(), Json::Arr(pmodels)),
        ("hazardsByProcess".to_string(), Json::Arr(hazard_rows)),
        ("remote".to_string(), remote),
        ("inertControllers".to_string(), Json::Arr(inert)),
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
