// =====================================================================
// ProjectDeliveryS24 — Sprint 24: formalViewpointsSchema (3 pts), increment 1 of
// formalViewpoints (D0056/D0057). Frozen-schema change WITH §2.5 sign-off (D0057).
// Story: part def Viewpoint + a declared registry of 8 viewpoints + validator wiring.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049); schema sign-off recorded in D0057.
// =====================================================================
package ProjectDeliveryS24 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part formalViewpointsSchemaStory : Story {
        :>> id = "61728394-0a5b-4c6d-bd50-405162738420";
        :>> title = "Sprint 24: formalViewpointsSchema (3 pts) — Viewpoint type + declared viewpoint registry (D0057)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS24 {
        action storyFormalViewpointsSchema;
        verification storyFormalViewpointsSchemaDoD : Test { :>> id = "61728394-0a5b-4c6d-ce51-405162738421"; :>> method = VerificationMethod::test; :>> procedureText = "formalViewpointsSchemaDoD in backlog (pass): part def Viewpoint added (schema sign-off D0057); .engine/views/viewpoint-registry.sysml declares 8 viewpoints; validate_schema + validate_instances green."; }
        part storyFormalViewpointsSchemaDoDR1 : TestResult { :>> id = "61728394-0a5b-4c6d-fe52-405162738422"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1224053"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    verification formalViewpointsSchemaRefineGate : Test {
        :>> id = "61728394-0a5b-4c6d-1153-405162738423";
        :>> title = "Sprint 24 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach (D0057, schema sign-off given): model Viewpoint as part def :> Element (engine-registry style, like AISkill — carries queryable concern/audience/sources/renderer; native viewpoint def can't and its expose/render don't parse). Declare 8 viewpoints in .engine/views/. Wire validate_instances + pre-commit + _ID_TYPES. Re-freeze schema after. Lightweight (no Capella/Kitalpha). estimatedPoints=3. DoD accepted.";
    }
    part formalViewpointsSchemaRefineGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-2253-405162738424"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1224053"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsSchemaStandupGate : Test {
        :>> id = "61728394-0a5b-4c6d-3353-405162738425";
        :>> title = "Sprint 24 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. formalViewpointsSchema: Viewpoint def + registry + validator wiring. No blockers.";
    }
    part formalViewpointsSchemaStandupGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-4453-405162738426"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1224053"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsSchemaImplementGate : Test {
        :>> id = "61728394-0a5b-4c6d-5553-405162738427";
        :>> title = "Sprint 24 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "part def Viewpoint :> Element {concern/audience/sources/renderer} in computed.sysml; .engine/views/viewpoint-registry.sysml declares orient/whats-next/traceability/safety/requirements/decisions/baselines/suspect; validate_instances INSTANCES += views glob + _ID_TYPES += Viewpoint; pre-commit covers .engine/views/; validate_schema 14/14 + validate_instances green.";
    }
    part formalViewpointsSchemaImplementGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-6653-405162738428"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1224053"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsSchemaReviewGate : Test {
        :>> id = "61728394-0a5b-4c6d-7753-405162738429";
        :>> title = "Sprint 24 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: Viewpoint is registry-style (consistent with AISkill); 8 viewpoints declared incl. the requested safety/requirements/whats-next/traceability; schema re-frozen. Phase-3 scan: no avoidable issues; increment 2 (renderer rewiring + concern-coverage) correctly deferred to formalViewpoints.";
    }
    part formalViewpointsSchemaReviewGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-8853-405162738430"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1224053"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsSchemaCloseOutGate : Test {
        :>> id = "61728394-0a5b-4c6d-9953-405162738431";
        :>> title = "Sprint 24 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + backlog DoD recorded; validate_all green. Schema sign-off was the human gate (D0057); ceremony gates AI.";
    }
    part formalViewpointsSchemaCloseOutGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-aa53-405162738432"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1224053"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification formalViewpointsSchemaRetroGate : Test {
        :>> id = "61728394-0a5b-4c6d-bb53-405162738433";
        :>> title = "Sprint 24 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049): no avoidable issues. Note: chose part def Viewpoint over native viewpoint def deliberately (queryable + parseable); increment 2 (formalViewpoints) will make renderers declaration-driven. Estimate 3 pts held.";
    }
    part formalViewpointsSchemaRetroGateR1 : TestResult { :>> id = "61728394-0a5b-4c6d-cc53-405162738434"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1224053"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
}
