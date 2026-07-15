// ProjectDeliveryS238 — Sprint 238: viewerFoundationClose — mark the already-delivered foundational
// viewer items done (human-directed: "mark delivered + build the 2 real ones"). Each was shipped across
// prior sprints + verified; the stable-on-expand gap (N-2) was closed first. EXECUTE. 1 pt.
package ProjectDeliveryS238 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewerFoundationCloseStory to d0114;

    part viewerFoundationCloseStory : Story {
        :>> id = "63940516-2c7d-4e8f-c238-51627384c001";
        :>> title = "Sprint 238: viewerFoundationClose (1 pt) - mark delivered viewer foundations done (keel API, graph-expand, visual viewpoint, generative UI)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS238 {
        action storyViewerFoundationClose;
        verification storyViewerFoundationCloseDoD : Test { :>> id = "63940516-2c7d-4e8f-c238-51627384c002"; :>> method = VerificationMethod::inspect; :>> procedureText = "Mark the four already-delivered foundational viewer items done (honest — pointing at the shipped work, D0098 no faked pass): (1) viewerKeelApi (srViewerKeelApi/N-6) — the committed, versioned read/write API, KEEL_API v1.11.0 with /api/version advertising read+write endpoints, CORS, and the schema/slice/change-impact/snapshot/baseline-compare/item/section endpoints (S220-236). (2) viewerGraphExpand (srViewerGraphExpand/N-2) — seed-and-expand + drill-in + ELK layered STABLE layout; the stable-on-expand gap CLOSED this sprint (placeNewNodes keeps existing positions; verified 0 existing nodes moved on expand; sysmlv2-viewer@17c2543). (3) viewerVisualViewpoint (srViewerVisualViewpoint/N-1) — a declarable VISUAL form beyond the 3 fixed modes: the encoding viewpoints (colour/x/swimlane/nested/size/shape channel bindings, modular D0124) + the shareable export (N-12) declare + share a visual form. (4) viewerGenerativeUi (srViewerGenerativeUi/N-17) — the viewer generates its UI from the declared model (types/attributes/actions), nothing per-type hardcoded (S223 + the whole viewer). VERIFY: each item's DoD result appended (done); the stable-on-expand fix verified live; keel validate + guard + check-engine green. HONEST NOTE: persisted NAMED viewpoints (save/load) are N-8 (a separate future need); the shareable-URL export is the current 'declare/share' path."; }
        part storyViewerFoundationCloseDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c238-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a0371ac"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    }

    verification fcRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c238-51627384c004"; :>> title = "Sprint 238 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: human directed marking the delivered foundations done. Scope: close the stable-on-expand gap, then append DoD results honestly. #CharteredBy d0114. estimatedPoints=1. DoR met."; }
    part fcRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c238-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a0371ac"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification fcStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c238-51627384c006"; :>> title = "Sprint 238 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: verify each foundation is genuinely delivered (not fake a pass); close the one gap (stable-on-expand); append DoD results. No blockers."; }
    part fcStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c238-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a0371ac"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification fcImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c238-51627384c008"; :>> title = "Sprint 238 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Stable-on-expand shipped (17c2543, verified 0 moved); the 4 foundation DoD results appended. validate/guard/check-engine green."; }
    part fcImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c238-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a0371ac"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification fcReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c238-51627384c00a"; :>> title = "Sprint 238 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Each marked item is genuinely delivered — and where a DoD clause wasn't met (graph-expand's 'stable on expand'), the gap was CLOSED before marking rather than faking a pass (D0098). The one honest caveat (persisted named viewpoints = N-8) is noted, not hidden. Clean."; }
    part fcReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c238-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a0371ac"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification fcCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c238-51627384c00c"; :>> title = "Sprint 238 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): the 4 foundations are marked done honestly. Self-evidencing. NEXT: build the two with real remaining work — viewerIterativeCritique (N-15) then viewerInProgramEdit (N-16)."; }
    part fcCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c238-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a0371ac"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification fcRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c238-51627384c00e"; :>> title = "Sprint 238 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: when asked to 'mark delivered', I AUDITED each DoD honestly instead of rubber-stamping — found + closed a real gap (stable-on-expand) and flagged an honest caveat (named viewpoints = N-8) rather than faking passes. AVOIDABLE-ISSUE SCAN: none. LESSON: 'mark delivered' still means verify the DoD is actually met; close small gaps, flag the rest. Estimate 1 pt. NEXT: viewerIterativeCritique, viewerInProgramEdit."; }
    part fcRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c238-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a0371ac"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
