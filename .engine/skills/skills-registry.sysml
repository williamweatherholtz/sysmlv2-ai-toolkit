// =====================================================================
// EngineSkillRegistry — the AI skills that ACTUALLY EXIST on disk.
//
// CR-10 (2026-06-11): the four phantom registrations (triage, staleness-sweep,
// implementer, grill-me — none existed) are gone; the five real skills are
// registered (critique P1). writePolicy is the INTENDED write boundary —
// enforcement arrives with the write API (D0003); until then it binds by
// discipline (CLAUDE.md §4).
// =====================================================================
package EngineSkillRegistry {
    private import EngineElement::*;     // WritePolicy enum
    private import EngineSkills::*;

    part engineTriage : AISkill {
        :>> id = "5b3f6f3e-91a4-4f47-9f57-1f2f3a4b5c6d";
        :>> title = "engine-triage";
        :>> purpose = "Classify EVERY substantive request into CHANGE/EXECUTE/RECORD/VIEW/BOOTSTRAP/ORIENT before acting; state the route out loud (CLAUDE.md 3).";
        :>> location = ".engine/skills/engine-triage/SKILL.md";
        :>> triggerCondition = "Any substantive request arrives.";
        :>> writePolicy = WritePolicy::readOnly;
    }
    part requirementQuality : AISkill {
        :>> id = "61c1f9c2-08e8-4f55-9b34-1f5fb1f0a111";
        :>> title = "requirement-quality";
        :>> purpose = "Audit requirements against INCOSE GtWR rules + EARS syntax; emit findings + rewrites.";
        :>> location = ".engine/skills/requirement-quality/SKILL.md";
        :>> triggerCondition = "A requirement is authored or changed; DoR gate.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part stpaSkill : AISkill {
        :>> id = "7d20a9c4-26cd-4f0a-8413-2b6fb2f0b222";
        :>> title = "stpa";
        :>> purpose = "Run STPA (losses/hazards/UCAs/constraints) per the SOP for safety-relevant scope.";
        :>> location = ".engine/skills/stpa/SKILL.md";
        :>> triggerCondition = "Safety-relevant Need/UseCase/Component enters scope.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part backlogRefinement : AISkill {
        :>> id = "8e31bad5-37de-4b1b-9524-3c70c3f0c333";
        :>> title = "backlog-refinement";
        :>> purpose = "Run backlog refinement: INVEST check, acceptance criteria as atomic Test items linked by verify, DoR gate, story-slicing.";
        :>> location = ".engine/skills/backlog-refinement/SKILL.md";
        :>> triggerCondition = "New backlog items need refinement to ready.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part testDesign : AISkill {
        :>> id = "a042dce7-59f0-4d2c-b635-5e92e5f0e555";
        :>> title = "test-design";
        :>> purpose = "Design verification Tests for engine items: choose method (test/demo/inspect/analysis/confirmation), write BDD/Gherkin scenarios, map acceptance criteria to atomic Test instances.";
        :>> location = ".engine/skills/test-design/SKILL.md";
        :>> triggerCondition = "Authoring DoD criteria, acceptance tests, or BDD scenarios for engine items.";
        :>> writePolicy = WritePolicy::prOnly;
    }
    part testResult : AISkill {
        :>> id = "b153edf8-6a01-4e3d-c746-6fa3f6f0f666";
        :>> title = "test-result";
        :>> purpose = "Record test execution outcomes as appended TestResult instances: outcome + judgedAgainst commit + judgedAt + judgedBy. Never mutate prior results.";
        :>> location = ".engine/skills/test-result/SKILL.md";
        :>> triggerCondition = "Executing tests, recording gate verdicts, or closing a DoD.";
        :>> writePolicy = WritePolicy::direct;
    }
    part traceabilityAudit : AISkill {
        :>> id = "c264fe09-7b12-4f4e-d857-7b047b01c777";
        :>> title = "traceability-audit";
        :>> purpose = "Audit satisfy/verify/allocate coverage: surface uncovered Needs, unverified Requirements, unallocated Components; trace-need paths; flag suspect items.";
        :>> location = ".engine/skills/traceability-audit/SKILL.md";
        :>> triggerCondition = "Architecture/Delivery review gates, DoD coverage checks, or asked to audit traceability.";
        :>> writePolicy = WritePolicy::readOnly;
    }
    part repoPush : AISkill {
        :>> id = "9f42cbe6-48ef-4c2c-a635-4d81d4f0d444";
        :>> title = "repo-push";
        :>> purpose = "Repository discipline: commit message grammar + pre-push checks (adapted: main-only policy, auto-push hook).";
        :>> location = ".engine/skills/repo-push/SKILL.md";
        :>> triggerCondition = "Committing engine/process work.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintPlanning : AISkill {
        :>> id = "d375af1a-8c23-4f5d-e968-8c15c2f1e888";
        :>> title = "sprint-planning";
        :>> purpose = "Orient on ready frontier, select scope, apply Fibonacci sizing to set estimatedPoints, pass DoR gate before sprint work begins.";
        :>> location = ".engine/skills/sprint-planning/SKILL.md";
        :>> triggerCondition = "Sprint refine gate; asked to plan next sprint, size a story, or select scope.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintStandup : AISkill {
        :>> id = "e486bg2b-9d34-4a6e-fa79-9d26d3g2f999";
        :>> title = "sprint-standup";
        :>> purpose = "Orient on current status, surface blockers, confirm active phase alignment, state next action — lightweight 2-minute ceremony.";
        :>> location = ".engine/skills/sprint-standup/SKILL.md";
        :>> triggerCondition = "Sprint standup gate; start of work session; asked for status or blockers.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintReview : AISkill {
        :>> id = "f597ch3c-0e45-4b7f-ab80-0e37e4h3g000";
        :>> title = "sprint-review";
        :>> purpose = "Verify DoD, record actualHours, compute velocity/efficiency/accuracy metrics, and run transcript review to produce improvement items.";
        :>> location = ".engine/skills/sprint-review/SKILL.md";
        :>> triggerCondition = "Sprint review gate; asked for velocity, efficiency, estimate accuracy, or process issues.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintCloseout : AISkill {
        :>> id = "a608di4d-1f56-4c8a-bc91-1f48f5i4h111";
        :>> title = "sprint-closeout";
        :>> purpose = "Verify DoD complete, confirm actualHours set, obtain explicit human sign-off, record confirmation gate TestResult, commit.";
        :>> location = ".engine/skills/sprint-closeout/SKILL.md";
        :>> triggerCondition = "Sprint closeOut gate; asked to close or accept a sprint.";
        :>> writePolicy = WritePolicy::direct;
    }
    part sprintRetro : AISkill {
        :>> id = "b719ej5e-2a67-4d9b-cd02-2a59a6j5i222";
        :>> title = "sprint-retro";
        :>> purpose = "Triage improvement items from sprint-review, dispatch accepted items (skill-update/decision/backlog), record retro gate confirmation, close sprint.";
        :>> location = ".engine/skills/sprint-retro/SKILL.md";
        :>> triggerCondition = "Sprint retro gate; asked for retrospective or process improvement dispatch.";
        :>> writePolicy = WritePolicy::direct;
    }
}
