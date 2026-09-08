// D0380 (status: the `status` field below is the truth) — emptyQueueSilence: do not republish when the queue is empty - the last page stands until the pending set gains a member
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0380Accept event on sign-off.
package Decision0380 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0380 : Decision {
        :>> id = "c8edd713-de57-4067-83ab-d8125a5a8b99";
        :>> title = "emptyQueueSilence: do not republish when the queue is empty - the last page stands until the pending set gains a member";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0376 proposed that an empty authority queue produce a republished RECEIPT page with no ask. On 2026-09-08 the human answered that ask on the brief itself with the reverse: '[x] Reverse: do not republish when the queue is empty'. D0376 is rejected on those words and this Decision records the course they chose. The decision-surfacing process (D0269, D0359) republishes the page when the pending set changes; the question was what a change TO EMPTY publishes. NOT A FORK: the human chose between the two courses on the page; this records their choice.";
        :>> decision = "When `keel show authority-queue` carries no decisionAcceptance row, the decision-surfacing process publishes nothing: the last published page stands, `.keel/decision-page.toml` keeps the ids of the last page that had an ask, and the turn says nothing about decisions. The next publish happens when the pending set gains a member. The brief contract in scripts/check_templates.py --brief keeps requiring an ask block and response options unconditionally - there is no empty-queue page shape.";
        :>> rationale = "The human's word, given on the page in answer to the ask. The receipt shape would have spent a publish, a check run and a line in the turn on telling them what they had just done themselves; the cost of the chosen course is that the last page names answered asks until a new one arrives, and the page's own provenance date lets the reader see that. This is a process-change (it fixes what decision-surfacing publishes) and the human's answer is its acceptance.";
        :>> consequences = "scripts/check_templates.py --brief is unchanged: no empty-queue branch is added. The decision-surfacing skill's step 2 table gains the row: pending set became empty -> nothing published, the toml is left as it was. scripts/exec_brief gains no receipt builder. D0376 stands rejected with the human's words as its rationale.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0380Accept : Test { :>> id = "7596ad65-9ff1-40af-9949-7ac4f04a710d"; :>> method = VerificationMethod::confirmation; :>> procedureText = "recorded from chat - their words, verbatim: “[x] Reverse: do not republish when the queue is empty (answers: d0376)”"; }
    part d0380AcceptR1 : TestResult { :>> id = "d5b50f37-3aff-4379-a2df-35d1ef0d12e6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "748746a"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
