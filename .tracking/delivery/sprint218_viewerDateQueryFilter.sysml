// ProjectDeliveryS218 — Sprint 218: viewerDateQueryFilter — TIME AS A QUERY FILTER (N-5, the human's
// actual temporal want per D0116): /api/slice gains dateattr+since+until, keeping only members whose date
// attribute (e.g. judgedAt, createdAt) falls in an ISO range. Delivers viewerDatePredicates. API 1.2.0 ->
// 1.3.0. EXECUTE. 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS218 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewerDateFilterStory to d0114;

    part viewerDateFilterStory : Story {
        :>> id = "63940516-2c7d-4e8f-c218-51627384c001";
        :>> title = "Sprint 218: viewerDateQueryFilter (2 pt) - /api/slice date-range filter (N-5, time-as-query-filter)";
        :>> createdAt = "2026-07-15"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS218 {
        action storyViewerDateFilter;
        verification storyViewerDateFilterDoD : Test { :>> id = "63940516-2c7d-4e8f-c218-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Satisfies srViewerDatePredicates (N-5) + delivers viewerDatePredicates — the human's actual temporal want (D0116: time-as-query-filter, NOT a due-state). view.rs: date_in_range (ISO-8601 lexicographic = chronological, no parsing) + DateFilter{attr,since,until}; slice_json keeps only members whose date attribute is in [since,until]. serve.rs: /api/slice gains dateattr+since+until (backward-compatible optional params); API 1.2.0 -> 1.3.0. So a query filters by time — 'created/reviewed since/until/in a range' over authored date facts (judgedAt/createdAt), e.g. 'processes not reviewed since <date>' re-serves UC-4. D0047 unit tests: date_in_range_iso_lexicographic (bounds, inclusive-lower, before/after) + configurable_slice test. SCOPE HONESTY: delivers the date-RANGE predicate on the explore surface (the human's want); broader query-vocabulary items (OR, declared-ViewSpec-select integration) are deferred — not requested. VERIFIED LIVE: /api/version 1.3.0; /api/slice depth-3 -> 9; +dateattr=createdAt&since=2026-07-14 -> 9; &since=2027-01-01 -> 0. VERIFY: clippy --workspace --all-targets clean; cargo test (86) green; keel validate + guard + check-engine green."; }
        part storyViewerDateFilterDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c218-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4d9e10b"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    }

    #Verify dependency from storyViewerDateFilterDoD to srViewerDatePredicates;

    verification vdfRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c218-51627384c004"; :>> title = "Sprint 218 refine gate (DoR)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0116 refocused temporal onto N-5 (time-as-query-filter). Build a date-range filter on the slice/query. ISO-8601 lexicographic compare = trivial + correct. estimatedPoints=2. #CharteredBy d0114. DoR met."; }
    part vdfRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c218-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4d9e10b"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vdfStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c218-51627384c006"; :>> title = "Sprint 218 standup gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: date_in_range + DateFilter; slice_json retains date-matching members; /api/slice params; version bump; unit test. No blockers."; }
    part vdfStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c218-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4d9e10b"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vdfImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c218-51627384c008"; :>> title = "Sprint 218 implement gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "date filter live (since/until verified: 9 -> 9 -> 0); API 1.3.0; DateFilter struct kept slice_json under the arg limit. clippy --workspace --all-targets clean; cargo test (86) green; keel validate + guard + check-engine green."; }
    part vdfImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c218-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4d9e10b"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vdfReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c218-51627384c00a"; :>> title = "Sprint 218 review gate"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Delivered EXACTLY the human's reframed want (time-as-query-filter) — no more (no due-state machine, which D0116 cut). The ISO-8601-lexicographic trick makes it parse-free + correct; the DateFilter struct is a clean fix for the arg-count lint (not a suppression). Backward-compatible optional params (minor bump). Honestly scoped: date-range on the explore surface now; OR + declared-select integration deferred as un-requested. Clean."; }
    part vdfReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c218-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4d9e10b"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vdfCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c218-51627384c00c"; :>> title = "Sprint 218 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; /api/slice filters by time (API 1.3.0); viewerDatePredicates DONE (N-5 core, the human's temporal want). clippy + cargo test + keel validate + guard + check-engine green. Self-evidencing (D0051). The read/explore/critique/time-filter API surface is now substantial; NEXT: N-16 in-program modify (via keel processes, per the clarified SR) — the write-side."; }
    part vdfCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c218-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4d9e10b"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
    verification vdfRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c218-51627384c00e"; :>> title = "Sprint 218 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-15"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the Q2 detour paid off fully — instead of a due-state machine nobody wanted, a tiny date-range filter (parse-free ISO compare, ~10 lines) delivered the human's real temporal need, live-verified. Reused the slice endpoint (no new surface). AVOIDABLE-ISSUE SCAN: the 8th-arg lint — fixed properly with a DateFilter struct (not #[allow]), which is also cleaner API. LESSON: ISO-8601 lexical ordering removes the need for date parsing; and grouping params into a struct beats suppressing too_many_arguments. Estimate 2 pt held. NEXT: the write-side (N-16 via keel processes)."; }
    part vdfRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c218-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4d9e10b"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "claudeOpus"; }
}
