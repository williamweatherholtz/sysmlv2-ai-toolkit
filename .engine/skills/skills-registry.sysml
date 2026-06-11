// =====================================================================
// Engine::Skills::Registry
// Default AI skill registrations shipped with the engine. A project may
// add its own. writePolicy is enforced by the API and cannot be
// self-promoted by a skill (decision 0003).
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package EngineSkillRegistry {
    private import EngineSkills::*;

    part triage : AISkill {
        :>> id = "f104da2c-9348-4558-bff2-1daf9a7d7f1f";
        :>> title = "triage";
        :>> purpose = "Run backlog refinement steps on new items up to "
                    + "the human sizing gate: acceptance criteria as "
                    + "Tests, dependency links.";
        :>> location = ".engine/skills/triage/SKILL.md";
        :>> triggerCondition = "WorkItem created in 'backlog'.";
        :>> writePolicy = "pr-only";
    }

    part stalenessSweepSkill : Agent {
        :>> id = "6ddd1d79-67eb-42ea-9ac2-c68f18d69017";
        :>> title = "staleness-sweep";
        :>> purpose = "Recompute suspicion on material upstream change "
                    + "and comment newly-suspect items on the PR. "
                    + "Mechanical bookkeeping only.";
        :>> location = ".engine/skills/staleness-sweep/SKILL.md";
        :>> triggerCondition = "Commit hook / scheduled.";
        :>> writePolicy = "direct";
        :>> subagents = ();
    }

    part implementer : Agent {
        :>> id = "0ff1c1c9-f980-4dc4-aecc-f623e78592b3";
        :>> title = "implementer";
        :>> purpose = "Take a 'ready' Story past Standup, build it + its "
                    + "Tests, append results, open a PR.";
        :>> location = ".engine/skills/implementer/SKILL.md";
        :>> triggerCondition = "Dispatched by human or planner.";
        :>> writePolicy = "pr-only";
        :>> subagents = ();
    }

    part grillMe : AISkill {
        :>> id = "ee6147f0-9514-465f-b054-194b4e78a734";
        :>> title = "grill-me";
        :>> purpose = "Antagonistically interrogate an item/plan to drive "
                    + "DoR gaps to resolution before work starts.";
        :>> location = "~/.claude/skills/grill-me/SKILL.md";
        :>> triggerCondition = "A Standup/DoR check fails and needs resolution.";
        :>> writePolicy = "read-only";
    }

}
