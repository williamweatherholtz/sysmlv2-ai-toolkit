// ProjectDeliveryS234 — Sprint 234: viewerChangeImpact — blast-radius view (N-10). From a focus element,
// the reachable elements GROUPED BY DISTANCE over configurable typed edges; cycles counted once; leaf →
// "nothing depends on this". keel view + /api/change-impact. Satisfies srViewerChangeImpact. EXECUTE. 2 pts.
package ProjectDeliveryS234 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewerChangeImpactStory to srViewerChangeImpact;

    part viewerChangeImpactStory : Story {
        :>> id = "63940516-2c7d-4e8f-c234-51627384c001";
        :>> title = "Sprint 234: viewerChangeImpact (2 pts) - blast-radius from a focus, grouped by distance (/api/change-impact)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS234 {
        action storyViewerChangeImpact;
        verification storyViewerChangeImpactDoD : Test { :>> id = "63940516-2c7d-4e8f-c234-51627384c002"; :>> method = VerificationMethod::demo; :>> procedureText = "Satisfies srViewerChangeImpact (N-10). IMPLEMENTED: view::change_impact_json BFS's from the focus over configurable typed `edges` in `dir` (default up = dependents), recording each node's SHORTEST distance (level), so cycles terminate (counted once); returns {seed, direction, impacted, note, byDistance:[{distance,count,items}]}. Empty => note 'nothing depends on this (leaf)'. Exposed at GET /api/change-impact?seed=&edges=&dir=; added to the KEEL_API read contract (v1.9.0). VERIFY (demo, live): /api/change-impact?seed=d0049 -> impacted 10 across 2 distance levels (dist 1: d0123 + critiques + sittingModelStory; dist 2: nested critique results + reviewQueueGovStory) — grouped by distance, terminates despite model cycles; leaf /api/change-impact?seed=d0123AcceptR1 -> impacted 0, note 'nothing depends on this (leaf)'. clippy --workspace -D warnings clean; keel validate + guard + check-engine green. The generative viewer consumes this endpoint (distance-grouped display)."; }
        part storyViewerChangeImpactDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c234-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dbb5287"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    }

    verification ciRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c234-51627384c004"; :>> title = "Sprint 234 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: viewerChangeImpact ready in orient. Scope: a keel change-impact view (distance-grouped dependents) + endpoint. Charters to the viewer program. estimatedPoints=2. DoR met."; }
    part ciRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c234-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dbb5287"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification ciStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c234-51627384c006"; :>> title = "Sprint 234 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: BFS with a shortest-distance map (cycle-safe) reusing the slice edge-direction logic; group by level; endpoint mirrors /api/slice parsing. No blockers."; }
    part ciStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c234-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dbb5287"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification ciImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c234-51627384c008"; :>> title = "Sprint 234 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::demo; :>> procedureText = "change_impact_json + /api/change-impact + contract + version bump implemented. clippy clean; live-verified (d0049 -> 2 distance levels; leaf -> note). validate/guard/check-engine green."; }
    part ciImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c234-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dbb5287"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification ciReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c234-51627384c00a"; :>> title = "Sprint 234 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The blast-radius is computed correctly (shortest-distance BFS, cycle-safe) and grouped by distance so a reviewer sees direct vs transitive impact; the leaf case is explicit. Configurable edges/direction keeps it general (dependents by default). Test-verifiable at the API; the generative viewer renders it. Clean."; }
    part ciReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c234-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dbb5287"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification ciCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c234-51627384c00c"; :>> title = "Sprint 234 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; change-impact view + endpoint delivered (srViewerChangeImpact). Self-evidencing. NEXT (this run): viewerExportShare (N-12) — snapshot a viewpoint stamped with as-of date + commit + scope."; }
    part ciCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c234-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dbb5287"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification ciRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c234-51627384c00e"; :>> title = "Sprint 234 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: reused the slice's edge-direction logic for a cycle-safe distance BFS — small, correct, test-verified live (incl. the leaf + cycle cases). AVOIDABLE-ISSUE SCAN: none. LESSON: change-impact is a slice with a distance annotation — don't invent a new traversal, add the level. Estimate 2 pts. NEXT: viewerExportShare."; }
    part ciRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c234-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dbb5287"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
