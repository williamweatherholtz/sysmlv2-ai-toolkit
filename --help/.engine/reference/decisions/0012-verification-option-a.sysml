package Decision0012 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0012 : Decision {
        :>> id = "ee75ccbf-5d66-43db-8fb3-64ff4e5b973e";
        :>> title = "Verification is native, method-tagged; human confirmation first-class";
        :>> status = DecisionStatus::accepted;
        :>> context = "Most engine tasks are not programming-centric; verification needed to be method-agnostic with human sign-off as a common path. Two options were defended (A: method on a native verification; B: code-test-centric workflow).";
        :>> decision = "Option A: every task is verified by a method-tagged criterion (inspect|analyze|demo|test|confirmation); done = a recorded pass result + the commit it was judged against (verifiedAtCommit). method=confirmation REQUIRES the human's explicit sign-off of the specific claim, never inferred.";
        :>> rationale = "Most engine tasks are not programming-centric, so verification must be method-agnostic with human sign-off as a common path; verifiedAtCommit enables git-ancestry suspicion. Automated tests are the procedure for method=test inside Delivery, not the universal workflow. Two options were defended (A: method on a native verification; B: code-test-centric workflow) and A was chosen. Accepted by user 2026-06-09/10. The interim AcceptanceCriterion dialect (verdict fields on the requirement) is recorded as debt, superseded by appended TestResults when CR-3 lands.";
        :>> consequences = "Lightweight universal done-semantics; suspicion gets its commit anchor; debt: mutable verdict fields until CR-3.";
    }

    verification d0012Accept : Test { :>> id = "01d464d9-1305-4e23-aca5-66281d3c8f2a"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0012 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0012AcceptR1 : TestResult { :>> id = "19675df7-e848-4138-94b5-59a85716a446"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
