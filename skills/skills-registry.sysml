package Skills {
    import Conventions::*;

    // ---------------------------------------------------------------
    // Registered AI skills and agents.
    // Each entry is a workflow-side pointer; the actual prompt /
    // logic lives at the file path in `location`.
    // writePolicy gates what the skill is allowed to do via the API
    // once the API exists. Until then, writePolicy is documentation.
    // ---------------------------------------------------------------

    part triageSkill : AISkill {
        :>> name = "triage";
        :>> purpose = "Move newly created WorkItems through refinement "
                    + "steps 1-4 (identify, acceptance criteria, size, "
                    + "link dependencies) up to the human sizing gate.";
        :>> location = "skills/triage/SKILL.md";
        :>> trigger = "WorkItem created with status='backlog'.";
        :>> writePolicy = "pr-only";
    }

    part stalenessAgent : Agent {
        :>> name = "staleness-sweep";
        :>> purpose = "Run the Staleness Sweep process on every commit "
                    + "touching Requirement, Decision, or Component.";
        :>> location = "skills/staleness-sweep/SKILL.md";
        :>> trigger = "Git commit hook OR scheduled.";
        :>> writePolicy = "direct";
        :>> subagents = ();
    }

    part implementationAgent : Agent {
        :>> name = "implementer";
        :>> purpose = "Pick up a Story in 'ready' status, decompose if "
                    + "needed, write code + tests, update model, open PR.";
        :>> location = "skills/implementer/SKILL.md";
        :>> trigger = "Human or planner agent dispatches.";
        :>> writePolicy = "pr-only";
        :>> subagents = ();
    }

    part docSyncSkill : AISkill {
        :>> name = "doc-sync";
        :>> purpose = "Keep docs/ in sync with model changes that "
                    + "affect user-facing behavior.";
        :>> location = "skills/doc-sync/SKILL.md";
        :>> trigger = "PR ready to merge with user-facing changes.";
        :>> writePolicy = "pr-only";
    }
}
