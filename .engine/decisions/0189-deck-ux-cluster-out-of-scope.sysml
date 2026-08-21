// D0189 (PROPOSED — NOT YET ACCEPTED) — D0189: the deck-UX polish cluster is out of scope until the human reopens it
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
        :>> status = DecisionStatus::proposed;
        :>> context = "Twelve of the model's twelve open issues (issue152-165, issue188, issue190) and the two top-ranked ready backlog items (dcDeckWorkSurface, dcLegibleOutput) are one cluster: polish of the mobile review deck and its click-to-act pathways. Both backlog items state UNSCOPED DELIBERATELY - their direction is needed. On 2026-08-21 the human gave that direction in chat, of exactly this queue: 'these are super irrelevant. proceed until there is a real decision for me, not just issues with no direction.' Separately, the deck ALREADY WORKS for its load-bearing purpose: this same day they dispositioned 19 findings and accepted 36 sitting reviews through it, and the MCP inbox transport replaced the artifact live-doc sync whose defects issue188/issue190 describe - those two defects no longer have a surface to occur on.";
        :>> decision = "The deck-UX polish cluster is closed as out of scope: this Decision supersedes issues 152, 154, 155, 157, 158, 159, 161, 162, 163, 165 (won't act - the polish is not worth the human's or the engine's attention), moots issues 188 and 190 (their defect surface, the artifact live-doc sync, was replaced by the MCP inbox transport), and supersedes backlog items dcDeckWorkSurface and dcLegibleOutput plus the remaining undelivered deck-cluster items whose DoD waits on a human demo the human has declined to perform. The deck remains in service exactly as it is. Reopening is a new Decision citing this one.";
        :>> rationale = "Invariant 4: 'we won't do X' is a first-class Decision that supersedes the need - the alternative is twelve permanently-open issues cluttering every orient and a frontier whose top two items can never start. The human's words are direction, but scope closure is a Decision-grade act (D0052 pause rule), so it is recorded PROPOSED and queued for their signature rather than inferred silently: their 'super irrelevant' named the four deck cards in front of them, and extending it to the whole cluster is a judgment that needs their sign-off, not mine.";
        :>> consequences = "IF ACCEPTED: the twelve issues resolve as superseded (computed, no prose RESOLVED notes), the ready frontier clears of items that cannot start, and orient stops reporting a queue the human has said they will not work. The deck keeps operating unchanged. COST: real defects are being deliberately shelved - issue161 (regeneration can overwrite unsubmitted marks) remains live behavior; anyone using the deck heavily will re-hit it, and reopening requires a new Decision. IF REJECTED: the issues stay open and the human names which parts they actually want built.";
    }
}
