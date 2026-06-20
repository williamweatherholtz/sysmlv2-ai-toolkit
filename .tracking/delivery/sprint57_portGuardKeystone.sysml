// =====================================================================
// ProjectDeliveryS57 — Sprint 57: portGuardKeystone (D0074 migration M3c).
// Story: port validate_process_change (the D0070 keystone hard-lock; staged git diff) to Rust,
//        AND add the unified `sysmlv2 guard` runner over all six ported guards. doc-sync
//        CLAUDE.md sec5 + usage.md to the Rust guard path. COMPLETES M3. Precedes M4 (retireQueryPy).
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS57 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from portGuardKeystoneStory to d0074;

    part portGuardKeystoneStory : Story {
        :>> id = "63940516-2c7d-4e8f-b057-516273849a01";
        :>> title = "Sprint 57: portGuardKeystone (5 pts) — port the process-change keystone guard + unified `sysmlv2 guard` runner; completes M3; D0074 M3c";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS57 {
        action storyPortGuardKeystone;
        verification storyPortGuardKeystoneDoD : Test { :>> id = "63940516-2c7d-4e8f-b057-516273849a02"; :>> method = VerificationMethod::test; :>> procedureText = "portGuardKeystoneDoD: `sysmlv2 guard process-change` (a staged .engine/processes|workflows change MUST co-commit a #Prospective/#SafetyChange-marked .engine/decisions file; git diff --cached --diff-filter=ACMR + git show :path; D0070) in Rust, exiting non-zero on violation. PLUS a unified `sysmlv2 guard` (no name) that runs all six guards and exits non-zero on any violation. PARITY: verdict + violation SET vs the python keystone guard (live repo + synthetic, unit-tested mirroring --selftest incl. prose-marker-does-not-count). doc-sync CLAUDE.md sec5 + usage.md to the Rust guard path. clippy --workspace --all-targets -D warnings + cargo test green. Completes M3."; }
        part storyPortGuardKeystoneDoDR1 : TestResult { :>> id = "59982e6b-e85d-43bd-bb37-9014467de88b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "09e0c4d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification portGuardKeystoneRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b057-516273849a03"; :>> title = "Sprint 57 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: M3c — port the keystone guard (staged git diff: process-def change requires a co-committed marked Decision; has_marker = line-anchored #Prospective/#SafetyChange, prose mentions excluded) + the unified `sysmlv2 guard` runner over all six. doc-sync rides this (M3c DoD). Parity bar = verdict + violation SET. estimatedPoints=5 (keystone 3 + runner/doc-sync 2). #CharteredBy to d0074. DoD accepted."; }
    part portGuardKeystoneRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b057-516273849a04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "09e0c4d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardKeystoneStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b057-516273849a05"; :>> title = "Sprint 57 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: extend src/guards.rs with process_change (staged_files via git diff --cached --diff-filter=ACMR; is_process_def/is_decision path predicates; has_process_marker line-anchored; keystone_violations pure core). Add a unified runner: bare `sysmlv2 guard` (or `guard all`) runs the six and aggregates exit. doc-sync CLAUDE.md sec5 + usage.md. Unit tests mirror validate_process_change --selftest (pass-with-marked, fail-unmarked, fail-no-decision, neutral, prose-marker-does-not-count). Smoke-test the staged git path. No blockers."; }
    part portGuardKeystoneStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b057-516273849a06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "09e0c4d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardKeystoneImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b057-516273849a07"; :>> title = "Sprint 57 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented guards::process_change (staged_files via git diff --cached --diff-filter=ACMR; is_process_def/is_decision_file; has_process_marker line-anchored; keystone_violations pure core) + the unified runner (run_one/run_all/GUARD_NAMES) wired to bare `sysmlv2 guard` / `guard all`. PARITY: keystone matches python on live (silent) AND a controlled staged process-def violation (both exit 1, byte-identical message), reverted after; `sysmlv2 guard` ALL PASS matches the six python guards. The unified runner CAUGHT a real parity bug in retro_scan_missing (it anchored on a 'RetroGate' substring in sprint56 standup prose, not the verification) — fixed to anchor on the `verification …RetroGate… : Test` declaration + regression test added. cargo test (25 lib) + clippy --workspace --all-targets -D warnings green; rust validate + all kernel-free validators PASS; CLAUDE.md sec5 + usage.md doc-synced."; }
    part portGuardKeystoneImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b057-516273849a08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "09e0c4d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardKeystoneReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b057-516273849a09"; :>> title = "Sprint 57 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. keystone is a pure keystone_violations core (unit-tested incl. prose-marker-does-not-count) behind git diff --cached / git show :path plumbing (smoke-tested both verdicts). The unified `sysmlv2 guard` runner (run_all over GUARD_NAMES) is the single Rust gate replacing the conda run python validate_*.py loop — and it immediately earned its keep by catching the retro_scan_missing anchoring bug that the per-guard M3b run had missed (Retro wasn't passed yet then). All six guards parity-verified vs python. M3 (the forward-guard port) is COMPLETE; query.py + the python guards retire at M4 (retireQueryPy)."; }
    part portGuardKeystoneReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b057-516273849a0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "09e0c4d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardKeystoneCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b057-516273849a0b"; :>> title = "Sprint 57 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyPortGuardKeystone + backlog portGuardKeystone DoDs recorded pass; cargo test + clippy + rust validate + all kernel-free validators green; `sysmlv2 guard` (all six) reports ALL PASS matching the python guards; keystone violation path smoke-tested + reverted. Self-evidencing (D0051). M3 COMPLETE."; }
    part portGuardKeystoneCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b057-516273849a0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "09e0c4d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardKeystoneRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b057-516273849a0d"; :>> title = "Sprint 57 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: building the unified `sysmlv2 guard` runner LAST (after all six guards) made it cheap, and it immediately caught a real retro_scan_missing parity bug that per-guard testing had missed — the strongest argument for an aggregate runner. The pure-core + git-plumbing split (keystone) again enabled a safe controlled violation smoke-test. CAUGHT (the avoidable issue): my retro_scan_missing anchored on a substring instead of the verification declaration — a FALSE-POSITIVE class that only surfaced when a delivery file mentions 'RetroGate' in prose; fixed + regression-tested. LESSON: text-anchored scans must key off the structural declaration (`verification … : Test`), not a bare substring — same class as the M2.2c statement-vs-line lesson; both are now guarded by tests, so no new tracked Issue is warranted. Estimate 5 pts held. M3 COMPLETE (all six forward guards in Rust). Next: M4 (retireQueryPy) — needs human go-ahead (deletes query.py)."; }
    part portGuardKeystoneRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b057-516273849a0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "09e0c4d"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
