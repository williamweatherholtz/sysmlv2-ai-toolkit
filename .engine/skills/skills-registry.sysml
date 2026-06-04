// =====================================================================
// Engine::Skills::Registry
// Default AI skill registrations shipped with the engine. A project may
// add its own. writePolicy is enforced by the API and cannot be
// self-promoted by a skill (decision 0003).
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Skills {
    private import Engine::Core::*;

    part triage : AISkill {
        :>> name = "triage";
        :>> purpose = "Run backlog refinement steps on new items up to "
                    + "the human sizing gate: acceptance criteria as "
                    + "Tests, dependency links.";
        :>> location = ".engine/skills/triage/SKILL.md";
        :>> trigger = "WorkItem created in 'backlog'.";
        :>> writePolicy = "pr-only";
    }

    part stalenessSweepSkill : Agent {
        :>> name = "staleness-sweep";
        :>> purpose = "Recompute suspicion on material upstream change "
                    + "and comment newly-suspect items on the PR. "
                    + "Mechanical bookkeeping only.";
        :>> location = ".engine/skills/staleness-sweep/SKILL.md";
        :>> trigger = "Commit hook / scheduled.";
        :>> writePolicy = "direct";
        :>> subagents = ();
    }

    part implementer : Agent {
        :>> name = "implementer";
        :>> purpose = "Take a 'ready' Story past Standup, build it + its "
                    + "Tests, append results, open a PR.";
        :>> location = ".engine/skills/implementer/SKILL.md";
        :>> trigger = "Dispatched by human or planner.";
        :>> writePolicy = "pr-only";
        :>> subagents = ();
    }

    part grillMe : AISkill {
        :>> name = "grill-me";
        :>> purpose = "Antagonistically interrogate an item/plan to drive "
                    + "DoR gaps to resolution before work starts.";
        :>> location = "~/.claude/skills/grill-me/SKILL.md";
        :>> trigger = "A Standup/DoR check fails and needs resolution.";
        :>> writePolicy = "read-only";
    }

}
}
