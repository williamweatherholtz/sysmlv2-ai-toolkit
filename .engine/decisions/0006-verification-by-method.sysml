package Engine { package Decisions {
    private import Engine::Core::*;

    part d0006 : Decision {
        :>> title = "Verification by method; drop subjective/objective; verify subsumes validate";
        :>> status = "accepted";
        :>> context = "An objective/subjective flag on tests was proposed. It is "
            + "redundant with the verification method. Separately, verify vs "
            + "validate were considered as distinct edges.";
        :>> decision = "Tests carry a `method`: test | analysis | inspection | "
            + "demonstration. Whether a human is required FALLS OUT of method "
            + "(test/analysis automatable; inspection/demonstration human) — no "
            + "separate flag. `validate` is NOT a distinct edge: validation is "
            + "`verify` pointing at a need / market-requirement / use-case, "
            + "distinguished by target type via query.";
        :>> consequences = "Pros: fewer concepts; re-verification automation "
            + "derives from method. Cons: V&V distinction is a query, not a "
            + "stored label (acceptable).";
    }
}}
