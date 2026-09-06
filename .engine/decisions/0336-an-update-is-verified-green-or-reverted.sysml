// D0336 (status: the `status` field below is the truth) — updateVerifiedOrReverted: keel migrate runs the project's own gate after applying and reverts on any red, recording the attempt where the human will see it
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0336Accept event on sign-off.
package Decision0336 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0336 : Decision {
        :>> id = "4ecfe344-f777-4996-b1af-a469b02fb4d1";
        :>> title = "updateVerifiedOrReverted: keel migrate runs the project's own gate after applying and reverts on any red, recording the attempt where the human will see it";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "srUpdateIsVerifiedOrReverted and srUnprovenUpdateAsksRatherThanActs (nUnverifiableChangeReverts, D0267): an automatically applied engine update is kept only if the project's own gate passes after it, and a reverted or refused update is recorded, not logged and forgotten. dcMigrationRollsBack delivered the rollback (the re-plan check and the interruption marker); nothing yet ran the PROJECT's gate after a migration, and the command ended by telling the reader to run it themselves - so an unattended update could leave a tree that migrated and no longer gated. The DoD noted the trap: gating on an empty plan is vacuous, because an empty plan returns before the re-stamp and never reaches the boundary.";
        :>> decision = "After apply, re-plan and re-stamp, keel migrate runs the project's own gate under the binary that wrote the tree - validate, guard all, check-engine - and retains the update on green (reporting what moved including the pin) or, on ANY red, restores .engine and .tracking to the pre-update commit, prints the gate's output verbatim, and records the attempt (version, date, outcome, the gate, its output) in the machine-local .keel/update-attempts.toml; keel status shows the latest attempt in its engine section and a later keel migrate of the same version says it was reverted here before trying again; --no-verify skips the gate and the run says the tree is UNVERIFIED.";
        :>> rationale = "Why the project's gate and not the re-plan: the re-plan proves the transform did what it reported; only the gate proves the project still holds under the new engine - a resynced guard the tree violates is exactly the issue089/issue090 class the requirement names. Why revert rather than leave the tree for inspection: migrated-but-not-gating is the one state the requirement forbids, and the rollback the earlier clause built makes reverting cheap and exact (git status --porcelain over .engine and .tracking is empty afterwards). Why a machine-local record rather than an Issue in the model: the tree has just been restored to a commit that must not change, and the record is about this machine's attempt; status and the next run read it, which is where the human looks. Why verify is the default: an update that must be verified by a human remembering to run three commands is the D0047 manual-vigilance shape. Arming (D0298): the realistic-fixture suite's sixth property - a green update retained and recorded; a committed duplicate-identity violation reverted with the gate's output, the pin unmoved, git status empty, status naming it, a re-run naming it.";
        :>> consequences = "Every migrate now takes the gate's time and ends in one of two states. A project whose gate was already red before an update will see that update reverted with the red reported, which is the honest reading of the requirement and is stated here. dcAutoUpdateVerifiedOrReverted is done; both SystemRequirements are verified by its DoD. STATED: the gate runs as three subprocesses of the same binary, so its verdict is the project's own and not this function's opinion of it.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0336Accept : Test { :>> id = "d4af11e1-b2f5-4052-b29a-8fed7007d415"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0336AcceptR1 : TestResult { :>> id = "fd5eb4c2-aaef-45d4-b125-a198d3a0bad7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9a726d2"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
