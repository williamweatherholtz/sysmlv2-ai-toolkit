// =====================================================================
// ProjectDeliveryS29 — Sprint 29: process-rigor hardening, part 1 — governance (3 pts).
// Story: bootstrap is over (the tracking engine exists), so retire the §3e
//        sprint-exemption and the "until the runtime exists" framings; mandate
//        sprints for substantive work; tighten §3 to mandatory triage-first.
//        Decision D0064 + CLAUDE.md surgery + doc-sync. Resolves bootstrapExitEval.
//        (The ENFORCEMENT mechanisms — no-sprint guard + triage skill + hook — are
//        sprint30; issue020.)
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049). Gate results appended via append-gate-result.
// =====================================================================
package ProjectDeliveryS29 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part processRigorHardeningStory : Story {
        :>> id = "63940516-2c7d-4e8f-a029-516273849501";
        :>> title = "Sprint 29: process-rigor hardening pt1 — retire §3e bootstrap exemption + mandate sprints + triage-first (D0064); resolves bootstrapExitEval";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS29 {
        action storyProcessRigorHardening;
        verification storyProcessRigorHardeningDoD : Test { :>> id = "63940516-2c7d-4e8f-a029-516273849502"; :>> method = VerificationMethod::inspect; :>> procedureText = "bootstrapExitEvalDoD (pass): D0064 recorded (bootstrap over -> retire §3e sprint-exemption + 'until runtime exists' framings; mandate sprints for substantive work; durable §4 rules kept; meta-process freeze out of scope); CLAUDE.md surgery (header status, §3 mandatory triage-first + flag-non-mapping, §3e retired, §4 framings) done + doc-synced; enforcement (no-sprint guard + triage skill + hook) queued as sprint30/issue020; validate_instances green."; }
        part storyProcessRigorHardeningDoDR1 : TestResult { :>> id = "cc95fa41-019b-4818-a166-944e218c0814"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "34fa14d"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification processRigorHardeningRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-a029-516273849503";
        :>> title = "Sprint 29 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = retire the §3e bootstrap sprint-exemption + CLAUDE.md hardening (delivers bootstrapExitEval). Trigger: issue020 (work committed with no sprint) + the §3e-vs-practice contradiction; bootstrap is over (write API + validators + computed orient exist). Split decided: governance here (3 pts); enforcement (no-sprint guard + triage skill + UserPromptSubmit hook) -> sprint30 (8-pt cap). No blocking dependency. DoD = D0064 + CLAUDE.md + doc-sync + validate green. estimatedPoints=3. DoD accepted.";
    }
    part processRigorHardeningRefineGateR1 : TestResult { :>> id = "060c35fd-14de-4a83-9952-365986d6f679"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2131802"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processRigorHardeningStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-a029-516273849504";
        :>> title = "Sprint 29 standup gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: (1) D0064 Decision (retire §3e exemption + 'until runtime exists' framings; mandate sprints for substantive work; keep durable §4 rules: validate-green, CR: commits, doc-sync, D0047, main-only; meta-process freeze out of scope); (2) CLAUDE.md edits — header BOOTSTRAP status, §3 mandatory triage-first + flag-non-mapping, §3e block, §4 framings; (3) doc-sync. No blockers.";
    }
    part processRigorHardeningStandupGateR1 : TestResult { :>> id = "e39303f7-9fd7-4a6f-9952-36598aebe661"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2131802"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processRigorHardeningImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-a029-516273849505";
        :>> title = "Sprint 29 implement gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "D0064 recorded; CLAUDE.md surgery applied (header status; §3 mandatory triage-first + flag-non-mapping; §3e retired; §4 'until runtime exists' framings updated); doc-sync; bootstrapExitEvalDoD recorded pass; validate_instances green. Enforcement queued (sprint30 tasks added).";
    }
    part processRigorHardeningImplementGateR1 : TestResult { :>> id = "e65654c3-de12-424f-acd3-ddde8fa5291d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "34fa14d"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processRigorHardeningReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-a029-516273849506";
        :>> title = "Sprint 29 review gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: D0064 uses alternatives/criteria convention; CLAUDE.md changes retire exemption without dropping durable rules; no broken internal refs. Phase-3 scan: enforcement deferral is explicit (sprint30/issue020), not dropped.";
    }
    part processRigorHardeningReviewGateR1 : TestResult { :>> id = "28d4cd78-a315-41ab-acd3-ddde92ca561d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "34fa14d"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processRigorHardeningCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-a029-516273849507";
        :>> title = "Sprint 29 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + bootstrapExitEval DoD recorded pass; validate_all green. No human gate (D0051: a recorded Decision + doc edits; the direction was accepted inline this sitting).";
    }
    part processRigorHardeningCloseOutGateR1 : TestResult { :>> id = "d42f2b8c-2a3a-4618-a166-944e1adb80a8"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "34fa14d"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification processRigorHardeningRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-a029-516273849508";
        :>> title = "Sprint 29 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) — avoidable-issue scan. No NEW avoidable issue arose this sprint (the prior one, issue020, is what this sprint addresses). Process improvement delivered: D0064 removes the §3e-vs-practice contradiction that was issue020's root cause. ENFORCEMENT still owed (D0047): the no-sprint guard + per-interaction triage skill + UserPromptSubmit hook are queued as sprint30 (noSprintGuard, triageRouterSkill); until the guard exists the no-sprint rule binds by the now-mandatory triage discipline. WELL: the governance/enforcement split kept this sprint clean. Estimate 3 pts held.";
    }
    part processRigorHardeningRetroGateR1 : TestResult { :>> id = "8945fd2a-0bed-4c58-a166-944e1e68a868"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "34fa14d"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
