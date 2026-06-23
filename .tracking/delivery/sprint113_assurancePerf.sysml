// =====================================================================
// ProjectDeliveryS113 — Sprint 113: assurancePerf (D0048 — the Rust authority must stay FAST). Profiling
// the slow assurance path pinned the cost to orient::done_names spawning one `git cat-file` PER task
// (~120 process spawns on Windows ≈ 13.7s of a 15s `assured`). FIX: batch ALL distinct passing-result
// SHAs into ONE `git cat-file --batch-check` (the `valid_commits` helper orient::compute already uses) —
// identical semantics, ~N× fewer spawns. Measured: report assurance 27s->4.7s, assured 14.9s->3.6s,
// guard 15.5s->3.7s (and `guard` runs in the pre-commit hook, so EVERY commit got faster). Output
// byte-identical (assured verdict + blocker counts + orient done=276 unchanged). Total: 3 pts. Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS113 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from assurancePerfStory to d0048;

    part assurancePerfStory : Story {
        :>> id = "63940516-2c7d-4e8f-c113-51627384c001";
        :>> title = "Sprint 113: assurancePerf (3 pts) — batch done_names git validation; assurance/guard ~4-7x faster (D0048)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::refactor;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS113 {
        action storyAssurancePerf;
        verification storyAssurancePerfDoD : Test { :>> id = "63940516-2c7d-4e8f-c113-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0048 fast-authority perf pass. PROFILED compute_readiness with per-step timing: done_names 13.7s, orient 1.5s, model 0.06s, guards each <0.2s — the cost was orient::done_names calling git_sha_valid (one `git cat-file -t` spawn) PER task, ~120 spawns. FIX (orient.rs done_names): collect all distinct non-empty passing-result SHAs, validate in ONE batched `git cat-file --batch-check` via the existing `valid_commits` helper (the same one orient::compute already uses), then look up per task — empty SHA -> done; else commit-validity with conservative-true on git failure (BYTE-IDENTICAL to git_sha_valid's semantics; git_sha_valid stays used by govern.rs so no dead code). VERIFY (measured this sprint): report assurance 27.2s->4.7s, assured 14.9s->3.6s, guard all 15.5s->3.7s; assured ready=true + all 5 blocker counts 0 + stale 13 + orient done=276 IDENTICAL before/after. clippy --workspace --all-targets -D warnings + cargo test (46 lib + all BDD, orient 8/8) + keel validate 126 + guard all (13) green."; }
        part storyAssurancePerfDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c113-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9dbba2"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification assurancePerfRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c113-51627384c004"; :>> title = "Sprint 113 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the console-arc retros flagged report:assurance cold ~17s as a forward perf item; it taxes `guard` too (a pre-commit gate). D0048 makes the Rust toolchain the FAST kernel-free authority, so this is in-charter. Resolver: assurancePerf. estimatedPoints=3. #CharteredBy d0048. DoD accepted."; }
    part assurancePerfRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c113-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9dbba2"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification assurancePerfStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c113-51627384c006"; :>> title = "Sprint 113 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: profile compute_readiness with temporary per-step timing to LOCATE the cost (not guess), then fix the proven hotspot. Expectation: done_names per-task git spawn. Fix = batch via the existing valid_commits helper (no new code path, reuse orient::compute's proven batching). Verify by measuring before/after AND asserting byte-identical assured/done output. No .engine/schema/process-def change -> no kernel/keystone. No blockers."; }
    part assurancePerfStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c113-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9dbba2"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification assurancePerfImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c113-51627384c008"; :>> title = "Sprint 113 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Profiled (temporary instrumentation, then reverted): done_names=13.7s was the whole cost. Replaced the per-task git_sha_valid loop in orient::done_names with one batched valid_commits call. VERIFY (live measured): report assurance 27.2s->4.7s; assured 14.9s->3.6s; guard 15.5s->3.7s. Output identical: assured ready=true, blockers all 0, stale 13, orient done=276. clippy -D warnings green; cargo test 46 lib + BDD (orient 8/8) pass; keel validate 126; guard all (13) PASS."; }
    part assurancePerfImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c113-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9dbba2"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification assurancePerfReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c113-51627384c00a"; :>> title = "Sprint 113 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The fix is the minimal correct one: it reuses orient::compute's OWN batching helper (valid_commits) rather than inventing a path, so done_names and compute now share identical SHA-validation semantics (closing a latent inconsistency — done_names used git_sha_valid while compute used valid_commits). Pure performance: assured/done output proven byte-identical before/after, so no view-meaning change (EXECUTE, not CHANGE). Biggest practical win: `guard` is in the pre-commit hook, so every commit is now ~4x faster. Clean."; }
    part assurancePerfReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c113-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9dbba2"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification assurancePerfCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c113-51627384c00c"; :>> title = "Sprint 113 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyAssurancePerf DoD recorded pass; the batched done_names is live + measured (4-7x across assurance/assured/guard) + output byte-identical. clippy + cargo test (46 lib + BDD) + keel validate 126 + guard all (13) green. Self-evidencing (D0051). First of four post-D0094 forward items; serveAgentCancel next."; }
    part assurancePerfCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c113-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9dbba2"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification assurancePerfRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c113-51627384c00e"; :>> title = "Sprint 113 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: PROFILED before optimizing (temporary per-step timing) instead of guessing — the guess (the 11-guard loop) was WRONG; the real cost was a single function (done_names) doing N git spawns, invisible without measurement. The fix was tiny because orient::compute had ALREADY solved the same problem (valid_commits batching) one sprint cluster earlier — reusing it both fixed perf AND unified two divergent SHA-validation code paths. Asserting byte-identical output (not just 'looks right') kept a perf change from silently shifting a computed verdict. AVOIDABLE-ISSUE SCAN: one avoidable self-inflicted slip caught + fixed immediately — an Edit dropped the action def + package closing braces from backlog.sysml (append-result then failed 'unexpected end of input'); restored before proceeding; LESSON: when an Edit's old_string ends at a block's closing braces, the new_string must re-include them (the write API's parse-on-append caught it instantly — the fail-loud path working). NON-blocking forward notes (NOT debt, carried): (1) report:assurance cold is still ~4.7s — a shared cached Model across the guards loop (vs Model::build per guard) could shave more, deferred; (2) the SSE fingerprint loop is still per-client. Estimate 3 pts held. assurancePerf done; serveAgentCancel (Windows process-tree kill) next."; }
    part assurancePerfRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c113-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9dbba2"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
