// =====================================================================
// Engine::Processes::Agile
// The disciplined workflow for solo + AI development (decision 0010).
// Ceremonies are kept as problem-solving PROCESSES, not team rituals:
// they give AI strong short-term clarity and put a human gate on
// medium/long-term direction.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package EngineAgileProcess {
    private import EngineProcess::*;
    private import EngineState::*;
    private import ScalarValues::*;

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
        :>> title = "Backlog Refinement";
        :>> purpose = "Move items backlog -> ready: acceptance criteria "
                    + "(as atomic Tests), sizing, dependency links.";
        :>> cadence = "continuous";
    }
    part refineCriteria : ProcessStep {
        :>> title = "Acceptance criteria as Tests";
        :>> order = 1;
        :>> triggerCondition = "Candidate Story selected.";
        :>> actionText = "For each criterion, create an atomic Test and link "
                  + "it by `verify`. NO acceptance-criteria text blob.";
        :>> producedArtifact = "Tests created and verify-linked.";
        :>> owner = "either";
    }
    part refineSize : ProcessStep {
        :>> title = "Size";
        :>> order = 2;
        :>> triggerCondition = "Criteria present as Tests.";
        :>> actionText = "Assign estimatedPoints (Fibonacci 1,2,3,5,8).";
        :>> producedArtifact = "Sized Story.";
        :>> owner = "human";
    }
    part refineLink : ProcessStep {
        :>> title = "Link & promote";
        :>> order = 3;
        :>> triggerCondition = "Sized.";
        :>> actionText = "Add dependency/satisfy edges; set currentState = ready.";
        :>> producedArtifact = "Graph-connected, sprint-eligible Story.";
        :>> owner = "either";
    }

    // ---- Planning (per-sprint, event-driven) ------------------------
    part planning : Process {
        :>> title = "Sprint Planning";
        :>> purpose = "Triage 'ready' Stories into a Sprint under the "
                    + "points budget, against a one-sentence goal.";
        :>> cadence = "event-driven";
    }
    part planGoal : ProcessStep {
        :>> title = "Set goal";
        :>> order = 1;
        :>> triggerCondition = "Previous sprint closed OR new batch initiated by human.";
        :>> actionText = "Write Sprint.goal.";
        :>> producedArtifact = "Sprint with goal.";
        :>> owner = "human";
    }
    part planPull : ProcessStep {
        :>> title = "Pull & triage";
        :>> order = 2;
        :>> triggerCondition = "Goal set.";
        :>> actionText = "Add 'ready' Stories until points ~ budget target. "
                  + "Triage priority and antagonistically question scope.";
        :>> producedArtifact = "Populated Sprint.";
        :>> owner = "either";
    }

    // ---- Standup (event-driven, per ~5 points) ----------------------
    // Canonical name 'Standup'. NOT daily/calendar — it fires before
    // each ~5-point batch of upcoming work. This is the Definition of
    // Ready gate in action (see definition-of-ready.sysml).
    part standup : Process {
        :>> title = "Standup";
        :>> purpose = "Before each ~5-point batch, antagonistically "
                    + "critique each upcoming item against the DoR gate. "
                    + "If gaps: resolve them, or invoke grill-me.";
        :>> cadence = "event-driven";
    }
    part standupGate : ProcessStep {
        :>> title = "Run DoR gate on the batch";
        :>> order = 1;
        :>> triggerCondition = "About to start ~5 points of work.";
        :>> actionText = "For each item run the Definition of Ready gate. "
                  + "Pass -> go. Fail -> resolve or invoke grill-me skill.";
        :>> producedArtifact = "Batch cleared to start, or gaps logged/resolved.";
        :>> owner = "either";
    }

    // ---- Implementation (per-story) ---------------------------------
    part implementation : Process {
        :>> title = "Implementation";
        :>> purpose = "Move a Story in_progress -> in_review -> done.";
        :>> cadence = "per-story";
    }
    part implDo : ProcessStep {
        :>> title = "Build";
        :>> order = 1;
        :>> triggerCondition = "Story in_progress and past Standup.";
        :>> actionText = "Write code/content + the Tests that verify it.";
        :>> producedArtifact = "Working changes; TestResults appended.";
        :>> owner = "either";
    }
    part implModel : ProcessStep {
        :>> title = "Update the model";
        :>> order = 2;
        :>> triggerCondition = "Implementation complete locally.";
        :>> actionText = "Append TestResults (with commit SHA). The suspicion "
                  + "engine recomputes coverage/staleness automatically.";
        :>> producedArtifact = "Authored facts current; views recompute.";
        :>> owner = "ai";
    }
    part implReview : ProcessStep {
        :>> title = "Review";
        :>> order = 3;
        :>> triggerCondition = "Changes ready.";
        :>> actionText = "Open PR (CI comments computed-state deltas). "
                  + "Set currentState = in_review.";
        :>> producedArtifact = "PR open.";
        :>> owner = "either";
    }
    part implClose : ProcessStep {
        :>> title = "Close out";
        :>> order = 4;
        :>> triggerCondition = "PR merged.";
        :>> actionText = "Run Definition of Done gate. If it passes, set "
                  + "currentState = done.";
        :>> producedArtifact = "Story done iff satisfaction == verified.";
        :>> owner = "ai";
    }

    // ---- Retrospective (per-sprint) ---------------------------------
    // Valuable for solo+AI: turns workflow learnings into Decisions /
    // process changes (e.g. "bash failed on Windows -> use PowerShell").
    part retrospective : Process {
        :>> title = "Retrospective";
        :>> purpose = "Promote workflow learnings into Decisions and "
                    + "process/convention changes.";
        :>> cadence = "per-sprint";
    }
    part retroPromote : ProcessStep {
        :>> title = "Promote learnings";
        :>> order = 1;
        :>> triggerCondition = "End of sprint; a learning changes future behavior.";
        :>> actionText = "Create a Decision; update Process/schema if needed.";
        :>> producedArtifact = "Tracked Decision; improved workflow.";
        :>> owner = "human";
    }

    // ---- Staleness sweep (continuous) -------------------------------
    part stalenessSweep : Process {
        :>> title = "Staleness Sweep";
        :>> purpose = "On material upstream change, recompute suspicion "
                    + "and surface newly-suspect items for review.";
        :>> cadence = "continuous";
    }
    part sweepDetect : ProcessStep {
        :>> title = "Recompute & flag";
        :>> order = 1;
        :>> triggerCondition = "Commit materially changes a Requirement, Decision, "
                  + "Component, Assumption, or any STPA element.";
        :>> actionText = "Recompute suspicionState; comment newly-suspect "
                  + "items on the PR.";
        :>> producedArtifact = "Suspect set surfaced.";
        :>> owner = "ai";
    }
    part sweepResolve : ProcessStep {
        :>> title = "Resolve suspicion";
        :>> order = 2;
        :>> triggerCondition = "Item is suspect.";
        :>> actionText = "Re-run/re-judge its Tests (auto or human by "
                  + "method), or open a follow-up Story.";
        :>> producedArtifact = "Coverage restored or follow-up created.";
        :>> owner = "either";
    }

}
