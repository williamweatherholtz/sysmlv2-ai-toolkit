// =====================================================================
// ProjectDeliveryS40 — Sprint 40: pglProcessMarkerCarrier (D0070).
// Story: revise sprint39's process-change carrier per D0070 — record a process change by a
//        #ProspectiveChange/#SafetyChange PREFIX MARKER on the Decision part (in-package,
//        kernel-valid) instead of a stored cross-package edge in a floating .tracking file.
//        Which-process + when are git-derived under the hard-lock guard (Inc 3 resolves).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049). Keystone hard-lock guard = sprint41.
// =====================================================================
package ProjectDeliveryS40 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // dogfood: chartered by D0070 (which superseded D0069's stored-edge carrier).
    #CharteredBy dependency from pglProcessMarkerCarrierStory to d0070;

    part pglProcessMarkerCarrierStory : Story {
        :>> id = "63940516-2c7d-4e8f-b040-516273849501";
        :>> title = "Sprint 40: pglProcessMarkerCarrier (3 pts) — process-change marker on the Decision + git-derived linkage; D0070 (supersedes D0069 edge)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS40 {
        action storyPglProcessMarkerCarrier;
        verification storyPglProcessMarkerCarrierDoD : Test { :>> id = "63940516-2c7d-4e8f-b040-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "pglProcessMarkerCarrierDoD (pass): D0070 recorded with acceptance event; D0069 status -> superseded (typed field) + header note; #ProspectiveChange prefix marker applied to d0039/d0049/d0064/d0067/d0070 (rust validate + validate_instances 79/79 confirm the in-package prefix-marker form in both parsers); .tracking/process-governance.sysml deleted; query.py process-changes retargeted to read the marker (5 decisions, no false matches); relationships.sysml comment + usage cheatsheet + syntax-notes + issue021 updated. validate_schema 14/14 + validate_instances + rust validate + actors/ceremony/coverage/parity green."; }
        part storyPglProcessMarkerCarrierDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b040-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "683b5ef"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
    }

    verification pglProcessMarkerCarrierRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b040-516273849503";
        :>> title = "Sprint 40 refine gate (DoR)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0070 (user-accepted 2026-06-18: 'truth stored in the data, not separate metadata files'; hard lock incl. typos, no intent interpretation). Revises sprint39's carrier: a marker on the Decision part instead of a stored cross-package edge. Marker syntax available (validate_probes); the rust-vs-kernel FORM difference (member `{ @Marker; }` vs prefix `#Marker part`) surfaced + was resolved during implement (see implement gate). Frozen-schema touch (relationships comment only) pre-authorized. Historical decisions grandfathered (no fabricated linkage, D0067). No blocking dependency. estimatedPoints=3. DoD accepted.";
    }
    part pglProcessMarkerCarrierRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b040-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "683b5ef"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessMarkerCarrierStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b040-516273849505";
        :>> title = "Sprint 40 standup gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: (1) record D0070 (supersede D0069's carrier, restate surviving architecture) + acceptance event; flip D0069 status. (2) #ProspectiveChange prefix marker on d0039/d0049/d0064/d0067/d0070 (import EngineRelationships). (3) delete the .tracking edge file. (4) retarget query.py reader (line-anchored regex so prose examples don't false-match). (5) doc-sync relationships/usage/syntax + issue021. No blockers.";
    }
    part pglProcessMarkerCarrierStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b040-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "683b5ef"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessMarkerCarrierImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b040-516273849507";
        :>> title = "Sprint 40 implement gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "D0070 recorded (+ d0070Accept/R1, judgedBy=wweatherholtz). D0069 status->superseded + header note. #ProspectiveChange applied to d0039/d0049/d0064/d0067/d0070. FORM CORRECTION (care discipline): the `{ @Marker; }` MEMBER form was tried first; rust `validate .` rejected it ('unexpected character @') though the kernel accepts it — switched to the PREFIX form `#ProspectiveChange part dNNNN : Decision`, which validate_instances (79/79) AND rust validate (49 clean) both accept. The form issue021 could not do cross-package; this one carries no cross-reference. .tracking/process-governance.sysml deleted. query.py process-changes reads 5 decisions, no false matches (line-anchored regex). relationships.sysml comment + usage (2 rows) + syntax-notes + issue021 updated. pglProcessMarkerCarrier backlog DoD recorded pass.";
    }
    part pglProcessMarkerCarrierImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b040-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "683b5ef"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessMarkerCarrierReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b040-516273849509";
        :>> title = "Sprint 40 review gate";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the marker is a self-contained in-package member (no cross-reference), so issue021 dissolves rather than being worked around — a strictly better design driven by the user's 'truth in the data' principle. Supersession is FORMAL (DecisionStatus::superseded typed field on D0069 + D0070 naming it), not internal metadata; no contradictory accepted pair (D0070 restates the surviving architecture). Historical decisions grandfathered, no fabricated linkage. Phase-3 scan: the keystone hard-lock (sprint41) + git-traversal resolver (sprint42) remain correctly deferred; sprint39's committed history stands (D0013).";
    }
    part pglProcessMarkerCarrierReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b040-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "683b5ef"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessMarkerCarrierCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b040-51627384950b";
        :>> title = "Sprint 40 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + pglProcessMarkerCarrier DoD recorded pass; validate_all green. The one human gate (D0051) — acceptance of D0070 — is already held (the user's explicit 'confirmed' 2026-06-18, recorded as d0070AcceptR1).";
    }
    part pglProcessMarkerCarrierCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b040-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "683b5ef"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }

    verification pglProcessMarkerCarrierRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b040-51627384950d";
        :>> title = "Sprint 40 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. WELL: the sprint39->40 revision was NOT churn — the user's review surfaced that a stored process REFERENCE duplicated git, and removing it dissolved issue021 entirely. The lesson is durable: store only what is NOT git-derivable (process-change linkage = derivable -> compute; decision supersession = not derivable -> store as a typed field). AVOIDABLE? The original D0069 edge design could have caught this at design time by asking 'is this derivable from git?' — added to the design-critique heuristics implicitly via D0070's rationale. SECOND lesson (rust-vs-kernel divergence): the `{ @Marker; }` member form is kernel-valid but rust-rejected; syntax-notes had claimed 'both work'. The rust authority caught it loudly and syntax-notes is corrected (prefix `#Marker` is the portable form) — the loud validator error IS the guard (D0047), no new tracked issue. No new tracked issue (the principles are recorded in D0070 + syntax-notes). Next: pglProcessChangeGuard (keystone hard-lock), then pglViews (resolver). Estimate 3 pts held.";
    }
    part pglProcessMarkerCarrierRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b040-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "683b5ef"; :>> judgedAt = "2026-06-18"; :>> judgedBy = "claudeOpus"; }
}
