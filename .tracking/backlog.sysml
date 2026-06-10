// =====================================================================
// EngineBacklog — the engine's own construction plan, as tracked SysML data
// (the self-build dogfood). Transitioned from RESUME's prose backlog.
//
// A project plan IS a behavior, so it's an `action def`: tasks are `action`s,
// dependencies are `succession` (first/then). Each task `satisfy`s a method-
// tagged `AcceptanceCriterion` (its DoD / "test"): test | analysis | inspection
// | demonstration | confirmation (confirmation = human sign-off). whats-next
// resolves the DAG; v3 will compute "done" from recorded pass results (+ commit
// SHA) and surface OUTSTANDING + SUSPECT.
// =====================================================================
package EngineBacklog {
    private import EngineArtifacts::*;

    action def EngineBuild {
        // ---- tasks (done this session + outstanding) ----
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

        // ---- acceptance criterion (DoD) per task, with verification method ----
        requirement claudeMdDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "interaction discipline + triage are clear and accepted"; }
        satisfy claudeMdDoD by claudeMd;
        requirement processSpecDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "process-model spec reviewed + accepted"; }
        satisfy processSpecDoD by processSpec;
        requirement schemaRewriteDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "validate_schema returns 12/12 green"; }
        satisfy schemaRewriteDoD by schemaRewrite;
        requirement workflowsDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "validate_workflows returns 7/7 green"; }
        satisfy workflowsDoD by workflows;
        requirement validatorFixDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "validator returns cleanly; zero orphaned JVMs"; }
        satisfy validatorFixDoD by validatorFix;
        requirement autoPushDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "a commit triggers an automatic push to origin"; }
        satisfy autoPushDoD by autoPush;
        requirement whatsNextV2DoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "resolver lays out all workflows incl. parallelism; clean exit"; }
        satisfy whatsNextV2DoD by whatsNextV2;
        requirement testModelDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "verification model decided + native grammar validated"; }
        satisfy testModelDoD by testModel;
        requirement instanceAwareDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "whats-next emits OUTSTANDING + SUSPECT from .tracking work-items"; }
        satisfy instanceAwareDoD by instanceAware;
        requirement schemaAuditDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "schema types aligned with natives (requirement/use-case/verification defs)"; }
        satisfy schemaAuditDoD by schemaAudit;
        requirement instanceMigrationDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "processes/decisions/skills-registry parse green under the new packages"; }
        satisfy instanceMigrationDoD by instanceMigration;
        requirement queryToolsDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "general query layer answers item/downstream/trace/suspect/outstanding"; }
        satisfy queryToolsDoD by queryTools;
    }
}
