// =====================================================================
// ProjectDeliveryS106 — Sprint 106: serveReadConsole (D0094 m1). `sysmlv2 serve [--port N] [ROOT]` —
// the Keel interactive console (localhost tokio+axum) + an embedded self-contained frontend
// (console.html, include_str!). The live READ tier: orient dashboard, recent decisions, processes-in-
// use, >=Medium findings/dispositions, reports, AND a read-only AI<->user INTERACTION-HISTORY view
// rendered from the Claude Code session transcripts (~/.claude/projects/<encoded-cwd>/*.jsonl), NEVER
// copied into the model. ONE truth — every endpoint computes from the existing view authority.
// Verified live: server binds 127.0.0.1, all 6 API endpoints + the frontend serve correctly; the
// history encoding resolved this project's transcript dir + parsed its turns.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS106 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveReadConsoleStory to d0094;

    part serveReadConsoleStory : Story {
        :>> id = "63940516-2c7d-4e8f-c106-51627384c001";
        :>> title = "Sprint 106: serveReadConsole (5 pts) — `sysmlv2 serve` localhost console + read tier + interaction-history view (D0094 m1)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS106 {
        action storyServeReadConsole;
        verification storyServeReadConsoleDoD : Test { :>> id = "63940516-2c7d-4e8f-c106-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0094 m1: add `sysmlv2 serve [--port N] [ROOT]` — a localhost (127.0.0.1) tokio+axum server (src/serve.rs) + an embedded self-contained frontend (assets/console.html, include_str!) serving the live READ console. Endpoints (all compute from the existing view authority; ONE truth, nothing stored): GET / (frontend), /api/orient, /api/decisions, /api/dispositions, /api/processes, /api/report/:name, /api/history. The interaction-history view (interaction_history) renders the Claude Code session transcripts from ~/.claude/projects/<encoded-cwd>/*.jsonl (cwd encoded by mapping non-alphanumeric -> '-'); read-only lens, NEVER copied into .tracking (compute-don't-store, section-2.1). Frontend polls every 6s for liveness (true SSE/file-watch deferred to where streaming is essential, m3). tokio+axum added to the workspace (D0094-accepted). VERIFY (LIVE, ran this sprint): `sysmlv2 serve --port 7788 .` binds localhost; / returns the 7KB frontend (200); /api/orient shows the live ready frontier; /api/processes lists titles/purposes; /api/dispositions 13/11/2; /api/report/assurance returns cards; /api/history available:true with this project's transcript dir resolved + the latest session's turns parsed; /api/decisions returns the ranked report. clippy --workspace -D warnings + cargo test (46 lib) + rust validate 119 + guard all (13) green."; }
        part storyServeReadConsoleDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c106-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e33148b"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveReadConsoleRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c106-51627384c004"; :>> title = "Sprint 106 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0094 accepted (tokio+axum, build straight through); m1 = the read console + interaction-history view, the foundation the action/agent tiers plug into. Resolver: serveReadConsole. estimatedPoints=5 (server + embedded frontend + history parsing). #CharteredBy d0094. DoD accepted."; }
    part serveReadConsoleRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c106-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e33148b"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveReadConsoleStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c106-51627384c006"; :>> title = "Sprint 106 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: serve.rs (axum Router + State{root}; handlers call orient::compute / view::* / a processes lister / interaction_history; localhost bind; runtime via tokio block_on); console.html (vanilla-JS tabbed SPA, embedded, 6s poll); cmd_serve (--port + ROOT parse, no indexing); lib.rs pub mod serve; deps tokio+axum. Verify by ACTUALLY running the server + curling every endpoint. No process-def/schema change -> no keystone/kernel. No blockers."; }
    part serveReadConsoleStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c106-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e33148b"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveReadConsoleImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c106-51627384c008"; :>> title = "Sprint 106 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented serve.rs + console.html + cmd_serve + deps. VERIFY (live): server on 127.0.0.1:7788; / = 200 (7KB frontend); /api/orient live ready frontier; /api/processes titles+purposes; /api/dispositions 13/11/2; /api/report/assurance cards; /api/history resolved this project's ~/.claude transcript dir + parsed the latest session; /api/decisions ranked report. clippy -D warnings green (fixed: serde_json [] indexing -> .get()); cargo test 46 lib pass; rust validate 119; guard all (13) PASS."; }
    part serveReadConsoleImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c106-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e33148b"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveReadConsoleReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c106-51627384c00a"; :>> title = "Sprint 106 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The console is ONE truth (every endpoint computes from the existing view authority; the server stores nothing); the interaction-history is a read-only lens (renders ~/.claude transcripts, never copied into the model — compute-don't-store honored). Self-contained frontend (embedded, no CDN — the cytoscape ethos). Localhost-only. Delivers the human's 'context + insight at a glance' on its own, and is the foundation serveActions (m2) + serveAgentBridge (m3) plug into. Polling (not SSE) is fine for the read tier; real SSE arrives where streaming is essential (the agent-bridge). Clean."; }
    part serveReadConsoleReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c106-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e33148b"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveReadConsoleCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c106-51627384c00c"; :>> title = "Sprint 106 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyServeReadConsole DoD recorded pass; `sysmlv2 serve` read console live + verified end-to-end; clippy + cargo test (46) + rust validate 119 + guard all (13) green. Self-evidencing (D0051). D0094 m1 done; serveActions (deterministic actions) + serveAgentBridge (the headless-Claude bridge) remain in the arc."; }
    part serveReadConsoleCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c106-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e33148b"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveReadConsoleRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c106-51627384c00e"; :>> title = "Sprint 106 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: every console endpoint reuses an EXISTING view computation (orient/decisions/dispositions/report) so the console is one-truth by construction (no parallel logic to drift); the interaction-history cwd-encoding (non-alnum -> '-') resolved this project's transcript dir on the first live run; verifying by actually starting the server + curling all 7 routes (not a code-read) confirmed it works end-to-end including the history parse. AVOIDABLE-ISSUE SCAN: no avoidable defect this sprint — the serde_json `[]`-indexing clippy nit (indexing_slicing on Value) was caught + fixed inline (-> .get()), a trivial one-off. NON-blocking forward notes (NOT tracked debt): (1) the read tier polls every 6s; true SSE/file-watch is deferred to serveAgentBridge where streaming is essential; (2) serve.rs is deliverable-source-dependent but not yet in deliverable-manifest.txt (the suspect-coverage hint names don't match 'serve') — add it when m2/m3 land if source-drift suspicion is wanted. Estimate 5 pts held. D0094 m1 delivered; m2 (serveActions) next."; }
    part serveReadConsoleRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c106-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e33148b"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
