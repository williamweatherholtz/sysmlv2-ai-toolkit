package Decision0028 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0028 : Decision {
        :>> id = "ef9a0b1c-d2e3-4f4a-cd5e-6f7a8b9c0d1e";
        :>> title = "S9 review doc-sync: add append-result and add-task to usage.md day-to-day table";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 9 (writeApi) added sysmlv2 append-result and sysmlv2 add-task subcommands as the sanctioned write path. usage.md day-to-day table documented the old direct-edit approach for 'Mark done' and had no entry for 'Add a new task'. The table needed updating to reflect the new write API.";
        :>> decision = "Update usage.md day-to-day table: rewrite the 'Mark done' row to lead with the sysmlv2 append-result command + flags (manual fallback after), and add a new 'Add a new task' row documenting sysmlv2 add-task command + flags. Recorded as D0028 per doc-sync process.";
        :>> rationale = "Sprint 9 made append-result/add-task the sanctioned write path, so the day-to-day table had to reflect it instead of the old direct-edit approach. Accepted under standing sprint review instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "usage.md day-to-day table now reflects Sprint 9 write API. No schema or process semantics change.";
    }

    verification d0028Accept : Test { :>> id = "8e8afb94-44b3-4487-8cfb-b5735b841157"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0028 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0028AcceptR1 : TestResult { :>> id = "65a6f620-a270-4ae0-97cd-2099985e1c8a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
