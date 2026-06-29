# Changelog

All notable releases of **keel** — the SysML-v2 work-tracking engine (text-is-truth; every
state computed, every fact authored via the write API, every change gated by honest-state guards).

## v0.1.0 — 2026-06-29

First public release. The engine is self-hosting: it has tracked its own construction through
168 sprints and 104 architecture decisions.

### Distribution
- Prebuilt `keel` binaries for Linux (x86_64), macOS (arm64), and Windows (x86_64) — no Rust
  toolchain required downstream.
- `keel init DIR` scaffolds a fresh project (binary-embedded `.engine/`); the `introduction`
  skill onboards a newcomer to first value.

### What `keel` does (no JVM, no kernel — pure Rust)
- **Orient / plan** — `keel orient`, `whats-next`, `suspect` compute state from authored facts + git
  (no status files; the model is the only tracker).
- **Write API** — `append-result`, `append-gate-result`, `add-task`, `apply-review` author facts with
  enforced UUIDs, provenance (who/when/commit), and append-only semantics.
- **Computed views** — `view`, `render`, `report`, `diagram`, `coverage`, `critique-coverage`,
  `tier-satisfaction`, `rootedness`, `boundary` / `boundary-sweep` (white/black-box subsystem critique),
  `sitting-coverage`, `dispositions`, `indicators` — all regenerable, never stored as truth.
- **Honest-state commit gate** — 13 hard-blocking forward guards + 1 warning guard
  (`keel guard`): truthful / well-formed / traceable, never "complete" (completeness is a
  non-blocking burndown surfaced in `orient`). Decisions must carry a substantive rationale;
  interconnects are typed edges, not prose.
- **Assurance** — antagonistic element critique (lens-tagged verifications), severity-carrying
  findings with typed human dispositions, git-temporal suspicion + `keel reverify` to refresh
  reproducible verifications at HEAD.
- **Interactive console** — `keel serve` (localhost): orient / decisions / sections / boundaries /
  findings / reports, with an optional `claude` agent bridge for directed, recorded critique.

### Releases
- A `v*` tag triggers `.github/workflows/release.yml`, which builds the three binaries and attaches
  them to a GitHub Release.
