// =====================================================================
// ProjectDeliveryS134 — Sprint 134: cargoTestGate (resolves issue042) — add a CI workflow enforcing
// `cargo test` + `cargo clippy -D warnings` on push/PR to main. Closes the gap the critique campaign
// found (n14/issue042): the .githooks pre-commit hook ran clippy + keel validate/guard but NOT cargo
// test, and there was no CI, so a failing test could reach main. A CI job (not the hook) keeps the
// ~15s test run off the per-commit path (n11/D0054 friction). Total: 2 pts. Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS134 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from cargoTestGateStory to n14TddBdd;

    part cargoTestGateStory : Story {
        :>> id = "63940516-2c7d-4e8f-c134-51627384c001";
        :>> title = "Sprint 134: cargoTestGate (2 pts) — CI workflow enforces cargo test + clippy green (resolves issue042 / n14)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS134 {
        action storyCargoTestGate;
        verification storyCargoTestGateDoD : Test { :>> id = "63940516-2c7d-4e8f-c134-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Resolves issue042 (ACT-dispositioned) + realizes n14/sr14's 'CI shall enforce cargo test and cargo clippy -D warnings green'. Added .github/workflows/ci.yml (on push + PR to main, ubuntu-latest): cargo clippy --workspace --all-targets -- -D warnings + cargo test --workspace --release (with rust-cache). A CI JOB rather than the pre-commit hook, per the issue042 ACT rationale — keeps the ~15s test run off the per-commit path (n11/D0054). Rust gates only; the .sysml kernel/keel guard checks stay in the pre-commit hook (they need the local conda kernel). VERIFY: the workflow is well-formed + runs the SAME commands that pass locally (clippy -D warnings green; cargo test 48 lib + BDD green); on push it now gates cargo-test/clippy regressions on main (the enforcement issue042 required); keel validate + guard all (13) green."; }
        part storyCargoTestGateDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c134-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "608529f"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification cargoTestGateRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c134-51627384c004"; :>> title = "Sprint 134 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: issue042 (cargo test not gated, no CI) was ACT-dispositioned with the rationale 'prefer a CI job over the pre-commit hook'. This realizes n14/sr14's CI clause. Resolver: cargoTestGate. estimatedPoints=2. #CharteredBy n14TddBdd. DoD accepted."; }
    part cargoTestGateRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c134-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "608529f"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoTestGateStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c134-51627384c006"; :>> title = "Sprint 134 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add .github/workflows/ci.yml running clippy -D warnings + cargo test on push/PR to main (ubuntu, rust-cache). CI job (not hook) per the ACT rationale. Verify the commands match the green-locally set. No .engine/schema/process-def change. No blockers."; }
    part cargoTestGateStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c134-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "608529f"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoTestGateImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c134-51627384c008"; :>> title = "Sprint 134 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Added .github/workflows/ci.yml (clippy --workspace --all-targets -D warnings + cargo test --workspace --release on push/PR to main). VERIFY: well-formed YAML; the commands are exactly the ones green locally (clippy clean; cargo test 48 lib + BDD pass); keel validate 147; guard all (13) PASS. CI now gates the cargo-test half of n14 that was previously unenforced."; }
    part cargoTestGateImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c134-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "608529f"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoTestGateReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c134-51627384c00a"; :>> title = "Sprint 134 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The CI job closes issue042 the way the ACT rationale chose (off the per-commit path): clippy + cargo test now gate main on GitHub, so a logic regression that the pre-commit hook misses (it doesn't run cargo test) is caught before it persists — D0047 'manual vigilance is not a control' satisfied with an automated gate. Rust-only scope is correct (the .sysml checks need the local kernel). Clean."; }
    part cargoTestGateReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c134-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "608529f"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoTestGateCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c134-51627384c00c"; :>> title = "Sprint 134 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyCargoTestGate DoD pass; CI workflow live; issue042 resolved (resolver done -> closed). keel validate 147 + guard all (13) green. Self-evidencing (D0051). First of the three ACT-resolver follow-ups."; }
    part cargoTestGateCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c134-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "608529f"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoTestGateRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c134-51627384c00e"; :>> title = "Sprint 134 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the CI job realizes the exact gap the critique campaign found (issue042) using the model the human's ACT rationale chose (CI over hook, to protect n11/D0054 friction) — the finding->disposition->resolver loop closed end to end. AVOIDABLE-ISSUE SCAN: no new defect. NON-blocking forward note (NOT debt): the CI's real first run happens on GitHub post-push; if the ubuntu runner surfaces any platform-specific test assumption it will be a follow-up (the suite is cross-platform: git-based BDD + the keel binary, no Windows-only unit tests). Estimate 2 pts held. Two ACT resolvers remain (refreshRequirementText, writePolicyResolve)."; }
    part cargoTestGateRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c134-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "608529f"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
