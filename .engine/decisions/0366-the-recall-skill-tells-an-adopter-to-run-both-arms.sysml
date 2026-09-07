// D0366 (PROPOSED — NOT YET ACCEPTED) — recallSkillNamesBothArms: the knowledge-graph-memory skill tells a project changing the ranker to measure on both benchmark arms, not one
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0366Accept event on sign-off.
package Decision0366 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0366 : Decision {
        :>> id = "e336ba9f-305c-42c8-a7a7-5a6fb0cc5b7d";
        :>> title = "recallSkillNamesBothArms: the knowledge-graph-memory skill tells a project changing the ranker to measure on both benchmark arms, not one";
        :>> createdAt = "2026-09-07";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The knowledge-graph-memory skill's step-7 row reads 'measure on your own model if you change the ranker' and points an adopting project at recall_bench.py and recall_ab.py. That sentence was not made false by D0364, which is why this is an addition rather than a doc-sync correction. It is, however, now known to be insufficient in a way that cost this repository three sprints: the default arm draws every query from a record one edge from the target, so a project that changes the ranker in a way that reaches further will measure six configurations, see identical numbers, and conclude the change does nothing. That is exactly what happened here, and D0325 acted on that conclusion for a different mechanism. An adopter has strictly less context than we did and no reason to suspect the instrument.";
        :>> decision = "The step-7 row names both arms: run `--hops 2` as well as the default, because a set of one-hop cases cannot tell any setting of a two-hop mechanism from off, and it cites D0364 so a reader can follow the measurement rather than take the claim. The row's existing guidance is unchanged.";
        :>> rationale = "The skill is the surface an adopting project actually reads; the finding is useless if it lives only in this repository's Decisions and sprint records. It is one line rather than a section because the failure mode is specific and the remedy is a flag - a longer treatment would be the skill explaining our benchmark rather than telling a project what to run. It goes through the process-change lock and waits for a signature rather than being slipped in as a doc edit, because that lock exists precisely so a process definition does not change on the author's own judgment, and the fact that this particular line is small and I believe it is right is not a reason to route around it.";
        :>> consequences = "The skill file changes, so this Decision is co-committed with it under D0209 clause 2 and stays PROPOSED - a process-change marker is outside standing consent (D0337) and this one waits on the human alongside d0360 to d0363. The change is already in the tree because the lock gates the commit on the marker, not on the signature; if the human declines it, the line comes out. NOT A FORK: this Decision chooses one course.";
    }
}
