// D0448 (status: the `status` field below is the truth) — The flow clock stops at the commit that RECORDS the retro result (pass or proposed), and carries the elapsed time from the preceding commit beside the file interval
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0448Accept event on sign-off.
package Decision0448 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0448 : Decision {
        :>> id = "cd551a7f-9537-4e9d-bfcc-ffa80b26a3fa";
        :>> title = "The flow clock stops at the commit that RECORDS the retro result (pass or proposed), and carries the elapsed time from the preceding commit beside the file interval";
        :>> createdAt = "2026-09-11";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "dcCycleTimeReadsFromGit moved cycle time and time-per-point from judgedAt (a date) to git (commit timestamps, minutes). The DoD names the clock's end as 'the commit carrying its retro gate's passing result'. Two facts of this tree bear on the meaning: - Under D0312 B an AI-examined retro result lands `proposed` and becomes `pass` only when the human judges it (`keel judge-set`). The eight most recent sprints (663-670) carry `proposed` retro results today. - 536 of 566 closed sprints, and 80 of the last 80, were authored and closed in ONE commit: the interval from the file's first commit to its retro landing is 0 for them, so a card built on it alone reads 0.0 h and every calibration bucket reads 0 min.";
        :>> decision = "The flow clock stops at the first commit whose version of the delivery file records the retro gate's result with outcome `pass` OR `proposed`. Beside that interval (`minutes`) the lens and the cards carry `elapsedMinutes`: the retro landing minus the timestamp of the repository commit that precedes the file's first commit. The point-calibration verdict is computed on `minutes` when fewer than half the window's sprints are one-commit sprints, else on `elapsedMinutes`, and the card names the basis it stands on. A sprint with no recorded retro result in any commit is `open`; it never contributes a 0.";
        :>> rationale = "A `proposed` result is the AI's testimony that the ceremony ended; what remains is the human's judgment, and D0204 says governance never measures the human. Stopping the clock at `pass` alone would make cycle time the length of the human's queue. `orient::gate_recorded` already reads `proposed` as recorded for the in-progress frontier, so the flow lens agrees with orient on which sprints are still open. The elapsed companion is the only interval git holds for a sprint born and closed in one commit; hiding it would make the calibration card say 'not predictive' about a project whose elapsed medians order by points (18 < 29 < 39 < 98 min over the last 80).";
        :>> consequences = "- `minutes` stays the DoD-literal measure and stays 0 for one-commit sprints; nobody should read that 0 as speed. - `elapsedMinutes` over-counts by whatever unrelated work sat between the preceding commit and the sprint's birth; it is an upper bound, stated as such in the lens note. - If the human later judges a `proposed` retro `fail`, the sprint's `closed` timestamp does not move back to open: the clock measured the ceremony, the judgment is a separate fact. - NOT A PROCESS CHANGE: no gate, guard, hook, workflow or contract moves; this fixes the meaning of one computed view.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0448Accept : Test { :>> id = "55c2bb2c-e7a9-4e4a-a5ad-a8a27bd61de1"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0448AcceptR1 : TestResult { :>> id = "6ec3748b-1489-4bf9-9781-c70ea69077b2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "40190c5"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
