package Decision0005 {
    private import EngineWork::*;

    part d0005 : Decision {
        :>> title = "Computed-state engine: git-ancestry, material-change, scoped edges";
        :>> status = "accepted";
        :>> context = "Satisfaction/coverage/suspicion must be computed (0001). "
            + "A naive implementation storms suspicion on cosmetic edits and "
            + "mis-orders evidence using wall-clock time.";
        :>> decisionText = "Three rules (full spec in contracts/computed-state.md): "
            + "(1) happened-before is GIT ANCESTRY, not wall-clock — a pass "
            + "covers a change only if its judgedAgainst commit is a "
            + "descendant-or-equal of the change commit. (2) MATERIAL-CHANGE "
            + "detection: only semantic-field changes bump an element's "
            + "last-change commit; cosmetic edits don't propagate. (3) "
            + "Suspicion propagates only along :>, satisfy, verify, allocate — "
            + "NOT along @OrderingOnly dependencies.";
        :>> consequences = "Pros: trustworthy live suspicion; no flag fatigue. "
            + "Cons: indexer needs git-ancestry + semantic-hash logic; coverage "
            + "states (covered/suspect/failing/uncovered) must be implemented.";
    }
}
