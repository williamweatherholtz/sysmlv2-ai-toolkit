// =====================================================================
// ProjectDeliveryS56 — Sprint 56: portGuardsCeremonyCharter (D0074 migration M3b).
// Story: port validate_ceremony (gate-ordering + retro-scan-evidence, issue010/issue011/D0047)
//        and validate_charter (newly-added Story has #CharteredBy, D0068 — the first git-aware
//        guard) to Rust. Parity vs the python guards. Precedes M3c (keystone + unified runner).
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS56 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from portGuardsCeremonyCharterStory to d0074;

    part portGuardsCeremonyCharterStory : Story {
        :>> id = "63940516-2c7d-4e8f-b056-516273849901";
        :>> title = "Sprint 56: portGuardsCeremonyCharter (5 pts) — port validate_ceremony + validate_charter to Rust; D0074 M3b";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS56 {
        action storyPortGuardsCeremonyCharter;
        verification storyPortGuardsCeremonyCharterDoD : Test { :>> id = "63940516-2c7d-4e8f-b056-516273849902"; :>> method = VerificationMethod::test; :>> procedureText = "portGuardsCeremonyCharterDoD: `sysmlv2 guard ceremony` (gate-ordering: no gate passed while an earlier DEFINED gate is unpassed; + retro-scan-evidence, issue011) + `sysmlv2 guard charter` (every newly-added .tracking/delivery Story has a #CharteredBy edge; git diff --cached --diff-filter=A + staged content, D0068) in Rust, each exiting non-zero on violation. PARITY: same verdict + violation SET as the python guards on the live repo AND on synthetic violations (unit tests mirror the python --selftests). ceremony reuses orient::gate_passed. clippy --workspace --all-targets -D warnings + cargo test green."; }
        part storyPortGuardsCeremonyCharterDoDR1 : TestResult { :>> id = "5f5b9d7c-b15d-42d3-a36d-e882ac98a461"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ecba70"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification portGuardsCeremonyCharterRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b056-516273849903"; :>> title = "Sprint 56 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: M3b — port validate_ceremony (text scan; reuse orient::gate_passed for passed + a defined-gate scan + ordering-violation + retro-scan-evidence) + validate_charter (the FIRST git-aware guard: git diff --cached --diff-filter=A added delivery files, staged content via git show :path, stories minus chartered). Parity bar = verdict + violation SET. estimatedPoints=5 (ceremony 2 + charter 3). #CharteredBy to d0074. DoD accepted."; }
    part portGuardsCeremonyCharterRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b056-516273849904"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ecba70"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCeremonyCharterStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b056-516273849905"; :>> title = "Sprint 56 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: extend src/guards.rs. ceremony: passed-set via orient::gate_passed over GATE_ORDER; defined-set via `verification <name ending {G}Gate>` scan; ordering_violations(defined,passed); retro_scan_missing (RetroGate procedureText must contain an avoidable/improvement keyword); GRANDFATHERED={sprint11_nativeSpikes}. charter: git diff --cached --diff-filter=A for added .tracking/delivery/*.sysml + git show :path staged content; charter_violations = stories - chartered (reuse algo::story_names + govern::collect_charter_work). Unit tests mirror the python --selftests; live-repo verdict parity. No blockers."; }
    part portGuardsCeremonyCharterStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b056-516273849906"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ecba70"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCeremonyCharterImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b056-516273849907"; :>> title = "Sprint 56 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Extended src/guards.rs with ceremony (gates_defined + gates_passed via orient::gate_passed; ordering_violations; retro_scan_missing) + charter (git diff --cached --diff-filter=A added delivery files, git show :path staged content, charter_violations = stories - chartered). `sysmlv2 guard ceremony|charter` wired. PARITY: both match the python guards on the live repo (ceremony 58 scanned/1 grandfathered warn/0 viol; charter 0 staged) AND the charter git path verified by staging sprint56 (both report 1 added, PASS). Unit tests: ceremony_ordering_violation_detected, retro_scan_evidence_required, charter_selftest (mirror the python --selftests). cargo test (24 lib) + clippy --workspace --all-targets -D warnings green; rust validate + all kernel-free validators PASS."; }
    part portGuardsCeremonyCharterImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b056-516273849908"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ecba70"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCeremonyCharterReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b056-516273849909"; :>> title = "Sprint 56 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. ceremony reuses orient::gate_passed for the passed-set (no re-implementation) + a focused gates_defined scan; the ordering + retro-scan logic mirrors validate_ceremony exactly. charter is the first git-aware guard: a pure charter_violations core (unit-tested) behind the git diff --cached / git show :path plumbing (smoke-tested with a real staged file). Parity = verdict + violation set + counts, all matching python. Both fold cleanly into the GuardReport abstraction, ready for M3c's unified runner. Next: M3c (process-change keystone + `sysmlv2 guard` runner + doc-sync)."; }
    part portGuardsCeremonyCharterReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b056-51627384990a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ecba70"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCeremonyCharterCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b056-51627384990b"; :>> title = "Sprint 56 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyPortGuardsCeremonyCharter + backlog portGuardsCeremonyCharter DoDs recorded pass; cargo test + clippy + rust validate + all kernel-free validators green; ceremony + charter Rust guards match the python guards' verdict + violation set (charter git path smoke-tested staged). Self-evidencing (D0051)."; }
    part portGuardsCeremonyCharterCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b056-51627384990c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ecba70"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCeremonyCharterRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b056-51627384990d"; :>> title = "Sprint 56 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: ceremony reused orient::gate_passed (the passed-set was already solved); charter's pure-core + git-plumbing split made it unit-testable AND let me smoke-test the real git path cheaply. The 5pt estimate held. CAUGHT: three clippy lints on the first build (doc backticks, case-sensitive extension compare, long first-doc-paragraph) — all mechanical, fixed immediately; no avoidable systemic issue, so no new tracked item or process improvement is needed (the deny-by-default lint config is doing its job). Next: M3c (the process-change keystone guard — staged git diff — plus the unified `sysmlv2 guard` runner over all six + doc-sync CLAUDE.md sec5)."; }
    part portGuardsCeremonyCharterRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b056-51627384990e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3ecba70"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
