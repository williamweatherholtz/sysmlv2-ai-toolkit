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
from source (see [Build from source](#build-from-source)). Confirm what you got with `keel version`
— it prints the release version, the commit it was built from, and the guard inventory it carries.
Quote that output in any bug report: guard behaviour is a property of the binary, so the version is
what makes a "keel is blocking me" report diagnosable.

**2. Scaffold a project.**
```
keel init myproject                      # a fresh (empty) directory defaults to --profile strict
keel init existing --profile guided     # a directory with existing content REQUIRES a declared profile
cd myproject
git init && git config core.hooksPath .githooks   # enable the Rust-only keel pre-commit gate
```
`keel init` lays down the engine (`.engine/`, with the architecture decisions as read-only
`reference/`), `CLAUDE.md` (how to work here), a starter `.tracking/`, a kernel-free
`.githooks/pre-commit` (runs `keel validate` + `keel guard`; **fails loud when the binary is
absent**, naming this release page), **the full in-loop enforcement surface** (`.claude/`:
five hook events, the keel output style, discoverable skills, and the protected-path check —
regenerate any time with `keel sync-claude`, drift-checked by `keel sync-claude --check`), and an
optional CI workflow (`.github/workflows/keel-gate.yml`) so verification runs hook-independent.
The adoption profile is **declared, never inferred** (`strict` = blocking gates from day one;
`guided` = advisory-first, promoted later citing measured evidence) and is recorded in
`.engine/contracts/adoption-profile.toml`.

Releases ship verified binaries for Windows, Linux, and macOS; expect the latest tagged release to
be current — install from this repository's
[Releases page](https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/releases) only (no
package managers, no auto-update, no install scripts — deliberately, D0175).

**Harness support:** in-loop enforcement (hooks, output style, protected paths) is
**Claude-Code-bound** today. On any other harness you still get the CLI, the commit and CI gates,
and `keel audit-history` — the layer that re-derives every verdict from the tree, hook-independent.
An MCP surface is a recorded direction (D0186), trigger-gated: if you need keel on a second
harness in real use, say so — that is the trigger.

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
> binary alone. The one in-console AI *action* is a directed, recording **critique** of a named
> element (or a bounded section — sr17 directed-only, no free-form chat); it shells out to a
> local `claude` CLI, so it needs [Claude Code](https://claude.com/claude-code) installed, on `PATH`,
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
