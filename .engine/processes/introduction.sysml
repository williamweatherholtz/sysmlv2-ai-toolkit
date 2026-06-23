// =====================================================================
// EngineIntroduction — the guided, PROJECT-BASED onboarding process (D0093). The first session on a
// freshly `keel init`-ed project: the AI orients the newcomer, captures the project's FIRST business
// need, runs the first refine->sprint to a real tracked artifact, and shows `orient` — learning by
// DOING the first real item, not a doc dump (onboarding research: time-to-first-value, project-based
// learning beats reading). Dual-surface (D0093): the AI drives via CLI; the human supervises (states
// the need, accepts). Deployed by the introduction skill.
// =====================================================================
package EngineIntroduction {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    part introduction : Process {
        :>> id = "63940516-2c7d-4e8f-c093-516273849370";
        :>> title = "Guided project-based onboarding (spin-up first value)";
        :>> purpose = "Take a newcomer on a freshly-scaffolded project from zero to FIRST VALUE in one guided session: a 1-minute mental model, then capture the project's first business Need, derive a first requirement + work item, run the first sprint to a real tracked artifact, and show `orient`. Project-based (learn by doing the first real item), NOT a doc dump — combat the MBSE adoption cliff (D0054) by demonstrating the loop + the friction win fast. Dual-surface (D0093): the AI drives the CLI; the human supervises (provides the need, accepts content).";
        :>> cadence = Cadence::adHoc;
    }

    part inOrient : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c093-516273849371";
        :>> title = "Orient the newcomer — the 1-minute mental model";
        :>> order = 1;
        :>> triggerCondition = "A freshly `keel init`-ed project, or a human asks how to start / 'introduce me to the engine'.";
        :>> actionText = "In ~1 minute, give the working mental model (do NOT recite the decision log): (1) TEXT IS TRUTH — author only irreducible facts (atomic items + typed edges); everything else (status, matrices, reports) is COMPUTED, never stored. (2) THE LOOP — every request is triaged to CHANGE / EXECUTE / RECORD / VIEW / ORIENT (CLAUDE.md section 3). (3) DUAL SURFACE (D0093) — the AI drives the CLI/JSON (the authority); you supervise via HTML views + by accepting decisions. (4) Where to look: `keel orient` for where-things-stand, CLAUDE.md for how-to-work. Point at the deeper reading (CLAUDE.md -> .engine/decisions) for LATER, not now.";
        :>> producedArtifact = "Shared understanding of the loop; nothing authored yet.";
        :>> owner = Owner::ai;
    }
    part inFirstNeed : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c093-516273849372";
        :>> title = "Capture the first business Need (what / why)";
        :>> order = 2;
        :>> triggerCondition = "The newcomer can state, in a sentence, what the project is for.";
        :>> actionText = "Elicit the project's FIRST business need (the Business workflow / 'what-why') and record it as a single atomic `Need` item in .tracking/ (id + title + createdAt/createdBy) — the human states it; the AI authors it. ONE need, not a backlog dump (project-based: start with the real, smallest first thing). This is the first authored fact and the root of traceability.";
        :>> producedArtifact = "One `Need` item in .tracking/ (the project's first authored truth).";
        :>> owner = Owner::ai;
    }
    part inRefine : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c093-516273849373";
        :>> title = "Derive a first requirement + work item (refine to ready)";
        :>> order = 3;
        :>> triggerCondition = "The first Need exists.";
        :>> actionText = "Derive ONE SystemRequirement that `satisfy`-links the Need, and one work item (Story/action) to deliver it; refine to DoR (acceptance criteria as a verifiable DoD Test). Use the write API (`keel add-task`) + typed edges — show that recording a fact is ONE command (the friction win, D0054), lower than a spreadsheet. Keep it minimal: one need -> one requirement -> one work item.";
        :>> producedArtifact = "A `SystemRequirement` (satisfy edge) + a refined work item with a DoD, ready to start.";
        :>> owner = Owner::ai;
    }
    part inFirstSprint : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c093-516273849374";
        :>> title = "Run the first sprint — produce a real artifact";
        :>> order = 4;
        :>> triggerCondition = "A first work item is ready (DoR passed).";
        :>> actionText = "Run the first sprint via the agile-workflow ceremony (refine->standup->implement->review->closeOut->retro, autonomous per D0049) to produce the work item's defined artifact, recording the DoD TestResult via the write API. The point is a COMPLETED real increment + a recorded fact, so the newcomer sees the full loop close once on their OWN work.";
        :>> producedArtifact = "A completed first sprint: the artifact + a recorded passing DoD TestResult.";
        :>> owner = Owner::ai;
    }
    part inShowValue : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c093-516273849375";
        :>> title = "Show value + hand off to the loop";
        :>> order = 5;
        :>> triggerCondition = "The first sprint closed.";
        :>> actionText = "Show the computed payoff: `keel orient` (where things stand + what's ready next), and the human-facing lenses (`keel orient --html` dashboard, `keel report`, `keel render` review) — proving status/trace/coverage are COMPUTED from the one need->requirement->work->verification chain just authored. Then hand off to the normal interaction loop (CLAUDE.md section 3): from here the AI drives, the human supervises + decides. Onboarding is COMPLETE when the newcomer has one closed item and can read orient.";
        :>> producedArtifact = "A demonstrated computed view over the newcomer's own first item; transition to the steady-state loop.";
        :>> owner = Owner::human;
    }
}
