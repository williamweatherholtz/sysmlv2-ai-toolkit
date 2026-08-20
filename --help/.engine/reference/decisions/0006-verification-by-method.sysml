package Decision0006 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0006 : Decision {
        :>> id = "9a61a74f-745b-4a0a-959d-af8003d87812";
        :>> title = "Verification by method; drop subjective/objective; verify subsumes validate";
        :>> status = DecisionStatus::accepted;
        :>> context = "An objective/subjective flag on tests was proposed. It is "
            + "redundant with the verification method. Separately, verify vs "
            + "validate were considered as distinct edges.";
        :>> decision = "Tests carry a `method` (test | analysis | inspection | demonstration); whether a human is required FALLS OUT of method with no separate flag; and `validate` is NOT a distinct edge — validation is `verify` pointing at a need / market-requirement / use-case, distinguished by target type via query.";
        :>> rationale = "An objective/subjective flag was proposed but is redundant with the verification method, and verify vs validate as distinct edges was also considered; deriving human-required and V&V distinction from method/target gives fewer concepts and method-driven re-verification automation, accepting that the V&V distinction becomes a query not a stored label.";
        :>> consequences = "Pros: fewer concepts; re-verification automation "
            + "derives from method. Cons: V&V distinction is a query, not a "
            + "stored label (acceptable).";
    }

    verification d0006Accept : Test { :>> id = "4fb36f3c-af32-4fa8-bfe9-b9df0a5ef867"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0006 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0006AcceptR1 : TestResult { :>> id = "3529a70d-0e09-48ee-863a-2e59a5545263"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
