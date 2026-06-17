// =====================================================================
// ProjectDeliveryS18 — Sprint 18: auditSkill (3 pts), W2 of the D0046 audit program.
// Story: create+register the architectural-critique skill (CAE+GQM+ATAM); rewrite the
//        critique process to record findings as tracked Issue/Decision items (issue007).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049) — all gates AI-judged; human confirms per sitting.
// =====================================================================
package ProjectDeliveryS18 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part auditSkillStory : Story {
        :>> id = "4a316e29-8adc-4f6e-bd1c-3f5a7b9c2e3d";
        :>> title = "Sprint 18: auditSkill (3 pts) — architectural-critique skill (CAE+GQM+ATAM); critique process records tracked findings";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS18 {
        action storyAuditSkill;
        verification storyAuditSkillDoD : Test { :>> id = "5b427f30-9bed-4f80-de2e-4a6b8c0d3e4e"; :>> method = VerificationMethod::inspect; :>> procedureText = "auditSkillDoD in backlog (pass): architectural-critique SKILL.md created + registered; encodes CAE + GQM metric set + ATAM; critique process rewritten to record findings as tracked Issue/Decision items; validate_all green."; }
        part storyAuditSkillDoDR1 : TestResult { :>> id = "6c538041-acfe-4f91-ce3f-5b7c9d1e4f5e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "216b291"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    // ---- ceremony gates (autonomous, D0049: closeOut=inspect, retro=analyze, all AI) ----
    verification auditSkillRefineGate : Test {
        :>> id = "7d649153-bd0f-4f02-fe40-6c8d0e2f5061";
        :>> title = "Sprint 18 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach agreed: new .engine/skills/architectural-critique/SKILL.md encoding D0046 (CAE Goal->Strategy->Evidence backbone + GQM metric set: gate-coverage, satisfy-coverage, done-verified, deliverable-parity, decisions-recorded, doc-truth + ATAM scenario walkthroughs); findings recorded as tracked Issue/Decision items. Rewrite architectural-critique.sysml criSynthesize/criAccept to tracked-findings + per-sitting disposition (issue007). Register the skill. estimatedPoints=3. Ready (orient: auditSkill unblocked after rustToolchainFix). DoD accepted.";
    }
    part auditSkillRefineGateR1 : TestResult { :>> id = "8e750264-ce10-4f03-0241-7d9e1f306172"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "216b291"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification auditSkillStandupGate : Test {
        :>> id = "9f861375-df21-4014-1352-8e0f20417283";
        :>> title = "Sprint 18 standup gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. auditSkill: SKILL.md + process rewrite + registry entry. No blockers.";
    }
    part auditSkillStandupGateR1 : TestResult { :>> id = "a0972486-e032-4125-2463-9f1031528394"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "216b291"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification auditSkillImplementGate : Test {
        :>> id = "b1083597-f143-4236-3574-0a2142639405";
        :>> title = "Sprint 18 implement gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "architectural-critique/SKILL.md authored (CAE+GQM+ATAM, tracked-findings discipline); skills-registry registers it; architectural-critique.sysml criSynthesize/criAccept rewritten to tracked findings + per-sitting disposition; validate_instances green.";
    }
    part auditSkillImplementGateR1 : TestResult { :>> id = "c2194608-0254-4347-4685-1b3253740516"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "216b291"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification auditSkillReviewGate : Test {
        :>> id = "d32a5719-1365-4458-5796-2c4364850627";
        :>> title = "Sprint 18 review gate";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: skill follows the SKILL.md shape; process steps consistent (criSynthesize→tracked items, criAccept→per-sitting); registry entry correct; doc-sync complete; no regressions. Phase-3 transcript scan: clean sprint, no avoidable issues beyond those already tracked.";
    }
    part auditSkillReviewGateR1 : TestResult { :>> id = "e43b682a-2476-4569-689a-3d5475960738"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "216b291"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification auditSkillCloseOutGate : Test {
        :>> id = "f54c793b-3587-465a-79ab-4e6586a70849";
        :>> title = "Sprint 18 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): storyAuditSkillDoD recorded (pass); auditSkillDoD in backlog (pass); .tracking validates green. No human gate.";
    }
    part auditSkillCloseOutGateR1 : TestResult { :>> id = "065d804c-4698-476b-8abc-5f7697b80950"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "216b291"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification auditSkillRetroGate : Test {
        :>> id = "176e915d-57a9-487c-9bcd-6a87a8c90a61";
        :>> title = "Sprint 18 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049, sprint-retro skill): no new avoidable issues — clean skill+process authoring sprint; existing items (issue007 closed by this work; issue011/012/013 tracked) stand. One-off note: estimate 3 pts held.";
    }
    part auditSkillRetroGateR1 : TestResult { :>> id = "287a026e-68ba-498d-acde-7b98b9da1b72"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "216b291"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
}
