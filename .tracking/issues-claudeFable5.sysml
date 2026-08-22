// Per-actor issues records for claudeFable5 (issue210/srDcPerActorWriteTargets): new records
// route here so concurrent contributors never conflict at a shared append anchor.
package IssuesClaudeFable5 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part issue214 : Issue {
        :>> id = "a6db1f9a-75cd-4d20-8ff8-c0030e6ca1fa";
        :>> title = "per-actor routing probe A";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "issue210's live verification probe, kept as the record of its own routing: recorded through keel record issue AFTER the per-actor change and required to land in issues-claudeFable5.sysml with the next globally-allocated number - which it did, making this Issue both the test and the evidence. Resolved by dcPerActorAppendFiles alongside issue210 itself.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcPerActorAppendFiles to issue214;
}
