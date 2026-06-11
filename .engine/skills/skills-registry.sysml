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
    private import EngineSkills::*;

    part engineTriage : AISkill {
        :>> id = "5b3f6f3e-91a4-4f47-9f57-1f2f3a4b5c6d";
        :>> title = "engine-triage";
        :>> purpose = "Classify EVERY substantive request into CHANGE/EXECUTE/RECORD/VIEW/BOOTSTRAP/ORIENT before acting; state the route out loud (CLAUDE.md 3).";
        :>> location = ".engine/skills/engine-triage/SKILL.md";
        :>> triggerCondition = "Any substantive request arrives.";
        :>> writePolicy = "read-only";
    }
    part requirementQuality : AISkill {
        :>> id = "61c1f9c2-08e8-4f55-9b34-1f5fb1f0a111";
        :>> title = "requirement-quality";
        :>> purpose = "Audit requirements against INCOSE GtWR rules + EARS syntax; emit findings + rewrites.";
        :>> location = ".engine/skills/requirement-quality/SKILL.md";
        :>> triggerCondition = "A requirement is authored or changed; DoR gate.";
        :>> writePolicy = "pr-only";
    }
    part stpaSkill : AISkill {
        :>> id = "7d20a9c4-26cd-4f0a-8413-2b6fb2f0b222";
        :>> title = "stpa";
        :>> purpose = "Run STPA (losses/hazards/UCAs/constraints) per the SOP for safety-relevant scope.";
        :>> location = ".engine/skills/stpa/SKILL.md";
        :>> triggerCondition = "Safety-relevant Need/UseCase/Component enters scope.";
        :>> writePolicy = "pr-only";
    }
    part agileRefinement : AISkill {
        :>> id = "8e31bad5-37de-4b1b-9524-3c70c3f0c333";
        :>> title = "agile-refinement";
        :>> purpose = "Run backlog refinement: criteria as verification DoDs, sizing, dependency links (rename to backlog-refinement pending).";
        :>> location = ".engine/skills/agile-refinement/SKILL.md";
        :>> triggerCondition = "New backlog items need refinement to ready.";
        :>> writePolicy = "pr-only";
    }
    part repoPush : AISkill {
        :>> id = "9f42cbe6-48ef-4c2c-a635-4d81d4f0d444";
        :>> title = "repo-push";
        :>> purpose = "Repository discipline: commit message grammar + pre-push checks (adapted: main-only policy, auto-push hook).";
        :>> location = ".engine/skills/repo-push/SKILL.md";
        :>> triggerCondition = "Committing engine/process work.";
        :>> writePolicy = "direct";
    }
}
