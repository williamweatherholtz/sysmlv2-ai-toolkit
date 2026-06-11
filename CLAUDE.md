# CLAUDE.md — how to work in this repo

This repo **is a work-tracking engine** built on SysML v2 text files. It tracks the work of
building things — and is being built using its own discipline. Read this before doing anything.

> **Status: BOOTSTRAP (late).** The schema parses green (four layer validators in
> `.engine/tools/validate/`) and a query layer computes views (`.engine/tools/query.py` —
> whats-next/suspect/trace). The write API, indexer, and GUI do **not exist yet**: direct text
> editing is still the write path, and the discipline below is enforced by *you and convention*
> plus the validators. Where a rule says the engine "computes" or "drives," the query layer does
> some of it; the rest you do by hand, by the rules here.

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

Authoritative reading order: this file → `.engine/README.md` → the design spec
`docs/superpowers/specs/2026-06-04-process-model-design.md` → `.engine/decisions/`.

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
  ├─ builds or fixes the engine's OWN runtime / tooling ........... BOOTSTRAP → §3e
  └─ asks where things stand / what is next ..................... ORIENT    → §3f
```

If a request spans categories, **split it** and route each part. If you can't tell
EXECUTE from BOOTSTRAP, ask: *am I building the engine (which tracks the work) or the
deliverable (what the work produces)?* — engine ⇒ BOOTSTRAP, deliverable ⇒ EXECUTE.
When unsure of the category, say so and ask rather than defaulting to EXECUTE.

**Classification is a visible, mandatory first move.** Open every substantive response by
naming the category and route — e.g. *"RECORD → §3c"* — *before* acting. Never
infer-and-act silently: a silent mis-route is exactly how an action slips past the
discipline (e.g. recording a confirmation that was never explicitly given). The
`engine-triage` skill encodes this checklist; invoke it at the start of a request when in
doubt.

**§3a — CHANGE.** Never freelance an edit to a workflow / phase / gate / schema. Route
through **Change Request** (§4): state the change + rationale, research alternatives if
non-trivial, get **explicit human acceptance**, then apply (create / `supersede` items),
validate green (§5), record a `Decision`, and commit `CR:`. `schema/core` is frozen
(human sign-off required); the Change Request workflow itself is frozen during bootstrap
(out-of-band Decision only — §4).

**§3b — EXECUTE.** The core loop:
1. **Orient** on the active workflow + phase from the state cursor in `.tracking/`. (None
   yet ⇒ Business workflow, first phase.)
2. **Act within the active phase only** — produce its defined artifact(s) as items + edges;
   don't invent artifacts the phase doesn't call for. If the request targets a *different*
   phase than the cursor, **surface the mismatch** — don't silently jump; switching phases
   is itself a recorded `Decision`.
3. **Record back** the items/edges + a recorded judgment (what, why) with authorship +
   timestamp into `.tracking/`. You are a task tool: you execute the phase, you don't
   redefine it.
4. **Gate** — exit only when the phase's gate passes (trace complete, verification criteria
   present, critics clear, decision recorded).

**§3c — RECORD.** Author one atomic item (`Decision` / `TestResult` / `Issue`) + a
judgment. A "won't do / reduce scope" is a `Decision` that `supersede`s the Need — capture
it even though it produces no action. Never a document blob.

- **Confirmation results require explicit human sign-off.** A `method=confirmation`
  verification *is* a recorded human attestation — its evidence is the human's word.
  Record it only on the human's explicit confirmation of that *specific* claim; never infer
  it from an instruction to "do the sign-offs," from the underlying work being done, or from
  your own judgment. (test / analysis / inspection / demonstration are recorded from their
  own evidence; confirmation's evidence is the attestation itself, so you must hold it.)
- **Every recorded fact carries provenance:** *who* (`authoredBy` / `verifiedBy`), *when*
  (an authored ISO-8601 `*At` timestamp — the attestation time is its own irreducible fact,
  distinct from the commit date), and the commit it was made against (`verifiedAtCommit`,
  which also drives suspicion).

**§3d — VIEW.** Compute the answer from authored facts + git and present it. **Never store
it and never mutate** — status, trace matrix, suspicion / stale set, coverage, ICD, MSRD,
baseline are all views (§2.1).

**§3e — BOOTSTRAP.** Building the engine's own runtime / tooling is exempt from the full
workflow (it can't yet track its own construction). Do the work, track it in `RESUME.md`,
and still validate green + commit `CR:` for any schema/process touch (§4, §5).

**§3f — ORIENT.** Read the state cursor and report; no mutation.

The six workflows (see the spec for detail):
**Business** (needs / "what-why") → **Architecture** (Data·Application·Technology / "how") →
**Delivery** (build/verify, continuous) → **Deploy** (release, config, V&V) →
**Operate** (field feedback); **Change Request** is cross-cutting.

---

## 4. Bootstrap rules (in force NOW, until the runtime exists)

- **Direct editing of `.sysml` / instance files is the sanctioned bootstrap write path.** There
  is no write API yet. Edit deliberately.
- **Every change to schema or a workflow/process definition MUST:**
  1. be recorded as a `Decision` (a Change Request with a one-line rationale — capture the
     decision even if small), and
  2. **validate green** before commit (§5).
- **Commit convention:** prefix commits that change process/schema with `CR: <short rationale>`
  so the audit trail exists before the engine can enforce it.
- **Doc-sync rides every change:** when you create or change an item type, schema, workflow,
  process, skill, tool, or template, run the Documentation Sync process
  (`.engine/processes/doc-sync.sysml`) — fix every doc claim the change invalidates **in the
  same commit**. Documentation drift was a recorded HIGH critique finding (2026-06-11).
- **Git is a sanctioned tool; changes still need acceptance.** Running git (stage/commit) while
  implementing *accepted* work needs no separate permission. But green-lighting an
  *investigation* or *experiment* is not blanket approval of the resulting changes — each CHANGE
  (process / schema / decision, §3a) needs human acceptance before commit; when unsure, treat it
  as needing acceptance.
- **`main` is the canonical branch — work on it directly.** Commit accepted work straight to
  `main`; the `post-commit` hook pushes every commit. No long-lived feature branches: everything
  is pushed and merged to `main` only. (This overrides the generic "branch off the default branch
  first" default — per explicit standing instruction, 2026-06-11.)
- **The meta-process is frozen during bootstrap:** do not use Change Request to modify the
  Change Request workflow itself — that goes through a plain Decision + human edit, out of band.
- **Bootstrap exemption:** building the engine's own tooling is tracked in `RESUME.md` (the
  sanctioned bootstrap tracker), not through the full workflow — the engine can't yet track its
  own construction. The first *real* dogfood is a downstream feature *after* the schema parses
  and one view computes.

---

## 5. Validation (mandatory for every `.sysml` change)

A change is not done until it parses with zero `ERROR:`. Run the validator that covers
what you touched (each starts the pilot kernel, ~20s):

```
conda run -n sysml --no-capture-output python .engine\tools\validate\validate_schema.py      # schema/core + safety
conda run -n sysml --no-capture-output python .engine\tools\validate\validate_workflows.py   # workflows/*.sysml + _meta
conda run -n sysml --no-capture-output python .engine\tools\validate\validate_instances.py   # .engine decisions/processes/skills
conda run -n sysml --no-capture-output python .engine\tools\validate\validate_tracking.py    # .tracking/*.sysml
```

(Run through `conda run -n sysml`; the kernel calls bare `java`. Sandbox must be disabled.
The legacy `validate_sysml.py` was retired 2026-06-11 — it predates the flat-package split.)
See `.engine/docs/sysmlv2-syntax-notes.md` for confirmed syntax do's/don'ts before authoring.

---

## 6. Environment notes

- Windows + PowerShell. Use PowerShell syntax (`$null`, `$env:VAR`, backtick line-continuation).
- SysML validation requires the `sysml` conda env (Jupyter SysML kernel, OpenJDK).
