// =====================================================================
// EngineBacklog — the engine's own construction plan, as tracked SysML data
// (the self-build dogfood). Transitioned from RESUME's prose backlog.
//
// A project plan IS a behavior, so it's an `action def`: tasks are `action`s,
// dependencies are `succession` (first/then). Each task `satisfy`s a method-
// tagged `AcceptanceCriterion` (its DoD). "Done" = the criterion has a
// `verifiedAtCommit` (a recorded pass result); absent = OUTSTANDING. The query
// tool computes done/outstanding, and SUSPECT via git-ancestry of the commits.
// =====================================================================
package EngineBacklog {
    private import EngineArtifacts::*;

    action def EngineBuild {
        // ---- tasks ----
        action claudeMd;
        action processSpec;
        action schemaRewrite;
        action workflows;
        action validatorFix;
        action autoPush;
        action whatsNextV2;
        action testModel;
        action instanceAware;
        action schemaAudit;
        action instanceMigration;
        action queryTools;

        // ---- dependencies (succession: earlier then later) ----
        first claudeMd then processSpec;
        first processSpec then schemaRewrite;
        first schemaRewrite then workflows;
        first workflows then whatsNextV2;
        first whatsNextV2 then schemaAudit;
        first whatsNextV2 then testModel;
        first schemaAudit then testModel;
        first testModel then instanceAware;
        first whatsNextV2 then instanceAware;
        first schemaAudit then instanceMigration;
        first instanceAware then queryTools;

        // ---- acceptance (DoD) per task: method + recorded result (verifiedAtCommit) ----
        // DONE (verified at 2f762a6):
        requirement claudeMdDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "interaction discipline + triage accepted"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedBy = "user"; }
        satisfy claudeMdDoD by claudeMd;
        requirement processSpecDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "process-model spec reviewed + accepted"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedBy = "user"; }
        satisfy processSpecDoD by processSpec;
        requirement schemaRewriteDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "validate_schema 12/12 green"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedBy = "validate_schema"; }
        satisfy schemaRewriteDoD by schemaRewrite;
        requirement workflowsDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "validate_workflows 7/7 green"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedBy = "validate_workflows"; }
        satisfy workflowsDoD by workflows;
        requirement validatorFixDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "validator returns cleanly; zero orphaned JVMs"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedBy = "demo"; }
        satisfy validatorFixDoD by validatorFix;
        requirement autoPushDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "a commit triggers an automatic push"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedBy = "demo"; }
        satisfy autoPushDoD by autoPush;
        requirement whatsNextV2DoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "resolver lays out all workflows incl. parallelism; clean exit"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedBy = "whats_next"; }
        satisfy whatsNextV2DoD by whatsNextV2;

        // OUTSTANDING (no verifiedAtCommit yet):
        requirement testModelDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "verification model decided + native grammar validated"; :>> verifiedAtCommit = "0f5c7d1"; :>> verifiedBy = "user"; }
        satisfy testModelDoD by testModel;
        requirement instanceAwareDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "query emits OUTSTANDING + SUSPECT from .tracking work-items"; }
        satisfy instanceAwareDoD by instanceAware;
        requirement schemaAuditDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "schema types aligned with natives"; }
        satisfy schemaAuditDoD by schemaAudit;
        requirement instanceMigrationDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "processes/decisions/skills-registry parse green under new packages"; }
        satisfy instanceMigrationDoD by instanceMigration;
        requirement queryToolsDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "general query layer answers item/downstream/trace/suspect/outstanding"; }
        satisfy queryToolsDoD by queryTools;
    }
}
