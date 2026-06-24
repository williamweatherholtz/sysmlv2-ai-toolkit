// =====================================================================
// ProjectIssues — open issues tracked against the engine.
// Convention: author a `part <name> : Issue { ... }` for each open
// issue. The `relatedTask` field names the backlog action that should
// address it. `discoveredInField = true` for issues found in production use.
// Surface with: python .engine/tools/query.py issues
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
        :>> id = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";
        :>> title = "status dashboard lacks process traceability view — no visibility into which processes sprints used, subversions, or evolution";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> description = "The dashboard shows workflow diagrams but does not trace which processes each sprint actually followed, when/how processes were subverted, how processes evolved over time via improvement items, or which backlog items link to which decisions and skill changes. User request: full system traceability — sprint-to-process linkage, process evolution timeline, item-to-decision trace, subversion highlighting.";
        :>> discoveredInField = true;
        :>> relatedTask = "baselineView";
    }

    part issue003 : Issue {
        :>> id = "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e";
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
        :>> description = "GENERAL process defect behind issue046: the engine has no forward control that a Decision which charters delivery work, or a shipped user-facing capability, traces (satisfy/derive) to a Need. The orphans view flags a Need with no Requirement, but NOTHING flags a capability with NO Need at all — so a whole feature (serve, issue046) can be built requirement-less and the engine cannot see it. Requirement-rootedness should be a KEY, un-ignorable element of the process, not something accidentally skippable (human directive, 2026-06-24). Per D0047 (corrections become guards) the obligation is a PERMANENT AUTOMATED GUARD, not vigilance. Resolver requirementRootednessGuard: record a Decision on the rule + scope (incl. exemptions like pure engine-internal refactors), then build it into keel guard (and/or an orphans-style view) that fails on an un-rooted capability.";
        :>> discoveredInField = false;
        :>> relatedTask = "requirementRootednessGuard";
        :>> severity = Severity::High;
    }
    #Resolves dependency from requirementRootednessGuard to issue047;
    #ProcessDefect dependency from issue047 to d0047;
}
