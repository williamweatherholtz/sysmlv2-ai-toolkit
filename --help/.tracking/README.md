# .tracking/ — your project's instance data

This directory holds THIS project's authored facts (needs, requirements, work items, issues,
decisions, test results) — the per-project INSTANCE. The reusable engine lives in `.engine/`.

Getting started: run the `introduction` skill (guided onboarding), or author your first `Need`
following `.engine/docs/tracking-template.sysml`. State is COMPUTED — run `keel orient .` to
see where things stand. The engine's design rationale is read-only in `.engine/reference/decisions/`;
your project authors its OWN decisions fresh in `.engine/decisions/`.
