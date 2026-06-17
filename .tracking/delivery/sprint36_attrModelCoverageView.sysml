// =====================================================================
// ProjectDeliveryS36 — Sprint 36: attrModelCoverageView (2 pts).
// Story: a process-required-attestation coverage view — query.py attestation-coverage
//        lists status=accepted decisions missing their acceptance event (D0066);
//        declared as the attestation-coverage Viewpoint (D0056/D0057).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049). Gate results via append-gate-result.
// =====================================================================
package ProjectDeliveryS36 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part attrModelCoverageViewStory : Story {
        :>> id = "67940516-2c7d-4e8f-b036-516273849501";
        :>> title = "Sprint 36: attrModelCoverageView (2 pts) — attestation-coverage view + declared viewpoint";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS36 {
        action storyAttrModelCoverageView;
        verification storyAttrModelCoverageViewDoD : Test { :>> id = "67940516-2c7d-4e8f-b036-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "attrModelCoverageViewDoD (pass): query.py attestation-coverage lists status=accepted decisions missing a passing acceptance event; demonstrated 66/66 covered, 0 missing; declared as attestationCoverageVP in the viewpoint-registry; usage.md row; validate_all green."; }
    }

    verification attrModelCoverageViewRefineGate : Test {
        :>> id = "67940516-2c7d-4e8f-b036-516273849503";
        :>> title = "Sprint 36 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = attrModelCoverageView (D0061). A computed VIEW (§3d) over decisions: which status=accepted decisions lack the D0066 acceptance event. Kernel-free query.py command + a declared Viewpoint (D0056/D0057). Pairs with attrModelGuard (the enforcement). No blocking dependency (attrModelAcceptanceEvents done). estimatedPoints=2. DoD accepted.";
    }
    part attrModelCoverageViewRefineGateR1 : TestResult { :>> id = "c1f1530e-260b-4a99-a9b2-4be932a49847"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bc21579"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelCoverageViewStandupGate : Test {
        :>> id = "67940516-2c7d-4e8f-b036-516273849504";
        :>> title = "Sprint 36 standup gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: add compute_attestation_coverage() to query.py (kernel-free, dispatched with issues/viewpoints/orphans) — glob decisions, find status=accepted lacking a passing dNNNNAcceptR1; declare attestationCoverageVP in viewpoint-registry; usage.md row. No blockers.";
    }
    part attrModelCoverageViewStandupGateR1 : TestResult { :>> id = "807bd14a-80a1-4b09-a9b2-4be9355880cb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bc21579"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelCoverageViewImplementGate : Test {
        :>> id = "67940516-2c7d-4e8f-b036-516273849505";
        :>> title = "Sprint 36 implement gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "query.py: compute_attestation_coverage() + 'attestation-coverage' dispatch (kernel-free). Ran: {total_accepted:66, covered:66, missing:[]} — full coverage demonstrated (the acceptance migration is complete). attestationCoverageVP declared in viewpoint-registry; usage.md row + decision-row updated for the acceptance event. validate_instances green. attrModelCoverageView backlog DoD recorded pass.";
    }

    verification attrModelCoverageViewReviewGate : Test {
        :>> id = "67940516-2c7d-4e8f-b036-516273849506";
        :>> title = "Sprint 36 review gate";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: the view is a pure computed read (§3d, no mutation); declared as a Viewpoint (D0056/D0057 — adding a view = declaring a viewpoint); extensible (more attestation rules can be added). Phase-3 scan: this view is exactly what attrModelGuard (next) will enforce at commit — view computes, guard blocks.";
    }

    verification attrModelCoverageViewCloseOutGate : Test {
        :>> id = "67940516-2c7d-4e8f-b036-516273849507";
        :>> title = "Sprint 36 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + attrModelCoverageView DoD recorded pass; validate_all green. No human gate (D0051: a computed view, self-evidencing — it ran and reported 0 missing).";
    }

    verification attrModelCoverageViewRetroGate : Test {
        :>> id = "67940516-2c7d-4e8f-b036-516273849508";
        :>> title = "Sprint 36 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) -- avoidable-issue scan. No avoidable issue. Process improvement: attestation coverage is now a computed, declared view (not vigilance) and confirms the acceptance migration is 100% complete. Only one attestation rule today (decision acceptance); the function is structured to add more (e.g. sprint-gate coverage) without new dispatch. Estimate 2 pts held.";
    }
}
