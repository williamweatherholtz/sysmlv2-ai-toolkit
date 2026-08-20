package Decision0015 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0015 : Decision {
        :>> id = "9b870d42-b946-4b8c-8012-e25394d09a86";
        :>> title = "Materialized views are sanctioned when marked and regenerable";
        :>> status = DecisionStatus::accepted;
        :>> context = "Critiquing invariant 2.6 surfaced real costs of pure-computed state: no legibility without a runtime, no portability to other tools, recompute cost.";
        :>> decision = "A derived answer (status, trace, baseline) MAY be cached/rendered for legibility, performance, or interop, PROVIDED it is clearly marked derived (#View) and regenerable from authored facts + git. Materializing a view is not authoring truth; only irreducible facts and recorded judgments are authored.";
        :>> rationale = "Critiquing invariant 2.6 surfaced real costs of pure-computed state: no legibility without a runtime, no portability to other tools, recompute cost. Materialized views keep truth single-sourced while allowing readable artifacts. Captured in CLAUDE.md 2.6. Accepted by user 2026-06-10 (commit 062b97f).";
        :>> consequences = "Keeps truth single-sourced while allowing readable artifacts; viewers must treat #View content as disposable.";
    }

    verification d0015Accept : Test { :>> id = "cd21b091-8e64-479c-984e-c689418ab395"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0015 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0015AcceptR1 : TestResult { :>> id = "369d519e-2bd1-4d35-882e-820d3239f4ac"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
