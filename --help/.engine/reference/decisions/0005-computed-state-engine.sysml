package Decision0005 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0005 : Decision {
        :>> id = "d48e2ef4-af60-454b-ac16-765e2486ddb6";
        :>> title = "Computed-state engine: git-ancestry, material-change, scoped edges";
        :>> status = DecisionStatus::accepted;
        :>> context = "Satisfaction/coverage/suspicion must be computed (0001). "
            + "A naive implementation storms suspicion on cosmetic edits and "
            + "mis-orders evidence using wall-clock time.";
        :>> decision = "Three rules govern computed state (full spec in contracts/computed-state.md): (1) happened-before is GIT ANCESTRY not wall-clock; (2) MATERIAL-CHANGE detection — only semantic-field changes bump an element's last-change commit; (3) suspicion propagates only along :>, satisfy, verify, allocate — NOT along @OrderingOnly dependencies.";
        :>> rationale = "Satisfaction/coverage/suspicion must be computed (per 0001), but a naive implementation storms suspicion on cosmetic edits and mis-orders evidence using wall-clock time; git-ancestry ordering, material-change detection, and scoped-edge propagation give trustworthy live suspicion with no flag fatigue.";
        :>> consequences = "Pros: trustworthy live suspicion; no flag fatigue. "
            + "Cons: indexer needs git-ancestry + semantic-hash logic; coverage "
            + "states (covered/suspect/failing/uncovered) must be implemented.";
    }

    verification d0005Accept : Test { :>> id = "7cdc0b08-f5fe-4ee0-b8e6-5175ae190186"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0005 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0005AcceptR1 : TestResult { :>> id = "29d0246c-a3ad-49c5-9935-7cb920677b73"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
