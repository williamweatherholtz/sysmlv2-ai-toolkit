package Decision0004 {
    private import EngineWork::*;

    part d0004 : Decision {
        :>> id = "0f794d10-85e6-40de-bd0c-a09b0546deda";
        :>> title = "Atomic items, not nested checklists";
        :>> status = DecisionStatus::accepted;
        :>> context = "An early proposal nested a verification checklist inside "
            + "each Story (tagged objective/subjective). Soft criteria buried "
            + "in items erode into rubber-stamps.";
        :>> decision = "Tests, decisions, requirements, and acceptance criteria are ATOMIC, first-class, independently queryable elements linked by typed edges — never sub-lines inside another item; a Story's acceptance criteria ARE its verify-linked Tests, and satisfaction propagates through the edge graph.";
        :>> rationale = "An early proposal nested a verification checklist inside each Story (objective/subjective tagged), but soft criteria buried in items erode into rubber-stamps; making them atomic gives independent coverage and clean queries, and a parent is not satisfied while a downstream verifying Test is not passing.";
        :>> consequences = "Pros: independent test/decision/requirement coverage, "
            + "clean queries, no eroding checklists. Cons: more elements and "
            + "edges to author (offset by AI doing the mechanical linking).";
    }
}
