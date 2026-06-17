package Decision0030 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0030 : Decision {
        :>> id = "f37e8f9a-b0c1-4d2e-dabc-4a5b6c7d8e9f";
        :>> title = "S10 review doc-sync: query.py issues + Issue.relatedTask added to usage.md and tracking template";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 10 (issueLoop) added the `query.py issues` subcommand and `Issue.relatedTask : String[0..1]` field. The usage.md day-to-day table had no entry for issue tracking. The tracking-template.sysml had no Issue authoring example.";
        :>> decision = "Update usage.md day-to-day table: add a 'Track an issue' row documenting the Issue authoring convention and `query.py issues` command; add an Issue example to tracking-template.sysml showing all fields including relatedTask. Recorded as D0030 per doc-sync process.";
        :>> rationale = "After Sprint 10 issueLoop, usage.md had no issue-tracking entry and the tracking template had no Issue example, so the docs had to catch up to the new feature. Accepted under standing sprint review instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "usage.md and tracking-template.sysml now reflect Sprint 10 issueLoop feature. No schema or process semantics change.";
    }

    verification d0030Accept : Test { :>> id = "72b6bf56-5a55-4587-8043-d4144fb166d7"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0030 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0030AcceptR1 : TestResult { :>> id = "6f6730aa-a1a7-4b7b-b094-1aa77a8c8c56"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
