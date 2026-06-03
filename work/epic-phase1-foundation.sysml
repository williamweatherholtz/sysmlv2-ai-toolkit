package Work {
    import Conventions::*;
    import ToolkitRequirements::*;
    import Architecture::*;

    // ---------------------------------------------------------------
    // EPIC: Phase 1 — parser, indexer, query CLI, validator.
    // The first tools that make the model machine-queryable.
    // No API and no GUI yet — that's Phase 2+.
    // ---------------------------------------------------------------

    part epicPhase1 : Epic {
        :>> title = "Phase 1 — parser, indexer, query CLI, validator";
        :>> status = "backlog";
        :>> owner = "wweath";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> themeArea = "tools";
    }

    part storyPickStack : Story {
        :>> title = "Pick implementation language and dependencies";
        :>> status = "ready";
        :>> owner = "tbd";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 1;
        :>> acceptanceCriteria =
            "1. Decision recorded in decisions/ choosing Python or "
          +    "TypeScript (or other).\n"
          + "2. Rationale notes ecosystem, parsing options, Kùzu "
          +    "bindings.\n"
          + "3. Architecture components updated with language field.";
    }

    part storyParserWrapper : Story {
        :>> title = "Wrap the pilot implementation parser";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 5;
        :>> acceptanceCriteria =
            "1. Given a .sysml file, returns a structured AST or "
          +    "raises a parse error with location.\n"
          + "2. Handles imports across packages.\n"
          + "3. Unit tests on a fixture set covering every "
          +    "construct used in conventions.sysml.";
    }

    part storyIndexer : Story {
        :>> title = "Build the Kùzu-backed indexer";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 5;
        :>> acceptanceCriteria =
            "1. Given a directory of .sysml files, populates a Kùzu "
          +    "graph: nodes for each element, edges for each typed "
          +    "relationship.\n"
          + "2. Idempotent: re-running drops and rebuilds cleanly.\n"
          + "3. Index lives at .index/ (gitignored).";
    }

    part storyValidator : Story {
        :>> title = "Implement the validator";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. Reports every dangling reference (satisfy/verify/"
          +    "dependency/etc. pointing at a non-existent "
          +    "qualified name).\n"
          + "2. Exit code non-zero on any error.\n"
          + "3. Runs in CI on every PR.";
    }

    part storyQueryCli : Story {
        :>> title = "Build the query CLI";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p0";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 5;
        :>> acceptanceCriteria =
            "1. Command: list <Type> — lists all elements of a type.\n"
          + "2. Command: get <qname> — shows element with relationships.\n"
          + "3. Command: whats-downstream <qname> — transitive closure.\n"
          + "4. Command: whats-stale-since <git-ref> — diff-based "
          +    "staleness sweep.\n"
          + "5. Command: validate — runs the validator.\n"
          + "6. Output is human-readable AND has --json mode for AI.";
    }

    part storyFileWatcher : Story {
        :>> title = "Add file-watch auto-reindex";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p1";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. Daemon mode watches the repo and re-indexes on "
          +    "change.\n"
          + "2. Debounced to avoid thrashing during a save burst.\n"
          + "3. CLI can start/stop the daemon.";
    }

    // satisfy edges to requirements
    satisfy parseable by storyParserWrapper;
    satisfy queryDownstream by storyQueryCli;
    satisfy stalenessDetection by storyQueryCli;
    satisfy validation by storyValidator;
    satisfy portable by storyParserWrapper;

    // architecture realization
    dependency parser_real : Dependency {
        :>> source = storyParserWrapper;
        :>> target = parser;
    }
    dependency indexer_real : Dependency {
        :>> source = storyIndexer;
        :>> target = indexer;
    }
    dependency validator_real : Dependency {
        :>> source = storyValidator;
        :>> target = validator;
    }
    dependency queryCli_real : Dependency {
        :>> source = storyQueryCli;
        :>> target = queryCli;
    }
    dependency watcher_real : Dependency {
        :>> source = storyFileWatcher;
        :>> target = watcher;
    }

    // epic decomposition
    dependency epicPhase1_decomp1 : Dependency {
        :>> source = epicPhase1;
        :>> target = storyPickStack;
    }
    dependency epicPhase1_decomp2 : Dependency {
        :>> source = epicPhase1;
        :>> target = storyParserWrapper;
    }
    dependency epicPhase1_decomp3 : Dependency {
        :>> source = epicPhase1;
        :>> target = storyIndexer;
    }
    dependency epicPhase1_decomp4 : Dependency {
        :>> source = epicPhase1;
        :>> target = storyValidator;
    }
    dependency epicPhase1_decomp5 : Dependency {
        :>> source = epicPhase1;
        :>> target = storyQueryCli;
    }
    dependency epicPhase1_decomp6 : Dependency {
        :>> source = epicPhase1;
        :>> target = storyFileWatcher;
    }
}
