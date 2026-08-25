// =====================================================================
// Engine::Processes::Agile
// The disciplined workflow for solo + AI development (decision 0010).
// Ceremonies are kept as problem-solving PROCESSES, not team rituals:
// they give AI strong short-term clarity and put a human gate on
// medium/long-term direction.
// =====================================================================
package EngineAgileProcess {
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;
    private import EngineGuardConstraints::*;  // D0139(D): the guards this process brings
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
    // Sprint — EVENT-DRIVEN (decision 0010). NOTE (CR-10): Sprint/points apply
    // when Story instances exist; during bootstrap the unit of work is the
    // backlog task (action + DoD criterion + appended TestResults). Not calendar. A sprint is
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
    action refinement : Process {
        :>> id = "27768cee-4609-49db-8229-aa1cd3ae1885";
        :>> title = "Backlog Refinement";
        :>> purpose = "Move items backlog -> ready: acceptance criteria "
                    + "(as atomic Tests), sizing, dependency links.";
        :>> cadence = Cadence::continuous;
        // APPLIES-WHEN: work arrives faster than it is finished, so what to do next has to be computed from a ranked frontier instead of chosen from a pile
    }
    action refineCriteria : ProcessStep {
        :>> id = "c9bef132-83ef-44df-a312-24211035a191";
        :>> title = "Acceptance criteria as Tests";
        :>> triggerCondition = "Candidate Story selected.";
        :>> actionText = "Author the task's one-line `verification <task>DoD : Test` "
                  + "criterion (method + procedureText) in the backlog action def. "
                  + "NO acceptance-criteria text blob.";
        :>> producedArtifact = "Method-tagged DoD criterion authored.";
        :>> owner = Owner::either;
    }
    action refineSize : ProcessStep {
        :>> id = "3fea750a-db19-4555-be64-717955fa110b";
        :>> title = "Size";
        :>> triggerCondition = "Criteria present as Tests.";
        :>> actionText = "Assign rough size (Fibonacci when Story instances exist; "
                  + "optional for backlog tasks during bootstrap).";
        :>> producedArtifact = "Sized item.";
        :>> owner = Owner::human;
    }
    action refineLink : ProcessStep {
        :>> id = "d4510352-ecf7-4b77-ac8a-d980ea60c18d";
        :>> title = "Link & promote";
        :>> triggerCondition = "Sized.";
        :>> actionText = "Add `first A then B` successions (#OrderingOnly when non-"
                  + "semantic). Readiness is the COMPUTED view - never authored.";
        :>> producedArtifact = "Graph-connected, sprint-eligible Story.";
        :>> owner = Owner::either;
        assert constraint enforcesPriorityInversion : priorityInversion;
        assert constraint enforcesVerificationTrace : verificationTrace;
    }

    // ---- Planning (per-sprint, event-driven) ------------------------
    action planning : Process {
        :>> id = "3e69977e-2488-4127-bebf-110d3001425b";
        :>> title = "Sprint Planning";
        :>> purpose = "Triage 'ready' Stories into a Sprint under the "
                    + "points budget, against a one-sentence goal.";
        :>> cadence = Cadence::eventDriven;
    }
    action planGoal : ProcessStep {
        :>> id = "b1dedc21-9235-4489-9b71-5765be94ebfc";
        :>> title = "Set goal";
        :>> triggerCondition = "Previous sprint closed OR new batch initiated by human.";
        :>> actionText = "Write Sprint.goal.";
        :>> producedArtifact = "Sprint with goal.";
        :>> owner = Owner::human;
    }
    action planPull : ProcessStep {
        :>> id = "f3fa7b01-4c55-4541-84dc-400bd6825f44";
        :>> title = "Pull & triage";
        :>> triggerCondition = "Goal set.";
        :>> actionText = "Pull `keel orient` ready tasks into a coherent batch "
                  + "(points budget when Stories exist). Triage priority and "
                  + "antagonistically question scope.";
        :>> producedArtifact = "Populated batch/Sprint.";
        :>> owner = Owner::either;
    }

    // ---- Standup (event-driven, per ~5 points) ----------------------
    // Canonical name 'Standup'. NOT daily/calendar — it fires before
    // each ~5-point batch of upcoming work. This is the Definition of
    // Ready gate in action (see definition-of-ready.sysml).
    action standup : Process {
        :>> id = "4c504acb-763b-4e88-b525-26469f5facb8";
        :>> title = "Standup";
        :>> purpose = "Before each ~5-point batch, antagonistically "
                    + "critique each upcoming item against the DoR gate. "
                    + "If gaps: resolve them, or invoke grill-me.";
        :>> cadence = Cadence::eventDriven;
    }
    action standupGate : ProcessStep {
        :>> id = "723f4b81-690e-4898-9544-118ea5d9a6ea";
        :>> title = "Run DoR gate on the batch";
        :>> triggerCondition = "About to start ~5 points of work.";
        :>> actionText = "For each item run the Definition of Ready gate. "
                  + "Pass -> go. Fail -> resolve or invoke grill-me skill.";
        :>> producedArtifact = "Batch cleared to start, or gaps logged/resolved.";
        :>> owner = Owner::either;
        assert constraint enforcesCeremony : ceremony;
        assert constraint enforcesSprintCoverage : sprintCoverage;
    }

    // ---- Implementation (per-story) ---------------------------------
    action implementation : Process {
        :>> id = "6219917f-aa45-4d82-ab15-253ac7c46e13";
        :>> title = "Implementation";
        :>> purpose = "Move a Story in_progress -> in_review -> done.";
        :>> cadence = Cadence::perStory;
    }
    action implDo : ProcessStep {
        :>> id = "a6fc664a-e64a-4133-bc69-9326bfd658c4";
        :>> title = "Build";
        :>> triggerCondition = "Story in_progress and past Standup.";
        :>> actionText = "Write code/content + the Tests that verify it.";
        :>> producedArtifact = "Working changes; TestResults appended.";
        :>> owner = Owner::either;
    }
    action implModel : ProcessStep {
        :>> id = "dba37bd4-406f-43d3-af02-8e3ef8b0c017";
        :>> title = "Update the model";
        :>> triggerCondition = "Implementation complete locally.";
        :>> actionText = "Append TestResults (with commit SHA). The suspicion "
                  + "engine recomputes coverage/staleness automatically.";
        :>> producedArtifact = "Authored facts current; views recompute.";
        :>> owner = Owner::ai;
    }
    action implReview : ProcessStep {
        :>> id = "a77f1c0a-4761-4d35-9bf2-1476b2bdb713";
        :>> title = "Review";
        :>> triggerCondition = "Changes ready.";
        :>> actionText = "Commit to main (per the main-only policy); review state is "
                  + "computed from the commit, never authored.";
        :>> producedArtifact = "PR open.";
        :>> owner = Owner::either;
    }
    action implClose : ProcessStep {
        :>> id = "893dad0b-5a2a-4548-94e7-11dc07e87647";
        :>> title = "Close out";
        :>> triggerCondition = "PR merged.";
        :>> actionText = "Run Definition of Done gate; append the passing TestResult. "
                  + "Done is the COMPUTED view over results - never authored.";
        :>> producedArtifact = "Story done iff satisfaction == verified.";
        :>> owner = Owner::ai;
    }

    // ---- Retrospective (per-sprint) ---------------------------------
    // Valuable for solo+AI: turns workflow learnings into Decisions /
    // process changes (e.g. "bash failed on Windows -> use PowerShell").
    action retrospective : Process {
        :>> id = "2a3a5a1f-7f8b-4388-948f-49787fe09656";
        :>> title = "Retrospective";
        :>> purpose = "Promote workflow learnings into Decisions and "
                    + "process/convention changes.";
        :>> cadence = Cadence::perSprint;
    }
    action retroIdentify : ProcessStep {
        :>> id = "0d6b0001-0813-4b01-9b3e-0000001b0001";
        :>> title = "Identify avoidable issues";
        :>> triggerCondition = "End of sprint (autonomous, D0049 — no human input).";
        :>> actionText = "Name what went well and, specifically, what was AVOIDABLE — a defect, rework, or friction that a guard, rule, skill, doc or check could have prevented. Distinguish avoidable from merely difficult: only the avoidable class produces an obligation. State each finding plainly, including findings against this thread's own earlier work.";
        :>> producedArtifact = "A recorded retro gate (method=analyze) naming each avoidable issue.";
        :>> owner = Owner::ai;
    }
    action retroTrack : ProcessStep {
        :>> id = "0d6b0002-0813-4b02-9b3e-0000001b0002";
        :>> title = "Every finding becomes a tracked, prioritized item — or says why not";
        :>> triggerCondition = "The retro named at least one avoidable issue.";
        :>> actionText = "For EACH avoidable issue, either (a) record a tracked `Issue` (with severity) plus the resolving backlog action, placed at its correct PRIORITY — declaration order IS priority (D0052), so ranking it means putting it in the right place in the file, never describing it as important; or (b) state explicitly in the retro why no item is needed (a control already exists, or it is a one-off with no recurrable class). What is FORBIDDEN is the third path taken in sprint 247: writing the lesson into prose — a doc, a commit message, or agent memory — where it carries no severity, no priority, no resolver and no id, is invisible to orient and the burndown, and is not project truth at all. A lesson that terminates in prose has not been retained (D0018, issue085). Per D0047 an avoidable issue with a recurrable class ALSO needs a permanent automated control, not a reminder.";
        :>> producedArtifact = "A tracked Issue + prioritized backlog action per avoidable issue, or a recorded justification for none.";
        :>> owner = Owner::ai;
        assert constraint enforcesRetroBacklog : retroBacklog;
    }
    action retroPromote : ProcessStep {
        :>> id = "5d66f22d-22db-4532-936d-8fc00b2b3a8e";
        :>> title = "Promote learnings that change the process";
        :>> triggerCondition = "A learning changes future BEHAVIOR (not just this sprint's outcome).";
        :>> actionText = "Where a finding changes how work is done, record a Decision and update the Process/schema through the Change Request path. Autonomous per D0049 — the retro itself is AI-recorded with no human sign-off; only a resulting CHANGE carries the usual human acceptance gate.";
        :>> producedArtifact = "Tracked Decision; improved workflow.";
        :>> owner = Owner::ai;
    }

    // ---- Staleness sweep (continuous) -------------------------------
    action stalenessSweep : Process {
        :>> id = "69fb93a5-7c68-4bf2-a925-d47c642deb22";
        :>> title = "Staleness Sweep";
        :>> purpose = "On material upstream change, recompute suspicion "
                    + "and surface newly-suspect items for review.";
        :>> cadence = Cadence::continuous;
    }
    action sweepDetect : ProcessStep {
        :>> id = "d0bca5d3-2f81-4986-8b9a-70bfd8f85fcc";
        :>> title = "Recompute & flag";
        :>> triggerCondition = "Commit materially changes a Requirement, Decision, "
                  + "Component, Assumption, or any STPA element.";
        :>> actionText = "Recompute suspicionState; comment newly-suspect "
                  + "items on the PR.";
        :>> producedArtifact = "Suspect set surfaced.";
        :>> owner = Owner::ai;
    }
    action sweepResolve : ProcessStep {
        :>> id = "e640fd43-a15e-428d-89f9-0e3558ab1310";
        :>> title = "Resolve suspicion";
        :>> triggerCondition = "Item is suspect.";
        :>> actionText = "Re-run/re-judge its Tests (auto or human by "
                  + "method), or open a follow-up Story.";
        :>> producedArtifact = "Coverage restored or follow-up created.";
        :>> owner = Owner::either;
    }


    // refinement: step order, native succession (D0143 — was `order : Integer`)
    first refineCriteria then refineSize;
    first refineSize then refineLink;

    // planning: step order, native succession (D0143 — was `order : Integer`)
    first planGoal then planPull;

    // implementation: step order, native succession (D0143 — was `order : Integer`)
    first implDo then implModel;
    first implModel then implReview;
    first implReview then implClose;

    // retrospective: step order, native succession (D0143 — was `order : Integer`)
    first retroIdentify then retroTrack;
    first retroTrack then retroPromote;

    // stalenessSweep: step order, native succession (D0143 — was `order : Integer`)
    first sweepDetect then sweepResolve;
}
