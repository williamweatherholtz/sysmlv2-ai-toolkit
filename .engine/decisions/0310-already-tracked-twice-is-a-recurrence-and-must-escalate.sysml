// D0310 (status: the `status` field below is the truth) — An open backlog item named by two or more sprint retros as the already-tracked home of a finding is a violation unless it is in the next three ready items or blocked on a proposed Decision - guard recurrence-escalates
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0310Accept event on sign-off.
package Decision0310 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0310 : Decision {
        :>> id = "76d22eb7-cee8-4c9f-b381-b87b58c15a46";
        :>> title = "An open backlog item named by two or more sprint retros as the already-tracked home of a finding is a violation unless it is in the next three ready items or blocked on a proposed Decision - guard recurrence-escalates";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human, 2026-09-05: are we not flagging issues in sprint retros? We were. The heredoc defect got an Issue and a backlog item on 2026-09-03, and every retro after it wrote already tracked - which the retro-backlog guard accepts, because D0294's fourth shape checks that a justification NAMES an item, not that the item is moving. The item had been placed below the High-issue resolvers to quiet a priority-inversion warning and stayed there while the defect recurred eight times. Measured on this tree before the guard: retros had named three open items twice as the tracked home - the heredoc item, dcVerificationByAuthority (a fork the human has not been handed) and dcProvenanceAsRefs (superseded since sprint 278, so not open at all - the first draft counted it).";
        :>> decision = "Guard 62, recurrence-escalates, hard: over every sprint's retro gate texts that carry a no-new-item justification, count the open backlog items each names; an item named by two or more is a violation - unless it is among the next three items of the ranked frontier (it is being worked) or carries a #DependsOn edge to a proposed Decision (it waits on the human). Superseded items are not open. The message names the count and the two remedies. The frontier is computed only when a candidate exists, so the common case costs one pass over the retros.";
        :>> rationale = "WHY TWO: one citation is tracking; the second means the finding came back while the item sat, which is the D0047 failure in its purest form - a reminder in an item's clothes. WHY THE FRONT AND THE FORK ARE EXEMPT: the guard exists to force one of two honest moves - work it next, or record why it waits - and both are computable; anything else is deferral by silence. WHY A VIOLATION: the retro-backlog guard's warning-free pass on the phrase already tracked was itself the mechanism of the failure, and a control that catches the recurrence of a control's blind spot must not share it. WHAT IT COSTS: pulling dcVerificationByAuthority to the front over eight High-issue resolvers, which the priority-inversion guard now warns about - deliberately, and said here as D0052 asks.";
        :>> consequences = "dcVerificationByAuthority moves to the front of the frontier and is the next item worked - its DoD is the fork Decision the human must be handed. issue372 records the process defect and resolves with this guard. The retro-backlog guard keeps its form check; this guard holds the substance. Eight priority-inversion warnings on this tree are the guard's own doing and are deliberate.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0310Accept : Test { :>> id = "9a60eead-b308-4e75-b320-17af5bba0018"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0310AcceptR1 : TestResult { :>> id = "28f571e7-7f43-4e83-9a3c-20d227af68c2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6702c38"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
