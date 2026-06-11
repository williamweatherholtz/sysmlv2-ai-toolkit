# .tracking/ — this project's instance data

Everything the engine *tracks for this project* lives here: backlogs, actors,
decisions-in-flight, workflow state. The engine (`.engine/`) is the reusable
machinery; this directory is what running it on a project produces.

**For THIS repo (the engine self-build), `.tracking/` is committed** — the engine's
own construction history is part of the deliverable's evidence. Downstream projects
choose their own policy.

## Authoring rules

- **Canonical vocabulary = `schema/core` types** (`EngineNeeds`, `EngineWork`,
  `EngineVerification`, ...). Copy the idioms from
  `.engine/docs/tracking-template.sysml` (it parses green).
- Author only **irreducible facts + recorded judgments** (CLAUDE.md §2). Status,
  coverage, trace are computed views — never write them down.
- Every item carries an immutable `:>> id` (UUID), `title`, and provenance.
- Subdirectories are fine (`business/`, `delivery/`, ...) — tooling scans recursively.

## The backlog dialect (read by `query.py`)

An `action def` here IS a work backlog: tasks = `action`s, dependencies =
`first A then B` successions. Each task's DoD is a method-tagged
`verification <task>DoD : Test` criterion. Results are **appended, immutable**
`part <task>R<n> : TestResult` records (outcome + `judgedAgainst` commit) —
re-verification appends `R2`, never edits `R1`. **Done = the latest result is a
pass.** Textual contract: `<task>DoD` / `<task>R<n>` naming, one line each
(the reader is line-based).

## Validate (mandatory before commit)

```
conda run -n sysml --no-capture-output python .engine/tools/validate/validate_tracking.py
```

Query: `python .engine/tools/query.py [whats-next|outstanding|suspect|item|downstream|trace]`.
