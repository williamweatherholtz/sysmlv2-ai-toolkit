package Architecture {
    import Conventions::*;

    // ---------------------------------------------------------------
    // Component decomposition of the toolkit itself.
    // Each Component is satisfied by one or more Stories in work/.
    // ---------------------------------------------------------------

    part parser : Component {
        :>> name = "Parser";
        :>> kind = "library";
        :>> language = "tbd";
        :>> notes = "Wraps the SysMLv2 pilot implementation (subprocess "
                  + "or REST). Optionally a Tree-sitter grammar later. "
                  + "Returns AST for the indexer and writer.";
    }

    part indexer : Component {
        :>> name = "Indexer";
        :>> kind = "library";
        :>> language = "tbd";
        :>> notes = "Walks AST, populates a Kùzu embedded graph DB. "
                  + "Files are truth; index is derived. Rebuilt on "
                  + "file watch events.";
    }

    part queryCli : Component {
        :>> name = "Query CLI";
        :>> kind = "cli";
        :>> language = "tbd";
        :>> notes = "Commands: whats-downstream, whats-stale-since, "
                  + "validate, list-by-type, get. Hits the indexer "
                  + "directly (no API yet at Phase 0).";
    }

    part validator : Component {
        :>> name = "Validator";
        :>> kind = "library";
        :>> language = "tbd";
        :>> notes = "Checks that every relationship references an "
                  + "element that exists. Reports dangling references. "
                  + "CI-runnable.";
    }

    part watcher : Component {
        :>> name = "File watcher";
        :>> kind = "library";
        :>> language = "tbd";
        :>> notes = "Watches the repo for .sysml changes; triggers "
                  + "re-parse and re-index. Debounced.";
    }

    part apiService : Component {
        :>> name = "API service";
        :>> kind = "service";
        :>> language = "tbd";
        :>> notes = "REST + GraphQL + WebSocket. Single API surface "
                  + "for GUI and AI. Read-only in Phase 1; writes in "
                  + "Phase 2 (proposal-based) and Phase 3 (direct).";
    }

    part writer : Component {
        :>> name = "Writer";
        :>> kind = "library";
        :>> language = "tbd";
        :>> notes = "Round-trips changes back to text. Strategy: "
                  + "surgical text edits (find named block, modify "
                  + "in place). Preserves human formatting. Hardest "
                  + "single piece of the build.";
    }

    part browserGui : Component {
        :>> name = "Browser GUI";
        :>> kind = "ui";
        :>> language = "tbd";
        :>> notes = "Schema-driven forms generated from Conventions. "
                  + "Kanban view, graph view, query views. Uses only "
                  + "the public API.";
    }

    // ---------------------------------------------------------------
    // Dependencies between toolkit components.
    // ---------------------------------------------------------------

    dependency indexer_on_parser : Dependency {
        :>> source = indexer;
        :>> target = parser;
    }

    dependency validator_on_parser : Dependency {
        :>> source = validator;
        :>> target = parser;
    }

    dependency queryCli_on_indexer : Dependency {
        :>> source = queryCli;
        :>> target = indexer;
    }

    dependency apiService_on_indexer : Dependency {
        :>> source = apiService;
        :>> target = indexer;
    }

    dependency apiService_on_writer : Dependency {
        :>> source = apiService;
        :>> target = writer;
    }

    dependency writer_on_parser : Dependency {
        :>> source = writer;
        :>> target = parser;
    }

    dependency browserGui_on_apiService : Dependency {
        :>> source = browserGui;
        :>> target = apiService;
    }

    dependency apiService_on_watcher : Dependency {
        :>> source = apiService;
        :>> target = watcher;
    }
}
