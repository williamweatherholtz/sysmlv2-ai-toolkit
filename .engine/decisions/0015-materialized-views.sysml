package Decision0015 {
    private import EngineWork::*;

    part d0015 : Decision {
        :>> id = "9b870d42-b946-4b8c-8012-e25394d09a86";
        :>> title = "Materialized views are sanctioned when marked and regenerable";
        :>> status = "accepted";
        :>> context = "Critiquing invariant 2.6 surfaced real costs of pure-computed state: no legibility without a runtime, no portability to other tools, recompute cost.";
        :>> decisionText = "A derived answer (status, trace, baseline) MAY be cached/rendered for legibility, performance, or interop, PROVIDED it is clearly marked derived (#View) and regenerable from authored facts + git. Materializing a view is not authoring truth; only irreducible facts and recorded judgments are authored. Captured in CLAUDE.md 2.6. Accepted by user 2026-06-10 (commit 062b97f).";
        :>> consequences = "Keeps truth single-sourced while allowing readable artifacts; viewers must treat #View content as disposable.";
    }
}
