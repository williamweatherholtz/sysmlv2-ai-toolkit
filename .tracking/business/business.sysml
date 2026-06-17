// =====================================================================
// ProjectBusiness — the Business-workflow artifacts for the sysmlv2-ai-toolkit self-build.
// First real dogfood of the Business workflow (dogfoodBusiness, 2026-06-11).
// Phases: brief → personas → needs → useCases  (BusinessWorkflow::Business succession).
// Phase gates authored here per S6 fix (zero modeled gates → verification instances).
// =====================================================================
package ProjectBusiness {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // ====================================================================
    // Phase 1: Brief
    // ====================================================================
    part projectBrief : Brief {
        :>> id = "f3a8c920-1e4b-4d2a-9b6e-7c5d1f0e2a4b";
        :>> title = "SysML v2 AI Toolkit — project brief";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> problem = "Engineering projects accumulate requirements, decisions, and verification evidence in scattered prose documents that diverge from code reality, cannot be queried, and do not enforce discipline around authorship or traceability.";
        :>> opportunity = "SysML v2 text format enables a git-native, schema-enforced work-tracking engine where the process is data: items are typed, edges carry the algebra, state is computed, and AI agents operate with structured discipline rather than loose context.";
        :>> constraintsNote = "Engine runs in a conda/miniforge3 environment with the SysML v2 pilot kernel; Windows + PowerShell host; no GUI required in the engine core.";
    }

    // ====================================================================
    // Phase 2: Personas
    // ====================================================================
    part systemsEngineer : Persona {
        :>> id = "2b4e6f08-3a1c-4d5b-8e9f-0a2b4c6d8e10";
        :>> title = "Systems Engineer";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A practicing SE on a hardware/software product. Manages requirements from stakeholder needs through architecture to verification evidence. Today uses DOORS, Excel, and Word; frustrated by drift and manual reconciliation across tools.";
        :>> goals = "Maintain live, accurate traceability from stakeholder need to verification evidence without manual reconciliation.";
    }

    part teamLead : Persona {
        :>> id = "7c9e1b23-4f5a-4e6c-9d0e-1a3b5c7d9f01";
        :>> title = "Team Lead";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "Software or systems team lead who needs live project state (what is ready, blocked, or suspect) without maintaining a separate wiki or status document. Reviews and accepts phase artifacts.";
        :>> goals = "Know at a glance what is ready to pick up, what is blocked, and what evidence has gone stale.";
    }

    part aiAgentPersona : Persona {
        :>> id = "4d6f8a02-7b1e-4f3c-a0b2-3c4e5f6a7b8c";
        :>> title = "AI Agent (Claude Code)";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "Claude Code or a similar LLM agent that executes phases, authors artifacts, runs validators, and records results. Needs machine-readable structured work items and discipline guards to act autonomously without drifting out of the process.";
        :>> goals = "Execute the active phase, record results with provenance, and advance the cursor without authoring outside the active scope.";
    }

    // ====================================================================
    // Phase 3: Needs
    // ====================================================================
    requirement n01AtomicItems : Need {
        :>> id = "1a2b3c4d-5e6f-4708-9a0b-1c2d3e4f5a6b";
        :>> title = "Work items must be atomic and typed";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::customer;
        :>> statement = "Every tracked artifact is a single, independently queryable item with a typed schema and an immutable identity, connected to other items only via the typed edge algebra.";
        :>> priority = Priority::must;
    }

    requirement n02DecisionsFirstClass : Need {
        :>> id = "8b9c0d1e-2f3a-4b5c-6d7e-8f9a0b1c2d3e";
        :>> title = "Decisions are first-class, including scope cuts";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::customer;
        :>> statement = "A decision not to pursue a need is as important as a decision to act; both are authored, traced, and queryable items rather than comments or commit messages.";
        :>> priority = Priority::must;
    }

    requirement n03VerificationTraced : Need {
        :>> id = "c3d4e5f6-7a8b-4c9d-0e1f-2a3b4c5d6e7f";
        :>> title = "Verification results are appended, immutable, and traced";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::customer;
        :>> statement = "Each verification execution produces an immutable record linked to a git commit; re-verification appends a new record rather than overwriting; currency is computed from ancestry, not stored.";
        :>> priority = Priority::must;
    }

    requirement n04ComputedState : Need {
        :>> id = "e5f6a7b8-c9d0-4e1f-2a3b-4c5d6e7f8a9b";
        :>> title = "State is always computed, never authored prose";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "Status, coverage, trace, suspicion, and any other derivable answer is a query result, never a field that an agent or human types into a document.";
        :>> priority = Priority::must;
    }

    requirement n05GitNative : Need {
        :>> id = "a7b8c9d0-e1f2-4a3b-4c5d-6e7f8a9b0c1d";
        :>> title = "The engine is git-native";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "All authored facts live in text files committed to git; git history is the audit trail; suspicion and evidence currency are computed over git ancestry, not wall-clock time.";
        :>> priority = Priority::must;
    }

    requirement n06ProcessAsData : Need {
        :>> id = "b8c9d0e1-f2a3-4b4c-5d6e-7f8a9b0c1d2e";
        :>> title = "The process is readable data, not embedded in agent context";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "Workflows, phases, gates, and skill registries are authored in the model; agents read and execute them rather than carrying the process in their system prompt or memory.";
        :>> priority = Priority::must;
    }

    requirement n07AgentAutonomy : Need {
        :>> id = "c9d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f";
        :>> title = "AI agents operate autonomously within model discipline";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::customer;
        :>> statement = "An AI agent can orient, pick up a ready task, execute it, validate, record results, and commit without human intervention for each step, as long as it stays within the active phase scope.";
        :>> priority = Priority::should;
    }

    requirement n08TraceQueryable : Need {
        :>> id = "d0e1f2a3-b4c5-4d6e-7f8a-9b0c1d2e3f4a";
        :>> title = "Stakeholder traceability is queryable end-to-end";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::customer;
        :>> statement = "Given a stakeholder need, the engine can trace it forward to satisfying requirements and verification evidence, and backward to the brief that motivated it.";
        :>> priority = Priority::should;
    }

    requirement n09SelfHosting : Need {
        :>> id = "e1f2a3b4-c5d6-4e7f-8a9b-0c1d2e3f4a5b";
        :>> title = "The engine is self-hosting";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "The engine tracks its own construction using its own process; the self-build is the primary dogfood case and evidence that the process works.";
        :>> priority = Priority::could;
    }

    // ====================================================================
    // Phase 4: Use Cases
    // ====================================================================
    use case uc01Orient : UseCase {
        :>> id = "f2a3b4c5-d6e7-4f8a-9b0c-1d2e3f4a5b6c";
        :>> title = "Orient — get the ready/suspect frontier";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A team lead or agent queries the project state to learn what is ready to pick up, what is suspect (stale evidence), and where the cursor is.";
        :>> mainSuccess = "User runs query.py orient; receives a JSON report with cursor, ready list, suspect list, and done/outstanding counts; no mutation occurs.";
        :>> extensions = "If the conda/kernel environment is not on PATH, the query fails with a helpful error pointing to the full executable path.";
    }

    use case uc02RecordDecision : UseCase {
        :>> id = "03b4c5d6-e7f8-4a9b-0c1d-2e3f4a5b6c7d";
        :>> title = "Record a decision (including scope cuts)";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "An engineer records a significant project decision — including a decision NOT to pursue a need — as a first-class typed item with rationale, provenance, and supersede edges where applicable.";
        :>> mainSuccess = "Author a Decision part in .tracking, add a #Supersede edge if applicable, validate green, commit.";
        :>> extensions = "If the decision supersedes a Need, the supersede edge is added and the need no longer appears as outstanding.";
    }

    use case uc03TraceNeed : UseCase {
        :>> id = "14c5d6e7-f8a9-4b0c-1d2e-3f4a5b6c7d8e";
        :>> title = "Trace a need to its verification evidence";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A systems engineer queries the satisfy-edge graph to see which requirements satisfy a given need and what verification evidence exists for those requirements.";
        :>> mainSuccess = "User runs query.py trace-need <id>; receives upstream (brief) and downstream (satisfying requirements, verification results) in one view.";
        :>> extensions = "If no satisfy edges exist yet (Architecture phase not started), the tool reports the gap clearly rather than returning empty results silently.";
    }

    use case uc04BusinessRun : UseCase {
        :>> id = "25d6e7f8-a9b0-4c1d-2e3f-4a5b6c7d8e9f";
        :>> title = "Run the Business workflow on a project";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A team initiates a new project by running the Business workflow: authoring a Brief, deriving Personas, eliciting Needs, and defining Use Cases, with phase gates confirming each transition.";
        :>> mainSuccess = "Agent orients, moves cursor to Business, authors Brief through UseCases phase-by-phase, each phase gate passes; team lead confirms; cursor advances.";
        :>> extensions = "If a candidate need is out of scope, a Decision is authored to supersede it rather than deleting the need.";
    }

    use case uc05AuthorNeed : UseCase {
        :>> id = "36e7f8a9-b0c1-4d2e-3f4a-5b6c7d8e9f0a";
        :>> title = "Author a new need and its verification criterion";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A team member authors a new stakeholder need as a requirement usage instance with source, priority, and a solution-free statement, then establishes its DoD as a verification criterion.";
        :>> mainSuccess = "Need authored in .tracking with UUID, source, priority, statement; verification criterion authored; validate_tracking green; committed.";
        :>> extensions = "If the need duplicates an existing one, a supersede edge is authored rather than deleting the duplicate.";
    }

    use case uc06WhatsNext : UseCase {
        :>> id = "47f8a9b0-c1d2-4e3f-4a5b-6c7d8e9f0a1b";
        :>> title = "Get the next ready tasks from the backlog";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "An agent or team lead queries all tracked backlogs to get the ready frontier: tasks whose predecessors are done and whose own DoD has not yet passed.";
        :>> mainSuccess = "User runs query.py orient (or a workflows subcommand); receives the ready list across all tracked action defs from a single tool with no split-brain.";
        :>> extensions = "If whats_next.py and query.py give different answers, the discrepancy surfaces as a suspect item until the tools are reconciled.";
    }

    use case uc07AgentExecute : UseCase {
        :>> id = "58a9b0c1-d2e3-4f4a-5b6c-7d8e9f0a1b2c";
        :>> title = "Agent executes a phase autonomously";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "An AI agent orients, picks up the top ready task in the active phase, produces the phase artifacts, validates, records results with provenance, and commits without human intervention for each step.";
        :>> mainSuccess = "Agent orients, classifies the request, acts within active phase scope, validates green, records TestResult with judgedAgainst commit, commits.";
        :>> extensions = "If the task requires human acceptance (method = confirmation), agent presents the artifact and waits for explicit sign-off before recording the result.";
    }

    use case uc08ChangeRequest : UseCase {
        :>> id = "69b0c1d2-e3f4-4a5b-6c7d-8e9f0a1b2c3d";
        :>> title = "Propose a schema or workflow change";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "An engineer proposes a change to a workflow, phase gate, or schema definition via the Change Request path: state the change and rationale, get acceptance, apply, validate, record Decision.";
        :>> mainSuccess = "CR authored, rationale stated, human acceptance obtained explicitly, change applied, validate_all green, Decision recorded, committed with CR: prefix.";
        :>> extensions = "If the change alters computed-view semantics, it is routed as CHANGE (not BOOTSTRAP) even if it only touches tooling code.";
    }

    use case uc09BaselineView : UseCase {
        :>> id = "7ac1d2e3-f4a5-4b6c-7d8e-9f0a1b2c3d4e";
        :>> title = "Produce a baseline view at a release commit";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A release manager authors a Release item (a specific commit SHA plus explicit member references) from which the engine computes a frozen snapshot of all items and their states at that commit.";
        :>> mainSuccess = "Release item authored with commit SHA; query.py baseline renders all items and their states as of that commit; view is clearly marked as derived and regenerable.";
        :>> extensions = "If the commit SHA is not in the ancestry of current HEAD, the tool warns before rendering.";
    }

    use case uc10IssueLoop : UseCase {
        :>> id = "8bd2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f";
        :>> title = "File and route a field issue to a backlog task";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "An operator files an issue observed in the field or during development as a typed Issue item in .tracking; the issue is linked to a backlog task and surfaces in query.py issues.";
        :>> mainSuccess = "Issue authored per convention in .tracking/issues.sysml; surfaces in query.py issues; linked to a backlog task via edge; validate_tracking green.";
        :>> extensions = "If no matching backlog task exists, a new task is authored and the issue linked to it.";
    }

    // ====================================================================
    // Phase gates (S6 fix: authored verification instances per phase).
    // TestResults are appended when the team lead confirms each phase.
    // ====================================================================
    verification briefGate : Test {
        :>> id = "9ce3f4a5-b6c7-4d8e-9f0a-1b2c3d4e5f6a";
        :>> title = "Brief phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Brief authored with problem, opportunity, and constraintsNote; problem statement is solution-free; opportunity names the mechanism; team lead accepts.";
    }
    part briefGateR1 : TestResult { :>> id = "b1c2d3e4-f5a6-4b7c-8d9e-0f1a2b3c4d5e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f70f0e"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }

    verification personasGate : Test {
        :>> id = "0df4a5b6-c7d8-4e9f-0a1b-2c3d4e5f6a7b";
        :>> title = "Personas phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "At least two distinct personas authored with description and goals; each represents a real stakeholder segment; team lead accepts.";
    }
    part personasGateR1 : TestResult { :>> id = "c2d3e4f5-a6b7-4c8d-9e0f-1a2b3c4d5e6f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f70f0e"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }

    verification needsGate : Test {
        :>> id = "1e05b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c";
        :>> title = "Needs phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "At least five Needs authored; each carries source, priority, and a solution-free statement; MoSCoW priorities distributed across must/should/could; team lead accepts.";
    }
    part needsGateR1 : TestResult { :>> id = "d3e4f5a6-b7c8-4d9e-0f1a-2b3c4d5e6f7a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f70f0e"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }

    verification useCasesGate : Test {
        :>> id = "2f16c7d8-e9f0-4a1b-2c3d-4e5f6a7b8c9d";
        :>> title = "Use Cases phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "At least five UseCases authored; every must-priority Need is covered by at least one UseCase; mainSuccess and extensions present on each; team lead accepts.";
    }
    part useCasesGateR1 : TestResult { :>> id = "e4f5a6b7-c8d9-4e0f-1a2b-3c4d5e6f7a8b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2f70f0e"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "wweatherholtz"; }

    // ====================================================================
    // DependsOn edges: Needs derive their context from the Brief
    // ====================================================================
    #DependsOn dependency from n01AtomicItems to projectBrief;
    #DependsOn dependency from n02DecisionsFirstClass to projectBrief;
    #DependsOn dependency from n03VerificationTraced to projectBrief;
    #DependsOn dependency from n04ComputedState to projectBrief;
    #DependsOn dependency from n05GitNative to projectBrief;
    #DependsOn dependency from n06ProcessAsData to projectBrief;
    #DependsOn dependency from n07AgentAutonomy to projectBrief;
    #DependsOn dependency from n08TraceQueryable to projectBrief;
    #DependsOn dependency from n09SelfHosting to projectBrief;

    // ====================================================================
    // Rust parser CLI initiative (2026-06-12): personas
    // ====================================================================
    part rustEngineer : Persona {
        :>> id = "ee6d528f-bb6c-43bc-a3f8-a8512c46b46e";
        :>> title = "Rust/Systems Engineer";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A systems developer building the Rust parser CLI who needs deterministic startup, zero-panic runtime guarantees, and a spec-pinned grammar. Frustrated by 20-second JVM cold starts blocking tight feedback loops; needs sub-second validate/orient for productive iteration.";
        :>> goals = "Build a parser that is panic-free, spec-traceable, and fast enough that it disappears from the workflow.";
    }

    part qualityEngineer : Persona {
        :>> id = "87b50f01-6be5-4188-9e82-360f00217feb";
        :>> title = "Quality/Reliability Engineer";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "Responsible for enforcing TDD/BDD practices, #![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)] compliance, property-based testing (proptest), BDD scenarios (cucumber-rs), and zero-unwrap policies. Catches regressions before they reach main.";
        :>> goals = "Every production code path has property coverage; every panic possibility is documented or eliminated; the build fails loudly on any drift from quality gates.";
    }

    // ====================================================================
    // Rust parser CLI initiative (2026-06-12): needs
    // ====================================================================
    requirement n10NoJvm : Need {
        :>> id = "93d69860-a5de-47d6-bfcd-f9503fe9c983";
        :>> title = "No JVM runtime dependency for parse and validate";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "The system shall provide parsing and validation of the engine's .sysml dialect without requiring a JVM, Jupyter kernel, or conda environment — a single Rust binary shall suffice.";
        :>> priority = Priority::must;
    }

    requirement n11FastStart : Need {
        :>> id = "dcd4d3c8-f946-484c-a05f-3541bd83fef4";
        :>> title = "Sub-second cold-start for parse and orient";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::customer;
        :>> statement = "A full parse-and-orient cycle over the current repo shall complete in under 500ms on developer hardware, enabling tight feedback loops without context-switching overhead.";
        :>> priority = Priority::must;
    }

    requirement n12PanicFree : Need {
        :>> id = "0f426c1f-3f34-4098-8ff8-dbd35d555c3c";
        :>> title = "Panic-free production runtime";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "All production-path Rust code shall compile under #![deny(warnings, clippy::all, clippy::pedantic, clippy::nursery)] with no .unwrap(), .expect(), or unguarded indexing; every recoverable error shall be surfaced via a typed error hierarchy.";
        :>> priority = Priority::must;
    }

    requirement n13SpecPin : Need {
        :>> id = "00f66b58-4292-4c5e-9007-c00d5c52f28e";
        :>> title = "Grammar pinned to canonical SysML v2 spec URL";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::customer;
        :>> statement = "The parser shall embed the canonical SysML v2 specification URL and grammar version in build.rs; the build shall fail if the local dialect diverges from the pinned spec version beyond declared extensions.";
        :>> priority = Priority::must;
    }

    requirement n14TddBdd : Need {
        :>> id = "58f1e760-d7d4-44c7-83a8-1cc747665b5c";
        :>> title = "TDD/BDD test coverage enforced by CI";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "All parser behavior shall be driven by failing BDD scenarios (cucumber-rs) and property-based tests (proptest) before implementation; CI shall reject PRs where cargo test or cargo clippy -D warnings fails.";
        :>> priority = Priority::should;
    }

    // ====================================================================
    // Rust parser CLI initiative (2026-06-12): use cases
    // ====================================================================
    use case uc11ParseNoJvm : UseCase {
        :>> id = "26d7e539-9694-4d23-89ea-73fa9b67a692";
        :>> title = "Parse and validate .sysml files without JVM";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A developer or CI runner validates tracking and schema files using the Rust CLI, receiving structured error output with file/line context within 500ms — no conda, no JVM, no kernel startup.";
        :>> mainSuccess = "User runs `sysmlv2 validate .tracking/`; receives PASS/FAIL per file with error location and message; exits in under 500ms.";
        :>> extensions = "If a file references a type not in the loaded schema, the error names the unresolved reference and the file line where it appears.";
    }

    use case uc12SpecCompat : UseCase {
        :>> id = "47d76e86-424c-4fea-ab6e-47493e7faf68";
        :>> title = "Spec compliance checked at build time";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "A Rust developer builds the sysmlv2-parser crate; build.rs fetches the grammar manifest from the canonical SysML v2 spec URL and verifies the hash matches the pinned version. Build fails if diverged.";
        :>> mainSuccess = "cargo build succeeds; build.rs prints 'SysML v2 grammar pinned at <version> — OK'.";
        :>> extensions = "If SYSML_V2_SPEC_OFFLINE=1, skip the remote check and use the locally cached manifest.";
    }

    use case uc13RustCli : UseCase {
        :>> id = "e91551d5-298f-4dec-aa55-52caa2cae992";
        :>> title = "Orient and query via Rust CLI (no kernel)";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "An agent or developer runs `sysmlv2 orient` or `sysmlv2 trace-need <name>` and gets the same JSON output as query.py — cursor, ready list, suspect list — without starting a JVM.";
        :>> mainSuccess = "Output is byte-for-byte JSON-equivalent to query.py orient for the same repo state.";
        :>> extensions = "If a required tracking file is missing, the CLI reports which file and exits non-zero.";
    }

    use case uc14SprintRetro : UseCase {
        :>> id = "50a2857d-a808-47b4-a699-b088106c4399";
        :>> title = "Sprint retro: review and update processes";
        :>> createdAt = "2026-06-12";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> description = "At the end of each Rust parser sprint, the team lead and agent review what went well, what to improve, and whether any process changes (workflow phases, DoD criteria, skill registry updates) are warranted. A Decision is authored for any accepted change.";
        :>> mainSuccess = "Retro held; action items identified; any accepted process changes recorded as Decisions; retroGate confirmed by team lead.";
        :>> extensions = "If no process changes are warranted, a 'no change' Decision is still authored to record the retro occurred.";
    }

    // ====================================================================
    // DependsOn edges: Rust parser Needs derive their context from the Brief
    // ====================================================================
    #DependsOn dependency from n10NoJvm to projectBrief;
    #DependsOn dependency from n11FastStart to projectBrief;
    #DependsOn dependency from n12PanicFree to projectBrief;
    #DependsOn dependency from n13SpecPin to projectBrief;
    #DependsOn dependency from n14TddBdd to projectBrief;
}
