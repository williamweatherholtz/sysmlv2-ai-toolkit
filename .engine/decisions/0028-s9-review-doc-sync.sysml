package Decision0028 {
    private import EngineWork::*;

    part d0028 : Decision {
        :>> id = "ef9a0b1c-d2e3-4f4a-cd5e-6f7a8b9c0d1e";
        :>> title = "S9 review doc-sync: add append-result and add-task to usage.md day-to-day table";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 9 (writeApi) added sysmlv2 append-result and sysmlv2 add-task subcommands as the sanctioned write path. usage.md day-to-day table documented the old direct-edit approach for 'Mark done' and had no entry for 'Add a new task'. The table needed updating to reflect the new write API.";
        :>> decision = "Update usage.md day-to-day table: rewrite the 'Mark done' row to lead with the sysmlv2 append-result command + flags (manual fallback after), and add a new 'Add a new task' row documenting sysmlv2 add-task command + flags. Recorded as D0028 per doc-sync process.";
        :>> rationale = "Sprint 9 made append-result/add-task the sanctioned write path, so the day-to-day table had to reflect it instead of the old direct-edit approach. Accepted under standing sprint review instruction (wweatherholtz, 2026-06-15).";
        :>> decisionText = "Update usage.md day-to-day table: (1) 'Mark done' row updated to lead with sysmlv2 append-result command + flag syntax, followed by the manual fallback; (2) Add new 'Add a new task' row documenting sysmlv2 add-task command + flags. Record as D0028 per doc-sync process. Accepted under standing sprint review instruction (wweatherholtz, 2026-06-15).";
        :>> consequences = "usage.md day-to-day table now reflects Sprint 9 write API. No schema or process semantics change.";
    }
}
