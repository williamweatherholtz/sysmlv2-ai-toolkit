// ProjectDeliveryS215 — Sprint 215: viewerConfigurableSlice — the configurable-depth slice endpoint
// (/api/slice), the graph backbone for the keel-viewer (shape B). Seed + configurable depth + edge-kind
// filter + direction; generalizes the fixed 1-hop neighbourhood. Satisfies N-4; enables N-2 (seed-expand)
// + N-10 (change-impact, dir=up). API bumped 1.0.0 -> 1.1.0. EXECUTE. 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS215 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewerSliceStory to d0114;

    part viewerSliceStory : Story {
        :>> id = "63940516-2c7d-4e8f-c215-51627384c001";
        :>> title = "Sprint 215: viewerConfigurableSlice (2 pt) - /api/slice (seed + depth + edges + direction); backbone for N-2/N-4/N-10";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS215 {
        action storyViewerSlice;
        verification storyViewerSliceDoD : Test { :>> id = "63940516-2c7d-4e8f-c215-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Satisfies srViewerConfigurableSlice (N-4) + delivers viewerConfigurableSlice. view.rs configurable_slice: BFS from a seed to configurable DEPTH over a SELECTABLE set of edge KINDS (empty=all) in a DIRECTION (SliceDir Down/Up/Both) — generalizes element_neighbourhood (fixed depth-1/both/all). slice_json emits the induced subgraph (reuses section_subgraph_json). serve.rs: GET /api/slice?seed&depth&edges&dir, added to the committed KEEL_API_READ_ENDPOINTS (API 1.0.0 -> 1.1.0 minor bump). D0047 unit test configurable_slice_respects_depth_dir_and_edges (depth/dir/edge-filter/unknown-seed). One endpoint serves THREE Needs: N-4 (configurable slice), N-2 (seed-and-expand = depth-limited slice, the graph-API half of viewerGraphExpand), N-10 (change-impact = dir=Up, the dependents reach of viewerChangeImpact). VERIFIED LIVE: /api/version 1.1.0/16 endpoints; /api/slice?seed=nViewerSeparation&depth=2 -> 7 items; dir=up on an SR -> the upstream reach. VERIFY: clippy --workspace --all-targets clean; cargo test (85) green; keel validate + guard + check-engine green."; }
        part storyViewerSliceDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c215-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "109fa45"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    }

    #Verify dependency from storyViewerSliceDoD to srViewerConfigurableSlice;

    verification vslRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c215-51627384c004"; :>> title = "Sprint 215 refine gate (DoR)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the configurable slice is the graph backbone (seed for critique-over-downstream, change-impact, explore). Generalize element_neighbourhood; expose /api/slice. estimatedPoints=2. #CharteredBy d0114. DoR met."; }
    part vslRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c215-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "109fa45"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification vslStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c215-51627384c006"; :>> title = "Sprint 215 standup gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: SliceDir + configurable_slice (BFS) + slice_json; /api/slice route + handler; contract + version bump; unit test. No blockers."; }
    part vslStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c215-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "109fa45"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification vslImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c215-51627384c008"; :>> title = "Sprint 215 implement gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "/api/slice live (depth/dir verified); API 1.1.0/16 endpoints; unit test green. clippy --workspace --all-targets clean; cargo test (85) green; keel validate + guard + check-engine green."; }
    part vslImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c215-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "109fa45"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification vslReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c215-51627384c00a"; :>> title = "Sprint 215 review gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. High-leverage brick: one small, well-tested traversal generalization (element_neighbourhood -> configurable_slice) exposed as /api/slice serves THREE Needs (N-4 directly; N-2 as depth-limited; N-10 as dir=Up) and is the seed mechanism the critique flows (UC-14 process+downstream) depend on. Reuses section_subgraph_json for emit (no new render path). The version bump correctly signals a minor, backward-compatible API addition. Clean; frontend graph interactivity (incremental layout) belongs to the separate viewer app, not keel."; }
    part vslReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c215-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "109fa45"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification vslCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c215-51627384c00c"; :>> title = "Sprint 215 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; /api/slice is live (API 1.1.0), viewerConfigurableSlice DONE (N-4 satisfied); N-2/N-10 enabled. clippy + cargo test + keel validate + guard + check-engine green. Self-evidencing (D0051). NEXT: viewerIterativeCritique (N-15) — iterate element-critique over a /api/slice result (the human's core goal, UC-14/UC-15)."; }
    part vslCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c215-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "109fa45"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification vslRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c215-51627384c00e"; :>> title = "Sprint 215 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: picked the highest-leverage next brick — a configurable slice that THREE Needs + the critique use-cases all sit on — and delivered it by GENERALIZING an existing function (element_neighbourhood -> depth/dir/edge-filtered) rather than a new subsystem, reusing the existing subgraph emit. Live-verified all three modes (depth, dir=up change-impact, version bump). AVOIDABLE-ISSUE SCAN: a handful of clippy nits on the new code (visibility, then_some, single-item into_iter) — all caught by the authoritative --workspace --all-targets clippy BEFORE commit, which is now habitual (the a642269 lesson holding). LESSON: build the shared substrate (slice) before the features (critique) that iterate over it. Estimate 2 pt held. NEXT: viewerIterativeCritique over /api/slice."; }
    part vslRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c215-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "109fa45"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
}
