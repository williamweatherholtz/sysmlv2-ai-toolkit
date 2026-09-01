// D0274 (status: the `status` field below is the truth) — A proposed Decision not covered by a published surface BLOCKS the turn
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0274Accept event on sign-off.
package Decision0274 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0274 : Decision {
        :>> id = "a86a06e0-022c-40ab-bf84-d1c0f8191ffa";
        :>> title = "A proposed Decision not covered by a published surface BLOCKS the turn";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0269 established that pending judgments lead the response with a durable link. I wrote into that Decision myself that no control can read my prose, and then broke the rule on the very next turn - which is how the human noticed it was unenforced. Their question was direct: is that process not enabled, not forced when a decision is needed. Enabled, yes; forced, no. On the fork they chose BLOCK over advise-only and over leaving it to my attention.";
        :>> decision = "keel hook stop REFUSES to end the turn while a Decision recorded as proposed is not covered by a recorded decision surface. The check is on the MODEL, never on my prose: a proposed Decision is an authored fact, and a surface is an authored fact, so the gate compares two things it can actually read. Satisfying it is a FILE WRITE that works offline - recording the surface, not publishing it - so there is no state in which the turn is trapped by a network.";
        :>> rationale = "WHY BLOCKING AND NOT ADVISING, on the criterion the human was given: whether they would rather I stall than end a turn with an unasked question. They chose the stall. The advisory option is what already failed - I had the rule, in a Decision I authored, and ended the turn anyway; D0047 says manual vigilance is not a control and an advisory aimed at me is exactly that. THE LOCKOUT OBJECTION, which is the real cost and is answered by construction rather than by assurance: a blocking gate I cannot satisfy would trap the turn, so the satisfying act is deliberately the local one. Recording the surface writes a file; publishing it is a separate step the gate does not require. WHAT THIS CANNOT DO, stated so it is not mistaken for complete: it cannot catch a decision posed ONLY in prose, because no control can read prose - the same limit D0269 named. It catches the case where a Decision exists in the model and I failed to surface it, which is the case that actually occurred. A question I never record at all remains uncovered, and that residual is why the response contract's verify-before-asserting clause still rests on discipline.";
        :>> consequences = "The turn boundary gains a blocking condition on the enforcement surface, so this Decision is marked SafetyChange and the hook and guard changes ride its keystone lock (D0209 clause 2). The signal is currently clean: there are ZERO Decisions recorded as proposed, so the gate starts from a true state rather than from a backlog it would immediately fire on. ARMING IS REQUIRED, not optional (D0253): the control must be observed to FAIL - a fixture with a proposed Decision and no surface must block, and the same fixture with the surface recorded must pass - because a gate that has only ever been seen green is a claim. A NEW FAILURE MODE IS ACCEPTED: a proposed Decision recorded late in a turn now forces the surface to be recorded before the turn can end, which is the intended behaviour and will occasionally feel like friction at exactly the moment the friction is the point.";
    }
    // NOT auto-accepted (D0207 covers NON-FORK decisions only). This was a three-way FORK put to the
    // human and they answered it, choosing the option I recommended. It is still their judgment, not
    // a standing consent, because a control that can stall my turn is theirs to authorise.
    verification d0274Accept : Test {
        :>> id = "3e07b6c1-58d4-4a92-b0e3-9c41f28a05d7";
        :>> title = "Blocking the turn on an unsurfaced pending decision (human adjudication)";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz, 2026-09-01, via the Unattended Distribution decision page returned as a Copy-for-AI digest. They selected, verbatim, 'Block: a proposed Decision not covered by the published surface stops the turn' over advise-only and over leaving it to my attention, on the criterion the page put to them: whether they would rather I stall than end a turn with an unasked question. WHAT THIS AUTHORISES SPECIFICALLY, because a blocking gate is not a small grant: keel hook stop may REFUSE to end my turn. The refusal condition is on the MODEL - a Decision recorded as proposed with no recorded surface covering it - never on my prose, and the satisfying act is a local file write so no network state can trap the turn. WHAT IT DOES NOT AUTHORISE: it does not gate on publishing, and it cannot catch a question posed only in conversation, which stays uncovered and is stated as such in the consequences. Their verbatim selection is quoted in d0274Attest1.";
    }

    verification d0274Attest1 : Test {
        :>> id = "4f18c7d2-69e5-4b03-a1f4-0d52639b16e8";
        :>> title = "Quote receipt for d0274Accept (D0198)";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Their selection, verbatim from the returned digest. Tab 'Force the surfacing?', question 'Should an unsurfaced pending decision BLOCK the turn?': [x] 'Block: a proposed Decision not covered by the published surface stops the turn - RECOMMENDED'. The unselected options were 'Advise only: tell me at the turn boundary, never block' and 'Neither - rely on me noticing, as now'. No note was entered on this tab. The question this answers was theirs, asked in their own words on 2026-09-01: is that process not enabled, not forced when a decision is needed.";
    }

    part d0274AcceptR1 : TestResult { :>> id = "5a29d8e3-70f6-4c14-b2a5-1e63740c27f9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ac8bdf6"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "wweatherholtz"; }
    part d0274Attest1R1 : TestResult { :>> id = "6b3ae9f4-81a7-4d25-c3b6-2f74851d38a0"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ac8bdf6"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }
}
