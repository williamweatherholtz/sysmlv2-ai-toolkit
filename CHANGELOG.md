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

## v0.3.1 — 2026-08-29

The pin, the wrapper, and the library — the portability release (D0250/D0251/D0252).

### The engine pin is BINDING (D0251)
- `engine-version.toml` escalates from a parity warning to a binding pin: a mismatched binary
  REFUSES writes (at the write-lock choke point, all paths by construction) and gates (in the one
  gate body: `gate`, `sync`, `land`, pre-commit, plus `validate`/`guard` directly). Reads warn and
  proceed; `version`/`migrate` never refuse; an absent declaration keeps working.
- `keelw`: a committed POSIX-sh wrapper resolves the pin against `.keel/bin/<version>/`, downloads
  EXACTLY the pinned version on a miss, verifies the SHA-256 committed in `keel-wrapper.toml`
  (never trust-on-first-use), and never falls back to PATH. `keel init` ships both and seeds the
  cache with the running binary, so a fresh project works offline immediately.

### The library (D0250)
- `keel library init|sync|list`: a machine-local clone of your portable-content repository under
  `<home>/.keel/library`. Sync is fetch + fast-forward ONLY (diverged = a named defect; ahead =
  the sanctioned post-publish state); an unreachable remote is a STATED staleness over the
  last-good cache. A project's gate is byte-identical with the library present and absent —
  availability is never activation, held by test.
- `keel process import --from-library <name>` resolves the cache and delegates to the one import
  path; `keel process publish <name>` exports into the clone and commits (naming unit + version),
  never pushes, and refuses to commit an unchanged unit.

### Assurance
- Guards 53–54: `manifest-key-portability` (absolute AND traversal keys refuse; the unit manifest
  is repository-relative always) and `control-map-reconciled` (every control event names a declared
  control or states why it is instrumentation; dangling `provenBy` proof pointers refuse).
- The control-propriety panel (process + schema + skill + `keel view propriety`): five adversarial
  lenses with a named defeater per verdict. Its first round found and fixed two High defects in
  the session's own controls.
- Arming probes: the write lock, the launcher, the pre-write tiers, the fire ledger, the pre-push
  hook and reverify are now PROVEN to fire by test, not classified by reasoning. The pre-push probe
  found and fixed a field defect: the bootstrap push of a fresh project was refused (issue306).

### Breaking / behavioral
- A project whose `engine-version.toml` names another version now refuses writes and gates under
  this binary — run the pinned version, or `keel migrate` (which re-stamps and announces the
  escalation). Pre-D0190 trees with no declaration are unaffected.
