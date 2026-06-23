// =====================================================================
// ProjectDeliveryS108 — Sprint 108: serveActions (D0094 m2) — the console's INTERACTIVE deterministic
// tier + serve observability. POST /api/disposition records a finding disposition via the write API
// (the human's click + rationale IS their explicit verdict, D0016; never auto-committed). GET
// /view/report/<name> + /view/diagram serve the full computed HTML (instantiate/render actions). A
// request-logging middleware (method/path/status/ms) + a route banner make the server debuggable
// (human ask). Findings tab gains [Act]/[Accept-risk]/[Dismiss] buttons; Reports tab gains open-as-HTML
// links. Subtitle set to "Smooth sailing with AI". ONE truth — writes go through the write API + guards.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS108 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveActionsStory to d0094;

    part serveActionsStory : Story {
        :>> id = "63940516-2c7d-4e8f-c108-51627384c001";
        :>> title = "Sprint 108: serveActions (5 pts) — interactive deterministic tier (disposition write + open-as-HTML) + serve logging (D0094 m2)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS108 {
        action storyServeActions;
        verification storyServeActionsDoD : Test { :>> id = "63940516-2c7d-4e8f-c108-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0094 m2 (deterministic actions) + the human's serve-debugging ask. Actions: POST /api/disposition {finding,verdict(act/accept-risk/dismiss),rationale,judged_at,judged_by} -> crate::write::append_disposition into .tracking/critiques.sysml (judgedAgainst = git HEAD; the human's UI click + rationale IS their explicit attestation, D0016; the agent does not infer it; NEVER auto-commits); GET /view/report/<name> -> report_html, GET /view/diagram -> diagram_html (instantiate/render). Frontend: Findings tab gets per-finding [Act]/[Accept-risk]/[Dismiss] buttons (prompt rationale + judgedBy -> POST -> refresh); Reports tab gets 'Open full HTML report / model diagram' links. OBSERVABILITY (debugging ask): a request-logging middleware logs method/path/status/elapsed-ms to the terminal + a startup route banner + clear JSON error bodies. Subtitle -> 'Smooth sailing with AI'. ONE truth (reads compute from the view authority; writes go through the write API + guards; no second store). VERIFY (LIVE, ran this sprint): GET /view/report/assurance 200 (4KB) + /view/diagram 200 (1.4MB); POST /api/disposition (ascii) -> 200 {ok,name:issue027Disp1} + wrote critiques.sysml (then REVERTED — a smoke test, not a real human disposition); bad verdict -> 400; request-log lines emitted. clippy --workspace -D warnings + cargo test (46 lib + BDD) + keel validate 121 + guard all (13) green."; }
        part storyServeActionsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c108-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec35576"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveActionsRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c108-51627384c004"; :>> title = "Sprint 108 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0094 m1 shipped (read console); the human asked to see the interactive tier + more serve debugging. m2 = deterministic actions (disposition write + open-as-HTML) + request logging. Resolver: serveActions. estimatedPoints=5. #CharteredBy d0094. DoD accepted."; }
    part serveActionsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c108-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec35576"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveActionsStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c108-51627384c006"; :>> title = "Sprint 108 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: serve.rs — log_request middleware (from_fn); git_head helper; DispReq + api_disposition (POST -> append_disposition; verdict map act/accept-risk/dismiss; 400 on bad); view_report/view_diagram (serve report_html/diagram_html); routes + startup banner. console.html — disposition buttons (prompt -> POST -> render), open-HTML links, subtitle. Verify by running the server + curling reads + a write (then revert). No .engine/schema/process-def change -> no kernel/keystone. No blockers."; }
    part serveActionsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c108-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec35576"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveActionsImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c108-51627384c008"; :>> title = "Sprint 108 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the disposition POST + open-as-HTML + logging + UI buttons + subtitle. VERIFY (live): /view/report/assurance 200; /view/diagram 200; POST disposition (ascii) 200 {ok,name} + wrote + reverted; bad verdict 400; request log emitted (method/path/status/ms). clippy -D warnings green; cargo test 46 lib + BDD pass; keel validate 121; guard all (13) PASS."; }
    part serveActionsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c108-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec35576"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveActionsReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c108-51627384c00a"; :>> title = "Sprint 108 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The console is now INTERACTIVE (m2): the human dispositions a finding from the UI (their click + rationale = explicit attestation, D0016) through the write API + guards (one truth, no second store, no auto-commit), and opens full HTML reports/diagram. The request-logging middleware + startup banner + JSON error bodies answer the debugging ask. Verified live incl. a write (reverted, since a real disposition is the human's act, not a smoke test). The headline interactive write demonstrates the m2 tier safely; m3 (agent-bridge) is the remaining tier. NOTE: the assurance report endpoint is slow on first call (~17s — report_cards recomputes readiness/coverage per request); a future perf pass (or a light cache) is the forward note. Clean."; }
    part serveActionsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c108-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec35576"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveActionsCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c108-51627384c00c"; :>> title = "Sprint 108 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyServeActions DoD recorded pass; the console's deterministic action tier + serve observability live + verified end-to-end (disposition write reverted post-test). clippy + cargo test + keel validate 121 + guard all (13) green. Self-evidencing (D0051). D0094 m2 done; serveAgentBridge (the headless-Claude bridge) is the last tier."; }
    part serveActionsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c108-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec35576"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveActionsRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c108-51627384c00e"; :>> title = "Sprint 108 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the disposition write reused the EXISTING write API (append_disposition, D0092) so the UI action is one-truth + guard-valid by construction; testing the write then REVERTING it kept the record honest (a real disposition is the human's attestation, not a smoke test); the request-logging middleware answered the debugging ask cheaply (axum from_fn, no extra dep) and immediately surfaced the slow report endpoint. AVOIDABLE-ISSUE SCAN: no avoidable defect — the only test hiccup was an em-dash in curl test-data mangled into invalid JSON (a test-harness artifact, not a server bug; the ascii write + the 400 path both proved the handler). NON-blocking forward notes (NOT debt): (1) the assurance report endpoint recomputes readiness per request (~17s first call) — a light per-request cache or a lighter readiness path would help; (2) serve.rs is deliverable-source-dependent but not yet in deliverable-manifest.txt. Estimate 5 pts held. D0094 m2 delivered; the interactive console writes now. serveAgentBridge (m3) is the finale — UI -> headless keel-authenticated `claude` -> SSE."; }
    part serveActionsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c108-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec35576"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
