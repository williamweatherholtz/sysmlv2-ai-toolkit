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
`first A then B` successions, each task `satisfy`s a `<task>DoD : AcceptanceCriterion`.
**Textual contract:** the DoD usage must be named `<taskName>DoD` and authored on
**one line** (the reader is line-based). Done = the DoD carries `verifiedAtCommit`.

## Validate (mandatory before commit)

```
conda run -n sysml --no-capture-output python .engine/tools/validate/validate_tracking.py
```

Query: `python .engine/tools/query.py [whats-next|outstanding|suspect|item|downstream|trace]`.
