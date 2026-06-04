package Engine { package Decisions {
    private import Engine::Core::*;

    part d0004 : Decision {
        :>> title = "Atomic items, not nested checklists";
        :>> status = "accepted";
        :>> context = "An early proposal nested a verification checklist inside "
            + "each Story (tagged objective/subjective). Soft criteria buried "
            + "in items erode into rubber-stamps.";
        :>> decision = "Tests, decisions, requirements, and acceptance criteria "
            + "are ATOMIC, first-class, independently queryable elements linked "
            + "by typed edges — never sub-lines inside another item. A Story's "
            + "acceptance criteria ARE its verify-linked Tests. Satisfaction "
            + "propagates through the edge graph: a parent is not satisfied "
            + "while a downstream verifying Test is not passing.";
        :>> consequences = "Pros: independent test/decision/requirement coverage, "
            + "clean queries, no eroding checklists. Cons: more elements and "
            + "edges to author (offset by AI doing the mechanical linking).";
    }
}}
