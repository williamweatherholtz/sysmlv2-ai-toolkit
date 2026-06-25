# CLAUDE.md — how to work in this repo

This repo **is a work-tracking engine** built on SysML v2 text files. It tracks the work of
building things — and is being built using its own discipline. Read this before doing anything.

> **Status: sprint discipline in force (D0064).** The tracking engine exists and is the
> authority (D0048): the Rust toolchain computes views (`keel orient`/`whats-next`/`suspect`)
> and the write API records facts (`append-result`/`add-task`/`append-gate-result`/`apply-review`); four layer
> validators gate every change. (The indexer and GUI don't exist yet; neither is needed for the
> discipline.) **All substantive work — CHANGE, delivery, and engine work — goes through a sprint**
> (refine→standup→implement→review→closeOut→retro); only trivial one-off edits are exempt, and
> triage-first is mandatory (§3). Full per-interaction enforcement (the no-sprint guard + a triage
> hook) is sprint30 (issue020); until then some rules bind by you + convention + the validators.

---

## 1. What you're looking at

- **`.engine/`** — the engine: the reusable schema, workflow definitions, contracts, processes,
  skills, and decisions. This is infrastructure (like `.git/`) and the deliverable of this
  project. Committed.
- **`.tracking/`** — instance data that falls out of running the process on *this* project
  (personas, needs, requirements, work items, decisions, test results, workflow state).
  **Committed in THIS repo** (the self-build's construction history is part of its evidence;
  recorded 2026-06-11). Downstream projects choose their own tracked-vs-ignored policy.
  See `.tracking/README.md` for layout + authoring rules.
- **Two models, never conflated:** the *engine model* tracks the work; the *deliverable* is
  what the work produces. The deliverable's domain vocabulary never enters the engine.

