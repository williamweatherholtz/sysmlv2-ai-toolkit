package Engine { package Decisions {
    private import Engine::Core::*;

    part d0001 : Decision {
        :>> title = "Text files are the sole source of truth; computed values are views";
        :>> status = "accepted";
        :>> context = "State could live in text (.sysml) or in a graph DB. "
            + "Two homes for one fact creates the 'racing the API' failure "
            + "mode. Separately, the engine relies on many COMPUTED fields "
            + "(satisfaction, coverage, suspicion).";
        :>> decision = "Authored facts (elements, edges, logged TestResults) "
            + "live ONLY in text. The index (Kuzu) is rebuilt from text and "
            + "is disposable. Computed values are VIEWS, recomputed on demand, "
            + "and NEVER stored as attributes in any .sysml file. A pure "
            + "function of truth is not itself truth.";
        :>> consequences = "Pros: git-native review, no sync tax, clean diffs, "
            + "rebuildable index. Cons: raw-file readers see evidence not "
            + "verdicts (mitigated by a CI bot commenting computed deltas on "
            + "PRs); text round-trip without destroying formatting becomes "
            + "the writer's hardest problem.";
    }
}}
