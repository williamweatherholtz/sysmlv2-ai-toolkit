package Decision0035 {
    private import EngineWork::*;

    part d0035 : Decision {
        :>> id = "cf5a0afd-4b5c-4d6e-efa7-8b9c0d1e2f3a";
        :>> title = "needLayer: Need vs Requirement discriminated by SysML type; no additional field";
        :>> status = DecisionStatus::accepted;
        :>> context = "EngineNeeds defines `Need` as `requirement def Need :> TrackedRequirement` (source-discriminated: customer/operator/safety/etc). EngineRequirements defines `Requirement :> TrackedRequirement`. Both are `requirement def` — same SysML metaclass. The question: is additional discrimination needed beyond the type name, or is `Need` vs `Requirement` sufficient?";
        :>> decision = "Adopted: the SysML type IS the discriminator. `Need` = stakeholder-voiced, source-discriminated outcome; `Requirement` = system-shaped, derive/satisfy-linked specification. No `layer` or `kind` field is added; the `source` enum on Need provides sub-discrimination and the satisfy edge connects the two layers.";
        :>> rationale = "Tooling can distinguish Needs from Requirements by type name alone (query.py trace-need already works with type-level discrimination), and the `source` enum (customer/operator/safety/regulatory/internal/competitive) gives the needed sub-discrimination — a Requirement satisfies a Need, and Safety Goals remain Needs with `source = NeedSource::safety`. Accepted: wweatherholtz 2026-06-16.";
        :>> decisionText = "Adopted: the SysML type IS the discriminator. `Need` = stakeholder-voiced, source-discriminated outcome; `Requirement` = system-shaped, derive/satisfy-linked specification. The `source` enum on Need (customer/operator/safety/regulatory/internal/competitive) provides sub-discrimination. No `layer` or `kind` field is added. The tooling can distinguish Needs from Requirements by type name alone. The satisfy edge connects the two layers (a Requirement satisfies a Need). Safety Goals remain Needs with `source = NeedSource::safety`. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "No schema change. Tooling (query.py trace-need) already works with type-level discrimination. Future query views can filter by type name to produce Need vs Requirement layers separately. EngineNeeds and EngineRequirements schemas unchanged.";
    }
}
