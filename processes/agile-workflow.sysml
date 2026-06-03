package AgileWorkflow {
    import Conventions::*;

    // ---------------------------------------------------------------
    // The lightweight agile process used in this repo.
    //
    // Philosophy: ceremonies exist only where they produce a tracked
    // artifact change. No ceremony for its own sake. AI runs the
    // mechanical bookkeeping; humans run the judgment calls.
    // ---------------------------------------------------------------

    part refinement : Process {
        :>> name = "Backlog Refinement";
        :>> purpose = "Move items from 'backlog' to 'ready' by adding "
                    + "acceptance criteria, sizing, and dependency links.";
        :>> cadence = "continuous";
    }

    part refinementStep1 : ProcessStep {
        :>> name = "Identify candidates";
        :>> order = 1;
        :>> trigger = "Backlog has > 5 items in 'backlog' status, or "
                    + "sprint planning is within 3 days.";
        :>> action = "Select 3-7 highest-priority items.";
        :>> output = "List of candidate Stories.";
        :>> owner = "either";
    }

    part refinementStep2 : ProcessStep {
        :>> name = "Add acceptance criteria";
        :>> order = 2;
        :>> trigger = "Candidate selected.";
        :>> action = "Write testable acceptance criteria. Identify "
                  + "satisfy/verify edges to requirements.";
        :>> output = "Story.acceptanceCriteria populated; satisfy "
                  + "and verify relationships added.";
        :>> owner = "either";
    }

    part refinementStep3 : ProcessStep {
        :>> name = "Size";
        :>> order = 3;
        :>> trigger = "Acceptance criteria present.";
        :>> action = "Assign Story.estimatedPoints (Fibonacci: 1,2,3,5,8).";
        :>> output = "Sized Story.";
        :>> owner = "human";
    }

    part refinementStep4 : ProcessStep {
        :>> name = "Link dependencies";
        :>> order = 4;
        :>> trigger = "Sized.";
        :>> action = "Add dependency edges to upstream items.";
        :>> output = "Story is graph-connected.";
        :>> owner = "either";
    }

    part refinementStep5 : ProcessStep {
        :>> name = "Promote to ready";
        :>> order = 5;
        :>> trigger = "Steps 2-4 complete.";
        :>> action = "Set Story.status = 'ready'.";
        :>> output = "Story is sprint-eligible.";
        :>> owner = "ai";
    }

    // ---------------------------------------------------------------

    part planning : Process {
        :>> name = "Sprint Planning";
        :>> purpose = "Select 'ready' Stories into a Sprint based on "
                    + "capacity and goal.";
        :>> cadence = "per-sprint";
    }

    part planningStep1 : ProcessStep {
        :>> name = "Set sprint goal";
        :>> order = 1;
        :>> trigger = "Start of sprint.";
        :>> action = "Write Sprint.goal — one sentence.";
        :>> output = "Sprint instance with goal.";
        :>> owner = "human";
    }

    part planningStep2 : ProcessStep {
        :>> name = "Pull stories";
        :>> order = 2;
        :>> trigger = "Goal set.";
        :>> action = "Add Stories from 'ready' to Sprint.items until "
                  + "estimated points >= capacity.";
        :>> output = "Sprint populated.";
        :>> owner = "either";
    }

    part planningStep3 : ProcessStep {
        :>> name = "Status promotion";
        :>> order = 3;
        :>> trigger = "Sprint populated.";
        :>> action = "Set each Story.status = 'in_progress' when work "
                  + "actually begins (not at planning time).";
        :>> output = "Status reflects reality.";
        :>> owner = "ai";
    }

    // ---------------------------------------------------------------

    part implementation : Process {
        :>> name = "Implementation";
        :>> purpose = "Move a Story from 'in_progress' to 'done'.";
        :>> cadence = "per-story";
    }

    part implStep1 : ProcessStep {
        :>> name = "Decompose if needed";
        :>> order = 1;
        :>> trigger = "Story.status = 'in_progress'.";
        :>> action = "If Story is > 5 points, add child Tasks.";
        :>> output = "Tasks created and linked via decomposition.";
        :>> owner = "either";
    }

    part implStep2 : ProcessStep {
        :>> name = "Make changes";
        :>> order = 2;
        :>> trigger = "Tasks ready (or Story small enough).";
        :>> action = "Write code, tests, docs.";
        :>> output = "Working tree changes; tests added.";
        :>> owner = "either";
    }

    part implStep3 : ProcessStep {
        :>> name = "Update the model";
        :>> order = 3;
        :>> trigger = "Implementation complete locally.";
        :>> action = "Update TestCase.lastResult; check downstream "
                  + "items for staleness; update affected ExternalRefs.";
        :>> output = "Model reflects new state.";
        :>> owner = "ai";
    }

    part implStep4 : ProcessStep {
        :>> name = "Review";
        :>> order = 4;
        :>> trigger = "Model and code changes ready.";
        :>> action = "Open PR. Set Story.status = 'in_review'.";
        :>> output = "PR open, review requested.";
        :>> owner = "either";
    }

    part implStep5 : ProcessStep {
        :>> name = "Close out";
        :>> order = 5;
        :>> trigger = "PR merged and tests green.";
        :>> action = "Set Story.status = 'done'. Run "
                  + "definition-of-done check.";
        :>> output = "Story done; DoD validated.";
        :>> owner = "ai";
    }

    // ---------------------------------------------------------------

    part retrospective : Process {
        :>> name = "Retrospective";
        :>> purpose = "Capture process learnings as Decisions when "
                    + "they change how we work going forward.";
        :>> cadence = "per-sprint";
    }

    part retroStep1 : ProcessStep {
        :>> name = "Identify learnings";
        :>> order = 1;
        :>> trigger = "End of sprint.";
        :>> action = "Review completed Stories. Note where the "
                  + "process helped or hurt.";
        :>> output = "List of candidate learnings.";
        :>> owner = "human";
    }

    part retroStep2 : ProcessStep {
        :>> name = "Promote learnings to Decisions";
        :>> order = 2;
        :>> trigger = "Learning that changes future behavior.";
        :>> action = "Create a Decision in decisions/. Update "
                  + "Process or Convention if needed.";
        :>> output = "Tracked Decision; updated process.";
        :>> owner = "human";
    }

    // ---------------------------------------------------------------
    // Staleness sweep — runs continuously, not tied to sprint cadence
    // ---------------------------------------------------------------

    part stalenessSweep : Process {
        :>> name = "Staleness Sweep";
        :>> purpose = "When an upstream item changes, surface every "
                    + "downstream item via satisfy / verify / refine "
                    + "/ dependency for human or AI review.";
        :>> cadence = "continuous";
    }

    part sweepStep1 : ProcessStep {
        :>> name = "Detect upstream change";
        :>> order = 1;
        :>> trigger = "Commit touches a Requirement, Decision, or "
                  + "Component.";
        :>> action = "Compute transitive closure of dependent items.";
        :>> output = "List of suspect items.";
        :>> owner = "ai";
    }

    part sweepStep2 : ProcessStep {
        :>> name = "Mark suspect";
        :>> order = 2;
        :>> trigger = "Suspect list non-empty.";
        :>> action = "Add a comment to each suspect item: 'review "
                  + "needed — upstream X changed at commit Y'.";
        :>> output = "Items flagged for review.";
        :>> owner = "ai";
    }

    part sweepStep3 : ProcessStep {
        :>> name = "Resolve suspicion";
        :>> order = 3;
        :>> trigger = "Item is flagged.";
        :>> action = "Inspect; either confirm no change needed (clear "
                  + "the flag) or open a follow-up Story.";
        :>> output = "Flag cleared or follow-up created.";
        :>> owner = "either";
    }
}
