# `.engine/` — the work-tracking engine

This dotfolder **is the engine**: a reusable, AI-complemented, SysMLv2-based
work-tracking system with strict discipline. It is tucked into a dotfolder for
the same reason `.git/` and `.github/` are — it's *infrastructure for the
project*, not the project's visible content.

## What the engine is

A schema + a set of disciplined processes + a computed-state contract that,
together, track **work being done** with traceability and live suspicion
detection. It tracks the work of building *anything* — software now, a
SysMLv2 organizational model later. The engine is the same regardless of
subject; only the instance data (the requirements, work items, decisions for a
given project) changes.

## Two models — do not conflate them

1. **The engine model** (this folder + a project's instance files) tracks the
   *work being done*.
2. **The deliverable** is whatever that work produces — software, or a future
   org/HR SysMLv2 model. The deliverable is a **separate artifact**. Its
   domain vocabulary (e.g. `Department`, `PensionPlan`) lives in the
   deliverable, **never** in the engine schema.

## Layout

```
schema/core/     Always imported. The universal work-tracking vocabulary.
schema/safety/   Optional. STPA. Import only for safety-relevant projects.
workflows/       The six workflows as native action defs (+ _meta artifact types).
contracts/       The computed-state specification (satisfaction/coverage/suspicion).
processes/       Agile-for-solo+AI, DoR, DoD, architectural-critique, doc-sync.
skills/          AI skill registrations + SKILL.md definitions.
decisions/       Architecture decision records — why the engine is shaped this way.
tools/           query.py, capture_user.py, whats_next.py + validate/ (4 validators).
docs/            Usage guide, syntax notes, tracking template.
```

## How a project uses the engine

A project's instance files live in **`.tracking/`** (see `.tracking/README.md`) and
import the flat schema packages:

```sysml
package MyProjectNeeds {
    private import EngineElement::*;   // bases, enums, value types
    private import EngineNeeds::*;     // + EngineWork / EngineVerification / ... as needed
    // EngineSafety only if safety-relevant
}
```

Copy authoring idioms from `docs/tracking-template.sysml` (it parses green). Query
the tracked work with `tools/query.py`; validate with the four layer validators in
`tools/validate/` (mandatory before commit, CLAUDE.md §5).

## Reuse model

The engine is reused as a **template**: clone the repo, keep `.engine/`, replace
`.tracking/`. There are no pluggable "domain packages" beyond the optional
`schema/safety` import — the engine schema is general enough to track any
project's work uniformly.

## Status

The schema and instance files parse green against the OMG pilot kernel (validated
2026-06-10/11; see `docs/sysmlv2-syntax-notes.md` for confirmed do's/don'ts). The
write API, indexer, and GUI do not exist yet — direct text editing is the
sanctioned bootstrap write path (CLAUDE.md §4).
