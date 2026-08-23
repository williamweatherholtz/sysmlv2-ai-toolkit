// D0200 (status: the `status` field below is the truth) — D0200: redesign the sitting gate - sampling with judgeable cards, or status quo with honest counting
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0200Accept event on sign-off.
package Decision0200 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0200 : Decision {
        :>> id = "bfb34c6a-079a-4e62-ae3a-9339695378b8";
        :>> title = "D0200: redesign the sitting gate - sampling with judgeable cards, or status quo with honest counting";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The process-value panel (nine lenses, 2026-08-22, docs/reviews/process-value-panel-2026-08-22.md) found the single mandatory human gate measurably rubber-stamped, and the numbers are verified: 62 percent of covered sittings were signed in two batch chat utterances; the deck card renders a title plus the string unreviewed - nothing to judge; the observed override rate is 0 percent, which the HITL literature treats as the alarm; the 279-sprint queue has been escalated 65 days with no behavioral consequence (and issue227 records that even that number disagrees between two views). Four of nine lenses independently recommended the same redesign family. The record of what happened is honest; the COUNT is not (a batch acknowledgment scores identically to a read review), and the design gives the human nothing to exercise judgment WITH.";
        :>> decision = "PROPOSED, two options costed - accepting means choosing ONE. OPTION A (the full redesign, panel-recommended): (1) sitting coverage splits into READ-REVIEWED vs BATCH-ACKNOWLEDGED - two numbers, never one (the EXAMINED/EXERCISED discipline applied to oversight itself); (2) the deck sitting card renders the sprint's COMPUTED DELTA (items added, decisions touched, guards changed, test outcomes) so a tap can be a judgment; (3) per sitting, the human deeply reviews N randomly-sampled plus M risk-flagged sprints and batch-acknowledges the rest explicitly; (4) an override-rate indicator (0 percent sustained = the alarm); (5) escalation gains teeth - past the threshold, new CHANGE-route work queues until the debt shrinks. COST: the human's per-sitting effort rises from one utterance to N+M real reads; the AI must compute risk flags honestly; throttling can stall delivery when the human is away. OPTION B (status quo with honest counting): only clause 1 (the split count) and clause 4 (the indicator) - the gesture economics stay exactly as the human chose them, but the numbers stop conflating trust-the-process with examined-the-work. COST: the gate remains a trust attestation in substance; the panel's complacency findings stand recorded rather than addressed. DRAFTER RECOMMENDS A with N=2, M=1: it is the panel's convergent design, it reuses existing idioms (indicators, typed verdicts, computed cards), and clause 5 is the only mechanism that makes the human's absence CHANGE anything - the meaningful-human-control criterion.";
        :>> rationale = "The gate exists to be the one place a human exercises judgment over autonomous work (D0049). The panel showed it cannot currently perform that function: no information on the card, no distinct verdict type, no measured override rate, no consequence to neglect. Either redesign it so judgment is possible and measured, or keep the economics and stop counting acknowledgment as examination - both are honest; silently continuing is not.";
        :>> consequences = "OPTION A: deck.rs card rendering, a batchAcknowledged verdict in the sitting write path, sitting-coverage split counts, one indicator, one throttle check at the CHANGE route; issue228 resolves. OPTION B: the split count and indicator only; issue228 resolves as partially-mooted with the residual recorded. EITHER WAY the two-view disagreement (issue227) is fixed separately by dcSittingViewParity. REJECTED-AS-WRITTEN leaves the panel's finding standing recorded against an unchanged gate.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0200Accept : Test { :>> id = "b7454b89-4795-40ed-927d-3357c4b824c9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "OPTION A chosen (the full redesign: read-vs-batch split counts, judgeable delta cards, N+M sampling, override-rate indicator, escalation teeth). Deck tap: uid bfb34c6a, verdict accept, note OPTION A, At 2026-08-23T01:15:30Z, deck HEAD 55d1381 - a single deliberate option tap."; }
    part d0200AcceptR1 : TestResult { :>> id = "beb6cd0b-7f39-423e-b05c-524e3d5c5c9e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71f6453"; :>> judgedAt = "2026-08-23"; :>> judgedBy = "wweatherholtz"; }
}
