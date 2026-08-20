// =====================================================================
// D0067 — migration-process: a research-grounded, gated process for safe bulk
// migrations of authored model data + schema, deployed by the `migration` skill (D0059).
// =====================================================================
package Decision0067 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // D0070: this Decision created the migration process (prospective; D0062 default). Grandfathered for resolution (pre-guard).
    #ProspectiveChange part d0067 : Decision {
        :>> id = "63940516-2c7d-4e8f-a067-516273849571";
        :>> title = "D0067: safe bulk-migration process (expand/migrate/contract + edge-case checklist) as a process + deploying skill";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Three bulk migrations this session (populate_created_by, contract_attr_fields, migrate_acceptance_events) hit a recurring edge-case class: a line-regex silently missed multi-line concatenated decisionText (~17%), caught only by a dry-run COUNT mismatch (64 vs 53); tightening createdBy to [1..1] would have broken every TestResult (which uses judgedBy); ~38 decisions lacked recorded who/when. D0040 says recurring work must be codified into a deploying-skill-backed process, not left to vigilance. Research converges (Fowler ParallelChange; Ambler 'Refactoring Databases'; Google LSC shard-transform-guard; event-sourcing upcast/never-mutate-history; Flyway/Liquibase versioned-idempotent-reversible; ETL control-total reconciliation; data-provenance integrity).";
        :>> decision = "Adopt a GATED safe-migration process, deployed by a `migration` skill: (0) justify+classify -> recorded Decision (CHANGE, human-accepted); (1) transform = a committed, reviewed script (codemod) -- never hand-edits at scale; prefer AST/structural matching over line-regex; (2) DRY-RUN + RECONCILE -- control totals balance (new==old; migrated+skipped+errored==total) + validators green on projected output, FAIL on mismatch; (3) EXPAND -- add the new shape additively, old+new both valid, commit green; (4) MIGRATE -- idempotent (skip-if-already-new), sharded, each commit green, copy-and-transform recorded facts, backfill-before-tighten; (5) RECONCILE post-apply -- old-shape usage count == 0; (6) CONTRACT -- drop the old shape only at usage==0, + a permanent guard against reintroduction (D0047). Historical-data rule: grandfather (cutoff) OR backfill-with-recorded-basis OR explicit human attestation -- NEVER fabricate provenance (D0016/D0051). Constraint rule: backfill-before-tighten, and never hoist a constraint a legitimate subtype cannot meet (LSP/ISP -- the TestResult/createdBy case).";
        :>> rationale = "The session's edge cases + the prior art show ad-hoc migration is undefensible: our 17% miss was caught by a count check we happened to run, not by discipline. A gated process makes the catch structural (reconcile is a gate, not luck) and encodes the hard-won rules (AST-over-regex, backfill-before-tighten, don't-hoist-unsatisfiable, grandfather-not-fabricate). ALTERNATIVES: (A) keep per-migration vigilance -- rejected: leaks process knowledge to conversation history (D0040), and relies on luck; (B CHOSEN) codified gated process + deploying skill, research-grounded. CRITERIA: defensibility x repeatability x catches-the-known-failure-modes (regex-miss, unsafe-tighten, fabricated-provenance).";
        :>> consequences = "Adds .engine/processes/migration.sysml (Process + 7 ProcessSteps) + .engine/skills/migration/SKILL.md (the gates + edge-case checklist + historical-data rule) + registry entry; CLAUDE.md + the engine-triage skill route any bulk migration through it. The expand/migrate/contract spine matches what the attribution migration (D0063) already did; this makes it the mandatory, gate-checked default. Reconciliation + AST-preference would have caught the decisionText miss by discipline, not luck. Pairs with D0047 (the contract step's anti-reintroduction guard) and D0048 (kernel-free reconciliation tooling).";
    }

    verification d0067Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a067-516273849572";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of decision d0067 (safe-migration process: full process + deploying skill + Decision), via the explicit AskUserQuestion answer on 2026-06-17.";
    }
    part d0067AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a067-516273849573";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "70f9484";
        :>> judgedAt = "2026-06-17";
        :>> judgedBy = "wweatherholtz";
    }
}
