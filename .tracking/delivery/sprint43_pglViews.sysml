// =====================================================================
// ProjectDeliveryS43 — Sprint 43: pglViews (D0068/D0069/D0070, Increment 3).
// Story: the git-traversal process-governance RESOLVER — query.py governing-version /
//        reprocess-candidates: compute (never store) the process version that governed a
//        work item AS-OF its charter (charter-time freeze, D0068), correlated with the
//        process-change Decisions in force then vs. after (D0070), reusing _is_ancestor +
//        git show. Declared as viewpoints (D0056/D0057). The payoff of Inc 1+2.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049). Closes the processGovernanceLineage epic.
// =====================================================================
package ProjectDeliveryS43 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // dogfood: chartered by D0069 (which specified the git-traversal resolver).
    #CharteredBy dependency from pglViewsStory to d0069;

    part pglViewsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b043-516273849501";
        :>> title = "Sprint 43: pglViews (5 pts) — git-traversal governance resolver; D0068/D0069/D0070 Increment 3";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS43 {
        action storyPglViews;
        verification storyPglViewsDoD : Test { :>> id = "63940516-2c7d-4e8f-b043-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "pglViewsDoD (pass): query.py governing-version <item> + reprocess-candidates added (pure computation, no new authored facts), reusing _is_ancestor + _file_at (git show). governing-version(item) = the process-def state at the latest change-commit that is an ancestor of the item's introduction commit (charter-time freeze, D0068), correlated with process-change Decisions in force at vs. after the charter (effective commit = acceptance-event judgedAgainst, D0070); process-as-it-was via git show; reprocess-candidates = items under a later SAFETY change (empty now — prospective default, D0062). work->process by convention (D0069). DEMONSTRATED on a real item: governing-version pglCharterEdgeStory -> governing delivery.sysml @6f965c0, decisions-in-force d0039/d0049/d0064/d0067, d0070 after-charter+prospective -> reprocess_required false. Declared governingVersionVP + reprocessVP viewpoints. validate_all green (incl. validate_instances for the registry)."; }
        part storyPglViewsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b043-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ea13ca9"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
    }

    verification pglViewsRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b043-516273849503";
        :>> title = "Sprint 43 refine gate (DoR)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0068/D0069/D0070 Increment 3, the resolver. Inputs are now guaranteed by the sprint41/42 guards (process-change markers + charter edges), so the resolver computes on solid ground. Design: pure git ancestry (charter-time freeze) — the governing version is the process-def state as-of the item's intro commit; decisions correlate by effective-commit ancestry; reprocess only on safety (D0062). Honest about the convention (work->process by kind) + grandfathered historical linkage (git-derived governed_defs empty for pre-guard decisions — not fabricated, D0067). estimatedPoints=5 (git-traversal + 2 viewpoints + demo). No blocking dependency. DoD accepted.";
    }
    part pglViewsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b043-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ea13ca9"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglViewsStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b043-516273849505";
        :>> title = "Sprint 43 standup gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: query.py resolver functions — _item_intro_commit (git log -S --reverse), _def_change_commits, process_change_decisions_full (effective commit = judgedAgainst; governed_defs = process-defs in the decision's intro commit), governing_version, reprocess_candidates. CLI: governing-version <item>, reprocess-candidates. Declare governingVersionVP + reprocessVP in viewpoint-registry. doc-sync usage.md. No blockers.";
    }
    part pglViewsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b043-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ea13ca9"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglViewsImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b043-516273849507";
        :>> title = "Sprint 43 implement gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "Resolver added to query.py (reuses _is_ancestor + _file_at). governing-version pglCharterEdgeStory: item_commit 6d639fc -> governing delivery.sysml @6f965c0; decisions_in_force_at_charter d0039/d0049/d0064/d0067; d0070 after-charter (prospective) -> reprocess_required false; later_version_count 0. reprocess-candidates: [] (prospective default, D0062). governingVersionVP + reprocessVP declared (registry 12 viewpoints; validate_instances green). usage.md: 2 rows. pglViews backlog DoD recorded pass.";
    }
    part pglViewsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b043-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ea13ca9"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglViewsReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b043-516273849509";
        :>> title = "Sprint 43 review gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the resolver STORES nothing — it computes governing-version purely from git ancestry over authored inputs the guards guarantee (D0001/D0005 compute-don't-store; D0013 git temporal). Charter-time freeze (D0068) is literal: the as-of anchor is the item's intro commit. Prospective default honored (D0062): the only post-charter change (d0070) does not flag the sprint38 item. Honest limits surfaced, not hidden: the work->process convention (sprint Story -> Delivery) and grandfathered historical git-linkage (governed_defs empty pre-guard, never fabricated — D0067). Phase-3 scan: this closes Increment 3 and the processGovernanceLineage epic; the three desired outputs of D0069 (accept-then-process / audit-regenerate / minimal-metadata) are all served.";
    }
    part pglViewsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b043-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ea13ca9"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglViewsCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b043-51627384950b";
        :>> title = "Sprint 43 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + pglViews DoD recorded pass; validate_all green; resolver demonstrated on real data. No human gate (D0051: pure computation over authored facts, self-evidencing via the live demonstration; the design was accepted in D0069/D0070).";
    }
    part pglViewsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b043-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ea13ca9"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglViewsRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b043-51627384950d";
        :>> title = "Sprint 43 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. WELL: building the guards FIRST (sprint41/42) meant the resolver consumes guaranteed inputs — no defensive handling of missing markers/charters. The honest-limits stance (convention + grandfathered linkage surfaced in the output, not hidden) keeps the view trustworthy. The reprocess-candidates view returning [] is a real result (prospective default), not a stub. NOTE for future: the work->process convention is currently a single map entry (Story->Delivery); as more work kinds gain processes it should generalize (a small follow-on, not a gap today). No avoidable issue. processGovernanceLineage epic ready to roll up. Next frontier: queryDrivenViewpoints (CHANGE — needs human acceptance). Estimate 5 pts held.";
    }
    part pglViewsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b043-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ea13ca9"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
}
