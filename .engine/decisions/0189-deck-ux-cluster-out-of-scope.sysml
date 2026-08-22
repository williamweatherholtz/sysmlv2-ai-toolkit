// D0189 (status: the `status` field below is the truth) — D0189: the deck-UX polish cluster is out of scope until the human reopens it
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0189Accept event on sign-off.
package Decision0189 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0189 : Decision {
        :>> id = "4f4fbff3-c67a-498d-a7de-78cd9de7fb82";
        :>> title = "D0189: the deck-UX polish cluster is out of scope until the human reopens it";
        :>> createdAt = "2026-08-21";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Twelve of the model's twelve open issues (issue152-165, issue188, issue190) and the two top-ranked ready backlog items (dcDeckWorkSurface, dcLegibleOutput) are one cluster: polish of the mobile review deck and its click-to-act pathways. Both backlog items state UNSCOPED DELIBERATELY - their direction is needed. On 2026-08-21 the human gave that direction in chat, of exactly this queue: 'these are super irrelevant. proceed until there is a real decision for me, not just issues with no direction.' Separately, the deck ALREADY WORKS for its load-bearing purpose: this same day they dispositioned 19 findings and accepted 36 sitting reviews through it, and the MCP inbox transport replaced the artifact live-doc sync whose defects issue188/issue190 describe - those two defects no longer have a surface to occur on.";
        :>> decision = "The deck-UX polish cluster is closed as out of scope: this Decision supersedes issues 152, 154, 155, 157, 158, 159, 161, 162, 163, 165 (won't act - the polish is not worth the human's or the engine's attention), moots issues 188 and 190 (their defect surface, the artifact live-doc sync, was replaced by the MCP inbox transport), and supersedes backlog items dcDeckWorkSurface and dcLegibleOutput plus the remaining undelivered deck-cluster items whose DoD waits on a human demo the human has declined to perform. The deck remains in service exactly as it is. Reopening is a new Decision citing this one.";
        :>> rationale = "Invariant 4: 'we won't do X' is a first-class Decision that supersedes the need - the alternative is twelve permanently-open issues cluttering every orient and a frontier whose top two items can never start. The human's words are direction, but scope closure is a Decision-grade act (D0052 pause rule), so it is recorded PROPOSED and queued for their signature rather than inferred silently: their 'super irrelevant' named the four deck cards in front of them, and extending it to the whole cluster is a judgment that needs their sign-off, not mine.";
        :>> consequences = "IF ACCEPTED: the twelve issues resolve as superseded (computed, no prose RESOLVED notes), the ready frontier clears of items that cannot start, and orient stops reporting a queue the human has said they will not work. The deck keeps operating unchanged. COST: real defects are being deliberately shelved - issue161 (regeneration can overwrite unsubmitted marks) remains live behavior; anyone using the deck heavily will re-hit it, and reopening requires a new Decision. IF REJECTED: the issues stay open and the human names which parts they actually want built.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0189Accept : Test { :>> id = "4b691b3c-e61d-4665-b051-0f61472f6126"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Their words, verbatim: 'approved. keep the momentum going' - said in direct response to the report presenting the four queued Decisions (D0189, D0190, D0192, D0193) and asking for signatures. Covers this Decision as one of that presented queue. D0192 is NOT covered by the same word: accepting it requires choosing between its two costed options, which a bare approval does not do - it stays proposed until they name A or B."; }
    part d0189AcceptR1 : TestResult { :>> id = "606e190b-abb4-4823-a284-193d6d925742"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ef26b9"; :>> judgedAt = "2026-08-21"; :>> judgedBy = "wweatherholtz"; }
    // APPLICATION (sprint 406, on the recorded acceptance): the scope closure this Decision decrees.
    // The twelve issues resolve as mooted-by-decision (computed - no prose RESOLVED notes); the
    // thirteen undeliverable-or-declined deck-cluster items leave the ready frontier by supersession.
    #Resolves dependency from d0189 to issue152;
    #Resolves dependency from d0189 to issue154;
    #Resolves dependency from d0189 to issue155;
    #Resolves dependency from d0189 to issue157;
    #Resolves dependency from d0189 to issue158;
    #Resolves dependency from d0189 to issue159;
    #Resolves dependency from d0189 to issue161;
    #Resolves dependency from d0189 to issue162;
    #Resolves dependency from d0189 to issue163;
    #Resolves dependency from d0189 to issue165;
    #Resolves dependency from d0189 to issue188;
    #Resolves dependency from d0189 to issue190;
    #Supersede dependency from d0189 to dcDeckWorkSurface;
    #Supersede dependency from d0189 to dcLegibleOutput;
    #Supersede dependency from d0189 to dcLiveDoc;
    #Supersede dependency from d0189 to dcObligationDeck;
    #Supersede dependency from d0189 to dcOneClickToAct;
    #Supersede dependency from d0189 to dcActionableDestination;
    #Supersede dependency from d0189 to dcConsoleSelfReport;
    #Supersede dependency from d0189 to dcDeckButtonsLive;
    #Supersede dependency from d0189 to dcDeckSyncAddressable;
    #Supersede dependency from d0189 to storyActionableDestination;
    #Supersede dependency from d0189 to storyOneClick;
    #Supersede dependency from d0189 to storyObligationDeck;
    #Supersede dependency from d0189 to storyLiveDoc;

}
