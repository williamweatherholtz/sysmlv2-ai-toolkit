// =====================================================================
// Engine::Processes::Agile
// The disciplined workflow for solo + AI development (decision 0010).
// Ceremonies are kept as problem-solving PROCESSES, not team rituals:
// they give AI strong short-term clarity and put a human gate on
// medium/long-term direction.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Processes {
    private import Engine::Core::*;

    // -----------------------------------------------------------------
    // Default workflow states (modular — a project may redefine).
    // -----------------------------------------------------------------
    part backlog     : WorkflowState { :>> name = "backlog";     :>> order = 1; :>> isTerminal = false; }
    part ready       : WorkflowState { :>> name = "ready";       :>> order = 2; :>> isTerminal = false; }
    part inProgress  : WorkflowState { :>> name = "in_progress"; :>> order = 3; :>> isTerminal = false; }
    part inReview    : WorkflowState { :>> name = "in_review";   :>> order = 4; :>> isTerminal = false; }
    part blocked     : WorkflowState { :>> name = "blocked";     :>> order = 5; :>> isTerminal = false; }
    part done        : WorkflowState { :>> name = "done";        :>> order = 6; :>> isTerminal = true;  }

    // -----------------------------------------------------------------
    // Sprint — EVENT-DRIVEN (decision 0010). Not calendar. A sprint is
    // a coherent batch sized by a points budget (initial target 30-50,
    // to be tuned experimentally) that makes the short-term path clear
    // while producing demonstrable work.
    // -----------------------------------------------------------------
    part def Sprint {
        // @Tracked
        attribute name : String;
        attribute goal : String;
        attribute pointsBudgetTarget : Integer;  // initial 30-50; tune from velocity
        attribute items : String[*];             // qualified names of Stories
    }

    // ---- Refinement (continuous) ------------------------------------
    part refinement : Process {
        :>> name = "Backlog Refinement";
        :>> purpose = "Move items backlog -> ready: acceptance criteria "
                    + "(as atomic Tests), sizing, dependency links.";
        :>> cadence = "continuous";
    }
    part refineCriteria : ProcessStep {
        :>> name = "Acceptance criteria as Tests";
        :>> order = 1;
        :>> trigger = "Candidate Story selected.";
        :>> action = "For each criterion, create an atomic Test and link "
                  + "it by `verify`. NO acceptance-criteria text blob.";
        :>> output = "Tests created and verify-linked.";
        :>> owner = "either";
    }
    part refineSize : ProcessStep {
        :>> name = "Size";
        :>> order = 2;
        :>> trigger = "Criteria present as Tests.";
        :>> action = "Assign estimatedPoints (Fibonacci 1,2,3,5,8).";
        :>> output = "Sized Story.";
        :>> owner = "human";
    }
    part refineLink : ProcessStep {
        :>> name = "Link & promote";
        :>> order = 3;
        :>> trigger = "Sized.";
        :>> action = "Add dependency/satisfy edges; set currentState = ready.";
        :>> output = "Graph-connected, sprint-eligible Story.";
        :>> owner = "either";
    }

    // ---- Planning (per-sprint, event-driven) ------------------------
    part planning : Process {
        :>> name = "Sprint Planning";
        :>> purpose = "Triage 'ready' Stories into a Sprint under the "
                    + "points budget, against a one-sentence goal.";
        :>> cadence = "event-driven";
    }
    part planGoal : ProcessStep {
        :>> name = "Set goal";
        :>> order = 1;
        :>> trigger = "Previous sprint closed OR new batch initiated by human.";
        :>> action = "Write Sprint.goal.";
        :>> output = "Sprint with goal.";
        :>> owner = "human";
    }
    part planPull : ProcessStep {
        :>> name = "Pull & triage";
        :>> order = 2;
        :>> trigger = "Goal set.";
        :>> action = "Add 'ready' Stories until points ~ budget target. "
                  + "Triage priority and antagonistically question scope.";
        :>> output = "Populated Sprint.";
        :>> owner = "either";
    }

    // ---- Standup (event-driven, per ~5 points) ----------------------
    // Canonical name 'Standup'. NOT daily/calendar — it fires before
    // each ~5-point batch of upcoming work. This is the Definition of
    // Ready gate in action (see definition-of-ready.sysml).
    part standup : Process {
        :>> name = "Standup";
        :>> purpose = "Before each ~5-point batch, antagonistically "
                    + "critique each upcoming item against the DoR gate. "
                    + "If gaps: resolve them, or invoke grill-me.";
        :>> cadence = "event-driven";
    }
    part standupGate : ProcessStep {
        :>> name = "Run DoR gate on the batch";
        :>> order = 1;
        :>> trigger = "About to start ~5 points of work.";
        :>> action = "For each item run the Definition of Ready gate. "
                  + "Pass -> go. Fail -> resolve or invoke grill-me skill.";
        :>> output = "Batch cleared to start, or gaps logged/resolved.";
        :>> owner = "either";
    }

    // ---- Implementation (per-story) ---------------------------------
    part implementation : Process {
        :>> name = "Implementation";
        :>> purpose = "Move a Story in_progress -> in_review -> done.";
        :>> cadence = "per-story";
    }
    part implDo : ProcessStep {
        :>> name = "Build";
        :>> order = 1;
        :>> trigger = "Story in_progress and past Standup.";
        :>> action = "Write code/content + the Tests that verify it.";
        :>> output = "Working changes; TestResults appended.";
        :>> owner = "either";
    }
    part implModel : ProcessStep {
        :>> name = "Update the model";
        :>> order = 2;
        :>> trigger = "Implementation complete locally.";
        :>> action = "Append TestResults (with commit SHA). The suspicion "
                  + "engine recomputes coverage/staleness automatically.";
        :>> output = "Authored facts current; views recompute.";
        :>> owner = "ai";
    }
    part implReview : ProcessStep {
        :>> name = "Review";
        :>> order = 3;
        :>> trigger = "Changes ready.";
        :>> action = "Open PR (CI comments computed-state deltas). "
                  + "Set currentState = in_review.";
        :>> output = "PR open.";
        :>> owner = "either";
    }
    part implClose : ProcessStep {
        :>> name = "Close out";
        :>> order = 4;
        :>> trigger = "PR merged.";
        :>> action = "Run Definition of Done gate. If it passes, set "
                  + "currentState = done.";
        :>> output = "Story done iff satisfaction == verified.";
        :>> owner = "ai";
    }

    // ---- Retrospective (per-sprint) ---------------------------------
    // Valuable for solo+AI: turns workflow learnings into Decisions /
    // process changes (e.g. "bash failed on Windows -> use PowerShell").
    part retrospective : Process {
        :>> name = "Retrospective";
        :>> purpose = "Promote workflow learnings into Decisions and "
                    + "process/convention changes.";
        :>> cadence = "per-sprint";
    }
    part retroPromote : ProcessStep {
        :>> name = "Promote learnings";
        :>> order = 1;
        :>> trigger = "End of sprint; a learning changes future behavior.";
        :>> action = "Create a Decision; update Process/schema if needed.";
        :>> output = "Tracked Decision; improved workflow.";
        :>> owner = "human";
    }

    // ---- Staleness sweep (continuous) -------------------------------
    part stalenessSweep : Process {
        :>> name = "Staleness Sweep";
        :>> purpose = "On material upstream change, recompute suspicion "
                    + "and surface newly-suspect items for review.";
        :>> cadence = "continuous";
    }
    part sweepDetect : ProcessStep {
        :>> name = "Recompute & flag";
        :>> order = 1;
        :>> trigger = "Commit materially changes a Requirement, Decision, "
                  + "Component, Assumption, or any STPA element.";
        :>> action = "Recompute suspicionState; comment newly-suspect "
                  + "items on the PR.";
        :>> output = "Suspect set surfaced.";
        :>> owner = "ai";
    }
    part sweepResolve : ProcessStep {
        :>> name = "Resolve suspicion";
        :>> order = 2;
        :>> trigger = "Item is suspect.";
        :>> action = "Re-run/re-judge its Tests (auto or human by "
                  + "method), or open a follow-up Story.";
        :>> output = "Coverage restored or follow-up created.";
        :>> owner = "either";
    }

}
}
