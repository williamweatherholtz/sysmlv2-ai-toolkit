// =====================================================================
// ProjectDeliveryS32 — Sprint 32: attrModelTooling (2 pts).
// Story: MIGRATE-readers step (D0063) — point the readers at the new attribution
//        fields (createdBy / decision) with fallback to the old ones. Rust needs
//        no change (generic parser); .engine/tools not in the deliverable manifest,
//        so no re-verify.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049). Gate results via append-gate-result.
// =====================================================================
package ProjectDeliveryS32 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part attrModelToolingStory : Story {
        :>> id = "63940516-2c7d-4e8f-b032-516273849501";
        :>> title = "Sprint 32: attrModelTooling (2 pts) — readers prefer createdBy/decision (D0063 migrate-readers)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS32 {
        action storyAttrModelTooling;
        verification storyAttrModelToolingDoD : Test { :>> id = "63940516-2c7d-4e8f-b032-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "attrModelToolingDoD (pass): validate_actors scans createdBy; query.py read_issues prefers createdBy (fallback authoredBy); report.py prefers decision (fallback decisionText); Rust unchanged (generic); no deliverable drift; readers smoke-tested; validate_all green."; }
        part storyAttrModelToolingDoDR1 : TestResult { :>> id = "bf28a758-8a34-4f19-9896-b72d602934eb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdf3d85"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification attrModelToolingRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b032-516273849503";
        :>> title = "Sprint 32 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = D0063 migrate-readers (attrModelTooling). Scoping grep found only 3 readers touch the old fields (validate_actors regex, query.py read_issues, report.py); Rust has zero authoredBy/decisionText refs (generic parser → no change); .engine/tools not in deliverable-manifest → no re-verify. Additive (fallback to old fields) so green throughout. No blocking dependency (attrModelInstances done). estimatedPoints=2. DoD accepted.";
    }
    part attrModelToolingRefineGateR1 : TestResult { :>> id = "a0164a24-7670-4bd5-aad5-9cb38220d583"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5e0bb1f"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelToolingStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b032-516273849504";
        :>> title = "Sprint 32 standup gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: validate_actors regex += createdBy; query.py read_issues value = createdBy or authoredBy; report.py decision-then-decisionText (the `decision\\s*=` regex won't match `decisionText` since the next char is 'T'). Smoke-test each. No blockers.";
    }
    part attrModelToolingStandupGateR1 : TestResult { :>> id = "5450e18d-6c42-4e09-aad5-9cb384429987"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5e0bb1f"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelToolingImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b032-516273849505";
        :>> title = "Sprint 32 implement gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "validate_actors.py regex now matches authoredBy|createdBy|judgedBy; query.py:450 value = createdBy or authoredBy; report.py:92 prefers `decision` (fallback decisionText). Smoke: validate_actors PASS (0 unknown-actor); query.py issues renders; report.py writes status.html (64 decisions). Rust unchanged (generic). attrModelTooling backlog DoD recorded pass.";
    }
    part attrModelToolingImplementGateR1 : TestResult { :>> id = "238825b0-c963-42ad-9896-b72d500df5bf"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdf3d85"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelToolingReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b032-516273849506";
        :>> title = "Sprint 32 review gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: all reader changes are additive (new field preferred, old field fallback) so nothing breaks while both fields coexist; the `decision\\s*=` regex is prefix-safe vs decisionText. Phase-3 scan: no avoidable issue; the readers are now ready for attrModelContract to drop the old fields.";
    }
    part attrModelToolingReviewGateR1 : TestResult { :>> id = "055202b1-a180-438d-9896-b72d544b892b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdf3d85"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelToolingCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b032-516273849507";
        :>> title = "Sprint 32 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + attrModelTooling DoD recorded pass; validate_all green. No human gate (D0051: tested reader changes, self-evidencing).";
    }
    part attrModelToolingCloseOutGateR1 : TestResult { :>> id = "029aab58-f865-4ef1-9896-b72d58d964ef"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdf3d85"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification attrModelToolingRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b032-516273849508";
        :>> title = "Sprint 32 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) -- avoidable-issue scan. No avoidable issue. Process improvement: the migrate-readers step was far smaller than the DoD implied (Rust is field-name-agnostic; only 3 Python readers touched the fields) -- the DoD was re-scoped to reality rather than inventing Rust work. With readers on the new fields, attrModelContract (drop authoredBy/decisionText) is unblocked. Estimate 2 pts held.";
    }
    part attrModelToolingRetroGateR1 : TestResult { :>> id = "7d6fb992-60bb-4bb9-9896-b72d5c050f43"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fdf3d85"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
