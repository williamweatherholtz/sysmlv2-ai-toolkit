// =====================================================================
// ProjectDeliveryS31 — Sprint 31: triageRouterSkill (3 pts).
// Story: upgrade engine-triage to the per-interaction router (break down -> route
//        each part -> flag non-mapping; BOOTSTRAP route removed, D0064) AND add a
//        UserPromptSubmit hook that fires it every turn (the harness-side 5b).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049). Gate results via append-gate-result.
// =====================================================================
package ProjectDeliveryS31 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part triageRouterSkillStory : Story {
        :>> id = "63940516-2c7d-4e8f-b031-516273849501";
        :>> title = "Sprint 31: triageRouterSkill (3 pts) — engine-triage per-interaction router + UserPromptSubmit hook (D0064)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS31 {
        action storyTriageRouterSkill;
        verification storyTriageRouterSkillDoD : Test { :>> id = "63940516-2c7d-4e8f-b031-516273849502"; :>> method = VerificationMethod::inspect; :>> procedureText = "triageRouterSkillDoD (pass): engine-triage SKILL.md upgraded (break down -> route EACH part -> flag non-mapping; BOOTSTRAP route removed per D0064; v0.2.0) + registry purpose updated; UserPromptSubmit hook added (.claude/settings.json -> python .engine/tools/triage_reminder.py, ASCII-only); settings.json valid; validate_instances green."; }
        part storyTriageRouterSkillDoDR1 : TestResult { :>> id = "329d3a76-92c5-4785-9783-c611ec0b6587"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "773126f"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
    }

    verification triageRouterSkillRefineGate : Test {
        :>> id = "63940516-2c7d-4e8f-b031-516273849503";
        :>> title = "Sprint 31 refine gate (DoR)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "DoR: scope = the per-interaction triage router (D0064 headline). (5a) engine-triage skill = break down, route each part, flag non-mapping, drop the retired BOOTSTRAP route; (5b) a UserPromptSubmit hook fires it every turn. Harness boundary flagged: the every-turn firing is harness config (.claude/settings.json), outside the SysML engine model. No blocking dependency. estimatedPoints=3. DoD accepted.";
    }
    part triageRouterSkillRefineGateR1 : TestResult { :>> id = "03b4f6a7-0abb-4056-b9bf-ae89b37e10e2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "659d0a3"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification triageRouterSkillStandupGate : Test {
        :>> id = "63940516-2c7d-4e8f-b031-516273849504";
        :>> title = "Sprint 31 standup gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Refine passed. Approach: rewrite engine-triage SKILL.md (route-each + flag + no BOOTSTRAP); update skills-registry purpose; add .engine/tools/triage_reminder.py (ASCII, prints route-first reminder to stdout); wire UserPromptSubmit in .claude/settings.json (tracked, so it travels with the repo). No blockers.";
    }
    part triageRouterSkillStandupGateR1 : TestResult { :>> id = "eb31902c-2611-429e-b9bf-ae89b6af26b6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "659d0a3"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification triageRouterSkillImplementGate : Test {
        :>> id = "63940516-2c7d-4e8f-b031-516273849505";
        :>> title = "Sprint 31 implement gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "engine-triage SKILL.md upgraded to v0.2.0 (break-down + route-each + flag-non-mapping; BOOTSTRAP route removed, D0064); skills-registry purpose updated; .engine/tools/triage_reminder.py added (ASCII-only, runs clean); UserPromptSubmit hook wired in .claude/settings.json (valid JSON, tracked). triageRouterSkill backlog DoD recorded pass. NOTE: the hook's actual per-turn firing is observable next prompt (harness behavior) — verified-on-use.";
    }
    part triageRouterSkillImplementGateR1 : TestResult { :>> id = "a0926c56-c359-4961-9783-c611dc9e8827"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "773126f"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification triageRouterSkillReviewGate : Test {
        :>> id = "63940516-2c7d-4e8f-b031-516273849506";
        :>> title = "Sprint 31 review gate";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: skill drops the retired BOOTSTRAP route and adds route-each + flag-non-mapping (matches D0064 + CLAUDE.md §3); hook reminder is ASCII (no mojibake in injected context); settings.json valid + tracked. Phase-3 scan: the only unverifiable-until-next-turn piece is whether the harness fires the hook — flagged honestly in the implement gate, not asserted as done.";
    }
    part triageRouterSkillReviewGateR1 : TestResult { :>> id = "a6fffdf6-e6e7-4f1d-9783-c611e082b797"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "773126f"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification triageRouterSkillCloseOutGate : Test {
        :>> id = "63940516-2c7d-4e8f-b031-516273849507";
        :>> title = "Sprint 31 closeOut gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Autonomous closeOut (D0049): story + triageRouterSkill DoD recorded pass; validate_all green; settings.json valid. No human gate (D0051: skill/tool/config edits, self-evidencing; hook firing is verified-on-use next turn).";
    }
    part triageRouterSkillCloseOutGateR1 : TestResult { :>> id = "164d8c3e-5046-44fd-9783-c611e4c346af"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "773126f"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }

    verification triageRouterSkillRetroGate : Test {
        :>> id = "63940516-2c7d-4e8f-b031-516273849508";
        :>> title = "Sprint 31 retro gate (autonomous, D0049)";
        :>> createdAt = "2026-06-17";
        :>> authoredBy = "claudeOpus"; :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "Autonomous retro (D0049) -- avoidable-issue scan. No new avoidable issue this sprint. Process improvement delivered: triage is now structural (fired every turn by the hook), not vigilance -- closing the D0064 mandate the §3 doc alone could not enforce. Avoidable-risk noted: the hook firing can't be self-verified at author time (harness behavior) -- it is verified-on-use on the next prompt; if it does not fire, fix the settings.json command/path. Estimate 3 pts held.";
    }
    part triageRouterSkillRetroGateR1 : TestResult { :>> id = "e65654c3-de12-45ed-9783-c611e8874007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "773126f"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "claudeOpus"; }
}
