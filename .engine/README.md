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
contracts/       The computed-state specification (satisfaction/coverage/suspicion).
processes/       The disciplined workflow (agile-for-solo+AI, DoD, DoR).
skills/          Default AI skill registrations.
decisions/       Architecture decision records — why the engine is shaped this way.
docs/            Usage guide.
```

## How a project uses the engine

A project's root SysMLv2 file imports the engine schema:

```sysml
package MyProject {
    import Engine::Core::*;          // always
    import Engine::Safety::*;        // only if safety-relevant

    // ... the project's requirements, work items, tests, decisions ...
}
```

The project's instance files (`requirements/`, `work/`, `architecture/`,
`decisions/`) live at the repo top level — visible, project-specific, and
replaced when the engine is reused as a template for a new project.

## Reuse model

The engine is reused as a **template**: clone the repo, keep `.engine/`,
replace the top-level instance files. There are no pluggable "domain packages"
beyond the optional `schema/safety` import — the engine schema is general
enough to track any project's work uniformly.

## Status / caveat

**SysMLv2 textual syntax in these files is PENDING VALIDATION against the
pilot implementation** (tracked work item: validate-against-pilot). The
*conceptual* schema — the types, relationships, and semantics — is settled;
the exact keyword spelling may shift when first parsed. Do not treat the
syntax as proven.
