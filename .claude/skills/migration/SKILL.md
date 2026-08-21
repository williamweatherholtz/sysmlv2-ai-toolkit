---
name: migration
description: |
  Deploys the Safe Bulk Migration process (D0067): when a change requires editing the same
  field/shape across many instances or files (rename/split/drop/add a field; a representation
  refactor of authored model data or schema), run it through the gated expand/migrate/contract
  lifecycle so the tree is green at every step, the transform is a reviewed + reconciled script,
  and recorded data is never fabricated or destroyed. Use for ANY bulk migration of .sysml model
  data or schema/core. This is the deploying skill for .engine/processes/migration.sysml (D0059).
metadata:
  version: 0.1.0
  domain: [migration, schema-evolution, refactoring, expand-migrate-contract, data-integrity, SysMLv2]
  writePolicy: direct
  engine: keel-ai-toolkit
---

# migration — safe bulk migration (expand / migrate / contract)

A bulk migration touches the same field/shape across many sites. Done ad-hoc it silently
corrupts or misses data (a line-regex missed ~17% of multi-line `decisionText` values; only a
count check caught it). This skill makes the catch STRUCTURAL — reconciliation is a gate, not luck.
Spine: Expand/Migrate/Contract (Fowler) + Refactoring Databases (Ambler) + Google LSC + event-
sourcing never-mutate-history + Flyway/Liquibase discipline + ETL control totals + provenance integrity.

## The gates (run in order; each is a hard gate)

0. **Justify & classify.** Record a `Decision` with rationale + impact (site/file counts). It's a
   CHANGE (§3a) — get human acceptance; run the migration inside a sprint (D0064).
1. **Author the transform as a committed script.** Write a codemod under
   `.engine/tools/migrations/`, reviewed + committed — never hand-edit across many sites.
   **Prefer structural/AST matching over a line-regex** (regex misses multi-line, concatenated
   (`"a" + "b"`), and aliased values). If you must use regex, step 2 is MANDATORY. Make it
   **idempotent** (skip a site already in the new shape) with a **dry-run default** + `--apply`.
2. **Dry-run + reconcile (control totals).** Before applying, assert the totals BALANCE:
   - **conservation**: `new_field_count == old_field_count` (rename/split conserves count);
   - **no-leak**: `migrated + skipped + errored == total`;
   - **content**: hash the unchanged portions (counts catch misses; hashes catch corruption).
   **FAIL on any mismatch.** Validators must be green on the projected output. *(This gate catches
   the silent-miss class.)*
3. **EXPAND.** Add the new shape additively (`[0..1]`); old + new both valid; validators green; commit.
4. **MIGRATE.** Apply the transform; shard large changes, commit green per shard. **Copy-and-transform
   recorded facts** (never mutate/destroy attestations + provenance). **Backfill before you tighten**
   any constraint. **Historical-data rule:** grandfather (cutoff) OR backfill-with-recorded-basis OR
   explicit human attestation — **NEVER fabricate who/when/provenance** (D0016/D0051).
5. **Reconcile post-apply.** Re-run the invariant check; old-shape usage must reach **0** (minus
   documented grandfathered cases) before contract. Validators green.
6. **CONTRACT + guard.** Only at usage `== 0`: remove the old shape. **Never hoist a constraint a
   legitimate subtype can't meet** (LSP/ISP — e.g. `createdBy [1..1]` breaks `TestResult`, which uses
   `judgedBy`). Add a **permanent guard** (validator/lint) preventing reintroduction (D0047).

## Anti-Patterns

- **Hand-editing across many sites** — author a committed script; the transform is a reviewed artifact.
- **Line-regex on structured values** — multi-line/concatenated/aliased values defeat it; parse, or reconcile.
- **Apply without a reconciled dry-run** — counts must balance first; FAIL on mismatch.
- **Tighten a constraint against violating rows** — backfill first, verify zero violators, then tighten.
- **Hoisting an unsatisfiable constraint to a base type** — put the obligation on subtypes that can meet it.
- **Fabricating historical who/when** — grandfather or backfill-with-basis; never synthesize attestation.
- **Contract before usage == 0** — Fowler: skipping/early-contracting leaves you worse than the start.

## Output Format

```yaml
migration: "<what field/shape, scope: N sites / M files>"
decision: D00NN (accepted)
transform: ".engine/tools/migrations/<script>.py (idempotent, dry-run default)"
reconcile: { conservation: new==old (N==N), no_leak: migrated+skipped+errored==total, content_hash: ok }
expand:   green @ <sha>
migrate:  green @ <sha> (shards: ...)
post_reconcile: old_shape_usage == 0 (grandfathered: [...])
contract: old shape removed @ <sha>; guard: <validator>
historical_data: grandfathered | backfilled-from <basis> | attested-by <human> — never fabricated
```

## Questions This Skill Answers

- "Rename / split / drop / add a field across the model" / "bulk-migrate the instances"
- "Is this migration safe to apply?" / "How do I not lose data?"
- "Why dry-run + reconcile?" / "When can I drop the old field?"
