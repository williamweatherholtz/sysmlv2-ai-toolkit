//! The Rust MIRROR of `.engine/cli/commands.sysml` (D0271, issue344).
//!
//! The `.sysml` is the home; guard `cli-surface-declared` holds this table, the dispatch
//! (`cli_surface`) and the facts equal both ways. `keel --help` renders from here so a synopsis has
//! one home and the help cannot go stale against the surface the way the hand-written block did
//! after D0273.

/// The sub-verbs a ROUTING command's invocation declares, each with its full segment, in declaration
/// order (D0451/D0454).
///
/// An invocation is a sub-verb list when it OPENS with a bare lowercase token and a `|` separates two
/// or more such segments (`decision|issue|... | task --file F | ...`). A segment opening with a flag, a
/// placeholder or a bracket is an argument alternative, not a sub-verb: inside a list it continues the
/// sub-verb it sits under (`reverify [--all-drift | --task N | --demos]` is ONE segment), and an
/// invocation that opens with one declares no sub-verbs at all (`FILE... | --spec-version`,
/// `<item> | --list | --mine`). This is the ONE reader: the control structure derives one action per
/// sub-verb from it, and `the_record_sub_verbs_match_the_fact` holds the router's arms to it.
#[must_use]
pub fn sub_verb_segments(invocation: &str) -> Vec<(String, String)> {
    let is_verb = |w: &str| w.starts_with(|c: char| c.is_ascii_lowercase()) && w.chars().all(|c| c.is_ascii_lowercase() || c == '-');
    let mut out: Vec<(String, String)> = Vec::new();
    for (i, seg) in invocation.split('|').enumerate() {
        match seg.split_whitespace().next() {
            Some(h) if is_verb(h) && !out.iter().any(|(v, _)| v == h) => out.push((h.to_string(), seg.to_string())),
            _ if i == 0 => return Vec::new(),
            // a flag, placeholder or bracket alternative continues the sub-verb it sits under
            _ => {
                if let Some(last) = out.last_mut() {
                    last.1.push('|');
                    last.1.push_str(seg);
                }
            }
        }
    }
    if out.len() < 2 {
        return Vec::new();
    }
    out.into_iter().map(|(v, s)| (v, s.trim().to_string())).collect()
}

/// The sub-verb names alone, in declaration order.
#[must_use]
pub fn sub_verbs_of(invocation: &str) -> Vec<String> {
    sub_verb_segments(invocation).into_iter().map(|(v, _)| v).collect()
}

/// One command or lens fact. `family == "lens"` means `keel show <name>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliFact {
    pub name: &'static str,
    pub family: &'static str,
    pub effect: &'static str,
    pub stability: &'static str,
    pub invocation: &'static str,
    pub synopsis: &'static str,
}

