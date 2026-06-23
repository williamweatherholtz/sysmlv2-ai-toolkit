// =====================================================================
// ProjectDeliveryS114 — Sprint 114: serveAgentCancel (D0094 agent-bridge robustness). On Windows the
// console agent is `cmd /C claude` → `claude.exe`; tokio's kill_on_drop reaps only the direct `cmd`
// child on client disconnect / Stop, ORPHANING the `claude.exe` grandchild (no process-tree kill, and
// #![forbid(unsafe_code)] rules out a Win32 Job Object). FIX: a TreeKiller Drop-guard captures the cmd
// PID at spawn and `taskkill /T /F /PID`-kills the WHOLE tree when the SSE stream drops, disarmed on
// normal exit so it never hits a recycled PID; Unix spawns `claude` directly so kill_on_drop suffices.
// Verified LIVE: agent claude.exe pid 17796 spawned (count 14->15), reaped on disconnect (->14), the
// user's own session untouched (specific PID tree, never /IM). Total: 2 pts. Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS114 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveAgentCancelStory to d0094;

    part serveAgentCancelStory : Story {
        :>> id = "63940516-2c7d-4e8f-c114-51627384c001";
        :>> title = "Sprint 114: serveAgentCancel (2 pts) — process-tree kill so a stopped agent reaps its claude.exe grandchild (D0094)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS114 {
        action storyServeAgentCancel;
        verification storyServeAgentCancelDoD : Test { :>> id = "63940516-2c7d-4e8f-c114-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0094 agent-bridge cancellation. serve.rs: a TreeKiller(Option<u32>) Drop-guard holds the spawned cmd PID; on drop (SSE stream end — client disconnect / Stop / normal completion) it runs `taskkill /T /F /PID <pid>` on Windows (whole process tree: cmd + claude.exe + descendants), no-op on Unix (claude is the direct child there, already reaped by kill_on_drop). Declared AFTER `child` so it drops FIRST (reverse decl order). DISARMED (pid=None) right after child.wait() on normal exit, so a completed/recycled PID is never targeted. const fn disarm (clippy nursery). VERIFY (LIVE this sprint): started `keel serve`, baseline claude.exe=14 (user session); opened /api/agent/stream -> claude.exe pid 17796 appeared (count 15); disconnected the SSE -> pid 17796 REAPED, count back to 14, user session untouched (we kill a specific PID tree, never /IM claude.exe). clippy --workspace --all-targets -D warnings + cargo test (46 lib + all BDD) + keel validate + guard all (13) green."; }
        part storyServeAgentCancelDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c114-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b10fc70"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveAgentCancelRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c114-51627384c004"; :>> title = "Sprint 114 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the console-arc retros flagged that on Windows a stopped agent orphans its claude.exe grandchild (kill_on_drop reaps only cmd). D0094 owns the agent-bridge robustness. #![forbid(unsafe_code)] rules out a Job Object, so taskkill /T is the dependency-free tree kill. Resolver: serveAgentCancel. estimatedPoints=2. #CharteredBy d0094. DoD accepted."; }
    part serveAgentCancelRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c114-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b10fc70"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentCancelStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c114-51627384c006"; :>> title = "Sprint 114 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add TreeKiller(Option<u32>) with a Drop that taskkill /T /F the cmd PID (Windows only via cfg!(windows)); arm it with child.id() after spawn, declared after `child`; disarm after child.wait(). Verify by a LIVE cancel: observe a new claude.exe PID appear, disconnect, confirm that PID is gone while the user's session count is unchanged. No .engine/schema/process-def change -> no kernel/keystone. No blockers."; }
    part serveAgentCancelStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c114-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b10fc70"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentCancelImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c114-51627384c008"; :>> title = "Sprint 114 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented TreeKiller + arm/disarm in api_agent_stream. VERIFY (live): baseline claude.exe=14; agent spawned pid 17796 (count 15); SSE disconnect -> pid 17796 reaped (count 14), user session untouched; server log shows the agent-stream request. clippy -D warnings green (fixed: const fn disarm — clippy::missing_const_for_fn); cargo test 46 lib + BDD (orient 8/8) pass; keel validate + guard all (13) PASS."; }
    part serveAgentCancelImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c114-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b10fc70"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentCancelReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c114-51627384c00a"; :>> title = "Sprint 114 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The guard is minimal + safe: it targets a SPECIFIC captured PID tree (taskkill /T /F /PID), never a blanket /IM that could kill the human's own Claude Code session — exactly the risk called out in the build notes. Drop order (killer after child) and disarm-on-normal-exit are correct: a cancelled run reaps the tree; a completed run leaves it alone (no recycled-PID hazard). cfg!(windows) keeps it a no-op on Unix where kill_on_drop already reaps the direct `claude` child. Live-tested rather than asserted. Clean."; }
    part serveAgentCancelReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c114-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b10fc70"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentCancelCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c114-51627384c00c"; :>> title = "Sprint 114 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyServeAgentCancel DoD recorded pass; the process-tree kill is live + LIVE-verified (pid 17796 reaped on disconnect, user session safe). clippy + cargo test (46 lib + BDD) + keel validate + guard all (13) green. Self-evidencing (D0051). Second of four post-D0094 forward items; init-smoke CI test next."; }
    part serveAgentCancelCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c114-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b10fc70"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentCancelRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c114-51627384c00e"; :>> title = "Sprint 114 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the #![forbid(unsafe_code)] constraint steered to the SIMPLER right answer (a taskkill /T Drop-guard) instead of a Win32 Job Object with FFI — fewer deps, no unsafe, same effect. LIVE-verifying the reap (watch a real claude.exe PID appear then disappear on disconnect) gave real evidence that kill_on_drop's grandchild-orphan gap is actually closed — a unit test could not have shown this. Capturing the SPECIFIC PID and using /T (not /IM) respected the standing safety note: never blanket-kill claude.exe (the human's session). AVOIDABLE-ISSUE SCAN: no NEW avoidable defect this sprint (the const-fn clippy nit was a known recurring lint, fixed inline). Re-confirmed: stop keel.exe before rebuild (binary lock). NON-blocking forward note (NOT debt): the disarm/Drop window has a negligible PID-reuse hazard between cmd-exit and guard-run, acceptable for localhost dev tooling. Estimate 2 pts held. serveAgentCancel done; initSmokeTest (CI `keel init` round-trip) next."; }
    part serveAgentCancelRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c114-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b10fc70"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
