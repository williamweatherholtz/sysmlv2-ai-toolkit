package Decision0006 {
    private import EngineWork::*;

    part d0006 : Decision {
        :>> id = "9a61a74f-745b-4a0a-959d-af8003d87812";
        :>> title = "Verification by method; drop subjective/objective; verify subsumes validate";
        :>> status = DecisionStatus::accepted;
        :>> context = "An objective/subjective flag on tests was proposed. It is "
            + "redundant with the verification method. Separately, verify vs "
            + "validate were considered as distinct edges.";
        :>> decisionText = "Tests carry a `method`: test | analysis | inspection | "
            + "demonstration. Whether a human is required FALLS OUT of method "
            + "(test/analysis automatable; inspection/demonstration human) — no "
            + "separate flag. `validate` is NOT a distinct edge: validation is "
            + "`verify` pointing at a need / market-requirement / use-case, "
            + "distinguished by target type via query.";
        :>> consequences = "Pros: fewer concepts; re-verification automation "
            + "derives from method. Cons: V&V distinction is a query, not a "
            + "stored label (acceptable).";
    }
}
