// ProjectDeliveryS235 — Sprint 235: viewerExportShare — export/share a viewpoint as a stamped snapshot
// (N-12). keel /api/snapshot returns the slice + provenance (commit + as-of date + scope), capped if
// oversized; the viewer downloads it + copies a shareable scope URL. Satisfies srViewerExportShare. 2 pts.
package ProjectDeliveryS235 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewerExportShareStory to srViewerExportShare;

    part viewerExportShareStory : Story {
        :>> id = "63940516-2c7d-4e8f-c235-51627384c001";
        :>> title = "Sprint 235: viewerExportShare (2 pts) - stamped viewpoint snapshot (/api/snapshot) + export + shareable link";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS235 {
        action storyViewerExportShare;
        verification storyViewerExportShareDoD : Test { :>> id = "63940516-2c7d-4e8f-c235-51627384c002"; :>> method = VerificationMethod::demo; :>> procedureText = "Satisfies srViewerExportShare (N-12). KEEL-SIDE: view::snapshot_json returns the slice for a scope STAMPED with provenance — source commit (git HEAD short), as-of date (HEAD commit date %cs), and the scope {seed,depth,dir,edges}; oversized (>500) -> capped subset + a note to re-run at the commit. GET /api/snapshot (KEEL_API 1.10.0). VIEWER-SIDE (sysmlv2-viewer@6618750): an Export button downloads the stamped snapshot as JSON + copies a shareable URL (?seed=&depth=&dir=) that pre-fills the scope on load (round-trip). VERIFY (demo, live): /api/snapshot?seed=d0120&depth=2&dir=both -> snapshot{commit 041978f, asOf 2026-07-15, scope, itemCount 16, truncated false}; ROUND-TRIP CONFIRMED — re-running the snapshot's scope via /api/slice yields the SAME 16 items (SAME: True). clippy --workspace -D warnings clean; keel validate + guard + check-engine green."; }
        part storyViewerExportShareDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c235-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "041978f"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; :>> notes = "viewer code at sysmlv2-viewer@6618750."; }
    }

    verification esRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c235-51627384c004"; :>> title = "Sprint 235 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: viewerExportShare ready. Scope: a stamped snapshot endpoint (commit+as-of+scope, capped) + a viewer export/link. #CharteredBy srViewerExportShare. estimatedPoints=2. DoR met."; }
    part esRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c235-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "041978f"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c235-51627384c006"; :>> title = "Sprint 235 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: snapshot_json wraps the slice with commit (git_head) + as-of (git_head_date %cs) + scope, capping at 500; viewer Export downloads the JSON + copies a scope URL; ?seed= pre-fills on load. No blockers."; }
    part esStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c235-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "041978f"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c235-51627384c008"; :>> title = "Sprint 235 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::demo; :>> procedureText = "snapshot_json + git_head_date + /api/snapshot + viewer Export/URL implemented. clippy clean; live-verified (stamp + round-trip SAME items). validate/guard/check-engine green."; }
    part esImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c235-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "041978f"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c235-51627384c00a"; :>> title = "Sprint 235 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The snapshot is honest + reproducible: it records the exact commit + scope so it round-trips (verified — same items), and it caps oversized slices with a clear note rather than dumping. The shareable URL gives the 'link' form; the file gives the 'snapshot' form. Provenance-stamped, no second store. Clean."; }
    part esReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c235-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "041978f"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c235-51627384c00c"; :>> title = "Sprint 235 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; snapshot export/share delivered (srViewerExportShare). Self-evidencing. NEXT (last of this run): viewerBaselineCompare (N-13) — diff a viewpoint between two commits."; }
    part esCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c235-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "041978f"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification esRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c235-51627384c00e"; :>> title = "Sprint 235 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: made the snapshot reproducible-by-construction (commit + scope) and PROVED the round-trip (same items) rather than asserting it. AVOIDABLE-ISSUE (banked from S234): a delivery Story MUST carry #CharteredBy — the pre-commit charter guard caught its omission last sprint; added it up-front here. LESSON: export = data + provenance (commit/date/scope), not a copy; the round-trip is the test. Estimate 2 pts. NEXT: viewerBaselineCompare."; }
    part esRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c235-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "041978f"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
