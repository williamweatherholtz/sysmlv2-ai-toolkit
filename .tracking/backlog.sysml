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
        requirement instanceAwareDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "query DISCOVERS work-item backlogs from .tracking/*.sysml (action defs) and emits OUTSTANDING + SUSPECT; no hardcoded target; --target focuses one; item + override verified"; :>> verifiedAtCommit = "adc35ab"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "demo"; }
        satisfy instanceAwareDoD by instanceAware;
        requirement schemaAuditDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "schema types aligned with natives"; :>> verifiedAtCommit = "72bb02b"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "user"; }
        satisfy schemaAuditDoD by schemaAudit;
        requirement instanceMigrationDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "all 14 instance files (10 decisions + 3 processes + skills-registry) migrated to flat Engine<Concern> packages; parse green via validate_instances (14/14); Gate/GateCheck reconceived as Process/ProcessStep; legacy validate_sysml.py retired"; :>> verifiedAtCommit = "e39a49e"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "validate_instances"; }
        satisfy instanceMigrationDoD by instanceMigration;
        requirement queryToolsDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "general query layer answers item/downstream/trace/suspect/outstanding (+ whats-next); verified on whatsNextV2/schemaAudit"; :>> verifiedAtCommit = "35f5d54"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "demo"; }
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
        requirement nativeViewsDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "engine views declared as native view def anchors (Outstanding/Suspect/Trace) + viewpoint in computed.sysml; expose/render NOT in pilot grammar so bodies stay computed by query.py (deferred to newer toolchain); 13/13 green"; :>> verifiedAtCommit = "71ad96d"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "demo"; }
        satisfy nativeViewsDoD by nativeViews;
        requirement checkableReqsDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "concrete requirements may carry native require/assume constraints (parses, _spike_trace.py); documented in requirements.sysml; evaluation is solver-dependent (external SysIDE), not the kernel; schema 12/12 green"; :>> verifiedAtCommit = "f0044f4"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "validate_schema"; }
        satisfy checkableReqsDoD by checkableReqs;
        requirement analysisCalcDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "derivations expressed as native calc/analysis defs (RequirementCoverage calc w/ expression + SuspicionSweep analysis) in computed.sysml; 13/13 green"; :>> verifiedAtCommit = "71ad96d"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "validate_schema"; }
        satisfy analysisCalcDoD by analysisCalc;
        requirement nativeAllocDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "native allocate X to Y + allocation def + connect + interface def(>=2 ends) confirmed parsing; allocation def anchor in EngineArchitecture; ICD/matrix stay computed; 13/13 green"; :>> verifiedAtCommit = "71ad96d"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "validate_schema"; }
        satisfy nativeAllocDoD by nativeAlloc;
        requirement temporalModelDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "DECIDED stay git-only (merge-base ancestry) for the engine timeline; native occurrence/snapshot/timeslice noted as available for deliverable/subject-version modeling, not the engine clock"; :>> verifiedAtCommit = "0e8b24c"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "user"; }
        satisfy temporalModelDoD by temporalModel;
        // ---- 2026-06-11 architectural critique: 13 CRs, ALL ACCEPTED BY USER ----
        // (docs/design-history/2026-06-11-architectural-critique.md §4; suggested order = chain below)
        action crWritePath;        // CR-1  validate_tracking preloads schema; recursive globs; .tracking template
        action crKillState;        // CR-7  delete currentState/updatedAt; fix agile "set state" steps
        action crTruthDocs;        // CR-9  docs vs git reality (.tracking policy, banner, usage.md, RESUME, lying comments)
        action crBootstrapHole;    // CR-13 semantics-changing tooling = CHANGE (§3e)
        action crDecisions;        // CR-8  backfill Decisions 0011+; CR acceptance artifacts henceforth
        action crTypeUniverse;     // CR-2  schema/core canonical; retire _meta shadow item defs
        action crVerification;     // CR-3  appended TestResults replace mutable DoD verdicts; method enum
        action crSuspicion;        // CR-4  D0005-honest suspicion + evidence validation
        action crIdentity;         // CR-5  UUIDs on instances; qualified keying; Person actors
        action crCursor;           // CR-6  .tracking state cursor + query orient
        action crProcessHonesty;   // CR-10 registry real skills; agile rebased; evaluability labels
        action crValidateAll;      // CR-11 validate_all; pre-commit; JVM hygiene
        action crNativeWins;       // CR-12 doc clauses; typed refs; finish enums; metadata spike; STPA refs; marker examples

        first crWritePath then crKillState;
        first crKillState then crTruthDocs;
        first crTruthDocs then crBootstrapHole;
        first crBootstrapHole then crDecisions;
        first crDecisions then crTypeUniverse;
        first crTypeUniverse then crVerification;
        first crVerification then crSuspicion;
        first crSuspicion then crIdentity;
        first crIdentity then crCursor;
        first crCursor then crProcessHonesty;
        first crProcessHonesty then crValidateAll;
        first crValidateAll then crNativeWins;

        requirement crWritePathDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "a schema/core-typed instance in .tracking passes validate_tracking; subdirs scanned; template exists"; :>> verifiedAtCommit = "1037d2d"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "validate_tracking"; }
        satisfy crWritePathDoD by crWritePath;
        requirement crKillStateDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "currentState/updatedAt gone from Element + Tracked bases; agile steps no longer author state; all green"; }
        satisfy crKillStateDoD by crKillState;
        requirement crTruthDocsDoD : AcceptanceCriterion { :>> method = "inspect"; :>> statement = "docs match git reality; .tracking policy decided + recorded; usage.md valid; RESUME thinned; lying comments gone"; }
        satisfy crTruthDocsDoD by crTruthDocs;
        requirement crBootstrapHoleDoD : AcceptanceCriterion { :>> method = "inspect"; :>> statement = "CLAUDE.md §3e routes computed-view semantics changes as CHANGE"; }
        satisfy crBootstrapHoleDoD by crBootstrapHole;
        requirement crDecisionsDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "Decisions 0011+ backfilled and parse green; CR-acceptance discipline recorded in CLAUDE.md §4"; }
        satisfy crDecisionsDoD by crDecisions;
        requirement crTypeUniverseDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "_meta shadow item defs retired; workflows typed by schema/core; workflows + tracking validate green"; }
        satisfy crTypeUniverseDoD by crTypeUniverse;
        requirement crVerificationDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "DoD verdicts are appended TestResult records (immutable); method uses VerificationMethod enum; query computes done from results; all green"; }
        satisfy crVerificationDoD by crVerification;
        requirement crSuspicionDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "suspicion = material change (git) over satisfy/:> + #OrderingOnly honored + transitive; verifiedAtCommit SHAs validated (INVALID-EVIDENCE bucket); ordering edge restored"; }
        satisfy crSuspicionDoD by crSuspicion;
        requirement crIdentityDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "all instances carry :>> id UUIDs; tooling keys qualified; actor registry = Person parts; validators warn on missing id"; }
        satisfy crIdentityDoD by crIdentity;
        requirement crCursorDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = ".tracking/state.sysml cursor exists; query orient merges cursor + ready + suspect"; }
        satisfy crCursorDoD by crCursor;
        requirement crProcessHonestyDoD : AcceptanceCriterion { :>> method = "inspect"; :>> statement = "registry lists the 5 real skills (phantoms gone); agile process matches main-only reality; DoR/DoD steps carry evaluability labels"; }
        satisfy crProcessHonestyDoD by crProcessHonesty;
        requirement crValidateAllDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "validate_all runs all layers on one kernel; pre-commit validates staged layers; stale-kernel sweep exists; no orphan JVM after interrupt"; }
        satisfy crValidateAllDoD by crValidateAll;
        requirement crNativeWinsDoD : AcceptanceCriterion { :>> method = "test"; :>> statement = "doc clauses on semantics-bearing elements; typed refs replace name-strings; remaining enums authored + strongTyping DoD statement superseded; metadata-def spike verdict; STPA control-loop refs; one worked example per marker; all green"; }
        satisfy crNativeWinsDoD by crNativeWins;

        requirement sec26MaterializeDoD : AcceptanceCriterion { :>> method = "confirmation"; :>> statement = "CLAUDE.md §2.6 gains a sanctioned marked-materialized-view clause"; :>> verifiedAtCommit = "062b97f"; :>> verifiedAt = "2026-06-10"; :>> verifiedBy = "user"; }
        satisfy sec26MaterializeDoD by sec26Materialize;
        requirement userCaptureDoD : AcceptanceCriterion { :>> method = "demonstration"; :>> statement = "capture_user.py captures the acting user from git EFFECTIVE identity (git var GIT_COMMITTER_IDENT — config is often unset; fallback .env/prompt) + idempotently registers in .tracking/actors.sysml ActorId enum; demonstrated (wweatherholtz); parses via validate_tracking. Wiring authoredBy/verifiedBy to ActorId deferred to instanceMigration"; :>> verifiedAtCommit = "3e40ff7"; :>> verifiedAt = "2026-06-11"; :>> verifiedBy = "demo"; }
        satisfy userCaptureDoD by userCapture;
    }
}
