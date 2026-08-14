# Guard reference

`keel guard` runs **25** forward guards, kernel-free. **19 hard-blocking** (exit ≠ 0 on any violation)
and **6 warning-only** (visible every commit, never blocking). `keel version` reports this split
computed from the enforced set, so it cannot drift from what actually runs.

Run one: `keel guard <name>`. This file is the catalogue; CLAUDE.md §5 has the commands.

## Hard-blocking

| Guard | What it enforces |
|---|---|
| `actors` | `createdBy`/`judgedBy` reference a registered `ProjectActors` entry (D0037) |
| `acceptance-events` | An accepted Decision carries a passing acceptance event (D0066) |
| `confirmation-authenticity` | That acceptance event is **human**-judged, never AI-fabricated (D0106/issue059). Rule-sourced |
| `attestation-substance` | A passing `method=confirmation` actually **says** something (D0130/issue083) |
| `sprint-coverage` | Substantive work went through a sprint (D0064/issue020) |
| `ceremony` | Sprint gates passed in order (D0047/issue010+011) |
| `charter` | Work traces to a chartering item (D0068) |
| `process-change` | A process-def change co-commits a `#ProspectiveChange` Decision — the D0070 keystone |
| `issues` | Every Issue is triaged with a `#Resolves` resolver (D0077/D0078). Rule-sourced |
| `viewpoint-renderer` | Every declared viewpoint names a real `keel` command (D0056/issue034) |
| `manifest-coverage` | The deliverable-suspicion manifest has no dead entries (D0050/issue033) |
| `critic-independence` | A critique is by an **independent** critic (D0080/issue031) |
| `process-skill` | No inert process — each has a deploying skill (D0059/issue036) |
| `requirement-rootedness` | A `#Capability` feature carries `#DerivedFrom`→Need (D0098/D0099). Rule-sourced |
| `decision-rationale` | Every Decision has a substantive context + rationale (D0103). Rule-sourced |
| `duplicate-identity` | No repeated id, item name, package name, or sequence number (D0129/issue074) |
| `marker-vocabulary` | Every marker used is declared — the engine's own algebra is **builtin** and needs no declaration; project markers declare in any project file (D0133/issue077, D0136/issue089) |
| `engine-lint` | `.engine/decisions/*.sysml` import `EngineWork` (D0112 phase 1) |
| `activation-manifest` | The activation contracts are well-formed — no unknown process or guard name (D0138). Hard because every check is exact set membership, and because a typo would *silently disable a control*, which is worse than a loud failure. Absent files are not a violation |

## Warning-only

Visible on every commit, never blocking — the D0102 *promote-once-low-noise* pattern.

| Guard | What it flags |
|---|---|
| `decision-requirement-link` | An accepted Decision naming a requirement in prose with no typed edge (D0102/issue052) |
| `verification-trace` | A **delivered** DoD naming a `SystemRequirement` it never `#Verify`-links (D0130/issue082) |
| `priority-inversion` | A ready item outranking work that resolves a ≥ High Issue (D0130/issue084) |
| `retro-backlog` | A retro naming a finding with no tracked item and no stated reason (D0131/issue085) |
| `doc-sync` | A staged definitional change with no co-committed doc update (D0113) |
| `hook-config-integrity` | A `.claude/settings*.json` hook command referencing a script that does not exist, so it fails every time it fires (D0047/issue093). Warning-level because this config is machine-local and partly gitignored: CI cannot see it, and one contributor's personal hook must never block another's commit |

## Non-blocking burndown (runnable, never gating)

Completeness is **honest state**, not a commit blocker (D0098): incomplete work flagged *as* incomplete
is truthful. `assured` (D0079c composite readiness) · `critique` (coverage) · `critique-rigor`
(D0080/issue030) · `defect-guard-coverage` (D0047/issue039).

## Three things worth knowing

**Forward-only grandfathering.** A new guard must never retro-fail items authored under the process in
force at the time (the issue068 lesson). `duplicate-identity` grandfathers 18 bootstrap duplicate ids
to warnings (issue080); `attestation-substance` grandfathers 9 thin attestations. Both lists are
visible every run and must not be extended — a new violation is a defect, not an exemption.

**Hard vs warning is a property of the check, not the topic.** Exact checks (set membership, duplicate
detection) block. Heuristics over prose warn. A guard that fires on ambiguity trains people to disable
it — which is how `issue081` cost eight bypassed commits.

**`sr_verified_pct` is not test coverage.** An SR counts as verified when *any* Test `#Verify`-links it,
and in this repo that set is ~70% `method=critique`. Always read `keel tier-satisfaction`'s
`verifiedByMethod` alongside the percentage.

## Marker vocabulary

The engine's own algebra (`#Verify`, `#DerivedFrom`, `#CharteredBy`, `#Resolves`, `#Measures`,
`#Informs`, `#JustifiedBy`, `#Dispositions`, `#Covers`, `#DependsOn`, `#Supersede`, `#OrderingOnly`,
`#ProspectiveChange`, `#SafetyChange`, `#Capability`, `#ProcessDefect`, `#View`) is **builtin to the
binary** and always valid — it is the engine's contract, so a project never re-declares it.

Your **own** markers: add `metadata def YourMarker;` in *any* of your `.engine` or `.tracking` files.
You never need to touch frozen `schema/core`.

Anything in neither set is a violation — that's the typo class, and it is the reason this guard is
hard. A misspelled `#Verify` would silently report a delivered requirement as *unverified*.

> D0136 fixed a regression here: D0133 originally read the declared set only from project schema, so an
> existing project with an older `.engine/` plus a newer binary hit 566 violations and could not commit
> at all. A control shipped in the binary must not depend on content the binary cannot guarantee.

## What this project has ADOPTED (D0138)

A guard is either **core** or **process-bound**. Core guards protect the integrity of the model itself —
identity, provenance, vocabulary, rootedness, well-formedness — and always run. Process-bound guards
belong to a process unit and run only while that process is active:

```
keel activation [ROOT]          # which processes are active; which guards are core
keel activate <process>         # adopt a process as a UNIT: skill + declared rules + guards
keel deactivate <process>
```

`.engine/contracts/process-units.toml` (engine fact) says what each process brings.
`.engine/contracts/activation.toml` (project choice) says which are active. **No file means everything
is active**, so an existing project is unaffected by upgrading.

A skipped guard is always *reported* as `NOT ACTIVE`, never silently dropped — "this control is off" is
precisely what a project needs to be able to see, and `orient` carries `inactive_processes` for the same
reason. Core guards are deliberately in no unit: activation exists to stop enforcing procedures you have
not adopted, **not** to make truthfulness optional. That distinction is what keeps this from becoming the
all-or-nothing bypass that `issue081` cost eight commits to learn about.

This supersedes inference-from-file-presence. Issues 089 and 090 were both really "what has this project
adopted?", answered by guessing from whether a file existed; now it is declared.

## Declared rules

Controls are migrating from bespoke Rust to declared `EdgeRule`/`ElementRule` instances in
`.engine/rules/`, evaluated by `keel rules`. **5 guards are rule-sourced** (marked above); the rest stay
Rust where the check is relational, external-file, text-blob, git-diff-aware, or per-file — D0105's
recorded rollback criterion. Two further rules run warning-only in `keel rules`:
`decisionNoVerdictProseRule` (a Decision restating its acceptance verdict as prose — dual truth) and
`researchSpikeCharterRule` (D0111/issue055).
