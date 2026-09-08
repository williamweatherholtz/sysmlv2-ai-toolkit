// D0373 (status: the `status` field below is the truth) — noHookDaemon: the resident hook process is mooted, because the idle turn boundary answers from the guard receipt in 218-318 ms
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0373Accept event on sign-off.
package Decision0373 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0373 : Decision {
        :>> id = "a86490a2-4646-4c6a-a59b-63b452f3ffe7";
        :>> title = "noHookDaemon: the resident hook process is mooted, because the idle turn boundary answers from the guard receipt in 218-318 ms";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0367 ranked a resident process serving the hooks sixth and made it conditional: dcHookDaemonOnlyIfTheReceiptIsNotEnough says a daemon is built only if dcGateAnswersFromItsReceipt leaves an idle turn boundary above one second on this tree, measured. Measured on this tree, shell timer, `echo '{'session_id':'perf','stop_hook_active':false}' | keel hook stop`: 5202 / 5428 ms before the receipt (D0371, sprint 601), 218 / 260 / 318 ms after against an equal key. The first boundary after an edit still runs everything (the edited file is inside the two-second racy window and the run writes no receipt), and the boundary after that runs everything once more and writes - so the daemon's case would be the edit-adjacent boundary, not the idle one.";
        :>> decision = "No hook daemon. The idle turn boundary at 218-318 ms is under the one-second bar dcHookDaemonOnlyIfTheReceiptIsNotEnough set, so the resident process it conditioned on that number is not built; `keel hook` stays a one-shot process answering from the tree and, when the key is equal, from the receipt. NOT A FORK: the item fixed the rule and the bar, and this Decision records which side of it the measurement fell on.";
        :>> rationale = "The item's own argument stands: a daemon answers the same question faster still, but its correctness rests on the watcher having seen every write, and a missed event is a green over a red tree with nothing in the tree to show it. The receipt rests on facts the tree carries at the moment of asking - HEAD, git's status, each listed path's (len, mtime), the .keel/ inputs, the binary - and it has taken the idle case to a quarter of a second without holding any state between runs but a file whose validity is recomputed on every read. What the receipt does not speed - the boundary right after an edit - is the boundary that SHOULD run everything, and a daemon would not make that run honest, only warm.";
        :>> consequences = "`keel hook stop` remains one process per fire, ~0.2-0.3 s idle and ~5 s after an edit on this host. No socket, no lifetime, no second cache to invalidate. If a future measurement puts the edit-adjacent boundary itself in question, the tiers to look at first are the ones D0372 named - fewer spawns and less CPU under contention - before a resident process. dcHookDaemonOnlyIfTheReceiptIsNotEnough is done by its own rule with the measurement in its result; with it, the six buildable ranks of D0367 are closed and rank 7 (the host's Defender exclusion or Dev Drive) remains the human's.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0373Accept : Test { :>> id = "dd65f79a-27c5-4bcd-bad6-74f2f29a8467"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0373AcceptR1 : TestResult { :>> id = "8b78e3d7-24da-4db1-9743-73abc062a76d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1335211"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
