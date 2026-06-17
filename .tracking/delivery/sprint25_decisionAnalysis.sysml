// =====================================================================
// ProjectDeliveryS25 — Sprint 25: decisionAnalysis (2 pts).
// Story: Decision Analysis convention (ISO 42010 / NPR 7123.1 / ADR) — capture
//        structured alternatives + criteria in decisionText; doc + self-demo (issue016).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049); convention-only, no schema change.
// =====================================================================
package ProjectDeliveryS25 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part decisionAnalysisStory : Story {
        :>> id = "4f506172-8394-4a51-bd60-40516273840b";
        :>> title = "Sprint 25: decisionAnalysis (2 pts) — alternatives+criteria convention in Decisions (issue016)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS25 {
        action storyDecisionAnalysis;
        verification storyDecisionAnalysisDoD : Test { :>> id = "4f506172-8394-4a51-ce61-40516273840c"; :>> method = VerificationMethod::inspect; :>> procedureText = "decisionAnalysisDoD in backlog (pass): convention documented in sysmlv2-syntax-notes; D0058 self-demonstrates ALTERNATIVES+CRITERIA; validate_instances green."; }
        part storyDecisionAnalysisDoDR1 : TestResult { :>> id = "4f506172-8394-4a51-fe62-40516273840d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2aa59b7"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
    }

    verification decisionAnalysisRefineGate : Test {
        :>> id = "4f506172-8394-4a51-1163-40516273840e";
        :>> title = "Sprint 25 refine gate (DoR)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Approach: lightweight CONVENTION (not schema — keeps core frozen, low friction): decisionText carries an ALTERNATIVES + CRITERIA tail when a real option was rejected. Document in sysmlv2-syntax-notes; self-demonstrate in D0058. estimatedPoints=2. DoD accepted.";
    }
    part decisionAnalysisRefineGateR1 : TestResult { :>> id = "4f506172-8394-4a51-2263-40516273840f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2aa59b7"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification decisionAnalysisStandupGate : Test {
        :>> id = "4f506172-8394-4a51-3363-405162738410";
        :>> title = "Sprint 25 standup gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. decisionAnalysis: convention doc + D0058 self-demo. No blockers.";
    }
    part decisionAnalysisStandupGateR1 : TestResult { :>> id = "4f506172-8394-4a51-4463-405162738411"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2aa59b7"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification decisionAnalysisImplementGate : Test {
        :>> id = "4f506172-8394-4a51-5563-405162738412";
        :>> title = "Sprint 25 implement gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "sysmlv2-syntax-notes Decision section gains the Decision Analysis convention (ALTERNATIVES+CRITERIA in decisionText, no schema field); D0058 recorded WITH a structured alternatives/criteria tail (self-demo); validate_instances green; no schema change (core stays frozen).";
    }
    part decisionAnalysisImplementGateR1 : TestResult { :>> id = "4f506172-8394-4a51-6663-405162738413"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2aa59b7"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification decisionAnalysisReviewGate : Test {
        :>> id = "4f506172-8394-4a51-7763-405162738414";
        :>> title = "Sprint 25 review gate";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: convention is text-only (friction-light, core frozen); D0058 demonstrates it on itself (alternatives A/B/C + criteria). Phase-3 scan: no avoidable issues. Closes issue016.";
    }
    part decisionAnalysisReviewGateR1 : TestResult { :>> id = "4f506172-8394-4a51-8863-405162738415"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2aa59b7"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification decisionAnalysisCloseOutGate : Test {
        :>> id = "4f506172-8394-4a51-9963-405162738416";
        :>> title = "Sprint 25 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + backlog DoD recorded; validate_all green. No human gate (D0051: convention/doc, no schema).";
    }
    part decisionAnalysisCloseOutGateR1 : TestResult { :>> id = "4f506172-8394-4a51-aa63-405162738417"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2aa59b7"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }

    verification decisionAnalysisRetroGate : Test {
        :>> id = "4f506172-8394-4a51-bb63-405162738418";
        :>> title = "Sprint 25 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049): no avoidable issues. Chose convention over schema deliberately (friction/frozen-core). Estimate 2 pts held.";
    }
    part decisionAnalysisRetroGateR1 : TestResult { :>> id = "4f506172-8394-4a51-cc63-405162738419"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2aa59b7"; :>> judgedAt = "2026-06-16"; :>> judgedBy = "claudeOpus"; }
}
