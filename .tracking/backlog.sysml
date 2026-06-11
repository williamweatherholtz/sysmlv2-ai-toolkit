// =====================================================================
// EngineBacklog — the engine's own construction plan, as tracked SysML data
// (the self-build dogfood). Transitioned from RESUME's prose backlog.
//
// A project plan IS a behavior, so it's an `action def`: tasks are `action`s,
// dependencies are `succession` (first/then). Each task's DoD is a method-tagged
// `verification <task>DoD : Test` (one line). Results are APPENDED, immutable
// `part <task>R<n> : TestResult` records (outcome + judgedAgainst commit) —
// re-verification appends R2, never edits R1 (Decision 0012; CR-3 2026-06-11).
// "Done" = the LATEST result is a pass. The query tool computes done/outstanding
// and SUSPECT via git-ancestry of judgedAgainst commits.
// =====================================================================
package EngineBacklog {
    private import EngineElement::*;        // enums (VerificationMethod, VerdictKind)
    private import EngineVerification::*;   // Test (criterion) + TestResult (appended record)
    private import EngineRelationships::*;  // #OrderingOnly marker

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
        // Restored 2026-06-11 (CR-4): this TRUE ordering fact was deleted on 2026-06-10 to
        // silence a false SUSPECT — workaround-by-deletion anti-pattern. Tagged #OrderingOnly:
        // it gates ordering but does NOT carry suspicion (D0005).
        #OrderingOnly first schemaAudit then testModel;
        first testModel then instanceAware;
        first whatsNextV2 then instanceAware;
        first schemaAudit then instanceMigration;
        first instanceAware then queryTools;

        // ---- acceptance (DoD) per task: method + recorded result (verifiedAtCommit) ----
        // (done/outstanding is COMPUTED by query.py - never narrated here)
        verification claudeMdDoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "interaction discipline + triage accepted"; }
        part claudeMdR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        verification processSpecDoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "process-model spec reviewed + accepted"; }
        part processSpecR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        part processSpecR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        verification schemaRewriteDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "validate_schema 12/12 green"; }
        part schemaRewriteR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part schemaRewriteR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification workflowsDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "validate_workflows 7/7 green"; }
        part workflowsR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_workflows"; }
        part workflowsR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_workflows"; }
        verification validatorFixDoD : Test { :>> method = VerificationMethod::demo; :>> procedureText = "validator returns cleanly; zero orphaned JVMs"; }
        part validatorFixR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "demo"; }
        verification autoPushDoD : Test { :>> method = VerificationMethod::demo; :>> procedureText = "a commit triggers an automatic push"; }
        part autoPushR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "demo"; }
        verification whatsNextV2DoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "resolver lays out all workflows incl. parallelism; clean exit"; }
        part whatsNextV2R1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "whats_next"; }
        part whatsNextV2R2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "whats_next"; }

        verification testModelDoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "verification model decided + native grammar validated"; }
        part testModelR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0f5c7d1"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        part testModelR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        verification instanceAwareDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "query DISCOVERS work-item backlogs from .tracking/*.sysml (action defs) and emits OUTSTANDING + SUSPECT; no hardcoded target; --target focuses one; item + override verified"; }
        part instanceAwareR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "adc35ab"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part instanceAwareR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        verification schemaAuditDoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "schema types aligned with natives"; }
        part schemaAuditR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "72bb02b"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        part schemaAuditR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        verification instanceMigrationDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "all 14 instance files (10 decisions + 3 processes + skills-registry) migrated to flat Engine<Concern> packages; parse green via validate_instances (14/14); Gate/GateCheck reconceived as Process/ProcessStep; legacy validate_sysml.py retired"; }
        part instanceMigrationR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e39a49e"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        part instanceMigrationR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification queryToolsDoD : Test { :>> method = VerificationMethod::demo; :>> procedureText = "general query layer answers item/downstream/trace/suspect/outstanding (+ whats-next); verified on whatsNextV2/schemaAudit"; }
        part queryToolsR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "35f5d54"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part queryToolsR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }

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

        verification strongTypingDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "enum def closed-sets + value types (Uuid/Timestamp) adopted across schema/core; validates 12/12 green (typed Actor ref deferred to userCapture)"; }
        part strongTypingR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c4d2f75"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part strongTypingR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification nativeTraceDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "edge strategy set + documented in relationships.sysml: satisfy (native stmt) + verify (native verification objective) confirmed parsing; derive/refine/trace are v1-only -> :>/deferred; allocate/dependsOn/supersede stay markers; schema validates 12/12 green"; }
        part nativeTraceR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c4ea4c7"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part nativeTraceR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification nativeViewsDoD : Test { :>> method = VerificationMethod::demo; :>> procedureText = "engine views declared as native view def anchors (Outstanding/Suspect/Trace) + viewpoint in computed.sysml; expose/render NOT in pilot grammar so bodies stay computed by query.py (deferred to newer toolchain); 13/13 green"; }
        part nativeViewsR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71ad96d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part nativeViewsR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        verification checkableReqsDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "concrete requirements may carry native require/assume constraints (parses, _spike_trace.py); documented in requirements.sysml; evaluation is solver-dependent (external SysIDE), not the kernel; schema 12/12 green"; }
        part checkableReqsR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f0044f4"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part checkableReqsR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification analysisCalcDoD : Test { :>> method = VerificationMethod::demo; :>> procedureText = "derivations expressed as native calc/analysis defs (RequirementCoverage calc w/ expression + SuspicionSweep analysis) in computed.sysml; 13/13 green"; }
        part analysisCalcR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71ad96d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        part analysisCalcR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        verification nativeAllocDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "native allocate X to Y + allocation def + connect + interface def(>=2 ends) confirmed parsing; allocation def anchor in EngineArchitecture; ICD/matrix stay computed; 13/13 green"; }
        part nativeAllocR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71ad96d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        part nativeAllocR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification temporalModelDoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "DECIDED stay git-only (merge-base ancestry) for the engine timeline; native occurrence/snapshot/timeslice noted as available for deliverable/subject-version modeling, not the engine clock"; }
        part temporalModelR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0e8b24c"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        part temporalModelR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
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

        verification crWritePathDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "a schema/core-typed instance in .tracking passes validate_tracking; subdirs scanned; template exists"; }
        part crWritePathR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1037d2d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_tracking"; }
        verification crKillStateDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "currentState/updatedAt gone from Element + Tracked bases; agile steps no longer author state; all green"; }
        part crKillStateR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4ef8679"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification crTruthDocsDoD : Test { :>> method = VerificationMethod::inspect; :>> procedureText = "docs match git reality; .tracking policy decided + recorded; usage.md valid; RESUME thinned; lying comments gone; EngineDocSync process added + tied into DoD and CLAUDE.md §4"; }
        part crTruthDocsR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "99e6ed7"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification crBootstrapHoleDoD : Test { :>> method = VerificationMethod::inspect; :>> procedureText = "CLAUDE.md §3e routes computed-view semantics changes as CHANGE"; }
        part crBootstrapHoleR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0a3aee2"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "inspect"; }
        verification crDecisionsDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "Decisions 0011-0017 backfilled and parse green (24/24); CR-acceptance discipline recorded in CLAUDE.md §4"; }
        part crDecisionsR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a092fc2"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification crTypeUniverseDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "_meta shadow item defs retired; workflows typed by schema/core; workflows + tracking validate green"; }
        part crTypeUniverseR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f965c0"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_workflows"; }
        verification crVerificationDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "DoD verdicts are appended TestResult records (immutable); method uses VerificationMethod enum; query computes done from results; all green"; }
        part crVerificationR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "df27aa2"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_tracking"; }
        verification crSuspicionDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "suspicion = material change (git) over satisfy/:> + #OrderingOnly honored + transitive; verifiedAtCommit SHAs validated (INVALID-EVIDENCE bucket); ordering edge restored"; }
        part crSuspicionR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "_test_suspect"; }
        verification crIdentityDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "all instances carry :>> id UUIDs; tooling keys qualified; actor registry = Person parts; validators warn on missing id"; }
        verification crCursorDoD : Test { :>> method = VerificationMethod::demo; :>> procedureText = ".tracking/state.sysml cursor exists; query orient merges cursor + ready + suspect"; }
        verification crProcessHonestyDoD : Test { :>> method = VerificationMethod::inspect; :>> procedureText = "registry lists the 5 real skills (phantoms gone); agile process matches main-only reality; DoR/DoD steps carry evaluability labels"; }
        verification crValidateAllDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "validate_all runs all layers on one kernel; pre-commit validates staged layers; stale-kernel sweep exists; no orphan JVM after interrupt"; }
        verification crNativeWinsDoD : Test { :>> method = VerificationMethod::test; :>> procedureText = "doc clauses on semantics-bearing elements; typed refs replace name-strings; remaining enums authored + strongTyping DoD statement superseded; metadata-def spike verdict; STPA control-loop refs; one worked example per marker; all green"; }

        verification sec26MaterializeDoD : Test { :>> method = VerificationMethod::confirmation; :>> procedureText = "CLAUDE.md §2.6 gains a sanctioned marked-materialized-view clause"; }
        part sec26MaterializeR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "062b97f"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        verification userCaptureDoD : Test { :>> method = VerificationMethod::demo; :>> procedureText = "capture_user.py captures the acting user from git EFFECTIVE identity (git var GIT_COMMITTER_IDENT — config is often unset; fallback .env/prompt) + idempotently registers in .tracking/actors.sysml ActorId enum; demonstrated (wweatherholtz); parses via validate_tracking. Wiring authoredBy/verifiedBy to ActorId deferred to instanceMigration"; }
        part userCaptureR1 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e40ff7"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part userCaptureR2 : TestResult { :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
    }
}
