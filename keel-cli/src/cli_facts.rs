//! The Rust MIRROR of `.engine/cli/commands.sysml` (D0271, issue344).
//!
//! The `.sysml` is the home; guard `cli-surface-declared` holds this table, the dispatch
//! (`cli_surface`) and the facts equal both ways. `keel --help` renders from here so a synopsis has
//! one home and the help cannot go stale against the surface the way the hand-written block did
//! after D0273.

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

pub const CLI_FACTS: [CliFact; 101] = [
    CliFact { name: "accept", family: "governance", effect: "writes", stability: "stable", invocation: "<decision> --note TEXT --by <person> --date YYYY-MM-DD", synopsis: "record a human's acceptance of a proposed Decision; refused for an AI actor" },
    CliFact { name: "override", family: "governance", effect: "writes", stability: "stable", invocation: "<path> --reason TEXT", synopsis: "arm a single-use, path-bound write unlock; consuming it records an obligation" },
    CliFact { name: "claim", family: "governance", effect: "both", stability: "stable", invocation: "<item> | --list | --mine", synopsis: "take or inspect a work claim; liveness is computed" },
    CliFact { name: "enroll", family: "governance", effect: "writes", stability: "stable", invocation: "--actor ID --name NAME --kind human|ai", synopsis: "enroll a contributor: register the actor, bind this machine, verify the gate" },
    CliFact { name: "actor", family: "governance", effect: "both", stability: "stable", invocation: "[set <id>]", synopsis: "show the actor this session writes as, or bind one for this machine" },
    CliFact { name: "advance", family: "governance", effect: "reads", stability: "stable", invocation: "<sprint> [--to GATE]", synopsis: "the sprint's current ceremony step; --to is refused until every earlier step's verify-Test passes" },
    CliFact { name: "add-task", family: "authoring", effect: "writes", stability: "stable", invocation: "--file F --def D --task T --method M --dod-from FILE", synopsis: "add a backlog task with its Definition of Done" },
    CliFact { name: "record", family: "authoring", effect: "writes", stability: "stable", invocation: "decision|issue|statement|story ...", synopsis: "record one atomic fact: a Decision, an Issue with its resolver, a human's words verbatim, or the story translating them" },
    CliFact { name: "new", family: "authoring", effect: "writes", stability: "stable", invocation: "sprint <N> <slug> --charter <dNNNN> [--points P]", synopsis: "scaffold a sprint's ceremony record with minted ids" },
    CliFact { name: "mint", family: "authoring", effect: "tooling", stability: "stable", invocation: "[N]", synopsis: "engine-minted v4 UUIDs, one per line" },
    CliFact { name: "append-result", family: "authoring", effect: "writes", stability: "stable", invocation: "--file F --task T --sha S [--verdict pass|fail] --judged-by A --judged-at D [--evidence TEXT]", synopsis: "record a verdict on a task's Definition of Done" },
    CliFact { name: "append-gate-result", family: "authoring", effect: "writes", stability: "stable", invocation: "--file F --gate G --sha S [--verdict pass|fail] --judged-by A --judged-at D [--evidence TEXT]", synopsis: "record a verdict on a ceremony gate" },
    CliFact { name: "apply-review", family: "authoring", effect: "writes", stability: "stable", invocation: "--batch FILE [--sha S] [--judged-by A] [--judged-at D]", synopsis: "write a human review batch back as linked critiques" },
    CliFact { name: "record-measurement", family: "authoring", effect: "writes", stability: "stable", invocation: "--indicator I --value V [--at DATE]", synopsis: "add one measurement to an indicator series" },
    CliFact { name: "snapshot-indicators", family: "authoring", effect: "writes", stability: "stable", invocation: "[ROOT]", synopsis: "stamp the current indicator values as a dated series point" },
    CliFact { name: "reverify", family: "authoring", effect: "writes", stability: "stable", invocation: "[--all-drift | --task N] [--by A]", synopsis: "re-run the declared gate at HEAD and stamp fresh results where it is green" },
    CliFact { name: "validate", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "semantic validation of every .tracking file - the authority" },
    CliFact { name: "check", family: "gating", effect: "reads", stability: "stable", invocation: "FILE... | --spec-version", synopsis: "parse-check .sysml files, or report the baked grammar version" },
    CliFact { name: "check-engine", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: ".engine instance reference resolution, kernel-free" },
    CliFact { name: "guard", family: "gating", effect: "reads", stability: "stable", invocation: "[NAME] [ROOT]", synopsis: "run every enforced honest-state guard, or one by name" },
    CliFact { name: "gate", family: "gating", effect: "reads", stability: "stable", invocation: "[--fast | --workspace] [ROOT]", synopsis: "the commit tier: validate plus guards; --fast is the per-edit tier; --workspace gates every project the commit touches" },
    CliFact { name: "rules", family: "gating", effect: "reads", stability: "stable", invocation: "[--enforce] [ROOT]", synopsis: "the declared rules and whether each holds" },
    CliFact { name: "audit", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "retrospective adherence: charter, ceremony, estimation, sitting review" },
    CliFact { name: "audit-history", family: "gating", effect: "reads", stability: "stable", invocation: "[--since REF] [--max N]", synopsis: "re-derive the gate verdict per commit over a range" },
    CliFact { name: "audit-adherence", family: "gating", effect: "reads", stability: "stable", invocation: "[--since REF]", synopsis: "re-derive guard-set and severity monotonicity per commit - a control cannot be disarmed unsigned" },
    CliFact { name: "assured", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "composite READY / NOT-READY assurance verdict with per-check detail" },
    CliFact { name: "adoption-check", family: "gating", effect: "reads", stability: "stable", invocation: "[ROOT] [--unit N] [--keep]", synopsis: "gate a foreign tree: every unit must land clean in a project that lacks it" },
    CliFact { name: "hook", family: "gating", effect: "tooling", stability: "internal", invocation: "post-edit|stop|pre-bash|pre-write|subagent-stop|user-prompt", synopsis: "the in-loop gates the harness calls; not for a human to invoke" },
    CliFact { name: "orient", family: "orientation", effect: "reads", stability: "stable", invocation: "[ROOT] [--html]", synopsis: "in-progress sprints, the ready and suspect frontier, the non-blocking burndown" },
    CliFact { name: "whats-next", family: "orientation", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "the ready list in priority order - declaration order is priority" },
    CliFact { name: "status", family: "orientation", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "every base in one screen: engine pin, library drift, model honesty, work, CI verdict for HEAD" },
    CliFact { name: "show", family: "orientation", effect: "reads", stability: "stable", invocation: "<lens> [ROOT]", synopsis: "one computed lens by name; `show` alone lists them" },
    CliFact { name: "view", family: "orientation", effect: "reads", stability: "stable", invocation: "<name> [ROOT]", synopsis: "render a declared TOML view as text" },
    CliFact { name: "item", family: "orientation", effect: "reads", stability: "stable", invocation: "<name> [ROOT]", synopsis: "one item with its attributes and edges" },
    CliFact { name: "arch", family: "orientation", effect: "reads", stability: "stable", invocation: "elements|criticality|coupling|drift|stpa-inputs|coverage [ROOT]", synopsis: "architecture lenses over the code registry" },
    CliFact { name: "attestation", family: "orientation", effect: "reads", stability: "stable", invocation: "[ROOT] [--json]", synopsis: "is a pass a receipt or a testimony: results by judge kind, receipts, fail rate" },
    CliFact { name: "actor-trace", family: "orientation", effect: "reads", stability: "stable", invocation: "<actor> [ROOT]", synopsis: "everything an actor authored, judged or owns" },
    CliFact { name: "governing-version", family: "orientation", effect: "reads", stability: "stable", invocation: "<item> [ROOT]", synopsis: "which process version governs an item" },
    CliFact { name: "reprocess-candidates", family: "orientation", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "items whose governing process version has moved since they were judged" },
    CliFact { name: "enforcement-report", family: "orientation", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "fires, blocks, overrides and red-yields from the machine-local fire-ledger" },
    CliFact { name: "render", family: "rendering", effect: "reads", stability: "stable", invocation: "<view> [--mode graph|table|review]", synopsis: "render any declared view as interactive HTML" },
    CliFact { name: "diagram", family: "rendering", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "whole-model interactive graph as HTML on stdout" },
    CliFact { name: "report", family: "rendering", effect: "reads", stability: "stable", invocation: "assurance|traceability|quality-debt|flow|governance|friction [--html] [--trend]", synopsis: "a human-facing scorecard" },
    CliFact { name: "decision-card", family: "rendering", effect: "reads", stability: "stable", invocation: "[NAME] [--proposed]", synopsis: "a Decision's deciding context as JSON - the channel issue body" },
    CliFact { name: "deck", family: "rendering", effect: "both", stability: "stable", invocation: "[ROOT] [--out FILE]", synopsis: "the mobile obligation deck; saving writes through the API" },
    CliFact { name: "serve", family: "rendering", effect: "both", stability: "stable", invocation: "[--port N] [ROOT]", synopsis: "the interactive console: lenses, approve queue, deck; wraps the write API" },
    CliFact { name: "init", family: "integration", effect: "writes", stability: "stable", invocation: "DIR", synopsis: "scaffold the engine into a new project" },
    CliFact { name: "sync", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT]", synopsis: "fetch, report divergence, integrate by merge, gate the result" },
    CliFact { name: "land", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT]", synopsis: "gate every project, push; on rejection merge and gate the merged tree, then retry" },
    CliFact { name: "migrate", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT] [--dry-run]", synopsis: "bring an existing project onto this binary's engine vintage, refusing and rolling back on failure" },
    CliFact { name: "sync-claude", family: "integration", effect: "both", stability: "stable", invocation: "[ROOT] [--check]", synopsis: "regenerate the keel-owned .claude/ surface; --check reports drift only" },
    CliFact { name: "projects", family: "integration", effect: "reads", stability: "stable", invocation: "[ROOT] [--json]", synopsis: "every keel project in this repository, and which one you are in" },
    CliFact { name: "version", family: "integration", effect: "tooling", stability: "stable", invocation: "[--json]  (also --version)", synopsis: "release version, build commit, guard inventory" },
    CliFact { name: "process", family: "distribution", effect: "both", stability: "stable", invocation: "list|search|show|export|import ...", synopsis: "the process catalogue: import or export a unit" },
    CliFact { name: "library", family: "distribution", effect: "both", stability: "stable", invocation: "init|sync|list", synopsis: "the machine-local cache of the portable-content repository" },
    CliFact { name: "onboard", family: "distribution", effect: "reads", stability: "stable", invocation: "[ROOT] [--json]", synopsis: "has this project chosen its processes, and on what basis" },
    CliFact { name: "activation", family: "distribution", effect: "reads", stability: "stable", invocation: "[ROOT]", synopsis: "which processes and viewpoints this project has adopted" },
    CliFact { name: "activate", family: "distribution", effect: "writes", stability: "stable", invocation: "<process|viewpoint> [ROOT]", synopsis: "adopt a process or viewpoint as a unit" },
    CliFact { name: "deactivate", family: "distribution", effect: "writes", stability: "stable", invocation: "<process|viewpoint> [ROOT]", synopsis: "drop a process or viewpoint as a unit" },
    CliFact { name: "github-pull", family: "channel", effect: "writes", stability: "stable", invocation: "--repo O/N --by ACTOR --at DATE [--limit N] [--trust T]", synopsis: "pull open issues and ingest the new ones as verbatim Statements; autonomy follows repository visibility" },
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
    CliFact { name: "control-structure", family: "lens", effect: "reads", stability: "stable", invocation: "show control-structure [ROOT]", synopsis: "STPA step 2 for this project's own workflow, computed: authorities, what each issues on which process carrying what data, and what feedback returns" },
    CliFact { name: "controls", family: "lens", effect: "reads", stability: "stable", invocation: "show controls [ROOT]", synopsis: "the two-way hazard/control diff: uncovered failure conditions and unanchored controls" },
    CliFact { name: "coverage", family: "lens", effect: "reads", stability: "stable", invocation: "show coverage [ROOT]", synopsis: "Needs and requirements with and without satisfy and verify edges" },
    CliFact { name: "critique-coverage", family: "lens", effect: "reads", stability: "stable", invocation: "show critique-coverage [ROOT]", synopsis: "per-element required-lens matrix and the gap set" },
    CliFact { name: "critique-policy", family: "lens", effect: "reads", stability: "stable", invocation: "show critique-policy [ROOT]", synopsis: "which antagonistic lenses each assurance-element type requires" },
    CliFact { name: "decision-follow-through", family: "lens", effect: "reads", stability: "stable", invocation: "show decision-follow-through [ROOT]", synopsis: "every accepted Decision's downstream items and evidence, and the gaps" },
    CliFact { name: "decisions", family: "lens", effect: "reads", stability: "stable", invocation: "show decisions [ROOT]", synopsis: "load-bearing Decisions, ranked by how much depends on them" },
    CliFact { name: "dispositions", family: "lens", effect: "reads", stability: "stable", invocation: "show dispositions [ROOT]", synopsis: "findings by verdict: act, acceptRisk, dismiss, undispositioned" },
    CliFact { name: "hardening", family: "lens", effect: "reads", stability: "stable", invocation: "show hardening [ROOT]", synopsis: "the critique process's own questions, computed" },
    CliFact { name: "indicators", family: "lens", effect: "reads", stability: "stable", invocation: "show indicators [ROOT]", synopsis: "monitored values with no enforced threshold" },
    CliFact { name: "intake", family: "lens", effect: "reads", stability: "stable", invocation: "show intake [ROOT]", synopsis: "statements, user stories and routing: unparsed, unrouted, unsourced" },
    CliFact { name: "knowledge", family: "lens", effect: "reads", stability: "stable", invocation: "show knowledge [ROOT]", synopsis: "question coverage: does seeding find an entity and traversal reach an answer" },
    CliFact { name: "launchables", family: "lens", effect: "reads", stability: "stable", invocation: "show launchables [ROOT]", synopsis: "the console's launchable set from declared skills and processes" },
    CliFact { name: "ls", family: "lens", effect: "reads", stability: "stable", invocation: "show ls [ROOT]", synopsis: "the .tracking files" },
    CliFact { name: "marker-census", family: "lens", effect: "reads", stability: "stable", invocation: "show marker-census [ROOT]", synopsis: "per-marker edge count against prose mentions" },
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
