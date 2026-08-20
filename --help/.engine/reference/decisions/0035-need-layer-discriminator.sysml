package Decision0035 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0035 : Decision {
        :>> id = "cf5a0afd-4b5c-4d6e-efa7-8b9c0d1e2f3a";
        :>> title = "needLayer: Need vs Requirement discriminated by SysML type; no additional field";
        :>> status = DecisionStatus::accepted;
        :>> context = "EngineNeeds defines `Need` as `requirement def Need :> TrackedRequirement` (source-discriminated: customer/operator/safety/etc). EngineRequirements defines `Requirement :> TrackedRequirement`. Both are `requirement def` — same SysML metaclass. The question: is additional discrimination needed beyond the type name, or is `Need` vs `Requirement` sufficient?";
        :>> decision = "Adopted: the SysML type IS the discriminator. `Need` = stakeholder-voiced, source-discriminated outcome; `Requirement` = system-shaped, derive/satisfy-linked specification. No `layer` or `kind` field is added; the `source` enum on Need provides sub-discrimination and the satisfy edge connects the two layers.";
        :>> rationale = "Tooling can distinguish Needs from Requirements by type name alone (query.py trace-need already works with type-level discrimination), and the `source` enum (customer/operator/safety/regulatory/internal/competitive) gives the needed sub-discrimination — a Requirement satisfies a Need, and Safety Goals remain Needs with `source = NeedSource::safety`. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "No schema change. Tooling (query.py trace-need) already works with type-level discrimination. Future query views can filter by type name to produce Need vs Requirement layers separately. EngineNeeds and EngineRequirements schemas unchanged.";
    }

    verification d0035Accept : Test { :>> id = "a347608b-a72e-437f-86a2-5bcb03d44be5"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0035 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0035AcceptR1 : TestResult { :>> id = "7ba0c31c-dada-48eb-a1ea-150ef1409f14"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
