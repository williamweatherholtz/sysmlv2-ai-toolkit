// D0376 (status: the `status` field below is the truth) — emptyQueueBrief: when nothing awaits the human, the republished brief is a receipt with no ask, and the contract accepts that shape
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0376Accept event on sign-off.
package Decision0376 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: Read 2026-09-08 in this tree: scripts/check_templates.py check_brief (clauses 1-6, the ask and options required unconditionally); .engine/processes/asi-templates.sysml and the decision-surfacing skill (no empty state named); `keel show authority-queue .` returning 167 findingDisposition rows and zero decisionAcceptance rows after d0375 was accepted; .keel/decision-page.toml ids = ['d0375'] against that empty set.
    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0376 : Decision {
        :>> id = "f01945d1-4207-494b-8812-30159ad64cd6";
        :>> title = "emptyQueueBrief: when nothing awaits the human, the republished brief is a receipt with no ask, and the contract accepts that shape";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::rejected;
        :>> context = "D0359 made the published page the channel for what awaits the human: the page is republished when the pending set CHANGES and the turn says one line. D0355 wrote the brief contract that scripts/check_templates.py --brief enforces: a headline with a verb, an ask block, response options that declare what they decide, two logic figures per ask, a provenance strip. On 2026-09-08 the human answered the page's one remaining ask and the pending set became EMPTY - and the process has no shape for that state. A page with no ask fails the contract ('a brief the reader cannot answer is a report'), so the only page that passes is one that still carries an answered ask, which is the repeated-prose failure D0359 removed, now as a stale page. The alternative - not republishing - leaves the page saying a decision waits when none does. Both leave the human reading something false about their own queue.";
        :>> decision = "When `keel show authority-queue` carries no decisionAcceptance row, the republished brief is a RECEIPT: its headline states that nothing waits for the human's word; its body names what was last answered, on what date and in whose words, and what that answer set in motion (built, or on the frontier); it carries the same provenance strip and copy-for-AI control; it carries NO ask block and NO response options, and the contract accepts the shape when the page declares `data-queue='empty'` on its digest title and carries a `data-digest='answered'` element naming the last answered record. Every other clause of the contract - verb in the headline, at least two figures with claims, no record id in reader text, provenance with a date, both themes - stands unchanged for the empty shape. NOT A FORK: this chooses one course - the page always says what is true about the queue - and the shape of a receipt is rendering, not judgment.";
        :>> rationale = "The page exists so the human never has to ask where their queue stands; a page that cannot represent 'empty' fails that at exactly the moment they finished the work it asked of them. The contract's demand for an ask is right for a page that HAS one - a report the reader cannot answer wastes their attention - and wrong when there is nothing to answer, where a manufactured ask (confirm my reading, reopen something) would be the gatekeeping the human named on 2026-09-08. The receipt shape keeps the contract's purpose (answer first, no pointers, figures that claim, provenance) and drops only the clause that presupposes an ask. This is a process-change because it alters what the decision-surfacing process publishes and what its contract checker accepts, so under D0337 it waits for the human's word rather than accepting itself; no plan the human signed names it, so under D0375 option C it is exactly the unplanned change that asks.";
        :>> consequences = "scripts/check_templates.py --brief gains the empty-queue branch (ask and options not required when data-queue is empty and an answered digest is present; every other check runs); scripts/exec_brief gains a receipt builder that refuses unless facts.json says zero pending; the decision-surfacing skill's step 3 names the shape; .keel/decision-page.toml records ids = [] for a receipt publish. Nothing changes for a page that has an ask.";
    }

    // rejection judgment (D0121 review-queue; D0106 — human-judged, not fabricated). Recorded by direct edit
    // mirroring write::reject_decision because no CLI reject path exists (issue414); the human's words are the receipt (D0289).
    verification d0376Reject : Test { :>> id = "eb716716-b52a-464e-bbe0-0a68f28ada0b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "REJECTED: recorded from chat - their words, verbatim: “[x] Reverse: do not republish when the queue is empty (answers: d0376)” - the human chose the reverse course, recorded as d0380."; }
    part d0376RejectR1 : TestResult { :>> id = "40d016d6-bd71-40e4-b681-11d1243cf5a6"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "748746a"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