Authoritative reading order: this file → `.engine/README.md` → `.engine/decisions/`
(0001–0018) → the critiques in `docs/design-history/`. (The original design spec
`docs/design-history/2026-06-04-process-model-design-retired.md` is retired —
superseded in full by decisions 0001–0018; decisions win.)
**Orient** (where things stand / what's next) is never read from prose — compute it.
The **Rust toolchain is the sole authority (D0048; query.py retired at M4/D0074)**, no kernel required:
`keel orient [ROOT]` (JSON) / `keel whats-next [ROOT]` (ready list).
`orient` suspect covers BOTH .sysml drift AND deliverable-source drift (D0050): a Rust
verification task (listed in `.engine/deliverable-manifest.txt`) is suspect when the
source changed since it was verified — re-verify at HEAD to clear it. For REPRODUCIBLE
`method=test` drift, `keel reverify [--all-drift | --task NAME]` (D0101) re-runs the gate
declared in `.engine/contracts/reverify.toml` and, on green, stamps a fresh judged-at-HEAD
`TestResult` per drift task (honest — never fabricated; judgment methods stay manual).
Views are formally DECLARED (D0056/D0057, `.engine/views/viewpoint-registry.sysml`) and the
Rust tooling computes them: `keel orphans` renders the orphans viewpoint (needs/requirements/
tasks/issues missing required edges); `keel view <name>`, `audit`, `attestation-coverage`,
`governing-version`, `reprocess-candidates`, `suspect`, `concern-coverage` (D0057/issue035 — which
declared viewpoint concerns are served vs planned), `dispositions` (D0092 — which ≥Medium findings
carry a typed ACT/ACCEPT-RISK/DISMISS verdict vs undispositioned), `sitting-coverage` (D0049/issue040
— which delivery sprints have a covering per-sitting review via `#Covers` vs await one), `rootedness`
(D0098/D0099 — the charter-source burndown + the `#Capability`-without-Need hard-gate set) and
`tier-satisfaction` (D0098 — the downward burndown: are Needs decomposed into SRs and SRs verified, a
leading indicator of insufficient implementation) are the other computed lenses
(`suspect` also flags elements with an unresolved failing critique — `critique_suspect`, D0086). Any declared view
renders as an interactive artifact via `keel render <view> --mode graph|table|review` (D0086;
the `diagram` is the whole-model graph preset), and a human review round-trips back as linked
critiques via `keel apply-review` (the review viewpoint + render skill). Human-digestible
AGGREGATE scorecards (coverage %, critique %, traceability, debt, volatility, flow) come from
`keel report <assurance|traceability|quality-debt|flow|governance|friction> [--html] [--trend]`
(D0087, the `report` viewpoint; health vs opportunity; `--trend` = git-derived sparklines; `friction`
is the D0054/issue029 write-path-vs-spreadsheet benchmark). (The SysML
viewpoint-registry stays the concern-coverage index.)

---

## 2. How to interpret the architecture (the invariants)

1. **Text is truth; everything derivable is a view.** Author only *irreducible decisions*:
   atomic items, typed edges, test results, recorded judgments. **Never author a document,
   matrix, baseline, ICD, BOM, or report** — those are *computed views*. Test: *can it be
   regenerated from other authored facts + git?* Yes → it's a view; don't store it.
2. **Atomic items, typed edges only.** Edge algebra: `:>` (specialize/derive), `satisfy`,
   `verify`, `allocate`, `dependency`, `supersede`. No checklist blobs inside items.
3. **Identity:** every item has an immutable `id` (UUID) — *items never collide on name*.
   `title` is an authored human string (may duplicate). `displayLabel` is a computed view.
4. **Capture decisions even when they cause no action.** "We won't do X" is a first-class
   `Decision` that `supersede`s the need. Scope = superseding Decisions, not a separate type.
5. **`schema/core` is frozen.** Changes to schema or process definitions are architectural and
   go through the Change Request path (§4).
6. **Reference procedure; don't embed it.** Record what *is* — facts, conditions, typed edges;
   let the referenced, modular process decide what to *do*. Anything that names an action,
   verdict, or sequence — `ready`, `blocked`, `done`, `needs-review`, execution order — is a
   *computed view* or a *reference*, never an authored field. (A phase's gate/DoD = its
   `verify`-linked Tests passing; execution order/parallelism = the dependency DAG, computed
   from the `succession` graph + typed edges. "Test" is the universal verifiable condition,
   distinguished by `method` and `verify` target — so gate-checks and critics are Tests too.)
   **Materialized views are allowed** — a derived answer (status, trace matrix, baseline) MAY
   be cached/rendered for legibility, performance, or tool interop, *provided* it is clearly
   marked as derived (`#View`) and regenerable from authored facts + git. Materializing a view
   is not authoring truth; only *irreducible* facts and recorded judgments are authored.
6. **Requirement vs constraint vs indicator (the measure spectrum, D0088).** A **constraint** is an
   executable true/false predicate over the model — our **guards** ARE the engine's constraint layer
   (SysML-v2-style "requirements-as-evaluable-constraints," realized as CI-enforced Rust predicates);
   the §2 invariants are constraints stated in prose + enforced by guards. A **requirement** is a
   constraint elevated to a verified stakeholder contract (Need/SystemRequirement + satisfy/verify).
   An **indicator** is a *monitored* measure with no enforced threshold — a first-class `Indicator`
   item (D0089) that informs by DIRECTION (goal), viewed via `keel indicators [--trend]`. The
   indicator set is the CANONICAL monitored-measure watchlist (D0090); a single shared computation
   (`metric_value`) feeds both the indicators and the reports, so each scalar metric is computed once,
   and reports *render* the indicators (+ point-in-time structure) rather than re-defining the metrics.
   Datapoints accumulate in a `Measurement` BANK: pulled/manual observations via `record-measurement`,
   and computed readings via `keel snapshot-indicators` (a recorded *observation*, not a cache —
   D0091, a controlled compute-don't-store exception). `keel indicators` is bank-first + emits the
   full series. Its data
   arrives by a measurement METHOD: `computed` (objective, repo-derived — series via the report/trend
   engine, no stored datapoints), `pulled` (objective, external API/scraper — recorded `Measurement`
   datapoints via `keel record-measurement`), or `manual` (subjective, e.g. a survey — recorded).
   `Measurement`s are irreducible point-in-time observations (authored, with provenance) for pulled/
   manual; computed series recompute from the repo. When a metric's "good enough" boundary can't yet
   be defensibly set, it stays an **indicator** — promote to a requirement/guard only when a justified
   boundary emerges (D0088; avoid the Goodhart trap). Parametric constraints (mass/power budgets, MoEs)
   are *deliverable-domain* (D0054), not modeled in the work/process engine.
7. **Dual surface, one truth (D0093).** The **CLI/JSON is the authority + automation substrate** (the
   AI agent's surface — every fact authored via the write API, every state computed by the Rust
   toolchain); **HTML is the human's ergonomic oversight lens** (orient/review/decide). HTML NEVER
   stores truth — it renders computed `#View`s (`diagram`, `render`, `report`, `orient --html`) and
   wraps the write API (`apply-review`); it never becomes a second store or a second authority. The
   engine **spins up** on a new project via `keel init DIR` (binary-embedded; engine architecture
   decisions ship as read-only `.engine/reference/`, the new project authors its own fresh), and a
   newcomer is onboarded by the guided, project-based `introduction` skill (D0093).

---

## 3. The interaction loop ("main")

There is no executable "main" yet; **this is the main.** Do **not** assume a request
means "do work in the current phase." **Classify every request first** — by *what it
changes* — then follow that route:

```
request
  ├─ changes a workflow / phase / gate / schema definition ........ CHANGE    → §3a
  ├─ produces the active phase's typed artifact (tracked work) ..... EXECUTE   → §3b
  ├─ records ONE atomic fact (decision / test result / issue) ...... RECORD    → §3c
  ├─ asks for a computed answer (status, trace, stale set, a doc) .. VIEW      → §3d
  └─ asks where things stand / what is next ..................... ORIENT    → §3f
```

If a request spans categories, **split it** and route each part, and flag anything that
doesn't cleanly map (§3). Engine work (building the engine's own runtime/tooling) is routed
by *what it changes* (schema/process ⇒ CHANGE §3a; otherwise ⇒ EXECUTE §3b) and goes through
a sprint; only trivial one-off edits skip a sprint.
When unsure of the category, say so and ask rather than defaulting to EXECUTE.

**Recurring-or-one-time check (D0040 — mandatory before EXECUTE or VIEW).**
After classifying, ask: *will this task recur?* If yes and no skill exists → treat
as CHANGE first: create/update a skill that encodes the approach, then execute using
it. If clearly one-time → execute directly. If ambiguous → ask the user.

| Example request                       | Recurring? | Route                                    |
|---------------------------------------|------------|------------------------------------------|
| "I'm on Windows"                      | Yes        | Permanent fact → CLAUDE.md §6 or memory |
| "Make an HTML status report"          | Recurring  | CREATE skill first (status-report)       |
| "Review sprint transcript"            | Recurring  | Existing skill: sprint-review            |
| "Deploy to GitHub"                    | Recurring  | Existing skill: repo-push                |
| "Rename this one variable"            | One-time   | Execute directly                         |
| "Generate the architecture diagram"   | Ambiguous  | Ask: recurring or one-time?              |

This rule exists because every recurring task executed without a skill leaks process
knowledge into conversation history, where it cannot be enforced, reviewed, or
improved. Skills are the durable encoding of how we do things.

**Triage is a MANDATORY first move on EVERY request (D0064).** Open every substantive response
by running the triage: break the request down, name the category + route for *each* part —
e.g. *"RECORD → §3c"* — *before* acting, and explicitly **flag anything that does not cleanly
map** to a process rather than force-fitting it. Never infer-and-act silently: a silent mis-route
is exactly how an action slips past the discipline (e.g. recording a confirmation that was never
given, or doing delivery/engine work with **no sprint** — issue020). The `engine-triage` skill
encodes this checklist; invoke it at the start of every request. (A `UserPromptSubmit` triage
hook that fires it every turn is sprint30.)

**§3a — CHANGE.** Never freelance an edit to a workflow / phase / gate / schema. Route
through **Change Request** (§4): state the change + rationale, research alternatives if
non-trivial, get **explicit human acceptance**, then apply (create / `supersede` items),
validate green (§5), record a `Decision`, and commit `CR:`. `schema/core` is frozen
(human sign-off required); the Change Request workflow itself is frozen (out-of-band
Decision only — §4). A tooling change that alters the *meaning* of a computed view
(what counts as done / ready / suspect / satisfied) is CHANGE too — it shifts process
behavior as surely as editing a gate.

**§3b — EXECUTE.** The core loop:
1. **Orient** — run `keel orient [ROOT]` to
   compute in-progress sprint ceremony status + ready/outstanding backlog frontier.
   (No cursor file — orientation is fully computed from delivery file TestResults, D0045.)
2. **Act within the appropriate phase** — produce its defined artifact(s) as items + edges;
   don't invent artifacts the phase doesn't call for. If the request targets a *different*
   phase than the current frontier, **surface the mismatch** — don't silently jump;
   switching work items is itself a recorded `Decision`.
3. **Record back** the items/edges + a recorded judgment (what, why) with authorship +
   timestamp into `.tracking/`. You are a task tool: you execute the phase, you don't
   redefine it.
4. **Gate** — exit only when the phase's gate passes (trace complete, verification criteria
   present, critics clear, decision recorded).

