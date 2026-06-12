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
        verification claudeMdDoD : Test { :>> id = "05bcb12e-1d2a-4cda-bfe8-06762c27d7c0"; :>> method = VerificationMethod::confirmation; :>> procedureText = "interaction discipline + triage accepted"; }
        part claudeMdR1 : TestResult { :>> id = "b5d37e1b-4a60-43d4-a0df-6cbaedcbc93f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        verification processSpecDoD : Test { :>> id = "f1847f4f-ca6f-45a5-af44-3179f327503a"; :>> method = VerificationMethod::confirmation; :>> procedureText = "process-model spec reviewed + accepted"; }
        part processSpecR1 : TestResult { :>> id = "67a24ab4-c724-48c1-80ca-2917724a9844"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        part processSpecR2 : TestResult { :>> id = "ad89c4e1-a307-4be7-ba10-841ad54392e3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        verification schemaRewriteDoD : Test { :>> id = "fbe432d5-a06b-4040-829b-b34a8fa4e249"; :>> method = VerificationMethod::test; :>> procedureText = "validate_schema 12/12 green"; }
        part schemaRewriteR1 : TestResult { :>> id = "74453aef-9251-4a4c-a428-8166b213a66f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part schemaRewriteR2 : TestResult { :>> id = "73d8d665-b91e-4f51-ac01-4f358c8705fc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification workflowsDoD : Test { :>> id = "bc582068-cf14-4cfc-a760-128b46fb9142"; :>> method = VerificationMethod::test; :>> procedureText = "validate_workflows 7/7 green"; }
        part workflowsR1 : TestResult { :>> id = "ada27b24-9192-4074-8a36-3350eccbd596"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_workflows"; }
        part workflowsR2 : TestResult { :>> id = "d8928c85-e00b-44ae-a88c-86a6aa32a165"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_workflows"; }
        verification validatorFixDoD : Test { :>> id = "d4f32a8c-98c0-43b7-8509-a891f18d0622"; :>> method = VerificationMethod::demo; :>> procedureText = "validator returns cleanly; zero orphaned JVMs"; }
        part validatorFixR1 : TestResult { :>> id = "efad84f0-4118-46ff-9618-75dd13230641"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "demo"; }
        verification autoPushDoD : Test { :>> id = "91807c1d-809c-4391-bac5-8d94e9c65a9d"; :>> method = VerificationMethod::demo; :>> procedureText = "a commit triggers an automatic push"; }
        part autoPushR1 : TestResult { :>> id = "ce9b6cec-dbca-4886-8469-a0fdde41bbb3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "demo"; }
        verification whatsNextV2DoD : Test { :>> id = "a979f11a-1c5a-407a-a0e2-fe2cc9e4fe8e"; :>> method = VerificationMethod::test; :>> procedureText = "resolver lays out all workflows incl. parallelism; clean exit"; }
        part whatsNextV2R1 : TestResult { :>> id = "5e0c9fff-f9df-48cd-be57-cfbb55106010"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f762a6"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "whats_next"; }
        part whatsNextV2R2 : TestResult { :>> id = "5462250c-ec8a-4e62-a6c7-60efb9ab4c23"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "whats_next"; }

        verification testModelDoD : Test { :>> id = "6f958fe9-6654-4e9f-b133-61f426941ec7"; :>> method = VerificationMethod::confirmation; :>> procedureText = "verification model decided + native grammar validated"; }
        part testModelR1 : TestResult { :>> id = "ceabbeff-5030-40ba-83f2-073d00fb5339"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0f5c7d1"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        part testModelR2 : TestResult { :>> id = "3ccf4138-df0f-4614-8686-29617ca0c1f8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        verification instanceAwareDoD : Test { :>> id = "2fe9a46e-ac18-4ad0-97ef-648f981bb011"; :>> method = VerificationMethod::test; :>> procedureText = "query DISCOVERS work-item backlogs from .tracking/*.sysml (action defs) and emits OUTSTANDING + SUSPECT; no hardcoded target; --target focuses one; item + override verified"; }
        part instanceAwareR1 : TestResult { :>> id = "a2dc3fbc-edf2-4b15-b373-1a59cd307e4c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "adc35ab"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part instanceAwareR2 : TestResult { :>> id = "3878c7f0-8f75-478c-895a-09d04b7e94d8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        verification schemaAuditDoD : Test { :>> id = "d9789ded-30e3-4562-8e91-dafe7ee2418c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "schema types aligned with natives"; }
        part schemaAuditR1 : TestResult { :>> id = "043dfc68-b309-4694-96be-c07bc50b8e25"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "72bb02b"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        part schemaAuditR2 : TestResult { :>> id = "25ffac6e-9c74-4b8d-a466-744a2d8e4fdc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        verification instanceMigrationDoD : Test { :>> id = "8dbe21dd-34ab-4194-8ab4-632d3332b13a"; :>> method = VerificationMethod::test; :>> procedureText = "all 14 instance files (10 decisions + 3 processes + skills-registry) migrated to flat Engine<Concern> packages; parse green via validate_instances (14/14); Gate/GateCheck reconceived as Process/ProcessStep; legacy validate_sysml.py retired"; }
        part instanceMigrationR1 : TestResult { :>> id = "5d946440-0d31-452e-b187-2ce18b837a0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e39a49e"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        part instanceMigrationR2 : TestResult { :>> id = "57120ab4-7f6f-43b9-a065-58412d1b55c7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification queryToolsDoD : Test { :>> id = "c32a8e54-e768-49b9-97b1-9896c3c49526"; :>> method = VerificationMethod::demo; :>> procedureText = "general query layer answers item/downstream/trace/suspect/outstanding (+ whats-next); verified on whatsNextV2/schemaAudit"; }
        part queryToolsR1 : TestResult { :>> id = "1c4a228c-4e12-4ef1-afe3-0a4598d60abf"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "35f5d54"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part queryToolsR2 : TestResult { :>> id = "e9922863-01ee-4b7c-a952-820d348a471d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }

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

        verification strongTypingDoD : Test { :>> id = "c6c995e7-f59d-4905-aea8-0de7bb1aa01d"; :>> method = VerificationMethod::test; :>> procedureText = "enum def closed-sets + value types (Uuid/Timestamp) adopted across schema/core; validates 12/12 green (typed Actor ref deferred to userCapture)"; }
        part strongTypingR1 : TestResult { :>> id = "8f88b479-02a0-4a23-99c7-ed437bbea5b9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c4d2f75"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part strongTypingR2 : TestResult { :>> id = "6bf73364-4322-410c-9c44-27cea4db4f9d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification nativeTraceDoD : Test { :>> id = "be31ddee-9404-443f-b2a6-156a799a3ccd"; :>> method = VerificationMethod::test; :>> procedureText = "edge strategy set + documented in relationships.sysml: satisfy (native stmt) + verify (native verification objective) confirmed parsing; derive/refine/trace are v1-only -> :>/deferred; allocate/dependsOn/supersede stay markers; schema validates 12/12 green"; }
        part nativeTraceR1 : TestResult { :>> id = "0e8cb659-0a2c-4c30-9d66-a8ccf567d4c2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c4ea4c7"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part nativeTraceR2 : TestResult { :>> id = "064b2863-2f6a-43f6-a1a4-d36be90eab19"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification nativeViewsDoD : Test { :>> id = "ef5d25c2-cb69-4d01-8fab-0adf3f42cce2"; :>> method = VerificationMethod::demo; :>> procedureText = "engine views declared as native view def anchors (Outstanding/Suspect/Trace) + viewpoint in computed.sysml; expose/render NOT in pilot grammar so bodies stay computed by query.py (deferred to newer toolchain); 13/13 green"; }
        part nativeViewsR1 : TestResult { :>> id = "435d78fd-f31e-480d-a74c-cf3434e37ef4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71ad96d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part nativeViewsR2 : TestResult { :>> id = "f4dc5da9-4fbe-4c05-b9ba-c30e2e59a1dc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        verification checkableReqsDoD : Test { :>> id = "ca1f5ca8-14fa-41d4-a3c1-8441cf6a0f08"; :>> method = VerificationMethod::test; :>> procedureText = "concrete requirements may carry native require/assume constraints (parses, _spike_trace.py); documented in requirements.sysml; evaluation is solver-dependent (external SysIDE), not the kernel; schema 12/12 green"; }
        part checkableReqsR1 : TestResult { :>> id = "f8215086-a2df-4fc2-b9ff-0f6106e288bc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f0044f4"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "validate_schema"; }
        part checkableReqsR2 : TestResult { :>> id = "1c47c046-1ad2-46cc-b7e7-be1c3fa1935d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification analysisCalcDoD : Test { :>> id = "9ee093f2-9347-408f-a779-1c23589f530f"; :>> method = VerificationMethod::demo; :>> procedureText = "derivations expressed as native calc/analysis defs (RequirementCoverage calc w/ expression + SuspicionSweep analysis) in computed.sysml; 13/13 green"; }
        part analysisCalcR1 : TestResult { :>> id = "9fd5a686-e8eb-41ad-ab93-7896b4f70f24"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71ad96d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        part analysisCalcR2 : TestResult { :>> id = "d172765c-1cce-4fe1-8e40-0dbaa4da14b8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        verification nativeAllocDoD : Test { :>> id = "e69e0039-a7da-4308-85a3-629d004c4b25"; :>> method = VerificationMethod::test; :>> procedureText = "native allocate X to Y + allocation def + connect + interface def(>=2 ends) confirmed parsing; allocation def anchor in EngineArchitecture; ICD/matrix stay computed; 13/13 green"; }
        part nativeAllocR1 : TestResult { :>> id = "c5a32d4b-8828-4459-ac38-fe981ac4f624"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71ad96d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        part nativeAllocR2 : TestResult { :>> id = "1aa1eb62-3b89-4564-bc58-5c1e77cd2f8b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification temporalModelDoD : Test { :>> id = "aa2e1808-792a-4b06-a544-4c91f24c25ee"; :>> method = VerificationMethod::confirmation; :>> procedureText = "DECIDED stay git-only (merge-base ancestry) for the engine timeline; native occurrence/snapshot/timeslice noted as available for deliverable/subject-version modeling, not the engine clock"; }
        part temporalModelR1 : TestResult { :>> id = "5d710f54-f27e-4d1b-9143-fcb6c248e2ad"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0e8b24c"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
        part temporalModelR2 : TestResult { :>> id = "623fddac-d65a-4525-9227-ea26a3711705"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9837c17"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "user"; }
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

        verification crWritePathDoD : Test { :>> id = "a8030800-ace9-4bbc-860e-a6dc89d4d6d0"; :>> method = VerificationMethod::test; :>> procedureText = "a schema/core-typed instance in .tracking passes validate_tracking; subdirs scanned; template exists"; }
        part crWritePathR1 : TestResult { :>> id = "2865943a-8782-407e-86cd-4ab5c3245459"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1037d2d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_tracking"; }
        verification crKillStateDoD : Test { :>> id = "c6d15c83-1fce-4fcf-be7f-d3872eea9514"; :>> method = VerificationMethod::test; :>> procedureText = "currentState/updatedAt gone from Element + Tracked bases; agile steps no longer author state; all green"; }
        part crKillStateR1 : TestResult { :>> id = "8a61de7b-c4f7-455f-bd77-4518f8d10d5c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4ef8679"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_schema"; }
        verification crTruthDocsDoD : Test { :>> id = "faa089be-ff02-471b-a2f6-017150ebb853"; :>> method = VerificationMethod::inspect; :>> procedureText = "docs match git reality; .tracking policy decided + recorded; usage.md valid; RESUME thinned; lying comments gone; EngineDocSync process added + tied into DoD and CLAUDE.md §4"; }
        part crTruthDocsR1 : TestResult { :>> id = "80032c4f-65f3-49f2-a169-0c3515c8d721"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "99e6ed7"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification crBootstrapHoleDoD : Test { :>> id = "7a96d253-8d2d-4f48-95ac-636fa25a8d89"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLAUDE.md §3e routes computed-view semantics changes as CHANGE"; }
        part crBootstrapHoleR1 : TestResult { :>> id = "830226a2-0fb8-4cf7-8380-ddee07668c3d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0a3aee2"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "inspect"; }
        verification crDecisionsDoD : Test { :>> id = "5e87ae46-69f3-4a87-8c23-2a7015313c91"; :>> method = VerificationMethod::test; :>> procedureText = "Decisions 0011-0017 backfilled and parse green (24/24); CR-acceptance discipline recorded in CLAUDE.md §4"; }
        part crDecisionsR1 : TestResult { :>> id = "2e7b0ad8-8821-4f49-8fa5-2d4cbe7cbfe0"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a092fc2"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification crTypeUniverseDoD : Test { :>> id = "c1603e83-b2b6-490a-b879-490941f19bd9"; :>> method = VerificationMethod::test; :>> procedureText = "_meta shadow item defs retired; workflows typed by schema/core; workflows + tracking validate green"; }
        part crTypeUniverseR1 : TestResult { :>> id = "f48c1225-cd53-4fb6-8f90-6bf7c241831e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f965c0"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_workflows"; }
        verification crVerificationDoD : Test { :>> id = "1e4afc6d-88c0-4b87-bbcf-899f4ba5996a"; :>> method = VerificationMethod::test; :>> procedureText = "DoD verdicts are appended TestResult records (immutable); method uses VerificationMethod enum; query computes done from results; all green"; }
        part crVerificationR1 : TestResult { :>> id = "0aa3a7eb-789d-4a55-81c2-0d4239419e70"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "df27aa2"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_tracking"; }
        verification crSuspicionDoD : Test { :>> id = "9d2f42e6-995a-4670-a2d6-f125ebcd5282"; :>> method = VerificationMethod::test; :>> procedureText = "suspicion = material change (git) over satisfy/:> + #OrderingOnly honored + transitive; verifiedAtCommit SHAs validated (INVALID-EVIDENCE bucket); ordering edge restored"; }
        part crSuspicionR1 : TestResult { :>> id = "3c377112-96e8-4df6-9408-512a6801a781"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "_test_suspect"; }
        verification crIdentityDoD : Test { :>> id = "072cbb2a-8b34-4b8c-871c-c32857c8c433"; :>> method = VerificationMethod::test; :>> procedureText = "all instances carry :>> id UUIDs; tooling keys qualified; actor registry = Person parts; validators warn on missing id"; }
        part crIdentityR1 : TestResult { :>> id = "6d261f2d-895d-428d-9630-d24b1e8a162d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bc4b6ca"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification crCursorDoD : Test { :>> id = "6eea640e-6f23-484b-80b7-f847b7df3954"; :>> method = VerificationMethod::demo; :>> procedureText = ".tracking/state.sysml cursor exists; query orient merges cursor + ready + suspect"; }
        part crCursorR1 : TestResult { :>> id = "e0c715f9-d583-43c4-b1f7-b29306d16f41"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd47bae"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        verification crProcessHonestyDoD : Test { :>> id = "492294a7-4f39-46ae-a3eb-630a329d47d4"; :>> method = VerificationMethod::inspect; :>> procedureText = "registry lists the 5 real skills (phantoms gone); agile process matches main-only reality; DoR/DoD steps carry evaluability labels"; }
        part crProcessHonestyR1 : TestResult { :>> id = "08ee8c66-a5cb-4ec2-aa52-973cbd136086"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0a509b1"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_instances"; }
        verification crValidateAllDoD : Test { :>> id = "3e8e94f9-8437-48a3-a054-3fc1559c661b"; :>> method = VerificationMethod::test; :>> procedureText = "validate_all runs all layers on one kernel; pre-commit validates staged layers; stale-kernel sweep exists; no orphan JVM after interrupt"; }
        part crValidateAllR1 : TestResult { :>> id = "1d64783a-5f52-4434-b771-49c9fdd8a710"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4a9332d"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_all"; }
        verification crNativeWinsDoD : Test { :>> id = "96654b2b-9a04-4bd9-9304-b305ad89d430"; :>> method = VerificationMethod::test; :>> procedureText = "doc clauses on semantics-bearing elements; typed refs replace name-strings; remaining enums authored + strongTyping DoD statement superseded; metadata-def spike verdict; STPA control-loop refs; one worked example per marker; all green"; }
        part crNativeWinsR1 : TestResult { :>> id = "c778d275-9071-4185-a887-4b787aecc9f5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "165a83f"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "validate_all"; }

        verification sec26MaterializeDoD : Test { :>> id = "77a68ad8-aa36-44c0-b652-9c91a70a81db"; :>> method = VerificationMethod::confirmation; :>> procedureText = "CLAUDE.md §2.6 gains a sanctioned marked-materialized-view clause"; }
        part sec26MaterializeR1 : TestResult { :>> id = "cbb2d6ae-0683-413c-a33d-103662bfcaeb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "062b97f"; :>> judgedAt = "2026-06-10"; :>> judgedBy = "user"; }
        verification userCaptureDoD : Test { :>> id = "e98cf42d-d21e-4eae-b297-d999da56047c"; :>> method = VerificationMethod::demo; :>> procedureText = "capture_user.py captures the acting user from git EFFECTIVE identity (git var GIT_COMMITTER_IDENT — config is often unset; fallback .env/prompt) + idempotently registers in .tracking/actors.sysml ActorId enum; demonstrated (wweatherholtz); parses via validate_tracking. Wiring authoredBy/verifiedBy to ActorId deferred to instanceMigration"; }
        part userCaptureR1 : TestResult { :>> id = "78f73f4e-84c5-4eca-9439-f3adc4e58fbb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e40ff7"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
        part userCaptureR2 : TestResult { :>> id = "3a019ce5-42fd-4599-82a7-c3b19ca892b5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "61850f4"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "demo"; }
    }
// =================================================================
    // NextWork — the post-bootstrap queue (2026-06-11 handoff). EngineBuild
    // (above) is COMPLETE 34/34; this is what comes next. Sources: the first
    // real dogfood (CLAUDE.md §4), deferred critique findings (M3/M6/U2/U5-U7,
    // UC3/UC9/UC10), parked pre-critique findings, and the runtime roadmap.
    // Resume: `python .engine/tools/query.py orient` surfaces the ready set.
    // =================================================================
    action def NextWork {
        // ---- the headline: first real dogfood (in order) ----
        action dogfoodBusiness;     // move cursor to BusinessWorkflow::Business/brief; author a REAL Brief->Personas->Needs->UseCases in .tracking (use the template idioms) + the phase-gate verification instances (critique S6)
        action dogfoodArchitecture; // derive SystemRequirements with native satisfy from the Needs; native allocate to Components
        action dogfoodDelivery;     // run one Story/task through DoR -> implement -> appended TestResults -> DoD

        // ---- query/tooling gaps from the use-case walkthroughs ----
        action traceNeeds;          // UC3: satisfy-edge text reader + `trace-need <name>` subcommand in query.py
        action issueLoop;           // UC10: .tracking/issues.sysml convention + `query.py issues` listing
        action baselineView;        // UC9: Release = commit SHA + explicit member refs; baseline view computed from it

        // ---- deferred native-leverage (spiked or design-ready) ----
        action trackedMetadata;     // U2: @Tracked attributed-metadata refactor (SPIKED GREEN) replacing the 4 Tracked* bases
        action stateDefSpike;       // M3: spike native state def/transition; if green replace WorkflowDefinition.transitions strings
        action occurrenceRetype;    // M6: spike occurrence def :> Element; retype TestResult (+Decision?) if green
        action authoredByRefs;      // M4 remainder: authoredBy/judgedBy as typed refs to ProjectActors parts (deferred at CR-5)

        // ---- parked design questions (pre-critique + critique) ----
        action safetyCaseViews;     // parked: curated-view + frozen Baseline/Signoff design for the safety case
        action needLayer;           // parked: does Need require a layer discriminator? decide + record
        action crVersioning;        // parked: CR self-modification versioning (meta-process unfreeze design)
        action skillsRefresh;       // rename agile-refinement->backlog-refinement; build test-design/test-result/traceability-audit skills; register

        // ---- the deliverable runtime (after dogfood proves the model) ----
        action runtimeParser;       // text parser/indexer (Rust candidate) replacing kernel+regex readers; opens SysON interop
        action writeApi;            // the write API: enforces writePolicy, ids, append-only results (D0003 becomes real)
        action toolchainWatch;      // adopt expose/render, verify-by, derive/refine/trace when a newer SysML v2 toolchain supports them

        action foldWhatsNext;       // UC6 leftover: merge whats_next.py's workflow-DAG resolution into query.py (a `workflows` subcommand); retire whats_next.py — one tool, one answer
        action contractAlign;       // computed-state contract vs query.py: implement coverageState/satisfaction over verify edges or amend the contract; remove its stale currentState reference
        action specRefresh;         // rewrite/retire the historical design spec so no doc contradicts decisions (was RESUME 'known debt' prose)
        action modelOnlyState;      // D0018: RESUME.md deleted; orient computed; CLAUDE.md bans prose state docs

        // ---- Rust parser CLI sprint backlog (2026-06-12): rustS0..rustS4 feed runtimeParser ----
        action rustS0Workspace;   // Sprint 0: Cargo workspace + quality gates + BDD harness
        action rustS1Lexer;       // Sprint 1: tokenizer for engine dialect; proptest-covered
        action rustS2Parser;      // Sprint 2: recursive descent parser; full thiserror; no .unwrap()
        action rustS3Semantic;    // Sprint 3: package registry + cross-package resolution + type checking
        action rustS4SpecCompat;  // Sprint 4: build.rs spec pin + compile-time grammar hash assertion

        verification rustS0WorkspaceDoR : Test { :>> id = "c6788072-6045-438f-a95b-879ce9147ebd"; :>> method = VerificationMethod::inspect; :>> procedureText = "Cargo workspace layout agreed (sysmlv2-parser lib + sysmlv2-cli bin); deny attributes decided; BDD framework (cucumber-rs) selected; CI script drafted; no blocking dependencies"; }
        verification rustS0WorkspaceDoD : Test { :>> id = "4092ffcd-70a7-4fc5-98f6-aebacb68ecff"; :>> method = VerificationMethod::test; :>> procedureText = "Cargo.toml workspace at repo root; sysmlv2-parser and sysmlv2-cli crates exist; both compile with #![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]; cargo test passes with zero warnings; pre-commit hook runs cargo clippy -D warnings; validate_all.py still green"; }
        verification rustS0RetroGate : Test { :>> id = "baecae84-8740-494d-b091-982bffed0b14"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Sprint 0 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete"; }
        verification rustS1LexerDoR : Test { :>> id = "554ef94a-5eba-4e38-8d39-6a503c20960b"; :>> method = VerificationMethod::inspect; :>> procedureText = "Token type list complete; at least three failing proptest strategies written (identifier, string_lit, enum_literal); lexer API type signature agreed; DoD accepted"; }
        verification rustS1LexerDoD : Test { :>> id = "113eaa88-c18b-4ed3-8cf8-11eeb4e94cee"; :>> method = VerificationMethod::test; :>> procedureText = "Lexer tokenizes all token types in the engine dialect (package, import, action/part/verification/requirement/use-case/attribute/enum def, :>>, ::, first/then, string, UUID, enum literal); proptest: minimum 50 cases/strategy; zero panics; no .unwrap() in src/; cargo test + cargo clippy -D warnings green"; }
        verification rustS1RetroGate : Test { :>> id = "4b5a378d-60f6-495c-96c1-964f99aaa6f5"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Sprint 1 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete"; }
        verification rustS2ParserDoR : Test { :>> id = "13b90e4a-4c91-4368-9383-cecb5834efe4"; :>> method = VerificationMethod::inspect; :>> procedureText = "BDD feature file has passing scenario skeletons for package, part def, action def, succession, satisfy, allocate; AST node types sketched; error recovery strategy (synchronize-on-semicolon) accepted; DoD accepted"; }
        verification rustS2ParserDoD : Test { :>> id = "5b62f1fd-2b49-44ac-87a3-e122a4c012c3"; :>> method = VerificationMethod::test; :>> procedureText = "Recursive-descent parser produces typed AST for full engine dialect (package, import, part/action/verification/requirement/use-case def, attribute, enum, block, :>> assignments, first/then, satisfy, allocate, #DependsOn); all existing .tracking/ and .engine/ files parse without error; BDD scenarios green; no .unwrap() in src/; cargo test + cargo clippy -D warnings green"; }
        verification rustS2RetroGate : Test { :>> id = "b746580e-0978-468e-b101-f88aa41f5a12"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Sprint 2 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete"; }
        verification rustS3SemanticDoR : Test { :>> id = "80047e3a-6b18-4d89-84ad-7be64038b968"; :>> method = VerificationMethod::inspect; :>> procedureText = "Package registry API agreed; cross-package import resolution algorithm sketched (topological load order); at least two failing BDD scenarios for unresolved-reference errors; DoD accepted"; }
        verification rustS3SemanticDoD : Test { :>> id = "ca479d99-727d-4367-837d-50bcde3781da"; :>> method = VerificationMethod::test; :>> procedureText = "PackageRegistry resolves `private import X::*` across all preloaded schema + tracking files; typed references (`: Story`, `: TestResult`) validated against registry; enum literals (e.g. `WorkKind::code`) validated; diagnostic messages include file + line + suggested fix; all .tracking/ files validate green; proptest covers registry merge; cargo test + cargo clippy -D warnings green"; }
        verification rustS3RetroGate : Test { :>> id = "51cbca04-1509-46ce-8fab-0cabb90ac7f5"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Sprint 3 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete"; }
        verification rustS4SpecCompatDoR : Test { :>> id = "171b72f1-f5b5-4fc1-bc12-701bed84c53c"; :>> method = VerificationMethod::inspect; :>> procedureText = "Canonical SysML v2 spec URL confirmed (https://github.com/Systems-Modeling/SysML-v2-Release); grammar manifest format understood; SYSML_V2_GRAMMAR_SHA constant to pin agreed; offline-mode env var name agreed; DoD accepted"; }
        verification rustS4SpecCompatDoD : Test { :>> id = "7ce88f0c-6f1d-40bb-a3c8-8c045daba7af"; :>> method = VerificationMethod::test; :>> procedureText = "build.rs defines SYSML_V2_SPEC_URL and SYSML_V2_GRAMMAR_SHA constants; fetches grammar manifest on build and verifies SHA256; build fails with actionable message if hash mismatch; SYSML_V2_SPEC_OFFLINE=1 skips remote check; version string emitted as cargo:rustc-env=SYSML_V2_GRAMMAR_VERSION; BDD scenario covers offline and mismatch cases; cargo test + cargo clippy -D warnings green"; }
        verification rustS4RetroGate : Test { :>> id = "0e7c8f1c-6aeb-49e4-af15-50a937a8f68a"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Sprint 4 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete"; }

        first dogfoodBusiness then dogfoodArchitecture;
        first dogfoodArchitecture then dogfoodDelivery;
        first traceNeeds then baselineView;
        first dogfoodDelivery then runtimeParser;
        first runtimeParser then writeApi;
        first dogfoodDelivery then rustS0Workspace;
        first rustS0Workspace then rustS1Lexer;
        first rustS1Lexer then rustS2Parser;
        first rustS2Parser then rustS3Semantic;
        first rustS3Semantic then rustS4SpecCompat;
        first rustS4SpecCompat then runtimeParser;

        verification foldWhatsNextDoD : Test { :>> id = "4338695c-eb93-4d73-b904-c9c4c1469840"; :>> method = VerificationMethod::test; :>> procedureText = "query.py workflows subcommand resolves the six workflow DAGs (parity with whats_next.py); whats_next.py retired; single query tool"; }
        part foldWhatsNextR1 : TestResult { :>> id = "6d7e8f9a-0b1c-4d2e-3f4a-5b6c7d8e9f0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5282ad3"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "claudeOpus"; }
        verification contractAlignDoD : Test { :>> id = "cda3ab15-159b-4e2d-850c-405237a85dc9"; :>> method = VerificationMethod::inspect; :>> procedureText = "computed-state contract and query.py agree clause-by-clause: each view implemented, amended, or explicitly queued; no stale references"; }
        verification specRefreshDoD : Test { :>> id = "3d2823ec-9cd9-4449-bc35-3db8a5f9773d"; :>> method = VerificationMethod::inspect; :>> procedureText = "the 2026-06-04 spec is rewritten to match decisions 0001-0018 or formally retired to design-history; no doc contradicts the model"; }
        verification modelOnlyStateDoD : Test { :>> id = "2dc52776-7c18-425f-9413-57ac62aabeec"; :>> method = VerificationMethod::inspect; :>> procedureText = "RESUME.md deleted; CLAUDE.md sec4 bans prose state docs + sec1 routes orientation to query.py orient; no file shadow-tracks the backlog"; }
        part modelOnlyStateR1 : TestResult { :>> id = "2c4dfc07-6c91-4bba-869f-829578959ad6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f1f65d9"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "inspect"; }
        verification dogfoodBusinessDoD : Test { :>> id = "dec73f98-f883-4f0c-b50a-ff8dd24e0e27"; :>> method = VerificationMethod::confirmation; :>> procedureText = "a real Brief/Personas/Needs/UseCases authored in .tracking, phase gates as verification instances, cursor moved, all green; user accepts the Business run"; }
        part dogfoodBusinessR1 : TestResult { :>> id = "a0b1c2d3-e4f5-4a6b-7c8d-9e0f1a2b3c4d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f70f0e"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }
        verification dogfoodArchitectureDoD : Test { :>> id = "1ddedbcd-ced1-4537-ace3-ae82759ee1bf"; :>> method = VerificationMethod::confirmation; :>> procedureText = "SystemRequirements satisfy-linked to Needs, allocated to Components; trace queryable; user accepts"; }
        part dogfoodArchitectureR1 : TestResult { :>> id = "f5a6b7c8-d9e0-4f1a-2b3c-4d5e6f7a8b9c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abb033a"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }
        verification dogfoodDeliveryDoD : Test { :>> id = "50c631b3-5396-4f3c-84f0-054da815afd3"; :>> method = VerificationMethod::confirmation; :>> procedureText = "one item through DoR->implement->appended results->DoD with gates computed; user accepts"; }
        part dogfoodDeliveryR1 : TestResult { :>> id = "8f9a0b1c-2d3e-4f4a-5b6c-7d8e9f0a1b2c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5282ad3"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }
        verification traceNeedsDoD : Test { :>> id = "9465131e-1510-4d5d-9827-72f311601e0c"; :>> method = VerificationMethod::test; :>> procedureText = "query.py trace-need answers upstream/downstream over satisfy edges read from text; demonstrated on dogfood data"; }
        part traceNeedsR1 : TestResult { :>> id = "9a0b1c2d-3e4f-4a5b-6c7d-8e9f0a1b2c3d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fcef256"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "claudeOpus"; }
        verification issueLoopDoD : Test { :>> id = "1456d1db-63be-4cad-9c0b-4bc8ea2dee97"; :>> method = VerificationMethod::demo; :>> procedureText = "an Issue authored per convention surfaces in `query.py issues` and routes to a backlog task"; }
        verification baselineViewDoD : Test { :>> id = "25e39c3c-632e-4d51-af2e-1b1604539dbb"; :>> method = VerificationMethod::demo; :>> procedureText = "a Release (commit + member refs) renders a computed baseline view"; }
        verification trackedMetadataDoD : Test { :>> id = "b9e59c15-75e2-475f-94fc-9f73e77646dc"; :>> method = VerificationMethod::test; :>> procedureText = "@Tracked metadata replaces the four Tracked* bases; instances migrated; validate_all green"; }
        verification stateDefSpikeDoD : Test { :>> id = "37c1b11f-e0f4-4955-bdd1-b1e15651aad5"; :>> method = VerificationMethod::test; :>> procedureText = "state def/transition spike verdict recorded; transitions strings replaced if green"; }
        verification occurrenceRetypeDoD : Test { :>> id = "4ef8cff4-11db-4a48-9e20-58447aafe676"; :>> method = VerificationMethod::test; :>> procedureText = "occurrence def spike verdict recorded; TestResult retyped if green; validate_all green"; }
        verification authoredByRefsDoD : Test { :>> id = "40a97fd6-50bb-4831-b505-60da3050ab97"; :>> method = VerificationMethod::test; :>> procedureText = "authoredBy/judgedBy reference ProjectActors parts (typed or validated-name); tooling reads them"; }
        verification safetyCaseViewsDoD : Test { :>> id = "a316b216-d051-4f5f-acec-11f7ec092981"; :>> method = VerificationMethod::confirmation; :>> procedureText = "curated-view + frozen Baseline/Signoff design decided + recorded as a Decision"; }
        verification needLayerDoD : Test { :>> id = "c2c81ff3-059e-4831-a609-602d7312809e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Need layer-discriminator question decided + recorded as a Decision"; }
        verification crVersioningDoD : Test { :>> id = "a0db11b0-fd52-4166-9436-db03ad1d9e42"; :>> method = VerificationMethod::confirmation; :>> procedureText = "CR self-modification versioning decided + recorded as a Decision"; }
        verification skillsRefreshDoD : Test { :>> id = "e5a7417c-bf60-46e0-895f-38aee849b01b"; :>> method = VerificationMethod::inspect; :>> procedureText = "agile-refinement renamed backlog-refinement; test-design/test-result/traceability-audit built + registered"; }
        verification runtimeParserDoD : Test { :>> id = "7c78b3cd-b51e-4a26-9b84-bd5e7040b605"; :>> method = VerificationMethod::test; :>> procedureText = "standalone parser/indexer reads schema+instances without the pilot kernel; query layer runs on it; parity with current outputs"; }
        verification writeApiDoD : Test { :>> id = "864722ea-23bb-469b-9813-a8840d0d526d"; :>> method = VerificationMethod::test; :>> procedureText = "write API enforces writePolicy/ids/append-only; direct text editing retired as the write path"; }
        verification toolchainWatchDoD : Test { :>> id = "e08f7e87-bd2b-4702-be9f-b39a76dbb028"; :>> method = VerificationMethod::confirmation; :>> procedureText = "newer toolchain evaluated; expose/verify-by/derive adoption decided + recorded"; }
    }
}
