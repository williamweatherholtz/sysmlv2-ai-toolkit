# sysmlv2-ai-toolkit

A reusable, AI-complemented **work-tracking engine** built on SysML v2 text files,
with strict discipline. It tracks the work of building anything — and is built
using its own workflow (every task, decision, and verification in this repo is
tracked by the engine itself).

## Orient (state is computed, never prose — Decision 0018)

```
./target/release/sysmlv2.exe orient .
```

Returns in-progress sprint ceremony status + the ready/suspect frontier. There is no status page,
roadmap doc, or handoff file: **the backlog (`.tracking/backlog.sysml`) is the only
tracker**, and views over it are computed by the `sysmlv2` Rust toolchain
(`orient | whats-next | suspect | orphans | audit | view <name> | attestation-coverage |
governing-version | reprocess-candidates`).

## What this is

A SysML v2 **schema** (native metaclasses: `requirement def`, `use case def`,
`verification def`, enums, value types) + **processes-as-data** (six workflows as
native `action def`s; agile/DoR/DoD/critique/doc-sync as `Process` instances) +
a **computed-state engine** (done = the latest appended `TestResult` is a pass;
git-ancestry **suspicion** flags work whose upstream definition changed after it
was verified).

**Two models, never conflated:** the engine model tracks the *work*; the
deliverable is what the work produces. Deliverable vocabulary never enters the
engine.

## Where things live

```
CLAUDE.md         The interaction contract (request triage, invariants, validation). READ FIRST.
.engine/          The engine (like .git/): schema/, workflows/, processes/, skills/,
                  decisions/ (0001–0018), contracts/, tools/ (query, validators), docs/.
.tracking/        THIS project's instance data: backlog (the tracker), actors, state cursor.
docs/             Design history (critiques; the original — now historical — design spec).
```

## The discipline (CLAUDE.md §2 is normative)

Text is truth; everything derivable is a computed view. Atomic items, typed edges
(native `satisfy`/`verify`/`allocate`/`:>` plus `#DependsOn`/`#Supersede`/
`#OrderingOnly` markers). Every item carries an immutable UUID `id`. Verification
results are appended, never overwritten. Every schema/process change = recorded
`Decision` + acceptance + green validation (a pre-commit hook enforces it; the
post-commit hook pushes to `main` — the only branch).

## Toolchain

The OMG pilot Jupyter SysML kernel (conda env `sysml`) drives validation and the
typed graph; scalar values are read from text (the kernel won't render them — see
`.engine/docs/sysmlv2-syntax-notes.md` for pilot-verified do's/don'ts). All 47
model files validate green. A standalone parser/runtime is queued work
(`runtimeParser` in the backlog).

## License

MIT — see `LICENSE`.
