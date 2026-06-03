package DefinitionOfDone {
    import Conventions::*;

    // ---------------------------------------------------------------
    // The Definition of Done is itself a Process — a checklist that
    // runs against every Story before its status flips to 'done'.
    // Modeling it as a Process keeps it queryable and revisable.
    // ---------------------------------------------------------------

    part dod : Process {
        :>> name = "Definition of Done";
        :>> purpose = "Gate that a Story actually meets quality bar "
                    + "before status = 'done'.";
        :>> cadence = "per-story";
    }

    part dodCheck1 : ProcessStep {
        :>> name = "Code committed and pushed";
        :>> order = 1;
        :>> trigger = "Story claimed to be ready for done.";
        :>> action = "Verify branch is merged to main or PR is "
                  + "approved.";
        :>> output = "Confirmed merged.";
        :>> owner = "ai";
    }

    part dodCheck2 : ProcessStep {
        :>> name = "Tests pass";
        :>> order = 2;
        :>> trigger = "Code merged.";
        :>> action = "All TestCases linked via 'verify' to the Story's "
                  + "requirement have lastResult = 'pass'.";
        :>> output = "Verified.";
        :>> owner = "ai";
    }

    part dodCheck3 : ProcessStep {
        :>> name = "Acceptance criteria met";
        :>> order = 3;
        :>> trigger = "Tests pass.";
        :>> action = "Verify each line of Story.acceptanceCriteria has "
                  + "either a passing TestCase or a human sign-off.";
        :>> output = "Criteria met.";
        :>> owner = "either";
    }

    part dodCheck4 : ProcessStep {
        :>> name = "Model updated";
        :>> order = 4;
        :>> trigger = "Implementation done.";
        :>> action = "Story attributes current; new Components or "
                  + "Decisions written; affected ExternalRefs updated.";
        :>> output = "Model reflects shipped reality.";
        :>> owner = "ai";
    }

    part dodCheck5 : ProcessStep {
        :>> name = "Downstream staleness cleared";
        :>> order = 5;
        :>> trigger = "Model updated.";
        :>> action = "Run StalenessSweep. Any items flagged by this "
                  + "Story's changes must be resolved (acknowledged or "
                  + "follow-up Story created).";
        :>> output = "No outstanding suspicion from this Story.";
        :>> owner = "either";
    }

    part dodCheck6 : ProcessStep {
        :>> name = "Documentation in sync";
        :>> order = 6;
        :>> trigger = "Story user-facing.";
        :>> action = "If Story changes user-facing behavior, "
                  + "docs/ is updated in the same PR.";
        :>> output = "Docs current.";
        :>> owner = "either";
    }
}
