// =====================================================================
// ProjectArchitecture — the Architecture-workflow artifacts for the sysmlv2-ai-toolkit self-build.
// First real dogfood of the Architecture workflow (dogfoodArchitecture, 2026-06-11).
//
// Three parallel phases (ArchitectureWorkflow::Architecture):
//   dataArch   → SystemRequirements (satisfy-linked to Needs)
//   appArch    → Components (the engine's software components)
//   techArch   → SubsystemRequirements (technical refinements)
//   allocIface → allocate edges (SR → Component) + phase gates
//
// File sorts after business/business.sysml (r > b) so ProjectBusiness is in
// kernel scope and Needs can be referenced in satisfy edges.
// =====================================================================
package ProjectArchitecture {
    private import EngineElement::*;
    private import EngineRequirements::*;
    private import EngineArchitecture::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;
    private import ProjectBusiness::*;

    // ====================================================================
    // Phase: dataArch — SystemRequirements (one per Need)
    // ====================================================================

    requirement sr01TypedItems : SystemRequirement {
        :>> id = "3a4b5c6d-7e8f-4091-a2b3-c4d5e6f70819";
        :>> title = "Typed item vocabulary with immutable identity";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall provide a schema/core type vocabulary such that every tracked element has an immutable id (UUID), a metaclass, and typed attribute fields.";
        :>> rationale = "Typed identity enables collision-free querying and suspicion propagation without manual name management.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement sr02DecisionRecord : SystemRequirement {
        :>> id = "4b5c6d7e-8f90-4112-b3c4-d5e6f7081920";
        :>> title = "First-class decision authoring with supersede edges";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall allow authoring of Decision items (including scope cuts) with rationale and typed #Supersede edges to the Needs they cancel.";
        :>> rationale = "Scope cuts that go unrecorded create invisible technical debt and confusion about what was deliberately excluded.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::inspect;
    }

    requirement sr03AppendResults : SystemRequirement {
        :>> id = "5c6d7e8f-9001-4223-c4d5-e6f708192031";
        :>> title = "Append-only immutable verification result records";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall record each verification execution as a distinct TestResult part with outcome, judgedAgainst commit SHA, and judgedBy; existing results shall never be mutated.";
        :>> rationale = "Mutable verdicts destroy the evidence trail; append-only records let git ancestry compute currency.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement sr04ComputedViews : SystemRequirement {
        :>> id = "6d7e8f90-0112-4334-d5e6-f70819203142";
        :>> title = "Project state always derived from authored facts";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall compute orientation, coverage, and suspicion from authored .sysml facts and git ancestry; no status, coverage, or suspicion fields shall be authored.";
        :>> rationale = "Authored state drifts; computed state is always consistent with the facts that produced it.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement sr05GitNative : SystemRequirement {
        :>> id = "7e8f9001-1223-4445-e6f7-081920314253";
        :>> title = "All authored facts in git-committed .sysml text";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall store all authored items and edges as .sysml text files committed to a git repository; git commit SHAs shall be the causal clock for suspicion computation.";
        :>> rationale = "Git provides a free, auditable, distributed history; using SHAs as the causal clock avoids wall-clock ordering ambiguity.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::inspect;
    }

    requirement sr06ProcessModel : SystemRequirement {
        :>> id = "8f900112-2334-4556-f708-192031425364";
        :>> title = "Workflow and phase structure authored as .sysml in .engine/";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall represent workflow definitions, phase action sequences, gate criteria, and skill registries as .sysml files in .engine/, readable and executable by agents without in-context embedding.";
        :>> rationale = "Process-as-data allows the engine to be updated without changing agent prompts; agents read the model rather than carrying the process in memory.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::inspect;
    }

    requirement sr07AgentLoop : SystemRequirement {
        :>> id = "90011223-3445-4667-a081-920314253647";
        :>> title = "CLAUDE.md + validator toolchain enables autonomous agent execution";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall provide a CLAUDE.md discipline guide and a validator toolchain (pre-commit hook + validate_all.py) that allows an AI agent to orient, execute a phase, validate green, record results, and commit without human intervention at each micro-step.";
        :>> rationale = "Human-in-the-loop at every micro-step defeats the purpose of AI-assisted engineering; the discipline should be in the process, not the approval chain.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::should;
        :>> verificationMethod = VerificationMethod::demo;
    }

    requirement sr08TraceQuery : SystemRequirement {
        :>> id = "a0122334-4556-4778-b192-031425364758";
        :>> title = "Satisfy-edge trace queryable end-to-end";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall provide a query subcommand that traces a given Need forward to its satisfying SystemRequirements and backward to the Brief, reading satisfy edges from .sysml text.";
        :>> rationale = "Traceability without a query tool is a document, not a model; the query layer is what makes the satisfy edges useful.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::should;
        :>> verificationMethod = VerificationMethod::demo;
    }

    requirement sr09SelfBuild : SystemRequirement {
        :>> id = "b0233445-5667-4889-c203-142536475869";
        :>> title = "Engine self-build constitutes the primary dogfood";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall track its own construction using its own process and schema, so that the .tracking/ history for this project constitutes the primary evidence of process correctness.";
        :>> rationale = "Dogfooding surfaces gaps that abstract design misses; a self-hosted engine is its own acceptance test.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::could;
        :>> verificationMethod = VerificationMethod::confirmation;
    }

    // ---- Rust parser CLI initiative (2026-06-12): SystemRequirements ----

    requirement sr10RustCli : SystemRequirement {
        :>> id = "751c232e-3ccc-4692-8580-8bb44f1ddb45";
        :>> title = "Rust CLI replaces JVM kernel for all parse/validate/query operations";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The system shall provide a Rust CLI binary (sysmlv2) that performs all operations currently requiring the pilot kernel — parse, validate, orient, workflows, trace-need — without JVM, conda, or Jupyter.";
        :>> rationale = "JVM cold start (~20s) is unacceptable for CI and tight feedback loops; a Rust binary delivers sub-second startup and removes the conda environment dependency.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement sr11FastStart : SystemRequirement {
        :>> id = "8012de84-7218-4499-8593-d1dc0efe3692";
        :>> title = "Cold-start parse and orient complete in under 500ms";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The sysmlv2 CLI shall parse the full engine repo (schema + tracking) and emit orient JSON in under 500ms on a developer laptop with no warm cache.";
        :>> rationale = "500ms is the human perception threshold for 'instant'; exceeding it forces developers to context-switch.";
        :>> kind = RequirementKind::nonFunctional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement sr12DenyWarnings : SystemRequirement {
        :>> id = "5b0feaba-235e-474d-884f-b457e6de6287";
        :>> title = "Production Rust code enforces deny(warnings) and zero-unwrap policy";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The sysmlv2-parser and sysmlv2-cli crates shall compile with #![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)]; no .unwrap(), .expect(), or index-without-bounds shall appear in src/ outside #[cfg(test)] blocks.";
        :>> rationale = "Unwrap panics are production bugs; clippy::pedantic surfaces logic issues before they manifest; deny(warnings) prevents warning accumulation.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement sr13SpecPin : SystemRequirement {
        :>> id = "75d66430-2d91-4d70-81bd-df374a3ae7ac";
        :>> title = "build.rs pins grammar to canonical SysML v2 spec URL";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The sysmlv2-parser crate's build.rs shall embed the canonical SysML v2 spec URL (https://github.com/Systems-Modeling/SysML-v2-Release) and the pinned grammar commit SHA as compile-time constants; the build shall fail if the fetched manifest hash diverges.";
        :>> rationale = "Without a spec pin, grammar drift is silent; a build-time check makes divergence visible and actionable.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement sr14TddBdd : SystemRequirement {
        :>> id = "859e584a-f1ba-4461-80ac-5975f088e2c3";
        :>> title = "BDD scenarios and property-based tests gate all parser behavior";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "All parser and semantic-layer behavior shall be covered by BDD scenarios authored with cucumber-rs before implementation; property-based tests with proptest shall cover all tokenizer and parser combinator cases; CI shall enforce cargo test and cargo clippy -D warnings green.";
        :>> rationale = "TDD/BDD drives correctness by specification rather than implementation; property tests find edge cases that example tests miss.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::should;
        :>> verificationMethod = VerificationMethod::test;
    }

    // ====================================================================
    // Phase: appArch — Components (the engine's software subsystems)
    // ====================================================================

    part schemaCoreComp : Component {
        :>> id = "c0344556-6778-499a-d314-253647586970";
        :>> title = "schema/core — the type vocabulary";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The .engine/schema/core/ package set. Defines all canonical item types (Element, Need, Requirement, TestResult, Decision, Component, …) and the edge vocabulary. Frozen by governance; changes go through CR.";
    }

    part workflowDefs : Component {
        :>> id = "d0455667-7889-4aab-e425-364758697081";
        :>> title = "workflow definitions — the process model";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The .engine/workflows/ action defs (Business, Architecture, Delivery, Deploy, Operate, ChangeRequest). Defines phase structure, item flows, and gate DoD criteria. Read by agents and validators.";
    }

    part validatorStack : Component {
        :>> id = "e0566778-8990-4bbc-f536-475869708192";
        :>> title = "validator toolchain — schema/process/instance/tracking validators";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The .engine/tools/validate/ layer (validate_schema, validate_workflows, validate_instances, validate_tracking, validate_all) plus the pre-commit hook. Enforces 0-ERROR parse green before every commit.";
    }

    part queryStack : Component {
        :>> id = "f0677889-9001-4ccd-0647-586970819203";
        :>> title = "query layer — orient/outstanding/suspect/trace";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The .engine/tools/query.py tool (and whats_next.py pending retirement). Computes orient, outstanding, suspect, and trace views from authored .sysml text and git ancestry. The single source of project state.";
    }

    part trackingData : Component {
        :>> id = "00788990-0112-4dde-1758-697081920314";
        :>> title = "tracking store — authored instance data";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The .tracking/ directory tree. Contains all per-project instance data: backlogs, actors, state cursor, Business/Architecture/Delivery artifacts. The truth store that the query layer reads.";
    }

    part processDefs : Component {
        :>> id = "018990a1-1223-4eef-2869-708192031425";
        :>> title = "process and skills — process steps + skill registry";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The .engine/processes/ (DoR, DoD, doc-sync, agile, critique) and .engine/skills/ (registry) definitions. Provides the procedural detail agents reference when executing phases.";
    }

    // ---- Rust parser CLI initiative (2026-06-12): Components ----

    part rustParserComp : Component {
        :>> id = "c75fd116-86d8-475c-9c3c-459a6a3e6235";
        :>> title = "sysmlv2-parser — Rust parser/indexer library crate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The sysmlv2-parser crate. Implements the lexer, recursive-descent parser, package registry, cross-package reference resolver, and type checker for the engine's SysML v2 dialect. Produces a typed AST and index structure. No .unwrap() in src/; full thiserror error hierarchy; proptest-covered.";
    }

    part rustCliComp : Component {
        :>> id = "1e1719d4-ca14-4892-9065-65228829d02d";
        :>> title = "sysmlv2-cli — Rust CLI binary crate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The sysmlv2-cli crate. Wraps sysmlv2-parser and implements the orient, workflows, trace-need, validate, and item subcommands. Produces JSON output parity with query.py. Replaces the JVM kernel calls in validate_*.py and query.py.";
    }

    // ====================================================================
    // Phase: techArch — SubsystemRequirements (technical refinements)
    // ====================================================================

    requirement ssr01ValidatorGreen : SubsystemRequirement {
        :>> id = "12a001b2-2334-4ff0-397a-819203142536";
        :>> title = "Validator accepts .sysml via pilot kernel with 0 ERROR lines";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The validator toolchain shall submit .sysml cells to the SysML v2 pilot kernel (jupyter-sysml-kernel) and report PASS iff the kernel emits no line containing 'error', 'unresolved', 'no viable', or synonyms.";
        :>> rationale = "The pilot kernel is the authoritative parser; PASS = 'it parses' is the tractable green-light criterion during bootstrap.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement ssr02QueryOrient : SubsystemRequirement {
        :>> id = "23b112c3-3445-400f-4a8b-920314253647";
        :>> title = "orient subcommand returns cursor + ready + suspect as JSON";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The query layer orient subcommand shall return a JSON object with cursor (activeWorkflow, activePhase, enteredAt, enteredBy), ready list, suspect list, invalidEvidence list, and done/outstanding counts.";
        :>> rationale = "A machine-readable orient output allows agents to parse state without screen-scraping prose.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement ssr03RecursiveTracking : SubsystemRequirement {
        :>> id = "34c223d4-4556-411a-5b9c-031425364758";
        :>> title = "Tracking validator scans .tracking/ recursively including subdirs";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "The validate_tracking tool shall discover and validate all .sysml files under .tracking/ recursively, including files in subdirectories (business/, architecture/, delivery/, …).";
        :>> rationale = "Flat-only scanning would force all tracking data into one directory; subdirectory organisation is required for readability at scale.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement ssr04CondaFullPath : SubsystemRequirement {
        :>> id = "45d334e5-5667-422b-6cad-142536475869";
        :>> title = "Toolchain invocation uses absolute miniforge3 conda path";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "All tool invocations that require the sysml conda environment shall use the absolute path to the miniforge3 conda executable, not rely on conda being on PATH.";
        :>> rationale = "Claude Code shell sessions do not run conda init; bare 'conda' is not found. Absolute path is resilient to shell environment differences.";
        :>> kind = RequirementKind::nonFunctional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::inspect;
    }

    // ---- Rust parser CLI initiative (2026-06-12): SubsystemRequirements ----

    requirement ssr05Workspace : SubsystemRequirement {
        :>> id = "751ebac0-f86e-42e4-a279-892b44e3402a";
        :>> title = "Cargo workspace at repo root with sysmlv2-parser and sysmlv2-cli crates";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "A Cargo.toml workspace shall exist at the repo root containing sysmlv2-parser (library) and sysmlv2-cli (binary) crates; the workspace shall enforce edition = '2021' and resolver = '2'.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement ssr06NoUnwrap : SubsystemRequirement {
        :>> id = "aa707cd5-3dc4-473b-b06c-bfe18da8434d";
        :>> title = "All error types use thiserror; no panic paths in sysmlv2-parser/src/";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "Every error type in sysmlv2-parser shall derive thiserror::Error; all Result return types shall use a typed error enum; no panic!, unwrap(), or expect() shall appear in src/ outside test modules; clippy::panics_disallowed shall be enabled.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement ssr07SpecBuild : SubsystemRequirement {
        :>> id = "b76ad1d7-9214-4517-bb08-6d8096f6f05b";
        :>> title = "build.rs fetches grammar manifest; build fails on hash mismatch";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "sysmlv2-parser/build.rs shall define SYSML_V2_SPEC_URL pointing to the SysML v2 Release repo grammar manifest; on build, it shall verify the SHA256 of the grammar files matches the pinned SYSML_V2_GRAMMAR_SHA constant; SYSML_V2_SPEC_OFFLINE=1 skips the check.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::must;
        :>> verificationMethod = VerificationMethod::test;
    }

    requirement ssr08TestCoverage : SubsystemRequirement {
        :>> id = "e970f35b-a7e7-4717-af73-343d0ba6de06";
        :>> title = "proptest strategies cover lexer and parser; cucumber BDD scenarios cover subcommands";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz";
        :>> statement = "sysmlv2-parser shall have proptest-based property tests for all token types and parser productions; sysmlv2-cli shall have cucumber-rs BDD feature files for orient, validate, trace-need, and workflows subcommands; minimum 50 proptest cases per strategy.";
        :>> kind = RequirementKind::functional;
        :>> priority = Priority::should;
        :>> verificationMethod = VerificationMethod::test;
    }

    // ====================================================================
    // Phase: allocIface — satisfy edges + allocate edges
    //
    // satisfy <need> by <systemRequirement>: SR satisfies the upstream Need.
    // allocate <sr> to <component>: SR allocated to its implementing component.
    // ====================================================================

    satisfy n01AtomicItems by sr01TypedItems;
    satisfy n02DecisionsFirstClass by sr02DecisionRecord;
    satisfy n03VerificationTraced by sr03AppendResults;
    satisfy n04ComputedState by sr04ComputedViews;
    satisfy n05GitNative by sr05GitNative;
    satisfy n06ProcessAsData by sr06ProcessModel;
    satisfy n07AgentAutonomy by sr07AgentLoop;
    satisfy n08TraceQueryable by sr08TraceQuery;
    satisfy n09SelfHosting by sr09SelfBuild;

    satisfy n10NoJvm by sr10RustCli;
    satisfy n11FastStart by sr11FastStart;
    satisfy n12PanicFree by sr12DenyWarnings;
    satisfy n13SpecPin by sr13SpecPin;
    satisfy n14TddBdd by sr14TddBdd;

    allocate sr01TypedItems to schemaCoreComp;
    allocate sr02DecisionRecord to trackingData;
    allocate sr03AppendResults to trackingData;
    allocate sr04ComputedViews to queryStack;
    allocate sr05GitNative to trackingData;
    allocate sr06ProcessModel to workflowDefs;
    allocate sr07AgentLoop to validatorStack;
    allocate sr08TraceQuery to queryStack;
    allocate sr09SelfBuild to trackingData;

    allocate ssr01ValidatorGreen to validatorStack;
    allocate ssr02QueryOrient to queryStack;
    allocate ssr03RecursiveTracking to validatorStack;
    allocate ssr04CondaFullPath to validatorStack;

    allocate sr10RustCli to rustCliComp;
    allocate sr11FastStart to rustParserComp;
    allocate sr12DenyWarnings to rustParserComp;
    allocate sr13SpecPin to rustParserComp;
    allocate sr14TddBdd to rustParserComp;
    allocate ssr05Workspace to rustCliComp;
    allocate ssr06NoUnwrap to rustParserComp;
    allocate ssr07SpecBuild to rustParserComp;
    allocate ssr08TestCoverage to rustParserComp;

    // DependsOn: SubsystemRequirements refine SystemRequirements
    #DependsOn dependency from ssr01ValidatorGreen to sr07AgentLoop;
    #DependsOn dependency from ssr02QueryOrient to sr04ComputedViews;
    #DependsOn dependency from ssr03RecursiveTracking to sr03AppendResults;
    #DependsOn dependency from ssr04CondaFullPath to sr07AgentLoop;

    #DependsOn dependency from ssr05Workspace to sr10RustCli;
    #DependsOn dependency from ssr06NoUnwrap to sr12DenyWarnings;
    #DependsOn dependency from ssr07SpecBuild to sr13SpecPin;
    #DependsOn dependency from ssr08TestCoverage to sr14TddBdd;

    // ====================================================================
    // Phase gates (S6 extension: Architecture phase gates)
    // TestResults appended when team lead confirms each phase.
    // ====================================================================
    verification dataArchGate : Test {
        :>> id = "56e445f6-6778-433c-7dbe-253647586970";
        :>> title = "dataArch phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Nine SystemRequirements authored (one per Need); each has statement, rationale, kind, priority, verificationMethod; satisfy edges link every SR to its Need; team lead accepts.";
    }
    part dataArchGateR1 : TestResult { :>> id = "0607182a-9b0c-41d2-3e4f-5a6b7c8d9e0f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abb033a"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }

    verification appArchGate : Test {
        :>> id = "67f55607-7889-444d-8ecf-364758697081";
        :>> title = "appArch phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "At least five Components authored with description; components represent the engine's real subsystems; team lead accepts.";
    }
    part appArchGateR1 : TestResult { :>> id = "1718293b-0c1d-42e3-4f5a-6b7c8d9e0f1a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abb033a"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }

    verification techArchGate : Test {
        :>> id = "78006718-8990-455e-9fd0-475869708192";
        :>> title = "techArch phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "At least four SubsystemRequirements authored; each refines a SystemRequirement via #DependsOn; team lead accepts.";
    }
    part techArchGateR1 : TestResult { :>> id = "2829304c-1d2e-43f4-5a6b-7c8d9e0f1a2b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abb033a"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }

    verification allocIfaceGate : Test {
        :>> id = "89117829-9001-466f-a0e1-586970819203";
        :>> title = "allocIface phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Every SystemRequirement has an allocate edge to a Component; satisfy edges from all nine Needs to SRs authored; team lead accepts.";
    }
    part allocIfaceGateR1 : TestResult { :>> id = "393a415d-2e3f-44a5-6b7c-8d9e0f1a2b3c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abb033a"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }
}
