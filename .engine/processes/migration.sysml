// =====================================================================
// EngineMigration — safe bulk migration of authored model data + schema (D0067).
//
// Expand/Migrate/Contract spine (Fowler) + Ambler deprecation window + Google LSC
// (transform-shard-guard) + event-sourcing never-mutate-history + Flyway/Liquibase
// (versioned/idempotent/reversible/dry-run) + ETL control-total reconciliation +
// data-provenance integrity. Deployed by the `migration` skill (D0059).
// =====================================================================
package EngineMigration {
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;

    part migration : Process {
        :>> id = "67940516-2c7d-4e8f-a067-516273849600";
        :>> title = "Safe Bulk Migration";
        :>> purpose = "Migrate authored model data + schema at scale (rename/split/drop/add a field across many sites) so the tree is green at every step, the transform is reviewed + reconciled, and recorded data is never fabricated or destroyed.";
        :>> cadence = Cadence::adHoc;
    }

    part mig0Justify : ProcessStep {
        :>> id = "67940516-2c7d-4e8f-a067-516273849601";
        :>> title = "Justify & classify";
        :>> order = 1;
        :>> triggerCondition = "A change requires editing the same field/shape across many instances or files (a bulk migration).";
        :>> actionText = "Record a Decision with rationale + impact estimate (site/file counts). This is a CHANGE (§3a) needing human acceptance; the migration runs inside a sprint.";
        :>> producedArtifact = "A recorded, accepted Decision scoping the migration.";
        :>> owner = Owner::either;
    }
    part mig1Transform : ProcessStep {
        :>> id = "67940516-2c7d-4e8f-a067-516273849602";
        :>> title = "Author the transform as a committed script";
        :>> order = 2;
        :>> actionText = "Write a migration SCRIPT (codemod) under .engine/tools/migrations/, committed + reviewed — never hand-edit across many sites. Prefer structural/AST matching over a line-regex (regex misses multi-line/concatenated/aliased values); if regex is used, reconciliation (step 3) is MANDATORY. Make it idempotent (skip a site already in the new shape) and have a dry-run (default) vs --apply mode.";
        :>> producedArtifact = "A committed, idempotent, dry-runnable transform script.";
        :>> owner = Owner::ai;
    }
    part mig2DryRunReconcile : ProcessStep {
        :>> id = "67940516-2c7d-4e8f-a067-516273849603";
        :>> title = "Dry-run + reconcile (control totals)";
        :>> order = 3;
        :>> actionText = "Run the transform in dry-run. Assert control totals BALANCE before applying: conservation (new_field_count == old_field_count for a rename/split) and no-leak (migrated + skipped + errored == total). Add a content check (hash unchanged portions) — counts catch misses, hashes catch corruption. FAIL the migration on any mismatch. (This gate is what catches the silent-miss class — e.g. the multi-line decisionText regex bug.)";
        :>> producedArtifact = "A reconciled dry-run report; mismatch blocks apply.";
        :>> owner = Owner::ai;
    }
    part mig3Expand : ProcessStep {
        :>> id = "67940516-2c7d-4e8f-a067-516273849604";
        :>> title = "EXPAND";
        :>> order = 4;
        :>> actionText = "Add the new field/shape additively (e.g. [0..1]); old + new both valid; validators green; commit. The tree stays releasable.";
        :>> producedArtifact = "Schema/instances with both shapes valid; green commit.";
        :>> owner = Owner::ai;
    }
    part mig4Migrate : ProcessStep {
        :>> id = "67940516-2c7d-4e8f-a067-516273849605";
        :>> title = "MIGRATE (idempotent, sharded)";
        :>> order = 5;
        :>> actionText = "Apply the transform; shard large changes and commit green per shard. Use copy-and-transform for recorded facts (never mutate/destroy attestations + provenance). Backfill-before-tighten for any constraint. Historical-data rule: grandfather (cutoff) OR backfill-with-recorded-basis OR explicit human attestation — NEVER fabricate who/when/provenance (D0016/D0051).";
        :>> producedArtifact = "Instances on the new shape; recorded data preserved; green per shard.";
        :>> owner = Owner::ai;
    }
    part mig5Reconcile : ProcessStep {
        :>> id = "67940516-2c7d-4e8f-a067-516273849606";
        :>> title = "Reconcile post-apply";
        :>> order = 6;
        :>> actionText = "Re-run the count/invariant check. Old-shape usage count must reach 0 (excluding documented grandfathered/exempt cases) before contract. Validators green.";
        :>> producedArtifact = "Verified old-shape usage == 0 (or explicitly grandfathered).";
        :>> owner = Owner::ai;
    }
    part mig6Contract : ProcessStep {
        :>> id = "67940516-2c7d-4e8f-a067-516273849607";
        :>> title = "CONTRACT + guard";
        :>> order = 7;
        :>> triggerCondition = "Old-shape usage count == 0.";
        :>> actionText = "Remove the old field/shape from schema + (any residual) instances. Never hoist a constraint a legitimate subtype cannot meet (LSP/ISP). Add a permanent guard (validator/lint) preventing reintroduction of the old shape (D0047).";
        :>> producedArtifact = "Old shape removed; a guard prevents its return; green.";
        :>> owner = Owner::ai;
    }
}
