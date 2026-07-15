// ProjectDeliveryS236 — Sprint 236: viewerBaselineCompare — diff a viewpoint between two commits (N-13).
// keel builds the model at each commit (git worktree), slices both, and classifies each element added/
// removed/changed/reverified/unchanged; no differences → "no drift". /api/baseline-compare. Satisfies
// srViewerBaselineCompare. Completes the remaining viewer-brick run. EXECUTE. 3 pts.
package ProjectDeliveryS236 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewerBaselineCompareStory to srViewerBaselineCompare;

    part viewerBaselineCompareStory : Story {
        :>> id = "63940516-2c7d-4e8f-c236-51627384c001";
        :>> title = "Sprint 236: viewerBaselineCompare (3 pts) - diff a viewpoint between two commits (/api/baseline-compare)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS236 {
        action storyViewerBaselineCompare;
        verification storyViewerBaselineCompareDoD : Test { :>> id = "63940516-2c7d-4e8f-c236-51627384c002"; :>> method = VerificationMethod::demo; :>> procedureText = "Satisfies srViewerBaselineCompare (N-13). IMPLEMENTED: view::model_at_commit checks a commit out into a throwaway `git worktree`, builds the model there, removes the worktree; view::baseline_compare_json slices the viewpoint (seed,depth,edges,dir) at `from` and `to`, then classifies each element in the union — ADDED (in to, not from = 'added since'), REMOVED (in from, not to), CHANGED (in both, type/attrs differ), REVERIFIED (a TestResult whose judgedAgainst moved), UNCHANGED. No differences → note 'no drift'. GET /api/baseline-compare?seed=&from=&to=&depth=&edges=&dir= (KEEL_API 1.11.0). VERIFY (demo, live): NO-DRIFT (from=to=HEAD, seed d0120) → note 'no drift', 0 added/removed/changed, 16 unchanged; ADDED-SINCE (from=633b689 → HEAD, seed d0120 — d0120 did not exist at 633b689) → 16 added, d0120 ∈ added = True (classified correctly). clippy --workspace -D warnings clean; keel validate + guard + check-engine green."; }
        part storyViewerBaselineCompareDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c236-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    }

    verification blRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c236-51627384c004"; :>> title = "Sprint 236 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: viewerBaselineCompare — the last ready viewer brick. Scope: diff a viewpoint between two commits via git. #CharteredBy srViewerBaselineCompare. estimatedPoints=3. DoR met."; }
    part blRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c236-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification blStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c236-51627384c006"; :>> title = "Sprint 236 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: git worktree per commit (clean, no tar/pipe) → Model::build → configurable_slice at each → classify the union by presence + type/attrs equality; reverified = TestResult judgedAgainst moved. No blockers."; }
    part blStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c236-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification blImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c236-51627384c008"; :>> title = "Sprint 236 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::demo; :>> procedureText = "model_at_commit + baseline_compare_json + /api/baseline-compare implemented. clippy clean; live-verified (no-drift + added-since classified correctly). validate/guard/check-engine green."; }
    part blImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c236-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification blReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c236-51627384c00a"; :>> title = "Sprint 236 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Diffs the viewpoint against ACTUAL git history (the model rebuilt at each commit via a throwaway worktree — honest, no snapshot store), classifying added/removed/changed/reverified with a clean 'no drift' case. It reuses the same slice definition, so 'compare' is genuinely the same viewpoint at two points. Verified both the drift + no-drift cases. Worktrees are cleaned up. Clean."; }
    part blReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c236-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification blCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c236-51627384c00c"; :>> title = "Sprint 236 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; baseline-compare delivered (srViewerBaselineCompare). Self-evidencing. ALL three remaining viewer bricks (viewerChangeImpact + viewerExportShare + viewerBaselineCompare) are now DONE. NEXT: whatever the human directs — polish (in-graph swimlane boxes; viewer Compare/Impact panels) or new work."; }
    part blCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c236-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification blRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c236-51627384c00e"; :>> title = "Sprint 236 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: completed the whole remaining viewer-brick run one at a time (change-impact → export/share → baseline-compare), each keel-side computed view test-verified live + governed by its own sprint. Baseline-compare rebuilds the model at each commit via a throwaway worktree — honest git-history diff, no snapshot store, cleaned up. AVOIDABLE-ISSUE (banked): ViewError has no Parse variant (used Track); and #CharteredBy is mandatory on every delivery Story (added up-front after S234's catch). LESSON: temporal diffs = the same view definition rebuilt at two commits; reuse the slice, don't special-case. Estimate 3 pts. NEXT: human-directed."; }
    part blRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c236-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e995440"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
