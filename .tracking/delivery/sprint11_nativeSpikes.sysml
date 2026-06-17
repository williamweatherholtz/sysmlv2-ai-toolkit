// =====================================================================
// ProjectDeliveryS11 — Sprint 11: native SysML v2 feature spikes.
// Stories: stateDefSpike (verdict: NOT green — body syntax not in pilot 0.59.0)
//          occurrenceRetype (verdict: GREEN — TestResult retyped to occurrence def)
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryS11 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 11 Story
    // ====================================================================
    part nativeSpikesStory : Story {
        :>> id = "ab1c2d3e-4f5a-4b6c-de7f-8a9b0c1d2e3f";
        :>> title = "Sprint 11: native SysML v2 spikes — state def (verdict: deferred) + occurrence retype (verdict: green, TestResult retyped)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 1;
    }

    // ====================================================================
    // Delivery backlog — DoD read by orient.
    // ====================================================================
    action def DeliveryRunS11 {
        action storyNativeSpikes;
        verification storyNativeSpikesDoD : Test { :>> id = "da057a8b-9c0d-4e1f-abc2-3d4e5f6a7b8c"; :>> method = VerificationMethod::test; :>> procedureText = "stateDefSpike verdict recorded (D0031): pilot 0.59.0 does not support state def body syntax; transitions stay String[*]. occurrenceRetype verdict recorded (D0032): green — TestResult retyped to occurrence def in verification.sysml; existing part instances still parse; validate_schema 13/13 green."; }
        part storyNativeSpikesDoDR1 : TestResult { :>> id = "ea168b9c-0d1e-4f2a-bcd3-4e5f6a7b8c9e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "146bc25"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification nativeSpikesRefineGate : Test {
        :>> id = "de4f5a6b-7c8d-4e9f-ab0c-1d2e3f4a5b6c";
        :>> title = "Sprint 11 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: _spike_statedef.py tests minimal state def + body syntax; _spike_occurrence.py tests minimal occ def + attributes + part instance usage; if state def green → update WorkflowDefinition.transitions; if occurrence green → retype TestResult; DoD accepted.";
    }

    verification nativeSpikesStandupGate : Test {
        :>> id = "ef5a6b7c-8d9e-4f0a-bc1d-2e3f4a5b6c7d";
        :>> title = "Sprint 11 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Spike scripts written: _spike_statedef.py (3 tests) and _spike_occurrence.py (5 tests). stateDefSpike result: minimal parses; transitions body fails in pilot 0.59.0. occurrenceRetype result: all tests pass (Test 2 false negative due to reserved 'actor' name; Test 5 part instance of occurrence def passes). No blocking issues.";
    }

    verification nativeSpikesImplementGate : Test {
        :>> id = "f06b7c8d-9e0f-4a1b-cd2e-3f4a5b6c7d8e";
        :>> title = "Sprint 11 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "occurrence def TestResult :> Element applied in verification.sysml; D0031 + D0032 recorded; validate_schema 13/13 green; validate_instances 37/37 green; validate_tracking 18/18 green.";
    }

    verification nativeSpikesReviewGate : Test {
        :>> id = "a17c8d9e-0f1a-4b2c-de3f-4a5b6c7d8e9f";
        :>> title = "Sprint 11 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: verification.sysml TestResult retype clean; D0031 + D0032 recorded; sysmlv2-syntax-notes.md updated (state def body gap + occurrence def confirmed); doc-sync D0033 recorded.";
    }

    verification nativeSpikesCloseOutGate : Test {
        :>> id = "b28d9e0f-1a2b-4c3d-ef4a-5b6c7d8e9f0a";
        :>> title = "Sprint 11 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyNativeSpikesDoDR1 recorded (pass); stateDefSpikeDoDR1 + occurrenceRetypeDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 11 complete.";
    }
    part nativeSpikesCloseOutGateR1 : TestResult { :>> id = "8f9a0b1c-d2e3-4f4a-edfa-7c8d9e0f1a2b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c431705"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }

    verification nativeSpikesRetroGate : Test {
        :>> id = "c39e0f1a-2b3c-4d4e-fa5b-6c7d8e9f0a1b";
        :>> title = "Sprint 11 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 11 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part nativeSpikesRetroGateR1 : TestResult { :>> id = "9a0b1c2d-e3f4-4a5b-feab-8d9e0f1a2b3c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c431705"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "wweatherholtz"; }
}
