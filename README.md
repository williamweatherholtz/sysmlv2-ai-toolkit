# Keel

**Keel** is a reusable, AI-complemented **work-tracking engine** built on SysML v2 text files,
with strict, *computed* discipline. It tracks the work of building anything — requirements,
decisions, verification, and traceability — as authored text, and computes everything derivable
(status, coverage, suspicion, reports) on demand. The AI drives the CLI; the human supervises.

The keel is what lets a ship carry full sail without capsizing: the discipline isn't a brake on
the AI — it's what turns raw output into clean forward motion.

> *(The repository is named `sysmlv2-ai-toolkit` after the **methodology** it's built on; the
> **product** is Keel — the `keel` binary.)*

---

## Quickstart (use Keel on your project)

**1. Get `keel`.** Download the prebuilt binary for your platform from the latest
[GitHub Release](https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/releases), *or* build
from source (see [Build from source](#build-from-source)).

**2. Scaffold a project.**
```
keel init myproject
cd myproject
git init && git config core.hooksPath .githooks   # enable the Rust-only keel pre-commit gate
```
`keel init` lays down the engine (`.engine/`, with the architecture decisions as read-only
`reference/`), `CLAUDE.md` (how to work here), a starter `.tracking/`, and a kernel-free
`.githooks/pre-commit` (runs `keel validate` + `keel guard`).

**3. Start working.** Read `CLAUDE.md`, then either run the guided **`introduction`** skill
(captures your first need and runs your first sprint) or jump straight in:
```
keel orient .        # where things stand (computed — never a prose status doc)
keel whats-next .    # the ready frontier
```

Your project authors its own facts in `.tracking/` and its own decisions in `.engine/decisions/`;
the engine's design rationale stays read-only in `.engine/reference/decisions/`.

---

## Core ideas

- **Text is truth; everything derivable is a computed view.** Status, coverage, traceability,
  suspicion, and reports are *queried*, never stored — there is no status page or handoff doc.
- **Atomic items, typed edges.** Every item has an immutable UUID `id` and connects to others only
  through the typed edge algebra (`satisfy`/`verify`/`allocate`/`:>` + `#Resolves`/`#Supersede`/…).
- **Computed state.** *Done* = the latest appended `TestResult` is a pass; git-ancestry **suspicion**
  flags work whose upstream definition changed since it was verified (re-verify to clear).
- **Two models, never conflated.** The engine model tracks the *work*; the deliverable is what the
  work produces. Deliverable vocabulary never enters the engine.
- **Authorization is the human commit-gate** (D0094/D0096): the agent runs under the discipline and
  never auto-commits — the human's commit *is* the boundary.

## The `keel` toolchain (no JVM)

A single Rust binary is the authority for the routine path — no kernel, conda, or Jupyter required.

| Area | Commands |
|---|---|
| Orient / flow | `orient` · `whats-next` · `suspect` · `outstanding` |
| Author (write API) | `append-result` · `append-gate-result` · `add-task` · `apply-review` |
| Assurance | `assured` · `coverage` · `critique-coverage` · `critique-policy` · `attestation-coverage` · `concern-coverage` · `dispositions` · `open-issues` |
| Views / reports | `view <name>` · `render <view>` · `diagram` · `report <kind> [--html]` · `indicators` |
| Trace / govern | `trace` · `trace-need` · `rootedness` · `tier-satisfaction` · `governing-version` · `reprocess-candidates` · `audit` · `orphans` |
| Gate | `validate` · `guard [name]` · `check` |
| Console / spin-up | `init DIR` · `serve [--port N]` (localhost oversight console) |

> **`serve` agent bridge is optional.** The read console, views, and reports work with the `keel`
> binary alone. The in-console *actions* (critique / investigate / report an element) shell out to a
> local `claude` CLI, so they need [Claude Code](https://claude.com/claude-code) installed, on `PATH`,
> and logged in to your Claude subscription/enterprise — **never** set `ANTHROPIC_API_KEY` (that forces
> API-rate billing). Without it the console degrades gracefully: a clear "not installed" message, not a
> failure. The agent never commits — your commit is the gate (D0096).

## Where things live

```
CLAUDE.md     The interaction contract (request triage, invariants, validation). READ FIRST.
.engine/      The engine (like .git/): schema/, workflows/, processes/, skills/, decisions/,
              views/, contracts/, docs/. (Downstream: architecture decisions are read-only under
              reference/decisions/.)
.tracking/    This project's instance data: backlog (the only tracker), business needs,
              requirements, work items, issues, decisions, test results, critiques, delivery sprints.
docs/         Design history.
```

---

## This repository (the engine's own source-of-record)

This repo is **Keel building itself** — every task, decision (`.engine/decisions/`, currently
d0001–d0096), and verification here is tracked by the engine, so the `.tracking/` history *is* the
primary evidence that the process works (the self-build / dogfood). Future engine modifications
happen here, under the same discipline.

- **State is computed:** `keel orient .` — there is no status/roadmap/handoff doc (Decision 0018).
- **Discipline:** `CLAUDE.md §2` is normative. Every schema/process change is a recorded `Decision`
  + acceptance + green validation; a pre-commit hook enforces it and the post-commit hook pushes to
  `main` (the only branch). CI (`.github/workflows/ci.yml`) runs `cargo test` + `clippy -D warnings`.
- **Deep `.engine` SysML semantics** are checked by the OMG pilot Jupyter kernel (conda env `sysml`)
  on demand / in the pre-commit hook for `.engine` edits; the routine `.tracking` path is Rust-only
  (`keel validate`/`guard`). This dev-only kernel toolchain is **not** shipped to `keel init` projects.

### Build from source

```
cargo build --release          # produces target/release/keel(.exe)
cargo test --workspace         # unit + BDD suites
cargo clippy --workspace --all-targets -- -D warnings
```

Requires a recent stable Rust toolchain (MSRV 1.96).

## License

MIT — see [`LICENSE`](LICENSE).
