// =====================================================================
// ProjectDeliveryS110 — Sprint 110: serveLiveCache (D0094 console deepening — PERF + EVENT-DRIVEN).
// A content FINGERPRINT (path+len+mtime over .tracking + .engine .sysml) drives both: (a) a per-view
// JSON cache (recompute a view ONLY when the model changes — orient cold ~2.2s -> warm ~0.11s, decisions
// ~0.6s -> ~0.12s; frontend pre-loads the light views to warm it); (b) GET /api/events SSE that polls
// the fingerprint server-side (~1.5s) and pushes `changed` ONLY when it flips, so the UI refetches
// EVENT-DRIVEN, not on a blind 6s timer (hash unchanged => unchanged; catches out-of-purview edits).
// Plus a keel-serve.log file sink. A materialized #View cache (regenerable, section-2.1).
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS110 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveLiveCacheStory to d0094;

    part serveLiveCacheStory : Story {
        :>> id = "63940516-2c7d-4e8f-c110-51627384c001";
        :>> title = "Sprint 110: serveLiveCache (5 pts) — content-hash view cache + SSE event-driven change-push + log file (D0094 perf)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS110 {
        action storyServeLiveCache;
        verification storyServeLiveCacheDoD : Test { :>> id = "63940516-2c7d-4e8f-c110-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0094 perf+events (the human's slow-UI + event-driven asks). FINGERPRINT: fingerprint(root) folds (path+len+mtime) over .tracking + .engine .sysml — cheap, catches out-of-purview edits. CACHE: cached(state,key,compute) serves a view's JSON from a per-fingerprint HashMap, recomputing ONLY when the fingerprint flips (orient/decisions/dispositions/processes/report:<name> cached; history uncached — reads ~/.claude). EVENTS: GET /api/events is an SSE stream that polls the fingerprint (~1.5s) + pushes `changed` only on a flip (hello/ping/changed); the frontend replaces its 6s poll with an EventSource on /api/events (refetch on `changed`; a slow 30s safety-net only) + pre-loads the light views on connect. LOG: log_request also appends to keel-serve.log (gitignored *.log). A materialized #View cache (regenerable, section-2.1). VERIFY (LIVE, measured this sprint): orient cold 2.19s -> warm 0.11s; decisions 0.60s -> 0.12s; /api/events emitted hello, ping, then `changed` the instant a tracked file was touched, then ping; keel-serve.log written with per-request timings. clippy --workspace -D warnings + cargo test (46 lib) + keel validate 123 + guard all (13) green."; }
        part storyServeLiveCacheDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c110-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90e6493"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveLiveCacheRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c110-51627384c004"; :>> title = "Sprint 110 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the human reported the UI is too slow (~2s/load) + wants event-driven updates not polling, intuiting a content hash. Plan (confirmed): serveLiveCache first (cache + events + log file), then introspect, then actions. Resolver: serveLiveCache. estimatedPoints=5. #CharteredBy d0094. DoD accepted."; }
    part serveLiveCacheRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c110-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90e6493"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveLiveCacheStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c110-51627384c006"; :>> title = "Sprint 110 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: AppState += cache (Mutex<HashMap<key,(fp,json)>>); fingerprint(); cached() (lock-check-clone-unlock, recompute outside the lock, store; guards dropped tight for clippy nursery); rewrite the 5 cacheable handlers; api_events SSE (async-stream + tokio time sleep); /api/events route; log_request appends keel-serve.log. Frontend: EventSource(/api/events) refetch-on-changed + pre-load + 30s safety-net. Verify by measuring cold-vs-warm + a touch-triggered `changed`. No .engine/schema change -> no kernel/keystone. No blockers."; }
    part serveLiveCacheStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c110-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90e6493"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveLiveCacheImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c110-51627384c008"; :>> title = "Sprint 110 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the fingerprint cache + /api/events + log file + frontend event-driven refresh + pre-load. VERIFY (live): orient 2.19s->0.11s; decisions 0.60s->0.12s; /api/events hello/ping/changed(on touch)/ping; keel-serve.log written. clippy -D warnings green (fixed: significant_drop_tightening on the cache guards); cargo test 46 lib pass; keel validate 123; guard all (13) PASS."; }
    part serveLiveCacheImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c110-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90e6493"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveLiveCacheReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c110-51627384c00a"; :>> title = "Sprint 110 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. One content fingerprint unifies BOTH asks exactly as the human intuited: it keys the per-view cache (20x faster warm loads; recompute only on real change) AND drives the SSE `changed` push (event-driven, no blind polling). The cache is a legitimate materialized #View (regenerable, invalidated by content hash — honors section-2.1; no second source of truth). history stays uncached (its source ~/.claude is outside the fingerprint). The frontend is now event-driven (EventSource) with a slow safety-net. Logs land in keel-serve.log too. Foundation for serveItemIntrospect + serveItemActions. Clean."; }
    part serveLiveCacheReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c110-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90e6493"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveLiveCacheCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c110-51627384c00c"; :>> title = "Sprint 110 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyServeLiveCache DoD recorded pass; cache + event-driven push + log file live + measured (cold->warm 20x; changed-on-touch). clippy + cargo test + keel validate 123 + guard all (13) green. Self-evidencing (D0051). The UI is fast + event-driven; serveItemIntrospect (generic expand/introspect) + serveItemActions (per-item critique/add-testresult) are next."; }
    part serveLiveCacheCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c110-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90e6493"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification serveLiveCacheRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c110-51627384c00e"; :>> title = "Sprint 110 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the human's own intuition (a content hash) was exactly right and collapsed two asks (perf + event-driven) onto ONE primitive — the fingerprint keys the cache AND triggers the push, so the design stayed small instead of ballooning; measuring cold-vs-warm + a live touch-triggered `changed` proved both end-to-end (not asserted). The cache is a clean materialized #View (regenerable, hash-invalidated) so it does not violate compute-don't-store. AVOIDABLE-ISSUE SCAN: one avoidable build-blocker caught + fixed — a stale `keel.exe` server held the binary (cargo could not relink) so kill it before rebuilding when serve is running (lesson: stop the running server before `cargo build`); plus a clippy nursery significant_drop_tightening on the MutexGuard, fixed by scoping the guard tight (clone-then-drop before building the response). NON-blocking forward notes (NOT debt): (1) the FIRST load of each view is still a cold recompute (cache miss) — a shared cached Model across views (vs re-Model::build per view) would cut even cold loads, deferred; (2) report:assurance cold is still ~17s (readiness recompute) — its own perf pass; (3) the SSE fingerprint loop runs per connected client (fine for localhost). Estimate 5 pts held. UI is fast + event-driven; introspect/actions next (serveItemIntrospect)."; }
    part serveLiveCacheRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c110-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90e6493"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
