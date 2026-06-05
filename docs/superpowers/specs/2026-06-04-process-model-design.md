# Process Model Design — the engine's project lifecycle as data

**Status:** Design / pending review · **Date:** 2026-06-04 · **Branch:** `engine-restructure`

This spec defines the engine's **project-onboarding + development process**: a rigorous,
reusable, *data-driven* lifecycle that the engine itself owns and executes. It is the
precursor to the `.engine/workflows/` content, the schema additions, and the Decision
records that will implement it. It deliberately follows a rigorous rather than a standard
approach — the engine exists to *make* rigorous approaches, so its own process must be one.

---

## 1. Purpose & scope

**In scope (what this defines):** the set of workflows that carry a project from first
business intent through operation; the typed artifacts each produces; the gates between them;
how safety is woven in; what is *authored* vs *computed*; the identity model; and the
process-as-data meta-model that lets the engine read, execute, and modify its own process.

**Out of scope (captured as deferred extension points, §17):** a standalone verification &
validation workflow; procurement / long-lead-item management; portfolio / multi-project
rollup; runtime/telemetry ingestion (stays in native tools, referenced by URI).

**The deliverable of *this* spec is the process, not the data that falls out of running it.**
When we dogfood this process on the toolkit, the personas/needs/requirements we produce are
instance data (they live in `.tracking/`, gitignored). The *process* — workflows, phases,
gates, artifact types, the view registry — is the reusable engine content and is committed.

---

## 2. Foundational principles

1. **Inversion of control — the engine is canonical.** The engine owns workflow definitions
   *as data*; only the engine modifies them (and only through the Change Request workflow,
   §10). Skills and agents are demoted to **task-level tools** the engine invokes for a
   specific job; they must **call back** to the engine to record results, timestamps, and
   authorship. They never own a workflow or set their own direction. (Consequence: the Forge
   `systems-engineering` skill, which today owns its phase chain in YAML and drives
   `phase-orchestrator`, is *inverted* — the chain moves into the engine; `phase-orchestrator`,
   the critics, and the arbiter survive only as engine-invoked task tools.)

2. **Text is truth; everything derivable is a view.** Authored facts live in `.sysml`.
   Anything that can be regenerated deterministically from authored facts + git history is a
   **view** and is never stored (§12). Extends decisions D0001 and D0005.

3. **Atomic items, typed edges.** First-class, independently queryable items connected by the
   edge algebra `:>`, `satisfy`, `verify`, `allocate`, `dependency`, `supersede`. No checklist
   blobs, no documents-as-artifacts. Extends D0004.

4. **Capture decisions even when they yield no action.** "We will *not* serve this need" is a
   first-class, traceable `Decision`. This is a deliberate advantage over kanban/ADR-lite
   tooling, which silently drops non-actions.

---

## 3. Two models, and where things live

The **engine model** tracks the *work*; the **deliverable** is what the work produces. Their
vocabularies never mix (see `.engine/README.md`).

```
.engine/                         committed · reusable · the deliverable of this project
  schema/core/   schema/safety/    item + edge vocabulary (+ Need, Port, Release, meta-model)
  workflows/                       NEW — the Workflow/Phase/Gate instances (process-as-data)
  contracts/computed-state.md      grows into the VIEW REGISTRY (§12)
  processes/                       Process/ProcessStep procedures (the Delivery loop, etc.)
  skills/   decisions/   docs/

.tracking/                       gitignored · per-project instance data
  business/  architecture/  delivery/  deploy/  operate/   + workflow state
```

Reuse model: clone, keep `.engine/`, drop `.tracking/`.

---

## 4. The workflow set (overview)

Six workflows. Five form the lifecycle spine (different cadences); one is cross-cutting. They
are **linked**, not merged — coupled by typed edges + the commit-as-version reference + the
suspicion engine. There is **no baseline ceremony**: a baseline is a *view* computed from a
release and a commit match (§12).

