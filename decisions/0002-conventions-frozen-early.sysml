package Decisions {
    import Conventions::*;

    part d0002 : Decision {
        :>> title = "Conventions package is frozen early";
        :>> status = "accepted";
        :>> date = "2026-06-03";

        :>> context = "If three contributors model 'task' three "
                    + "different ways, the typed-graph value "
                    + "evaporates. The original critique called this "
                    + "out as the convention-drift risk.";

        :>> decision = "The Conventions package (part defs and "
                    + "relationship intents) is treated as "
                    + "architectural. Changes require a Decision "
                    + "record. The initial set is intentionally small "
                    + "and grows only with explicit justification.";

        :>> consequences = "Pros: typed-graph integrity preserved; "
                    + "new contributors have one shape to learn. "
                    + "Cons: friction when a new domain concept "
                    + "needs to be added — must go through a "
                    + "Decision rather than ad-hoc.";

        :>> supersededBy = "";
    }

    part d0003 : Decision {
        :>> title = "Single API surface for humans and AI";
        :>> status = "accepted";
        :>> date = "2026-06-03";

        :>> context = "Building two APIs (a 'rich' one for the GUI "
                    + "and a 'simple' one for AI) is a common "
                    + "anti-pattern that creates drift between human "
                    + "and AI capabilities.";

        :>> decision = "The browser GUI and AI agents use the same "
                    + "API. Authorization differs (per-skill "
                    + "writePolicy) but capability does not. Any "
                    + "operation the GUI can do, an AI can do.";

        :>> consequences = "Pros: parity, no drift, AI is "
                    + "first-class. Cons: API must serve both UI "
                    + "ergonomics and machine readability — slight "
                    + "tension in shape (GraphQL helps).";

        :>> supersededBy = "";
    }
}
