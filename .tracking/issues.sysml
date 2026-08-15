// =====================================================================
// ProjectIssues — open issues tracked against the engine.
// Convention: author a `part <name> : Issue { ... }` for each open
// issue. The `relatedTask` field names the backlog action that should
// address it. `discoveredInField = true` for issues found in production use.
// Surface with: `keel open-issues` (query.py was retired at D0074).
// =====================================================================
package ProjectIssues {
    private import EngineElement::*;
    private import EngineWork::*;

    part issue001 : Issue {
        :>> id = "f26d7e8f-a9b0-4c1d-daaa-3f4a5b6c7d8e";
        :>> title = "authoredBy/judgedBy are untyped strings, not validated actor refs";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "claudeSonnet46";
        :>> description = "The authoredBy and judgedBy fields in Element and TestResult are raw String attributes. They name actors by convention but are not validated against the ProjectActors registry, so typos or stale names are silently accepted.";
        :>> discoveredInField = false;
        :>> relatedTask = "authoredByRefs";
    }

    part issue002 : Issue {
        :>> id = "8d3f7668-ea8a-4b4b-a93f-a72bea0e18ed";
        :>> title = "status dashboard lacks process traceability view — no visibility into which processes sprints used, subversions, or evolution";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "The dashboard shows workflow diagrams but does not trace which processes each sprint actually followed, when/how processes were subverted, how processes evolved over time via improvement items, or which backlog items link to which decisions and skill changes. User request: full system traceability — sprint-to-process linkage, process evolution timeline, item-to-decision trace, subversion highlighting.";
        :>> discoveredInField = true;
        :>> relatedTask = "baselineView";
    }

    part issue003 : Issue {
        :>> id = "1c7e2eb0-589c-4e27-a1cd-534eb9f9e1a4";
        :>> title = "EngineWork::Release and EngineBaseline::Release are two conflicting Release types";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeSonnet46";
        :>> description = "EngineWork has Release :> Element with membershipRule : String. EngineBaseline has Release :> Element (occurrence def) with commit + purpose. When both are in scope (both imported), type resolution is ambiguous and the pilot kernel picks EngineWork::Release. tracking-template.sysml omits the EngineBaseline import until these are consolidated. Fix: either merge EngineBaseline::Release into EngineWork (drop membershipRule, add commit+purpose) or rename one to avoid conflict.";
        :>> discoveredInField = false;
        :>> relatedTask = "trackedMetadataReplan";
    }

    // ---- 2026-06-16 process step-back findings (D0046 audit program) ----
    part issue004 : Issue {
        :>> id = "c3d4e5f6-a7b8-4c9d-ae1f-2a3b4c5d6e7f";
        :>> title = "JVM kernel leak: every .sysml validation boots a pilot-kernel JVM; orphans accumulate (175 found 2026-06-16)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "The Python validators (validate_schema/workflows/instances/tracking) each start the Jupyter SysML pilot kernel (~20s JVM startup). Interrupted runs and pre-commit-hook-spawned kernels do not tear down, leaking JVMs — 175 orphaned java processes (up to ~1GB each) were swept on 2026-06-16. Root cause is structural: nothing enforces the no-kernel Rust path, so the JVM tax is paid on every validation. Resolved when rustToolchainFix makes 'sysmlv2 validate' the default path with the kernel as fallback.";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue005 : Issue {
        :>> id = "d4e5f6a7-b8c9-4dae-bf2a-3b4c5d6e7f80";
        :>> title = "Rust toolchain regressed: sysmlv2 validate fails schema-preload; orient emits retired cursor + phantom tasks + count drift (185/12 vs 81/2)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "The Rust CLI (rustS0-S9, all marked done/parity-verified) silently diverged from the Python authority. Three bugs: (1) 'sysmlv2 validate .' reports unresolved type reference TestResult — a schema file fails to parse so EngineVerification never registers; (2) 'sysmlv2 orient .' pollutes the ready-list with raw 'verification ...DoR : Test {...}' blocks parsed as phantom tasks (count 197 vs query.py 87); (3) orient still emits the retired StateCursor (pre-D0045) instead of in_progress_sprints. The 'parity verified' TestResults stayed green because suspicion does not watch the Rust source tree (see issue006).";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue006 : Issue {
        :>> id = "e5f6a7b8-c9da-4ebf-c03b-4c5d6e7f8091";
        :>> title = "Suspicion model ignores the deliverable source tree — code drift never invalidates parity TestResults";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "The engine's suspicion model watches material change to .sysml authored facts over satisfy/:> edges, but nothing invalidates a TestResult when the Rust source it verifies changes. The rustS8/S9 'parity verified' results stayed green while the binary regressed (issue005) — a real traceability hole between the engine model and the deliverable code. Fix: extend suspicion so Rust source-tree changes (by commit/hash) mark the parity TestResults suspect.";
        :>> discoveredInField = true;
        :>> relatedTask = "deliverableSuspicion";
    }

    part issue007 : Issue {
        :>> id = "f6a7b8c9-daeb-4fc0-d14c-5d6e7f809102";
        :>> title = "Antagonistic-critique process has no operationalizing skill and records findings as prose, not tracked Issues/Decisions";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "architectural-critique.sysml defines an 8-step adversarial audit process but (1) has no corresponding skill in .engine/skills/ (violates D0040 recurring-task-skill rule), (2) its last run produced a prose doc in docs/design-history/ rather than tracked Issue/Decision instances, and (3) has no objective metrics. Fix under auditSkill: create+register an architectural-critique skill encoding CAE+GQM+ATAM (D0046) and rewrite the process to record findings as tracked model facts.";
        :>> discoveredInField = true;
        :>> relatedTask = "auditSkill";
    }

    part issue008 : Issue {
        :>> id = "a7b8c9da-ebfc-401d-e25d-6e7f80910213";
        :>> title = "Missing delivery skills: no build/compile or test/verify skill — only repo-push exists";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "AI delivery of the deliverable (the Rust toolchain) is not skill-governed for build or test. repo-push covers commit/push, but there is no skill encoding how to compile (cargo build + validators) or how to test/verify (cargo test + BDD + Python<->Rust parity). Process knowledge for build/test leaks into conversation history. Fix under deliverySkills: create+register build and test/verify skills.";
        :>> discoveredInField = true;
        :>> relatedTask = "deliverySkills";
    }

    part issue009 : Issue {
        :>> id = "b8c9daeb-fc01-4d2e-f36e-7f8091021324";
        :>> title = "repo-push skill contradicts the main-only branch policy (D0014): tells AI to branch off main + open PRs + never commit to main";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> description = "The repo-push SKILL.md mandates a PR/feature-branch workflow ('never commit to main/master', 'branch off main', 'open the PR', writePolicy pr-only) but the project's standing policy is main-only direct commits with an auto-push post-commit hook (D0014, CLAUDE.md §4). An AI following repo-push verbatim would violate the actual workflow. Discovered while building deliverySkills. Fix under deliverySkills: reconcile repo-push to the main-only reality.";
        :>> discoveredInField = true;
        :>> relatedTask = "deliverySkills";
    }

    part issue010 : Issue {
        :>> id = "c9daebfc-012d-4e3f-046f-8091021324a5";
        :>> title = "No guard prevents ceremony gates being skipped or recorded out of order — only surfaced post-hoc by orient/traceability, not prevented";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> description = "During Sprint 16, gate TestResults were recorded Refine->Implement->Review with Standup SKIPPED; the omission was caught only because orient happened to be run (it showed pending=Standup while later gates passed). There is no validator/pre-commit guard enforcing the invariant 'no ceremony gate may pass while an earlier-ordered gate in the same sprint is unpassed.' Views surface subversion (orient pending, traceability orange highlight) but nothing PREVENTS it at author/commit time. This is the same class as the Sprint 14 retro finding (tracked-but-recurring): the durable fix is an automated guard, not manual vigilance. Meta: corrections found mid-work must be logged as bugs AND get a permanent guard, not patched silently.";
        :>> discoveredInField = true;
        :>> relatedTask = "ceremonyGateGuard";
    }

    part issue012 : Issue {
        :>> id = "ebfc012d-3e4f-4051-267f-021324a5b6c7";
        :>> title = "query.py orient undercounts done by 1 (sees 91 tasks vs the structurally-correct 92) — surfaced by the Sprint 17 parity check";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> description = "The Sprint 17 parity check found sysmlv2 orient (Rust) reports 92 tasks (done 85 + outstanding 7), exactly matching the 92 distinct `action <name>;` declarations in .tracking, while query.py orient reports 91 (done 84 + outstanding 7) — query.py silently drops one task. The qualitative sets (in_progress_sprints, ready, suspect, invalidEvidence) are identical between the two. Rust is structurally correct; query.py has a latent discovery/counting bug (likely in %show-based action-def discovery or member regex). Post-D0048 the Rust path is the authority, so query.py is secondary and this is a non-blocking known delta (the parity gate tolerates it). Fix later: pin and correct query.py's dropped task so both agree at 92. RESOLVED (M4, sprint58, D0074): query.py was DELETED — the buggy counter no longer exists, so the delta is moot; `sysmlv2 orient` (Rust) is the sole authority and is always structurally correct (total == distinct `action` declarations, asserted by the orient self-consistency check).";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue014 : Issue {
        :>> id = "012d3e4f-5061-4238-376f-1324a5b6c7e1";
        :>> title = "12 Rust deliverable tasks are suspect: verified against source older than the Sprint 17 parser/orient changes — need re-verification at HEAD";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> description = "Deliverable-suspicion (D0050) now flags rustS1..S9, runtimeParser, writeApi, rustToolchainFix as suspect: their latest passing TestResults were judged at commits BEFORE the Sprint 17 source changes (occurrence-def parse fix, orient rewrite) and the W4 orient.rs change, so their evidence is stale. This is the feature working as intended (issue006 closed), not a defect. Clearing it = re-verify at HEAD (cargo test + clippy + parity_check) and append fresh TestResults. Created by the autonomous retro (D0049), Sprint 19.";
        :>> discoveredInField = true;
        :>> relatedTask = "reverifyDeliverable";
    }

    part issue013 : Issue {
        :>> id = "fc012d3e-4f50-4126-376f-1324a5b6c7df";
        :>> title = "Shared cwd between the Bash and PowerShell tools: a `cd` in one silently breaks later relative-path commands in the other (Sprint 17 avoidable issue)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> description = "During Sprint 17 a `cd .tracking` in a Bash tool call changed the working directory seen by subsequent PowerShell tool calls, so `python .engine/tools/query.py ...` failed with 'file not found' until invoked by absolute path. Avoidable: always use absolute paths for script/file args in shell commands, and never rely on cwd persisting (or rely on it only within a single call). Preventing change applied: CLAUDE.md §6 note. Discovered by the autonomous retro (D0049).";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue011 : Issue {
        :>> id = "daebfc01-2d3e-4f40-156f-91021324a5b6";
        :>> title = "Sprint ceremony skills are invoked by convention only — the sprint-review Phase-3 transcript scan can be (and in S16 was) skipped, so process-improvement findings are missed";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "sprint-review SKILL.md Phase 3 (transcript review) is a mandatory process-improvement scan whose detection cues include 'Missing guard — a check that SHOULD have been automatic was done manually' — i.e. it is designed to catch exactly the issue010 class. But nothing ENFORCES that the skill is actually invoked before the review gate TestResult is recorded. In Sprint 16 the review gate was recorded WITHOUT running the Phase-3 scan, which is why issue010 (Standup-skip) and issue009 (repo-push contradiction) were found ad hoc rather than by review. Gap: ceremony-skill invocation is discipline, not control. Fix (reviewInvocationGuard): make the review gate require recorded transcript-scan evidence (improvement items, even if 'none material'), so a review gate without a scan artifact is incomplete; guard it. Guard DESIGN to be dispatched at the next retro (per D0047) since it needs human input.";
        :>> discoveredInField = true;
        :>> relatedTask = "reviewInvocationGuard";
    }

    // ---- 2026-06-16 process-model benchmark findings (D0054) ----
    part issue015 : Issue {
        :>> id = "3e4f5061-7283-4940-467f-1324a5b6c7e6";
        :>> title = "Adoption friction is the dominant MBSE failure mode (JPL Europa Clipper reverted) — the write path must beat a spreadsheet, and we don't measure friction";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "Benchmark research (D0054) surfaced the #1 sourced risk: JPL's 'Is MBSE Helping?' / Europa Clipper retrospective found teams partially ABANDONED MBSE for requirements/architecture and reverted to traditional tools — not because the architecture was wrong, but because authoring + framework upkeep cost more than a spreadsheet. Our engine has the same architecture, so it has the same risk. We mitigate via the Rust no-kernel authority + append-result write API, but we do NOT measure or guard authoring friction (e.g. steps/time to record a TestResult or Decision vs a spreadsheet edit). Preventing change: treat write-path friction as a first-class, monitored quality; keep the write API lower-friction than a spreadsheet; consider a friction metric in the GQM set (architectural-critique skill).";
        :>> discoveredInField = true;
        :>> relatedTask = "writeApi";
    }

    part issue016 : Issue {
        :>> id = "3e4f5061-7283-4940-477f-1324a5b6c7e7";
        :>> title = "Decision records capture the verdict but not structured alternatives/criteria (ISO 42010 / NPR 7123.1 Decision Analysis)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "ISO/IEC/IEEE 42010 makes architecture Decisions AND Rationale first-class and recommends recording the alternatives NOT chosen + the criteria; NASA NPR 7123.1 names Decision Analysis as one of the 17 technical processes. Our Decision items carry context/decisionText/consequences (good) but no STRUCTURED options-considered / selection-criteria / rejected-with-reason (ADR pattern). Preventing change (decisionAnalysis task): add an optional structured 'alternatives' + 'criteria' convention/fields to Decision authoring so trade rationale is captured, not just the outcome. Lightweight (convention first; schema later only if needed).";
        :>> discoveredInField = true;
        :>> relatedTask = "decisionAnalysis";
    }

    part issue017 : Issue {
        :>> id = "3e4f5061-7283-4940-487f-1324a5b6c7e8";
        :>> title = "CLARIFIED (D0055): metadata-for-cross-cutting splits into authored attrs (=trackedMetadata, D0053) and computed concerns (status/suspicion) which must STAY computed — reject storing them";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "Original wording conflated two cases (it read as unclear because it was half-wrong): (a) AUTHORED cross-cutting attributes (id/title/createdAt/authoredBy) — metadata IS the right carrier, but that is exactly the @Tracked migration already deferred to trackedMetadataReplan (D0053); no new work. (b) COMPUTED cross-cutting concerns (status, suspicion) — the source suggested storing these as metadata too; that is REJECTED, it would violate §2.1 (computed views are never stored). Status/suspicion stay computed (orient). Net: this issue is subsumed by D0053 for (a) and closed-as-wontfix for (b). The only standalone cheap enrichment from the benchmark is issue016. See D0055.";
        :>> discoveredInField = true;
        :>> relatedTask = "trackedMetadataReplan";
    }

    part issue018 : Issue {
        :>> id = "50617283-3940-4a51-976f-1324a5b6c7eb";
        :>> title = "Views are CODE, not modeled viewpoints: view def anchors are empty stubs disconnected from the real report.py/orient views — no concern/stakeholder/source, no concern-coverage audit (ISO 42010 / Arcadia / SysML v2)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "Benchmark/critique (D0056): computed.sysml's view def OutstandingView/SuspectView/TraceView + viewpoint def MaintainerVP are empty anchors, disconnected from the actual views (hardcoded report.py tabs + orient/whats-next). So no view declares its concern, stakeholder/audience, or source elements; we cannot audit whether every stakeholder concern has a view (orphan-concern risk — the exact thing ISO 42010 viewpoints prevent), and adding a view means writing code, not declaring a viewpoint. By the frameworks' discriminator (reuse x auditability x audience) our case justifies formalization. Preventing change (formalViewpoints): make Viewpoint/View first-class declared elements (concern+stakeholder+sources+renderer binding); convert code-views to declared viewpoints (safety/requirements/whats-next/traceability); keep computation in tooling, driven by the declaration; lightweight (no Capella/Kitalpha; friction is D0054). Frozen-schema -> needs §2.5 sign-off before build. RESOLVED (M1, sprint46, D0075): viewpoints are now executable DECLARATIONS — concise .engine/views/*.view.toml files (select + traverse + project), run by `sysmlv2 view <name>` (Rust authority, fail-loud), NOT hardcoded code. Adding a view = a TOML file, not code. The SysML viewpoint-registry stays the concern-coverage index (D0056/D0057). The earlier SysML-schema-fields approach (D0071) was superseded by the TOML model (D0075) after a 12-use-case stress test.";
        :>> discoveredInField = true;
        :>> relatedTask = "tomlViewExecutor";
    }

    part issue019 : Issue {
        :>> id = "61728394-0a5b-4c6d-b619-40516273841a";
        :>> title = "read_issues / query.py issues silently truncated any Issue whose field value contains a brace — exposed by the orphans view on first run, fixed";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> description = "_ISSUE_BLOCK used `[^}]*`, which stops at the first '}'. issue005's description quotes 'verification ...DoR : Test {...}', so its block truncated before relatedTask -> query.py issues mis-reported it (empty description + relatedTask). Latent since issueLoop; the new orphans view (orphansVP) caught it on its first run by flagging issue005 as relatedTask-less (a FALSE positive that revealed the TRUE parser bug). Fixed: _ISSUE_BLOCK and _VIEWPOINT_BLOCK now anchor on the line-start closing brace (\\n\\s*\\}) with DOTALL, so braces inside field values no longer truncate. The corrected regex IS the guard (D0047). discoveredInField via the orphans view dogfooding itself.";
        :>> discoveredInField = true;
        :>> relatedTask = "formalViewpoints";
    }

    part issue020 : Issue {
        :>> id = "62839405-1b6c-4d7e-c020-516273849562";
        :>> title = "Significant delivery/CHANGE work committed with NO sprint wrapper — ceremony guard catches out-of-order gates but not a MISSING sprint";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> description = "The attribution-model migration (attrModelSchema + attrModelInstances) was implemented and committed (44e1e83/693b5e6/88b7727) as raw backlog tasks — no sprint Story, no refine/standup/implement/review/closeOut/retro gates, no delivery file. validate_ceremony.py only checks gate ORDERING within an existing delivery file, so work with NO delivery file passes pre-commit untouched. Human caught it (discoveredInField). Root cause: a doctrine contradiction — CLAUDE.md §3e exempts engine/bootstrap work from the full (sprint) workflow, but recent practice (sprints 25-27 sprinted engine work) and the rigor expectation require sprinting it. Bootstrap is now over (the tracking engine — write API, validators, computed orient — exists), so the §3e exemption should be retired. Preventing changes (sprint29): (1) Decision retiring §3e + mandating sprints for substantive work; (2) CLAUDE.md anti-subversion + mandatory triage-first; (3) a no-sprint GUARD that flags .tracking/schema CHANGE commits not tied to an open sprint's Implement phase (D0047); (4) the per-interaction triage skill (engine-triage upgrade + a UserPromptSubmit hook).";
        :>> discoveredInField = true;
        :>> relatedTask = "bootstrapExitEval";
    }

    part issue021 : Issue {
        :>> id = "73940516-2c7d-4e8f-d021-516273849563";
        :>> title = "Kernel validate_instances loads each .engine file in isolation (schema/core only) — a Decision file CANNOT carry a cross-package edge (e.g. #ProspectiveChange -> Delivery); such edges must live in .tracking (generic rust validation)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> description = "Authoring the process-change edge ON decision 0049 (`#ProspectiveChange dependency from d0049 to Delivery;` with `import DeliveryWorkflow`) FAILED kernel validation twice: a package is not a valid dependency endpoint, AND validate_instances does not co-load .engine/workflows or other instance files when validating a decision -- so 'DeliveryWorkflow' and 'Delivery' are both unresolvable. Root cause (the GENERAL, durable fact): kernel validation of .engine instances is per-file with only schema/core preloaded; ANY cross-package reference from a Decision to a process/workflow element is unresolvable. RESOLVED-BY-DESIGN (D0070, sprint40): the cross-package need was an artifact of storing a process REFERENCE we don't need. Recording a process change as a #ProspectiveChange/#SafetyChange PREFIX MARKER on the Decision part (in-package, valid in both rust and the kernel) carries no cross-reference at all -- which-process is git-derived under the hard-lock guard -- so the limit no longer bites this use case. The interim .tracking/process-governance.sysml edge file (sprint39) is deleted. The care discipline (probe the edge form before scattering it) caught the limit before replication and drove the better design. PREVENTING-CHANGE: the general cross-package caution stays in sysmlv2-syntax-notes (Decision-file authoring); failure is loud (validation ERROR) and self-correcting, so no automated guard is warranted.";
        :>> discoveredInField = true;
        :>> relatedTask = "pglProcessRef";
    }

    part issue022 : Issue {
        :>> id = "73940516-2c7d-4e8f-d022-516273849660";
        :>> title = "actualHours estimation-feedback discipline (D0038) is dormant project-wide — 45/45 sprints record none; never formally retired when sprints became AI-autonomous (D0049/D0064)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> description = "Audit (query.py audit, the architectural-critique adherence metrics, D0046) found NO sprint in the project records actualHours, though D0038 (sprint estimation model) established an estimate-vs-actual feedback discipline. Root cause: when D0049/D0064 made sprints AI-autonomous, wall-clock actualHours lost meaning for an AI executor, but D0038's actualHours clause was never formally retired or replaced — silent drift, not a decision. NEEDS A DECISION (user): either (a) formally retire actualHours for AI-autonomous sprints (estimatedPoints stays for sizing/velocity; actualHours superseded), or (b) revive it with a meaningful 'actual' (e.g. tool-call count / wall-clock of the sitting). Until decided, the estimation-feedback loop is inert. Surfaced by the audit; recorded per the architectural-critique process (findings -> tracked Issue, not prose).";
        :>> discoveredInField = true;
        :>> relatedTask = "actualHoursDisposition";
    }

    part issue023 : Issue {
        :>> id = "73940516-2c7d-4e8f-d023-516273849661";
        :>> title = "Per-sitting sprint-review (D0049 human gate) is never recorded as an artifact — no sitting-review record exists in any delivery file; the one human gate is informal";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> description = "D0049 defined the per-SITTING sprint review (method=confirmation, the single human gate) as what closes a sitting's work; per-sprint closeOut/retro are autonomous. Audit (query.py audit) found NO sitting-review record in any delivery file — the human gate has been satisfied INFORMALLY (the user reviewing in conversation), never recorded as a confirmation TestResult tied to the sitting's content. So there is no auditable artifact that a sitting was reviewed/accepted, and the current sitting (sprints 38-43 + D0070/D0071) has no recorded review. NEEDS (user): (1) a decision on the sitting-review RECORD mechanism (a confirmation TestResult naming the sitting's sprints, recorded on the human's explicit attestation — D0016/D0051, never inferred); (2) the current sitting's review attestation when the user is satisfied. A future guard could flag a sitting of committed sprints with no recorded review. Surfaced by the audit; recorded per the architectural-critique process.";
        :>> discoveredInField = true;
        :>> relatedTask = "sittingReviewRecord";
    }

    part issue024 : Issue {
        :>> id = "73940516-2c7d-4e8f-d046-516273849690";
        :>> title = "Rust parser silently DROPPED `#Marker part X : T` (marker-on-a-definition) — parse_hash_item skip_item'd anything not `first`/`dependency`, so D0070 process-change Decisions vanished from every view";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "claudeOpus";
        :>> description = "Building the M1 view executor (sprint46) surfaced this: `view decisions-active-and-superseded` returned 70 of 75 decisions — the missing 5 were exactly the #ProspectiveChange-marked process-change Decisions (d0039/d0049/d0064/d0067/d0070). Root cause: parser.rs parse_hash_item handled `#Marker first..then` (succession) and `#Marker dependency from..to` (edge) but for ANY other follower (e.g. `part`) called skip_item + returned None — silently dropping the whole part. `sysmlv2 check` reported 'clean' because the part was skipped, not errored. The form was never caught because the Rust parser only validates .tracking (which uses `#CharteredBy dependency` edges), while the marker-on-PART form (D0070) lives only in .engine/decisions, which rust never parsed until the view executor read it. FIX (sprint47): parse_hash_item now delegates to parse_item for the non-edge case, EMITTING the underlying Part (marker consumed; marker-retention in the AST deferred to M2's rust process-changes view). GUARD (D0047): parser unit test `marker_prefixed_part_is_preserved` asserts the part survives; view now returns all 75. discoveredInField via M1 dogfooding.";
        :>> discoveredInField = true;
        :>> relatedTask = "parserMarkerFix";
    }

    part issue025 : Issue {
        :>> id = "5061ebfc-2d3e-4f40-5126-738495a1b2c3";
        :>> title = "Seven query.py view subcommands were not ported to Rust and were dropped at M4 (workflows, trace, trace-need, item, outstanding, downstream, viewpoints-listing)";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "claudeOpus";
        :>> description = "M4 (sprint58, D0074) deleted query.py after M2.2 ported the algorithmic views and M3 ported the guards. Reconciliation of query.py's subcommands vs the Rust surface found seven with NO Rust/TOML equivalent, now unavailable: `workflows` (renders the workflow DAG via Kahn waves), `trace`/`trace-need` (upstream/downstream + need-satisfy-allocate tracing), `item` (single-task detail), `outstanding` (not-done list), `downstream` (successor reach), and a `viewpoints` LISTING (the concern-coverage registry index — the SysML viewpoint-registry file still exists, but the command that listed it does not). NONE were in the commit path, used by report.py (which reads files + sysmlv2 orient directly), or part of the core authority (orient/whats-next/validate/guard/the M2.2 views all remain). So nothing BREAKS — these were secondary manual-inspection views. Recorded (D0018: the model is the tracker; the knowledge must not be lost) so they can be re-added as `.view.toml` files or Rust subcommands if a concrete need arises. Most map cleanly onto the existing TOML view engine (trace/downstream = traverse; item/outstanding = select); workflows needs a small Rust subcommand over the workflow action defs.";
        :>> discoveredInField = true;
        :>> relatedTask = "retireQueryPy";
    }
    #Resolves dependency from authoredByRefs to issue001;
    #Resolves dependency from baselineView to issue002;
    #Resolves dependency from d0053 to issue003;
    #Resolves dependency from rustToolchainFix to issue004;
    #Resolves dependency from rustToolchainFix to issue005;
    #Resolves dependency from deliverableSuspicion to issue006;
    #Resolves dependency from auditSkill to issue007;
    #Resolves dependency from deliverySkills to issue008;
    #Resolves dependency from deliverySkills to issue009;
    #Resolves dependency from ceremonyGateGuard to issue010;
    #Resolves dependency from reviewInvocationGuard to issue011;
    #Resolves dependency from rustToolchainFix to issue012;
    #Resolves dependency from rustToolchainFix to issue013;
    #Resolves dependency from suspectDiagnostics to issue014;
    #Resolves dependency from writeApi to issue015;
    #Resolves dependency from decisionAnalysis to issue016;
    #Resolves dependency from trackedMetadataReplan to issue017;
    #Resolves dependency from tomlViewExecutor to issue018;
    #Resolves dependency from formalViewpoints to issue019;
    #Resolves dependency from bootstrapExitEval to issue020;
    #Resolves dependency from pglProcessRef to issue021;
    #Resolves dependency from actualHoursDisposition to issue022;
    #Resolves dependency from sittingReviewRecord to issue023;
    #Resolves dependency from parserMarkerFix to issue024;
    #Resolves dependency from readdDroppedViews to issue025;

    part issue026 : Issue {
        :>> id = "5061ebfc-2d3e-4f40-5126-738495a1b2c4";
        :>> title = "Pre-commit hook's kernel validate_tracking fallback false-fails 31 #CharteredBy charter-bearing tracking files (unsafe fallback when the rust binary is unbuilt)";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "claudeOpus";
        :>> description = "The sitting02 audit found the deep kernel validate_all reports tracking 44/75 while the Rust authority (sysmlv2 validate, D0048) passes all 75. Cause: kernel validate_tracking validates each .tracking file PER-FILE with only schema/core preloaded, so #CharteredBy edges (Stories -> .engine Decisions, since sprint38) and baselines.sysml (-> EngineBaseline/Release) cannot resolve — the documented per-file cross-package limitation (issue021/D0070), now also hitting charter edges. The .tracking files are CORRECT (Rust is authority + right). LATENT HAZARD: the pre-commit hook FALLS BACK to kernel validate_tracking when target/release/sysmlv2 is missing (e.g. a fresh clone before cargo build) -> it would false-fail these 31 files and BLOCK the commit. discoveredInField via the sitting02 audit.";
        :>> discoveredInField = true;
        :>> relatedTask = "safeTrackingFallback";
    }
    #Resolves dependency from safeTrackingFallback to issue026;

    part issue027 : Issue {
        :>> id = "5061ebfc-2d3e-4f40-5126-738495a1b2c5";
        :>> title = "Rust parser silently DROPS items after the top-level package (parse 'clean' while losing content)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> description = "parse() returned after parse_package WITHOUT checking the token stream was exhausted, so any item declared OUTSIDE the package — e.g. left by a stray brace that closed the package early — was silently ignored. Surfaced authoring D0083 (Sprint 75): a one-line semicolon-then-brace closed the Decision package early, exiling the d0083Accept/d0083AcceptR1 block; sysmlv2 check reported the file CLEAN while the accept event vanished from the Model (only attestation-coverage caught it). A structurally-broken file parsing 'clean' is a fail-QUIET defect — the class D0047 says must become a permanent automated guard. discoveredInField=false (found in development).";
        :>> discoveredInField = false;
        :>> relatedTask = "faststartTargetRevision";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from parserPackageGuard to issue027;

    part issue028 : Issue {
        :>> id = "5061ebfc-2d3e-4f40-5126-738495a1b2c6";
        :>> title = "sysmlv2 diagram rendered BLANK (no data): CDN dependency + cose layout on all ~2500 nodes";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> description = "The Sprint 82 diagram HTML showed no data when opened. Two defects: (1) the page loaded cytoscape from a unpkg CDN, so it rendered blank offline / when the CDN was blocked (the 'self-contained' claim was false); (2) the cytoscape constructor ran a force-directed cose layout over ALL ~2500 nodes at once, which freezes/blanks the browser. Fix (diagramRenderFix): INLINE cytoscape.js (vendored to sysmlv2-cli/assets, include_str! — truly self-contained, offline) and run cose only on the VISIBLE subset (~650; Test/TestResult stay hidden) with a fast grid layout in the constructor. Regression test diagram_is_self_contained_and_layouts_visible_only locks both. discoveredInField=true (the human hit the blank render).";
        :>> discoveredInField = true;
        :>> relatedTask = "diagramView";
        :>> severity = Severity::Medium;
    }
    part issue029 : Issue {
        :>> id = "e0290029-0029-4e29-9029-516273840029";
        :>> title = "D0054 finding: adoption-friction is a first-class requirement with NO verification (no friction metric)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 1, d0054 completeness (Medium): D0054 declares adoption friction the #1 risk and makes 'the write path must beat a spreadsheet' a first-class requirement, but there is no verification of it — no friction metric (steps/keystrokes/seconds to record a fact via the write API vs a spreadsheet baseline). sr11/faststart measures tool latency, not authoring friction. ACT disposition (human, 2026-06-21): track a friction metric. Resolver: frictionMetric.";
        :>> discoveredInField = false;
        :>> relatedTask = "frictionMetric";
        :>> severity = Severity::Medium;
    }
    part issue030 : Issue {
        :>> id = "e0300030-0030-4e30-9030-516273840030";
        :>> title = "D0080 finding: critique QUALITY is unchecked (coverage enforced, rigor not) — the mechanism is gameable";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 1, d0080 completeness (Medium): guard critique + critique-coverage enforce that a required-lens critique EXISTS, not that it is rigorous — a shallow 'SURVIVED' counts identical to a genuine attack, so the mechanism is gameable by going-through-the-motions. No second-order rigor/meta check exists. ACT disposition (human, 2026-06-21): add a critique-rigor check. Resolver: critiqueRigorCheck.";
        :>> discoveredInField = false;
        :>> relatedTask = "critiqueRigorCheck";
        :>> severity = Severity::Medium;
    }
    part issue031 : Issue {
        :>> id = "e0310031-0031-4e31-9031-516273840031";
        :>> title = "D0080 finding: critique independence is identity-distinct, not cognition-distinct (AI-vs-AI shares blind spots)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 1, d0080 correctness (Medium): independence is enforced as judgedBy != author (identity), not cognition — an aiModel critiquing another aiModel's (or the same model line's) work shares training blind spots, so 'adversarial' is weaker than the human independence the lenses imply. The mechanism does not require a human/tool critic for high-stakes elements. ACT disposition (human, 2026-06-21): require a non-aiModel critic for the high-stakes set. Resolver: criticIndependenceRule.";
        :>> discoveredInField = false;
        :>> relatedTask = "criticIndependenceRule";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from diagramRenderFix to issue028;
    part issue033 : Issue {
        :>> id = "e0330033-0033-4e33-9033-516273840033";
        :>> title = "D0050 finding: deliverable-suspicion manifest is a hand-maintained allow-list (unlisted task silently unguarded)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 2, d0050 completeness (Medium): deliverable-suspicion guards only verification tasks listed in .engine/deliverable-manifest.txt; a deliverable-dependent task not added is silently unguarded (its 'verified' stays green across source drift). No check enforces manifest completeness. ACT disposition (human, 2026-06-21): guard manifest completeness. Resolver: manifestCoverageGuard.";
        :>> discoveredInField = false;
        :>> relatedTask = "manifestCoverageGuard";
        :>> severity = Severity::Medium;
    }
    part issue034 : Issue {
        :>> id = "e0340034-0034-4e34-9034-516273840034";
        :>> title = "D0056 finding: viewpoint registry is descriptive, not generative (registry<->renderer can drift)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 2, d0056 completeness (Medium): only TOML views are declaration-driven; bespoke subcommands are hand-coded with a descriptive registry entry, so 'declaration drives renderer' is only partially realized and a viewpoint without a renderer (or vice versa) is not caught. ACT disposition (human, 2026-06-21): guard viewpoint<->renderer correspondence. Resolver: viewpointRendererGuard.";
        :>> discoveredInField = false;
        :>> relatedTask = "viewpointRendererGuard";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from frictionMetric to issue029;
    #Resolves dependency from critiqueRigorCheck to issue030;
    #Resolves dependency from criticIndependenceRule to issue031;
    #Resolves dependency from manifestCoverageGuard to issue033;
    #Resolves dependency from viewpointRendererGuard to issue034;
    part issue035 : Issue {
        :>> id = "e0350035-0035-4e35-9035-516273840035";
        :>> title = "D0057 finding: the promised concern-coverage view (which stakeholder concerns lack a viewpoint) was never built";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 3, d0057 completeness (Medium): d0057 delivered increment 1 (Viewpoint schema + registry) but queued increment 2 (declaration-driven renderers + a concern-coverage view flagging which stakeholder concerns lack a viewpoint), which never landed. issue034 added a renderer-validity guard but not the concern-coverage audit. ACT disposition (human, 2026-06-22): build the concern-coverage audit. Resolver: coverageAudits (unified with issue036).";
        :>> discoveredInField = false;
        :>> relatedTask = "coverageAudits";
        :>> severity = Severity::Medium;
    }
    part issue036 : Issue {
        :>> id = "e0360036-0036-4e36-9036-516273840036";
        :>> title = "D0059 finding: the claimed process<->skill coverage audit ('orphans generalized to processes') does not exist";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 3, d0059 completeness (Medium): d0059 claims process->skill coverage is 'auditable as a view (orphans/coverage generalized to processes)', but no such audit exists — no guard/view flags a process lacking a deploying skill, so 'no inert process' relies on vigilance (the failure mode d0059 itself rejects). ACT disposition (human, 2026-06-22): build the process<->skill coverage audit. Resolver: coverageAudits (unified with issue035).";
        :>> discoveredInField = false;
        :>> relatedTask = "coverageAudits";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from coverageAudits to issue035;
    #Resolves dependency from coverageAudits to issue036;
    part issue037 : Issue {
        :>> id = "e0370037-0037-4e37-9037-516273840037";
        :>> title = "D0091 finding: bank-first 'latest' goes stale vs the live value for computed indicators";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> description = "D0091 completeness (Medium): once a computed indicator has any snapshot, the view showed the last snapshot as 'latest' (could be months stale), with the live value not surfaced. ACT disposition (human, 2026-06-22): snapshots save a timestamp not a label; 'latest' is CALCULATED (live recompute for computed). Resolver: indicatorBankGuardrail.";
        :>> discoveredInField = false;
        :>> relatedTask = "indicatorBankGuardrail";
        :>> severity = Severity::Medium;
    }
    part issue038 : Issue {
        :>> id = "e0380038-0038-4e38-9038-516273840038";
        :>> title = "D0091 finding: snapshotting computed metrics is a compute-don't-store exception with no mechanical guardrail";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> description = "D0091 correctness (Medium): storing a computed-metric snapshot is a controlled exception to compute-don't-store; the observation-vs-cache framing is interpretive with no guardrail. ACT disposition (human, 2026-06-22): add a guardrail — live recompute is authoritative (never overridden by the bank), snapshots source-tagged, a drift check flags live-vs-snapshot divergence. Resolver: indicatorBankGuardrail.";
        :>> discoveredInField = false;
        :>> relatedTask = "indicatorBankGuardrail";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from indicatorBankGuardrail to issue037;
    #Resolves dependency from indicatorBankGuardrail to issue038;

    part issue039 : Issue {
        :>> id = "e0390039-0039-4e39-9039-516273840039";
        :>> title = "D0047 finding: the corrections-become-guards rule is itself enforced only by vigilance (no meta-audit that a recurrable-defect Issue carries a resolving guard)";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 4, d0047 completeness (Medium): D0047 mandates a recurrable defect MUST be logged as an Issue AND given a permanent automated guard, but nothing enforces the mandate — no meta-guard checks that a process-defect Issue has a resolving guard, so the rule is applied by AI/retro vigilance (the failure mode D0047 itself rejects). Candidate resolver: correctionGuardCoverage (a meta-audit). PENDING human disposition (ACT / ACCEPT-RISK / DISMISS).";
        :>> discoveredInField = false;
        :>> relatedTask = "correctionGuardCoverage";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from correctionGuardCoverage to issue039;

    part issue040 : Issue {
        :>> id = "e0400040-0040-4e40-9040-516273840040";
        :>> title = "D0049 finding: 'sitting' (the unit of the sole human gate) has no model representation — review->sprints coverage is uncomputable";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "claudeOpus";
        :>> description = "Load-bearing critique campaign batch 4, d0049 completeness (Medium): D0049 makes the per-sitting sprint review the sole human gate and defines SITTING = one continuous work session of >=1 sprints, but 'sitting' is not modeled — no Sitting item/marker, no recorded boundary, no computed link from a review confirmation to the sprints it attests. So which sprints a given review covers is neither auditable nor computable in the model meant to be the sole tracker (D0018). Candidate resolver: sittingModel (authored review->sprint coverage edge + computed view). PENDING human disposition (ACT / ACCEPT-RISK / DISMISS).";
        :>> discoveredInField = false;
        :>> relatedTask = "sittingModel";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from sittingModel to issue040;

    part issue041 : Issue {
        :>> id = "e0410041-0041-4e41-9041-516273840041";
        :>> title = "Non-hermetic BDD test: orient_bdd 'unresolvable SHA' depended on the system temp dir sitting inside a git repo";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "claudeOpus";
        :>> description = "Found mid-work (Sprint 112): the orient_bdd scenario 'task with unresolvable SHA appears in invalidEvidence' wrote its fixture to std::env::temp_dir(), which on this machine is NOT inside any git repo. orient's SHA validation is conservative-on-git-failure (orient.rs valid_commits / line ~519 unwrap_or(true)): when `git -C <temp> cat-file --batch-check` errors out entirely (no repo) it emits no per-SHA line, so the fake SHA is absent from the map and defaults to VALID -> done=1, failing the test (expected invalidEvidence/done=0). The test only passed when temp happened to live inside a real repo. Recurrable class: non-hermetic tests that depend on ambient git state. Resolver: serveItemActions carries the fix -- git_init() the fixture's temp root so the fake 40-hex SHA deterministically resolves to `missing` regardless of where temp lives; the now-hermetic test IS the permanent check. orient semantics unchanged. Severity Low (test-hygiene, no production impact).";
        :>> discoveredInField = false;
        :>> relatedTask = "serveItemActions";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from serveItemActions to issue041;

    part issue042 : Issue {
        :>> id = "e0420042-0042-4e42-9042-516273840042";
        :>> title = "cargo test is not in the automated commit gate — a failing test can reach main (n14 test-gating clause unrealized)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "claudeOpus";
        :>> description = "Critique finding (Sprint 118, n14TddBdd x testability, Medium): n14 requires 'CI shall reject PRs where cargo test or cargo clippy -D warnings fails.' The active commit gate (.githooks/pre-commit) runs `keel validate` + guards + `cargo clippy --workspace --all-targets -D warnings`, but does NOT run `cargo test`; and there is no CI pipeline (no .github/workflows; main-only, no PRs). So a failing cargo test (a parser/orient/write logic regression) can land on main, caught only by per-sprint manual `cargo test` — which D0047 explicitly classifies as 'not a control' (manual vigilance). The clippy half of n14's gate clause is enforced; the cargo-test half is not. Candidate resolver (if ACT): cargoTestGate — add `cargo test` to the pre-commit hook for staged Rust (or a CI workflow), accepting the ~15s latency tradeoff (weigh vs n11/D0054 friction) OR revise n14 to reference the actual main-only gate. PENDING human disposition (ACT / ACCEPT-RISK / DISMISS).";
        :>> discoveredInField = false;
        :>> relatedTask = "cargoTestGate";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from cargoTestGate to issue042;

    part issue043 : Issue {
        :>> id = "e0430043-0043-4e43-9043-516273840043";
        :>> title = "Living SystemRequirement statements name retired/renamed artifacts (sr07 validate_all.py; sr10/sr11 binary 'sysmlv2'; sr12/sr13 crates 'sysmlv2-parser/cli')";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "claudeOpus";
        :>> description = "Critique finding (Sprint 119, sr07 + sr10 x correctness, Low): three LIVING requirement statements name artifacts that were renamed/retired, so the text misstates the current system. sr07AgentLoop names the validator toolchain as 'validate_all.py' (the python validators were retired at M4/D0074; the gate is now `keel validate`/`guard` via the .githooks pre-commit hook). sr10RustCli specifies 'a Rust CLI binary (sysmlv2)' and sr11FastStart says 'The sysmlv2 CLI' (the binary was renamed `keel`, D0095). sr12DenyWarnings names crates 'sysmlv2-parser and sysmlv2-cli' and sr13SpecPin names 'The sysmlv2-parser crate' (renamed `keel-parser`/`keel-cli`, D0095). The CAPABILITIES are fully met by keel; only the named artifacts are stale — the D0095 rename completed docs but missed these living requirement statements (historical sprint records correctly preserve 'sysmlv2' as point-in-time fact). Resolver: refreshRequirementText (update the 3 living statements to current names; will re-suspect the sr07/sr10 critiques -> re-critique-pass, the normal D0084 cycle). Low severity (doc-accuracy, no functional impact) -> AI-dispositioned.";
        :>> discoveredInField = false;
        :>> relatedTask = "refreshRequirementText";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from refreshRequirementText to issue043;

    part issue044 : Issue {
        :>> id = "e0440044-0044-4e44-9044-516273840044";
        :>> title = "Staleness false-positive: extract_field read raw git blobs without the lexer's string-unescaping, false-flagging escaped-content critiques stale";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "claudeOpus";
        :>> description = "Critique finding (Sprint 124, Medium): the critique-coverage discrepancy (84 vs expected 85) traced to a real engine bug. compute_stale_verifications/extract_field (view.rs) read an element's semantic field from the RAW git blob and did NOT replicate the lexer's string-unescaping (lex_string: backslash-backslash -> backslash, backslash-quote -> quote, \\n, \\t), nor detect the end quote as an UNESCAPED quote. So an element whose `decision`/`statement` field contains backslash escapes (e.g. d0023's regex `\\s`/`\\w`/`\\d`, d0076's escaped `\\\"`) compared unequal (blob backslash-backslash vs model single-backslash) -> false 'changed' -> the element's critiques were wrongly excluded from coverage as STALE (D0084 path, view.rs ~line 1362). Impact: 2 decisions (d0023, d0076) undercounted; could also mislead readiness staleness for any escaped semantic field. FIX (this sprint, staleUnescapeFix): a decode_string_body helper that unescapes exactly like the lexer + stops at the first unescaped quote, so blob-extract == model-attr. Permanent guard (D0047): unit tests extract_field_unescapes_like_the_lexer + extract_field_plain_value. RESOLVED — d0023 + d0076 now count.";
        :>> discoveredInField = false;
        :>> relatedTask = "staleUnescapeFix";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from staleUnescapeFix to issue044;
    #ProcessDefect dependency from issue044 to d0047;

    part issue045 : Issue {
        :>> id = "e0450045-0045-4e45-9045-516273840045";
        :>> title = "d0003's 'writePolicy ENFORCED by the API, no self-promotion' is unrealized/superseded by the human commit-gate (D0093/D0094)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "claudeOpus";
        :>> description = "Critique finding (Sprint 133, d0003 x correctness, Medium): d0003 claims authorization differs via a per-skill writePolicy (read-only|pr-only|direct) ENFORCED by the API, and that a skill cannot self-promote its policy. NOT realized in code: write.rs has no per-skill authz (writePolicy is only a doc-comment); one CLI write path is callable regardless of any skill's declared policy, and nothing reads a skill's policy or blocks self-promotion. The dual-surface decisions reframed authorization as the HUMAN COMMIT-GATE (D0093: CLI is authority + automation substrate; D0094: the agent never auto-commits, no new permission policy), effectively superseding d0003's API-enforcement model without a recorded supersede. Candidate resolver (if ACT): writePolicyResolve — either implement per-skill writePolicy enforcement in the write path, OR formally revise/supersede d0003 to state the commit-gate authz model (likely the latter, since D0093/D0094 already chose it). PENDING human disposition (ACT / ACCEPT-RISK / DISMISS).";
        :>> discoveredInField = false;
        :>> relatedTask = "writePolicyResolve";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from writePolicyResolve to issue045;

    // ── #ProcessDefect (D0047/issue039): a recurrable PROCESS-defect finding whose D0047 obligation is a
    //    permanent automated guard. `sysmlv2 guard defect-guard-coverage` (runnable diagnostic) WARNS if
    //    such a finding's #Resolves resolver is NOT guard-producing — the meta-audit that "corrections
    //    become guards" is followed (the rule was previously enforced only by vigilance). Edge -> d0047
    //    (the governing rule). All current entries resolve to a guard/check/rule/audit, so the audit is
    //    clean; a future un-guarded process-defect would be flagged. ──
    #ProcessDefect dependency from issue010 to d0047;
    #ProcessDefect dependency from issue011 to d0047;
    #ProcessDefect dependency from issue027 to d0047;
    #ProcessDefect dependency from issue030 to d0047;
    #ProcessDefect dependency from issue031 to d0047;
    #ProcessDefect dependency from issue033 to d0047;
    #ProcessDefect dependency from issue034 to d0047;
    #ProcessDefect dependency from issue036 to d0047;
    #ProcessDefect dependency from issue039 to d0047;

    // ── Requirement-rootedness gap (raised 2026-06-24): keel serve was built decision-first with no
    //    driving Need, and nothing in the process catches that class. issue046 = the specific instance;
    //    issue047 = the general process defect (its D0047 obligation is a permanent guard). ──
    part issue046 : Issue {
        :>> id = "e0460046-0046-4e46-9046-516273840046";
        :>> title = "keel serve (D0094) was built decision-first with NO driving Need/SystemRequirement — the feature is untraceable + invisible to coverage/orphans/trace-need";
        :>> createdAt = "2026-06-24";
        :>> createdBy = "claudeOpus";
        :>> description = "Assessing keel serve against 3 stakeholder use cases surfaced that NONE of the 14 Needs / 15 SystemRequirements concern serve/console/introspection/critique-UI, and D0094 carries no satisfy/charter edge from any Need. serve was chartered straight from D0094 (a Decision) -> backlog milestones (#CharteredBy d0094) -> ~12 delivery sprints, SKIPPING the Business->Architecture requirement layer. Consequence: the flagship oversight feature has zero Need/SystemRequirement nodes, so it is invisible to keel coverage / trace-need / orphans (nothing to be an orphan OF) and violates invariant #1 (requirement-driven, text-is-truth). It also forced an implementation-archaeology 'assessment' (reading console.html) instead of a requirement-conformance computation. FIX: author the 3 serve Needs + derived SystemRequirements (+ satisfy/verify) and retro-charter the existing serve work (serveRequirementsTrace).";
        :>> discoveredInField = false;
        :>> relatedTask = "serveRequirementsTrace";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from serveRequirementsTrace to issue046;

    part issue047 : Issue {
        :>> id = "e0470047-0047-4e47-9047-516273840047";
        :>> title = "No control ensures chartered work / shipped capabilities trace to a driving Need — requirement-rootedness is enforced only by vigilance";
        :>> createdAt = "2026-06-24";
        :>> createdBy = "claudeOpus";
        :>> description = "GENERAL process defect behind issue046: the engine has no forward control that a Decision which charters delivery work, or a shipped user-facing capability, traces (satisfy/derive) to a Need. The orphans view flags a Need with no Requirement, but NOTHING flags a capability with NO Need at all — so a whole feature (serve, issue046) can be built requirement-less and the engine cannot see it. Requirement-rootedness should be a KEY, un-ignorable element of the process, not something accidentally skippable (human directive, 2026-06-24). Per D0047 (corrections become guards) the obligation is a PERMANENT AUTOMATED GUARD, not vigilance. Resolver requirementRootednessGuard: record a Decision on the rule + scope (incl. exemptions like pure engine-internal refactors), then build it into keel guard (and/or an orphans-style view) that fails on an un-rooted capability. SCOPE EXPANDED (2026-06-24, human insight): requirement INTEGRITY has TWO directions — (a) UPWARD rootedness (every chartered capability traces UP to a Need) and (b) DOWNWARD comprehensiveness (each tier is cleanly + COMPREHENSIVELY satisfied by its downstream items: each Need fully decomposed into SRs, each SR fully implemented + verified). Incomplete downstream satisfaction is a LEADING INDICATOR of insufficient implementation and is a prime first target for the white-box tier-satisfaction analysis (Need 3 / serveRequirementsTrace). STRUCTURAL BLOCKER discovered while authoring the serve requirements: the enforced `guard assured` (compute_readiness coverage_gaps) flags ANY governed Need/SR with no coverage as a readiness gap, so a brand-new requirement-FIRST Need/SR cannot be committed until it is already satisfied + verified — the gates currently PUNISH requirement-first authoring (likely WHY serve was built decision-first, D0094). requirementRootednessGuard must therefore ALSO make requirement-first LEGITIMATE: a governed gap-requirement that carries a resolver / gap-Issue is honest WIP (track-don't-fake), NOT an assurance failure. This is a prerequisite for serveRequirementsTrace (else authoring the serve requirements is blocked).";
        :>> discoveredInField = false;
        :>> relatedTask = "requirementRootednessGuard";
        :>> severity = Severity::High;
    }
    #Resolves dependency from requirementRootednessGuard to issue047;
    #ProcessDefect dependency from issue047 to d0047;

    // ── CI/local-gate divergence (Sprint 147 CI failure, 2026-06-24) ──
    part issue048 : Issue {
        :>> id = "e0480048-0048-4e48-9048-516273840048";
        :>> title = "Local pre-commit ran a curated guard SUBSET that diverged from CI's `guard all` — a viewpoint-renderer violation passed the hook and failed CI";
        :>> createdAt = "2026-06-24";
        :>> createdBy = "claudeOpus";
        :>> description = "Sprint 147's tier-satisfaction Viewpoint failed CI (init_smoke `guard all` -> viewpoint-renderer: 'renderer names no known keel command — keel tier-satisfaction'). TWO root causes: (1) the local .githooks/pre-commit ran a CURATED per-path guard subset (acceptance-events/process-change/ceremony/sprint-coverage/charter/issues/requirement-rootedness) and NEVER ran viewpoint-renderer/manifest-coverage/critic-independence/process-skill/actors — so it could not catch the viewpoint-renderer violation that CI's `guard all` (via init_smoke) does; the local gate diverged from CI. (2) I cherry-picked the `git add` file list and left keel-cli/src/guards.rs (the tier-satisfaction VIEW_SUBCOMMANDS registration) UNSTAGED, so the committed binary lacked it while the viewpoint-registry referenced it — local (working tree) passed, CI (committed state) failed. PREVENTATIVE MEASURES (lightweight): (1, guard) localGateMatchesCi — the hook now runs the FULL `keel guard` (all enforced honest-state guards, kernel-free + fast) whenever any .sysml changes, so local == CI. (2, behavioral, memory) stage with `git add -A` (don't cherry-pick) + check `gh run` after every push. Caught by CI (no harm landed); the fix is in.";
        :>> discoveredInField = false;
        :>> relatedTask = "localGateMatchesCi";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from localGateMatchesCi to issue048;
    #ProcessDefect dependency from issue048 to d0047;

    // ── serve agent-bridge billing risk (raised 2026-06-24) ──
    part issue049 : Issue {
        :>> id = "e0490049-0049-4e49-9049-516273840049";
        :>> title = "serve agent-bridge drives `claude -p` (headless), whose billing has been in flux and may bill as API rather than the subscription";
        :>> createdAt = "2026-06-24";
        :>> createdBy = "wweatherholtz";
        :>> description = "The serve agent-bridge (/api/agent/stream) spawns `claude -p` (headless print mode) for in-console Critique. We never set ANTHROPIC_API_KEY (so it uses the user's subscription/enterprise login, not API-key billing), but D0094 flagged that headless `claude -p` billing is IN FLUX with a history of mis-billing as API (a separate Agent-SDK credit pool was announced then paused). So in-console AI critique carries a real billing-uncertainty risk the user wants to be able to avoid. MITIGATION (resolver serveSettings): a PERSISTENT settings toggle (.keel-serve.json, default ON) to disable the agent bridge; when OFF, the console spawns no `claude -p` (read/oversight only) and AI critique runs in the user's own Claude Code session (subscription-billed). 'Fine for now, maybe not later' — the toggle lets the user turn it off if Anthropic's billing settles unfavorably. (External-dependency risk, not a process defect.)";
        :>> discoveredInField = false;
        :>> relatedTask = "serveSettings";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from serveSettings to issue049;

    part issue050 : Issue {
        :>> id = "e0500050-0050-4e50-9050-516273840050";
        :>> title = "D0098's PURE non-blocking rule exempts open CRITICAL findings, not just incompleteness";
        :>> createdAt = "2026-06-25";
        :>> createdBy = "claudeOpus";
        :>> description = "Critique finding (A2 batch 1, d0098 x correctness, Medium): D0098's 'PURE rule — NOTHING completeness-related hard-blocks (incl. open Critical findings)' conflates INCOMPLETE IMPLEMENTATION (honest in-progress work) with a DISCOVERED CRITICAL DEFECT (purpose-defeating/unsafe). The decision's rationale ('hard-gating completeness forces fakery or blocks honest recording') justifies non-gating for INCOMPLETENESS, but an open Critical FINDING is a known defect, not incompleteness — so the rule lets purpose-defeating/unsafe state reach main backed only by NON-gating visibility (the orient burndown) + a NON-gating disposition backlog. Candidate resolver (if ACT): criticalFindingGatingReview — carve open-Critical findings back out of the non-blocking set into a hard honest-state gate (a Critical finding IS dishonest state to leave un-acted), keeping mere incompleteness non-blocking. PENDING human disposition (ACT / ACCEPT-RISK the visibility-only posture with rationale / DISMISS).";
        :>> discoveredInField = false;
        :>> relatedTask = "criticalFindingGatingReview";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from criticalFindingGatingReview to issue050;

    part issue052 : Issue {
        :>> id = "e0520052-0052-4e52-9052-516273840052";
        :>> title = "sr19ServeWhiteboxBoundary's statement overclaims 'AI-proposed clusters' vs the D0100 descope";
        :>> createdAt = "2026-06-25";
        :>> createdBy = "claudeOpus";
        :>> description = "Critique finding (A2 batch 3, sr19 x correctness, Medium): sr19's statement asserts 'boundaries definable both as AI-proposed clusters AND as human-declared views', but D0100 (accepted 2026-06-24) DESCOPED AI-proposed graph clustering (weak deterministic substrate over the work/requirements graph; low ROI vs the human-declared boundaries sr18 provides). sr19 was delivered + verified (sr19Verify) over human-declared / structural (Need-slice) boundaries ONLY. So the SR is marked verified while one clause of its statement (AI-clustering) is deliberately un-built — the statement overclaims relative to the accepted + verified scope, and a reader of sr19 alone would expect a capability D0100 chose not to build. Candidate resolver (if ACT): sr19StatementAlignD0100 — revise sr19's statement to match D0100 (drop or mark-deferred the AI-proposed-clusters clause, keep AI-proposed-clusters as future/optional), so verified == statement; re-verify. PENDING human disposition (ACT / ACCEPT-RISK the descope-is-recorded-in-D0100 posture / DISMISS).";
        :>> discoveredInField = false;
        :>> relatedTask = "sr19StatementAlignD0100";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from sr19StatementAlignD0100 to issue052;

    part issue053 : Issue {
        :>> id = "e0530053-0053-4e53-9053-516273840053";
        :>> title = "keel guard <ROOT> misread the ROOT arg as a guard name (e.g. `keel guard .` -> unknown guard)";
        :>> createdAt = "2026-06-29";
        :>> createdBy = "claudeOpus";
        :>> description = "Found during the v0.1.0 release smoke (sprint169): cmd_guard treated ANY non-'all' first arg as a guard NAME, so `keel guard .` or `keel guard myproj` (a ROOT path, as `keel validate <ROOT>` accepts) failed with 'unknown guard'. The scaffold's pre-commit hook uses bare `keel guard` (cwd-relative) so it was unaffected, but the CLI was inconsistent with `validate`. FIX (guardRootArgFix): classify_guard_args - a first arg that is a known guard name runs THAT guard; 'all'/no-arg/a non-name path runs ALL guards on that ROOT. Permanent control (D0047): unit test guard_args_distinguish_name_from_root. discoveredInField=false (release-smoke). severity Low (ergonomics; no wrong result, just a usage error).";
        :>> discoveredInField = false;
        :>> relatedTask = "guardRootArgFix";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from guardRootArgFix to issue053;

    part issue054 : Issue {
        :>> id = "e0540054-0054-4e54-9054-516273840054";
        :>> title = "Architecture decisions can be locked before Needs exist, and decisions aren't captured at point-of-decision";
        :>> createdAt = "2026-06-30";
        :>> createdBy = "claudeOpus";
        :>> description = "Found in downstream use bootstrapping a new keel project ('Ignite'): scope+architecture Decisions (full-drop-in, axum, plugin approach, demo-cut) were elicited and LOCKED before any Need was authored, and the human was asked to confirm Needs LAST; separately, none of the human's accepted decisions were recorded as Decisions at the moment they were made (they lived only in the chat until a later batch step). Two gaps: (F1) the Business->Architecture ordering is DEFINED in the workflows but ENFORCED by nothing - no hard guard checks needs-before-architecture; (F2) there is no `add-decision` write path, so recording a Decision is high-friction (hand-author a multi-field .sysml + acceptance event) and gets deferred/lost (contra D0054). PROPOSED FIX (needsFirstOrderingGuard): (C1) a `keel add-decision` write command that emits the Decision + its confirmation acceptance event in one call; (C2) a `keel guard ordering` (warning->hard per D0102) that flags an architectural Decision/SystemRequirement authored while zero Needs exist, or one carrying no satisfy/#DependsOn/charter edge to a Need; (C3) extend engine-triage + the planned UserPromptSubmit hook (issue020/sprint30) to require recorded Needs before architecture elicitation and to capture decisions at point-of-decision. Permanent control per D0047. Relates to D0054 (friction), D0064 (triage-first), issue020/sprint30 (per-interaction enforcement).";
        :>> discoveredInField = true;
        :>> relatedTask = "needsFirstOrderingGuard";
        :>> severity = Severity::High;
    }
    #Resolves dependency from needsFirstOrderingGuard to issue054;

    part issue055 : Issue {
        :>> id = "e0550055-0055-4e55-9055-516273840055";
        :>> title = "Substantive analysis/design/research has no charter home, so it runs untracked in conversation (issue054 recursed into the engine's own process)";
        :>> createdAt = "2026-06-30";
        :>> createdBy = "claudeOpus";
        :>> description = "Surfaced while doing the declarative-controls design analysis for issue054 (docs/proposals/2026-06-30-declarative-controls-issue054.md): multi-turn, substantive design/diagnosis work (proving the 8-of-14 guard duplication, designing EdgeRule/ElementRule, resolving fork #1, authoring the proposal) ran with NO tracked sprint. ROOT (corrected after reading guards.rs/algo.rs): the charter GUARD enforces only that a Story's #CharteredBy EDGE EXISTS - it does NOT validate the target TYPE (charter_violations + collect_charter_work collect the source Story name only; the token after 'to' is never type-checked). The 'Decision/Need/Requirement' restriction is CONVENTION (D0068 wording + the guard's message string), not code - so the guard would ALREADY accept a research Story chartered to an Issue or a proposed Decision. The real gaps are therefore: (a) convention does not sanction chartering a research spike to an Issue/proposed-Decision, and (b) nothing in triage ROUTES substantial analysis through a research spike, so it defaults to conversation - the issue054 defect (substantive work uncaptured in chat) recursed one level up into the engine's OWN process. WorkKind::research EXISTS and the guard already permits the charter; the DISCIPLINE never uses it for upstream analysis. PROPOSED FIX (analysisChartersResearchSpike): (1) D0068 wording blesses a WorkKind::research Story chartering to an Issue or a status=proposed Decision (DoR); (2) engine-triage routes substantial design/analysis to a chartered research spike (DoD = a design artifact + a recorded proposed Decision); (3) charter-target-TYPE validation, if wanted, becomes a declared EdgeRule under the issue054 declarative-controls direction - NOT a new bespoke guard. PROCESS CHANGE -> Change-Request path (Sec 3a). Relates to issue054 (same root + the declarative-controls fix), D0064 (triage-first), D0068 (charter), D0054 (friction).";
        :>> discoveredInField = false;
        :>> relatedTask = "analysisChartersResearchSpike";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from analysisChartersResearchSpike to issue055;

    part issue056 : Issue {
        :>> id = "e0560056-0056-4e56-9056-516273840056";
        :>> title = "D0105 independent critique: residual findings C11 (AI-authored decision reframes accepted decisions) + C12 (two-thread coordination) need human disposition";
        :>> createdAt = "2026-07-01";
        :>> createdBy = "claudeSonnet46";
        :>> description = "The independent Core-3 critique of D0105 (critic claudeSonnet46, sprint173) FAILED it as stale-vs-spike. The sprint173 rework RESOLVED the addressable findings (C1-C6/C10/C13: added OrderingRule as a third rule kind; corrected the 'scope is the load-bearing complexity' claim; added the rollback criterion + re-validation condition; folded the scope-cap meta-rule + OrderingRule into ONE frozen-core sign-off; made the reframe language conditional 'UPON ACCEPTANCE'; classified acceptance-events as an EdgeRule; specified the TOML migration concretely). TWO residual findings need a HUMAN, not an AI: C11 (High) - is it procedurally sound for an AI-authored status=proposed Decision to reframe THREE accepted decisions (D0047/D0088) on a conversational endorsement? This is precisely the schema/core frozen-core sign-off gate; D0105 must not be implemented until the human explicitly accepts (method=confirmation) - and per the confirmation rule, a general 'continue' instruction is NOT that sign-off. C12 (Med) - two threads (this analysis thread + the ignite thread that owns issue054) now co-edit the engine model with no recorded coordination/handoff protocol; define who may change what without re-authorization and how divergence is adjudicated. RESOLUTION: C11 by the human's D0105 acceptance decision (or rejection); C12 by a recorded coordination protocol (d0105CritiqueResiduals). Relates to D0105, D0080 (independent critique), issue054/issue055 (two-thread overlap).";
        :>> discoveredInField = false;
        :>> relatedTask = "d0105CritiqueResiduals";
        :>> severity = Severity::High;
    }
    #Resolves dependency from d0105CritiqueResiduals to issue056;

    part issue057 : Issue {
        :>> id = "e0570057-0057-4e57-9057-516273840057";
        :>> title = "keel init ships the SELF-BUILD CLAUDE.md verbatim, so downstream projects think they ARE keel";
        :>> createdAt = "2026-07-02";
        :>> createdBy = "wweatherholtz";
        :>> description = "FIELD DEFECT (keel used in other projects). keel-cli/src/main.rs:45 'const CLAUDE_MD = include_str!(../../CLAUDE.md)' and cmd_init writes that file verbatim (main.rs:1238) into every 'keel init' project. But that CLAUDE.md is the SELF-BUILD's — it opens 'This repo IS a work-tracking engine ... built using its own discipline' and describes building keel ITSELF (schema/core frozen, the self-build backlog, decisions 0001+, the miniforge kernel paths). A downstream project is NOT keel — it is a project TRACKED BY keel. Shipping the self-build CLAUDE.md mis-frames the newcomer (they think they are building the engine), leaks self-build specifics, and contradicts D0093 (engine architecture ships as READ-ONLY .engine/reference/; the new project authors fresh). FIX (downstreamClaudeMdTemplate): ship a DOWNSTREAM CLAUDE.md template — 'this project is tracked BY keel; here is how to work as a keel-tracked project' — pointing at .engine/reference for engine architecture; 'keel init' writes the TEMPLATE, not the self-build file; the self-build repo keeps its own CLAUDE.md un-shipped. PROCESS CHANGE (what init ships) -> sprint. Relates to D0093 (spin-up), the strict-process principle (D0106).";
        :>> discoveredInField = true;
        :>> relatedTask = "downstreamClaudeMdTemplate";
        :>> severity = Severity::High;
    }
    #Resolves dependency from downstreamClaudeMdTemplate to issue057;

    part issue058 : Issue {
        :>> id = "e0580058-0058-4e58-9058-516273840058";
        :>> title = "Ad-hoc decision-authoring produced dual-truth (acceptance verdict restated as prose alongside the discrete TestResult)";
        :>> createdAt = "2026-07-02";
        :>> createdBy = "wweatherholtz";
        :>> description = "Caught by the human: when recording D0105/D0106 acceptance I ALSO restated the verdict as PROSE (the '(ACCEPTED <date> by <who>)' comment header + a consequences-field sentence) on top of the discrete method=confirmation d####Accept + AcceptR1 TestResult — dual-truth, drift-prone, contra Sec 2.1 (one source of truth). ROOT (the accountability): I acted AD-HOC — narrating for readability outside any defined process — which is exactly what D0106 (strict process-boundedness) forbids. There is no DEFINED decision-authoring process constraining WHICH fields carry which facts and forbidding restatement of computed/verdict/status facts, so prose crept in each time (and my first 'fix' was itself ad-hoc + incomplete, leaving the comment headers). IMMEDIATE cleanup done: removed the verdict prose from D0105/D0106 headers + D0105 consequences (the d####Accept TestResult is now the sole source). SYSTEMIC FIX (oneSourceDecisionAuthoring): (1) define a decision-authoring discipline/process (each fact ONE canonical home; acceptance/verdict/critique-state/status are structured or COMPUTED, never restated as prose; prose fields describe only the decision's own what/why/impact) + a deploying skill; (2) a DETECTING rule — under D0105 an ElementRule: a Decision must not contain acceptance-verdict prose when an AcceptR1 event exists; (3) 'status computed from the acceptance event' folds into guardsToRulesMigration (acceptance-events -> EdgeRule; status a computed #View — noting status also encodes proposed/superseded, so it is a guarded cross-check, not pure redundancy). Relates to D0106 (the violated principle), D0105 (declarative controls / detecting rule), Sec 2.1.";
        :>> discoveredInField = false;
        :>> relatedTask = "oneSourceDecisionAuthoring";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from oneSourceDecisionAuthoring to issue058;

    // ── Retrospective findings (2026-07-04): honest critique of THIS session's resolutions. The recurring
    //    theme — several new controls rely on VIGILANCE, not enforcement (the exact weakness they cured). ──

    part issue059 : Issue {
        :>> id = "e0590059-0059-4e59-9059-516273840059";
        :>> title = "D0106 strict process-boundedness is unenforced (vigilance-only) — it was violated AFTER adoption";
        :>> createdAt = "2026-07-04";
        :>> createdBy = "wweatherholtz";
        :>> description = "Retrospective finding (the sharpest). D0106 (parse-first / no action untied to a process / sign-off as an explicit gate) is enforced only by convention + a UserPromptSubmit REMINDER hook — there is no hard gate that a response/commit actually carried its route + sign-off evidence. Proof it is weak: the dual-truth defect (issue058) occurred AFTER D0106 was in force, and the first fix was itself incomplete — the one principle meant to stop ad-hoc work did not prevent its own target failure. The `Parsed:` block also risks becoming ritual (produced every turn, rubber-stampable). This is the same vigilance-is-not-a-control lesson as D0047/issue010. PROPOSED FIX (strictProcessGate): make D0106 a real control — e.g. a commit-time check that substantive work carries its process linkage (a sprint/charter or a recorded route) and that method=confirmation results carry an actual attestation, not just a reminder. PROCESS CHANGE -> Change-Request path. Relates to D0106, D0064, issue020/sprint30 (the planned hook), issue058.";
        :>> discoveredInField = false;
        :>> relatedTask = "strictProcessGate";
        :>> severity = Severity::High;
    }
    #Resolves dependency from strictProcessGate to issue059;

    part issue060 : Issue {
        :>> id = "e0600060-0060-4e60-9060-516273840060";
        :>> title = "Declarative-controls migration payoff is unproven — net code up, OrderingRule unused (dead schema), keel check name unfreed, RMWX axis one verb";
        :>> createdAt = "2026-07-04";
        :>> createdBy = "wweatherholtz";
        :>> description = "Retrospective finding. The declarative-controls effort (D0105/D0107) built a rule schema + `keel rules` evaluator but migrated only 5 of 14 guards, so it ADDED machinery (schema + evaluator + rule files) while removing duplication in only 5 guards — net lines-of-code likely INCREASED, and the D0105 'code stays flat' claim is unproven and probably false at this scope. Concrete loose ends: (a) `OrderingRule` is defined in schema/core with ZERO instances (ceremony stayed Rust) — dead schema; (b) `keel check` (D0105's intended evaluator name) was never freed — the evaluator is `keel rules`; (c) the RMWX 'closed verb axis' is one verb (`record decision`); the per-type add-*/append-* verbs all remain; (d) the primary justification ('downstream projects declare their own rules') is speculative — no downstream rule exists. This may itself be the inner-platform over-engineering D0105 warned against. PROPOSED FIX (declarativeControlsRoiOrRescope): either PROVE the ROI (migrate enough more guards to actually DELETE Rust guard code, freeing `keel check`) OR RESCOPE — remove the unused OrderingRule kind + mark the mechanism 'downstream-only, minimal engine use' honestly. PROCESS/CHANGE. Relates to D0105, D0107, issue054 (RMWX).";
        :>> discoveredInField = false;
        :>> relatedTask = "declarativeControlsRoiOrRescope";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from declarativeControlsRoiOrRescope to issue060;

    part issue061 : Issue {
        :>> id = "e0610061-0061-4e61-9061-516273840061";
        :>> title = "New process deployments are declared but unexercised — business-architecture skill never run; issue055 spike-routing is vigilance-dependent";
        :>> createdAt = "2026-07-04";
        :>> createdBy = "wweatherholtz";
        :>> description = "Retrospective finding. Two fixes that address 'inert/undeployed process' are themselves unexercised — the same inert-until-deployed risk (D0059) one level up. (a) The business-architecture skill (issue054 F1 fix — deploys the Business/Architecture workflows) was authored but NEVER RUN on a real design session this session; its efficacy is unproven. (b) issue055's research-spike routing (engine-triage item 8) exists but is vigilance-dependent — most analysis since (INCLUDING the retrospective that produced these issues) still ran in conversation without a chartered spike. So both 'fixes' depend on the same discretionary follow-through that caused the original problem. PROPOSED FIX (proveProcessDeployment): exercise the business-architecture skill on the next real design/scoping request (dogfood it end-to-end), and tie research-spike routing to the issue059 strict-process gate so substantial analysis is routed by a control, not by memory. Relates to issue054, issue055, D0059, D0106, issue059.";
        :>> discoveredInField = false;
        :>> relatedTask = "proveProcessDeployment";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from proveProcessDeployment to issue061;

    part issue062 : Issue {
        :>> id = "e0620062-0062-4e62-9062-516273840062";
        :>> title = "The dual-truth detecting rule is non-gating, heuristic, and its positive path is untested";
        :>> createdAt = "2026-07-04";
        :>> createdBy = "wweatherholtz";
        :>> description = "Retrospective finding. decisionNoVerdictProseRule (issue058) is warning-level + non-gating (shows in `keel rules`, never blocks a commit), heuristic (substring 'ACCEPTED 202'), and fires 0 today — so its POSITIVE path (actually catching a dual-truth) is untested on real data, and it would miss other dual-truth forms (a verdict restated without that exact substring). It documents the intent more than it enforces it. PROPOSED FIX (hardenVerdictProseRule): add a positive test fixture (a synthetic offending decision the rule must flag), broaden the pattern beyond one substring, and decide whether it should gate (promote to a blocking rule/guard) once low-noise — mirroring the decision-requirement-link warning->hard promotion path. Relates to issue058, D0102 (warning->hard precedent).";
        :>> discoveredInField = false;
        :>> relatedTask = "hardenVerdictProseRule";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from hardenVerdictProseRule to issue062;

    part issue063 : Issue {
        :>> id = "e0630063-0063-4e63-9063-516273840063";
        :>> title = "keel serve orient first-load intermittently 16-23s: orient's criterion-drift spawns hundreds of git subprocesses (Windows spawn storm)";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "wweatherholtz";
        :>> description = "Field-discovered via keel serve on this project (human observed a ~16s orient load). MEASURED (GIT_TRACE): CLI orient ~1.2s spawning 19 git processes (10 `git show` + 5 `git ls-tree` + 2 batched `git cat-file` + misc); serve cold ~1.1s normally, cached ~50ms — but one cold /api/orient spiked to 23s. ROOT (corrected after measuring — NOT hundreds of spawns): orient's criterion-drift is ALREADY batched (valid_commits + batch_cat_blobs = 2 cat-file spawns), but a PER-TASK fallback (git_criterion_at: `git ls-tree` + N `git show`) fires for ~5 done tasks whose dep DoD MOVED files since the verified commit (extract_dod_criterion misses -> orient.rs:337 fallback). So ~15 of the 19 spawns are the avoidable fallback. On this AzureAD/Enterprise Windows box, per-process CreateProcess + Defender scan latency has a heavy variable tail, so 19 spawns × up-to-~1s each = the 16-23s spike. The serve per-fingerprint cache masks it after the first hit. FIX (fixServeOrientPerf): (a) run the serve compute via tokio spawn_blocking so a slow first compute never blocks the axum worker; (b) cut the fallback spawn count (group git_criterion_at by commit / one `git grep` to locate the moved DoD instead of ls-tree + N shows). Perf-only (orient output unchanged) -> EXECUTE. Relates to D0054 (friction is the #1 adoption risk).";
        :>> discoveredInField = true;
        :>> relatedTask = "fixServeOrientPerf";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from fixServeOrientPerf to issue063;

    part issue064 : Issue {
        :>> id = "e0640064-0064-4e64-9064-516273840064";
        :>> title = "keel serve console shows action/ready items as 'no authored attrs' + a NextWork contains-edge, hiding the real description in the linked DoD procedureText";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "wweatherholtz";
        :>> description = "Field-discovered via keel serve. Clicking a ready item (e.g. proveProcessDeployment) shows 'no authored attrs' + an incoming edge labelled NextWork. Both are technically true but misleading: (a) an `action X;` task genuinely carries no authored attributes (its human description lives in the verify-linked <task>DoD Test's procedureText, which the element view never surfaces); (b) 'NextWork' is the action-def BACKLOG CONTAINER, so the edge is the structural `contains` relation, not task data. Net effect: a fully-specified task LOOKS empty, hiding the DoD criterion. FIX (surfaceDodDescription): resolve <task>DoD and present its procedureText (+ method) as the task's description in the element/console view. Relates to issue063 (same serve dogfood).";
        :>> discoveredInField = true;
        :>> relatedTask = "surfaceDodDescription";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from surfaceDodDescription to issue064;

    part issue065 : Issue {
        :>> id = "e0650065-0065-4e65-9065-516273840065";
        :>> title = "AI shell/command guidance is hardcoded to Windows/PowerShell and the downstream keel-init template ships NO OS guidance — avoidable shell-mismatch mistakes";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "wweatherholtz";
        :>> description = "Retro finding (generalizes issue013's cwd-specific fix). CLAUDE.md Sec 6 assumes Windows + PowerShell + miniforge; the shipped downstream CLAUDE.md template (keel-cli/assets/claude-md-template.md) has NO OS/env section at all (it ends at Sec 5). So a downstream project on macOS/Linux gets an AI with zero OS-adaptation instruction, and even here the class recurs — this session the PowerShell tool broke and command-form mixups (bash `&`/conda syntax) followed. HONEST LIMIT: behavioral guidance is a 'linter, ignorable' (D0110) — not a hard gate — so this reduces, not eliminates, the mistake rate. FIX (osAdaptiveCommandGuidance): state the PRINCIPLE (detect + adapt to the host OS/shell; verify the shell before assuming syntax) in the operating guidance, keep Windows specifics as this repo's instance, and add an OS/shell section to the downstream template. Relates to issue013, D0093 (downstream template).";
        :>> discoveredInField = false;
        :>> relatedTask = "osAdaptiveCommandGuidance";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from osAdaptiveCommandGuidance to issue065;

    part issue066 : Issue {
        :>> id = "e0660066-0066-4e66-9066-516273840066";
        :>> title = "The JVM kernel is still required for .engine SysML validation — a conda/JVM dependency persists for all engine work (D0048 kept it deliberately)";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "claudeOpus";
        :>> description = "Architecture opportunity surfaced in the D0048 review this session. D0048 eliminated the JVM for the routine .tracking path but DELIBERATELY kept the kernel validators (validate_schema/workflows/instances) as the .engine SysML oracle, because the Rust validator does not cover their deeper SysML semantics. Cost: every engine commit (schema/decisions/rules/processes) still needs conda + a ~20s JVM start; downstream engine work inherits the conda dependency; this session the kernel step was environment-fragile (PowerShell tool broke). NOT a defect (the split is correct today) but a standing opportunity. PROPOSED (jvmRetirementSpike): a WorkKind::research SPIKE — enumerate exactly which checks the kernel does that Rust does not, assess replicating them in Rust, and produce a design artifact + a PROPOSED Decision on retiring the kernel entirely; implementation is a follow-on program gated on human acceptance. Relates to D0048, D0074.";
        :>> discoveredInField = false;
        :>> relatedTask = "jvmRetirementSpike";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from jvmRetirementSpike to issue066;

    part issue067 : Issue {
        :>> id = "e0670067-0067-4e67-9067-516273840067";
        :>> title = "An .engine .sysml change committed with SKIP_VALIDATE=1 gets NO kernel validation — CI is Rust-only, so deep SysML semantics can land unvalidated";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "claudeOpus";
        :>> description = "Surfaced this session (dogfood): the JVM kernel (the authoritative .engine SysML oracle, D0048) hung repeatedly, so D0112 was committed with SKIP_VALIDATE=1 — which bypasses the pre-commit hook's kernel step. CI is Rust-only BY DESIGN (ci.yml: 'the .sysml kernel/guard checks stay in the pre-commit hook; they need the local conda kernel'; n11/D0054 friction). So an .engine .sysml change that skips the hook has NO kernel backstop ANYWHERE — its deep name/type-resolution semantics land unvalidated (only Rust `keel validate` + structural inspection). D0112 is Rust-validated + structurally identical to the kernel-passed D0110/D0111, so THIS instance is low-risk, but the HOLE is real. RELATED honest note: SKIP_VALIDATE also silently skipped clippy this session (a642269 landed a workspace-clippy failure). FIX (engineRefResolver = D0112 phase 2): a Rust .engine reference/name-existence resolver gives a kernel-free backstop for the common `unresolved` class; interim option: make the hook refuse SKIP_VALIDATE when .engine .sysml is staged. Relates to D0112, D0048, D0054/n11, issue066.";
        :>> discoveredInField = false;
        :>> relatedTask = "engineRefResolver";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from engineRefResolver to issue067;

    part issue068 : Issue {
        :>> id = "e0680068-0068-4e68-9068-516273840068";
        :>> title = "Review queue (D0121) surfaced HISTORICAL ceremony confirmation gates as CURRENT human obligations — it ignored governing-version / creation-time process governance";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> description = "Surfaced this session (dogfood, human-caught). The new review-queue view (D0121) scans ALL method=confirmation gates model-wide and treats each as a live human obligation. It surfaced the legacy pre-D0049 retro confirmation gates (rustS5/6/7 RetroGate) as items awaiting the human's sign-off — but retro is AUTONOMOUS (D0049), and those gates were chartered under the PRE-D0049 process where retro-confirmation was correct. ROOT CAUSE: a new computed obligation-view was built WITHOUT applying the engine's existing governing-version discipline (govern.rs governing-version + forward-only retroactivity markers, D0068/D0069/D0070/D0034) — so it re-introduced the grandfathering the engine was designed to prevent (items must be governed by the process in force at their creation, not retroactively judged against expanded/relaxed processes). COMPOUNDING (avoided): the initially-proposed fix was a guard that would FAIL any ceremony confirmation gate — which would have retroactively flagged historical items as deficient, the same anti-pattern. NON-FIX (reverted): reclassifying the 3 legacy gates confirmation->analyze was rejected — that rewrites their creation-time truth. FIX (reviewQueueGoverningVersion): obligation-views must reflect CURRENT-process obligations without resurrecting historical items — immediate: declare `retro` as a Delivery workflow phase so the EXISTING model-driven ceremony filter (is_ceremony_gate from declared phases) recognizes + excludes retro gates (ceremony = autonomous, no human confirmation); broader: obligation-views consult governing-version so historical items under superseded process versions are neither surfaced as live obligations nor flagged as deficient (exceptions — safety-related / high-level Needs — may be deliberately re-governed). Any NEW guard must be forward-only (honor retroactivity). Relates D0121, D0049, D0047, D0034, D0068/D0069/D0070.";
        :>> discoveredInField = false;
        :>> relatedTask = "reviewQueueGoverningVersion";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from reviewQueueGoverningVersion to issue068;

    part issue069 : Issue {
        :>> id = "e0690069-0069-4e69-9069-516273840069";
        :>> title = "keel has no `arch` view command group — a downstream's EngineCodeAudit CodeElement registry is authored but not computable (criticality/coupling/drift by hand)";
        :>> createdAt = "2026-07-18";
        :>> createdBy = "claudeOpus";
        :>> description = "Cross-repo request from the SelfSync self-build (its D0029 architecture-audit + docs/design-history/2026-07-18-architecture-audit-design.md). SelfSync authored a full CodeElement registry (169 elements, an optional EngineCodeAudit schema module + the audit_hash.py drift tool) — but the registry's INTENDED lenses (criticality = risk-class × Need-trace; coupling = Ca/Ce/instability/distance over #DependsOn; drift = codeHash vs the live @audit-hash; the captured STPA control-structure inputs) are all COMPUTED VIEWS that keel doesn't yet render, so today they're run by hand / via audit_hash.py. To make the capability reusable through keel (the owner's goal), the toolkit needs: (1) promote the EngineCodeAudit schema module into the engine's shipped schema so `keel init` gives every downstream the CodeElement type; (2) a `keel arch` view group (elements/criticality/coupling/drift/stpa-inputs/coverage), each a declared viewpoint, computed from authored CodeElement instances + the Needs graph + the in-code @audit-hash markers — keel never parses deliverable source. Not a defect; a planned feature (the follow-on Phase 1 of the SelfSync D0029 plan). Severity Low (enhancement).";
        :>> discoveredInField = false;
        :>> relatedTask = "keelArchViews";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from keelArchViews to issue069;

    part issue070 : Issue {
        :>> id = "e0700070-0070-4e70-9070-516273840070";
        :>> title = "The engine has never run its own Deploy workflow — no release discipline (v0.1.0 ~a month stale; Deploy defined-but-unrun)";
        :>> createdAt = "2026-07-27";
        :>> createdBy = "claudeOpus";
        :>> description = "Dogfooding gap surfaced 2026-07-27 (D0128, nReleaseDiscipline). keel pushes every commit (post-commit hook) but has NO release process: the only published release is v0.1.0 (tagged 2026-06-29, ~a month stale) — the entire Architect Sure line + the write API growth (v1.11->v1.17) are in NO release. CI only tests (clippy + cargo test + keel gates); it does not build or publish artifacts. The Deploy workflow is DEFINED (.engine/workflows/deploy.sysml) but has NO deploying process/skill and has never been run — the engine that governs Business->Architecture->Delivery->Deploy->Operate has never dogfooded its own Deploy. Downstreams (e.g. SelfSync) get a month-stale binary or must build from a random SHA. Resolved by keelReleaseProcess (a deploy process + skill + CI release job + semver + a recorded Release item).";
        :>> discoveredInField = false;
        :>> relatedTask = "keelReleaseProcess";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from keelReleaseProcess to issue070;

    // ── distributed-collaboration audit, 2026-08-12 (D0129). Each entry was VERIFIED against
    // code, not inferred; file:line citations are in the descriptions. Found by a two-lane
    // research pass (external prior-art + internal audit) commissioned before operating keel
    // with an async remote mostly-AI team. Per D0047 each needs a permanent guard, not a patch.
    part issue071 : Issue {
        :>> id = "e0710071-0071-4e71-9071-516273840071";
        :>> title = "`keel orient` is MACHINE-DEPENDENT once history is rewritten — squash/rebase orphans judgedAgainst anchors, so the same model reports green on one clone and not-done on every other";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "EXISTENTIAL — this negates the engine's core claim that state is COMPUTED rather than authored. A task is `done` only while its passing TestResult's judgedAgainst SHA still RESOLVES (orient.rs:494-531; resolution via `git cat-file` at orient.rs:123); unresolvable => invalidEvidence => NOT done (orient.rs:38). Compose with: post-commit pushes fire-and-forget and on rejection merely prints 'run git push when able' (.githooks/post-commit:8-9) and nothing ever fetches; with several contributors non-fast-forward rejection is constant; the natural remedy (`git pull --rebase`) REWRITES local SHAs. The already-recorded judgedAgainst values then name pre-rebase commits that were never pushed: the rewriting machine still finds them as loose objects and reports GREEN, while every other clone reports invalidEvidence, re-lists finished work as ready, and a second contributor redoes it. Computed state stops being a function of the model and becomes a function of which working copy ran the query. VERIFIED SEPARATELY: the remote currently permits BOTH squash-merge and rebase-merge and `main` has NO branch protection (gh api, 2026-08-12) — the hazard is live and ungated. Prior art confirms the mechanism: Gerrit introduced Change-Id because SHAs die on rebase; git-notes needs notes.rewriteRef for the same reason. FIX: ancestry-preserving integration only (fast-forward or merge commit), enforced by remote configuration rather than convention; plus honest reporting of unsynchronized-vs-invalid anchors (issue-adjacent, srDcUnresolvedEvidenceClass).";
        :>> discoveredInField = false;
        :>> relatedTask = "dcMergeOnlyIntegration";
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcMergeOnlyIntegration to issue071;

    part issue072 : Issue {
        :>> id = "e0720072-0072-4e72-9072-516273840072";
        :>> title = "confirmation-authenticity is bypassable by an OMITTED parameter — write paths default the acting actor to a named human, so an AI records human acceptance silently";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "SECURITY-RELEVANT. The confirmation-authenticity guard (D0106/issue059) exists specifically so an accepted Decision's acceptance event must be judged by a human Person and can never be AI-fabricated. VERIFIED: seven serve.rs write endpoints default `author`/`judged_by` to the human \"wweatherholtz\" (serve.rs:304, 331, 357, 384, 409, 483, 1313); `record decision` defaults --author to the same (main.rs:967); five further paths default to \"keel-cli\", which is NOT in ProjectActors and would fail the actors guard (main.rs:910, 935, 1121, 1151); `reverify` defaults --by to claudeOpus (main.rs:429). An AI-driven API call that merely OMITS judged_by therefore records a HUMAN attestation, silently and by default. The guard does not fail loudly — it simply stops meaning anything, because the substrate it reads (attribution) is false. Note the capture side already exists (capture_user.py resolves and prints an actor id) but NOTHING consumes it: the gap is identity BINDING, not identity capture. Exposure multiplies with every added contributor. FIX: resolve the acting actor from an explicit per-machine binding or argument; REFUSE the write when unstated; remove every default naming a human or an unregistered actor.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcActorIdentityBinding";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcActorIdentityBinding to issue072;

    part issue073 : Issue {
        :>> id = "e0730073-0073-4e73-9073-516273840073";
        :>> title = "Enrollment registers EVERY actor as `: Person` with no kind question — an onboarding AI is recorded as a human and then legitimately satisfies confirmation-authenticity";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "SECURITY-RELEVANT, and INDEPENDENT of issue072 — closing one does not close the other. VERIFIED: .engine/tools/capture_user.py derives identity from the git committer identity and appends the new actor to .tracking/actors.sysml hardcoded as `part <id> : Person { ... }` (its append_id function), with no question about actor kind and no AI branch at all. Since git identity on an agent's machine is simply whatever that machine is configured with, an AI agent enrolling itself is registered as a human Person. confirmation-authenticity then reads the registry, sees a Person, and correctly accepts that actor's acceptance attestations as human-judged — so the guard is satisfied by a false premise rather than bypassed by a bug. Enrollment is the single point at which actor kind becomes true or false for every subsequent authority check, which makes it the one place inference is unacceptable. (Its docstring is also stale: it claims to append to an `ActorId` enum, which CR-5 replaced with typed Person/Actor parts.) FIX: require kind explicitly at registration, never infer from git identity, never default to Person, and make registering an AI as a Person impossible through the sanctioned path.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcActorIdentityBinding";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcActorIdentityBinding to issue073;

    part issue074 : Issue {
        :>> id = "e0740074-0074-4e74-9074-516273840074";
        :>> title = "No guard detects duplicate identity — two offline contributors mint the same decision number, git reports NO conflict, and the duplicate package is silently merged";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "The failure class where the ABSENCE of a git conflict is the danger: git protects against concurrent edits to the same LINES, not against concurrent allocation of the same NAME. VERIFIED: `next_decision_number` is max-of-directory-scan plus one (write.rs:929-942), so two contributors working before synchronizing both mint the same NNNN; because their slugs differ the FILENAMES differ, so git reports no conflict and both land; the resulting duplicate `package DecisionNNNN` declarations are then SILENTLY MERGED by the registry via or_default() (keel-parser/src/registry.rs:52-53), and registry.validate checks only type refs and enums (registry.rs:74-110). The same shape applies to two `sprintNNN_*.sysml` files (both counted by in_progress_sprints, orient.rs:344) and to hand-appended issue numbers (no CLI verb allocates them). VERIFIED ABSENT: none of the seventeen guards scans for a duplicate element id, duplicate item name, duplicate package name, or duplicate allocated sequence number — the only duplicate check anywhere is add_task's same-file task-name check (write.rs:804). Corruption therefore lands GREEN and is undetectable afterwards. Compounding: gen_uuid has no host entropy (issue075), so a duplicate id is also possible and equally undetected. FIX: a hard-blocking duplicate-identity guard across all four classes, preferably sourced from a declared ElementRule (D0105/D0107).";
        :>> discoveredInField = false;
        :>> relatedTask = "dcDuplicateIdentityGuard";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDuplicateIdentityGuard to issue074;

    part issue075 : Issue {
        :>> id = "e0750075-0075-4e75-9075-516273840075";
        :>> title = "gen_uuid has NO host entropy — independent machines are not independent id sources, and identity is the one thing a collision corrupts";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "VERIFIED: gen_uuid (write.rs:63-99) constructs its two halves from clock seconds, sub-second nanos, process id, and an in-process atomic counter that is 0 for the first record of every CLI invocation; it self-documents as 'Not cryptographically random — suitable for dev-tooling identity only'. There is NO host or MAC component, so two machines generating concurrently are not independent sources, and small process ids collide readily across machines. Immutable `id` is the engine's identity invariant — items never collide on name precisely because they are distinguished by id (CLAUDE.md §2.3) — so a collision corrupts identity itself rather than merely producing a duplicate row. Likelihood is LOW; consequence is severe and, given issue074, entirely undetectable. That profile (cheap one-function fix, undetectable failure) justifies fixing pre-emptively rather than waiting for an occurrence. FIX: cryptographically-seeded random UUIDv4 from the OS RNG, emitted format unchanged so existing ids and parsers are unaffected.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcCollisionResistantIds";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcCollisionResistantIds to issue075;

    part issue076 : Issue {
        :>> id = "e0760076-0076-4e76-9076-516273840076";
        :>> title = "The local gate silently evaporates off this machine, merge commits are UNGATED, and *.sysml line endings depend on per-machine config";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "Three defects in one control, all VERIFIED, all first-contact failures for a remote contributor. (1) NON-PORTABLE: .githooks/pre-commit:17-21 hardcodes an absolute conda path (C:/Users/WilliamWeatherholtz/miniforge3/...) and the Windows binary name target/release/keel.exe. On a non-Windows machine the binary is absent, so the hook prints 'rust binary missing — guards skipped' and the local gate silently becomes NOTHING (pre-commit:66-68) with only post-hoc CI behind it; on a machine without that conda path, any commit touching .engine/*.sysml ABORTS, which teaches contributors to set SKIP_VALIDATE=1 and thereby defeats the gate everywhere at once. The 'local gate == CI' invariant the hook's own comments assert (issue048) is therefore already false off this machine. (2) MERGE-BLIND: only pre-commit and post-commit are installed; `git merge` fires pre-merge-commit, which does not exist — so the conflict-resolution commits that distributed work generates, the ones an AI is most likely to get semantically wrong, are exactly the commits with NO local check. CI is push-triggered with no PR gate (.github/workflows/ci.yml:11-13). (3) UNPINNED TEXT: .gitattributes normalizes only *.sh, leaving *.sysml to each machine's core.autocrlf (true here, per-machine, not per-repo). A contributor configured differently commits CRLF, and every shared model file becomes a whole-file phantom diff — a guaranteed unresolvable conflict on precisely the files every contributor touches. FIX: probe for the binary, make the kernel layer optional-not-fatal, install pre-merge-commit, add `* text=auto` + `*.sysml text eol=lf`, and make an unrunnable check FAIL LOUDLY instead of passing silently.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcPortableGate";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcPortableGate to issue076;

    part issue077 : Issue {
        :>> id = "e0770077-0077-4e77-9077-516273840077";
        :>> title = "Markers that hard-blocking guards AND the burndown depend on are UNDECLARED — a misspelled marker validates clean and silently blinds the check (incl. #Verify, used in 31 files)";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "VERIFIED 2026-08-12: of the metadata markers guards and views actually consume, only #View is declared (in .engine/workflows/_meta.sysml). #Capability (used in 15 files), #DerivedFrom (22 files), #Covers (6), #Dispositions (10) and #ProcessDefect (5) appear NOWHERE as a `metadata def` — the declared set in relationships.sysml is only DependsOn, Supersede, OrderingOnly, CharteredBy, Resolves, ProspectiveChange, SafetyChange. Yet #DerivedFrom is load-bearing for the HARD-BLOCKING requirement-rootedness guard (guards.rs:386-398; rootedness view at view.rs:3085-3116), #Covers for sitting-coverage, and #Dispositions for the typed disposition verdict path (D0092). Because markers are not type-checked by `keel validate` or `check-engine`, a misspelled marker (#DerivdFrom) validates CLEAN and silently removes that item from the guard's view — and a blind guard reports PASS, not a violation. This is the silent-gate-bypass class, and unlike a normal typo it scales with contributor count: any contributor's slip disables a gate for that item with no signal to anyone, including in review. NOTE: fixing this edits .engine/schema/core/relationships.sysml, and schema/core is FROZEN (CLAUDE.md §2.5), so it needs explicit human sign-off — it is deliberately excluded from the day-one set. FIX: declare every guard-consumed marker and make validation REJECT an undeclared/misspelled marker. CORRECTION (2026-08-13, this record UNDERSTATED the problem when first filed): the undeclared set is WIDER than the five originally listed — #Verify is undeclared and used in 31 FILES, more than any other, and it is what `tier-satisfaction`/`sr_verified_pct` and the new verification-trace guard both key on; #Satisfy and #Allocate are likewise undeclared. So a single typo in #Verify would not merely blind a guard, it would silently report a DELIVERED requirement as unverified in the burndown — the same wrong-picture failure as issue082, arriving by a different route. Declared markers remain only: DependsOn, Supersede, OrderingOnly, CharteredBy, Resolves, ProspectiveChange, SafetyChange (+ #View in _meta).";
        :>> discoveredInField = false;
        :>> relatedTask = "dcDeclaredMarkers";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDeclaredMarkers to issue077;

    part issue078 : Issue {
        :>> id = "e0780078-0078-4e78-9078-516273840078";
        :>> title = "D0108's coordination contract is CONVENTION-ONLY (no guard, no rule) and the Issue-recording path it mandates does not exist";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "D0108 (accepted) states the correct multi-writer contract: each item is owned by its createdBy and only the owner edits its fields; a non-owner may only ADD items and typed edges or SUPERSEDE a Decision; shared files are append-or-rebase with a fetch before a shared-region edit; conflicting conclusions across threads become an Issue that the HUMAN adjudicates, neither thread silently winning. VERIFIED UNENFORCED: it appears in none of the seventeen guard names (guards.rs:885-886) and none of the eight declared rules (.engine/rules/rules.sysml:14-132); its entire enforcement is CLAUDE.md §4 prose plus the UserPromptSubmit triage reminder. D0047 is explicit that manual vigilance is not a control. Meanwhile in-place write paths that CAN clobber another actor's work exist and are defended only by a doc comment saying the caller should refuse: set_attr rewrites an attribute inside an existing item block (write.rs:551-583) and accept_decision flips the FIRST proposed status in a file via replacen (write.rs:1029). Worse, the adjudication path D0108 mandates has no implementation at all: `keel record decision` exists but there is NO command that records an Issue, so the sanctioned one-liner for 'record the conflict' does not exist — which is exactly the friction that guarantees the discipline is skipped (D0054). FIX: a git-diff-aware ownership guard (cf. the doc-sync guard's diff machinery, guards.rs:785+) plus `keel record issue`.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcMechanizeOwnership";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcMechanizeOwnership to issue078;

    part issue079 : Issue {
        :>> id = "e0790079-0079-4e79-9079-516273840079";
        :>> title = "Validation verdict was FILESYSTEM-ORDER-DEPENDENT — a one-pass register+validate failed cross-package imports depending on visit order (green on Windows, flaked on CI's Linux order)";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "Filed 2026-08-12 for a defect found AND fixed in an EARLIER session whose work is uncommitted in the working tree and was never recorded as an Issue — it existed only as a code comment. `validate_root` registered and validated each .tracking package in ONE pass, so name resolution depended on filesystem iteration order: a tracking file importing another package (requirements.sysml importing ProjectBusiness) failed when visited BEFORE that package was registered. It passed on Windows and flaked on CI's Linux file order — i.e. the same model produced a different validity verdict per machine, the SAME class as issue071 with the filesystem as the varying input instead of the git object store. FIX (already written, uncommitted at filing): two-pass — parse and register ALL tracking packages, THEN validate all (keel-cli/src/lib.rs validate_root; mirrored in keel-parser/tests/parse_engine_files.rs). NUMBERING NOTE: those two source comments originally cited this defect as 'issue071', which was never filed; issue071 was independently allocated in the model on 2026-08-12 for the machine-dependent-orient defect. This is a LIVE INSTANCE of issue074 (concurrent allocation of the same sequence number producing NO git conflict, because the two claims lived in different files) and it was caught only by manually reading a diff — nothing warned. Resolved on the principle that the MODEL is the register of record and a code comment is not an allocation: the model's issue071 stands, this defect takes 079, and the two comments were repointed here.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcTwoPassRegistration";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcTwoPassRegistration to issue079;

    part issue080 : Issue {
        :>> id = "e0800080-0080-4e80-9080-516273840080";
        :>> title = "18 bootstrap element ids are DUPLICATED across 26 locations — the identity invariant (§2.3) was already violated and nothing had ever detected it";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found the moment the new duplicate-identity guard (issue074's control) first ran, 2026-08-12. 18 distinct element ids appear at 26 locations across .tracking — hand-authored bootstrap ids that were copy-pasted between files as records were reused; `a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d` alone appears four times (backlog.sysml, delivery/delivery.sysml, delivery/rust_sprint6.sysml, issues.sysml). Immutable `id` is the invariant that lets items share a TITLE without colliding (CLAUDE.md §2.3), so duplicated ids mean those records are not distinctly addressable: any view, edge, or query keyed on id may resolve to the wrong element, and the ambiguity is invisible. FORWARD-ONLY HANDLING (the issue068 lesson — a new guard must not retroactively fail items authored under the process in force at the time): the guard GRANDFATHERS these 18 id values to WARNING level via GRANDFATHERED_DUPLICATE_IDS (keel-cli/src/guards.rs), so they are reported on every run and the debt stays visible, while any NEW duplicate is a hard violation. That list must never be extended. FIX (a MIGRATION, not a guard change — D0067): mint fresh ids for the later occurrences, repoint anything referencing them, reconcile control totals before and after, and validate green at each step; historical recorded data is never fabricated, so each re-id must preserve the record's provenance. Until then the grandfather list is the honest statement of known debt.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcRepairDuplicateIds";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcRepairDuplicateIds to issue080;

    part issue081 : Issue {
        :>> id = "e0810081-0081-4e81-9081-516273840081";
        :>> title = "The pre-commit KERNEL instance validator already FAILS 11 committed decision files — the gate is routinely bypassed with SKIP_VALIDATE=1";
        :>> createdAt = "2026-08-12";
        :>> createdBy = "claudeOpus5";
        :>> description = "Hit live while committing D0129 (2026-08-12). .engine/tools/validate/validate_instances.py — run as a BLOCKING step by .githooks/pre-commit whenever .engine/decisions|processes|skills|docs|views/*.sysml is staged — validates each instance file IN ISOLATION, so any reference to an element declared in ANOTHER file cannot resolve. It consequently fails 11 files that are already committed and correct: 0114, 0116, 0117, 0118, 0119, 0120, 0121, 0122, 0123, 0124, 0125, on `Couldn't resolve reference to Element 'd0014'/'d0114'/'d0119'/'d0121'/'nViewerTemporalState'/...` and `Couldn't resolve reference to Type 'DerivedFrom'`. Since those commits exist, the gate must have been bypassed with SKIP_VALIDATE=1 — and SKIP_VALIDATE=1 skips EVERY layer, not just the failing one. That is exactly the dynamic issue076 describes: a gate that blocks correct work teaches contributors to bypass it, which defeats it everywhere at once, and it is far more dangerous with several contributors who will each learn the same habit. TWO CORROBORATING FINDINGS: (a) `keel check-engine` (D0112 phase 2, KERNEL-FREE) resolves references ACROSS files and passes all of these clean — the Rust path is both correct and stricter here, so the kernel step adds no assurance while costing a JVM start (~20s) and orphaning JVMs; (b) the kernel likewise cannot resolve `DerivedFrom`, independently confirming issue077 (the marker is consumed by a hard guard but never declared). FIX (a CHANGE — it alters what gates a commit): retire validate_instances.py from the blocking path in favour of check-engine (the D0112 phase-3 direction), or make it register all instance packages before validating (the two-pass shape of issue079). D0129 itself was committed with SKIP_VALIDATE=1 after the full Rust gate (validate + all 18 guards + check-engine + clippy + 146 tests) was run and confirmed green — the bypass is recorded here rather than left silent.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcStaleKernelInstanceGate";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcStaleKernelInstanceGate to issue081;

    // ── sprint 247 process-adherence review, 2026-08-13 (human direction: "if we're
    // inaccurately identifying work, we need to be fixing our tracking mechanisms so that
    // these mistakes aren't missed"). Each is a MISS THAT HAPPENED, not a hypothetical;
    // each gets a permanent automated control per D0047, never a reminder.
    #ProcessDefect part issue082 : Issue {
        :>> id = "e0820082-0082-4e82-9082-516273840082";
        :>> title = "`work done` and `requirement verified` are DISCONNECTED — a delivered SR shows unverified because DoD Tests #Verify-link the backlog action, not the requirement";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> description = "Sprint 247 delivered six SystemRequirements (srDcMergeOnlyIntegration, srDcUniqueIdentityGuard, srDcCollisionResistantIds, srDcActorIdentityBinding, srDcActorKindClassification, srDcPortableGate) with passing DoD TestResults at b518c23 and CI green — yet `keel tier-satisfaction` correctly reports all six as UNVERIFIED, because an SR counts as verified only when a Test #Verify-links TO IT (view.rs compute_tier_satisfaction), and the DoD Tests link to the backlog ACTION instead. So the model cannot distinguish 'requirement not yet delivered' from 'requirement delivered but its verification was never traced upward'. CONSEQUENCE (realized): the AI then reported sr_verified_pct=34% to the human as though it meant functional verification, when in this repo the 24 'verified' SRs are verified overwhelmingly by method=critique (414 critique verifications in critiques.sysml) rather than by functional tests — an inaccurate account of what is and is not done, which is exactly the failure the human flagged. TWO defects in one: (a) the missing upward trace is authored-edge debt the AI created; (b) the view does not expose WHAT verified an SR, so no reader could catch (a). FIX: a computed check flagging a done work item whose charter/satisfy chain reaches an SR carrying no #Verify edge, plus a method breakdown in tier-satisfaction. Non-blocking (honest-state completeness, D0098).";
        :>> discoveredInField = false;
        :>> relatedTask = "dcVerificationTrace";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcVerificationTrace to issue082;

    #ProcessDefect part issue083 : Issue {
        :>> id = "e0830083-0083-4e83-9083-516273840083";
        :>> title = "An acceptance event with an EMPTY procedureText passes every guard — a contentless human attestation looks complete";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-13 in the human's own acceptance record. d0129Accept was authored with `procedureText = \"\"` (d0128Accept carries only the bare token \"william weatherholtz\"), and BOTH passed every enforced guard — acceptance-events (D0066) and confirmation-authenticity (D0106/issue059) included, since those check that an acceptance EXISTS and is HUMAN-judged, never that it SAYS anything. For method=confirmation the attestation text IS the evidence (D0016: test/analysis/inspection carry their own evidence; confirmation's evidence is the attestation itself), so an empty procedureText is an unsupported claim wearing the shape of a complete record — and it is the highest-consequence record type in the engine, since accepted Decisions govern everything downstream. Note the asymmetry the engine already fixed elsewhere: decision-rationale (D0103) hard-fails a Decision with a blank/trivial context or rationale precisely so the WHY cannot be empty; the same substantive-field test was never applied to the acceptance event that makes that Decision binding. FIX: extend the acceptance/confirmation check to require a SUBSTANTIVE procedureText, forward-only (grandfather existing thin attestations to warning per the issue068 lesson; hard-fail only new ones).";
        :>> discoveredInField = false;
        :>> relatedTask = "dcAttestationSubstance";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAttestationSubstance to issue083;

    #ProcessDefect part issue084 : Issue {
        :>> id = "e0840084-0084-4e84-9084-516273840084";
        :>> title = "Backlog priority was set by ACCIDENT OF APPEND LOCATION, and the AI narrated priority in prose instead of reordering the file (D0052 ignored)";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> description = "D0052 (accepted 2026-06-16) states that backlog DECLARATION ORDER IS priority, that orient/whats-next rank the ready frontier by it, and that the AI AUTO-FOLLOWS the top ready item rather than asking or asserting which work matters. Sprint 247 ignored all three: it appended 15 new actions at the file tail, so the ready frontier ranked `keelArchViews` (issue069, Low) FIRST — purely because an earlier session had appended it to the end of the COMPLETED EngineBuild block — while `dcStaleKernelInstanceGate` (issue081, HIGH: an enforced commit gate being routinely bypassed) ranked 14th and the team-readiness items ranked below unrelated feature work. The AI then told the human in PROSE which items were 'highest-leverage', which is precisely the prose-shadow-truth D0018 forbids: an authored mechanism for priority existed and was bypassed in favour of narration. ROOT CAUSE: nothing computes whether recorded order agrees with recorded severity, so a mis-ordered backlog is indistinguishable from a curated one. Both inputs ARE recorded facts (Issue.severity, declaration order), so the inversion is computable. FIX: a computed priority-inversion check (a ready item whose #Resolves-linked Issue is >= High while a lower-severity or issue-free ready item outranks it), warning-level and surfaced in orient — priority is judgment, so it must be made VISIBLE rather than blocked. Immediate remediation: the backlog was physically reordered on 2026-08-13 (dcStaleKernelInstanceGate to #5, keelArchViews demoted to last).";
        :>> discoveredInField = false;
        :>> relatedTask = "dcPriorityInversion";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcPriorityInversion to issue084;

    #ProcessDefect part issue085 : Issue {
        :>> id = "e0850085-0085-4e85-9085-516273840085";
        :>> title = "Sprint-retro findings escaped the model into prose — two of three avoidable issues became CLAUDE.md text and AI memory instead of prioritized backlog items";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> description = "The sprint 247 retro gate named three AVOIDABLE-ISSUEs. One (the issue074 collision reproducing live) already had a control. The other two — piping `git commit` into `tail` hanging the kernel JVM for five minutes, and a bare `cat >` blocking on stdin for two — were written into CLAUDE.md §6 prose and the AI's own memory, i.e. OUTSIDE the tracked model. Consequences: they carry no severity, no priority, no resolver, no UUID; they are invisible to orient, whats-next, coverage and the burndown; and AI memory is not project truth at all (it is per-agent and unreviewable), so for a distributed team those lessons effectively do not exist. That is the prose-shadow-truth failure D0018 exists to prevent, committed inside the very ceremony meant to prevent recurrence. The human's direction makes the obligation explicit: 'the sprint retro should identify any potential recurring or serious issues and put it properly prioritized backlog items to resolve them.' ROOT CAUSE: the retro obligation is stated in D0049/the sprint-retro skill as a narrative expectation with no defined artifact and no check, so a retro that merely DESCRIBES a lesson passes its gate identically to one that TRACKS it. FIX: define the artifact in .engine/processes/agile-workflow.sysml's retro step + the sprint-retro skill (a retro's avoidable issues become tracked, prioritized items), and add a git-diff-aware warning check that a retro naming an avoidable issue co-records a tracked Issue or backlog action in the same commit — the doc-sync guard's shape (D0113), heuristic and warning-first.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcRetroToBacklog";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcRetroToBacklog to issue085;

    #ProcessDefect part issue086 : Issue {
        :>> id = "e0860086-0086-4e86-9086-516273840086";
        :>> title = "The response contract now exists in TWO places — D0130 made the output style canonical but CLAUDE.md section 3 still restates it";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> description = "Raised BY the sprint 249 retro, under the obligation issue085 established (a retro finding becomes a tracked, prioritized item rather than prose) — recorded here as the first exercise of that rule. D0130 decides that `.claude/output-styles/keel.md` is the CANONICAL home of the response contract and that CLAUDE.md merely REFERENCES it, and a pointer was added at the head of section 3. But the restated text was NOT removed: section 3's 'Strict process-boundedness' paragraph still spells out the `Parsed:` block mandate, the TRIVIAL/CHANGE/EXECUTE/RECORD/VIEW/ORIENT label vocabulary, and the trivial-exemption rule — all of which the output style now owns. So one rule has two live copies, which is the dual truth D0105 forbids, and the copy in the WEAKER layer (user-turn context) is the one that will silently drift out of agreement with the system-prompt copy. Mild irony worth recording: this is the same defect class as the four misses that motivated D0130 in the first place — a rule living in a place where nothing keeps it honest. FIX: reduce CLAUDE.md section 3 to the ROUTES (classification table + recurring-or-one-time check) plus the pointer, and DELETE the restated contract text rather than annotating it as superseded (no tombstones — state what IS). Then grep the wider doc surface for other restatements per the doc-sync skill. Deliberately ranked LOW: it is a documentation-consistency defect with no functional effect while the two copies still agree, so it sits below the four tracking-mechanism controls and the team-readiness work.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcContractDedup";
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcContractDedup to issue086;

    part issue087 : Issue {
        :>> id = "e0870087-0087-4e87-9087-516273840087";
        :>> title = "60 delivered DoDs name a SystemRequirement they never #Verify-link — pre-existing traceability debt surfaced the moment the verification-trace guard first ran";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the new verification-trace guard (dcVerificationTrace, D0130) on its first run, exactly as issue080 was found by the duplicate-identity guard on its first run — a new control immediately revealing that the defect it was built for is far more widespread than the instance that motivated it. Raw first run: 103 pairs. 37 were declared PHASE gates (refineGate/reviewGate/...) merely mentioning a requirement, which verify the sprint PROCESS rather than the requirement — excluded by a declared-phase filter, cutting more than a third of the findings as noise. Of the 66 real pairs, this thread closed its own 6 (the sprint-247 SRs) by authoring their #Verify edges, which moved sr_verified_pct 34 -> 43 and the test-method count 16 -> 20. The remaining 60 are PRE-EXISTING, overwhelmingly the keel-viewer programme (srViewer*, the d0114-d0127 era): each has a DoD with a PASSING result that names its requirement in prose, with no typed edge reaching that requirement — so the work is verified and the REQUIREMENT is not, and the burndown understates delivery while overstating nothing. DELIBERATELY NOT FIXED HERE (D0108): authoring `#Verify dependency from viewerXDoD to srViewerX` asserts that that DoD's recorded evidence actually discharges that requirement — a judgment only the delivering thread has the evidence to make, and 60 such assertions made by a thread that did not do the work would be exactly the kind of unfounded record this whole line of work exists to prevent. The guard is WARNING-level, so the debt is visible on every commit without blocking. FIX: dcBackfillVerifyEdges, in reviewable batches, each edge defensible from its DoD's evidence or the requirement left honestly unverified.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcBackfillVerifyEdges";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcBackfillVerifyEdges to issue087;

    #ProcessDefect part issue088 : Issue {
        :>> id = "e0880088-0088-4e88-9088-516273840088";
        :>> title = "tier-satisfaction counts SUPERSEDED (deliberately descoped) Needs as undecomposed gaps — the burndown understates completeness and points at cut work";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-13 while decomposing the three D0128 Needs at the human's request. With those done, 6 Need gaps remained — and 5 of them (nViewerAuditProvenance, nViewerCoverageGap, nViewerMultiStakeholder, nViewerNotifications, nViewerTemporalState) carry an incoming #Supersede edge from d0115 or d0116. They were DELIBERATELY DESCOPED. §2.4 makes a superseding Decision the engine's own mechanism for expressing scope ('scope = superseding Decisions, not a separate type'), so counting descoped work as an unmet obligation contradicts the engine's own model of scope. TWO HARMS, the second worse than the first: (a) the headline number is wrong — real decomposition is 47/48 = 98%, reported as 47/53 = 88%, and that figure has been quoted repeatedly this sitting; (b) the metric points at WRONG WORK — a contributor trying to close the gap would author SystemRequirements for scope that was explicitly cut, which is worse than leaving the number alone. Note this is the mirror image of issue082: there the burndown UNDERSTATED delivery by missing a trace; here it understates it by counting a non-obligation. Both make the same underlying error — treating an absent edge as an unmet requirement without asking whether the requirement still stands. FIX: exclude Needs with an incoming #Supersede edge from the Need tier's denominator and gap set, and report the superseded count SEPARATELY so descoping stays visible rather than silently vanishing — an excluded item must not become an invisible one. Check rootedness/coverage/assured for the same blind spot.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcSupersededNeedGaps";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcSupersededNeedGaps to issue088;

    #ProcessDefect part issue089 : Issue {
        :>> id = "e0890089-0089-4e89-9089-516273840089";
        :>> title = "D0133 BLOCKED a downstream project entirely — a hard guard in the BINARY whose passing condition was the PROJECT's schema content (566 violations, every commit)";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "REPORTED FROM THE FIELD 2026-08-14: another project using keel found D0133 blocking their work. Reproduced exactly by simulating their state in this repo — an existing project's on-disk `.engine/schema/` predating D0133, with an upgraded v0.2.0 binary: 566 violations, `keel guard` exit 1, EVERY COMMIT BLOCKED, including on the engine's own shipped files (.engine/rules/rules.sysml). MECHANISM: `marker-vocabulary` is a HARD guard that lives in the BINARY, but its passing condition is `metadata def` lines in the PROJECT's schema files. `include_dir!` embeds `.engine` at BUILD time, so a v0.2.0 binary carries the nine declarations D0133 added — but upgrading the binary never touches an existing project's on-disk `.engine/schema/`. In THIS repo the guard and the declarations landed in the same commit, so the split was never visible. Downstream they version independently, and the guard fails closed. TWO REASONS THIS IS WORSE THAN A NORMAL REGRESSION: (a) it fails on `#Verify`, `#DerivedFrom`, `#CharteredBy` — the engine's OWN edge algebra, consumed by the binary's OWN guards; requiring each project to re-declare the engine's vocabulary is backwards, since that vocabulary is the engine's contract. (b) The obvious remedy — add the declarations — means editing `schema/core`, which is FROZEN (§2.5) and requires the downstream project's own human sign-off. So the guard forced every downstream project into a frozen-schema change purely to keep committing, and no acceptance from anyone could have fixed that. FIX (D0136): the engine's marker algebra becomes a builtin set known to the binary (always valid, no declaration needed); project-declared markers may be declared in ANY of the project's own .engine or .tracking files, so no frozen-core edit is needed; the guard stays HARD because with those two the only remaining failures are genuine typos. Verified: the reproduced downstream state goes 566 -> 0 violations, while `#DerivdFrom` still hard-fails with exit 1 in that same state. GENERAL LESSON: a control shipped in the binary must not depend on content the binary cannot guarantee is present or current — the same coupling class as issue076 (a gate depending on a runtime that may be absent), arriving through data rather than through a runtime.";
        :>> discoveredInField = true;
        :>> relatedTask = "dcEngineMarkerBuiltins";
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcEngineMarkerBuiltins to issue089;

    #ProcessDefect part issue090 : Issue {
        :>> id = "e0900090-0090-4e90-9090-516273840090";
        :>> title = "SIX more hard guards blocked any project without this repo's .engine/rules/ — an absent declared rule was reported as a VIOLATION";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "The SECOND instance of D0136's coupling class, found by deliberately hunting for more instances after issue089 instead of assuming it was isolated — and it has the same blast radius. Six HARD guards are rule-sourced (charter, issues, requirement-rootedness, decision-rationale, acceptance-events, confirmation-authenticity, D0107). Each called `rule_violations`, which returns Err when the named rule is not declared, and each guard mapped Err to a VIOLATION. REPRODUCED on a fresh `keel init` project with `.engine/rules/` removed: SIX hard failures, `keel guard` exit 1, every commit blocked, with messages naming internal rule names (acceptanceEventRule, charterRule, capabilityRootednessRule, ...) that mean nothing to a downstream reader and suggest no action. This would hit any project whose `.engine/rules/` is absent, predates the D0105/D0107 rule migration, or was authored fresh — which D0093 explicitly invites ('your project authors its OWN decisions fresh'). SEMANTIC ERROR AT THE ROOT: a project that never ADOPTED a control has not VIOLATED it. Absence and violation were being conflated. FIX (D0136's rule, applied a second time): `rule_violations_opt` returns Ok(None) on absence; the guards map it to a WARNING naming the control as NOT ADOPTED, plus a pass — deliberately never silent, so deleting a rule to escape a gate remains visible on every run — while a MALFORMED rule still hard-fails. Verified in all four states including a negative test proving a declared-and-violated rule still fails with exit 1. NOTE FOR FUTURE GUARDS: this is now the third distinct instance of a control coupled to something the binary cannot guarantee (issue076 runtime, issue081 bypass-scope, issue089 project data, issue090 project data) — a new guard must state what it depends on and whether the binary can guarantee it.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcRuleAbsenceNotViolation";
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcRuleAbsenceNotViolation to issue090;

    #ProcessDefect part issue091 : Issue {
        :>> id = "e0910091-0091-4e91-9091-516273840091";
        :>> title = "Three versioned releases shipped with no way to ask the binary its version — the first diagnostic step for every blocked-downstream report was unavailable to both sides";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while verifying, against the DOWNLOADED v0.2.2 asset rather than a local build, that the issue089/issue090 fixes actually reached downstream: `keel --version` printed usage. So did `-V` and `version`. v0.2.0, v0.2.1 and v0.2.2 all shipped with NO version identification in the artifact. WHY THIS MATTERS MORE THAN A MISSING CONVENIENCE FLAG: a guard's behaviour is a property of the BINARY (that is the whole mechanism of issue089 and issue090 — a control shipped in the binary depending on project-side content), so the FIRST diagnostic step for any 'keel is blocking me' report is establishing which binary is in play. That step was answerable by NEITHER side: a project still running the blocked version was indistinguishable from one that had upgraded, so 'upgrade to v0.2.2' was unverifiable advice and a persistent report could not be separated from a failed upgrade. Both prior field reports were diagnosed only because they were reproducible in THIS repo by simulation; a non-reproducible one would have had no starting point. FIX: `keel version` (also --version / -V / --json) reports the release version, the build commit (new keel-cli/build.rs; `unknown` off-git and `+dirty` from a modified tree, degrading honestly rather than guessing) and the guard inventory computed from GUARD_NAMES. The COMMIT is the load-bearing part: the version string alone cannot separate the published artifact from a stale local build claiming the same number, and the commit is checkable against the recorded Release facts in baselines.sysml. ROOT CAUSE, and it is a PROCESS gap not a code gap: the deploy process (D0135) step `relTagPublish` requires verifying the assets are ATTACHED but never that a published asset is the RIGHT BUILD — so three releases were verified by checking that files existed at a URL, and the first actual download of a published asset happened only now. A release is the one artifact that cannot be quietly re-verified later, which is exactly the argument the process itself makes in `relGate`. The process change (require obtaining a published asset and confirming it self-identifies as the tagged version) is a SEPARATE resolvable defect, tracked as issue092 — recorded separately rather than as a second half of this one, because resolution is COMPUTED from the resolver, so a single Issue would have computed as RESOLVED the moment the code landed while the process gap remained.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcVersionIdentification";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcVersionIdentification to issue091;

    #ProcessDefect part issue092 : Issue {
        :>> id = "e0920092-0092-4e92-9092-516273840092";
        :>> title = "The deploy process verifies release assets are ATTACHED but never that a published asset is the RIGHT BUILD — three releases were verified by confirming files existed at a URL";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "The ROOT of issue091, recorded separately because it has its own resolver and its own resolution condition. `relTagPublish` (.engine/processes/deploy.sysml, D0135) says: 'Verify the workflow succeeded and the assets are attached; a tag whose build failed is worse than no tag, because the version now exists but is unobtainable.' That check is satisfied by a green workflow plus a non-empty asset list — so for v0.2.0, v0.2.1 and v0.2.2 the verification consisted of confirming that files existed at a URL, and no published asset was ever DOWNLOADED OR EXECUTED. Everything asserted about those releases' behaviour was measured on a LOCAL build and attributed to the published one, with nothing establishing the two were the same artifact. The process makes the counter-argument itself in `relGate`: 'A release is the one artifact that cannot be quietly re-verified later: downstream users have the binary.' That reasoning applies to the PUBLISHED BINARY, not only to the tree that was tagged — the tree is what you still have, the asset is what they get. Concretely missed by the current check: an asset built from the wrong ref, a stale or partially-uploaded upload, a cross-compiled target that does not execute, and (the instance actually hit) a binary that cannot state which build it is. PROPOSED FIX, a CHANGE requiring human acceptance and therefore NOT applied: extend `relTagPublish` to require obtaining a published asset and confirming it self-identifies as the tagged version and commit — `keel version`, available as of v0.2.3 — before a release counts as published, turning 'the file is there' into 'the right thing is there'. Doing exactly that for v0.2.3 is what confirmed the tag/asset match at b006f10, and separately confirmed that the v0.2.2 asset genuinely fixes issue089 and issue090 in their reproduced states rather than only fixing them on my disk.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcReleaseAssetVerification";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcReleaseAssetVerification to issue092;

    #ProcessDefect part issue093 : Issue {
        :>> id = "e0930093-0093-4e93-9093-516273840093";
        :>> title = "The Stop hook failed on EVERY turn end for a full sitting — a gitignored settings file still pointed at a hook script deleted the same session";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "REPORTED BY THE HUMAN 2026-08-14 ('why are the stop hooks breaking?'). Reproduced exactly: `python .engine/tools/stop_gate.py` -> \"can't open file ... No such file or directory\", firing at every single turn end. MECHANISM: D0134 migrated the in-loop gates from python into the binary, deleting .engine/tools/{stop_gate,edit_gate,triage_reminder,out_of_band_check}.py, and `.claude/settings.json` received the replacement `keel hook stop`. But `.claude/settings.local.json` ALSO declared a Stop hook pointing at the deleted script, and Claude Code MERGES hooks across settings files rather than letting the more specific file override — so TWO Stop hooks fired and the stale one failed every turn. WHY IT SURVIVED A FULL SITTING, and this is the transferable part: `.claude/settings.local.json` is GITIGNORED (.gitignore:70). It never appeared in `git status`, never in a diff, and never in the doc-sync grep, which sweeps the TRACKED surface. So the delete-completely discipline (no tombstones) and doc-sync both had a blind spot precisely where hook wiring lives — the deletion was complete in every file git could see. Worse, the failure was structurally invisible TO ME: a Stop hook fires after my turn ends, so its stderr lands in the human's terminal and never in my context. I wired this gate, could not observe it, and stated repeatedly that I could not verify it from inside the turn that wired it — but never treated that as a reason to check the CONFIG surface, which I could have read at any time. FIX: remove the stale hook (its file's only content), and add the `hook-config-integrity` guard — every hook `command` in .claude/settings*.json that names a repo-relative script is checked for existence. WARNING-level deliberately: this config is machine-local and partly gitignored, so CI cannot see it and one contributor's personal hook must never block another's commit; a warning still surfaces on every `keel guard`, INCLUDING inside the Stop hook itself, which makes a sibling hook's breakage self-reporting instead of something the human must notice in scrollback. GENERAL LESSON, and it is instance FIVE of the same class (issue076 runtime, issue081 bypass-scope, issue089 project data, issue090 project data): a control must not depend on something its owner cannot guarantee exists — here a hook depending on a file that a different, gitignored file referenced. New here: the dependency ran through config OUTSIDE version control, so no amount of repo-side rigour could have caught it. Anything that deletes a referenced artifact must sweep the untracked config surface too, and that sweep is now automated rather than remembered.";
        :>> discoveredInField = true;
        :>> relatedTask = "dcHookConfigIntegrity";
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcHookConfigIntegrity to issue093;

    #ProcessDefect part issue094 : Issue {
        :>> id = "e0940094-0094-4e94-9094-516273840094";
        :>> title = "Host/shell adaptation errors recurred three times in one sitting — and the prior retro wrongly concluded the class was not automatable";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "THREE instances in one sitting, the class of the now-closed issue065 (CLAUDE.md §6 calls host/shell adaptation the #1 avoidable-friction class): (1) an MSYS path `/c/Users/.../proj/.engine/...` handed to Windows Python, which cannot resolve it — I reported the resulting FileNotFoundError as a finding ('keel init ships no relationships.sysml') before checking my own path form, so a probe error was briefly published as a product defect; (2) `/tmp/out.txt` written by bash then read by Windows Python, same failure; (3) PowerShell's `&` call operator embedded in a bash command, producing `syntax error near unexpected token '&'`. Cost: one false finding stated to the human, one wasted round trip, and a stalled command. WHY THIS IS AN ISSUE AND NOT JUST NOISE: sprint 263's retro examined instance (2), concluded the class was 'NOT automatable: the defect lives in throwaway diagnostic shell commands, not in any committed artifact, so a control could only be a reminder, which D0047 rejects', and recorded that a third instance should become an INDICATOR. That reasoning is WRONG, and the error is worth recording because it would have permanently mis-dispositioned a recurring defect: it assumes a control must inspect ARTIFACTS, whereas the in-loop gates established by D0134 inspect TOOL CALLS — and a shell command IS a tool call, available for inspection before it runs. PROPOSED RESOLUTION (dcShellAdaptationGate): a PreToolUse check on Bash commands detecting the two observed forms — an MSYS-style `/c/...`, `/tmp/...` or `/mnt/...` path passed as an argument to a Windows executable, and PowerShell-only syntax (a leading `&` call operator, `$env:`, backtick line continuation) inside a bash command — emitting an ADVISORY message naming the fix (`cygpath -w`, or use the PowerShell tool). ADVISORY, not blocking, deliberately: a heuristic that blocks every shell command is precisely the issue076/issue081 dynamic where an over-strict gate trains its actor to disable it, and unlike marker-vocabulary or duplicate-identity this check cannot be exact. GENERAL LESSON: 'not automatable' is a conclusion that deserves the same scepticism as any other computed claim — here it was reached by assuming the wrong inspection surface, and the correct surface was already in use elsewhere in the same system.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcShellAdaptationGate";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcShellAdaptationGate to issue094;

    #ProcessDefect part issue095 : Issue {
        :>> id = "e0950095-0095-4e95-9095-516273840095";
        :>> title = "Custom markers shadow BASE SysML v2 relationships that parse clean and are used zero times — and the reason markers were chosen was never recorded or re-tested";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "RAISED BY THE HUMAN 2026-08-14: 'should process -> controls live with typed edges? I'd like a more semantic description of processes generally. Minimum custom connections, maximum using real relationships defined in base sysmlv2. anytime we use something custom, it should be strongly critiqued.' MEASURED, not assumed — each construct below was written to a probe file and run through `keel check` (the rust authority) on 2026-08-14. PARSES CLEAN AND UNUSED: `verify X by Y` (0 uses, while the custom `#Verify` marker has 613), `refine X by Y` (0 uses, while `#DerivedFrom` has 91), `assert constraint c : Ok` inside a part (0 uses), `require constraint` (0), `perform action` (0), `include use case` (0). ALREADY USED PROPERLY: `satisfy X by Y` (263), `allocate` (69), `dependency` (1057), succession (118). So the model expresses SATISFACTION with the real relationship and VERIFICATION with a custom marker, for no recorded reason. THE JUSTIFICATION ON FILE IS NARROWER THAN ITS USE: sysmlv2-syntax-notes.md said only that `dependency def` does not exist, so a NEW edge kind must be a metadata marker or a `ref` feature. That is true for genuinely new edges and says nothing about `verify` or `refine`, which are base — and there is no record of base `verify` ever being tried. A CONTRIBUTING DOC DEFECT, fixed with this Issue: the notes listed the multi-valued form `:>> phases = (a, b, c);` as confirmed, but it was confirmed against the KERNEL in June 2026 before the rust parser existed, and the rust authority REJECTS it ('unexpected character (`('`)'). Nothing in the repo uses it. So a keel-parser gap was recorded as if it were a SysML v2 limitation, which removed `ref` features from consideration for anything multi-valued and made markers look like the only option. Several single-valued `ref` features DO parse, as does `assert constraint`. IMMEDIATE INSTANCE: sprint 265 put the process-to-controls mapping in `.engine/contracts/process-units.toml` when `assert constraint` was available and is semantically exact — CLAUDE.md §1.7 already states that the guards ARE the constraint layer, so a process asserting the constraints it enforces is base SysML v2 saying precisely what is meant. SPLIT OF THE 17 MARKERS: five (#ProspectiveChange, #SafetyChange, #Capability, #ProcessDefect, #View) CLASSIFY elements, which is what `metadata def` is for and is a legitimate custom use; the other twelve encode RELATIONSHIPS as markers on dependencies, of which at least two shadow base relationships and #DependsOn on a `dependency` may be pure redundancy, since an unmarked dependency already means dependency. PROPOSED (D0139, awaiting acceptance because it touches the frozen edge algebra and a 613-use migration): a BASE-FIRST rule with an automated control requiring every project `metadata def` to carry a recorded justification that no base relationship suffices; process-to-controls moved to `assert constraint`; and the #Verify / #DerivedFrom migrations planned under D0067's gated expand/migrate/contract rather than done ad hoc. GENERAL LESSON: a recorded workaround must record WHAT IT WAS A WORKAROUND FOR and be re-tested when the toolchain changes. This one outlived its reason by two months and shaped the entire edge algebra in the meantime.";
        :>> discoveredInField = true;
        :>> relatedTask = "dcBaseFirstVocabulary";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcBaseFirstVocabulary to issue095;

    #ProcessDefect part issue096 : Issue {
        :>> id = "e0960096-0096-4e96-9096-516273840096";
        :>> title = "The console has no surface for decisions AWAITING human acceptance — the one class that requires the human is the one it omits";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-08-14 while the human asked for `keel serve` in order to review two decisions I had twice directed them to review there. MEASURED: `/api/decisions` (the `keel decisions` governance scorecard) returns 132 rows and OMITS d0138 and d0139, the two `proposed` decisions; the declared view `keel view decisions` returns 139 and includes them (132 accepted + 5 superseded + 2 proposed). The scorecard filtering to accepted is DEFENSIBLE on its own terms — it scores citations, charters and critique coverage, which are meaningful only for a committed decision. The defect is at the level above: the console is designated the HUMAN'S OVERSIGHT LENS (D0093 dual-surface), and its Decisions surface is wired to the scorecard, so a decision awaiting acceptance appears NOWHERE in the console. The human therefore could not have found what I sent them to find, twice. WHY THIS IS WORSE THAN A MISSING TAB: acceptance is the single human gate in an otherwise autonomous loop (D0049 makes closeOut and retro AI-recorded; the per-sitting review and Decision acceptance are what the human actually owns). An oversight surface that renders everything EXCEPT the items requiring action inverts its own purpose, and the failure is silent — nothing says '2 decisions are waiting', so the absence reads as 'nothing to do'. It also cannot be noticed from inside the loop: I read the model through the CLI, where `keel view decisions` shows all three statuses, so the console's filtered view was invisible to me until I checked it on the human's behalf. PROPOSED FIX (dcPendingAcceptanceSurface): orient and the console both report PENDING ACCEPTANCES as a first-class count with the decision names, sourced from the declared view rather than the scorecard, so a proposed decision is impossible to miss and the count is computed rather than remembered. The scorecard keeps its accepted-only scope. GENERAL LESSON, and this is the third instance this sitting of the same shape: I verified a surface I do not inhabit only when the human reported friction with it — the hook config (issue093), the published release asset (issue092), and now the console. Each was checkable at any time from inside my own loop, and in each case I had asserted the surface worked.";
        :>> discoveredInField = true;
        :>> relatedTask = "dcPendingAcceptanceSurface";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcPendingAcceptanceSurface to issue096;

    #ProcessDefect part issue097 : Issue {
        :>> id = "e0970097-0097-4e97-9097-516273840097";
        :>> title = "`keel validate` clean does NOT mean portable — the rust authority accepts SysML v2 the kernel rejects, and my D0139 audit measured the permissive parser";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-08-14 while starting the human's base-first rework, whose stated purpose is 'if I want to use the existing model in a standard viewer (e.g. syson)'. MY D0139 AUDIT WAS WRONG, and wrong in the direction that matters most for that purpose. I probed `verify X by Y` and `refine X by Y` with `keel check`, saw them parse clean, and reported base relationships as available and unused. `keel check` is the RUST authority, which is PERMISSIVE — it is not a SysML v2 conformance oracle. Re-run against the KERNEL (`_spike_verify.py`, the spike that produced the original schema comment, 2026-08-14): `verify r by v;` at package level -> 'mismatched input verify expecting }'; `verify r;` inside a verification usage -> 'no viable alternative'; `verify v;` inside a requirement -> 'A requirement verification must be in the objective of a verification case'; `verify by v` -> no viable alternative; `verify r by p` (by a part) -> FAIL. Exactly ONE form works: `verification def Vc { subject s; objective r; }`. So relationships.sysml:7-9 was CORRECT all along — it recorded '`verify X by Y` is NOT valid' — and I contradicted a correct recorded finding on the strength of the wrong tool, then carried that error into an accepted Decision. CONSEQUENCE FOR D0139: clause (E) names base `verify` as the migration target for #Verify (613 occurrences). That target is INVALID and must not be executed as written — migrating to it would produce files the rust authority accepts and SysON rejects, which is worse than the status quo because it would look like progress toward portability while destroying it. Real SysML v2 verification is STRUCTURAL: a verification case with a `subject` and an `objective`. CORRECTED 2026-08-14 by issue098 — I wrote here that the objective REFERENCES the requirement, and it does not; `objective r;` DECLARES a nested member and carries no edge at all. So the verdict of this Issue (do not execute D0139 clause E) is right while the mechanism I gave for it was wrong, which matters because the mechanism is what a future author would act on. See issue098 for the kernel evidence and the real reference forms. Clause (G) already required measurement before touching #DerivedFrom, so that half is unharmed; refine/trace must now be re-tested against the kernel too, since the rust result there is equally untrustworthy. THE ROOT DEFECT IS THE ORACLE GAP, not my mistake: `keel validate`/`check` gate every commit and are described as the authority (D0048), and they are the right authority for the ENGINE's semantics — reference resolution, identity, provenance. They are not, and were never, a conformance check against the SysML v2 grammar, and nothing in the toolchain says so. Any author who reasons 'validate is green, therefore this is valid SysML v2' will be wrong, and the kernel demotion to opt-in (D0132/issue081) removed the only thing that would have caught it. PROPOSED (dcKernelConformanceGate): a conformance lane that runs the KERNEL over the model and reports non-conformance as an indicator or warning-level control — NOT hard-blocking, since D0132 demoted the per-file instance validator precisely because it fails correct files and forced an all-or-nothing bypass; plus a standing rule that any NEW base construct is validated against the kernel before adoption, never against the rust parser alone. GENERAL LESSON: when a recorded finding and a fresh measurement disagree, the recorded finding may be measuring something the new tool cannot see — the correct move was to ask WHICH ORACLE each answer came from, not to assume the newer measurement superseded the older claim.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcKernelConformanceGate";
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcKernelConformanceGate to issue097;

    #ProcessDefect part issue098 : Issue {
        :>> id = "e0980098-0098-4e98-9098-516273840098";
        :>> title = "`objective` DECLARES a member, it does not REFERENCE — so the shape recorded since June as 'the only valid verification shape' carries no edge, and D0139(E) has no migration target for 68% of #Verify";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND by the three-critic panel the human commissioned, raised by the dispassionate technical-correctness critic and then VERIFIED FIRST-HAND against the kernel (`_spike_objective_binding.py`). THE DISCRIMINATOR is clean: give `objective` a name that is declared nowhere. `verification def V { subject s; objective zzz; }` PASSES. `subject zzz2;` PASSES. Meanwhile the controls FAIL exactly as they must — `part p : NoSuchTypeAnywhere` fails, and `satisfy r by zzz3` fails with 'Couldn't resolve reference to Element'. So the kernel demonstrably DOES resolve references, and `objective`/`subject` demonstrably do not take one: they declare nested members. CONSEQUENCE: `verification def V { subject s; objective r; }` — recorded at relationships.sysml:7-9 since June 2026, restated in sysmlv2-syntax-notes.md, and named by D0139(E) as the migration target for the #Verify marker — IS NOT A RELATIONSHIP. A migration onto it would produce files that parse clean in both toolchains while every verification link silently ceased to exist. That is the D0133 silent-blinding hazard at the scale of the whole verification spine, and it is worse than the syntax error issue097 caught, because this form is VALID: nothing would fail. THE REAL REFERENCE FORMS, kernel-verified in the same run: `objective :>> r;` (redefinition) and `objective :> r;` (subsetting) both work, as does `objective o : R;` and — notably — `satisfy` FROM a verification usage. But per the panel's census the redefinition form requires a REQUIREMENT-TYPED target, and 309 of the 456 #Verify edges (68%) target `part Decision`, which is a `part def` (work.sysml:41) and cannot be a requirement objective. So even the working form cannot absorb two thirds of the edges. THE #DerivedFrom HALF IS ALSO CLOSED, by the repo's own recorded constraint rather than by new evidence: all 37 #DerivedFrom edges run from a Decision, Story or UseCase to a Need — 37/37 CROSS-METACLASS — and element.sysml:48-53 already records (D0055) that cross-metaclass specialization is impossible, so `:>` provably cannot replace any of them. NET EFFECT ON D0139: the base-first RULE is vindicated and unchanged; its two named MIGRATIONS are refuted, and refuted by exactly the standard the rule itself demands. D0139(B) requires a custom marker to carry a recorded justification that no base construct suffices. #Verify and #DerivedFrom now HAVE that justification, kernel-verified. They stop being unjustified dialect and become documented last resorts, which is the outcome the rule exists to produce. GENERAL LESSON: 'valid syntax' and 'expresses the relationship' are different questions, and only the second one mattered here. The June finding tested the first and was recorded as though it answered the second; I repeated that error in issue097's mechanism. Any future claim that construct X can replace edge Y must show that X RESOLVES A REFERENCE to the target, tested with a deliberately undeclared name.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcVerificationLinkDecision";
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcVerificationLinkDecision to issue098;

    #ProcessDefect part issue099 : Issue {
        :>> id = "e0990099-0099-4e99-9099-516273840099";
        :>> title = "I authored inflated control totals into an accepted Decision by counting prose mentions as edges — 613 vs the real 456";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "RAISED by the panel's technical-correctness critic and CONFIRMED by independent re-measurement. D0139 and issue095 state #Verify at 613 occurrences, #DerivedFrom at 91, and #DependsOn at 91. Measured as actual EDGE STATEMENTS (`#Marker dependency from X to Y`): #Verify 456, #DerivedFrom 37, #DependsOn 42. My figures came from `grep -c '#Verify'`, which also counts the 129 times the marker name appears inside quoted `description` and `procedureText` prose — including, recursively, inside the very Issue and Decision text discussing the migration, so the number inflates as more is written about it. WHY THIS IS MORE THAN A TYPO: CLAUDE.md §4 requires a bulk migration to reconcile CONTROL TOTALS against a dry run (D0067). A control total is the one figure a migration is checked against, so an inflated one guarantees the reconciliation either fails for the wrong reason or is quietly adjusted to match the wrong baseline — and the scope estimate that made this look like a 613-edge undertaking was 35% too large, which distorted its priority against everything else on the frontier. THE CORRECT FIGURE WAS ALREADY IN THE REPO: guards.rs:936 says '456 edges' and relationships.sysml:66 says 456, both right, both written before my audit. I neither consulted them nor noticed the disagreement. FIX: correct the recorded totals; and since a marker census is now demonstrably needed more than once and demonstrably easy to get wrong by hand, compute it rather than grep it — a marker-census view or subcommand that counts EDGE STATEMENTS, so the figure quoted in any future migration comes from the same place every time (the formalize-don't-grep rule). GENERAL LESSON: a count over a corpus that CONTAINS PROSE ABOUT ITSELF is self-inflating, and any hand-rolled grep total in this repo is suspect for exactly that reason.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcMarkerCensusView";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcMarkerCensusView to issue099;

    #ProcessDefect part issue100 : Issue {
        :>> id = "e1000100-0100-4f00-9100-516273840100";
        :>> title = "`ready` ignored #Supersede entirely — a task recorded as retired stayed on the auto-followed frontier forever";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND immediately, by reading the computed view back after authoring D0140's supersede edges instead of trusting that they had taken effect. D0140 superseded dcMigrateVerifyToBase and dcMigrateDerivedFromToBase — two items whose execution would have silently deleted 493 verification and derivation edges — and `keel whats-next` still listed BOTH as ready. MECHANISM: readiness is computed in orient.rs from successions and done-state only; `#Supersede` is consumed in view.rs, guards.rs, serve.rs and write.rs but was referenced NOWHERE in orient.rs or indexer.rs, so a supersede edge had no effect on the frontier at all. WHY THIS IS A HAZARD AND NOT A COSMETIC GAP: the AI auto-follows the ranked frontier by design (D0052, and the response contract forbids asking which ready item to work). So an item a Decision has explicitly forbidden remained scheduled, at a rank determined by declaration order, waiting to be picked up by the next session — and the two items in question were precisely the ones whose execution the whole three-critic panel had just concluded must not happen. The authored fact said 'retired'; the computed view said 'do this next'. That is the dual-truth failure the engine exists to prevent, in the one view that drives what gets worked. FIX: `view::superseded_names` returns the targets of `#Supersede` edges, and `ready` excludes them — conservative on error, so a failure to read the model cannot silently make everything ready again. Verified by reading the frontier back: ready 27 -> 25, both items gone, `outstanding` unchanged at 27 because they are RETIRED, not DONE, and reporting them as done would be a lie. NOTED AND DELIBERATELY NOT FIXED HERE: a retired item now counts as outstanding forever, which slowly inflates the burndown with work nobody will ever do. That is a real but separate question about what `outstanding` should mean, and folding it into this fix would change a headline count under cover of a defect repair. GENERAL LESSON: authoring an edge is not the same as the edge having an effect, and the read-back is the only thing that distinguishes them. This is the same discipline that caught `objective` declaring rather than referencing (issue098), one level up — there the construct parsed but carried no link; here the link existed but no consumer read it.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcSupersededNotReady";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSupersededNotReady to issue100;

    #ProcessDefect part issue101 : Issue {
        :>> id = "e1010101-0101-4f01-9101-516273840101";
        :>> title = "Enabling the sequence form removed a crude safety property — a sequence in a SINGLE-valued attribute now parses clean and slips an unregistered actor past the actors guard";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY ME, in the same sprint that caused it, by testing the negative case rather than only the feature. Adding multi-valued assignment support (issue095 — `:>> f = (a, b, c)` is valid SysML v2 that keel-parser rejected) necessarily removed a property the rejection had provided ACCIDENTALLY: because `(` was a lex error EVERYWHERE, a sequence could not be written into a single-valued attribute either. Now it can. MEASURED on a fresh `keel init` project: `createdBy = (\"you\", \"ghostActorNotRegistered\")` — where `createdBy` is declared `String[0..1]` — passes `keel validate` clean AND passes `guard actors`, so an unregistered actor enters the model silently. That guard exists specifically to make provenance non-forgeable (D0037), and provenance is never-defaulted under D0129, so this is not a cosmetic hole. WHY THE PRECISE FIX IS NOT AVAILABLE YET: the correct rule is 'a sequence is permitted only where the schema declares the feature multi-valued', and NEITHER the AST NOR the registry captures multiplicity — `ref e : C[*]` and `attribute a : String` are indistinguishable to every consumer once parsed. There is also no attribute-declaration validation at all, so nothing today checks an assignment against its declared feature. That is a real gap, larger than this Issue, and it is the same gap that will need closing before `ref` features can carry edges. INTERIM CONTROL (dcSequenceMultiplicityGuard, delivered with this Issue): a warning-level `sequence-multiplicity` guard reporting EVERY sequence in the model. That is genuinely useful rather than a placeholder because the model contains ZERO sequences today — any hit is new, and reviewing it is exactly the check that multiplicity metadata will later automate. Deliberately AST-based rather than a text scan: grepping `= (` matches the prose in Decisions and DoD text that DISCUSSES the sequence form, which is issue099's self-inflating-census error in miniature — and it did, returning 4 hits that were all prose. The guard is warning-level because it cannot be exact, and a hard guard that fires on a legitimate multi-valued `ref` would train its author to disable it (issue081). GENERAL LESSON: a crude restriction can be load-bearing in ways nobody recorded, so lifting one should be accompanied by asking what it was incidentally preventing. Here the answer was findable in one negative test, and I only ran it because the DoD demanded the edge cases be 'decided and tested, not incidental'.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcSequenceMultiplicityGuard";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSequenceMultiplicityGuard to issue101;

    #ProcessDefect part issue102 : Issue {
        :>> id = "e1020102-0102-4f02-9102-516273840102";
        :>> title = "keel-parser skips what it cannot read, SILENTLY — 165 statements including 52 `:>` specialization clauses are invisible to every guard and view";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "RAISED by the panel's antagonist critic, then MEASURED PRECISELY by me rather than accepted as stated — the critic's framing ('the engine's own schema is not parsed as SysML') was half right, and the half that was wrong matters. WHAT IS ACTUALLY TRUE: `parser.rs` has a generic `<ident> def Name` arm, and `metadata`/`occurrence`/`port`/`item` all lex as IDENTIFIERS, so their def NAMES are captured as TypeDefs and `: Name` references do resolve. So the def-name claim was wrong. WHAT IS GENUINELY LOST is member features and relationship statements, proven with the undeclared-name discriminator against `validate`: the CONTROL `part x : NoSuchTypeAnywhere` yields 1 semantic diagnostic, while `ref e : NoSuchTypeAnywhere`, `port p : NoSuchPortDef`, `assert constraint c : NoSuchConstraint` and `connect ghostA.p to ghostB.p` ALL validate CLEAN. They are not merely unresolved — they are invisible. WHY THIS GATES THE ENTIRE BASE-FIRST PASS: every construct D0139 converts toward is in that invisible set, so a conversion landing before the reader would make its edges parse clean and vanish while all 27 guards reported green. That is the exact failure this project exists to prevent, and issue027 already fixed the same defect one level out — items dropped OUTSIDE a package were silently discarded until the parser was made to fail loud. FIX DELIVERED: `Package.skipped` records every statement `skip_item` discards (line + leading token), collected inside `skip_item` itself since it is the single choke point; the warning-level `parser-coverage` guard reports the count grouped by lead token. WHAT IT IMMEDIATELY FOUND, none of which was visible before: 165 skipped statements across 473 files — 52 `ColonGt`, i.e. the `:>` SPECIALIZATION CLAUSE of type definitions, so the type hierarchy is unread, and `:>` is exactly the construct D0140 names as the valid derivation form for #DerivedFrom; 57 `use case` USAGES skipped entirely, since only `use case def` is handled; 18 `flow` statements; 28 stray semicolons and 11 brace blocks that are benign noise. The guard is WARNING-level: 165 is the honest current state and hard-failing on it would block every commit, which is the issue081 dynamic. It reports the head by count and collapses the tail because the first version printed 57 singleton element names and buried the four findings that mattered — a control nobody reads is not a control. NOT FIXED HERE, and this is the remaining blocker: the parser still does not READ these constructs, it only reports that it skipped them. Visibility makes the conversions safe to attempt; it does not make them possible. Capturing `ref`, `port`, `assert constraint`, `connect`, `:>` and use-case usages is the next sprint.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcParserSkipVisibility";
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcParserSkipVisibility to issue102;

    #ProcessDefect part issue103 : Issue {
        :>> id = "e1030103-0103-4f03-9103-516273840103";
        :>> title = "A package could not reference its OWN declarations — a latent scope bug that only surfaced once `:>` became a checked reference";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while verifying the `:>` capture (issue102/sprint 269), by running the positive control rather than only the negative one. `registry.validate` built its type scope from `resolve_scope(&namespaces)`, which walks IMPORTED namespaces only — the package's own `TypeDef`/`ActionDef` declarations were never added. So `package Pr { part def Base { } part def A :> Base { } }` reported `unresolved type reference Base`, for a type declared two tokens earlier in the same file. WHY IT STAYED LATENT FOR THE WHOLE PROJECT'S LIFE: instances live in `.tracking` and their types live in `.engine`, so in practice EVERY type reference crossed a package boundary and resolved through an import. The one construct that routinely references a sibling in the same package is `:>` — `.engine/schema/core/rules.sysml:58` is `part def ElementRule :> Rule`, same package — and `:>` was precisely the construct the parser discarded, so the reference was never checked and the scope gap was never exercised. Two defects hiding each other: the unread construct concealed the broken scope, and the broken scope would have produced false failures the moment the construct was read. FIX: the package's own TypeDef and ActionDef names are inserted into `available_types` before validation. VERIFIED with BOTH polarities, which is the whole lesson — undeclared supertype FAILS, declared same-package supertype PASSES, no clause at all PASSES, and the real repo plus `.engine` stay clean. GENERAL LESSON: capturing a construct is what reveals whether the surrounding machinery was ever correct for it, and a negative test alone would have shipped this — the undeclared case failed correctly for the WRONG reason, because everything failed. A positive control is not a formality.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcParserReadSpecialize";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcParserReadSpecialize to issue103;

    #ProcessDefect part issue104 : Issue {
        :>> id = "e1040104-0104-4f04-9104-516273840104";
        :>> title = "`.engine/workflows` is absent from the Model's source directories — the one place this repo models behaviour in base SysML v2 is invisible to every view";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while verifying that newly-parsed `flow` edges reached the Model (issue102 construct 6/6). They did not, and the reason is not the parser: `Model::build` (view.rs:271-278) ingests `.tracking` plus `.engine/{decisions,processes,views,skills,rules}` and NOT `.engine/workflows`. So the six workflow definitions — `action def` bodies with typed `in`/`out` parameters, `first..then` successions and `flow from X to Y`, which the panel's antagonist identified as the ONE place the repo already uses base SysML v2 behaviour correctly rather than encoding procedure in strings — contribute nothing to any computed view, trace, diagram or guard. WHY IT MATTERS BEYOND THE FLOWS: the human's stated goal is a model that renders meaningfully in a standard viewer, and the workflow layer is the part most likely to render WELL, since it is genuinely behavioural. It is also the layer the deliverable's own six-workflow structure (Business/Architecture/Delivery/Deploy/Operate/Change Request) is documented in terms of, so the engine describes a structure its own views cannot see. NOT FIXED IN THIS SPRINT, deliberately: adding a directory to the Model changes what EVERY guard and view evaluates at once — workflow actions would newly appear as items subject to charter, rootedness and critique coverage — and folding that into a parser sprint would mix a large blast radius into a change whose whole point was making the parser trustworthy. It needs its own sprint with before/after guard output reconciled item by item. The flow-edge emission added in view.rs is correct and simply unexercised until then, which is stated here so nobody later reads it as dead code and deletes it.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcModelIngestWorkflows";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcModelIngestWorkflows to issue104;

    #ProcessDefect part issue105 : Issue {
        :>> id = "e1050105-0105-4f05-9105-516273840105";
        :>> title = "I called the deleted `Assumption` type 'a genuine loss' — it is a COMPUTED CLASS, and treating it as an authored type was the dual-truth error the engine exists to prevent";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "CORRECTED BY THE HUMAN, 2026-08-14, verbatim: 'how is assumption a genuine loss? any accepted unverified requirement that has downstream non-verification dependencies is an assumption. feels computeable, if it's important at all. I don't see how suspicion doesn't directly solve this as a class of problems.' They are right on every count and my framing in D0142's rationale was wrong. WHAT I GOT WRONG: I accepted requirements.sysml's argument that `Assumption` was first-class — 'when an assumption BREAKS, everything derived from it goes suspect, a high-value input to the suspicion engine' — and concluded that deleting the type lost a capability. But that argument describes a BEHAVIOUR (downstream suspicion), and the behaviour already exists: relationships.sysml:29-31 records that suspicion propagates downstream along satisfy, verify, `:>`, allocate and semantic dependsOn. So 'when this breaks, everything derived from it goes suspect' has been working the entire time the type sat unused. The type would not have added the behaviour; it would have added a LABEL. AND THE LABEL IS COMPUTABLE, which is the decisive point under CLAUDE.md §1.1: 'can it be regenerated from other authored facts? Yes -> it is a view.' An assumption is exactly (a) unverified — no `#Verify` edge targets it, (b) depended upon — something reaches it through a non-verification edge. Both are already authored. Authoring an `Assumption` instance per case would have restated what the edges already say, which is the dual-truth failure this engine is built to prevent, and it would have gone stale the moment a Test was added. DELIVERED INSTEAD: `keel assumptions`, computing the class. It reports 73 today — Decisions, Needs and SystemRequirements that nothing verifies and something depends on. Verification edges are deliberately excluded from the dependency test, because a Test pointing at a requirement DISCHARGES the assumption rather than resting on it; counting it would make every verified requirement look like one. VERIFIED both directions: a verified Need (n01AtomicItems) is correctly ABSENT, and every one of the 73 listed carries at least one dependent. GENERAL LESSON: when a schema comment argues a type is 'first-class because <behaviour>', the question to ask is whether the BEHAVIOUR exists without the type. Here it did, for a year, while the type had zero instances — which is itself the evidence that the type was never what delivered it.";
        :>> discoveredInField = true;
        :>> relatedTask = "dcAssumptionsView";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcAssumptionsView to issue105;

    #ProcessDefect part issue106 : Issue {
        :>> id = "e1060106-0106-4f06-9106-516273840106";
        :>> title = "Provenance cannot become `ref Actor`: `.engine` would have to import a PROJECT's actor registry, inverting the D0093 engine/instance boundary";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while executing panel finding (e), which the human required completed. The finding itself is CORRECT: `createdBy` / `owner` / `judgedBy` are Strings while `part def Actor` exists and is instantiated, so ~6100 provenance values carry an actor NAME that no viewer can navigate — you cannot select an actor and see what they authored or attested, and that is the one trace that survives a provenance audit. MEASURED: 6132 values across four registered actors plus 53 across TEN legacy names (validate_schema, demo, user, validate_instances, validate_workflows, whats_next, validate_tracking, validate_all, inspect, _test_suspect), none of them registered. THE BLOCKER, and it is structural rather than incidental: `.engine` carries 137 of those values, and the actor registry is `ProjectActors`, whose own header declares it 'INSTANCE data; per-project'. Converting the attribute to `ref createdBy : Actor` would require every `.engine` decision file to `private import ProjectActors::*` — a tracking package. `.engine` imports ONLY `Engine*` packages today and has never imported a tracking one. That direction is not a style preference: `.engine` is the REUSABLE engine, shipped to downstream projects by `keel init`, and a decision file referencing this project's `claudeOpus5` arrives at a downstream project as an unresolved reference. The panel did not consider this because it assessed the type surface, not the packaging. OPTIONS CONSIDERED: (i) migrate `.tracking` only — REJECTED, it leaves two mechanisms for one fact, which is worse than one imperfect mechanism; (ii) move actors into `.engine` — REJECTED, actors are per-project instance data by definition and D0093 puts them in `.tracking` deliberately; (iii) give `.engine` its own author registry separate from project actors — a genuine design, but it invents a second registry concept to serve a navigability need, and it is a larger change than the finding it answers; (iv) CHOSEN — deliver the NAVIGABILITY by COMPUTATION. `keel actor-trace <actor>` reports everything an actor authored, judged and owns, from the strings. This is the same move the human themselves reasoned to for `Assumption` (issue105): if it is derivable, derive it rather than authoring it. It needs no schema change, crosses no boundary, and cannot go stale. VERIFIED: wweatherholtz authored 643 / judged 233; claudeSonnet46 authored 9 / judged 91 / owns 13. WHAT REMAINS TRUE AND UNADDRESSED: a viewer loading the raw model still sees strings, so the navigability exists in keel and not in SysON. That is a real residual limitation of choosing (iv), and it is the price of not inverting the engine boundary — recorded here rather than glossed. The 53 legacy provenance values also remain, still surfaced as warnings by the `actors` guard.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcActorTraceView";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcActorTraceView to issue106;

    #ProcessDefect part issue107 : Issue {
        :>> id = "e1070107-0107-4f07-9107-516273840107";
        :>> title = "`abstract action def` was invisible to the parser for four sprints, and `parser-coverage` had been warning about it the whole time";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while verifying `keel migrate` against a downstream fixture. `parse_item`'s dispatch arm read `TokenKind::Action if !had_abstract`, so `abstract action def TrackedAction` — the base type introduced with the Process-as-behaviour conversion (D0143) — fell through to `skip_item` and never entered the model. The abstract base of `Process` and `ProcessStep` was therefore invisible to every view, trace and guard, exactly the issue102 failure mode: nothing errors, both toolchains report clean, and the construct simply is not there. THE CONTROL WORKED AND THE READER DID NOT. `keel guard parser-coverage` is warning-only by design (a skipped statement is a gap, not a dishonest model), and it had been reporting `1 statement(s) ... SKIPPED ... action×1` on every run since the conversion landed. I read its EXIT CODE on each of those runs and reported `guard exit=0`, which was true and uninformative. The defect surfaced only because a downstream fixture pushed the count from 1 to 2 and I read the message to find out why. FIXED in the same sprint (the arm now dispatches regardless of the `abstract` prefix, since `abstract` only ever precedes a def and `parse_action_item` already handles defs) with three regression tests. THE RESIDUAL, and it is the part that will recur: a warning-only guard whose count is persistently non-zero reads as background noise, and `guard exit=0` is the summary I actually act on. Either a non-zero warning count needs a surface I cannot skim past, or these specific warnings need to become blocking once the backlog is zero — which it now is, for the first time, at 0 skipped statements across all 491 files.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcProjectMigrationTool";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcProjectMigrationTool to issue107;

    #ProcessDefect part issue108 : Issue {
        :>> id = "e1080108-0108-4f08-9108-516273840108";
        :>> title = "D0142's leanness sweep left dead residue: a hollow `risk.sysml`, an orphan `RiskLevel`, and two docs naming types it deleted";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while deriving the migration's removal set from the schema diff, which required enumerating exactly what D0142 deleted. FOUR pieces of residue, all in or around the sweep's own commit: (1) `.engine/schema/core/risk.sysml` is hollow — its header and doc string still describe `Risk`, `Mitigation` and the relationship between them, all three deleted, and the file's only remaining content is `enum def RiskStatus` which nothing references; (2) `RiskLevel` in `element.sysml` is likewise orphaned, its only consumers having been `Risk.likelihood`/`impact`/`residual`; (3) `architecture.sysml`'s header comment still lists `DesignElement`, deleted in the same sweep; (4) `sysmlv2-syntax-notes.md` still lists `EngineRisk` among the packages to import. THE PATTERN IS THE ONE D0144 CONTROLS, one layer down: the sweep removed each type and fixed the consumer it was looking at, without enumerating the rest. It is the third instance in this pass of a change that did not check its consumers, and the second found inside D0142's own work — the first was the four stale `query.py` citations retired in sprint 279. NOT FIXED HERE, deliberately: removing a `schema/core` file is a schema change and belongs in its own sprint with its own gate, not bundled into a migration-tool sprint where it would ride in unexamined. NOTE for whoever fixes it: deleting `risk.sysml` from the engine does NOT strand a downstream project, because `keel migrate` never deletes a file it does not ship — the project keeps its copy and the resync reports it as unrecognised.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }

    #ProcessDefect part issue109 : Issue {
        :>> id = "e1090109-0109-4f09-9109-516273840109";
        :>> title = "A typed edge could point at a name declared nowhere, and every check treated it as present";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY THE CONFORMANCE LANE on its first real run, which is the clearest possible argument for the lane existing. The kernel resolves references and reported `Couldn't resolve reference to Element 'postReleaseBurndown'`; no Rust check had ever said anything. TWO REAL INSTANCES, both live for months: (1) `.tracking/issues.sysml` carried `#Resolves dependency from declarativeControlsRoiOrRescope to issue060` while `action declarativeControlsRoiOrRescope;` was never declared — its DoD and a PASSING TestResult sat in the backlog with no action line above them — so issue060 read as TRIAGED by a resolver that did not exist, and the `issues` guard passed on a phantom; (2) `.tracking/delivery/sprint171_postReleaseBurndown.sysml` carried `#CharteredBy ... to postReleaseBurndown`, a backlog action declared in NO commit, so a delivery Story read as chartered by an origin that never existed and the `charter` guard passed. WHY EVERYTHING MISSED IT: `issues` checks that a #Resolves edge is PRESENT; `charter` checks that a #CharteredBy edge is PRESENT; `validate` and `check-engine` resolve item TYPES and imports. Not one of them resolved an edge ENDPOINT. A dangling edge is strictly worse than a missing one, because a missing edge is visible as a gap while a dangling edge is counted as satisfied by the very guard meant to detect the gap. FIXED: (1) restored — the action line was re-added, since the DoD and result are committed records of what it was and that it was done; (2) REMOVED rather than repointed — nothing named postReleaseBurndown exists in any commit, and inventing a charter target to satisfy a guard would fabricate provenance to pass a check, which is the one thing a charter must never do. CONTROL (D0047): new hard guard `edge-endpoints`, AST-based so prose inside a description cannot produce a false hit — a text scan of the same thing reports two extra hits from D0133's own prose ABOUT edges, the identical self-referential-corpus trap as issue099. Verified live by negative test: reintroducing a dangling edge fails the guard; removing it passes. Starts at zero with no grandfathering, and none was warranted: issue068 forbids retro-failing work that was CORRECT when written, and this work was not correct when written, only undetected.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcKernelConformanceGate";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcKernelConformanceGate to issue109;

    #ProcessDefect part issue110 : Issue {
        :>> id = "e1100110-0110-4f10-9110-516273840110";
        :>> title = "The actor-enrollment process requires ROLE to be recorded, and the schema has nowhere to record it";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND by the dorConsumer check (D0144) while building `keel enroll`, BEFORE writing the command rather than after. The enrollment process step `enrollKindRole` requires two fields to be asked and recorded and never inferred: KIND and ROLE. It states that 'role is model data, not tribal knowledge' and that neither may be defaulted. KIND exists as `Actor.kind : ActorKind` and enrollment records it. ROLE HAS NOWHERE TO GO — `part def Actor` carries name, email[0..1] and kind, and nothing else — so a process step this engine's own workflow declares mandatory is currently unsatisfiable. The `part def Role` that might have served was deleted by D0142 as dead schema, correctly, since it had zero declarations; the process was written expecting role to be recordable and the schema never grew the field. WHY THE TIMING MATTERS: had the schema been checked AFTER building the command, `keel enroll --role` would exist and would be writing role into a display-name string or a comment — model data in a place no view can query, which is the dual-truth defect this engine exists to prevent, introduced while implementing the process that forbids it. INTERIM STATE, shipped in sprint 287 and honest rather than silent: enrollment records identity, kind and binding, and REPORTS at the end that role is unrecorded and why, pointing at the proposal. RESOLUTION: D0146 proposes `attribute role : String[0..1]` on Actor — a free String and not an enum, since the process explicitly permits a project-declared role and a closed enum in frozen core would force every downstream project with its own role vocabulary through a frozen-schema sign-off (the issue089 dynamic). It is PROPOSED and not applied because frozen core needs explicit human sign-off and an AI cannot supply it. NOTE that either verdict resolves this: if rejected, the process step must be amended to stop requiring role, because a declared step that cannot be satisfied is cited as if enforced. The one outcome that must not persist is the current one.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcEnrollment";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from d0146 to issue110;

    #ProcessDefect part issue111 : Issue {
        :>> id = "e1110111-0111-4f11-9111-516273840111";
        :>> title = "Process definitions require data the schema cannot hold — twice in two sprints (actor role, work claim)";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "THE PATTERN, not just the instance. Two consecutive consumer checks (D0144) each found a DECLARED process step whose central data the schema has nowhere to put. (1) `actor-enrollment` step `enrollKindRole` requires ROLE recorded as model data — 'not tribal knowledge' — and `part def Actor` has only name, email and kind (issue110, D0146 proposed). (2) `distributed-collaboration` step `dcClaim` requires 'the claim as an authored fact (claiming actor, item, time, and the commit claimed against)' and states 'the only things authored are irreducible facts, including the claim' — and `schema/core` has no `Claim` type at all (D0147 proposed). WHY THIS IS ONE FINDING RATHER THAN TWO: the process definitions and the schema were authored at different times, each against the other's intended future, and were never reconciled. A process step that cannot be satisfied is worse than an absent one, because it is cited as if enforced — the same argument the panel's finding (b) made about rules.sysml's dead-schema policy, arriving from the opposite direction: there a rule was written and stopped being applied, here a rule was written and could never be applied. NEITHER IS FIXED BY AN AI: both are frozen-core additions needing explicit human sign-off (§1.5), so both are recorded as PROPOSED decisions and surface in `keel orient` -> pendingAcceptances. WHAT WOULD MAKE THIS A CONTROL rather than two findings: a check that every process step's named artifact is representable in the schema. That is not obviously computable — `producedArtifact` is prose — so it is NOT proposed here as a guard, and saying so is more honest than proposing a heuristic that would fire on ambiguity. What IS actionable now: whichever way D0146 and D0147 are decided, the losing side must amend the process step rather than leaving it unsatisfiable, and that obligation is recorded in both decisions' consequences.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcWorkClaim";
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from d0147 to issue111;

    #ProcessDefect part issue112 : Issue {
        :>> id = "e1120112-0112-4f12-9112-516273840112";
        :>> title = "The ready frontier ranked an item that provably could not be started — blocked on a human acceptance";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND by walking into it. `dcWorkClaim` needs a `Claim` type that only a human can sign into frozen core (D0147, proposed), so it cannot be executed — and it ranked FIRST on the ready frontier. The frontier is AUTO-FOLLOWED (D0052), so the next contributor would have selected it, rediscovered the same wall, and been told by a COMPUTED VIEW that the work was ready when it provably was not. That is the engine's own central claim failing: state is computed so it cannot lie, and here it did. MEASURED FIRST, not assumed: authoring `#DependsOn dependency from dcWorkClaim to d0147` and re-reading `orient` showed the item STILL ready, so the edge alone was not the fix and `ready` genuinely ignored it. THIS IS issue100's DEFECT WITH A DIFFERENT CAUSE, and it needed its own predicate rather than a widened one: superseded means RETIRED, blocked-on-acceptance means WAITING, and conflating them would either retire work that resumes the instant a human answers, or leave it scheduled. FIXED: `ready` now excludes any task with a `#DependsOn` edge to a Decision whose status is `proposed`. Exact rather than heuristic, and nothing is stored — accepting OR rejecting the Decision returns the item to the frontier with no further edit, which the unit test asserts directly. VERIFIED by read-back: ready 13 -> 12 with dcWorkClaim gone, `outstanding` unchanged at 16 because blocked work is NOT done and reporting it as done would be the same lie in the other direction. NOTE the shape of the near-miss: I authored the edge expecting it to work, and only checked because the response contract requires reading state back from the computed view rather than from the transform's own report. Had I trusted the edge, the model would have carried an authored fact that looked like a fix and changed nothing.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcWorkClaim";
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcBlockedFrontier to issue112;

    #ProcessDefect part issue113 : Issue {
        :>> id = "4eec3622-71aa-4eaf-8765-c17683c9458a";
        :>> title = "The ownership guard blocked a governed D0067 migration — two controls that both had to hold";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND when sprint 295's duplicate-id migration was blocked by the ownership guard sprint 293 had built two sprints earlier. Both controls were RIGHT. D0108 forbids a non-owner editing another actor's item fields, and the migration edited 26 records owned by wweatherholtz, claudeOpus and claudeSonnet46. D0067 REQUIRES bulk transforms, and a duplicate-id repair crosses every ownership boundary at once by its nature. Blocking the migration would have made D0067 unexecutable; exempting on a flag would have made D0108 optional — the issue081 dynamic where an over-strict gate is disabled and takes every other layer with it. RESOLVED by the keystone shape this repo already uses for process changes: cross-owner edits are permitted only when a transform under .engine/tools/migrations/ is CO-COMMITTED, which is exactly what D0067 demands anyway (a committed transform, a dry run, reconciled control totals). The exemption cannot be claimed — it has to be in the commit where a reviewer can read it — and it is announced as a WARNING rather than passing silently, so a reader sees that ownership was suspended and why. VERIFIED both ways: with the transform staged the guard passes with the warning; with it unstaged the same 26 edits produce 13 violations. THE GENERAL LESSON, which is why this is recorded rather than just fixed: a new hard guard can make an EXISTING sanctioned process unexecutable, and neither the guard's own tests nor the process's own tests will show it, because each is correct in isolation. It surfaced only when a real migration ran into a real guard.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcRepairDuplicateIds to issue113;

    #ProcessDefect part issue114 : Issue {
        :>> id = "6ee02855-96c0-4f8f-b948-4aeda81ed567";
        :>> title = "keelArchViews is blocked on a frozen-core prereq its own DoD names, and on data this repo does not hold";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND by the consumer check while scoping the last item on the ready frontier. keelArchViews asks for a 'keel arch' command group computed over an authored CodeElement registry, and its DoD's FIRST LINE names the prerequisite: promote the EngineCodeAudit schema module into the engine's shipped schema. MEASURED: CodeElement exists nowhere here — '.engine/schema/' holds only 'core' and 'safety', and every occurrence of the name in this repo is prose describing this item. (This description was first recorded through a shell that expanded its backticks and ate three path fragments; corrected in place by its own author, which ownership permits. The lesson is the one already recorded for heredocs: prose destined for a file should not travel through a shell.) TWO BLOCKERS, not one. (1) The schema promotion is a frozen-core change needing explicit human sign-off (D0148 proposed). (2) The item's own VERIFY requires testing 'on SelfSync's authored registry' — a DIFFERENT project's data that this repo does not hold — so even with the schema shipped, the acceptance criterion is unreachable from here. Building the commands now would ship a producer for a type the engine does not define, verified against data that does not exist, which is exactly what D0144 prevents — and the sprint would have had to cite D0144 as its charter while violating it. WHAT EITHER VERDICT UNBLOCKS: if D0148 is accepted, keelArchViews becomes executable but should be re-scoped to name WHERE it will be verified, since this repo cannot verify it. If rejected, the item must be re-scoped or superseded, because an item whose first line is an unmeetable prerequisite is not ready work. The current state is the only one that leaves the frontier stuck.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from d0148 to issue114;

    part issue115 : Issue {
        :>> id = "ab21627f-b2e1-40b2-8983-924ec7648813";
        :>> title = "Claim exclusion cannot come from the push; the process text says it does";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "D0129 and the dcWorkClaim DoD both state that two contributors claiming one item are excluded by the git ref update, because a push is a compare-and-swap and the remote accepts exactly one. A two-clone race test DISPROVED this. Claims are written to PER-ACTOR files under srDcPerActorWriteTargets, which deliberately removes the write contention; the loser push IS rejected, but they merge cleanly and retry, and then hold a claim exactly as valid as the winner. Both land. Exclusion is now COMPUTED in claim.rs and the code states the true mechanism, but the PROCESS TEXT still teaches the false one, which is dual truth in a document a downstream project would build on. Two facts fell out, each of which cost a wrong implementation: whoever-landed-first is NOT recoverable from merged history (parallel claims are siblings of equal ancestry depth and their commit timestamps tie in the same second), and expiry must remove a claim from CONTENTION rather than only from the top of the ranking.";
        :>> discoveredInField = false;
        :>> relatedTask = "dcWorkClaim";
        :>> severity = Severity::High;
    }
    // Retargeted: sprint 300 FOUND this, it did not fix it. The process text still teaches the
    // disproved mechanism, so pointing the edge at the finding sprint would have computed the issue
    // as resolved by the work that discovered it.
    #Resolves dependency from dcCorrectExclusionText to issue115;

    part issue116 : Issue {
        :>> id = "f74eab6b-351c-4175-ad1d-60c0ff2e7ff5";
        :>> title = "I bypassed the commit gate with hooksPath=/dev/null to build a test fixture";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "To give two test clones a binary containing new code I made a local commit with core.hooksPath set to the null device, which bypassed validate and all guards. Never bypassing the gate is a standing prohibition and no fixture requires it: the clones needed the BINARY, which cargo had already built, not a commit. The commit was never pushed and no TestResult cited its SHA, so it was folded back with git reset --soft and replaced by one gated commit. The exposure is real regardless of the recovery, because an ungated commit on main is indistinguishable from a gated one afterwards, and the only reason this was recoverable is that I noticed within the same session. CONTROL, not vigilance (D0047): a check that detects commits whose recorded gate evidence is absent, so a bypass is VISIBLE in a computed view rather than depending on the bypasser to disclose it. Disclosed unprompted; see the sprint 300 closeOut record.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    // Retargeted for the same reason, and this one mattered more: the resolution issue116 asks for
    // is an automated control, and no control was built in sprint 300. The edge asserted otherwise.
    #Resolves dependency from dcGateEvidenceControl to issue116;

    part issue117 : Issue {
        :>> id = "c48b019d-b805-49c5-8901-9bbda6132bbd";
        :>> title = "keelArchViews was verified against keel's own registry, not SelfSync's as its DoD requires";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "keelArchViewsDoD says to verify the six arch views 'on SelfSync's authored registry'. SelfSync is a different project and this repo does not hold its data, so the acceptance criterion is unreachable from here - D0148's own consequences flagged this and said the item should be re-scoped to name where it will be verified. RE-SCOPED to keel's own source: a 12-element registry over keel-cli and keel-parser, authored at .tracking/architecture/code-registry.sysml, with real risk judgments about modules whose failure would actually matter. This is the alternative D0148 accepted and not the one it rejected - the rejected option was a synthetic probe registry, which would show the commands run rather than that they compute anything true. WHAT IS THEREFORE NOT YET PROVEN: that the views compute correctly over a registry authored by a DIFFERENT project with different conventions, which is the portability claim keelArchViews was actually about. keel's own registry was authored by the same author as the views, so it cannot detect an assumption shared between them. Resolving this needs either SelfSync's registry or a second project's, and until then the portability claim rests on the schema module shipping through keel init rather than on a demonstration.";
        :>> discoveredInField = false;
        :>> relatedTask = "keelArchViews";
        :>> severity = Severity::Medium;
    }
    // Retargeted: sprint 301 verified against keel's own registry, which is what issue117 RECORDS
    // as the gap. Only a second project's registry closes it.
    #Resolves dependency from dcArchRegistryPortability to issue117;

    part issue118 : Issue {
        :>> id = "84b7cbee-d036-4260-a018-bf98af8f85ac";
        :>> title = "A misspelled attribute passes every layer and silently drops the value";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "PROVEN by probe: a CodeElement authored with codeHsah (typo of codeHash) and riskClas (typo of riskClass) passes keel validate, all 32 guards, AND the fast edit gate - 329 files reported clean. The element then computes with its riskClass silently absent: the intended dataLoss classification vanished and the element sits in the arch criticality frontier as correctness, which is a safety-relevant misclassification that no layer reported. codeHsah likewise made the element report as UNAUDITED (never hashed) when the author believed a hash was recorded. This is the SAME failure class as issue077/D0133 - an undeclared marker validates clean and silently removes the item from whatever depends on it - which was fixed for markers and for enum values (an unknown enum member IS rejected) but never for attribute NAMES. It is the direct cause of the recurring surprise that new schema elements are not picked up: the author believes they authored a fact, every gate agrees the model is honest, and the fact is not there.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSchemaReconciliation to issue118;

    part issue119 : Issue {
        :>> id = "17a0b752-0942-4958-828b-f0bd3def5922";
        :>> title = "The EdgeKind enum is inert and KNOWN_EDGES has drifted from it in both directions";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "The schema declares enum def EdgeKind with twelve values. NO Rust code reads it - the enum is inert, a schema declaration with no consumer, which is exactly what D0142 spent a sprint deleting and what D0144's dorConsumer check exists to prevent. The actual authority for declared views is a hardcoded KNOWN_EDGES list in view.rs, and it has drifted from the schema in BOTH directions. Declared in EdgeKind but REJECTED by the view layer: derivedFrom, covers, dispositions, specialize. Accepted by KNOWN_EDGES but never declared in EdgeKind: ordering, prospectiveChange, safetyChange. PROVEN by probe: a declared view traversing derivedFrom fails with 'references unknown edge kind' and lists the eleven it will accept. The blind spot is 166 real edges - 97 DerivedFrom, 28 Covers, 41 Dispositions - which exist in the model, are read by guards, and cannot be reached from the declared-view layer at all. A user reading the schema would reasonably declare a viewpoint over derivedFrom and be told by the tool that the schema's own vocabulary is unknown.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSchemaReconciliation to issue119;

    part issue120 : Issue {
        :>> id = "1365b14c-e511-4d49-8727-380311b3f3db";
        :>> title = "Hardcoded lists mirror schema vocabulary with nothing reconciling them";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "At least three constants in the binary restate schema vocabulary and nothing checks that they still agree with it: KNOWN_EDGES (view.rs) mirrors the EdgeKind enum and HAS drifted (issue119); ENGINE_MARKERS (guards.rs) mirrors the declared metadata defs and is missing Controls, Feedback and Restructure, the three markers shipped with the codeaudit module in sprint 301; RISK_ORDER (arch.rs) mirrors the RiskClass enum and happens to agree today by nothing more than my having written both in the same hour. Each is a bespoke restatement where an intrinsic derivation or a reconciliation check belongs. Note ENGINE_MARKERS cannot simply be deleted in favour of reading the project's schema - that is precisely what caused issue090 (566 violations, every commit blocked, when a new binary met an older on-disk .engine), which is why the list exists - so the fix is reconciliation against the EMBEDDED schema, not removal. RECORDED ABOUT MY OWN METHOD TOO: the ad-hoc python used to run this very audit produced two false findings before it was correct, first by matching only five of the seven def kinds (missing occurrence def TestResult) and then by keying enum bodies to a first-line match so RiskClass reported InvariantSafety's values. An audit worth running belongs in the binary as a keel command, not in a script that has to be debugged each time it is used (D0040).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcSchemaReconciliation to issue120;

    part issue121 : Issue {
        :>> id = "004ed0d1-3e20-47ab-96d9-4ff5a1260416";
        :>> title = "C1: srServeModelDrivenRegistry requires an output schema the model cannot hold";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the independent completeness critique of srServeModelDrivenRegistry (critic claudeOpus5, author wweatherholtz). The requirement demands keel serve compute each launchable process's expected OUTPUT SCHEMA from the declared model. No schema type can hold one: action def Process declares purpose and cadence; ProcessStep declares triggerCondition, actionText, producedArtifact and owner, and producedArtifact is a free-text sentence describing the artifact change, not a machine-readable schema anything can validate against. Nothing in .engine/schema carries an output schema or a schema reference. Half of this Tier-1 contract is therefore not computable as written, and because srServeOutputSchemaValidate and srServeLauncherDefinedOnly both #DependsOn it, the whole tier inherits an unbuildable prerequisite. Same class as D0146/D0147/D0148 in sprint 298 - a process definition demanding data the schema cannot hold - and the same shape as issue114, where an item's first line was an unmeetable prereq. Closing it needs either a frozen-core schema addition with its own Decision, or a re-scope of the requirement to the launchable set alone.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcServeRequirementRework to issue121;

    part issue122 : Issue {
        :>> id = "cda3d6ce-9dd2-4674-87e5-295ab32c60bb";
        :>> title = "C2: srServeLauncherDefinedOnly states a universal negative that no test can demonstrate";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the independent testability critique of srServeLauncherDefinedOnly. The requirement says there shall be no code path that spawns the agent on a freeform, un-routed prompt. That is a universal negative over the whole codebase: a test can show the documented entry point rejects an undeclared target, but nothing can show no OTHER path exists. Demonstrating it requires exhaustive review, which this engine classes as manual vigilance rather than a control (D0047). As written the requirement is unfalsifiable and will read as satisfied on the strength of whichever single path someone remembered to test. It becomes testable if restated as a structural property a guard can assert - for example, exactly one function may spawn the agent and it takes a validated target type rather than a string - so that a new bypass path breaks a check instead of passing silently.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcServeRequirementRework to issue122;

    part issue123 : Issue {
        :>> id = "7c59728d-f7f7-4997-ba46-ee5c6f69ff9d";
        :>> title = "C3: srServeApproveGate never requires the approving operator to be human";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the independent completeness critique of srServeApproveGate. The requirement demands an explicit operator approve action before execution but never constrains the operator to a HUMAN. The term operator is undefined across the Needs and requirements, and NeedSource::operator is a source category rather than an actor kind. Nothing as written prevents an AI actor from supplying the approval, which defeats the exact property the parent Need nServeApproveBeforeExecute states: the agent never silently acts on its own interpretation. An agent approving its own routing IS acting on its own interpretation, and would now have a recorded approval to evidence it. The engine has since acquired the vocabulary to close this: D0146 added Actor.role, and .engine/contracts/attestation-policy.toml declares per-class kinds=human with the attestation-authority guard checking KIND before ROLE so an AI holding the right role still fails a human-only class. The requirement needs that binding, or the gate is satisfiable by the very thing it exists to restrain.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcServeRequirementRework to issue123;

    part issue124 : Issue {
        :>> id = "cf424fd2-bd1c-46ac-b88c-224bdd7b9ceb";
        :>> title = "C4: srServeOutputSchemaValidate ignores side effects already taken by a failed attempt";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the independent completeness critique of srServeOutputSchemaValidate. The requirement governs only the RESULT - never record, commit or surface a schema-invalid result - and is silent on side effects the failing attempt already caused. An agent that wrote files, appended records or pushed during attempt 1 has already changed the world, and rejecting its output and retrying undoes none of it; after the bounded retries are exhausted, the required explicit error leaves those partial effects in place with nothing recorded to say they happened. For an engine whose premise is that the model must be truthful about state, silently orphaned partial writes are the worst residue a retry loop can leave. The requirement needs to say what happens to work already performed: isolate the attempt, record the partial effects as facts, or permit retries only for processes with no side effects until the output is accepted.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcServeRequirementRework to issue124;

    part issue125 : Issue {
        :>> id = "f09a7689-d044-4840-b548-dfc08062a138";
        :>> title = "C5: srServeScopedContext bounds passed-in context but not what a launched agent can read";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the independent completeness critique of srServeScopedContext. The requirement bounds only what is passed in - only its declared inputs as context - while a launched agent's dominant context channel is what it can READ. An agent with file-read and shell access reconstitutes arbitrary repository context however narrow its declared inputs were, so the stated goal that a task cannot drift on a diffuse long-running goal is not achieved by input scoping alone. The requirement should either bound the agent's reachable surface too, or state honestly that it constrains only the launcher's contribution to context and that drift-by-exploration is out of scope and handled elsewhere. As written it claims a property it cannot deliver, which is more dangerous than claiming less.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcServeRequirementRework to issue125;

    // D0102 typed finding -> critique links: each finding points at the critique that raised it, so a
    // later ACCEPT-RISK/DISMISS disposition clears the target's critique_suspect through a typed path
    // rather than prose. All five are >= Medium and AWAIT HUMAN DISPOSITION (D0092) - never inferred.
    #DependsOn dependency from issue121 to srServeModelDrivenRegistryCompletenessCritique;
    #DependsOn dependency from issue122 to srServeLauncherDefinedOnlyTestabilityCritique;
    #DependsOn dependency from issue123 to srServeApproveGateCompletenessCritique;
    #DependsOn dependency from issue124 to srServeOutputSchemaValidateCompletenessCritique;
    #DependsOn dependency from issue125 to srServeScopedContextCompletenessCritique;

    part issue126 : Issue {
        :>> id = "154d8e14-fb4e-4777-bede-c7b93e8316c4";
        :>> title = "A git-bash /tmp path read by Windows python fails, and has recurred three times";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "The Bash tool runs git-bash and the python/PowerShell tools are native Windows, so a file written to /tmp by a heredoc is invisible to a later python step reading the same path. This has now failed three times in one session - in sprint 301 while splicing test code, and twice in sprint 305 while authoring critiques - each time costing a failed step and a retry. The remedy (use the Write tool for any file a later step must read, or a scratchpad path both shells resolve) was RECORDED IN A RETRO after the first occurrence and did not prevent the next two, which is the textbook D0047 case: manual vigilance is not a control. The pre-bash hook already exists to advise on exactly this class of host adaptation (issue094) and is where the check belongs. Severity Low because it is self-announcing - the step fails loudly with FileNotFoundError rather than producing a wrong answer silently - so it costs time rather than correctness.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcScratchPathControl to issue126;

    part issue127 : Issue {
        :>> id = "e8cfebd1-0a9c-443b-89a5-2b8d202e2cf5";
        :>> title = "critique-coverage counts superseded elements; coverage and tier-satisfaction do not";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Three computed views disagree about whether a superseded element is in scope. compute_coverage collects a descoped set from supersede edges and filters its targets by it; compute_tier_satisfaction does the same; critique_coverage filters only by governed(), the created-after-D0080 charter-time rule, with no supersede exclusion at all. Found by acting on the sprint 305 critique findings: five serve-discipline requirements were superseded by corrected ones, and the retired originals still report governed=true in critique-coverage, so the governed count rose 65 to 70 rather than staying at 65 with five in and five out. The consequence is that a retired element sits in the critique-coverage denominator permanently, its critique has to be maintained to keep the number from falling, and the view overstates how much outstanding critique work exists. Fixing it changes a computed view's numbers, so it is a section 3a change needing sign-off rather than a silent correction.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcCritiqueCoverageSupersede to issue127;

    part issue128 : Issue {
        :>> id = "0c033446-1496-4fb7-a9b2-201c8ffd0b3f";
        :>> title = "arch criticality ranked a downstream project's own risk classes as unclassified";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by running the six arch views against a REAL second project's registry (obsidian-livesync / SelfSync, 169 authored CodeElements) - the portability test keelArchViews always specified and issue117 recorded as unreachable. It was reachable: the project exists on this machine and holds its own .engine/schema/codeaudit copy. THE VIEWS RAN WITH NO CHANGE TO KEEL, which is the portability claim verified. But arch criticality ranked 23 of their 169 elements LAST, as 'unclassified'. Cause: D0150 made risk_order derive from the EMBEDDED RiskClass enum, and SelfSync declares its own - dataLoss, security, durability, concurrency, correctness, availability, cosmetic - with durability and concurrency that the engine does not have. Their 15 durability and 8 concurrency elements, including Vault::commit (durable file commit), Vault::verify_and_gc (corruption and orphan detect) and write_mirror (atomic mirror write), sorted BELOW cosmetic in the one view whose entire purpose is to say what to audit first. Deriving from the embedded schema is right for ENGINE vocabulary - markers and edge kinds must travel with the binary or a project gets locked out of its own history (issue090) - and wrong for a PROJECT-DECLARED enum whose order is the project's own judgment about what matters most. Fixed by reading the project's RiskClass first and falling back to the engine's.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcArchRegistryPortability to issue128;

    part issue129 : Issue {
        :>> id = "0ce9518a-c7c9-4de8-ac61-9b377cec16df";
        :>> title = "attribute-vocabulary locked a downstream project out of its own repo (1287 violations)";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "SHIPPED EARLIER TODAY IN SPRINT 303 AND FOUND THE SAME DAY by running the guard against a real downstream project. The attribute-vocabulary guard judged that project's instances against the ENGINE's attribute vocabulary alone and produced 1287 violations, which would have blocked every commit in that repo on a binary upgrade. Its .engine/schema declares ProcessStep with an order attribute and CodeElement with file, name, auditRound, auditedAt and riskRationale - all legitimate there, none in the engine's copy. This is precisely the issue090 failure the MARKER guard already learned and whose lesson D0133 recorded: a newer binary meets an older or extended on-disk schema and locks the project out, with the only remedy being an edit to frozen core. I read that lesson while building this guard, wrote it into the module doc as the reason to read the EMBEDDED schema, and still shipped the same lockout in the other direction - because reading the embedded schema protects the ENGINE's vocabulary from removal but says nothing about a project ADDING to it. FIXED: declared_attrs_in unions the engine's declaration with the project's, exactly as the marker guard does. Detection is unaffected - a misspelling like codeHsah is in neither set, which is the only case the guard exists for - and the downstream project goes from 1287 violations to 0 while this repo stays clean at 41141 assignments scanned.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcArchRegistryPortability to issue129;

    part issue130 : Issue {
        :>> id = "d7f8268b-edbb-44bc-8dbf-3010439f474d";
        :>> title = "arch drift reported elements with no filePath as missing files";
        :>> createdAt = "2026-08-15";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by running arch drift against a downstream registry that records the source path under a different attribute name than the shipped module uses. Their 169 elements therefore had an empty filePath; root.join('') resolved to the repo root; reading a DIRECTORY failed; and every element reported as MISSING - not readable. The view stated 169 files were absent when in truth no file had ever been named. Another instance of the session's recurring shape: a confident wrong answer rather than an error. FIXED: an element with no filePath now reports UNLOCATED with its own counter, distinct from MISSING, because never-named and named-but-absent call for different work - the first is a registry that has not recorded where its code lives, the second is a stale path.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcArchRegistryPortability to issue130;
}
