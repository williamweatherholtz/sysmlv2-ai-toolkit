// =====================================================================
// ProjectDeliveryS105 — Sprint 105: orientDashboard (D0093 final milestone — COMPLETES the spin-up arc).
// `sysmlv2 orient --html` emits the human's recurring oversight dashboard (progress, ready frontier,
// sprints in progress, open issues, suspect/stale, assurance readiness) as a self-contained computed
// #View, reusing the report card template. Static (the human-chosen mode over a live server, after
// research: a file:// page can't read local files + a JS reimpl would duplicate the Rust authority;
// regeneration is sub-second). The dual-surface invariant (D0093) joins CLAUDE.md section 2.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS105 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from orientDashboardStory to d0093;

    part orientDashboardStory : Story {
        :>> id = "63940516-2c7d-4e8f-c105-51627384c001";
        :>> title = "Sprint 105: orientDashboard (2 pts) — `sysmlv2 orient --html` human dashboard; completes the D0093 spin-up arc";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS105 {
        action storyOrientDashboard;
        verification storyOrientDashboardDoD : Test { :>> id = "63940516-2c7d-4e8f-c105-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0093 final milestone: `sysmlv2 orient --html` emits the human dashboard. view.rs orient_html reuses the report card template (REPORT_TEMPLATE/TABLE_STYLE/card) over orient::compute + compute_readiness: cards = Progress (done/outstanding), Ready to start, Sprints in progress (+ pending gate), Open issues, Suspect/stale, Assurance readiness. cmd_orient gains a --html flag (flag-aware root parse). STATIC mode (human-chosen after research: a file:// page cannot reliably read local files [opaque-origin CORS] and a JS recompute would duplicate the Rust authority; a live server is a deferred future option). doc-sync: orient viewpoint renderer (--html), usage.md dashboard row, main.rs help, CLAUDE.md section-2 dual-surface invariant (D0093), D0093 rollout note. VERIFY (live): `sysmlv2 orient . --html` emits self-contained HTML with all 6 cards; clippy --workspace -D warnings + cargo test (47 lib) + rust validate + guard all (13) green; kernel validate_instances PASS (viewpoint-registry + d0093)."; }
        part storyOrientDashboardDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c105-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5a8d9fb"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification orientDashboardRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c105-51627384c004"; :>> title = "Sprint 105 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0093's last milestone. The human, after research on live-refresh (server vs static), chose STATIC (`orient --html`) — the leaner, on-brand mode matching diagram/render/report; a live `serve` is a documented future option. Resolver: orientDashboard. estimatedPoints=2. #CharteredBy d0093. DoD accepted."; }
    part orientDashboardRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c105-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5a8d9fb"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification orientDashboardStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c105-51627384c006"; :>> title = "Sprint 105 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: orient_html(root) reuses REPORT_TEMPLATE + card() over orient::compute + compute_readiness; cmd_orient --html flag; doc-sync (viewpoint renderer, usage.md, help, CLAUDE.md dual-surface invariant, d0093 rollout). No schema/process-def change -> no keystone (kernel re-validates the touched .engine instances). No blockers."; }
    part orientDashboardStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c105-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5a8d9fb"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification orientDashboardImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c105-51627384c008"; :>> title = "Sprint 105 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented orient_html + cmd_orient --html + doc-sync. VERIFY (live): `sysmlv2 orient . --html` emits self-contained HTML with Progress/Ready to start/Sprints in progress/Open issues/Suspect/Assurance readiness cards; clippy -D warnings green; cargo test 47 lib pass; rust validate green; guard all (13) PASS; kernel validate_instances PASS (viewpoint-registry + d0093)."; }
    part orientDashboardImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c105-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5a8d9fb"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification orientDashboardReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c105-51627384c00a"; :>> title = "Sprint 105 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The dashboard is a computed #View (regenerate-don't-commit) reusing the report template — consistent with diagram/render/report; it surfaces the human's most-asked question (where things stand) drilling down to the JSON authority. STATIC was the right call (research-backed: file:// can't read local files, JS recompute would split the authority; regeneration is sub-second; a live server stays a clean future upgrade). The dual-surface invariant is now explicit in CLAUDE.md section 2. This COMPLETES the D0093 spin-up arc (init -> introduction -> orient dashboard). Clean."; }
    part orientDashboardReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c105-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5a8d9fb"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification orientDashboardCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c105-51627384c00c"; :>> title = "Sprint 105 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyOrientDashboard DoD recorded pass; `orient --html` live; dual-surface invariant in CLAUDE.md; clippy + cargo test (47) + rust validate + guard all (13) + kernel green. Self-evidencing (D0051). D0093 spin-up arc COMPLETE — all three milestones (introductionProcess, engineInit, orientDashboard) delivered. The engine is now spin-up-able as a product."; }
    part orientDashboardCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c105-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5a8d9fb"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification orientDashboardRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c105-51627384c00e"; :>> title = "Sprint 105 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: researching the live-refresh question BEFORE building surfaced the real constraint (file:// can't read local files; a JS recompute splits the Rust authority) and let the human make an INFORMED static-vs-server call rather than over-building a server speculatively; reusing the report card template made the dashboard a ~1-card-array function (fast, consistent). This COMPLETES D0093 — the engine can now be spun up on a new project (init), onboard a newcomer (introduction), and give the human a live-enough dashboard (orient --html). AVOIDABLE-ISSUE SCAN: no avoidable defect this sprint (the orient_html doc-paragraph clippy nit was caught + fixed inline, a trivial one-off). Two NON-blocking forward options remain documented, NOT tracked as debt: (1) `sysmlv2 serve` (live SSE dashboard) if a mission-control / in-browser-action need ever materializes; (2) a CI init-smoke-test (scaffold + guard a temp project) to lock the engine/instance boundary against future instance-specific files (the engineInit lesson). Estimate 2 pts held. The engine-as-product (D0093) is DONE."; }
    part orientDashboardRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c105-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5a8d9fb"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
