// D0156 — PROPOSED, awaiting wweatherholtz. N-8 is owned by the human and states scope D0115 removed;
// only its owner can narrow it (D0108). This Decision exists so the resolving work is BLOCKED rather
// than ranked as ready for an actor not permitted to do it.
// #ProspectiveChange (D0070): changes one Need's statement; nothing already derived from it is invalidated.
package Decision0156 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0156 : Decision {
        :>> id = "a71f3e08-4c92-4b65-8d17-903fe2cb5471";
        :>> title = "D0156: narrow N-8's statement to the single-user scope D0115 left standing";
        :>> createdAt = "2026-08-17";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "issue132, open since 2026-08-15. nViewerPersistedViewpoints (N-8) states that viewpoints and view state must be 'shareable across users' with 'per-user view state'. D0115 revised the viewer scope to single-user and unauthenticated, routing N-7's saveable-viewpoint concern into N-8 while dropping auth, multi-user and per-user. So part of N-8's own statement describes scope an accepted Decision removed, and a reader decomposing N-8 today would derive requirements for work that was cut — the exact trap sprint 294 warned about when it excluded superseded Needs from the gap set. srViewerPersistedViewState decomposes only the surviving half and says so in its rationale, which contains the damage without repairing it. THE REASON THIS IS A PROPOSAL AND NOT A FIX: N-8 is owned by wweatherholtz, and D0108 permits a non-owner to ADD or SUPERSEDE but never to overwrite an owner's field. An AI narrowing a human's Need statement is exactly the edit that rule exists to prevent, however obviously correct the narrowing looks.";
        :>> decision = "PROPOSED, two ways to take it — the owner picks: (A) narrow N-8's statement in place to the single-user, unauthenticated scope D0115 left standing, dropping the 'shareable across users' and 'per-user view state' clauses; or (B) supersede N-8 with a new Need that states the surviving concern, leaving N-8 in history with its supersession recorded. Either resolves issue132. On acceptance, the owner may make the edit themselves or authorize it explicitly in the acceptance, which is the same ownership exemption a co-committed migration carries (D0067).";
        :>> rationale = "WHY A DECISION AND NOT JUST A BACKLOG ITEM: the resolving work must be visible without being schedulable. The frontier is AUTO-FOLLOWED (D0052), so a ready item is an item the next actor starts — and this one cannot be started by me at all. A `#DependsOn` edge to a PROPOSED Decision is the existing, exact mechanism for that state (blocked-on-acceptance, not superseded: WAITING rather than RETIRED), so `dcNeed8ScopeNarrowing` sits off the ready list and appears in the human's own queue instead. WHY BOTH OPTIONS ARE OFFERED rather than my recommending one: (A) is less ceremony and keeps one Need per concern; (B) preserves the exact words a reader of D0115-era history would have seen. That trade is about how much of the record the owner wants to keep visible, which is theirs to weigh and not a technical question I can settle for them.";
        :>> consequences = "Until accepted, issue132 stays open and `dcNeed8ScopeNarrowing` stays blocked — deliberately, since the alternative is an item ranked ready that its only permitted author cannot be. Once accepted, N-8 states only surviving scope and a future reader decomposing it cannot derive requirements for cut work. If REJECTED, N-8's statement stands as written and issue132 should be closed by a superseding Decision saying the wider scope is intended after all — which would in turn reopen whether D0115's single-user revision still holds.";
    }

    #DependsOn dependency from dcNeed8ScopeNarrowing to d0156;
}
