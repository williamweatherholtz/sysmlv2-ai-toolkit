// ProjectDeliveryS151 — Sprint 151: serveRecency (verifies sr15ServeIntrospect) — the introspection
// recency lens: `keel recent` (git-derived recent-activity timeline) + a console Recent tab + Decisions
// sorted newest-first. EXECUTE (serve redesign). 3 pts. AUTONOMOUS (D0049).
package ProjectDeliveryS151 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from serveRecencyStory to sr15ServeIntrospect;

    part serveRecencyStory : Story {
        :>> id = "63940516-2c7d-4e8f-c151-51627384c001";
        :>> title = "Sprint 151: serveRecency (3 pts) — git-derived recent-activity timeline + Recent tab + decisions newest-first (verifies sr15)";
        :>> createdAt = "2026-06-24"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS151 {
        action storyServeRecency;
        verification storyServeRecencyDoD : Test { :>> id = "63940516-2c7d-4e8f-c151-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Satisfies sr15ServeIntrospect's recency clauses (live-state + decision-context were already met). FIX: (1) `keel recent` + GET /api/recent — a git-derived recent-activity timeline (git log of the latest 25 commits touching .tracking/.engine + the element files each changed, newest first; uncached, git-sourced); (2) console Recent tab rendering it; (3) Decisions tab sorted newest-first (dNNNN desc; the modal already shows full context/decision/rationale/consequences). Added a `recent` Viewpoint + VIEW_SUBCOMMANDS entry. VERIFIED (live): `keel recent .` = 25 commits; GET /api/recent = 25; console has the Recent tab; clippy -D warnings + cargo build + keel validate + guard all (12) green. sr15Verify (#Verify -> sr15) records it; tier-satisfaction SR-verified ticks up. Remaining serve SRs: sr16 (critique loop), sr18 (section critique), sr19 (white-box boundary)."; }
        part storyServeRecencyDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c151-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "744ffe4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "claudeOpus"; }
    }

    verification serveRecencyRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c151-51627384c004"; :>> title = "Sprint 151 refine gate (DoR)"; :>> createdAt = "2026-06-24"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: sr15's only unmet clause is the git-derived recent-activity timeline (+ decisions newest-first). Smallest serve SR, pure read-path. estimatedPoints=3. #CharteredBy sr15ServeIntrospect. DoR met."; }
    part serveRecencyRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c151-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "744ffe4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "claudeOpus"; }
    verification serveRecencyStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c151-51627384c006"; :>> title = "Sprint 151 standup gate"; :>> createdAt = "2026-06-24"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: `recent` view (git_out log parse) + /api/recent route (uncached) + Recent tab + decisions newest-first sort + Viewpoint/known-command. No blockers."; }
    part serveRecencyStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c151-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "744ffe4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "claudeOpus"; }
    verification serveRecencyImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c151-51627384c008"; :>> title = "Sprint 151 implement gate"; :>> createdAt = "2026-06-24"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Added recent()/recent_flush + /api/recent + Recent tab + decisions newest-first + recent Viewpoint + VIEW_SUBCOMMANDS. VERIFIED live: keel recent 25 commits; /api/recent 25; Recent tab present; clippy + build + validate + guard 12 green."; }
    part serveRecencyImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c151-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "744ffe4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "claudeOpus"; }
    verification serveRecencyReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c151-51627384c00a"; :>> title = "Sprint 151 review gate"; :>> createdAt = "2026-06-24"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The recent-activity timeline is computed from git (compute-don't-store), uncached (git is outside the model fingerprint), and the Recent tab + newest-first decisions give the 'what changed recently and why' lens directly (click any decision -> full context). Honest, small, verifies sr15. Clean."; }
    part serveRecencyReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c151-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "744ffe4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "claudeOpus"; }
    verification serveRecencyCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c151-51627384c00c"; :>> title = "Sprint 151 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-24"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; recency lens live; sr15 verified. clippy + build + keel validate + guard all (12) green. Self-evidencing (D0051). 2 of 5 serve SRs done (sr17, sr15); sr16/sr18/sr19 remain."; }
    part serveRecencyCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c151-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "744ffe4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "claudeOpus"; }
    verification serveRecencyRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c151-51627384c00e"; :>> title = "Sprint 151 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-24"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: another serve SR built + verified, tier-satisfaction ticks up honestly. The recent view reuses git_out (no new git plumbing) + the established view/subcommand/Viewpoint/route pattern. AVOIDABLE-ISSUE SCAN: caught (during this sprint) that the console fetched /api/recent before the route existed — added the serve route; a reminder to wire BOTH the CLI subcommand AND the /api route for any console-facing view. LESSON: a console view needs three things in lockstep — the view fn, the /api route, and the tab; missing the route 404s silently. NON-blocking next: sr16 (critique loop — may need input on whether the console drives an AI that WRITES the fix), sr18 (section critique), sr19 (white-box boundary). Estimate 3 pts held."; }
    part serveRecencyRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c151-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "744ffe4"; :>> judgedAt = "2026-06-24"; :>> judgedBy = "claudeOpus"; }
}
