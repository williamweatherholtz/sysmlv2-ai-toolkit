# sysmlv2-ai-toolkit

A reusable, AI-complemented **work-tracking engine** built on SysMLv2 text
files, with strict-yet-flexible discipline. It tracks the work of building
anything — and is being built using its own workflow.

## What this is

The engine is a SysMLv2 **schema** + disciplined **processes** + a
**computed-state contract** that track *work being done* with full
traceability and live suspicion detection: change an upstream item and every
downstream item that may now be stale is found by a graph query, not a manual
hunt.

**Two models, never conflated:**
1. **The engine model** tracks the *work* (this repo).
2. **The deliverable** is what the work produces — software, or a future
   SysMLv2 org/HR model. A separate artifact; its domain vocabulary never
   enters the engine.

## Where things live

```
.engine/        THE ENGINE — infrastructure (like .git/). See .engine/README.md.
  schema/core/    Always imported: requirements, work, tests, decisions,
                  risk, process, workflow, skills + the Element/relationship base.
  schema/safety/  Optional: STPA (HARA/ASIL intentionally out of scope).
  contracts/      The computed-state spec (satisfaction/coverage/suspicion).
  processes/      Agile-for-solo+AI, Definition of Ready (Standup), Definition of Done.
  skills/         Default AI skill registrations + write policies.
  decisions/      0001–0010: the architecture decisions behind the engine.
  docs/           Usage guide.

(top level)     PROJECT INSTANCE — created next phase, using the engine:
                requirements/, work/, architecture/, decisions/ for the tools,
                then the tool source itself.
```

## The discipline (engine invariants)

1. **Text is truth; computed values are views.** Authored facts live in
   `.sysml`; satisfaction/coverage/suspicion are recomputed, never stored.
2. **Atomic items.** Tests, decisions, requirements are first-class and
   independently queryable — never checklist lines inside other items.
3. **Typed edges only:** `:>`, `satisfy`, `verify`, `allocate`, `dependency`,
   `supersede`.
4. **State, not events.** Runtime events (CI, images, telemetry) stay in their
   native systems.
5. **AI is first-class** and uses the same API as the GUI, gated by an enforced
   per-skill write policy.
6. **`schema/core` is frozen**; `schema/safety` is optional; changes go through
   a Decision.

## Build order

- **Phase 0 (done):** the `.engine/` SysMLv2 infrastructure — authored by hand
  (it can't track its own bootstrap).
- **Next:** validate all `.sysml` against the SysMLv2 pilot implementation
  (syntax is currently *unproven*), then use the engine to track building the
  tools (parser, indexer, validator, query CLI, API, browser GUI).

## Caveat

SysMLv2 textual syntax in `.engine/` is **pending validation** against the
pilot implementation. The conceptual schema is settled; exact keyword spelling
may shift. Don't treat the syntax as proven.

## License

MIT — see `LICENSE`.
