# sysmlv2-ai-toolkit

A SysMLv2-based work-tracking toolkit and template repository. The toolkit itself
is being built **using its own workflow** — eating the dog food from day one.

## What this is

- A **template** for project repositories that use SysMLv2 as the typed,
  text-file work-and-architecture spine.
- A **toolkit** of tools (parser bindings, query CLI, indexer, API service,
  browser GUI) that operate on a project's `.sysml` files.

The same repository serves both purposes. The directories under `conventions/`,
`processes/`, and `tools/` (once built) lift cleanly into any new project. The
contents of `requirements/`, `architecture/`, `decisions/`, and `work/` are
specific to *this* project (building the toolkit) and would be replaced when
seeding a new repo.

## Repository layout

```
conventions/      Frozen base types — WorkItem, AISkill, Decision, etc.
                  Lifts into any project. Treat changes here as architectural.

processes/        The agile workflow modeled as SysMLv2 processes. Lifts.

requirements/     What the toolkit must do. Project-specific.

architecture/     How the toolkit is structured. Project-specific.

decisions/        ADRs in SysMLv2 form. Project-specific (the decisions are),
                  but the convention lifts.

work/             Epics, stories, tasks. Status lives here. Project-specific.

skills/           AISkill / Agent registrations pointing to skill prompts on
                  disk. Lifts the convention; the specific skills are
                  project-specific.

docs/             Human-readable guides on using the workflow.

tools/            (To be built.) Parser, indexer, query CLI, API, GUI.
```

## The discipline

1. **Files are truth.** No parallel work tracking in GitHub Issues, Linear,
   Jira, or a kanban board. If it isn't in a `.sysml` file, it isn't tracked.
2. **Typed relationships only.** Use `satisfy`, `verify`, `refine`,
   `dependency` — not English prose pointing at other items.
3. **`conventions/` is frozen.** Changes are themselves architectural decisions.
4. **State, not events.** The model records *what we decided to build* and
   *what state items are in*. Runtime events (CI runs, image digests,
   telemetry) live in their native systems and are referenced by URI when
   relevant.
5. **AI agents are first-class.** The API surface and the GUI are the same
   API surface AI agents use. No private channels.

## Status

Phase 0 — bootstrap. See `work/phase-0-bootstrap.sysml`.

## SysMLv2 syntax note

SysMLv2's textual notation is still maturing (OMG specification finalized
2023+, pilot implementation ongoing). The `.sysml` files in this repo aim for
current syntax but may need adjustment against the pilot implementation's
parser. Validate with the pilot impl before relying on the syntax in any new
file.
