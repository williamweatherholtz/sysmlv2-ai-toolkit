# Guard reference

`keel guard` runs **45** forward guards, kernel-free — hard-blocking (exit ≠ 0 on any violation)
and warning-only (visible every commit, never blocking). `keel version` reports the exact split
computed from the enforced set — read it there rather than here, so the number has one home.

Run one: `keel guard <name>`. This file is the catalogue; CLAUDE.md §5 has the commands.

## Hard-blocking

| Guard | What it enforces |
|---|---|
| `actors` | `createdBy`/`judgedBy` reference a registered `ProjectActors` entry (D0037) |
| `acceptance-events` | An accepted Decision carries a passing acceptance event (D0066) |
| `question-coverage` | Declared knowledge facts are well-formed (D0161): a Question carries its text, an Alias carries its term and maps to an existing element. Well-formedness only - coverage stays a view; absent `.knowledge/` = unplugged, green. Unit-owned by `knowledge-graph-memory` |
| `confirmation-authenticity` | That acceptance event is **human**-judged, never AI-fabricated (D0106/issue059), and — when the attestation policy declares a recording delegation (D0192 option A) — that a delegated acceptance actually quotes the human's words or cites their gesture, forward-only from the delegation date. Rule-sourced from both `confirmationAuthenticityRule` and `delegatedAcceptanceSubstanceRule` |
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
| `edge-endpoints` | Every typed-edge endpoint resolves to a declared item (issue109). A `#Marker dependency from A to B;` whose endpoint is declared nowhere asserts a relationship that is not there, and is worse than a missing edge because every consumer treats it as present: an Issue read as *triaged* by a resolver declared in no commit, and a Story read as *chartered* by an origin that never existed. Both passed validate, check-engine and all 28 other guards, because each checks that the EDGE is present and none resolved its endpoints. Found by the conformance lane, not by any Rust check. AST-based, so a marker written inside a `description` string cannot produce a false hit — a text scan for the same thing reports two extra hits from D0133's own prose about edges, the self-referential-corpus trap that inflated the census in issue099 |
| `type-collision` | A PROJECT type must not shadow an ENGINE type (D0128). Projects declaring their own domain types in `.tracking/` is supported and wanted — it already resolves. A project `part def Story :> Element` is not: `Story` is the type `orient` counts work by, and whichever definition wins the other is silently ignored, so a computed view starts counting something other than what the reader believes. Names only, deliberately: whether the project meant to extend or replace is not decidable from the text, and a guard that guessed would be wrong in one direction. Starts at zero — 91 engine defs against 305 project defs, no collision |
| `attribute-vocabulary` | Every `:>> name =` on an ENGINE-typed element must name an attribute that type declares, following `:>` inheritance (issue118). The engine checked markers (D0133) and enum members but never attribute NAMES: a `CodeElement` authored with `codeHsah` and `riskClas` passed validate, all guards and the fast gate — 329 files clean — while silently losing its risk classification, so it sat in the audit frontier as `correctness` when its author wrote `dataLoss`. An undeclared attribute is accepted and the value is simply LOST. Types the engine schema does not declare are PROJECT types and are skipped entirely, never judged — a binary with opinions about a project's own vocabulary is what blocked every commit in issue090. Vocabulary is DERIVED from the schema embedded in the binary, so it cannot drift from what the schema says. Starts at zero — 40355 assignments scanned, no violation |
| `resolver-kind` | Every `#Resolves` resolver is a declared `action` (work that closes the Issue) or a `Decision` (which moots it) — never a requirement, Need, Test or Story, none of which is an act and none of which can ever compute as complete against the Issue (issue136). `issues` checks only that the edge EXISTS, and its own message already told authors the resolver should be an action or Decision — unchecked, so an edge pasted from the line above it pointed a viewer `SystemRequirement` at an Issue about a flag parser, and `open-issues` reported `untriaged: false`. A nominally-triaged Issue is worse than an untriaged one because it reports as handled. Deliberately a bespoke predicate rather than an `EdgeRule` (the D0107 bad-fit precedent): `objectType` filters by declared item TYPE and an `action` is not a typed element, so the 127 legitimate action resolvers would all fail. Starts at zero once the two pasted edges are repointed — 134 edges scanned |
| `stale-gate-prose` | No comment claims a gate is PENDING when its acceptance result PASSES (issue140). Prose frozen at authoring time contradicting the record beside it is the D0018 defect inside the model files, and it is not harmless: `keel-viewer.sysml` carried "PENDING human acceptance of N-18 — NOT yet accepted" on the line ABOVE N-18's passing acceptance result, and the AI believed the comment over the record and published a false claim in a landed critique — a reader has no reason to distrust a comment sitting next to the thing it describes. PRECISE: fires only when a comment within three lines of a PASSING `*Accept*` TestResult claims the opposite, so a PENDING note beside a gate that genuinely is unsigned is correct and untouched. Verified both directions — 148 acceptance results scanned clean, and the exact stale comment re-injected fails with its file and line |
| `ownership` | Only an item's OWNER edits its fields (D0108/D0129). A non-owner may ADD items and typed edges, or SUPERSEDE — never overwrite in place. Compares the staged file against its HEAD blob AST-to-AST rather than by diff lines, because a hunk does not know which item a line belongs to and misattributing an edit is the one thing an ownership check must not do. Adding an item, adding an edge and superseding are invisible to it *by construction* rather than by an exemption list that could drift. An unbound actor is a violation, not a pass: an unattributable edit is worse than a refused one. EXEMPTION, and the only one: a commit that co-commits a transform under `.engine/tools/migrations/` may cross ownership freely (D0067 bulk migration), announced as a warning so the suspension is visible — the same keystone shape `process-change` uses, and it cannot be claimed, only committed (issue113) |
| `impossible-evidence-date` | A `TestResult` may not cite a `judgedAgainst` commit that was made AFTER its `judgedAt` (issue144/D0162). A judgment cannot be made against a commit that did not exist yet. This is the control for fabricating an attestation MECHANICALLY: a bulk stamp that rewrites every `PENDING` SHA in a file will happily point a human's day-old `method=confirmation` result at today's commit, leaving every field well-formed and every other attestation guard green. Forward-only from 2026-08-18, keyed on the CITED COMMIT's date rather than on `judgedAt` so no future stamp escapes by being attached to an old judgment; the thirteen exempted cases are two midnight-rollover sessions. |
| `identity-present` | Every id-bearing declaration actually carries an `:>> id` (issue166). Closes an UNGUARDED invariant: section 1.3 makes identity an immutable UUID, and `keel validate` passed with an `Issue` missing its id entirely. `engine-lint` is `.engine`-scoped by design, the python tracking validator is demoted (D0132), and `duplicate-identity` catches two items sharing an id while saying nothing about an item having none. A TEXT SCAN over `.tracking` + `.engine`, ~200ms for 8738 declarations with no model build. No grandfather line: the corpus was measured first and nothing was missing, so an exemption would have been ceremony over an empty set. |
| `identity-well-formed` | Every `id` is SHAPED like a UUID — 8-4-4-4-12 of `[0-9a-z]` (issue170/D0168). Guard 37 checks an id is PRESENT and `duplicate-identity` checks two items do not SHARE one; the middle property was enforced by nothing, and `id = "not-a-uuid-at-all"` passed validate and all 37 guards. A malformed id is still UNIQUE, so nothing in the model ever notices. SHAPE not strict hex: 78 ids deliberately carry a mnemonic suffix (`…-000000000i01`) that is shaped but not hexadecimal. Grandfathers 15 historical malformations as an EXPLICIT LIST whose size is asserted by a test — not a date line, which is how guard 36's first version exempted the defect it existed for. Identity is immutable (§1.3), so those 15 are excused, never rewritten. |
| `scaffold-placeholder` | No `.sysml` in the model carries the scaffold's `KEEL-SCAFFOLD-FILL-ME` token (dcSprintScaffold). `keel new sprint` writes every judgment-bearing text as the token so an unfilled skeleton is honest about being unfilled — this guard makes that honesty enforceable: it cannot pass a gate or be committed, by construction. Also runs in the fast per-edit tier (`keel gate --fast`), so the rejection lands at edit time rather than commit time. |
| `claude-surface-drift` | The keel-owned subset of `.claude/` (five hook events, output style, per-registry skills) matches this binary's generator (D0174/P0.2) — the check IS `keel sync-claude --check`, one implementation. A project with no `.claude/` passes with zero scanned (CLI + commit/CI gates remain its enforcement). Version skew is a WARNING (regenerate obligation); drift in a keel-owned hook command is a violation — a mutated hook is a silently weakened control (K7). Foreign (user-owned) entries are never inspected. |
| `tool-reference` | Every `.engine/tools/<file>` named on the LIVING doc surface (processes, skills, docs, contracts, workflows, rules, CLAUDE.md) exists on disk (issue196). Sprint 377's closeOut recorded the python deck generator as deleted while the file still existed with two live references — a claimed deletion nobody ran `ls` against; the guard's first dry run found a second stale reference in another skill. Scope excludes decisions and `.tracking`: historical records may truthfully name tools that no longer exist. |
| `attestation-authority` | A disposition of a finding at or above Medium must be judged by a registered `Person` (D0092/D0080). Extends to dispositions the human-only rule `confirmation-authenticity` already enforces for Decision acceptance. Medium AND ABOVE only: D0080 explicitly permits an AI to disposition a LOW finding, and this repo contains a correct documented example — a guard demanding a human on every disposition would fail legitimate work and force either a false attestation or a bypass |

