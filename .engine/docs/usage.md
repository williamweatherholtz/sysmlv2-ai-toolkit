# Engine Usage Guide

How to use the work-tracking engine, for humans and AI agents.

## Mental model

Every artifact — work items, requirements, tests, decisions, safety analysis —
is a typed element connected by typed edges (`:>`, `satisfy`, `verify`,
`allocate`, `dependency`, `supersede`). The model is `.sysml` text. There is no
other tracking system (no GitHub Issues, no kanban app). Computed state
(satisfaction, coverage, suspicion) is a **view**, recomputed from the text +
git history — never stored.

## Starting a project on the engine

The project's root file imports the schema:

```sysml
package MyProject {
    import Engine::Core::*;
    import Engine::Safety::*;   // only if safety-relevant
    // requirements, work, tests, decisions go in top-level instance files
}
```

Instance files live at the repo top level (`requirements/`, `work/`,
`architecture/`, `decisions/`) — visible and project-specific.

## Day-to-day

| You want to... | Do this |
|---|---|
| Add new work | Create an `Epic`/`Story`/`Task` (set `kind`); link via `dependency`/`satisfy`. |
| Define "done" for a Story | Create atomic `Test`s; link with `verify`. No text-blob criteria. |
| Start work | Pass the Standup / Definition-of-Ready gate, then set `currentState = in_progress`. |
| Record a result | Append a `TestResult` with `outcome`, `judgedAgainst` (commit SHA), `judgedBy`. |
| See what's affected by a change | Query `whats-stale-since <ref>` (once the query tool exists). |
| Record a decision | Create a `Decision`; supersede an old one via the `supersede` edge. |
| Register an AI skill | Add an `AISkill`/`Agent` to the skills registry with a `writePolicy`. |

## Statuses (default workflow — modular, decision 0009)

`backlog → ready → in_progress → in_review → done` (+ `blocked`). States are
data; a project may define others (e.g. `deployment`, `ai-review`).

## Edge cheatsheet

| To say... | Use |
|---|---|
| X fulfills requirement R | `satisfy R by X;` |
| Test T checks element E | `verify E by T;` (validation = verify a need/market-req) |
| B derives from / refines A | `B :> A;` (specialization) |
| B can't start until A | `dependency` (tag `@OrderingOnly`) |
| Function F runs on component C | `allocate F to C;` |
| D2 replaces D1 | `dependency ... : Supersede { source=D2; target=D1; }` |

## What never goes in the model

Runtime events (CI runs, image digests, telemetry), and any deliverable-domain
vocabulary (e.g. `Department`, `PensionPlan` — those belong in the *deliverable*
model, not the engine).

## For AI agents

You use the same API as the GUI. Your `writePolicy` is set in the skills
registry and enforced by the API — you cannot change it. `read-only` = query
only; `pr-only` = open a branch + PR; `direct` = commit to main (mechanical
bookkeeping only). When a Standup/DoR check fails, invoke `grill-me` to drive it
to resolution before starting work.
