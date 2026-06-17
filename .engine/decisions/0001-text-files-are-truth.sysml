package Decision0001 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0001 : Decision {
        :>> id = "219fca70-508f-482a-9f65-71f8bf8c2070";
        :>> title = "Text files are the sole source of truth; computed values are views";
        :>> status = DecisionStatus::accepted;
        :>> context = "State could live in text (.sysml) or in a graph DB. "
            + "Two homes for one fact creates the 'racing the API' failure "
            + "mode. Separately, the engine relies on many COMPUTED fields "
            + "(satisfaction, coverage, suspicion).";
        :>> decision = "Authored facts (elements, edges, logged TestResults) live ONLY in text; the index (Kuzu) is rebuilt from text and is disposable; computed values are VIEWS, recomputed on demand and NEVER stored as attributes in any .sysml file.";
        :>> rationale = "Two homes for one fact creates the 'racing the API' failure mode; a pure function of truth is not itself truth, so storing computed values would duplicate truth — text stays canonical and the index stays rebuildable/disposable.";
        :>> consequences = "Pros: git-native review, no sync tax, clean diffs, "
            + "rebuildable index. Cons: raw-file readers see evidence not "
            + "verdicts (mitigated by a CI bot commenting computed deltas on "
            + "PRs); text round-trip without destroying formatting becomes "
            + "the writer's hardest problem.";
    }

    verification d0001Accept : Test { :>> id = "f2c9ceaa-8477-4a9b-a4cd-e94292ace36d"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0001 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0001AcceptR1 : TestResult { :>> id = "f461ad93-25e7-42fc-865f-917b03520bdd"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
