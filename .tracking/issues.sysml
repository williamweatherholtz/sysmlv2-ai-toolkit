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
        :>> authoredBy = "claudeSonnet46";
        :>> description = "The authoredBy and judgedBy fields in Element and TestResult are raw String attributes. They name actors by convention but are not validated against the ProjectActors registry, so typos or stale names are silently accepted.";
        :>> discoveredInField = false;
        :>> relatedTask = "authoredByRefs";
    }

    part issue002 : Issue {
        :>> id = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";
        :>> title = "status dashboard lacks process traceability view — no visibility into which processes sprints used, subversions, or evolution";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The dashboard shows workflow diagrams but does not trace which processes each sprint actually followed, when/how processes were subverted, how processes evolved over time via improvement items, or which backlog items link to which decisions and skill changes. User request: full system traceability — sprint-to-process linkage, process evolution timeline, item-to-decision trace, subversion highlighting.";
        :>> discoveredInField = true;
        :>> relatedTask = "baselineView";
    }

    part issue003 : Issue {
        :>> id = "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e";
        :>> title = "EngineWork::Release and EngineBaseline::Release are two conflicting Release types";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeSonnet46";
        :>> description = "EngineWork has Release :> Element with membershipRule : String. EngineBaseline has Release :> Element (occurrence def) with commit + purpose. When both are in scope (both imported), type resolution is ambiguous and the pilot kernel picks EngineWork::Release. tracking-template.sysml omits the EngineBaseline import until these are consolidated. Fix: either merge EngineBaseline::Release into EngineWork (drop membershipRule, add commit+purpose) or rename one to avoid conflict.";
        :>> discoveredInField = false;
        :>> relatedTask = "trackedMetadata";
    }

    // ---- 2026-06-16 process step-back findings (D0046 audit program) ----
    part issue004 : Issue {
        :>> id = "c3d4e5f6-a7b8-4c9d-ae1f-2a3b4c5d6e7f";
        :>> title = "JVM kernel leak: every .sysml validation boots a pilot-kernel JVM; orphans accumulate (175 found 2026-06-16)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The Python validators (validate_schema/workflows/instances/tracking) each start the Jupyter SysML pilot kernel (~20s JVM startup). Interrupted runs and pre-commit-hook-spawned kernels do not tear down, leaking JVMs — 175 orphaned java processes (up to ~1GB each) were swept on 2026-06-16. Root cause is structural: nothing enforces the no-kernel Rust path, so the JVM tax is paid on every validation. Resolved when rustToolchainFix makes 'sysmlv2 validate' the default path with the kernel as fallback.";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue005 : Issue {
        :>> id = "d4e5f6a7-b8c9-4dae-bf2a-3b4c5d6e7f80";
        :>> title = "Rust toolchain regressed: sysmlv2 validate fails schema-preload; orient emits retired cursor + phantom tasks + count drift (185/12 vs 81/2)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The Rust CLI (rustS0-S9, all marked done/parity-verified) silently diverged from the Python authority. Three bugs: (1) 'sysmlv2 validate .' reports unresolved type reference TestResult — a schema file fails to parse so EngineVerification never registers; (2) 'sysmlv2 orient .' pollutes the ready-list with raw 'verification ...DoR : Test {...}' blocks parsed as phantom tasks (count 197 vs query.py 87); (3) orient still emits the retired StateCursor (pre-D0045) instead of in_progress_sprints. The 'parity verified' TestResults stayed green because suspicion does not watch the Rust source tree (see issue006).";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue006 : Issue {
        :>> id = "e5f6a7b8-c9da-4ebf-c03b-4c5d6e7f8091";
        :>> title = "Suspicion model ignores the deliverable source tree — code drift never invalidates parity TestResults";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "The engine's suspicion model watches material change to .sysml authored facts over satisfy/:> edges, but nothing invalidates a TestResult when the Rust source it verifies changes. The rustS8/S9 'parity verified' results stayed green while the binary regressed (issue005) — a real traceability hole between the engine model and the deliverable code. Fix: extend suspicion so Rust source-tree changes (by commit/hash) mark the parity TestResults suspect.";
        :>> discoveredInField = true;
        :>> relatedTask = "deliverableSuspicion";
    }

    part issue007 : Issue {
        :>> id = "f6a7b8c9-daeb-4fc0-d14c-5d6e7f809102";
        :>> title = "Antagonistic-critique process has no operationalizing skill and records findings as prose, not tracked Issues/Decisions";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "architectural-critique.sysml defines an 8-step adversarial audit process but (1) has no corresponding skill in .engine/skills/ (violates D0040 recurring-task-skill rule), (2) its last run produced a prose doc in docs/design-history/ rather than tracked Issue/Decision instances, and (3) has no objective metrics. Fix under auditSkill: create+register an architectural-critique skill encoding CAE+GQM+ATAM (D0046) and rewrite the process to record findings as tracked model facts.";
        :>> discoveredInField = true;
        :>> relatedTask = "auditSkill";
    }

    part issue008 : Issue {
        :>> id = "a7b8c9da-ebfc-401d-e25d-6e7f80910213";
        :>> title = "Missing delivery skills: no build/compile or test/verify skill — only repo-push exists";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "AI delivery of the deliverable (the Rust toolchain) is not skill-governed for build or test. repo-push covers commit/push, but there is no skill encoding how to compile (cargo build + validators) or how to test/verify (cargo test + BDD + Python<->Rust parity). Process knowledge for build/test leaks into conversation history. Fix under deliverySkills: create+register build and test/verify skills.";
        :>> discoveredInField = true;
        :>> relatedTask = "deliverySkills";
    }

    part issue009 : Issue {
        :>> id = "b8c9daeb-fc01-4d2e-f36e-7f8091021324";
        :>> title = "repo-push skill contradicts the main-only branch policy (D0014): tells AI to branch off main + open PRs + never commit to main";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> description = "The repo-push SKILL.md mandates a PR/feature-branch workflow ('never commit to main/master', 'branch off main', 'open the PR', writePolicy pr-only) but the project's standing policy is main-only direct commits with an auto-push post-commit hook (D0014, CLAUDE.md §4). An AI following repo-push verbatim would violate the actual workflow. Discovered while building deliverySkills. Fix under deliverySkills: reconcile repo-push to the main-only reality.";
        :>> discoveredInField = true;
        :>> relatedTask = "deliverySkills";
    }

    part issue010 : Issue {
        :>> id = "c9daebfc-012d-4e3f-046f-8091021324a5";
        :>> title = "No guard prevents ceremony gates being skipped or recorded out of order — only surfaced post-hoc by orient/traceability, not prevented";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> description = "During Sprint 16, gate TestResults were recorded Refine->Implement->Review with Standup SKIPPED; the omission was caught only because orient happened to be run (it showed pending=Standup while later gates passed). There is no validator/pre-commit guard enforcing the invariant 'no ceremony gate may pass while an earlier-ordered gate in the same sprint is unpassed.' Views surface subversion (orient pending, traceability orange highlight) but nothing PREVENTS it at author/commit time. This is the same class as the Sprint 14 retro finding (tracked-but-recurring): the durable fix is an automated guard, not manual vigilance. Meta: corrections found mid-work must be logged as bugs AND get a permanent guard, not patched silently.";
        :>> discoveredInField = true;
        :>> relatedTask = "ceremonyGateGuard";
    }

    part issue012 : Issue {
        :>> id = "ebfc012d-3e4f-4051-267f-021324a5b6c7";
        :>> title = "query.py orient undercounts done by 1 (sees 91 tasks vs the structurally-correct 92) — surfaced by the Sprint 17 parity check";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> description = "The Sprint 17 parity check found sysmlv2 orient (Rust) reports 92 tasks (done 85 + outstanding 7), exactly matching the 92 distinct `action <name>;` declarations in .tracking, while query.py orient reports 91 (done 84 + outstanding 7) — query.py silently drops one task. The qualitative sets (in_progress_sprints, ready, suspect, invalidEvidence) are identical between the two. Rust is structurally correct; query.py has a latent discovery/counting bug (likely in %show-based action-def discovery or member regex). Post-D0048 the Rust path is the authority, so query.py is secondary and this is a non-blocking known delta (the parity gate tolerates it). Fix later: pin and correct query.py's dropped task so both agree at 92.";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue014 : Issue {
        :>> id = "012d3e4f-5061-4238-376f-1324a5b6c7e1";
        :>> title = "12 Rust deliverable tasks are suspect: verified against source older than the Sprint 17 parser/orient changes — need re-verification at HEAD";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> description = "Deliverable-suspicion (D0050) now flags rustS1..S9, runtimeParser, writeApi, rustToolchainFix as suspect: their latest passing TestResults were judged at commits BEFORE the Sprint 17 source changes (occurrence-def parse fix, orient rewrite) and the W4 orient.rs change, so their evidence is stale. This is the feature working as intended (issue006 closed), not a defect. Clearing it = re-verify at HEAD (cargo test + clippy + parity_check) and append fresh TestResults. Created by the autonomous retro (D0049), Sprint 19.";
        :>> discoveredInField = true;
        :>> relatedTask = "reverifyDeliverable";
    }

    part issue013 : Issue {
        :>> id = "fc012d3e-4f50-4126-376f-1324a5b6c7df";
        :>> title = "Shared cwd between the Bash and PowerShell tools: a `cd` in one silently breaks later relative-path commands in the other (Sprint 17 avoidable issue)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> description = "During Sprint 17 a `cd .tracking` in a Bash tool call changed the working directory seen by subsequent PowerShell tool calls, so `python .engine/tools/query.py ...` failed with 'file not found' until invoked by absolute path. Avoidable: always use absolute paths for script/file args in shell commands, and never rely on cwd persisting (or rely on it only within a single call). Preventing change applied: CLAUDE.md §6 note. Discovered by the autonomous retro (D0049).";
        :>> discoveredInField = true;
        :>> relatedTask = "rustToolchainFix";
    }

    part issue011 : Issue {
        :>> id = "daebfc01-2d3e-4f40-156f-91021324a5b6";
        :>> title = "Sprint ceremony skills are invoked by convention only — the sprint-review Phase-3 transcript scan can be (and in S16 was) skipped, so process-improvement findings are missed";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> description = "sprint-review SKILL.md Phase 3 (transcript review) is a mandatory process-improvement scan whose detection cues include 'Missing guard — a check that SHOULD have been automatic was done manually' — i.e. it is designed to catch exactly the issue010 class. But nothing ENFORCES that the skill is actually invoked before the review gate TestResult is recorded. In Sprint 16 the review gate was recorded WITHOUT running the Phase-3 scan, which is why issue010 (Standup-skip) and issue009 (repo-push contradiction) were found ad hoc rather than by review. Gap: ceremony-skill invocation is discipline, not control. Fix (reviewInvocationGuard): make the review gate require recorded transcript-scan evidence (improvement items, even if 'none material'), so a review gate without a scan artifact is incomplete; guard it. Guard DESIGN to be dispatched at the next retro (per D0047) since it needs human input.";
        :>> discoveredInField = true;
        :>> relatedTask = "reviewInvocationGuard";
    }
}
