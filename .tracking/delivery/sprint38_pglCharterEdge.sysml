// =====================================================================
// ProjectDeliveryS38 — Sprint 38: pglCharterEdge (3 pts).
// Story: Increment 1 of D0068 — a typed #CharteredBy edge (work item -> the
//        originating Decision/Need/Requirement that chartered it). Resolves the
//        core of issue002 (sprint->process linkage). Demonstrated + traversable.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049). Gate results via append-gate-result.
// =====================================================================
package ProjectDeliveryS38 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;  // #CharteredBy marker

    // dogfood: this sprint (which built the charter edge) is itself chartered by D0068.
    #CharteredBy dependency from pglCharterEdgeStory to d0068;

    part pglCharterEdgeStory : Story {
        :>> id = "63940516-2c7d-4e8f-b038-516273849501";
        :>> title = "Sprint 38: pglCharterEdge (3 pts) — typed #CharteredBy edge (work -> origin); D0068 Increment 1";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS38 {
        action storyPglCharterEdge;
        verification storyPglCharterEdgeDoD : Test { :>> id = "63940516-2c7d-4e8f-b038-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "pglCharterEdgeDoD (pass): metadata def CharteredBy added to relationships.sysml; >=1 #CharteredBy link demonstrated (sprint29 story -> d0064; sprint38 -> d0068); query.py charter <item> traverses work->charter; validate_schema 14/14 + validate . + check green."; }
        part storyPglCharterEdgeDoDR1 : TestResult { :>> id = "3aa73650-e022-4768-9d71-9abb307681fc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6d639fc"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification pglCharterEdgeRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b038-516273849503";
        :>> title = "Sprint 38 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0068 Increment 1 (charter edge). Investigated the edge mechanism first (migration/care discipline): no native form -> a metadata marker on a dependency (`#Marker dependency from a to b`), the engine's confirmed custom-edge idiom (syntax-notes; parser test; validated). Frozen-schema change, pre-authorized by D0068. No blocking dependency. estimatedPoints=3. DoD accepted.";
    }
    part pglCharterEdgeRefineGateR1 : TestResult { :>> id = "976e252e-63a7-413c-8b1a-ccc35245c334"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4a1f674"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEdgeStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b038-516273849504";
        :>> title = "Sprint 38 standup gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: metadata def CharteredBy in relationships.sysml (mirrors #Supersede); demonstrate `#CharteredBy dependency from <story> to <decision>` (import EngineRelationships); query.py read_charter_edges() + `charter` command (kernel-free) for traversability; doc-sync usage.md + cheatsheet. No blockers.";
    }
    part pglCharterEdgeStandupGateR1 : TestResult { :>> id = "5fee6ef5-4e9a-4408-8b1a-ccc3594ddc34"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4a1f674"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEdgeImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b038-516273849505";
        :>> title = "Sprint 38 implement gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "metadata def CharteredBy added (relationships.sysml) — validate_schema 14/14. Demonstrated: #CharteredBy dependency sprint29 story -> d0064 (+ sprint38 -> d0068 dogfood); sysmlv2 validate . 46 clean + check clean. query.py charter <item> traverses (processRigorHardeningStory -> d0064). doc-sync: usage.md row + edge cheatsheet. pglCharterEdge backlog DoD recorded pass. Resolves the core of issue002.";
    }
    part pglCharterEdgeImplementGateR1 : TestResult { :>> id = "09625841-f05a-4bfc-9d71-9abb0b1e7a04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6d639fc"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEdgeReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b038-516273849506";
        :>> title = "Sprint 38 review gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the edge uses the confirmed `#Marker dependency` idiom (kernel-validated, not invented syntax); from/to are name refs (like satisfy/relatedTask), traversed by query.py without kernel cross-resolution. Phase-3 scan: this is the authored IDENTITY half of D0068; the computed governing-VERSION (charter-time-freeze as-of resolution) is pglProcessRef + pglViews (Inc 2/3), correctly deferred.";
    }
    part pglCharterEdgeReviewGateR1 : TestResult { :>> id = "3df200f2-9372-486c-9d71-9abb1411f71c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6d639fc"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEdgeCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b038-516273849507";
        :>> title = "Sprint 38 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + pglCharterEdge DoD recorded pass; validate_all green. No human gate (D0051: a tested schema edge + reader, self-evidencing; pre-authorized by D0068).";
    }
    part pglCharterEdgeCloseOutGateR1 : TestResult { :>> id = "a4b37473-97fa-4430-9d71-9abb1ce48bfc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6d639fc"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification pglCharterEdgeRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b038-516273849508";
        :>> title = "Sprint 38 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) -- avoidable-issue scan. No avoidable issue. WELL: per the migration/care discipline, investigated the edge-authoring mechanism BEFORE the frozen-schema edit (confirmed #Marker-dependency is the validated idiom) — no fatigue-error. issue002's core (work->process linkage) is resolved by the charter edge. Next: pglProcessRef (governing-version + retroactivity class), pglViews. Estimate 3 pts held.";
    }
    part pglCharterEdgeRetroGateR1 : TestResult { :>> id = "25ceb27a-4dc0-4ec8-9d71-9abb26ee2e8c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6d639fc"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
