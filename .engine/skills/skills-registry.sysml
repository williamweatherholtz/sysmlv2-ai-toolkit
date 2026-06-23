// =====================================================================
// EngineSkillRegistry — the AI skills that ACTUALLY EXIST on disk.
//
// CR-10 (2026-06-11): the four phantom registrations (triage, staleness-sweep,
// implementer, grill-me — none existed) are gone; only skills that ACTUALLY
// EXIST on disk are registered (critique P1). The set has since grown (ceremony
// skills D0039; build + test-verify D0046/issue008). writePolicy is the INTENDED
// write boundary — enforcement arrives with the write API (D0003); until then it
// binds by discipline (CLAUDE.md §4).
//
// CONVENTION (D0059/issue036, process-skill guard): a skill that DEPLOYS a process MUST name the
// `.engine/processes/<name>.sysml` it deploys in its `purpose` — that path reference is what the
// "every process has a deploying skill (no inert process)" audit checks. Ceremony processes
// (definition-of-ready / definition-of-done / agile-workflow) are deployed by the sprint-* skills
// (D0059); their purposes name those process files too.
// =====================================================================
package EngineSkillRegistry {
    private import EngineElement::*;     // WritePolicy enum
    private import EngineSkills::*;

    part engineTriage : AISkill {
        :>> id = "5b3f6f3e-91a4-4f47-9f57-1f2f3a4b5c6d";
        :>> title = "engine-triage";
        :>> purpose = "Break EVERY request into parts and route EACH to CHANGE/EXECUTE/RECORD/VIEW/ORIENT before acting; flag what doesn't cleanly map; state routes out loud (CLAUDE.md 3, D0064). Fired every turn by a UserPromptSubmit hook.";
        :>> location = ".engine/skills/engine-triage/SKILL.md";
        :>> triggerCondition = "Any substantive request arrives.";
        :>> writePolicy = WritePolicy::readOnly;
    }
    part requirementQuality : AISkill {
        :>> id = "61c1f9c2-08e8-4f55-9b34-1f5fb1f0a111";
        :>> title = "requirement-quality";
        :>> purpose = "Audit requirements against INCOSE GtWR rules + EARS syntax; emit findings + rewrites.";
        :>> location = ".engine/skills/requirement-quality/SKILL.md";
        :>> triggerCondition = "A requirement is authored or changed; DoR gate.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part stpaSkill : AISkill {
        :>> id = "7d20a9c4-26cd-4f0a-8413-2b6fb2f0b222";
        :>> title = "stpa";
        :>> purpose = "Run STPA (losses/hazards/UCAs/constraints) per the SOP for safety-relevant scope.";
        :>> location = ".engine/skills/stpa/SKILL.md";
        :>> triggerCondition = "Safety-relevant Need/UseCase/Component enters scope.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part backlogRefinement : AISkill {
        :>> id = "8e31bad5-37de-4b1b-9524-3c70c3f0c333";
        :>> title = "backlog-refinement";
        :>> purpose = "Run backlog refinement: INVEST check, acceptance criteria as atomic Test items linked by verify, DoR gate, story-slicing.";
        :>> location = ".engine/skills/backlog-refinement/SKILL.md";
        :>> triggerCondition = "New backlog items need refinement to ready.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part testDesign : AISkill {
        :>> id = "a042dce7-59f0-4d2c-b635-5e92e5f0e555";
        :>> title = "test-design";
        :>> purpose = "Design verification Tests for engine items: choose method (test/demo/inspect/analysis/confirmation), write BDD/Gherkin scenarios, map acceptance criteria to atomic Test instances.";
        :>> location = ".engine/skills/test-design/SKILL.md";
        :>> triggerCondition = "Authoring DoD criteria, acceptance tests, or BDD scenarios for engine items.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part testResult : AISkill {
        :>> id = "b153edf8-6a01-4e3d-c746-6fa3f6f0f666";
        :>> title = "test-result";
        :>> purpose = "Record test execution outcomes as appended TestResult instances: outcome + judgedAgainst commit + judgedAt + judgedBy. Never mutate prior results.";
        :>> location = ".engine/skills/test-result/SKILL.md";
        :>> triggerCondition = "Executing tests, recording gate verdicts, or closing a DoD.";
        :>> writePolicy = WritePolicy::direct;
    }
    part traceabilityAudit : AISkill {
        :>> id = "c264fe09-7b12-4f4e-d857-7b047b01c777";
        :>> title = "traceability-audit";
        :>> purpose = "Audit satisfy/verify/allocate coverage: surface uncovered Needs, unverified Requirements, unallocated Components; trace-need paths; flag suspect items.";
        :>> location = ".engine/skills/traceability-audit/SKILL.md";
        :>> triggerCondition = "Architecture/Delivery review gates, DoD coverage checks, or asked to audit traceability.";
        :>> writePolicy = WritePolicy::readOnly;
    }
    part repoPush : AISkill {
        :>> id = "9f42cbe6-48ef-4c2c-a635-4d81d4f0d444";
        :>> title = "repo-push";
        :>> purpose = "Repository discipline: main-only direct commits, CR: prefix for schema/process changes, Co-Authored-By trailer, pre-commit validation + post-commit auto-push hooks (reconciled to D0014, issue009).";
        :>> location = ".engine/skills/repo-push/SKILL.md";
        :>> triggerCondition = "Committing engine/process work.";
        :>> writePolicy = WritePolicy::direct;
    }
    part build : AISkill {
        :>> id = "a3c8e1f0-5b9d-4e2a-8f6c-1d7b3e9a2c50";
        :>> title = "build";
        :>> purpose = "Compile the deliverable (Rust keel workspace) with warnings-as-errors and confirm the engine model still validates; report binary path + validation verdict. Prerequisite for test-verify; does not test or commit.";
        :>> location = ".engine/skills/build/SKILL.md";
        :>> triggerCondition = "Asked to build/compile, or before claiming the toolchain works.";
        :>> writePolicy = WritePolicy::readOnly;
    }
    part testVerify : AISkill {
        :>> id = "b4d9f201-6c0e-4f3b-9a7d-2e8c4f0b3d61";
        :>> title = "test-verify";
        :>> purpose = "Run the deliverable's verification gate: cargo test + clippy -D warnings + BDD scenarios + Python<->Rust orient parity; report per-layer pass/fail as evidence for a method=test DoD. Hands off to test-result for recording; does not commit.";
        :>> location = ".engine/skills/test-verify/SKILL.md";
        :>> triggerCondition = "Asked to test/verify, check parity, or before recording a deliverable DoD TestResult.";
        :>> writePolicy = WritePolicy::readOnly;
    }
    part sprintPlanning : AISkill {
        :>> id = "d375af1a-8c23-4f5d-e968-8c15c2f1e888";
        :>> title = "sprint-planning";
        :>> purpose = "Orient on ready frontier, select scope, apply Fibonacci sizing to set estimatedPoints, pass DoR gate before sprint work begins. Deploys the ceremony process .engine/processes/definition-of-ready.sysml (DoR, D0059).";
        :>> location = ".engine/skills/sprint-planning/SKILL.md";
        :>> triggerCondition = "Sprint refine gate; asked to plan next sprint, size a story, or select scope.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintStandup : AISkill {
        :>> id = "e486bg2b-9d34-4a6e-fa79-9d26d3g2f999";
        :>> title = "sprint-standup";
        :>> purpose = "Orient on current status, surface blockers, confirm active phase alignment, state next action — lightweight 2-minute ceremony. Runs the overall sprint loop .engine/processes/agile-workflow.sysml (deployed by the sprint-* skills, D0059).";
        :>> location = ".engine/skills/sprint-standup/SKILL.md";
        :>> triggerCondition = "Sprint standup gate; start of work session; asked for status or blockers.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintReview : AISkill {
        :>> id = "f597ch3c-0e45-4b7f-ab80-0e37e4h3g000";
        :>> title = "sprint-review";
        :>> purpose = "Per-sitting human review (D0049): summarize the sitting's sprints + metrics (velocity/efficiency/accuracy), run the transcript scan that feeds the autonomous retro, and record the human's explicit confirmation of the sitting — the single human gate.";
        :>> location = ".engine/skills/sprint-review/SKILL.md";
        :>> triggerCondition = "Sprint review gate; asked for velocity, efficiency, estimate accuracy, or process issues.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintCloseout : AISkill {
        :>> id = "a608di4d-1f56-4c8a-bc91-1f48f5i4h111";
        :>> title = "sprint-closeout";
        :>> purpose = "Autonomously close a sprint (D0049): verify DoD passes, record actualHours, record the closeOut gate (method=inspect, AI-judged) with NO human confirmation, hand off to the autonomous retro. Deploys the ceremony process .engine/processes/definition-of-done.sysml (DoD, D0059).";
        :>> location = ".engine/skills/sprint-closeout/SKILL.md";
        :>> triggerCondition = "Sprint closeOut gate; asked to close or accept a sprint.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintRetro : AISkill {
        :>> id = "b719ej5e-2a67-4d9b-cd02-2a59a6j5i222";
        :>> title = "sprint-retro";
        :>> purpose = "Autonomously retro the just-finished sprint (D0049): identify AVOIDABLE issues (preventable by a guard/skill/doc/check), create tracked Issue + backlog items for the fixes, record the retro gate (method=analysis, AI-judged) — no human input.";
        :>> location = ".engine/skills/sprint-retro/SKILL.md";
        :>> triggerCondition = "Sprint retro gate; asked for retrospective or process improvement dispatch.";
        :>> writePolicy = WritePolicy::direct;
    }
    part architecturalCritique : AISkill {
        :>> id = "c5f0a2b1-6d3e-4f8a-9b1c-2e7d4a0c8e61";
        :>> title = "architectural-critique";
        :>> purpose = "Run the antagonistic architecture/process audit with the CAE+GQM+ATAM framework (D0046): attack each claim for refuting evidence, compute the GQM metric set, play ATAM scenarios; record findings as tracked Issue/Decision items (not prose). The deploying skill for .engine/processes/architectural-critique.sysml.";
        :>> location = ".engine/skills/architectural-critique/SKILL.md";
        :>> triggerCondition = "Asked to audit/critique the engine or processes; on the architectural-critique process; periodic stress-test.";
        :>> writePolicy = WritePolicy::direct;
    }
    part docSync : AISkill {
        :>> id = "d6e0a3c2-7e4f-4a9b-8c2d-3f8e5b1d9f72";
        :>> title = "doc-sync";
        :>> purpose = "Deploy the Documentation Sync process (D0059): on any change to a type/schema/workflow/process/skill/tool/template/superseding-decision/convention, grep the doc surface and fix every invalidated claim IN THE SAME COMMIT. The deploying skill for .engine/processes/doc-sync.sysml.";
        :>> location = ".engine/skills/doc-sync/SKILL.md";
        :>> triggerCondition = "Any CHANGE (§3a) or schema/process/skill/tool edit; before committing such a change.";
        :>> writePolicy = WritePolicy::direct;
    }
    part statusReport : AISkill {
        :>> id = "c820fk6f-3b78-4e0c-de13-3b60b7k6j333";
        :>> title = "status-report";
        :>> purpose = "Generate a self-contained HTML project status dashboard: active sprint, velocity/efficiency, backlog, decisions, git activity. Materialized VIEW — not authored content.";
        :>> location = ".engine/skills/status-report/SKILL.md";
        :>> triggerCondition = "Asked for a status report, HTML dashboard, visual overview, or 'where does the project stand'.";
        :>> writePolicy = WritePolicy::readOnly;
    }
    part migration : AISkill {
        :>> id = "67940516-2c7d-4e8f-a067-516273849610";
        :>> title = "migration";
        :>> purpose = "Deploy the Safe Bulk Migration process (D0067): run any bulk migration of model data/schema (rename/split/drop/add a field across many sites) through the gated expand/migrate/contract lifecycle — committed transform script, dry-run+reconcile control totals, green at every step, backfill-before-tighten, grandfather/backfill-with-basis NEVER fabricate. The deploying skill for .engine/processes/migration.sysml.";
        :>> location = ".engine/skills/migration/SKILL.md";
        :>> triggerCondition = "Editing the same field/shape across many instances/files; a schema rename/split/drop/add; a representation refactor of authored model data.";
        :>> writePolicy = WritePolicy::direct;
    }
    part viewSkill : AISkill {
        :>> id = "67940516-2c7d-4e8f-a075-516273849611";
        :>> title = "view";
        :>> purpose = "Deploy query-driven viewpoints (D0075): read/write/execute lightweight TOML view files (.engine/views/*.view.toml) — a select + optional traverse + project filter over the tracking model, run by `keel view <name>` (Rust authority, fail-loud). The deploying skill for D0075; resolves issue018.";
        :>> location = ".engine/skills/view/SKILL.md";
        :>> triggerCondition = "Asked to create/run a viewpoint, slice the model by a concern, or trace items along edges.";
        :>> writePolicy = WritePolicy::direct;
    }
    part issueResolutionSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a078-516273849612";
        :>> title = "issue-resolution";
        :>> purpose = "Deploy the Issue Resolution process (D0078/D0077): link every Issue to the work or Decision that resolves it via a #Resolves edge, let open/resolved state COMPUTE from resolver-completeness (never a stored status), and record decision-moots-item as a typed edge (#Resolves for an Issue, supersede for a Need/Requirement) — never prose. The deploying skill for .engine/processes/issue-resolution.sysml.";
        :>> location = ".engine/skills/issue-resolution/SKILL.md";
        :>> triggerCondition = "Recording/triaging an Issue; asked to resolve an issue or list open issues; a Decision moots an item; the `issues` guard fails (untriaged).";
        :>> writePolicy = WritePolicy::direct;
    }

    part elementCritiqueSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a080-516273849613";
        :>> title = "element-critique";
        :>> purpose = "Deploy the Antagonistic Element Critique process (D0080/D0079): adversarially verify each tracked Need/SystemRequirement/Decision/Architecture element through required adversarial lenses by an INDEPENDENT critic, recording each critique as a method=critique verification + result #Verify-linked to the target; raise findings as severity-carrying Issues (#Resolves loop); let critique coverage + staleness COMPUTE from the assurance substrate (keel coverage, suspect). The deploying skill for .engine/processes/element-critique.sysml.";
        :>> location = ".engine/skills/element-critique/SKILL.md";
        :>> triggerCondition = "Asked to critique/red-team/adversarially review a requirement/need/decision; find weaknesses/gaps/ambiguities in the model; what hasn't been critiqued; or a critique going suspect after a change.";
        :>> writePolicy = WritePolicy::direct;
    }

    part diagramSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a085-516273849614";
        :>> title = "diagram";
        :>> purpose = "Deploy the Traceability Diagram process (D0085): generate the comprehensive traceability diagram as a regenerated, interactive, self-contained HTML view via `keel diagram` (whole model — typed nodes + metadata + every typed edge; in-page filter/search/focus/fit); never author or commit a diagram (it's a computed #View, §2.1/D0015 — git-ignored). The deploying skill for .engine/processes/diagram.sysml.";
        :>> location = ".engine/skills/diagram/SKILL.md";
        :>> triggerCondition = "Asked to see/draw/visualize/diagram the model, traceability, reliance, linkage, supersession, effects, or metadata; to map how decisions/needs/requirements/work/processes connect; or to explore the graph part at a time.";
        :>> writePolicy = WritePolicy::direct;
    }
    part renderSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a086-516273849615";
        :>> title = "render";
        :>> purpose = "Deploy the Modular Render + Review process (D0086): render ANY declared view as a self-contained interactive artifact via `keel render <view> --mode graph|table|review` (graph generalizes the D0085 diagram; table = sortable rows; review = accept/finding + rationale capture), and round-trip a human review back via `keel apply-review` as new linked critique items that attest state + induce suspicion. Never author or commit a rendered artifact (computed #View, §2.1/D0015 — git-ignored). The deploying skill for .engine/processes/render.sysml.";
        :>> location = ".engine/skills/render/SKILL.md";
        :>> triggerCondition = "Asked to render/tabulate/explore a specific view as an artifact, build a table from a view, review/disposition elements (accept/reject + rationale), capture critique findings, or export a review batch; or to apply a review batch back into the model.";
        :>> writePolicy = WritePolicy::direct;
    }
    part reportSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a087-516273849616";
        :>> title = "report";
        :>> purpose = "Deploy the Computed Report / Scorecard process (D0087): generate human-digestible aggregate reports via `keel report <assurance|traceability|quality-debt|flow> [--html]` — coverage %, critique %, attestation %, findings, traceability completeness, debt, volatility, flow/velocity — rolled up from the per-element views and split into HEALTH vs OPPORTUNITY; never author a metric or commit a snapshot (computed #View, §2.1/D0015 — git-ignored). The deploying skill for .engine/processes/report.sysml.";
        :>> location = ".engine/skills/report/SKILL.md";
        :>> triggerCondition = "Asked for a status/health report, coverage or test totals/%, a scorecard, metrics/KPIs, model health, technical debt, velocity, or 'how are we doing / where can we improve'.";
        :>> writePolicy = WritePolicy::direct;
    }
    part indicatorSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a089-516273849617";
        :>> title = "indicator";
        :>> purpose = "Deploy the Indicator Monitoring process (D0089): declare monitored measures (Indicators) that inform by DIRECTION not threshold (D0088), and collect data by method — computed (repo-derived, `keel indicators`), pulled (external programmatic) or manual (subjective), recorded via `keel record-measurement`. A Measurement is one datapoint; never fabricate an external/manual observation. ISO-15939/PSM/SMM-grounded. The deploying skill for .engine/processes/indicator.sysml.";
        :>> location = ".engine/skills/indicator/SKILL.md";
        :>> triggerCondition = "Asked to track/watch/monitor a metric or KPI over time, add an indicator, record a measurement, pull an external signal (market/regulation/social), or see how a measure is trending.";
        :>> writePolicy = WritePolicy::direct;
    }
    part introductionSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a093-516273849618";
        :>> title = "introduction";
        :>> purpose = "Deploy the guided project-based Onboarding process (D0093): take a newcomer on a freshly `keel init`-ed project from zero to FIRST VALUE in one session — 1-minute mental model, capture the first business Need, derive a first requirement + work item, run the first sprint to a real artifact, show `orient`. Learn by DOING the first real item, not a doc dump; dual-surface (AI drives CLI, human supervises). The deploying skill for .engine/processes/introduction.sysml.";
        :>> location = ".engine/skills/introduction/SKILL.md";
        :>> triggerCondition = "A human asks to be introduced/onboarded, 'how do I start', 'set up / spin up a new project', or right after `keel init`.";
        :>> writePolicy = WritePolicy::direct;
    }
}
