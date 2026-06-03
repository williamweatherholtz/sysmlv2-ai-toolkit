package Work {
    import Conventions::*;
    import ToolkitRequirements::*;
    import Architecture::*;

    // ---------------------------------------------------------------
    // EPIC: Phase 2 — API service (read-only) + writer + write API
    // (PR-only). The first network surface for the model.
    // ---------------------------------------------------------------

    part epicPhase2 : Epic {
        :>> title = "Phase 2 — API service and proposal-based writes";
        :>> status = "backlog";
        :>> owner = "wweath";
        :>> priority = "p1";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> themeArea = "tools";
    }

    part storyReadOnlyApi : Story {
        :>> title = "Read-only REST + GraphQL API";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p1";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 5;
        :>> acceptanceCriteria =
            "1. REST: GET /elements?type=X, GET /elements/<qname>, "
          +    "GET /downstream/<qname>, GET /stale-since/<ref>.\n"
          + "2. GraphQL: schema introspected from Conventions; "
          +    "graph traversal queries.\n"
          + "3. WebSocket: pushes 'element-changed' events.\n"
          + "4. Auth: API token (single-user default).";
    }

    part storyWriter : Story {
        :>> title = "Implement the writer (round-trip text)";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p1";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 8;
        :>> acceptanceCriteria =
            "1. Given (file, element-qname, attribute-changes), "
          +    "edits the file in place preserving comments and "
          +    "formatting outside the changed block.\n"
          + "2. Round-trip property: parse -> modify -> write -> "
          +    "parse yields equivalent AST.\n"
          + "3. Fuzz tested across the Conventions construct set.\n"
          + "4. Fails closed if the edit would corrupt syntax.";
    }

    part storyPrWriteApi : Story {
        :>> title = "Write API — PR-only mode";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p1";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 5;
        :>> acceptanceCriteria =
            "1. POST /elements creates an element on a new branch.\n"
          + "2. PATCH /elements/<qname> edits on a new branch.\n"
          + "3. Branches push and open a PR via git CLI / gh.\n"
          + "4. Response includes branch name and PR URL.";
    }

    part storyOptimisticLocking : Story {
        :>> title = "Optimistic concurrency control";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p1";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. Every GET returns an etag (file hash).\n"
          + "2. PATCH requires If-Match etag.\n"
          + "3. 409 Conflict on mismatch.\n"
          + "4. AI client respects 409 and re-reads before retry.";
    }

    part storyAiWritePolicy : Story {
        :>> title = "Enforce AISkill writePolicy in API";
        :>> status = "backlog";
        :>> owner = "tbd";
        :>> priority = "p1";
        :>> created = "2026-06-03";
        :>> updated = "2026-06-03";
        :>> estimatedPoints = 3;
        :>> acceptanceCriteria =
            "1. API token is associated with an AISkill identity.\n"
          + "2. writePolicy = 'read-only' rejects all writes.\n"
          + "3. writePolicy = 'pr-only' forces branch + PR; rejects "
          +    "any attempt at direct.\n"
          + "4. writePolicy = 'direct' allowed only after Phase 3.";
    }

    // satisfy edges
    satisfy readOnlyAPI by storyReadOnlyApi;
    satisfy writeAPI by storyPrWriteApi;
    satisfy writeAPI by storyWriter;
    satisfy concurrencySafe by storyOptimisticLocking;
    satisfy aiSkillWritePolicy by storyAiWritePolicy;
    satisfy aiClientParity by storyReadOnlyApi;
    satisfy aiClientParity by storyPrWriteApi;

    // architecture realization
    dependency apiService_real : Dependency {
        :>> source = storyReadOnlyApi;
        :>> target = apiService;
    }
    dependency writer_real : Dependency {
        :>> source = storyWriter;
        :>> target = writer;
    }

    // epic decomposition
    dependency epicPhase2_decomp1 : Dependency {
        :>> source = epicPhase2;
        :>> target = storyReadOnlyApi;
    }
    dependency epicPhase2_decomp2 : Dependency {
        :>> source = epicPhase2;
        :>> target = storyWriter;
    }
    dependency epicPhase2_decomp3 : Dependency {
        :>> source = epicPhase2;
        :>> target = storyPrWriteApi;
    }
    dependency epicPhase2_decomp4 : Dependency {
        :>> source = epicPhase2;
        :>> target = storyOptimisticLocking;
    }
    dependency epicPhase2_decomp5 : Dependency {
        :>> source = epicPhase2;
        :>> target = storyAiWritePolicy;
    }

    // phase ordering — Phase 2 depends on Phase 1 epic
    dependency phase2_on_phase1 : Dependency {
        :>> source = epicPhase2;
        :>> target = epicPhase1;
    }
}
