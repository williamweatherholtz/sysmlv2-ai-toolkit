// =====================================================================
// ProjectDeliveryS13 — Sprint 13: authoredByRefs — actor validation.
// Story: authoredByRefs — claudeSonnet46 added to actors.sysml;
//   validate_actors.py validates authoredBy/judgedBy against ProjectActors.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS13 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 13 Story
    // ====================================================================
    part authoredByRefsStory : Story {
        :>> id = "fc6b2dc0-8e9f-4a0b-bcd1-2f3a4b5c6d7e";
        :>> title = "Sprint 13: authoredByRefs — actor validation; claudeSonnet46 in ProjectActors; validate_actors.py green";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 1;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunS13 {
        action storyAuthoredByRefs;
        verification storyAuthoredByRefsDoD : Test { :>> id = "ad7c3ed1-9f0a-4b1c-cde2-3a4b5c6d7e8f"; :>> method = VerificationMethod::test; :>> procedureText = "claudeSonnet46 in actors.sysml; validate_actors.py exits 0 (PASS); 0 unknown-actor errors; D0037 recorded; authoredByRefsDoDR1 in backlog."; }
        part storyAuthoredByRefsDoDR1 : TestResult { :>> id = "be8d4fe2-0a1b-4c2d-def3-4b5c6d7e8f9a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "35bf281"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification authoredByRefsRefineGate : Test {
        :>> id = "cf9e5gf3-1b2c-4d3e-efa4-5c6d7e8f9a0b";
        :>> title = "Sprint 13 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: validated-name (not typed ref — cross-file ref scoping unreliable in pilot 0.59.0); claudeSonnet46 added to actors.sysml; validate_actors.py reads actors.sysml + scans tracking; LEGACY_ACTORS set for pre-convention names; DoD accepted.";
    }
    part authoredByRefsRefineGateR1 : TestResult { :>> id = "da0f6ag4-2c3d-4e4f-fab5-6d7e8f9a0b1c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "60c565c"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification authoredByRefsStandupGate : Test {
        :>> id = "eb1a7bh5-3d4e-4f5a-abc6-7e8f9a0b1c2d";
        :>> title = "Sprint 13 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "claudeSonnet46 added to actors.sysml; validate_actors.py written; runs PASS (0 errors, 52 legacy WARNs); D0037 drafted; no blockers.";
    }
    part authoredByRefsStandupGateR1 : TestResult { :>> id = "fc2b8ci6-4e5f-4a6b-bcd7-8f9a0b1c2d3e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "60c565c"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification authoredByRefsImplementGate : Test {
        :>> id = "ad3c9dj7-5f6a-4b7c-cde8-9a0b1c2d3e4f";
        :>> title = "Sprint 13 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "claudeSonnet46 in actors.sysml; validate_actors.py exits 0; 0 unknown-actor errors; D0037 recorded; validate_instances green.";
    }
    part authoredByRefsImplementGateR1 : TestResult { :>> id = "cf4c9ek7-5f6a-4b7c-cde8-9a0b1c2d3e4f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "35bf281"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification authoredByRefsReviewGate : Test {
        :>> id = "be4daek8-6a7b-4c8d-def9-0b1c2d3e4f5a";
        :>> title = "Sprint 13 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: actors.sysml clean (claudeSonnet46 added); validate_actors.py clean (no subprocess, pure-Python regex, legacy set documented); D0037 consistent with D0035 (type approach); CLAUDE.md §5 updated if needed.";
    }
    part authoredByRefsReviewGateR1 : TestResult { :>> id = "da5d0fl8-6a7b-4c8d-def9-0b1c2d3e4f5a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "35bf281"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeSonnet46"; }

    verification authoredByRefsCloseOutGate : Test {
        :>> id = "cf5ebl9-7b8c-4d9e-efa0-1c2d3e4f5a6b";
        :>> title = "Sprint 13 closeOut gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyAuthoredByRefsDoDR1 recorded (pass); authoredByRefsDoDR1 in backlog (pass); validate_actors.py PASS; user accepts Sprint 13 complete.";
    }

    verification authoredByRefsRetroGate : Test {
        :>> id = "da6fcm0-8c9d-4e0f-fab1-2d3e4f5a6b7c";
        :>> title = "Sprint 13 retro gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 13 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
}