**§3c — RECORD.** Author one atomic item (`Decision` / `TestResult` / `Issue`) + a
judgment. A "won't do / reduce scope" is a `Decision` that `supersede`s the Need — capture
it even though it produces no action. Never a document blob.
An **`Issue` must be TRIAGED** (issue-resolution process/skill, D0077/D0078): give it a
`#Resolves` edge from a resolving **action** (create one if none) or a mooting **Decision** —
`#Resolves dependency from <resolver> to <issueNNN>;`. Resolution is then COMPUTED (resolved
iff the resolver is done/accepted; `keel open-issues` / `orient` open_issues), never a prose
"RESOLVED" note; `keel guard issues` fails on an untriaged Issue. When a Decision moots an
Issue, record `#Resolves` from the Decision (for a Need/Requirement, `supersede`) — not prose.

- **Confirmation results require explicit human sign-off.** A `method=confirmation`
  verification *is* a recorded human attestation — its evidence is the human's word.
  Record it only on the human's explicit confirmation of that *specific* claim; never infer
  it from an instruction to "do the sign-offs," from the underlying work being done, or from
  your own judgment. (test / analysis / inspection / demonstration are recorded from their
  own evidence; confirmation's evidence is the attestation itself, so you must hold it.)
  A sixth method, `critique` (D0080), records an antagonistic lens-tagged verification of a
  tracked element by an *independent* critic; its findings become severity-carrying `Issue`s,
  and any finding ≥ Medium needs a human disposition (run the `element-critique` skill). The
  REQUIRED lenses per element type are a DECLARED, downstream-overridable policy
  (`.engine/contracts/critique-policy.toml`, D0097 — default Core-3: Need/SystemRequirement →
  completeness/correctness/testability, Decision → completeness/correctness/feasibility); the
  lens vocabulary itself (`CritiqueLens`) is the generic requirement-quality canon in schema/core.
  `keel critique-policy` shows the active policy; `keel critique-coverage` + `guard critique` read it. A
  disposition is itself a TYPED recorded judgment (D0092): a `method=confirmation` verification
  carrying `disposition : DispositionKind` (`act`/`acceptRisk`/`dismiss`), `#Dispositions`-linked
  to the finding, written via `keel apply-review` — never prose. ACCEPT-RISK/DISMISS close the
  finding; ACT also needs a `#Resolves` resolver. `keel dispositions` + `assured` read the verdict.
- **Sprint ceremony is autonomous; the human gate is the per-sitting review (D0049).**
  Per-sprint closeOut (`method=inspect`) and retro (`method=analysis`) are AI-recorded with
  NO human sign-off — a sprint closes when its DoD passes, and the retro autonomously turns
  *avoidable* issues into tracked items. The single human `confirmation` is the per-**sitting**
  sprint review (a sitting = one work session, ≥1 sprint), where the human accepts the
  sitting's content (batchable, D0019). Do not pause to confirm individual sprint ends.
- **Confirm only what tests can't (D0051).** `method=test/inspect/analyze` items are
  self-evidencing — their automated runs (cargo test, clippy, `keel validate`, `keel
  guard`) ARE the evidence; never ask a human to confirm a green test. The
  only confirmation-worthy class is non-test-verifiable judgment — Decisions / direction —
  where the evidence IS the human's word (D0016). A sitting of all-tested work with
  inline-accepted decisions has nothing to confirm.
- **Every recorded fact carries provenance:** *who* (`authoredBy` / `verifiedBy`), *when*
  (an authored ISO-8601 `*At` timestamp — the attestation time is its own irreducible fact,
  distinct from the commit date), and the commit it was made against (`verifiedAtCommit`,
  which also drives suspicion).

**§3d — VIEW.** Compute the answer from authored facts + git and present it. **Never store
it and never mutate** — status, trace matrix, suspicion / stale set, coverage, ICD, MSRD,
baseline are all views (§2.1).

**§3f — ORIENT.** Compute from authored facts — `keel orient [ROOT]` returns in-progress sprint ceremony status (which gate each live sprint is pending) + the ready/outstanding backlog frontier + a non-blocking `burndown` block (D0098 — tier-satisfaction pcts, unrooted capabilities, orphan stories; the always-visible "what's incomplete" headline). No cursor file; no mutation.

The six workflows (see the spec for detail):
**Business** (needs / "what-why") → **Architecture** (Data·Application·Technology / "how") →
**Delivery** (build/verify, continuous) → **Deploy** (release, config, V&V) →
**Operate** (field feedback); **Change Request** is cross-cutting.

---

## 4. Working rules (sprint discipline in force, D0064)

- **The write API is the sanctioned write path (Sprint 9, 2026-06-15).** Use `keel append-result`
  to append a `TestResult` to an action task, `keel append-gate-result` to append a `TestResult`
  to a ceremony gate (`verification` — the `{gate}R{n}` form, used by sprint closeOut/retro), and
  `keel add-task` to add a task + `DoD` to an action def — all enforce UUID generation and
  append-only semantics automatically. Direct editing of `.sysml` / instance files is still possible
  but is no longer the primary path; use it only when the write API does not yet cover the operation
  (schema changes, decision files).
- **Every change to schema or a workflow/process definition MUST:**
  1. be recorded as a `Decision` **file in `.engine/decisions/`** (a Change Request with its
     rationale — capture the decision even if small; commit messages and memory are NOT
     decision records — this lapsed once for ~11 CRs and was a HIGH critique finding), and
  2. carry its **recorded acceptance** — who accepted, when, at what commit (the Decision
     file or a confirmation-method DoD is the artifact), and
  3. **validate green** before commit (§5).
- **Commit convention:** prefix commits that change process/schema with `CR: <short rationale>`
  so the audit trail exists before the engine can enforce it.
- **Doc-sync rides every change (run the `doc-sync` skill):** when you create or change an item
  type, schema, workflow, process, skill, tool, template, or a superseding decision/convention,
  run the **`doc-sync` skill** (which deploys `.engine/processes/doc-sync.sysml`) — grep the doc
  surface and fix every doc claim the change invalidates **in the same commit**. Documentation
  drift was a recorded HIGH critique finding (2026-06-11).
- **Every process has a downstream deploying skill (D0059).** A process defined without a skill
  is inert (applied by vigilance, inconsistently). Each process is deployed by its own skill
  (doc-sync→doc-sync, architectural-critique→architectural-critique) or a consuming ceremony skill
  (DoR→sprint-planning, DoD→sprint-closeout, agile-workflow→sprint-*). A process with no deploying
  skill is an orphan. Cement recurring process work in skills (generalizes D0040).
- **Corrections become permanent guards (D0047):** a defect or correction found mid-work that
  reveals a *recurrable* process gap MUST be (1) logged as a tracked `Issue` and (2) given a
  permanent automated guard (validator / pre-commit check / lint) — never patched silently.
  Trivial one-off edits (typos, wording) are exempt; the test is *"could this class recur?"*
  Manual vigilance is not a control (the Sprint 14 → 16 repeat proved it).
- **Bulk migrations follow the migration process (run the `migration` skill, D0067):** any change
  that edits the same field/shape across many instances/files (rename/split/drop/add) goes through
  the gated expand/migrate/contract lifecycle — a committed transform script, a dry-run that
  reconciles control totals (counts must balance), green at every step, backfill-before-tighten,
  and historical/recorded data is **never fabricated** (grandfather or backfill-with-recorded-basis).
  Deploys `.engine/processes/migration.sysml`.
- **Authoring friction is the #1 risk (D0054).** The benchmark research found the dominant
  MBSE failure mode is not bad architecture but *adoption friction* — JPL's Europa Clipper
  partially reverted requirements/architecture to spreadsheets because authoring cost more
  than they were worth. Our architecture matches flagship practice (JPL OpenMBEE/openCAESAR,
  NASA NPR 7123.1, DoD ASoT), so we inherit the same risk. **The write path must stay lower-
  friction than a spreadsheet** — prefer the Rust no-kernel authority + `append-result`/`append-gate-result`/`add-task`
  write API over hand-editing; if recording a fact is harder than a spreadsheet edit, fix that
  first (issue015). Friction is a first-class quality, not an afterthought.
- **Git is a sanctioned tool; changes still need acceptance.** Running git (stage/commit) while
  implementing *accepted* work needs no separate permission. But green-lighting an
  *investigation* or *experiment* is not blanket approval of the resulting changes — each CHANGE
  (process / schema / decision, §3a) needs human acceptance before commit; when unsure, treat it
  as needing acceptance.
- **`main` is the canonical branch — work on it directly.** Commit accepted work straight to
  `main`; the `post-commit` hook pushes every commit. No long-lived feature branches: everything
  is pushed and merged to `main` only. (This overrides the generic "branch off the default branch
  first" default — per explicit standing instruction, 2026-06-11.)
- **The meta-process is frozen:** do not use Change Request to modify the
  Change Request workflow itself — that goes through a plain Decision + human edit, out of band.
- **There is NO prose state/handoff document — the model is the only tracker (Decision 0018).**
  `RESUME.md` was deleted 2026-06-11: it shadow-tracked the backlog (critique finding A7,
  reproduced once even after the critique). Where things stand is COMPUTED
  (`keel orient [ROOT]` / `keel whats-next [ROOT]`); what's next is the backlog's ready frontier;
  how to work here is THIS file; mechanics live in `.tracking/README.md`,
  `.engine/docs/` and `.engine/decisions/`. Never author a status/worklist/handoff doc —
  if resuming requires knowledge, it belongs in the model, a Decision, or these docs.

---

## 5. Validation (mandatory for every `.sysml` change)

A change is not done until it parses with zero `ERROR:`. **The Rust toolchain is the
canonical validator for `.tracking/` (D0048) — fast, no JVM:**

```
.\target\release\keel.exe validate .                                                          # .tracking/*.sysml — AUTHORITY (no kernel)
.\target\release\keel.exe guard                                                               # ALL fourteen forward guards (no kernel) — 13 hard-blocking (exit≠0 on any violation) + decision-requirement-link (warning-only)
.\target\release\keel.exe guard <name>                                                        # one guard: actors | acceptance-events | sprint-coverage | ceremony | charter | process-change | issues | viewpoint-renderer | manifest-coverage | critic-independence | process-skill | requirement-rootedness | decision-rationale (D0103) | decision-requirement-link (warning-only, D0102)  (+ runnable burndown/diagnostics, NOT enforced: assured, critique, critique-rigor, defect-guard-coverage)
.\target\release\keel.exe reverify --all-drift                                                 # D0101: re-run the .engine/contracts/reverify.toml gate at HEAD; on green, stamp a fresh TestResult per drift-suspect task (honest auto-re-verify; reproducible method=test only)
```
**Honest-state gates, not self-assurance gates (D0098).** A commit gate enforces only that the recorded
model is TRUTHFUL / well-formed / traceable — never that the work is COMPLETE. Completeness (coverage,
critique-coverage, readiness) is a NON-BLOCKING burndown surfaced in `orient` + run on demand
(`keel assured`/`keel critique-coverage`); incomplete implementation flagged AS incomplete is honest
state, never a commit blocker (don't fake a pass, don't block recording true state).
The thirteen hard-blocking honest-state guards are the Rust authority (D0074 M3/M4; D0098): `keel guard` (actors
D0037, acceptance-events D0066, sprint-coverage D0064/issue020, ceremony D0047/issue010+011, charter
D0068, process-change D0070 keystone, issues D0077/D0078 [every recorded problem accounted for],
viewpoint-renderer D0056/issue034 [renderers must name a real `keel` command, no retired query.py/
report.py], manifest-coverage D0050/issue033 [the deliverable-suspicion manifest stays valid — no dead
task/path entries], critic-independence D0080/issue031 [a critique must be by an INDEPENDENT critic —
honesty], process-skill D0059/issue036 [no inert process — every `.engine/processes/*.sysml` is named
by a deploying skill's purpose], requirement-rootedness D0098/D0099/issue047 [a `#Capability`-marked
user-facing feature must carry a `#DerivedFrom`→Need edge; UNMARKED decision-driven work is exempt —
the engine is legitimately decision-driven, D0064; the full charter-source balance is the non-blocking
`keel rootedness` burndown], decision-rationale D0103 [every Decision must carry a SUBSTANTIVE context +
rationale — the why — not a blank/trivial field; guarantees the decision-record's basis for future
improvement + reevaluation]). A FOURTEENTH guard, `decision-requirement-link` (D0102/issue052), RUNS in
`keel guard` every commit but is WARNING-level (visible, never blocks): it flags an accepted Decision
that names a Need/SystemRequirement in its prose with NO typed edge to it (a governance link that should
be typed, not prose) — promotable to a hard gate once proven low-noise. (Relatedly, `critique_suspect`
honors dispositions, D0102: a `fail` critique whose finding is ACCEPT-RISK'd/DISMISSED — via a typed
`#DependsOn` finding→critique edge — no longer induces suspicion.)
RUNNABLE BURNDOWN / diagnostics (computed, surfaced in orient, NEVER blocking — D0098): `assured`
D0079c [composite readiness], `critique` D0080/D0079 [critique-COVERAGE; note INDEPENDENCE stays
enforced above]; plus `critique-rigor` D0080/issue030 [low-rigor critiques + affirming-only critics];
`defect-guard-coverage` D0047/issue039 [a #ProcessDefect finding must resolve to a guard-producing
action]. The python `validate_*.py` guards, `query.py`, and `parity_check.py` were RETIRED at M4
(sprint58, issue012 closed) — the Rust path is the sole gate.

**`.engine/` schema/workflow/instance changes still use the kernel validators** (deeper
SysML semantics than the Rust validator covers), and they remain the authoritative SysML
oracle on demand / in CI (each starts the pilot kernel, ~20s):

```
$conda = "C:\Users\WilliamWeatherholtz\miniforge3\Scripts\conda.exe"
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_schema.py      # schema/core + safety
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_workflows.py   # workflows/*.sysml + _meta
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_instances.py   # .engine decisions/processes/skills
& $conda run -n sysml --no-capture-output python .engine\tools\validate\validate_tracking.py    # .tracking (kernel cross-check / fallback when the rust binary is unbuilt)
```

(Run through the full miniforge3 conda path — §6 explains why bare `conda` is not on PATH.
The kernel calls bare `java`. Sandbox must be disabled. The legacy `validate_sysml.py` was
retired 2026-06-11 — it predates the flat-package split.)
See `.engine/docs/keel-syntax-notes.md` for confirmed syntax do's/don'ts before authoring.

---

## 6. Environment notes

- Windows + PowerShell. Use PowerShell syntax (`$null`, `$env:VAR`, backtick line-continuation).
- **`conda` is NOT on `$env:PATH`** in PowerShell sessions that don't run conda init (e.g.
  Claude Code's shell). Use the full miniforge3 path every time:
  ```
  & "C:\Users\WilliamWeatherholtz\miniforge3\Scripts\conda.exe" run -n sysml --no-capture-output python ...
  ```
  Installation root: `C:\Users\WilliamWeatherholtz\miniforge3` (miniforge3, **not** miniconda3).
  The validator commands in §5 must use this prefix — `conda run` as a bare word will not be found.
- **NEVER pipe `conda run` output into a live cmdlet or redirect** (`| Select-String`,
  `| Out-Null`, `> $null`) — the kernel JVM holds the pipe and the shell HANGS. Run plain.
- Interrupted kernel runs can orphan JVMs: `python .engine/tools/kill_stale_kernels.py`.
- SysML validation requires the `sysml` conda env (Jupyter SysML kernel, OpenJDK).
- **Use absolute paths in shell commands; don't rely on cwd (issue013).** The Bash and
  PowerShell tools share one working directory, so a `cd` in one silently changes the cwd
  the other sees and breaks later relative-path commands. Pass absolute paths to scripts
  and files (the `keel` binary takes an explicit `[ROOT]`, and the kernel validators self-locate the repo, so cwd doesn't matter to them).
- **Validation-path tools must be kernel-free where possible (D0048).** A tool that gates
  commits or routine checks should not start the JVM kernel — it's slow and orphans JVMs
  (the leak W1 fixed). The forward guards + views are all kernel-free Rust (`keel guard` /
  `keel validate`); the JVM kernel runs only for deep `.engine` SysML semantics.