```
BUSINESS ──A──▶ ARCHITECTURE ──B──▶ DELIVERY ──C──▶ DEPLOY ──▶ OPERATE
(what/why)      (how, technical)    (build/verify)   (release)   (support)
                                                                    │
        CHANGE REQUEST  ── cross-cutting; triggerable from any phase ┘
        SAFETY          ── woven through Business→Operate (not a separate workflow, §11)
```

| Workflow | TOGAF domain | Cadence | Exit gate |
|---|---|---|---|
| Business | Business | once; re-enter on pivot | **A** — every Need traced; verification criteria present |
| Architecture | Data · Application · Technology | once per major design | **B** — technical reqs `satisfy` Needs; allocation complete |
| Delivery | — | continuous, per-item | **C** — DoD: satisfaction == verified |
| Deploy | — | per-release | release-level V&V + safety-case validation passed |
| Operate | — | continuous | — (feeds back) |
| Change Request | — | event-driven, any phase | human-accept gate |

---

## 5. Business workflow (onboarding; "what / why")

The subjective business-and-needs layer. Mostly qualitative; a few objective/measurable.

**Phase chain:** `Brief / Opportunity → Personas → Needs → Use Cases`
(scenarios are sub-elements of use cases — main success scenario + extensions — not a peer
phase, per Cockburn and ASI's own Jama, which has no separate Scenario type).

**The `Need` type (the central collapse).** Market requirements and needs are the same thing
at this layer; they differ only by **source**. One authored type:

```
Need {
  id; title;                        // identity (UUID) + authored human string
  source : String;                  // customer | operator | internal | competitive | regulatory | safety | …
  statement;                        // solution-free desired outcome (JTBD/Ulwick style)
  marketJustification [0..1];        // opportunity / competitive rationale (optional)
  priority;                         // MoSCoW
  // verification criteria attach via verify-linked Tests, not a text blob
}
```

- A **Safety Goal is a `Need` with `source = safety`** — this is why safety is *woven into the
  one graph*, not a parallel lane (§11).
- Regulatory access (e.g. "ISO 26262 to sell in the EU") is a `Need` with `source = regulatory`.
  There is **no separate fiat `Constraint` type.**
- The **MSRD and "market requirements"** are **views** (rollups/filters over Needs), not
  authored artifacts (§12).

**No business-requirements decomposition.** Restating a need in requirement-grammar is an
anti-pattern (redundant authored data = staleness surface). The business layer is exactly
`Need + Use Case + Decision`.

**Scope = `Decision`s that `supersede` Needs.** Rather than a separate "constraint" step:
- "Provide only Y" → a `Decision` that supersedes Need X with a narrowed Need.
- "Won't do X" → a `Decision` that supersedes X with nothing downstream (a captured non-action).
- Deprecating a Need = its `currentState → superseded` **plus** the `Decision` that explains why.
  Nothing is lost (git + the edge remain); the suspicion engine flags anything downstream.

**Gate A:** every Need is traced (to a persona/use case) and carries verification criteria;
critics (stakeholder, devil's-advocate, premortem) clear; arbiter promotes.

---

## 6. Architecture workflow (technical "how" that satisfies Needs)

Three explicit **TOGAF solution-side domains**, each producing items that `satisfy` Business
Needs and `allocate` across the structure:

| Domain | Toolkit example | ASI robotics example |
|---|---|---|
| **Data** | the SysML schema + graph store | perception / telemetry data model |
| **Application** | parser, indexer, CLI, API, GUI | autonomy software controllers |
| **Technology** | runtime, deploy infrastructure | compute / ECU / hardware platform |

**Produces (authored):** the technical requirement cascade `System → Subsystem → Component`
(EARS, with `safety_level`, rationale, verification method); `Component`s with **`port`/
interface definitions**; `Design Element`s; architecture `Decision`s (including technical/
physical *givens* such as "must run on the existing 32-bit ECU" — these are Decisions, not a
Constraint type); and `allocate` edges.

**ICDs are views**, computed from `port` definitions + their connections (SysML `port def` /
`interface def`). Likewise the allocation matrix and BOM (§12).

**Gate B:** the technical requirements `satisfy` the Needs (no orphan Needs, no orphan reqs);
allocation is complete; interfaces resolve; critics + arbiter promote.

---

## 7. Delivery workflow (build / verify — already modeled)

The existing `agile-workflow.sysml` loop, unchanged in shape: `refine → standup(DoR) →
implement (+ detailed design) → review → done(DoD)`, plus retrospective and the staleness
sweep. Continuous, per-item, event-driven sprints (D0010). Detailed design lives here (the
Architecture↔detailed-design seam is intentionally *not* a workflow boundary — 15288 notes it
is inherently fuzzy). **Gate C:** Definition of Done — an item is done iff its satisfaction
computes to `verified`.

---

## 8. Deploy workflow (release · configuration · deployment · system V&V)

**Authored:** a `Release` (a declaration + a membership rule — "this release includes items
matching X at commit Y"); deployment *approval* (a recorded judgment).

**Views:** the **baseline** (computed from the release + commit match), release notes, the
release pack, the BOM/configuration (computed from the component tree + git versions), and the
**safety case** (computed from the loss→hazard→goal→requirement→V&V graph).

**Activities:** system-level V&V (the FAT/SAT/PPT equivalent), safety-case **validation** (does
the fielded system actually achieve its safety goals — validation vs. verification),
configuration management. *Runtime* deployment execution and telemetry stay in native tools and
are referenced by URI (D0001 boundary); the engine records the *decision/intent* to deploy.

---

## 9. Operate workflow (field support / feedback)

Ongoing support. Field incidents become new `Need`s or `Hazard`s (including SOTIF-style hazards
discovered only in operation) and route through Change Request (§10). The suspicion engine flags
affected upstream safety/requirements items for re-analysis.

---

## 10. Change Request workflow (cross-cutting; governs self-modification)

A CR is about *managing* a change, not viewing what it is — so it is a real, first-class
workflow, triggerable from **any** phase, by a human **or** an AI:

```
Propose ──▶ Research vs. competing alternatives ──▶ HUMAN-ACCEPT GATE (subjective) ──▶ apply | reject
 (who/what)   (critic + research task tools)         (a human must accept)              (create/    (captured
                                                                                         supersede)   Decision)
```

This is the **single controlled path by which the engine modifies its own process-as-data**
(its workflows, phases, gates, schema). It operationalizes the inversion-of-control model: AI
may *propose* improvements, but a human must accept them. Both acceptance and rejection are
captured (rejection as a `Decision`, per §2.4). The CR process is itself modular and
critiqueable — and may be improved *through* the CR process.

---

## 11. Safety — explicit touchpoints (not a "lane")

Safety is woven into the one graph (a Safety Goal is a `Need`; safety requirements `satisfy`
goals and `allocate` to components). STPA **splits across phases**:

| Workflow | Safety interaction | STPA |
|---|---|---|
| **Business** | Author Losses + system-level Hazards + Safety Goals (= safety-sourced Needs). HARA (severity / exposure / controllability → ASIL/SIL); exposure is read from the Use Cases. | **Step 1** (purpose: losses, hazards) |
| **Architecture** | Model the control structure (an Application+Technology view), derive UCAs and loss scenarios, derive Safety Requirements that `satisfy` Safety Goals and `allocate` to components. | **Steps 2–4** (control structure → UCAs → scenarios) |
| **Delivery** | Safety requirements get `verify`-linked Tests; **stricter DoD for `safety_level` items** (independent review; `analysis`/`inspection` method, not just test). The suspicion engine acts as a safety control — touch a safety item and everything downstream goes suspect. | — |
| **Deploy** | Safety case (a **view**) assembled from the graph; safety **validation** evidence gates the release; ASIL/SIL confirmation. | — |
| **Operate** | Field incidents → new Hazards/Needs (SOTIF); suspicion triggers re-analysis. | feedback → Step 1 |

Uses the existing optional `schema/safety` import (D0008: STPA in, HARA/ASIL scoring kept
lightweight; ASI's Jama models full HARA RPN as *calculated* fields — i.e. views).

---

## 12. Real data vs. views — the central architecture decision

> **Real (authored):** an irreducible decision. Deleting it loses information no one can
> recompute. → *items + typed edges + results + recorded judgments.*
> **View (computed, never stored):** regenerable from authored facts + git. Deleting it loses
> nothing.
> **The test:** *can I regenerate it deterministically from other authored facts + git?*
> Yes → view. No → real.

| Artifact | Real / View | Computed from |
|---|---|---|
| Persona, Need, Use Case (+ scenarios) | Real | authored |
| Technical reqs (system/subsystem/component) | Real | authored (EARS) |
| Component + its ports/interfaces | Real | authored (SysML `port def`) |
| `satisfy`/`verify`/`allocate`/`dependency`/`supersede` edges | Real | authored |
| Architecture `Decision` (incl. scope + technical givens) | Real | authored |
| Test, Test Result (with commit SHA) | Real | authored |
| Release (declaration + membership rule), deployment approval | Real | authored judgment |
| Hazard, Loss, Safety Goal, Safety Requirement, UCA | Real | authored |
| **ICD** | View | port defs + connections |
| **Allocation matrix, RTM / trace matrix** | View | the edges |
| **BOM / configuration** | View | component tree + git versions |
| **Baseline, release notes, release pack** | View | release + commit match |
| **MSRD / PRD / SyRS / any "document"** | View | rollup of the atomic items |
| **RPN / risk level** | View | severity × occurrence × detection |
| **Coverage / satisfaction / suspicion** | View | edges + results + git (D0005) |
| **DoR / DoD / gate exit-criteria evaluation** | View | item facts + linked tests |
| **Velocity / burndown** | View | work items + history |
| **Safety case** | View | loss→hazard→goal→req→V&V graph |

Headline: **we never author documents — we author atomic items + edges, and every document is a
query rendered.** `contracts/computed-state.md` grows from "satisfaction/coverage/suspicion"
into a full **view registry** enumerating each computed artifact and its derivation. Payoff:
anything computed can never go stale, so each artifact moved from *real* to *view* shrinks the
suspicion surface.

---

## 13. Identity & naming

- **`id` (UUID): real, immutable.** The element's true identity. Items therefore **never collide
  on name** — identity is the UUID, not the title. SysML elements are declared with
  UUID-derived names (e.g. `need_a1b2c3`) so the parser never sees a clash.
- **`title`: real, authored** human string; may duplicate freely.
- **`displayLabel`: a view** — computed from `title` + type + state + attributes (e.g.
  `[NEED·safety] Prevent rollover (P0, EU) ⚠superseded`).

---

## 14. Process-as-data meta-model

The workflows in §4 are not hardcoded — they are **instances** of a SysML meta-model the engine
reads to drive itself and edits (via CR) to change the process:

```sysml
Workflow      { name; purpose; cadence; phases : Phase[*] (ordered) }
Phase         { name; order; produces : ArtifactType[*];
                entryGate; exitGate; critics : CriticBinding[*];
                procedure : Process[0..1] }      // reuses existing Process/ProcessStep
Gate          { name; kind = entry|exit; checks : GateCheck[*] }   // exit-criteria eval is a VIEW
ArtifactType  { name; schemaType; nature = real|view; derivation } // derivation set iff view
CriticBinding { critic; severityThreshold }      // premortem | devils-advocate | stakeholder | traceability
```

`Process`/`ProcessStep` (already in the schema) model the **procedure inside a phase** — the
Delivery loop is the clearest case. Adding a workflow or augmenting a phase later is a *data*
edit, never a code change.

---

## 15. Inter-workflow linkage & the handoff contract

- **No baseline ceremony.** Coupling is via typed edges + commit-as-version + the suspicion
  engine. Change a Need in a later commit and downstream delivery work is flagged stale
  automatically — that *is* re-baselining, computed rather than ceremonial.
- **A handoff = typed artifact + upstream trace links + verification criteria** (the ASPICE /
  ISO 26262 model). At each gate the **traceability critic** verifies the trace and the
  **arbiter** returns promote / revise / halt.

---

## 16. Executors (skills & agents as callback tools)

Per §2.1, executors are task-level tools. A typical execution: the engine selects a Phase from
its data, hands the executor the phase config + upstream artifacts, the executor performs the
task (draft an item, run a critic, assemble a view), then **writes items + a recorded judgment
back** into tracked data with timestamp + authorship. The executor never decides the chain. The
write path will eventually be the single-API write policy (D0003); until tooling exists, "call
back" means writing the tracked `.sysml`/instance files per the contract.

---

## 17. Deferred extension points (decisions captured, no action now)

- **Standalone V&V workflow** — *deferred.* V&V is folded into Delivery (per-item `verify`) +
  Deploy (system-level). Promote to its own workflow if release-level V&V grows complex.
- **Procurement / long-lead items** — *deferred.* Relevant to ASI hardware (the PDLC Design
  phase triggers procurement); out of scope for the software toolkit. A future workflow.
- **Portfolio / multi-project rollup** — *deferred.* The engine is single-project; portfolio is
  a future *view*/overlay across project instances.
- **Runtime telemetry / event ingestion** — *out, by principle.* Stays in native tools,
  referenced by URI (D0001).

---

## 18. Schema & decision implications

New or changed schema (to be applied in the schema rewrite, which must also pass pilot
validation — see `RESUME.md`):
- Add `Need` (with `source`, `marketJustification`, `priority`); make Safety Goal a `Need`
  specialization or a `source=safety` Need.
- **Remove** any standalone business-layer `Constraint` type; model scope via `Decision` +
  `supersede`.
- Add `Port`/interface features on `Component`; ICD becomes a view.
- Add `Release` (declaration + membership rule).
- Add the meta-model types (`Workflow`, `Phase`, `Gate`, `GateCheck`, `ArtifactType`,
  `CriticBinding`); relate to existing `Process`/`ProcessStep`.
- Grow `contracts/computed-state.md` into the view registry (§12).

Likely new Decision records (`.engine/decisions/`, currently 0001–0010): inversion of control;
`.tracking` gitignored; six-workflow set; Need-collapse (market = need by source); scope-as-
supersede; real-vs-view view-registry; identity model; Change-Request self-modification
governance; TOGAF architecture domains; safety-as-woven-touchpoints.

---

## 19. Research grounding (abridged)

Sixteen frameworks reviewed (full reports in session history). Convergence: problem → solution-
structure → build, with the two hardest gates at *needs/requirements baselined* and
*architecture+allocation+interfaces baselined*; a handoff is artifact + trace + criteria; safety
is a traceability thread, not a separate phase. Sources include ISO/IEC/IEEE 15288 & 29148,
INCOSE SE Handbook, NASA SP-2016-6105, Automotive SPICE, ISO 26262, Stage-Gate, Double Diamond,
Dual-Track Agile, Lean Product Playbook, JTBD/ODI, Pragmatic (MRD/PRD), Working Backwards,
Cooper (personas), Cockburn (use cases), BABOK, TOGAF.

**ASI alignment (Autonomous Solutions, Inc.):** the company's Jama "robotics product framework
(incl. functional safety)" already runs `Persona → Use Case → Product → System → Subsystem →
Component` requirements in EARS + MoSCoW + verification-method + `safety_level`, with STPA
(Control Structure / UCA / unsafe Controller Action), HARA, Safety Goal, Safety Requirement,
System/Subsystem ICDs, Verification/Validation test cases, Change Request, and a RAASIC (RACI)
model — validating engine decisions D0004 (atomic items), D0006 (verification by method), D0008
(STPA), and the EARS choice. ASI's market requirements + MSRD live at a program/portfolio layer.
The engine's contribution over Jama: git-native traceability, computed suspicion/staleness, and
a reusable process-as-data definition.
