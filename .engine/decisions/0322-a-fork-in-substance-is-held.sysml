// D0322 (status: the `status` field below is the truth) — forkShapeIsDetected: a Decision that weighs alternatives in prose without the OPTION marker is held proposed, not auto-accepted
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0322Accept event on sign-off.
package Decision0322 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0322 : Decision {
        :>> id = "252bc5d1-b7c1-49d0-978e-28c566d039ce";
        :>> title = "forkShapeIsDetected: a Decision that weighs alternatives in prose without the OPTION marker is held proposed, not auto-accepted";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "stpa-self run 1 (D0313) recorded UCA-R1 and issue373: record-time standing consent (D0291) decides fork-or-not by ONE lexical shape - deck::fork_options reads the uppercase-letter-plus-parenthesised-label enumeration - so a Decision that weighs alternatives in prose without that marker auto-accepts on the spot, and a direction the human owns is recorded as accepted by an AI (EHZ3). The author controls the lexicon that decides whether the human is asked. Measured before the threshold was set, the D0102 way: over 109 auto-accepted Decisions, four carry two fork signals in their decision text - D0207, D0269 and D0288 describe the surfacing process itself, and D0292 recommended a routing with per-item choices and executed it under consent, which is exactly the class - and none carries three.";
        :>> decision = "keel record decision reads the decision text for fork signals - alternative, either, or-we-could, option, versus, trade-off, a lettered enumeration, recommend - as whole words, and when two distinct signals are present and the text does not carry the marker, standing consent does not apply: the Decision is recorded proposed with a message naming the words that weigh and the two ways forward - write it as a fork with the marker and a COST per course, or state NOT A FORK: <why it chooses one course> in the text and re-record - while a human may still accept it with their quoted word (D0289). NOT A FORK: this Decision chooses that course; the words above are its subject, not its alternatives.";
        :>> rationale = "Why two signals: one is how a decision names the alternative it rejected - a decision by definition chose - and the measured corpus bears it out (105 of 109 auto-accepted Decisions carry at most one); two is the weighing shape, and the four it catches split three-to-one between meta false positives the author answers in five words and the D0292 class the control exists for. Why hold rather than refuse: the record is true either way - the Decision exists and is proposed - and refusing to write it would lose the author's text; what is refused is the acceptance nobody gave. Why the escape is a stated sentence in the record: an author who says NOT A FORK has made a visible claim a reader can dispute, which is the D0192 shape for every other assertion-based control here. Arming (D0298): unit tests on the signal reader (the D0292 shape held; one signal in passing not held; whole-word only; the escape) and keel-cli/tests/disguised_fork_stays_proposed.rs on a real scaffold with one decider - the disguised fork is recorded proposed with no acceptance result, the genuine decision auto-accepts.";
        :>> consequences = "Every future auto-acceptance passes through the reader; a held Decision is surfaced like any fork (decision-surfacing) and accepted on the human's words. issue373 is resolved by dcForkShapeIsDetected; CC-R1 (ccRecordForkShape, engine-ucas.sysml) now binds to a control and its edge moves from the Issue to this Decision's mechanism at the next stpa-self run. STATED RESIDUAL: a fork written with none of these words - two courses described without naming them as such - is not caught; the reader covers the words that weigh, not every way of weighing.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0322Accept : Test { :>> id = "07c9123b-d12b-4c22-83d6-b61d8eb73049"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0322AcceptR1 : TestResult { :>> id = "57f39c6b-1e70-41db-9889-8b2a79b6df3e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "50175b6"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