## Warning-only

| `decision-scaffolding` | An accepted `#ProspectiveChange` Decision must be reachable by a tracked-item edge (charteredby/derivedfrom/resolves/satisfy) — it promises process change, so it charters work (D0188, answering the human's "the scaffolding under a decision isn't being made"). WARNING-tier by D0188's composed rule with D0180: promotion to hard is a recorded review citing the fire-ledger window. Forward-only from D0188's own recorded acceptance date, read from the model; the newest violator is exempt (the landing-sprint grace). |
| `release-recorded` | Every local version tag (`v`-prefixed) has a `Release` item whose title names it and whose recorded commit matches the tag's commit (D0191, owned by the `deploy` unit) - process-enforcement.toml's own admitted checkable claims, unguarded until guard 43. WARNING-tier per the guided profile's advisory-first rule; zero tags scans zero, so a project without releases is untouched. First run caught both defect shapes: v0.1.0 had no Release item (backfilled), and v0.2.0's tag sits two housekeeping commits past its recorded commit (standing, accurate, owner's to reconcile). |
| `enrollment-binding` | When a machine binding (`.keel/actor`) exists, its name resolves to a registered `Person` or to an `Actor` carrying a declared kind (D0191, owned by the `actor-enrollment` unit). Until guard 44 NOTHING validated the binding file - an unregistered or kindless name surfaced only when a downstream write refused. WARNING-tier; an absent binding scans zero (binding is per-machine and optional until a write needs an actor). |
| `control-event-coverage` | Every control-relevant event is DECLARED in `.engine/contracts/control-events.toml` with its required record, and the declaration matches what the binary emits - a two-way diff (declared-not-emitted warns as a dead declaration; emitted-not-declared warns as an uncounted event). Closes the four-instance invisible-control-event family (issues 203/205/207 + the sr13 sentinel) with ONE control instead of per-instance sweeps (D0193/D0047). WARNING-tier; an absent contract reports not-adopted (D0136). |


Visible on every commit, never blocking — the D0102 *promote-once-low-noise* pattern.

| Guard | What it flags |
|---|---|
| `decision-requirement-link` | An accepted Decision naming a requirement in prose with no typed edge (D0102/issue052) |
| `verification-trace` | A **delivered** DoD naming a `SystemRequirement` it never `#Verify`-links (D0130/issue082) |
| `priority-inversion` | A ready item outranking work that resolves a ≥ High Issue (D0130/issue084) |
| `retro-backlog` | A retro naming a finding with no tracked item and no stated reason (D0131/issue085) |
| `doc-sync` | A staged definitional change with no co-committed doc update (D0113) |
| `hook-config-integrity` | A `.claude/settings*.json` hook command referencing a script that does not exist, so it fails every time it fires (D0047/issue093). Warning-level because this config is machine-local and partly gitignored: CI cannot see it, and one contributor's personal hook must never block another's commit |
| `sequence-multiplicity` | Every multi-valued assignment `:>> f = (a, b, c)` in the model (issue101). Enabling the sequence form removed a crude safety property — `(` used to be a parse error everywhere, so it could not be written into a *single-valued* attribute; now `createdBy = ("you", "ghost")` parses clean and `actors` passes. The exact check needs multiplicity metadata the AST does not yet carry, so every sequence is reported instead — useful, not a placeholder, because the model contains **zero** today. AST-based on purpose: grepping `= (` matches the prose that *discusses* sequences (the issue099 self-inflating-census error). Retire when multiplicity lands |
| `parser-coverage` | Statements keel-parser cannot read, grouped by leading token (issue102). The parser recognises a fixed set and skips the rest — silently, until now. Measured with an undeclared target, `ref`, `port`, `assert constraint` and `connect` all validate CLEAN while the control (`part x : NoSuchType`) correctly diagnoses, so they are invisible, not merely unresolved. Every construct D0139 converts toward is in that set, so this is the safety property that makes the base-first pass survivable. Currently reports 165 skipped statements — including 52 `:>` specialization clauses and 57 `use case` usages |
| `base-first-justification` | A PROJECT-declared `metadata def` with no recorded justification (D0139(B)). A custom edge is a last resort; an undocumented one is dialect nobody can audit. The engine's own 17 markers are grandfathered (issue068) and D0140 supplied kernel-verified justifications for the two that needed them, so this fires only on markers a project declares from here on — currently zero. Text-based, because the AST does not capture `doc` clauses; that is a stated limitation, not an oversight |

## Non-blocking burndown (runnable, never gating)

Completeness is **honest state**, not a commit blocker (D0098): incomplete work flagged *as* incomplete
is truthful. `assured` (D0079c composite readiness) · `critique` (coverage) · `critique-rigor`
(D0080/issue030) · `defect-guard-coverage` (D0047/issue039).

## Three things worth knowing

**Forward-only grandfathering.** A new guard must never retro-fail items authored under the process in
force at the time (the issue068 lesson). `duplicate-identity` has NO exemption list — the 18 bootstrap duplicates were
re-identified by a D0067 migration (issue080 resolved), and the list was deleted rather than emptied; `attestation-substance` grandfathers 9 thin attestations. Both lists are
visible every run and must not be extended — a new violation is a defect, not an exemption.

**Hard vs warning is a property of the check, not the topic.** Exact checks (set membership, duplicate
detection) block. Heuristics over prose warn. A guard that fires on ambiguity trains people to disable
it — which is how `issue081` cost eight bypassed commits.

**`sr_verified_pct` is not test coverage.** An SR counts as verified when *any* Test `#Verify`-links it,
so the number is the UNION of two unrelated claims — someone examined the requirement, and the system
was run against it. Use **`keel verification`**, which reports them separately and never as one
number, and `keel verification --pending` for exactly what is outstanding in each. (`keel
tier-satisfaction`'s `verifiedByMethod` counts EDGES; `keel verification` counts distinct
requirements, so a Core-3 critique is 3 there and 1 here.)

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

What each process BRINGS is read from the MODEL: `assert constraint` members on the parts in its
`.engine/processes/` file (D0139(D)). The constraint def is the camelCase form of the guard name.
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
