package Work {
    import Conventions::*;
    import ToolkitRequirements::*;
    import Architecture::*;

    // ---------------------------------------------------------------
    // EPIC: Phase 3 — browser GUI + direct-write trust gradient.
    // ---------------------------------------------------------------

    part epicPhase3 : Epic {
        :>> title = "Phase 3 — browser GUI and direct-write policy";
        :>> status = "backlog";
        :>> owner = "wweath";
        :>> priority = "p2";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> themeArea = "ui";
    }

    part storySchemaForms : Story {
        :>> title = "Schema-driven form generator";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p2";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 5;
        :>> acceptanceCriteria =
            "1. GUI introspects Conventions via the API.\n"
          + "2. For each part def, a form is generated from "
          +    "attribute names and types.\n"
          + "3. Adding an attribute to a part def surfaces it in "
          +    "the form on next reload — no GUI code change.";
    }

    part storyKanbanView : Story {
        :>> title = "Kanban view of WorkItems";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p2";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. Columns: backlog, ready, in_progress, in_review, "
          +    "done, blocked.\n"
          + "2. Drag-and-drop changes status via API.\n"
          + "3. Filter by Epic, owner, priority.";
    }

    part storyGraphView : Story {
        :>> title = "Graph view of model elements";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p2";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 5;
        :>> acceptanceCriteria =
            "1. Renders the typed graph centered on a selected "
          +    "element.\n"
          + "2. Edges colored by relationship type.\n"
          + "3. Click-to-navigate to neighbors.";
    }

    part storyStaleView : Story {
        :>> title = "Staleness queue view";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p2";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. Shows items flagged by the staleness sweep.\n"
          + "2. Each row links to the upstream change that "
          +    "triggered the flag.\n"
          + "3. Resolve actions: acknowledge, open follow-up Story.";
    }

    part storyDirectWritePolicy : Story {
        :>> title = "Direct-commit write policy";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p2";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. AISkill.writePolicy = 'direct' enables direct "
          +    "commits to main via the API.\n"
          + "2. Audit log captures every direct write with "
          +    "skill identity and timestamp.\n"
          + "3. Per-skill rate limit configurable.";
    }

    // satisfy edges
    satisfy browserGUI by storySchemaForms;
    satisfy browserGUI by storyKanbanView;
    satisfy browserGUI by storyGraphView;
    satisfy schemaDrivenForms by storySchemaForms;

    // architecture realization
    dependency gui_real : Dependency {
        :>> source = storySchemaForms;
        :>> target = browserGui;
    }

    // epic decomposition
    dependency epicPhase3_decomp1 : Dependency {
        :>> source = epicPhase3;
        :>> target = storySchemaForms;
    }
    dependency epicPhase3_decomp2 : Dependency {
        :>> source = epicPhase3;
        :>> target = storyKanbanView;
    }
    dependency epicPhase3_decomp3 : Dependency {
        :>> source = epicPhase3;
        :>> target = storyGraphView;
    }
    dependency epicPhase3_decomp4 : Dependency {
        :>> source = epicPhase3;
        :>> target = storyStaleView;
    }
    dependency epicPhase3_decomp5 : Dependency {
        :>> source = epicPhase3;
        :>> target = storyDirectWritePolicy;
    }

    // phase ordering
    dependency phase3_on_phase2 : Dependency {
        :>> source = epicPhase3;
        :>> target = epicPhase2;
    }
}
