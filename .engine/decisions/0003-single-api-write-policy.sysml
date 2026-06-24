package Decision0003 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0003 : Decision {
        :>> id = "13e4fd55-8929-4e78-9942-44d9d8473085";
        :>> title = "Single API surface for humans and AI; enforced write policy";
        :>> status = DecisionStatus::superseded;  // superseded by D0096: the same-identical-API claim was overtaken by D0093's dual-surface, and the API-enforced writePolicy was never realized (issue045) — authorization is the human commit-gate (D0094). supersede back-reference is computed.
        :>> context = "Building a 'rich' GUI API and a 'simple' AI API causes "
            + "drift between human and AI capability. AI must be first-class "
            + "but not unconstrained.";
        :>> decision = "The browser GUI and AI agents use the SAME API with identical capability; authorization differs via a per-skill writePolicy (read-only | pr-only | direct) ENFORCED by the API, and a skill cannot self-promote its policy (changing it requires a human edit to the skills registry).";
        :>> rationale = "Building a 'rich' GUI API and a 'simple' AI API causes drift between human and AI capability; AI must be first-class but not unconstrained, so a single enforced API with policy-only differences gives parity, no drift, and auditable AI writes.";
        :>> consequences = "Pros: parity, no drift, auditable AI writes. Cons: "
            + "one API must serve UI ergonomics and machine use (GraphQL eases "
            + "this).";
    }

    verification d0003Accept : Test { :>> id = "2d5ce709-1369-4308-93c2-2a8bf69e06c2"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0003 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0003AcceptR1 : TestResult { :>> id = "32883715-4194-4caa-9c0d-70745b821254"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
