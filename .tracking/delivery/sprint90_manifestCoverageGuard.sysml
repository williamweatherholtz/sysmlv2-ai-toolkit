// =====================================================================
// ProjectDeliveryS90 — Sprint 90: manifestCoverageGuard (resolves issue033; hardens D0050). The
// d0050 finding: the deliverable-suspicion manifest is a hand-maintained allow-list — a dead/renamed
// entry or an unlisted deliverable task silently drops source-drift coverage. Add the manifest-coverage
// GUARD (enforced): every manifest entry must name a real task + real paths (violation); deliverable-
// pattern tasks not listed warn. ENGINE HARDENING (campaign batch-2 ACT disposition).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS90 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from manifestCoverageGuardStory to d0050;

    part manifestCoverageGuardStory : Story {
        :>> id = "63940516-2c7d-4e8f-b090-51627384bb01";
        :>> title = "Sprint 90: manifestCoverageGuard (2 pts) — enforce deliverable-manifest validity + flag unlisted deliverable tasks (resolves issue033, hardens D0050)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS90 {
        action storyManifestCoverageGuard;
        verification storyManifestCoverageGuardDoD : Test { :>> id = "63940516-2c7d-4e8f-b090-51627384bb02"; :>> method = VerificationMethod::test; :>> procedureText = "Resolve issue033 (d0050 finding: deliverable-manifest is a hand-maintained allow-list; silent coverage loss) via the backlog action manifestCoverageGuard. guards.rs manifest_coverage + parse_manifest + declared_task_names: VIOLATION when a manifest entry names a task that no longer exists or a path that no longer exists (dead entry = silently dropped suspicion); WARNING when a declared backlog task whose name looks deliverable-dependent (DELIVERABLE_TASK_HINTS: rust/Parser/writeApi/runtimeParser/specVersion; story* excluded) is not manifest-listed. Add to GUARD_NAMES (11, ENFORCED) + run_one; regression test manifest_parses_per_task_entries. VERIFY (live): `sysmlv2 guard manifest-coverage` = 13 scanned, 0 violations, 2 warnings (rustS0Workspace + rustFailLoudLints flagged for confirmation); clippy --workspace --all-targets -D warnings + cargo test (41 lib) + rust validate (102) + guard all (11) green; doc-sync CLAUDE.md (ten->eleven guards). append-result marked manifestCoverageGuard done -> issue033 RESOLVED (open 4->3)."; }
        part storyManifestCoverageGuardDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b090-51627384bb03"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b577f9c"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification manifestCoverageGuardRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b090-51627384bb04"; :>> title = "Sprint 90 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: engine hardening — work campaign-batch-2 ACT disposition issue033 (resolver manifestCoverageGuard). The d0050 finding: deliverable-suspicion only guards manifest-listed tasks, a manual allow-list that can silently lose coverage. Plan: a guard that keeps the manifest valid (no dead task/path entries = violation) + flags likely-unlisted deliverable tasks (warning). estimatedPoints=2. #CharteredBy d0050. DoD accepted."; }
    part manifestCoverageGuardRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b090-51627384bb05"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b577f9c"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification manifestCoverageGuardStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b090-51627384bb06"; :>> title = "Sprint 90 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: guards.rs parse_manifest (task: NAME | paths) + declared_task_names (scan action <name>; in backlog+delivery, skip action def + story*) + manifest_coverage guard; GUARD_NAMES 10->11 + run_one; regression test. The 2 expected warnings (rustS0Workspace, rustFailLoudLints) are genuine 'confirm' candidates, left as warnings not auto-added (the guard surfaces, the human decides). No blockers."; }
    part manifestCoverageGuardStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b090-51627384bb07"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b577f9c"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification manifestCoverageGuardImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b090-51627384bb08"; :>> title = "Sprint 90 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented manifest_coverage + parse_manifest + declared_task_names + GUARD_NAMES[11] + run_one + regression test manifest_parses_per_task_entries. VERIFY (live): `sysmlv2 guard manifest-coverage` = 13 scanned / 0 violations / 2 warnings (story* false positives excluded); clippy --workspace --all-targets -D warnings green; cargo test 41 lib pass; rust validate 102 clean; guard all (11) PASS; append-result -> issue033 RESOLVED (open 4->3); CLAUDE.md guard list -> eleven."; }
    part manifestCoverageGuardImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b090-51627384bb09"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b577f9c"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification manifestCoverageGuardReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b090-51627384bb0a"; :>> title = "Sprint 90 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The guard closes the d0050 finding's core risk: the manifest can no longer silently rot (a renamed/removed task or path that drops suspicion coverage now fails pre-commit). The forward gap (a NEW unlisted deliverable task) is genuinely hard to detect precisely, so it is a low-false-positive WARNING (deliverable-name heuristic, story* excluded), not a blocking violation — surfaced for human confirmation rather than force-added (honest about the heuristic's limits). parse_manifest is a pure tested helper. issue033 resolves by computation. No new decision/process-def (a Rust guard under D0050); no keystone."; }
    part manifestCoverageGuardReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b090-51627384bb0b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b577f9c"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification manifestCoverageGuardCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b090-51627384bb0c"; :>> title = "Sprint 90 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyManifestCoverageGuard DoD recorded pass; manifest-coverage guard enforced (GUARD_NAMES[11]); issue033 RESOLVED; clippy + cargo test (41) + rust validate (102) + guard all (11) green; doc-sync done. Self-evidencing (D0051). Second engine-hardening finding closed (2 of 5; issue029/030/031 remain)."; }
    part manifestCoverageGuardCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b090-51627384bb0d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b577f9c"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification manifestCoverageGuardRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b090-51627384bb0e"; :>> title = "Sprint 90 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: a second campaign finding (d0050) closed into a permanent enforced guard (D0047) — the manifest can no longer silently rot; reused the now-familiar guard shape (parse + classify + GUARD_NAMES + test) so this was a fast 2-pointer. CAUGHT (avoidable-issue candidate): the first cut of the unlisted-task WARNING fired on 5 story* sprint-wrapper actions (false positives matching the 'Parser' hint) — an avoidable noise source I narrowed by excluding story* (the manifest is about BACKLOG tasks); left the 2 genuine candidates (rustS0Workspace, rustFailLoudLints) as warnings rather than force-adding them (the guard surfaces, the human decides). Lesson: a name-heuristic guard must exclude the obvious wrapper-name class or its warnings drown the signal. Estimate 2 pts held. 2 of 5 hardening findings closed; issue029 (friction)/030 (rigor)/031 (independence) remain (the design-heavier set)."; }
    part manifestCoverageGuardRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b090-51627384bb0f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b577f9c"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
