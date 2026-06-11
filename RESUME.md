# RESUME — where this project stands

Handoff pointer for a fresh context. **The backlog is the tracker** — do not duplicate
it here (this file once shadowed it; that was a recorded critique finding).

## What this is

A reusable, AI-complemented **work-tracking engine** built on SysML v2 text files
(`.engine/`), built using its own discipline. Read `CLAUDE.md` first — it is the
interaction contract (triage state machine, invariants, bootstrap rules, validation).

## How to orient (ORIENT → §3f)

```
python .engine/tools/query.py              # whats-next: ready / blocked / done / suspect
python .engine/tools/query.py outstanding  # everything not done
```

(Run via `conda run -n sysml --no-capture-output ...` when the kernel is needed;
sandbox disabled. PowerShell: never pipe conda-run output into a cmdlet — it hangs.)

## Git state

- **`main` is the only branch** (standing instruction 2026-06-11); every commit
  auto-pushes via `.githooks/post-commit` (`sh ./bootstrap.sh` once per clone enables it).
- Commit convention: `CR:` prefix for process/schema changes.

## Where things are

| What | Where |
|---|---|
| Interaction discipline | `CLAUDE.md` |
| Schema (canonical vocabulary) | `.engine/schema/core/` (+ `safety/`) |
| Workflows (six, as action defs) | `.engine/workflows/` |
| Processes (agile, DoR, DoD, critique, doc-sync) | `.engine/processes/` |
| Decisions (ADRs) | `.engine/decisions/` |
| Tools (query, capture_user, 4 validators) | `.engine/tools/` |
| Instance data (backlog, actors) | `.tracking/` (committed for the self-build) |
| Authoring idioms | `.engine/docs/tracking-template.sysml`, `.engine/docs/usage.md` |
| Syntax do's/don'ts (pilot-verified) | `.engine/docs/sysmlv2-syntax-notes.md` |
| Architecture critiques | `docs/design-history/` (latest: 2026-06-11, 13 CRs accepted) |

## Known constraints (pilot kernel 0.59.0)

`%show` won't render RequirementUsage attribute values (tools read them from text);
`expose`/`render`, `verify X by Y`, `derive`/`refine`/`trace` don't parse; native
`elementId` regenerates per parse (authored `id` is identity); interrupted tool runs
can orphan kernel JVMs (sweep: kill `java.exe` matching `ISysML`).
