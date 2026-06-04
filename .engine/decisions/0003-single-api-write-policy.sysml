package Engine { package Decisions {
    private import Engine::Core::*;

    part d0003 : Decision {
        :>> title = "Single API surface for humans and AI; enforced write policy";
        :>> status = "accepted";
        :>> context = "Building a 'rich' GUI API and a 'simple' AI API causes "
            + "drift between human and AI capability. AI must be first-class "
            + "but not unconstrained.";
        :>> decision = "The browser GUI and AI agents use the SAME API. "
            + "Capability is identical; authorization differs via per-skill "
            + "writePolicy (read-only | pr-only | direct), ENFORCED by the API. "
            + "A skill cannot self-promote its policy; changing it requires a "
            + "human edit to the skills registry.";
        :>> consequences = "Pros: parity, no drift, auditable AI writes. Cons: "
            + "one API must serve UI ergonomics and machine use (GraphQL eases "
            + "this).";
    }
}}
