> **IMPLEMENTATION STATUS (2026-06-21).** Implemented (Rust `keel` authority; query.py retired
> at M4/D0074): done-from-appended-results, ready/blocked, D0005 suspicion (material-change trigger,
> transitive, #OrderingOnly excluded), evidence validation; **coverageState/satisfaction as the
> three-tier `keel coverage` (verified/attested/addressed, D0082) + `critique-coverage` (D0080) +
> the `assured` composite (D0079) + element-content staleness (D0084)**. Material-change is the
> two-tier proxy described in rule 2 (not full field-hashing — see there). `coverageGaps` = the
> coverage/critique gap sets; `whats-stale-since` = `suspect` + element-content drift. Fixed
> (contractAlign, 2026-06-15): removed stale `currentState` — "done" is computed from appended
> TestResults (CR-7).

# Computed-State Contract

This is the engine's defining mechanism. Read it before building any tool.

**Rule zero (decision 0001 + 0005):** computed values are **views, never
stored**. They are recomputed on demand from authored facts. They do **not**
appear as attributes in any `.sysml` file. The index (Kùzu) may cache them, but
the index is disposable and rebuildable from text at any moment.

## Authored facts (live in text — the inputs)

- Elements and their attributes.
- Typed edges: `:>`, `satisfy`, `verify`, `allocate`, `dependency`, `supersede`.
- `TestResult` log entries: `{ outcome, judgedAt, judgedAgainst (commit SHA), judgedBy }`.
- Git history (commit ancestry).

## Computed views (the outputs)

### `coverageState(element)`
For an element with verifying Tests:
- **covered** — every verifying Test has a latest `pass` result whose
  `judgedAgainst` commit is a **descendant-or-equal** of the element's last
  material-change commit (i.e. the test was judged *after* the change it must cover).
- **suspect** — a verifying Test's latest result is `pass`, but `judgedAgainst`
  is an **ancestor** of (older than) the element's last material change. The
  pass no longer demonstrably covers the current state — needs re-run.
- **failing** — a verifying Test's latest result is `fail`.
- **uncovered** — no verifying Test exists.

### `satisfaction(element)`
- **verified** — `coverageState == covered` AND, transitively, every element it
  depends on (via suspicion-carrying edges) is also `verified`.
- **stale** — would be verified, but a downstream Test result is `suspect`.
- **unverified** — otherwise.

An item is truly **done** only when `satisfaction == verified` (computed from
appended TestResults and git ancestry — `currentState` was deleted in CR-7).
The query ("marked done but not yet verified") is itself a computed view.

### `suspicionState(element)`
- **clean** — no upstream material change postdates this element's coverage.
- **suspect** — an upstream element (reachable via suspicion-carrying edges)
  changed materially at a commit that is a descendant of this element's latest
  covering evidence.

Suspicion is **transitive** up the chain: a suspect leaf makes everything that
`satisfy`/`verify`/`:>`/`allocate`-depends on it suspect until re-verified.

## The three sharp rules (decision 0005)

1. **Happened-before = git ancestry, not wall-clock.** A `pass` covers a change
   only if its `judgedAgainst` commit is a **descendant-or-equal** of the
   commit that made the change. Wall-clock `judgedAt` is display only; clocks
   skew and branches diverge.

2. **Material-change detection.** Only changes to an element's **semantic**
   content bump its "last material change" commit; cosmetic edits (whitespace,
   doc-string typo) should not trigger suspicion. **As implemented (two tiers,
   D0082/D0084 — narrower + cheaper than the original "hash every semantic field"
   design, deliberately so):**
   - **criterion / source drift** — a task's `DoD` `procedureText` changing, and a
     deliverable-manifest source **path** drifting in git, trigger task suspicion
     (orient). This watches the one unambiguous material thing per task.
   - **assurance-element-content drift (D0084)** — for a Need / SystemRequirement /
     accepted Decision carrying a `verify`/critique edge, a change to its **primary
     semantic field** (`statement` for needs/requirements, `decision` for decisions)
     since the verification's latest result commit marks that verification/critique
     **suspect** (re-verify / re-critique). Skipped when the element did **not** exist
     at that commit (so same-commit create+verify isn't falsely flagged).
   General per-element semantic-field **hashing** (the original D0005 wording) is the
   aspirational superset — deferred: the targeted tiers above cover the elements that
   carry verification, avoid the fragile semantic-vs-cosmetic field classification
   (the storm risk), and avoid per-element historical reads (orientPerf). Revisit if a
   concrete need for broader element suspicion appears.

3. **Suspicion-carrying edges only.** Propagate along `:>`, `satisfy`,
   `verify`, `allocate`. Do **not** propagate along `dependency` tagged
   `@OrderingOnly` (B-after-A ordering is not semantic dependence). Untagged
   semantic dependencies DO carry suspicion.

## Re-verification by method

When an element goes `suspect`, its verifying Tests must be re-run:
- `method == test | analysis` → re-run automatically (CI / tool).
- `method == inspection | demonstration` → re-ask a human; append a new
  `TestResult` with the human as `judgedBy`.

## PR-reviewer blindness (solved, not stored)

Raw `git`/PR diffs show **evidence** (edges, results), not **verdicts**
(computed state). A CI bot computes the deltas at review time and **comments**
them on the PR ("this change put 7 items into `suspect`"). The verdict is never
committed to text.

## Other computed views

- `workflowProgress(epic)` — rolled up from child work items.
- `supersededBy(element)` — inverse of the `supersede` edge.
- `orphans()` — elements with no upstream derivation.
- `coverageGaps()` — requirements with no verifying Test.
