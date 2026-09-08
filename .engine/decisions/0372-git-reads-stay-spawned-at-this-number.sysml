// D0372 (status: the `status` field below is the truth) — gitReadsStaySpawned: the in-process gix move is mooted, because the residual git spawn time in keel orient is 406-433 ms after the duplicated divergence read was dropped
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0372Accept event on sign-off.
package Decision0372 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0372 : Decision {
        :>> id = "f1f6e71e-87db-41e9-8b26-9561f3265070";
        :>> title = "gitReadsStaySpawned: the in-process gix move is mooted, because the residual git spawn time in keel orient is 406-433 ms after the duplicated divergence read was dropped";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0367 ranked the in-process git move fifth and made it conditional: dcGitReadsStayInProcess says the residual spawns left after the parallel guards (D0368), the commit-fact cache (D0369) and the one-process corpus (D0370) are MEASURED, and the measurement decides - above 500 ms per orient the immutable-object reads move onto gix; below it the move is mooted by a Decision naming the number. Measured on this tree, KEEL_PERF=2, warm cache, shell timer: `keel orient .` spawned git 11 times in 473 / 476 / 561 ms (rev-parse --abbrev-ref x4, rev-list --left-right x2, cat-file --batch-check, log -1, rev-parse --git-path, --short, HEAD), of which `sync::divergence` was called twice from one function in orient.rs (lines 721 and 841) and accounted for 6 of the 11. With the second call reusing the first's value: git x8 in 406 / 431 / 433 ms, orient wall 1387-1459 ms, stdout byte-identical. In `keel guard --no-receipt` git spawns x41 sum to 4.9-9.4 s across the eight worker threads, but that sum is contention, not the critical path: the slowest guard alone (priority-inversion, 1452 ms) spends 374 ms in 7 spawns, and the immutable-object reads gix would replace (cat-file x4, log x5) are 9 of the 41.";
        :>> decision = "The immutable-object git reads stay as spawns. gix is not adopted: 406-433 ms per orient is under the 500 ms bar dcGitReadsStayInProcess set, and of the eight spawns left five read refs or the working tree (abbrev-ref, rev-list, git-path, HEAD, short), which gix would not remove. The one change made is the reuse in orient.rs of the divergence value already computed in the same function - a duplicate read, not a cache. NOT A FORK: the item itself fixed the rule and the number, and this Decision records which side of the bar the number fell on.";
        :>> rationale = "Sixty crates of dependency are not bought for less than half a second, in the item's own words - and here they would buy less than that, because the reads gix replaces are two of the eight orient spawns (cat-file and log, about 90 ms) while the ref reads it cannot replace are the rest. The residual is process-start cost per spawn (about 45-55 ms each on this host), and the honest answer to that is fewer spawns, which is what the duplicate removal did; the next tier of that answer, memoizing `staged_changes` across the six guards that call it (twelve spawns to two), belongs to guards.rs under the keystone lock and is not taken here - it is recorded as the number to beat if a guard's wall time is ever the question again.";
        :>> consequences = "`keel orient` spawns git eight times (was eleven) with byte-identical output. No new dependency. The `keel guard` residual is CPU across sixty-five guards under parallel contention (priority-inversion 1452 ms alone, 4915 ms under contention), not git; a future perf item on guard wall time starts from that fact, and from the receipt (D0371) already answering the idle case. dcGitReadsStayInProcess is done by its own rule with the measurement in its result.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0372Accept : Test { :>> id = "cf9c3d97-846c-439d-9a0d-299680d66ece"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0372AcceptR1 : TestResult { :>> id = "8ba427cd-f4a4-4193-9c96-da94aeee3b44"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1335211"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
