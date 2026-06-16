// =====================================================================
// ProjectDeliveryS12 — Sprint 12: three architecture decisions.
// Stories: crVersioning (D0034) + needLayer (D0035) + safetyCaseViews (D0036)
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS12 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 12 Story
    // ====================================================================
    part archDecisionsStory : Story {
        :>> id = "eb5a1baf-6c7d-4e8f-abc9-0d1e2f3a4b5c";
        :>> title = "Sprint 12: architecture decisions — crVersioning (D0034) + needLayer (D0035) + safetyCaseViews (D0036)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::decision;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 1;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunS12 {
        action storyArchDecisions;
        verification storyArchDecisionsDoD : Test { :>> id = "fc6b2cb0-7d8e-4f9a-bcd0-1e2f3a4b5c6d"; :>> method = VerificationMethod::confirmation; :>> procedureText = "D0034 (crVersioning), D0035 (needLayer), D0036 (safetyCaseViews) recorded and accepted; validate_instances green; crVersioningDoDR1 + needLayerDoDR1 + safetyCaseViewsDoDR1 recorded in backlog; user accepts Sprint 12 complete."; }
        part storyArchDecisionsDoDR1 : TestResult { :>> id = "be9e5fe3-0a1b-4c2d-efa4-5c6d7e8f9a0b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "30a9476"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification archDecisionsRefineGate : Test {
        :>> id = "ad7c3dc1-8e9f-4a0b-cde1-2f3a4b5c6d7e";
        :>> title = "Sprint 12 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: crVersioning → git history is the version record (no workflowVersion field); needLayer → type IS the discriminator (no additional field); safetyCaseViews → Release + Decision-as-signoff + filtered query (no new schema types); DoD accepted.";
    }
    part archDecisionsRefineGateR1 : TestResult { :>> id = "be8d4ed2-9f0a-4b1c-def2-3a4b5c6d7e8f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "780c9c9"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification archDecisionsStandupGate : Test {
        :>> id = "cf9e5fe3-0a1b-4c2d-efa3-4b5c6d7e8f9a";
        :>> title = "Sprint 12 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "D0034 + D0035 + D0036 drafted; validate_instances check planned; no blockers.";
    }
    part archDecisionsStandupGateR1 : TestResult { :>> id = "da0f6af4-1b2c-4d3e-fab4-5c6d7e8f9a0b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "780c9c9"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification archDecisionsImplementGate : Test {
        :>> id = "eb1a7ba5-2c3d-4e4f-abc5-6d7e8f9a0b1c";
        :>> title = "Sprint 12 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "D0034 + D0035 + D0036 recorded in .engine/decisions/; validate_instances green (43/43); no schema changes.";
    }
    part archDecisionsImplementGateR1 : TestResult { :>> id = "cf0f6af4-1b2c-4d3e-fab5-6d7e8f9a0b1c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "30a9476"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification archDecisionsReviewGate : Test {
        :>> id = "fc2b8cb6-3d4e-4f5a-bcd6-7e8f9a0b1c2d";
        :>> title = "Sprint 12 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Decisions reviewed: D0034 consistent with D0013 (git temporal model); D0035 consistent with EngineNeeds/EngineRequirements schema; D0036 consistent with D0015 (materialized views) and EngineWork::Release; no doc-sync needed (no schema changes).";
    }
    part archDecisionsReviewGateR1 : TestResult { :>> id = "da1a7ba5-2c3d-4e4f-abc6-7e8f9a0b1c2d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "30a9476"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification archDecisionsCloseOutGate : Test {
        :>> id = "ad3c9dc7-4e5f-4a6b-cde7-8f9a0b1c2d3e";
        :>> title = "Sprint 12 closeOut gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyArchDecisionsDoDR1 recorded (pass); crVersioningDoDR1 + needLayerDoDR1 + safetyCaseViewsDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 12 complete.";
    }
    part archDecisionsCloseOutGateR1 : TestResult { :>> id = "eb5ebf9-6a7b-4c8d-efa9-0b1c2d3e4f5a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d61c32e"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }

    verification archDecisionsRetroGate : Test {
        :>> id = "be4dae8-5f6a-4b7c-def8-9a0b1c2d3e4f";
        :>> title = "Sprint 12 retro gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 12 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part archDecisionsRetroGateR1 : TestResult { :>> id = "fc6fcn1-9d0e-4f1a-abc2-3e4f5a6b7c8d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d61c32e"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }
}
