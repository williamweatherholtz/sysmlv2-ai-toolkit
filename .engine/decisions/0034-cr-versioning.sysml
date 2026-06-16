package Decision0034 {
    private import EngineWork::*;

    part d0034 : Decision {
        :>> id = "be499efc-3a4b-4c5d-def6-7a8b9c0d1e2f";
        :>> title = "crVersioning: CR workflow versioning via git history + judgedAgainst SHA; no separate version field";
        :>> status = DecisionStatus::accepted;
        :>> context = "The CR workflow itself can be modified by a CR (it IS a workflow/process definition). The question: when the CR workflow is updated, how do old CRs remain coherent, and how does a new CR know which workflow version it operates under? Three options: (1) add a `workflowVersion` field to each CR recording the version in force; (2) rely on git history — the Decision file's `judgedAgainst` SHA anchors the CR to the workflow as it existed at that commit; (3) require a frozen CR workflow that only changes out-of-band (current bootstrap rule §4).";
        :>> decisionText = "Adopted: option (2) — git history is the version record. The `judgedAgainst` SHA on the CR's TestResult anchors the CR to the state of the workflow at the commit it was evaluated against. No `workflowVersion` field is added to the schema. The bootstrap freeze (§4) is retained for the CR workflow during bootstrap; post-bootstrap, CRs that change the CR workflow record the change in their own `decisionText`. An auditor can reconstruct the governing workflow at any point by checking out the `judgedAgainst` commit. This is consistent with D0013 (git is the temporal model). Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "No schema change. CRs from before a workflow revision remain valid under the workflow version they were processed under — recoverable from git. Engineers following the CR process should note the workflow version (commit) in their `decisionText` when the workflow itself is under change. EngineWork::Decision schema unchanged.";
    }
}
