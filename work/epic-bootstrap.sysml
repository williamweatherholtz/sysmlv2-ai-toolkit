package Work {
    import Conventions::*;
    import ToolkitRequirements::*;
    import Architecture::*;

    // ---------------------------------------------------------------
    // EPIC: Bootstrap — get the workflow itself running.
    // No tools yet; the model exists as text files and is queried by
    // humans + AI reading files directly until the indexer lands.
    // ---------------------------------------------------------------

    part epicBootstrap : Epic {
        :>> title = "Bootstrap the SysMLv2 work-tracking workflow";
        :>> status = "in_progress";
        :>> owner = "wweath";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> themeArea = "foundation";
    }

    part storyConventions : Story {
        :>> title = "Author initial Conventions package";
        :>> status = "done";
        :>> owner = "wweath";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. conventions/conventions.sysml exists.\n"
          + "2. Defines: WorkItem, Epic, Story, Task, Decision, "
          +    "Process, ProcessStep, AISkill, Agent, UseCase, "
          +    "Component, TestCase, ExternalRef, Sprint.\n"
          + "3. Status vocabulary documented in-file.";
    }

    part storyAgileProcess : Story {
        :>> title = "Model the agile workflow as Process / ProcessStep";
        :>> status = "done";
        :>> owner = "wweath";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. processes/agile-workflow.sysml exists.\n"
          + "2. Refinement, Planning, Implementation, Retrospective, "
          +    "and Staleness Sweep are modeled.\n"
          + "3. Each ProcessStep tags owner as human, ai, or either.";
    }

    part storyDefinitionOfDone : Story {
        :>> title = "Model Definition of Done";
        :>> status = "done";
        :>> owner = "wweath";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 2;
        :>> acceptanceCriteria =
            "1. processes/definition-of-done.sysml exists.\n"
          + "2. Steps cover: merged, tests pass, criteria met, "
          +    "model updated, staleness cleared, docs in sync.";
    }

    part storyToolkitRequirements : Story {
        :>> title = "Capture toolkit requirements";
        :>> status = "done";
        :>> owner = "wweath";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 2;
        :>> acceptanceCriteria =
            "1. requirements/toolkit-requirements.sysml exists.\n"
          + "2. Requirements cover parsing, querying, staleness, "
          +    "validation, API, GUI, AI parity, write policy, "
          +    "schema-driven forms, concurrency, portability.";
    }

    part storyArchitectureSketch : Story {
        :>> title = "Sketch toolkit architecture";
        :>> status = "done";
        :>> owner = "wweath";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 2;
        :>> acceptanceCriteria =
            "1. architecture/system.sysml exists.\n"
          + "2. Identifies: parser, indexer, queryCli, validator, "
          +    "watcher, apiService, writer, browserGui.\n"
          + "3. Dependency edges between components captured.";
    }

    part storyValidateAgainstPilot : Story {
        :>> title = "Validate all .sysml files against pilot impl";
        :>> status = "ready";
        :>> owner = "tbd";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. SysMLv2 pilot implementation installed locally.\n"
          + "2. Every .sysml file in repo parses without errors.\n"
          + "3. Syntax adjustments captured in a Decision if any "
          +    "convention changes.";
    }

    // satisfy edges
    satisfy parseable by storyValidateAgainstPilot;

    // decomposition (Epic -> Stories) — modeled via dependency for
    // now until decomposition syntax is validated
    dependency epicBootstrap_decomp1 : Dependency {
        :>> source = epicBootstrap;
        :>> target = storyConventions;
    }
    dependency epicBootstrap_decomp2 : Dependency {
        :>> source = epicBootstrap;
        :>> target = storyAgileProcess;
    }
    dependency epicBootstrap_decomp3 : Dependency {
        :>> source = epicBootstrap;
        :>> target = storyDefinitionOfDone;
    }
    dependency epicBootstrap_decomp4 : Dependency {
        :>> source = epicBootstrap;
        :>> target = storyToolkitRequirements;
    }
    dependency epicBootstrap_decomp5 : Dependency {
        :>> source = epicBootstrap;
        :>> target = storyArchitectureSketch;
    }
    dependency epicBootstrap_decomp6 : Dependency {
        :>> source = epicBootstrap;
        :>> target = storyValidateAgainstPilot;
    }
}
