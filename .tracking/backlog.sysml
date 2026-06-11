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
        // (removed `first schemaAudit then testModel` 2026-06-10: testModel — verification
        //  model + grammar — was parallel to schemaAudit, not downstream of it; the edge
        //  made testModel falsely SUSPECT when schemaAudit landed later.)
        first testModel then instanceAware;
        first whatsNextV2 then instanceAware;
        first schemaAudit then instanceMigration;
        first instanceAware then queryTools;

        // ---- acceptance (DoD) per task: method + recorded result (verifiedAtCommit) ----
        // DONE (verified at 2f762a6):
        requirement claudeMdDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "interaction discipline + triage accepted"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "user"; }
        satisfy claudeMdDoD by claudeMd;
        requirement processSpecDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "process-model spec reviewed + accepted"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "user"; }
        satisfy processSpecDoD by processSpec;
        requirement schemaRewriteDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "validate_schema 12/12 green"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "validate_schema"; }
        satisfy schemaRewriteDoD by schemaRewrite;
        requirement workflowsDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "validate_workflows 7/7 green"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "validate_workflows"; }
        satisfy workflowsDoD by workflows;
        requirement validatorFixDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "validator returns cleanly; zero orphaned JVMs"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "demo"; }
        satisfy validatorFixDoD by validatorFix;
        requirement autoPushDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "a commit triggers an automatic push"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "demo"; }
        satisfy autoPushDoD by autoPush;
        requirement whatsNextV2DoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "resolver lays out all workflows incl. parallelism; clean exit"; :>> verifiedAtCommit = "2f762a6"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "whats_next"; }
        satisfy whatsNextV2DoD by whatsNextV2;

        // OUTSTANDING (no verifiedAtCommit yet):
        requirement testModelDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "verification model decided + native grammar validated"; :>> verifiedAtCommit = "0f5c7d1"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "user"; }
        satisfy testModelDoD by testModel;
        requirement instanceAwareDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "query emits OUTSTANDING + SUSPECT from .tracking work-items"; }
        satisfy instanceAwareDoD by instanceAware;
        requirement schemaAuditDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "schema types aligned with natives"; :>> verifiedAtCommit = "72bb02b"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "user"; }
        satisfy schemaAuditDoD by schemaAudit;
        requirement instanceMigrationDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "processes/decisions/skills-registry parse green under new packages"; }
        satisfy instanceMigrationDoD by instanceMigration;
        requirement queryToolsDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "general query layer answers item/downstream/trace/suspect/outstanding"; }
        satisfy queryToolsDoD by queryTools;

        // ---- schemaAudit critique findings (2026-06-10): native SysML v2 leverage ----
        // All depend on schemaAudit (the native-metaclass migration). strongTyping is
        // foundational; views expose traces; checkable reqs build on typed constraints.
        action strongTyping;     // enum def + typed Actor refs + value types (no more String-everywhere)
        action nativeTrace;      // native derive/refine/trace/satisfy/verify relationships (B)
        action nativeViews;      // view def / viewpoint / expose for the materialized-view layer (A)
        action checkableReqs;    // requirements carry require/assume evaluable constraints (C)
        action analysisCalc;     // computed derivations as native calc/analysis def
        action nativeAlloc;      // architecture allocate + interface def / connection
        action temporalModel;    // decide: native occurrence/snapshot/timeslice vs git-only ancestry
        action sec26Materialize; // add a sanctioned materialized-view sentence to CLAUDE.md §2.6
        action userCapture;      // capture acting user (git piggyback; fallback .env/prompt) -> actor registry

        first schemaAudit then strongTyping;
        first strongTyping then userCapture;
        first schemaAudit then nativeTrace;
        first schemaAudit then analysisCalc;
        first schemaAudit then nativeAlloc;
        first schemaAudit then temporalModel;
        first strongTyping then checkableReqs;
        first strongTyping then nativeViews;
        first nativeTrace then nativeViews;

        requirement strongTypingDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "enum def closed-sets + value types (Uuid/Timestamp) adopted across schema/core; validates 12/12 green (typed Actor ref deferred to userCapture)"; :>> verifiedAtCommit = "c4d2f75"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "validate_schema"; }
        satisfy strongTypingDoD by strongTyping;
        requirement nativeTraceDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "edge strategy set + documented in relationships.sysml: satisfy (native stmt) + verify (native verification objective) confirmed parsing; derive/refine/trace are v1-only -> :>/deferred; allocate/dependsOn/supersede stay markers; schema validates 12/12 green"; :>> verifiedAtCommit = "c4ea4c7"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "validate_schema"; }
        satisfy nativeTraceDoD by nativeTrace;
        requirement nativeViewsDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "at least one engine view authored as a native view def with expose; query renders/computes it"; }
        satisfy nativeViewsDoD by nativeViews;
        requirement checkableReqsDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "concrete requirements may carry native require/assume constraints (parses, _spike_trace.py); documented in requirements.sysml; evaluation is solver-dependent (external SysIDE), not the kernel; schema 12/12 green"; :>> verifiedAtCommit = "f0044f4"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "validate_schema"; }
        satisfy checkableReqsDoD by checkableReqs;
        requirement analysisCalcDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "at least one computed derivation (e.g. coverage) expressed as a native calc/analysis def"; }
        satisfy analysisCalcDoD by analysisCalc;
        requirement nativeAllocDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "architecture function->component via native allocate + interface def parses green"; }
        satisfy nativeAllocDoD by nativeAlloc;
        requirement temporalModelDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "decided: adopt native occurrence/snapshot/timeslice or stay git-only for ancestry"; }
        satisfy temporalModelDoD by temporalModel;
        requirement sec26MaterializeDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "CLAUDE.md §2.6 gains a sanctioned marked-materialized-view clause"; :>> verifiedAtCommit = "062b97f"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "user"; }
        satisfy sec26MaterializeDoD by sec26Materialize;
        requirement userCaptureDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "capture_user.py captures the acting user from git EFFECTIVE identity (git var GIT_COMMITTER_IDENT — config is often unset; fallback .env/prompt) + idempotently registers in .tracking/actors.sysml ActorId enum; demonstrated (wweatherholtz); parses via validate_tracking. Wiring authoredBy/verifiedBy to ActorId deferred to instanceMigration"; :>> verifiedAtCommit = "3e40ff7"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "demo"; }
        satisfy userCaptureDoD by userCapture;
    }
}
