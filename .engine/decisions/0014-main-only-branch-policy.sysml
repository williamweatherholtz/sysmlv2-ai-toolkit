package Decision0014 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0014 : Decision {
        :>> id = "e92f5006-c0f2-4945-8aac-b74d2a540f3f";
        :>> title = "main is the canonical branch; everything pushes and merges to main only";
        :>> status = DecisionStatus::accepted;
        :>> context = "Work had accumulated on a feature branch (engine-restructure) under a don't-merge-until-told caution; the user found work stranded on side branches counterproductive for this solo+AI project.";
        :>> decision = "Commit accepted work directly to main; the post-commit hook pushes every commit; no long-lived feature branches. main is the canonical branch and everything pushes and merges to main only.";
        :>> rationale = "Work had stranded on a feature branch (engine-restructure) under a don't-merge-until-told caution, which the user found counterproductive for this solo+AI project. Acceptance still gates each CHANGE - this changes where commits land, not whether they need acceptance. Standing instruction from the user, 2026-06-11 (commit ae01dee; engine-restructure fast-forwarded into main).";
        :>> consequences = "Zero-friction publication; cost: nothing mechanical checks before publication (mitigation: CR-11 pre-commit validation).";
    }

    verification d0014Accept : Test { :>> id = "47a9cf8e-7afc-4f0c-8f1b-9b150784b74c"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0014 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0014AcceptR1 : TestResult { :>> id = "cc21e955-c90e-48f5-a6af-e049b186a8b6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
