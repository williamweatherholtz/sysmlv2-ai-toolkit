package Decision0014 {
    private import EngineWork::*;

    part d0014 : Decision {
        :>> id = "e92f5006-c0f2-4945-8aac-b74d2a540f3f";
        :>> title = "main is the canonical branch; everything pushes and merges to main only";
        :>> status = DecisionStatus::accepted;
        :>> context = "Work had accumulated on a feature branch (engine-restructure) under a don't-merge-until-told caution; the user found work stranded on side branches counterproductive for this solo+AI project.";
        :>> decisionText = "Commit accepted work directly to main; the post-commit hook pushes every commit; no long-lived feature branches. Acceptance still gates each CHANGE - this changes where commits land, not whether they need acceptance. Standing instruction from the user, 2026-06-11 (commit ae01dee; engine-restructure fast-forwarded into main).";
        :>> consequences = "Zero-friction publication; cost: nothing mechanical checks before publication (mitigation: CR-11 pre-commit validation).";
    }
}