pub const CLI_FACTS: [CliFact; 99] = [
    CliFact { name: "accept", family: "governance", effect: "writes", stability: "stable", invocation: "<decision> --note TEXT --by <person> --date YYYY-MM-DD", synopsis: "record a human's acceptance of a proposed Decision; refused for an AI actor" },
    CliFact { name: "reject", family: "governance", effect: "writes", stability: "stable", invocation: "<decision> (--words TEXT | --note TEXT) --by <person> --date YYYY-MM-DD", synopsis: "record a human's rejection of a proposed Decision; the same channel rules as accept" },
    CliFact { name: "judge-set", family: "governance", effect: "writes", stability: "stable", invocation: "<.tracking file> (--words TEXT | --note TEXT) --by <person> --date YYYY-MM-DD [--verdict pass|fail] [--fail <test>,..] [--all]", synopsis: "record a human's verdict on the sampled proposed results of one file - one result and one quote receipt per item (D0443); accept's channel rules under confirmationRecord" },
    CliFact { name: "override", family: "governance", effect: "writes", stability: "stable", invocation: "<path> --reason TEXT", synopsis: "arm a single-use, path-bound write unlock; consuming it records an obligation" },
    CliFact { name: "claim", family: "governance", effect: "both", stability: "stable", invocation: "<item> | --list | --mine", synopsis: "take or inspect a work claim; liveness is computed" },
    CliFact { name: "enroll", family: "governance", effect: "writes", stability: "stable", invocation: "--actor ID --name NAME --kind human|ai", synopsis: "enroll a contributor: register the actor, bind this machine, verify the gate" },
    CliFact { name: "actor", family: "governance", effect: "both", stability: "stable", invocation: "[set <id>]", synopsis: "show the actor this session writes as, or bind one for this machine" },
    CliFact { name: "advance", family: "governance", effect: "reads", stability: "stable", invocation: "<sprint|process> [--to GATE|step]", synopsis: "a sprint's current ceremony step, or any process's steps with each bound check's verdict; --to is refused while an earlier step's check is red (D0436)" },
    CliFact { name: "record", family: "authoring", effect: "writes", stability: "stable", invocation: "decision|issue|statement|story ... | task --file F --def D --task T --method M --dod-from FILE | sprint <N> <slug> --charter <dNNNN> [--points P] [--fill FILE] | result --file F --task T --sha S [--verdict pass|fail] --judged-by A --judged-at D [--evidence TEXT] | gate-result --file F --gate G --sha S [--verdict pass|fail] --judged-by A --judged-at D [--evidence TEXT] | review --batch FILE [--sha S] [--judged-by A] [--judged-at D] | measurement --indicator I --value V [--at DATE] | indicator-snapshot [ROOT] | reverify [--all-drift | --task N | --demos] [--by A] | mint [N]", synopsis: "record one atomic fact, each sub-verb named for what it writes: a Decision, an Issue with its resolver, a human's words verbatim, the story translating them, a task with its Definition of Done, a sprint's ceremony scaffold (--fill never writes a result, D0301), a verdict on a task's DoD or a ceremony gate, a review batch as linked critiques, an indicator measurement or a dated snapshot, fresh gate results at HEAD where the gate is green (--demos replays replayable receipts, D0444); mint prints engine-minted v4 UUIDs and writes nothing" },
    CliFact { name: "validate", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "semantic validation of every .tracking file - the authority" },
    CliFact { name: "check", family: "gating", effect: "reads", stability: "stable", invocation: "FILE... | --spec-version", synopsis: "parse-check .sysml files, or report the baked grammar version" },
    CliFact { name: "check-engine", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: ".engine instance reference resolution, kernel-free" },
    CliFact { name: "guard", family: "gating", effect: "reads", stability: "stable", invocation: "[NAME] [ROOT]", synopsis: "run every enforced honest-state guard, or one by name" },
    CliFact { name: "gate", family: "gating", effect: "reads", stability: "stable", invocation: "[--fast | --workspace] [ROOT]", synopsis: "the commit tier: validate plus guards; --fast is the per-edit tier; --workspace gates every project the commit touches" },
    CliFact { name: "rules", family: "gating", effect: "reads", stability: "stable", invocation: "[--enforce] [ROOT]", synopsis: "the declared rules and whether each holds" },
    CliFact { name: "audit", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "retrospective adherence: charter, ceremony, estimation, sitting review" },
    CliFact { name: "audit-history", family: "gating", effect: "reads", stability: "stable", invocation: "[--since REF] [--max N]", synopsis: "re-derive the gate verdict per commit over a range" },
    CliFact { name: "audit-adherence", family: "gating", effect: "reads", stability: "stable", invocation: "[--since REF]", synopsis: "re-derive guard-set and severity monotonicity per commit - a control cannot be disarmed unsigned" },
    CliFact { name: "audit-ci-runs", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT] [--repo OWNER/NAME]", synopsis: "the external-fact gate: every ci-run receipt names a real green run on the judged SHA (D0323)" },
    CliFact { name: "assured", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "composite READY / NOT-READY assurance verdict with per-check detail" },
    CliFact { name: "adoption-check", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT] [--unit N] [--keep] [--vintage VERSION]", synopsis: "gate a foreign tree: every unit must land clean in a project that lacks it" },
    CliFact { name: "hook", family: "gating", effect: "tooling", stability: "internal", invocation: "post-edit|stop|pre-bash|pre-write|subagent-stop|user-prompt|config-change", synopsis: "the in-loop gates the harness calls; not for a human to invoke" },
    CliFact { name: "orient", family: "lens", effect: "reads", stability: "stable", invocation: "show orient [ROOT] [--html]", synopsis: "in-progress sprints, the ready and suspect frontier, the non-blocking burndown" },
    CliFact { name: "whats-next", family: "lens", effect: "reads", stability: "stable", invocation: "show whats-next [ROOT]", synopsis: "the ready list in priority order - declaration order is priority" },
    CliFact { name: "status", family: "lens", effect: "reads", stability: "stable", invocation: "show status [ROOT]", synopsis: "every base in one screen: engine pin, library drift, model honesty, work, CI verdict for HEAD" },
    CliFact { name: "show", family: "orientation", effect: "reads", stability: "stable", invocation: "<lens> [ROOT]", synopsis: "one computed lens by name; `show` alone lists them" },
    CliFact { name: "view", family: "lens", effect: "reads", stability: "stable", invocation: "show view <name> [ROOT]", synopsis: "render a declared TOML view as text" },
    CliFact { name: "item", family: "lens", effect: "reads", stability: "stable", invocation: "show item <name> [ROOT]", synopsis: "one item with its attributes and edges" },
    CliFact { name: "arch", family: "lens", effect: "reads", stability: "stable", invocation: "show arch elements|criticality|coupling|drift|stpa-inputs|coverage [ROOT]", synopsis: "architecture lenses over the code registry" },
    CliFact { name: "attestation", family: "lens", effect: "reads", stability: "stable", invocation: "show attestation [ROOT] [--json]", synopsis: "is a pass a receipt or a testimony: results by judge kind, receipts, fail rate" },
    CliFact { name: "actor-trace", family: "lens", effect: "reads", stability: "stable", invocation: "show actor-trace <actor> [ROOT]", synopsis: "everything an actor authored, judged or owns" },
    CliFact { name: "governing-version", family: "lens", effect: "reads", stability: "stable", invocation: "show governing-version <item> [ROOT]", synopsis: "which process version governs an item" },
    CliFact { name: "reprocess-candidates", family: "lens", effect: "reads", stability: "stable", invocation: "show reprocess-candidates [ROOT]", synopsis: "items whose governing process version has moved since they were judged" },
    CliFact { name: "enforcement-report", family: "lens", effect: "reads", stability: "stable", invocation: "show enforcement-report [ROOT]", synopsis: "fires, blocks, overrides and red-yields from the machine-local fire-ledger" },
    CliFact { name: "render", family: "rendering", effect: "reads", stability: "stable", invocation: "<view>|model [--mode graph|table|review] | report <assurance|traceability|quality-debt|flow|governance|friction> [--html] [--trend] | decision-card [NAME] [--proposed]", synopsis: "everything that draws (D0449): a declared view or the whole-model graph as interactive HTML; a scorecard; a Decision's deciding context as JSON - the channel issue body" },
    CliFact { name: "deck", family: "rendering", effect: "both", stability: "stable", invocation: "[ROOT] [--out FILE]", synopsis: "the mobile obligation deck; saving writes through the API" },
    CliFact { name: "serve", family: "rendering", effect: "both", stability: "stable", invocation: "[--port N] [ROOT]", synopsis: "the interactive console: lenses, approve queue, deck; wraps the write API" },
    CliFact { name: "init", family: "integration", effect: "writes", stability: "stable", invocation: "DIR", synopsis: "scaffold the engine into a new project" },
    CliFact { name: "sync", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT]", synopsis: "fetch, report divergence, integrate by merge, gate the result" },
    CliFact { name: "land", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT]", synopsis: "gate every project, push; on rejection merge and gate the merged tree, then retry; refuses the push naming the paths whose working-copy line ending differs from their .gitattributes eol (issue478)" },
    CliFact { name: "migrate", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT] [--dry-run]", synopsis: "bring an existing project onto this binary's engine vintage, refusing and rolling back on failure" },
    CliFact { name: "claude", family: "integration", effect: "tooling", stability: "stable", invocation: "[claude args...]", synopsis: "launch Claude Code with the keel hooks pinned ON: the plugin rendering, disableAllHooks:false above project scope, KEEL_BIN = this binary (D0296)" },
    CliFact { name: "sync-claude", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT] [--check]", synopsis: "regenerate the keel-owned .claude/ surface; --check reports drift only" },
    CliFact { name: "projects", family: "integration", effect: "reads", stability: "stable", invocation: "[ROOT] [--json]", synopsis: "every keel project in this repository, and which one you are in" },
    CliFact { name: "version", family: "integration", effect: "tooling", stability: "stable", invocation: "[--json]  (also --version)", synopsis: "release version, build commit, guard inventory" },
    CliFact { name: "process", family: "distribution", effect: "both", stability: "stable", invocation: "list|search|show|export|import|publish|retire|remove ...", synopsis: "the process catalogue: import, export or remove a unit" },
    CliFact { name: "library", family: "distribution", effect: "both", stability: "stable", invocation: "init|sync|list", synopsis: "the machine-local cache of the portable-content repository" },
    CliFact { name: "onboard", family: "distribution", effect: "reads", stability: "stable", invocation: "[ROOT] [--json]", synopsis: "has this project chosen its processes, and on what basis" },
    CliFact { name: "activation", family: "distribution", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "which processes and viewpoints this project has adopted" },
    CliFact { name: "activate", family: "distribution", effect: "writes", stability: "stable", invocation: "<process|viewpoint> [ROOT]", synopsis: "adopt a process or viewpoint as a unit" },
    CliFact { name: "deactivate", family: "distribution", effect: "writes", stability: "stable", invocation: "<process|viewpoint> [ROOT]", synopsis: "drop a process or viewpoint as a unit" },
    CliFact { name: "github-pull", family: "channel", effect: "writes", stability: "stable", invocation: "--repo O/N --by ACTOR --at DATE [--limit N] [--trust T]", synopsis: "pull open issues and ingest the new ones as verbatim Statements; autonomy follows repository visibility" },
    CliFact { name: "suite", family: "gating", effect: "both", stability: "stable", invocation: "[ROOT] [--touched] [-- <cargo test args>]", synopsis: "run the full suite and write the machine-local receipt of what it cost - a measurement, gating nothing (D0356); refuses from the image cargo relinks and records no verdict for a run that never reached a test (issue386); --touched runs only the integration tests that name a module changed since the base and writes its own receipt (D0421); --touched refuses to run, receipt outcome eol-mismatch, when a tracked path holds a line ending its .gitattributes eol does not declare (issue478)" },
    CliFact { name: "currency", family: "channel", effect: "writes", stability: "stable", invocation: "[ROOT] --repo OWNER/NAME --by ACTOR [--at DATE] [--skip-pull]", synopsis: "the unattended currency pass: pull open issues as verbatim Statements, sync the library, read drift back - one report, never a triage (D0338)" },
    CliFact { name: "github-ingest", family: "channel", effect: "writes", stability: "stable", invocation: "--repo O/N --issue N --by ACTOR --at DATE [--from FILE]", synopsis: "one GitHub issue becomes a verbatim Statement, idempotent on its URL" },
    CliFact { name: "github-decider", family: "channel", effect: "reads", stability: "deprecated", invocation: "[<login>]", synopsis: "who may decide on the channel; an unmapped login is refused, never defaulted" },
    CliFact { name: "github-gesture", family: "channel", effect: "reads", stability: "deprecated", invocation: "(env: COMMENT_BODY ...)", synopsis: "parse a channel comment into a JSON verdict; called by the workflow" },
    CliFact { name: "github-decision-id", family: "channel", effect: "tooling", stability: "deprecated", invocation: "<id>", synopsis: "split a channel decision id into project and name" },
    CliFact { name: "recall", family: "knowledge", effect: "reads", stability: "stable", invocation: "--prompt -", synopsis: "seed recall from a prompt on stdin and print a budgeted brief; zero model calls" },
    CliFact { name: "assumptions", family: "lens", effect: "reads", stability: "stable", invocation: "show assumptions [ROOT]", synopsis: "accepted-but-unverified items something depends on" },
    CliFact { name: "attestation-coverage", family: "lens", effect: "reads", stability: "stable", invocation: "show attestation-coverage [ROOT]", synopsis: "accepted Decisions lacking a passing acceptance result" },
    CliFact { name: "authority-queue", family: "lens", effect: "reads", stability: "stable", invocation: "show authority-queue [ROOT]", synopsis: "what awaits a human's authority, and what may not be self-attested" },
    CliFact { name: "boundary", family: "lens", effect: "reads", stability: "stable", invocation: "show boundary [ROOT]", synopsis: "one element's interface surface; takes an element" },
    CliFact { name: "boundary-sweep", family: "lens", effect: "reads", stability: "stable", invocation: "show boundary-sweep [ROOT]", synopsis: "tier-satisfaction white-box sweep, per Need slice" },
    CliFact { name: "business", family: "lens", effect: "reads", stability: "stable", invocation: "show business [ROOT]", synopsis: "the what/why layer: Brief, Personas, Needs, UseCases" },
    CliFact { name: "concern-coverage", family: "lens", effect: "reads", stability: "stable", invocation: "show concern-coverage [ROOT]", synopsis: "declared viewpoints against stakeholder concerns - which concerns nothing serves" },
    CliFact { name: "contentions", family: "lens", effect: "reads", stability: "stable", invocation: "show contentions [ROOT]", synopsis: "recorded disagreements between contributors awaiting adjudication" },
    CliFact { name: "commit-delta", family: "lens", effect: "reads", stability: "stable", invocation: "show commit-delta [ROOT] [--range A..B]", synopsis: "the model delta a git range made - Needs, Requirements, Decisions, Issues and tasks added, items retired by #Supersede and Issues resolved, with titles, reconciled against the diff's added declarations by count; empty when the range changed no item (D0282)" },
    CliFact { name: "control-census", family: "lens", effect: "reads", stability: "stable", invocation: "show control-census [ROOT]", synopsis: "every control by whose act it binds, the ledger blocks by actor kind, its motivating incidents (observed vs code-read) and its evidence class - friction and hypothetical first" },
    CliFact { name: "control-structure", family: "lens", effect: "reads", stability: "stable", invocation: "show control-structure [--svg] [ROOT]", synopsis: "STPA step 2 for this project's own workflow, computed: authorities, what each issues on which process carrying what data, and what feedback returns; --svg draws it (D0285)" },
    CliFact { name: "controls", family: "lens", effect: "reads", stability: "stable", invocation: "show controls [ROOT]", synopsis: "the two-way hazard/control diff: uncovered failure conditions and unanchored controls; per control DECLARED, ARMED and PROOF (PROVEN / UNPROVEN / UNDETERMINED, D0360)" },
    CliFact { name: "coverage", family: "lens", effect: "reads", stability: "stable", invocation: "show coverage [ROOT]", synopsis: "Needs and requirements with and without satisfy and verify edges" },
    CliFact { name: "critique-coverage", family: "lens", effect: "reads", stability: "stable", invocation: "show critique-coverage [ROOT]", synopsis: "per-element required-lens matrix and the gap set" },
    CliFact { name: "critique-policy", family: "lens", effect: "reads", stability: "stable", invocation: "show critique-policy [ROOT]", synopsis: "which antagonistic lenses each assurance-element type requires" },
    CliFact { name: "decision-follow-through", family: "lens", effect: "reads", stability: "stable", invocation: "show decision-follow-through [ROOT]", synopsis: "every accepted Decision's downstream items and evidence, and the gaps" },
    CliFact { name: "decisions", family: "lens", effect: "reads", stability: "stable", invocation: "show decisions [ROOT]", synopsis: "load-bearing Decisions, ranked by how much depends on them" },
    CliFact { name: "dispositions", family: "lens", effect: "reads", stability: "stable", invocation: "show dispositions [ROOT]", synopsis: "findings by verdict: act, acceptRisk, dismiss, undispositioned" },
    CliFact { name: "flow", family: "lens", effect: "reads", stability: "stable", invocation: "show flow [ROOT] [--json]", synopsis: "per-sprint cycle time from GIT in minutes - first commit touching the delivery file to the commit landing its retro result, open while none has - with commits, the elapsed time from the preceding commit, the inter-commit gap over the last 7 days against the 7 before, and the point calibration over the last 80 closed sprints (issue483/issue485)" },
    CliFact { name: "hardening", family: "lens", effect: "reads", stability: "stable", invocation: "show hardening [ROOT]", synopsis: "the critique process's own questions, computed" },
    CliFact { name: "indicators", family: "lens", effect: "reads", stability: "stable", invocation: "show indicators [ROOT]", synopsis: "monitored values with no enforced threshold" },
    CliFact { name: "intake", family: "lens", effect: "reads", stability: "stable", invocation: "show intake [ROOT]", synopsis: "statements, user stories and routing: unparsed, unrouted, unsourced" },
    CliFact { name: "knowledge", family: "lens", effect: "reads", stability: "stable", invocation: "show knowledge [ROOT]", synopsis: "question coverage: does seeding find an entity and traversal reach an answer" },
    CliFact { name: "launchables", family: "lens", effect: "reads", stability: "stable", invocation: "show launchables [ROOT]", synopsis: "the console's launchable set from declared skills and processes" },
    CliFact { name: "ls", family: "lens", effect: "reads", stability: "stable", invocation: "show ls [ROOT]", synopsis: "the .tracking files" },
    CliFact { name: "marker-census", family: "lens", effect: "reads", stability: "stable", invocation: "show marker-census [ROOT]", synopsis: "per-marker edge count against prose mentions" },
    CliFact { name: "priority", family: "lens", effect: "reads", stability: "stable", invocation: "show priority [ROOT]", synopsis: "the priority metric: every ready item in declared order with its computed class - resolver severity or retro recurrence - and the inversions (D0311)" },
    CliFact { name: "open-issues", family: "lens", effect: "reads", stability: "stable", invocation: "show open-issues [ROOT]", synopsis: "every open Issue, its resolvers, and whether each resolver is complete" },
    CliFact { name: "orphans", family: "lens", effect: "reads", stability: "stable", invocation: "show orphans [ROOT]", synopsis: "items nothing references: tasks with no DoD, Issues with no resolver" },
    CliFact { name: "outstanding", family: "lens", effect: "reads", stability: "stable", invocation: "show outstanding [ROOT]", synopsis: "every not-done item, flat" },
    CliFact { name: "recent", family: "lens", effect: "reads", stability: "stable", invocation: "show recent [ROOT]", synopsis: "git-derived activity timeline over .tracking and .engine" },
    CliFact { name: "rootedness", family: "lens", effect: "reads", stability: "stable", invocation: "show rootedness [ROOT]", synopsis: "charter-source burndown: need-rooted, decision-chartered, orphan" },
    CliFact { name: "sitting-coverage", family: "lens", effect: "reads", stability: "stable", invocation: "show sitting-coverage [ROOT]", synopsis: "per-sitting human review currency" },
    CliFact { name: "suspect", family: "lens", effect: "reads", stability: "stable", invocation: "show suspect [ROOT]", synopsis: "done work whose evidence drifted from the tree it was judged against" },
    CliFact { name: "tier-satisfaction", family: "lens", effect: "reads", stability: "stable", invocation: "show tier-satisfaction [ROOT]", synopsis: "per tier, the fraction cleanly satisfied downstream" },
    CliFact { name: "trace", family: "lens", effect: "reads", stability: "stable", invocation: "show trace [ROOT]", synopsis: "every typed edge reaching an item, both directions" },
    CliFact { name: "trace-need", family: "lens", effect: "reads", stability: "stable", invocation: "show trace-need [ROOT]", synopsis: "one Need's satisfaction chain down to test results" },
    CliFact { name: "verification", family: "lens", effect: "reads", stability: "stable", invocation: "show verification [ROOT]", synopsis: "EXAMINED against EXERCISED, never one number; --pending lists the gap" },
    CliFact { name: "why", family: "lens", effect: "reads", stability: "stable", invocation: "show why [ROOT]", synopsis: "answer a question from the model as a graph, with provenance" },
    CliFact { name: "workflows", family: "lens", effect: "reads", stability: "stable", invocation: "show workflows [ROOT]", synopsis: "the six workflows and their phases" },
];

