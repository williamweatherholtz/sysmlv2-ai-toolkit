// D0234 (PROPOSED — NOT YET ACCEPTED) — everyReplyReceipted: a terminal reply on the channel stamps its idempotency key, and issue creation serialises
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0234Accept event on sign-off.
package Decision0234 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: Verified live before fixing: the open keel-standing issue list (exactly one, #29), the workflow-run history (decision-record schedule runs succeeding hourly with an unchanged head SHA while the thread kept updating at run-completion times), the thread's comments (20+ identical nags all quoting comment 5405430593), and #28 (closed by the human at 2026-08-25T04:56:46Z, created 2s before #29 - the issue262 double-creation, whose label-lookup fix is already on main).
    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0234 : Decision {
        :>> id = "c857f4c0-bfb8-421e-89ab-b1e8b3f0229a";
        :>> title = "everyReplyReceipted: a terminal reply on the channel stamps its idempotency key, and issue creation serialises";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "Reported by the human as CI email spam. The standing override thread (#29) accumulated 20+ IDENTICAL 'Didn't parse' nags, one per scheduled decision-record run, each emailing the repo owner. Measured live: the sweeper selects the latest decider comment with no `receipt-for-comment: <id>` in any comment body - that marker is its ONLY dedup signal - but three of the recorder's terminal replies (didn't-parse, not-a-declared-decider, no-decision-marker) never stamped it. So any decider comment that parses to nothing is re-driven on EVERY sweep, forever. The triggering comment was the human's own reopen note after the recorder wrongly closed the standing thread (issue262) - the repair for one defect became the fuel for the next. A second residual from issue262 was found in the same investigation: decision-issue has no concurrency group, so two runs triggered by near-simultaneous pushes could still both list-then-create the standing thread (the label lookup and in-process cache only close the WITHIN-run race - the live double-creation of #28/#29 was two lookups in ONE run, but the cross-run window is the same shape).";
        :>> decision = "Every reply in record_decision.sh's gesture region stamps `receipt-for-comment: $COMMENT_ID`, with one deliberate exception: the push-contention reply stamps `receipt-pending-for-comment` so the sweeper retries the recording. The invariant is enforced by a unit test (keel-cli github.rs, every_reply_in_the_gesture_region_stamps_its_receipt) that reads the ACTUAL script, anchors on a sentinel comment whose deletion fails the test loudly, and fails on any unreceipted reply - proven able to fail by stripping the receipts and watching it go red. decision-issue.yml gains `concurrency: group: decision-issue, cancel-in-progress: false`, the repo-level mirror of decision-record's group. The dchReceipt process step's actionText states the once-not-once-per-sweep behaviour.";
        :>> rationale = "The loop is structural, not content: deleting the one looping comment removes the instance, never the class - the next prose comment by a decider on any decision-labeled issue restarts it. Receipting every terminal reply makes every swept comment converge in exactly one pass, and an unparseable comment costs the human exactly ONE nag. Manual vigilance is not a control (D0047), and a script is invisible to every keel surface, so the invariant lives where CI runs it: a test that reads the script itself (the D0221 split - deterministic text logic testable in the binary's crate, gh effects in the thin script).";
        :>> consequences = "The standing thread self-heals on the first post-merge sweep: it posts one final didn't-parse nag WITH the receipt for the reopen comment, then goes silent. The historical nags remain as history - comments are never deleted by the engine. A future reply added to the gesture region without a receipt fails cargo test in CI. Two racing pushes can no longer double-create the standing thread. The enforcement surface (a CI workflow) changes under this co-committed Decision per D0209 clause 2.";
    }
}
