// =====================================================================
// ProjectDeliveryS103 — Sprint 103: introductionProcess (D0093 milestone, FIRST-VALUE / your priority).
// The guided, project-based onboarding: .engine/processes/introduction.sysml (a 5-step Process —
// orient -> first Need -> refine -> first sprint -> show value) + the deploying `introduction` AISkill
// (skills-registry). Learn by DOING the first real item, not a doc dump; dual-surface (AI drives CLI,
// human supervises). Co-records the new process-def with D0093 (D0070 keystone).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS103 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from introductionProcessStory to d0093;

    part introductionProcessStory : Story {
        :>> id = "63940516-2c7d-4e8f-c103-51627384c001";
        :>> title = "Sprint 103: introductionProcess (3 pts) — guided project-based onboarding process + deploying skill (D0093 first-value milestone)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::modeling;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS103 {
        action storyIntroductionProcess;
        verification storyIntroductionProcessDoD : Test { :>> id = "63940516-2c7d-4e8f-c103-51627384c002"; :>> method = VerificationMethod::inspect; :>> procedureText = "D0093 first-value milestone (human-prioritized): author `.engine/processes/introduction.sysml` — a Process (cadence adHoc) with 5 ordered ProcessSteps (inOrient = 1-min mental model; inFirstNeed = capture the project's first business Need; inRefine = derive a first SystemRequirement(satisfy) + work item to DoR; inFirstSprint = run the first agile-workflow sprint to a recorded passing DoD; inShowValue = show orient/--html/report and hand off to the steady-state loop). Project-based (learn by doing the first real item), NOT a doc dump; dual-surface (AI drives CLI, human supervises). Add the deploying `introduction` AISkill (.engine/skills/introduction/SKILL.md) + register it (skills-registry, purpose naming the process path per the process-skill guard convention). doc-sync: usage.md onboarding pointer; D0093 rollout note. VERIFY: kernel validate_instances PASS (introduction.sysml + skills-registry + d0093); process-skill guard PASS (13 scanned — introduction covered); rust validate + guard all (13) green; introductionProcess marked done."; }
        part storyIntroductionProcessDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c103-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90b29b8"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification introductionProcessRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c103-51627384c004"; :>> title = "Sprint 103 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0093 accepted; the human prioritized onboarding ('introduction first'). Milestone = the introduction process + deploying skill. Resolver: introductionProcess. estimatedPoints=3. #CharteredBy d0093. A new process-def -> co-stage d0093 (rollout note) for the D0070 keystone. DoD accepted."; }
    part introductionProcessRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c103-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90b29b8"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification introductionProcessStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c103-51627384c006"; :>> title = "Sprint 103 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: mirror an existing process-def (render.sysml) shape — Process + ordered ProcessSteps (id/title/order/triggerCondition/actionText/producedArtifact/owner); a concise SKILL.md (frontmatter + vocabulary + anti-patterns + the 5 steps + trigger questions); register the skill in skills-registry with the process path; usage.md onboarding pointer; co-stage d0093 (keystone). No schema change. No blockers."; }
    part introductionProcessStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c103-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90b29b8"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification introductionProcessImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c103-51627384c008"; :>> title = "Sprint 103 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Implemented introduction.sysml (5 steps) + introduction SKILL.md + skills-registry entry + usage.md pointer + d0093 rollout note. VERIFY (live): process-skill guard PASS (13 scanned, introduction covered); rust validate + guard all (13) PASS; kernel validate_instances PASS (introduction.sysml + skills-registry + d0093). method=inspect (a process/doc artifact, self-evidencing — no code/test)."; }
    part introductionProcessImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c103-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90b29b8"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification introductionProcessReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c103-51627384c00a"; :>> title = "Sprint 103 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The process encodes the research-grounded onboarding shape (project-based, time-to-first-value, no doc dump) + the engine's invariants (dual-surface D0093, write-API not hand-edit, engine-vs-instance, decisions-as-reference). It is a real Process+skill (D0059 — not inert: the process-skill guard confirms coverage), the deploying convention is honored, and it composes with the existing ceremony (agile-workflow) + views (orient/report/render). The first-value milestone is delivered; it gains its cold-start partner when engineInit lands. Clean."; }
    part introductionProcessReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c103-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90b29b8"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification introductionProcessCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c103-51627384c00c"; :>> title = "Sprint 103 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyIntroductionProcess DoD recorded pass; introduction process + skill live + registered; process-skill guard 13/0; rust validate + guard all (13) + kernel green. Self-evidencing (D0051). The D0093 first-value milestone is done; engineInit (cold start) + orientDashboard (human home) remain."; }
    part introductionProcessCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c103-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90b29b8"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification introductionProcessRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c103-51627384c00e"; :>> title = "Sprint 103 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the onboarding process was authored as a real, guard-covered Process+skill (not prose), so the engine's 'no inert process' (D0059) + process-skill guard immediately confirmed coverage; mirroring render.sysml's shape made it fast + consistent; the steps bake in the research (project-based, TTFV) AND the engine's hardest-won invariants (dual-surface, write-API, decisions-as-reference) so onboarding TEACHES the discipline by enacting it. Co-staging d0093's rollout note handled the D0070 keystone cleanly (the established pattern for a process-def edit after its decision commit). AVOIDABLE-ISSUE SCAN: no avoidable defect this sprint — the one observation (the introduction process is currently UNexercised: it describes onboarding but has not been run on a real fresh project, since no init exists yet) is EXPECTED, not an avoidable miss; its true test is the first real spin-up. Forward (not an avoidable-issue item, a sequencing note): validate introductionProcess end-to-end during the engineInit milestone (run init -> introduction on a throwaway dir). Estimate 3 pts held. First-value milestone delivered; the spin-up arc now needs engineInit (cold start) + orientDashboard (human home)."; }
    part introductionProcessRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c103-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "90b29b8"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
