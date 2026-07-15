// ProjectDeliveryS222 — Sprint 222: serveCorsLocalhost — localhost CORS on `keel serve` so a SEPARATE
// viewer app (D0114 shape B) can consume /api/* cross-port. The enabling gap for shape B: without CORS
// headers a separate-origin SPA physically cannot fetch keel's API. Satisfies srViewerKeelApi. EXECUTE. 1 pt.
package ProjectDeliveryS222 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveCorsStory to d0114;

    part serveCorsStory : Story {
        :>> id = "63940516-2c7d-4e8f-c222-51627384c001";
        :>> title = "Sprint 222: serveCorsLocalhost (1 pt) - localhost CORS on keel serve (shape B cross-origin enabler)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS222 {
        action storyServeCors;
        verification storyServeCorsDoD : Test { :>> id = "63940516-2c7d-4e8f-c222-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Satisfies srViewerKeelApi (D0114 shape B). IMPLEMENTED: serve::cors_localhost middleware (dependency-free) reflects a localhost/127.0.0.1 Origin on any port (is_localhost_origin), advertises GET/POST/OPTIONS + content-type, sets Vary: Origin, and short-circuits the OPTIONS preflight with 204. Non-local origins get NO CORS headers; the server stays 127.0.0.1-bound (defense-in-depth — this only enables the local cross-port case shape B needs). Unit test cors_reflects_localhost_origins_only. VERIFY (test): clippy --workspace --all-targets -D warnings clean; cargo test green; live curl -> GET with Origin http://localhost:5173 returns access-control-allow-origin reflected + methods/headers/vary; OPTIONS preflight returns 204 with the headers; a remote Origin gets no ACAO. Code @d72bbea. This unblocks the separate viewer app (sysmlv2-viewer) consuming /api/*."; }
        part storyServeCorsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c222-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d72bbea"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    }

    verification scoRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c222-51627384c004"; :>> title = "Sprint 222 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: pivoting to BUILD the viewer app (shape B, separate repo) surfaced a hard prerequisite — keel serve sends no CORS headers, so a separate-origin SPA cannot fetch /api/*. Scope: add localhost CORS. estimatedPoints=1. #CharteredBy d0114. DoR met."; }
    part scoRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c222-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d72bbea"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification scoStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c222-51627384c006"; :>> title = "Sprint 222 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: a from_fn middleware (no tower-http dependency, keeping keel lean) that reflects localhost origins + answers preflight; layered on the router. No blockers."; }
    part scoStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c222-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d72bbea"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification scoImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c222-51627384c008"; :>> title = "Sprint 222 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "cors_localhost + is_localhost_origin + add_cors_headers + router layer + unit test implemented. clippy -D warnings clean; cargo test green; live-verified (localhost reflected; preflight 204; remote denied)."; }
    part scoImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c222-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d72bbea"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification scoReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c222-51627384c00a"; :>> title = "Sprint 222 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. CORS is the honest prerequisite for shape B — the 'committed, versioned, CONSUMABLE API' is not consumable cross-origin without it. Scoped tightly to localhost (any port) so it enables the local viewer without opening the 127.0.0.1-bound server to remote callers. Dependency-free (no tower-http) keeps keel lean. Clean."; }
    part scoReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c222-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d72bbea"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification scoCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c222-51627384c00c"; :>> title = "Sprint 222 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; localhost CORS delivered + live-verified; the viewer app can now fetch /api/*. Self-evidencing (D0051). NEXT: scaffold the sysmlv2-viewer SPA (generative: fetch /api/version + /api/schema, render a viewpoint explorer) under that repo's own keel discipline."; }
    part scoCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c222-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d72bbea"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification scoRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c222-51627384c00e"; :>> title = "Sprint 222 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: starting the viewer pivot immediately exposed the real, concrete blocker (no CORS) rather than discovering it after writing SPA code that silently failed to fetch; fixed it keel-side, minimally + tested, before touching the viewer. AVOIDABLE-ISSUE SCAN: none. LESSON: when a 'consumable API' gains a separate consumer, the browser same-origin policy is a first-class part of the contract — verify cross-origin fetch works before building the consumer. Estimate 1 pt held. NEXT: sysmlv2-viewer SPA scaffold."; }
    part scoRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c222-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d72bbea"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
