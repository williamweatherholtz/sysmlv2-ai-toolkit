// =====================================================================
// ProjectDeliveryS109 — Sprint 109: serveAgentBridge (D0094 m3 — COMPLETES the console arc). The UI
// drives the LOCALLY-AUTHENTICATED `claude` CLI and STREAMS its work over SSE: GET /api/agent/stream
// ?action=<critique|investigate|report>&target=<x> spawns `claude -p --output-format stream-json
// --include-partial-messages --max-turns 30` in the repo (loads CLAUDE.md + skills), forwarding each
// NDJSON event as an SSE event. Auth = the user's subscription/ENTERPRISE seat — the server NEVER sets
// ANTHROPIC_API_KEY (verified live: the streamed init event shows apiKeySource:none). Runs under the
// engine's EXISTING discipline; the prompt forbids git commit; per-action turn cap (30) + concurrency
// cap (2). Frontend: an Agent tab (action/target + Run/Stop, EventSource log). Windows: routed via
// `cmd /C` to resolve the claude.cmd npm shim.
// Total: 8 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS109 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveAgentBridgeStory to d0094;

    part serveAgentBridgeStory : Story {
        :>> id = "63940516-2c7d-4e8f-c109-51627384c001";
        :>> title = "Sprint 109: serveAgentBridge (8 pts) — UI -> headless local `claude` -> SSE (critique/investigate/report); completes the D0094 console arc";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 8;
    }

    action def DeliveryRunS109 {
        action storyServeAgentBridge;
        verification storyServeAgentBridgeDoD : Test { :>> id = "63940516-2c7d-4e8f-c109-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0094 m3 (the agent-bridge). GET /api/agent/stream?action=&target= -> an axum Sse stream (async-stream + tokio-stream) that spawns headless Claude Code in the repo cwd (`claude -p <prompt> --output-format stream-json --include-partial-messages --verbose --max-turns 30`), reads its stdout line-by-line (tokio BufReader.lines), and forwards each NDJSON line as an SSE `agent` event (+ status/done/error events). AUTH: pipes to the user's LOCALLY-AUTHENTICATED `claude` (subscription/enterprise); the server NEVER sets ANTHROPIC_API_KEY (D0094). DISCIPLINE: the agent inherits CLAUDE.md + skills from the cwd; the prompt forbids `git commit` (commits stay the human's gate, D0016); per-action turn cap (AGENT_MAX_TURNS=30) + concurrency cap (AGENT_MAX_CONCURRENT=2 via an AtomicUsize + a Drop slot guard); kill_on_drop on the child. WINDOWS: route via `cmd /C claude` (the claude.cmd npm shim is not directly spawnable); prompts ASCII (cmd-codepage safe). Frontend: an Agent tab (action select critique/investigate/report + target input + Run/Stop, an EventSource that renders assistant text + tool markers + result/cost into a log; the 6s auto-refresh skips this tab). VERIFY (LIVE, ran this sprint): the SSE endpoint streamed a real agent run — the init event confirmed apiKeySource:none (enterprise seat, NOT an API key) + model claude-opus-4-8 + cwd = the repo; status/agent/done events flowed; graceful-degrade returns a clear error event when `claude` is absent; bad/missing target handled. clippy --workspace -D warnings + cargo test (46 lib) + keel validate 122 + guard all (13) green; frontend serves the Agent tab."; }
        part storyServeAgentBridgeDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c109-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e619ced"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveAgentBridgeRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c109-51627384c004"; :>> title = "Sprint 109 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0094 m1+m2 shipped; m3 = the agent-bridge (the exciting tier). The claude-code-guide research + the human's directives settle the mechanism (CLI stream-json, no API key, same discipline, review hooks). Resolver: serveAgentBridge. estimatedPoints=8 (async subprocess streaming + SSE + cross-platform spawn + frontend). #CharteredBy d0094. DoD accepted."; }
    part serveAgentBridgeRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c109-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e619ced"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentBridgeStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c109-51627384c006"; :>> title = "Sprint 109 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: serve.rs — AppState += agents counter; AgentReq query; build_agent_prompt (ASCII, no-commit); api_agent_stream (async_stream::stream! over a spawned tokio process; status/agent/done/error SSE events; AgentSlot Drop guard for the concurrency cap; cmd /C on Windows; kill_on_drop). deps tokio[process,io-util] + async-stream + tokio-stream. console.html — Agent tab + EventSource client (runAgent/stopAgent/renderAgentEvent). Verify by running a live SSE peek (confirm claude launches + apiKeySource:none). No .engine/schema change -> no kernel/keystone. No blockers."; }
    part serveAgentBridgeStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c109-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e619ced"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentBridgeImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c109-51627384c008"; :>> title = "Sprint 109 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the SSE agent-bridge + Agent tab. VERIFY (live): /api/agent/stream streamed a real run — status event, then stream-json agent events incl. the init showing apiKeySource:none + model claude-opus-4-8 + repo cwd; graceful-degrade error event when claude is absent (pre-fix) and when present after the cmd /C Windows fix it launched; concurrency/turn caps in place; frontend serves the Agent tab (runAgent + /api/agent/stream present). clippy -D warnings green; cargo test 46 lib pass; keel validate 122; guard all (13) PASS."; }
    part serveAgentBridgeImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c109-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e619ced"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentBridgeReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c109-51627384c00a"; :>> title = "Sprint 109 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The cockpit is complete: the human can drive a disciplined agent from a button (critique/investigate/report), watching it stream live. The auth requirement is MET + verified (apiKeySource:none -> enterprise seat, never an API key). The agent runs under the engine's existing discipline (CLAUDE.md + skills from the cwd), forbidden to commit; bounded by the turn cap + concurrency cap; localhost-only. KNOWN LIMITATION (noted, not a blocker): on Windows kill_on_drop reaps `cmd` but not the `claude.exe` grandchild (no job-object process-tree kill) -> a cancelled/disconnected agent runs to its turn cap rather than dying instantly; the cap + the read-only-by-prompt default bound the blast radius. This completes the D0094 console arc (read -> actions -> agent-bridge). Clean."; }
    part serveAgentBridgeReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c109-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e619ced"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentBridgeCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c109-51627384c00c"; :>> title = "Sprint 109 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyServeAgentBridge DoD recorded pass; the agent-bridge streams a real headless-claude run over SSE (apiKeySource:none verified) + the Agent tab is live; clippy + cargo test + keel validate 122 + guard all (13) green. Self-evidencing (D0051). D0094 console arc COMPLETE (m1 read + m2 actions + m3 agent-bridge) — Keel is a self-improvement cockpit."; }
    part serveAgentBridgeCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c109-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e619ced"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveAgentBridgeRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c109-51627384c00e"; :>> title = "Sprint 109 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: a live SSE peek (not a code-read) immediately PROVED the two things that mattered — claude actually launches AND apiKeySource:none (the enterprise-seat / no-API-key requirement the human set, confirmed from the streamed init event); routing via `cmd /C` fixed the Windows claude.cmd-shim spawn that 'program not found' first exposed; the prompt-level 'do NOT git commit' + cwd-inherited CLAUDE.md keep the UI-launched agent disciplined without a new permission policy (per the human). AVOIDABLE-ISSUE SCAN — two caught: (1) Command::new('claude') failed on Windows because claude is a .cmd npm shim (CreateProcess needs cmd /C) — fixed + ASCII-ified the prompts for cmd-codepage safety; (2) kill_on_drop reaps cmd but ORPHANS the claude.exe grandchild on Windows (no process-tree kill) — the smoke-test agent had to be hunted by commandline; this is a real cancellation gap, bounded by --max-turns + read-only-by-default, with the proper fix (a Windows job object / process-group kill) noted as a NON-blocking forward item (serveAgentCancel — not yet tracked as debt; the caps bound the risk). NON-blocking: serve.rs is deliverable-source-dependent but not in deliverable-manifest.txt. Estimate 8 pts held (the cross-platform spawn surprise absorbed). The D0094 cockpit is DONE end to end: read + act + drive-the-agent, all from the browser, on the enterprise seat."; }
    part serveAgentBridgeRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c109-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e619ced"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
