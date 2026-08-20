// ProjectSitting05 — Sitting 05 review. The human reviewed the outstanding queue in the obligations
// deck and instructed "accept all sprints". This covers the 34 sprints that were DUE at that moment --
// the live obligation. The 300-odd sittings grandfathered by D0155 are not re-attested here; they remain
// exactly as D0155 recorded them.
package ProjectSitting05 {
    private import EngineElement::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    verification sitting05Review : Test {
        :>> id = "794a3cbd-c73d-5516-89a3-8a3f1a2a3d4f";
        :>> title = "Sitting 05 review — the 34 sprints due at the 2026-08-20 obligations-deck review (sprints 317-353)";
        :>> createdAt = "2026-08-20";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Per-sitting sprint review (D0049 single human gate; D0073 mechanism; batched per D0019). wweatherholtz, verbatim: 'accept all sprints, and no need to accept issues that have been resolved. auto accept issues by your authority.' Submitted alongside an obligations-deck export in which they also marked the aggregate row '307 sprint(s) awaiting a sitting review' ACCEPT, which is the same instruction expressed through the deck. SCOPE STATED RATHER THAN INFERRED: this covers the 34 sprints reported DUE by `keel sitting-coverage` at that moment - the live obligation, meaning sprints postdating D0155's grandfather line. It does NOT re-attest the sittings D0155 grandfathered; those stand on that Decision's own record. Their instruction was a blanket accept of the sprint records, given without a per-sprint note, and no per-sprint rationale is invented here.";
    }
    part sitting05ReviewR1 : TestResult {
        :>> id = "5e5e6d6d-228e-52a4-8514-7bcc9c15a7ce";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "b7e000d";
        :>> judgedAt = "2026-08-20";
        :>> judgedBy = "wweatherholtz";
    }

    #Covers dependency from sitting05Review to actionableDestinationStory;
    #Covers dependency from sitting05Review to batchCommitDatesStory;
    #Covers dependency from sitting05Review to consoleLoudFailureStory;
    #Covers dependency from sitting05Review to consoleTestLadderStory;
    #Covers dependency from sitting05Review to critPass10Story;
    #Covers dependency from sitting05Review to critPass11Story;
    #Covers dependency from sitting05Review to critPass12Story;
    #Covers dependency from sitting05Review to critPass13Story;
    #Covers dependency from sitting05Review to critPass14Story;
    #Covers dependency from sitting05Review to critPass1Story;
    #Covers dependency from sitting05Review to critPass2Story;
    #Covers dependency from sitting05Review to critPass3Story;
    #Covers dependency from sitting05Review to critPass4Story;
    #Covers dependency from sitting05Review to critPass5Story;
    #Covers dependency from sitting05Review to critPass6Story;
    #Covers dependency from sitting05Review to critPass7Story;
    #Covers dependency from sitting05Review to critPass8Story;
    #Covers dependency from sitting05Review to critPass9Story;
    #Covers dependency from sitting05Review to impossibleEvidenceStory;
    #Covers dependency from sitting05Review to knowledgeDesignStory;
    #Covers dependency from sitting05Review to knowledgeProcessStory;
    #Covers dependency from sitting05Review to machineScopeStory;
    #Covers dependency from sitting05Review to needScopeStory;
    #Covers dependency from sitting05Review to obligationActionableStory;
    #Covers dependency from sitting05Review to obligationDeckStory;
    #Covers dependency from sitting05Review to oneClickStory;
    #Covers dependency from sitting05Review to oversightAdvisoryStory;
    #Covers dependency from sitting05Review to perfDiagStory;
    #Covers dependency from sitting05Review to perfInstrumentStory;
    #Covers dependency from sitting05Review to removalPathStory;
    #Covers dependency from sitting05Review to staleProseStory;
    #Covers dependency from sitting05Review to unitScopeStory;
    #Covers dependency from sitting05Review to viewpointActivationStory;
    #Covers dependency from sitting05Review to vpEnumStory;
}
