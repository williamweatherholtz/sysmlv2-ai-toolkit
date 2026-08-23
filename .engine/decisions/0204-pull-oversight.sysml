// D0204 (status: the `status` field below is the truth) — pullOversight: the governance binds the AI, never the human - no surface schedules the supervisor
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0204Accept event on sign-off.
package Decision0204 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0204 : Decision {
        :>> id = "dc06ec94-bdc8-4698-9b03-a103dd909949";
        :>> title = "pullOversight: the governance binds the AI, never the human - no surface schedules the supervisor";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Their words, 2026-08-23, verbatim: 'none of that. I don't need governance, you do. I don't need a schedule, or due items. highlight blockers and the most critical items, but I'm not here to subscribe to your processes.' Said in response to a report of review queues, batch-acknowledge homework, sampled deep-review asks, and an escalation throttle keyed to their review cadence - machinery from D0200 OPTION A which they had signed a day earlier and rejected on contact with what it asked OF THEM. The process-value panel's own deepest conclusion agrees: verifiable process is the AI's collateral; it exists so the AI's work is checkable, never so the human owes it attention.";
        :>> decision = "OVERSIGHT IS PULL, NOT PUSH. (1) Human-facing surfaces (deck, chat reports) show ONLY decisions that BLOCK work and the most critical items - never anything framed as the human's duty, debt, schedule, or acknowledgment homework; the deck's sitting section, batch-acknowledge solicitation, and deep-review sampling ASK are retired. (2) Nothing gates or throttles the AI's work on the human's attention cadence - the D0200 clause 5 process-change throttle is removed; authority-queue stops listing sitting coverage as awaiting-the-human. (3) The RECORDS all remain: sitting-coverage keeps computing its read/batch split, the override-rate indicator stays on the dashboard, sittings stay reviewable on demand through the console or API whenever the human chooses - pull, never push. (4) SUPERSEDES the obligation semantics of D0200 clauses 3 and 5 and the framing of the per-sitting review as a mandatory human gate (D0049): the human's only inherent gates are the direction decisions that block work and the confirmations they choose to give; everything else about them is optional audit.";
        :>> rationale = "An oversight design that conscripts the supervisor converges to rubber-stamping (the panel proved it happened) or to rejection (which happened next). A design where the AI's records are always auditable but never demand audit keeps the honesty layer at full strength - every number still computes - while returning the human's attention to the only decisions that are genuinely theirs. Detection strength is unchanged: what was removed is the ASK, not the record.";
        :>> consequences = "Deck: sitting section, sampling labels, and batch-ack buttons removed (the API verdict remains for voluntary use); authority-queue: perSittingReview row removed; guards: the process-change oversight throttle removed; CLAUDE.md: the D0049 sitting-gate framing rewritten to optional pull-audit; chat reports: blockers, then critical items, then stop. sitting-coverage, the read/batch split, and sittingOverrideRateIndicator persist untouched as records of the AI's governance.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0204Accept : Test { :>> id = "9740f582-ae9b-4677-82fb-ced4cdd82a8b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Accepted on their explicit direction. Their words, verbatim: 'none of that. I don't need governance, you do. I don't need a schedule, or due items. highlight blockers and the most critical items, but I'm not here to subscribe to your processes' (chat, 2026-08-23). The decision records exactly that direction and its application."; }
    part d0204AcceptR1 : TestResult { :>> id = "a7ec0b59-4f5e-4432-aac9-f526c7c4eb52"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7ef4443"; :>> judgedAt = "2026-08-23"; :>> judgedBy = "wweatherholtz"; }
}