/// Every fact whose family is not `lens` - the verbs typed directly after `keel`.
pub fn command_facts() -> impl Iterator<Item = &'static CliFact> { CLI_FACTS.iter().filter(|f| f.family != "lens") }
/// Every `show` lens.
pub fn lens_facts() -> impl Iterator<Item = &'static CliFact> { CLI_FACTS.iter().filter(|f| f.family == "lens") }

/// `keel --help`, rendered from the facts: verbs grouped by family, then the lenses.
#[must_use]
pub fn render_help() -> String {
    use std::fmt::Write as _;
    let mut out = String::from("keel - text is truth, state is computed

usage: keel <command> [args]
");
    let mut fam: &str = "";
    for f in command_facts() {
        if f.family != fam {
            fam = f.family;
            let _ = writeln!(out, "
  {fam}");
        }
        let stab = if f.stability == "stable" { String::new() } else { format!(" [{}]", f.stability) };
        let _ = writeln!(out, "    {:<20} {}{}
      {}", f.name, f.invocation, stab, f.synopsis);
    }
    out.push_str("
  show <lens> [ROOT]  - the computed lenses:
");
    for f in lens_facts() {
        let _ = writeln!(out, "    {:<24} {}", f.name, f.synopsis);
    }
    out
}

#[cfg(test)]
mod sub_verb_tests {
    use super::*;

    #[test]
    fn a_routing_invocation_yields_its_sub_verbs_with_inner_alternatives_kept_whole() {
        let inv = "decision|issue ... | reverify [--all-drift | --task N | --demos] [--by A] | gate-result --file F [--verdict pass|fail] | mint [N]";
        let segs = sub_verb_segments(inv);
        assert_eq!(sub_verbs_of(inv), vec!["decision", "issue", "reverify", "gate-result", "mint"]);
        assert_eq!(segs[2].1, "reverify [--all-drift | --task N | --demos] [--by A]");
        assert_eq!(segs[3].1, "gate-result --file F [--verdict pass|fail]");
    }

    #[test]
    fn argument_alternatives_and_plain_invocations_declare_no_sub_verbs() {
        for inv in ["FILE... | --spec-version", "<item> | --list | --mine", "[--fast | --workspace] [ROOT]", "<decision> (--words TEXT | --note TEXT) --by <person>", "[ROOT]", "", "set <id>"] {
            assert!(sub_verbs_of(inv).is_empty(), "{inv:?} is not a sub-verb list");
        }
    }

    #[test]
    fn the_live_record_fact_declares_thirteen_and_process_eight() {
        let inv = |n: &str| CLI_FACTS.iter().find(|f| f.name == n).expect("fact").invocation;
        assert_eq!(sub_verbs_of(inv("record")).len(), 13, "{:?}", sub_verbs_of(inv("record")));
        assert_eq!(sub_verbs_of(inv("process")), vec!["list", "search", "show", "export", "import", "publish", "retire", "remove"]);
        assert_eq!(sub_verbs_of(inv("library")), vec!["init", "sync", "list"]);
        assert!(sub_verbs_of(inv("claim")).is_empty());
    }
}
