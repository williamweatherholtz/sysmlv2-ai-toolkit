# CLAUDE.md — how to work in this repo

This repo **is a work-tracking engine** built on SysML v2 text files. It tracks the work of
building things — and is being built using its own discipline. Read this before doing anything.

> **Status: BOOTSTRAP.** The engine's runtime (parser, indexer, query/view engine, write API)
> does **not exist yet**, and the schema does **not parse yet** (see `RESUME.md`). So the
> discipline below is enforced by *you and convention*, not by tooling. Where a rule says the
> engine "computes" or "drives," that is the target; today you do it by hand, by the rules here.

---

## 1. What you're looking at

- **`.engine/`** — the engine: the reusable schema, workflow definitions, contracts, processes,
  skills, and decisions. This is infrastructure (like `.git/`) and the deliverable of this
  project. Committed.
- **`.tracking/`** — instance data that falls out of running the process on *this* project
  (personas, needs, requirements, work items, decisions, test results, workflow state).
  **Gitignored.** Replaced per project; never the deliverable.
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

---

## 3. The interaction loop ("main")

There is no executable "main" yet; **this is the main.** To do *any* work in this repo:

1. **Orient.** Read the active workflow + phase from the workflow state in `.tracking/`
   (the runtime cursor). If none exists, you are in the Business workflow at its first phase.
2. **Act within the active phase only.** Produce the phase's defined typed artifact(s) as
   items + edges. Do **not** freelance work that belongs to another phase, and do **not**
   invent artifacts the phase doesn't call for.
3. **Record back.** Write the items/edges and a recorded judgment (what you did, why) with
   authorship + timestamp into `.tracking/`. You are a *task tool* serving the engine — you
   execute the defined phase; you do not redefine the workflow.
4. **Gate.** A phase exits through its gate: trace is complete, verification criteria present,
   critics clear, a decision is recorded. Don't promote work that hasn't passed its gate.
5. **Change the process only via Change Request (§4).**

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
- **The meta-process is frozen during bootstrap:** do not use Change Request to modify the
  Change Request workflow itself — that goes through a plain Decision + human edit, out of band.
- **Bootstrap exemption:** building the engine's own tooling is tracked in `RESUME.md` (the
  sanctioned bootstrap tracker), not through the full workflow — the engine can't yet track its
  own construction. The first *real* dogfood is a downstream feature *after* the schema parses
  and one view computes.

---

## 5. Validation (mandatory for every `.sysml` change)

The SysML v2 syntax here is **pending validation against the pilot implementation** — treat it
as unproven. A change is not done until it parses with zero `ERROR:`:

```
conda run -n sysml --no-capture-output python .engine\tools\validate\validate_sysml.py
```

(Run through `conda run -n sysml`; the kernel calls bare `java`. Sandbox must be disabled.)
See `.engine/docs/sysmlv2-syntax-notes.md` for confirmed syntax do's/don'ts before authoring.

---

## 6. Environment notes

- Windows + PowerShell. Use PowerShell syntax (`$null`, `$env:VAR`, backtick line-continuation).
- SysML validation requires the `sysml` conda env (Jupyter SysML kernel, OpenJDK).
