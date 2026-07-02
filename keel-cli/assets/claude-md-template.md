# CLAUDE.md — how to work in this project

This project is **tracked by keel**, a work-tracking engine built on SysML v2 text files.
keel records *the work of building this project* — needs, decisions, work items, tests — as
authored facts, and computes every status/view from them. **This project is not keel itself;**
keel is the tooling (like `git`). Read this before doing anything.

---

## 1. What you're looking at

- **`.tracking/`** — *your* project's authored facts (needs, requirements, work items,
  decisions, test results, sprints). This is where the work is recorded. You author here.
- **`.engine/`** — the keel engine that ships with the tool: schema, workflows, processes,
  skills, and — under **`.engine/reference/`** — the engine's own **read-only** design history
  (decisions 0001+). Treat `.engine/` as infrastructure; **you don't edit it** (it's the tool).
  Consult `.engine/reference/` to understand *why* the engine works the way it does.
- **The `keel` CLI is the authority.** State is never read from prose — it is **computed**:
  `keel orient .` (where things stand / what's ready), `keel whats-next .`, `keel validate .`,
  `keel guard .`. Author facts via the write API (`keel add-task`, `keel append-result`, …).

New here? Run the guided **`introduction`** skill (`keel` deploys it) — it onboards you by
capturing this project's first real need and running the first sprint.

---

## 2. The invariants (how keel thinks)

1. **Text is truth; everything derivable is a view.** Author only *irreducible* facts — atomic
   items, typed edges, test results, recorded judgments. **Never author a document, matrix,
   baseline, or report** — those are *computed views* (`keel report`, `keel render`). Test: can
   it be regenerated from other authored facts + git? Yes → it's a view; don't store it.
2. **Atomic items, typed edges only** (`:>` specialize/derive, `satisfy`, `verify`, `allocate`,
   `dependency`, `supersede`). No checklist blobs inside items.
3. **Every item has an immutable `id` (UUID);** `title` is a human string; `displayLabel` is
   computed. Items never collide on name.
4. **Capture decisions even when they cause no action.** "We won't do X" is a first-class
   `Decision`. Record the *why* (context + rationale) so it can be re-evaluated later.
5. **Validate before done.** A change is not done until `keel validate .` is clean and
   `keel guard .` passes.

---

## 3. How to work — parse first, stay process-bound

The AI's job is to **parse/interpret input → route each part to a defined process → carry it
out** (in parallel where possible). **No action is taken that is not tied to a process.**

- **Open every response with a visible `Parsed:` block** — one line per part, each labelled
  `TRIVIAL` / `CHANGE` / `EXECUTE` / `RECORD` / `VIEW` / `ORIENT` with its route. Example:
  *"Parsed: 1. RECORD — capture the login latency need. 2. EXECUTE — add the story to the sprint."*
- **Route each part:** *CHANGE* a workflow/gate/schema → needs explicit human acceptance first;
  *EXECUTE* tracked work → through a sprint; *RECORD* one atomic fact; *VIEW* → compute and show;
  *ORIENT* → `keel orient .`. When no process fits, define one — don't free-form.
- **Only strictly-trivial one-off edits** (a typo, a single rename) use a fast-path — and are
  still **labelled `TRIVIAL`** so the exemption is visible.
- **Human sign-off is an explicit step** — a `method=confirmation` verification whose passing
  result carries the human's attestation. Never infer sign-off from a general instruction.
- **The AI drives the CLI; the human supervises** (orients, reviews, signs off).

---

## 4. Working rules

- **Use the write API** (`keel add-task` / `append-result` / …) — it enforces UUIDs and
  append-only semantics. Hand-edit `.sysml` only when the API doesn't cover the operation.
- **Substantive work goes through a sprint** (refine → standup → implement → review → closeOut
  → retro); only trivial one-off edits are exempt.
- **`.engine/` is the tool — don't modify it.** Your decisions live in `.tracking/`; the
  engine's own architecture is read-only reference in `.engine/reference/`.
- **Commit discipline:** validate green before commit; record decisions as first-class
  `Decision` items with their rationale.

---

## 5. Validation

```
keel validate .    # your .tracking facts parse clean (no ERROR)
keel guard .       # the honest-state guards pass (well-formed / traceable / truthful)
keel orient .      # where things stand + what's ready + the burndown
```

Run these before considering any change done. keel is kernel-free for these — fast, no JVM.
