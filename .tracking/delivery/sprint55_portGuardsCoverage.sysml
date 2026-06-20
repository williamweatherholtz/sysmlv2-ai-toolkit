// =====================================================================
// ProjectDeliveryS55 — Sprint 55: portGuardsCoverage (D0074 migration M3a).
// Story: port the three no-git forward GUARDS — validate_actors, validate_acceptance_events,
//        validate_sprint_coverage — to Rust (`sysmlv2 guard <name>`), parity vs the python guards.
//        First of the M3 guard ports. Precedes M3b (ceremony+charter) + M3c (keystone+runner).
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS55 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from portGuardsCoverageStory to d0074;

    part portGuardsCoverageStory : Story {
        :>> id = "63940516-2c7d-4e8f-b055-516273849801";
        :>> title = "Sprint 55: portGuardsCoverage (5 pts) — port validate_actors/acceptance_events/sprint_coverage to Rust; D0074 M3a";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS55 {
        action storyPortGuardsCoverage;
        verification storyPortGuardsCoverageDoD : Test { :>> id = "63940516-2c7d-4e8f-b055-516273849802"; :>> method = VerificationMethod::test; :>> procedureText = "portGuardsCoverageDoD: `sysmlv2 guard actors` + `sysmlv2 guard acceptance-events` + `sysmlv2 guard sprint-coverage` in Rust, each exiting non-zero on violation. PARITY: same PASS/FAIL verdict + same violation SET as the python guard on the live repo (all PASS) AND on synthetic violations (unit-tested, mirroring the python --selftests). acceptance-events reuses view::compute_attestation (M2.2a). clippy --workspace --all-targets -D warnings + cargo test green."; }
        part storyPortGuardsCoverageDoDR1 : TestResult { :>> id = "1df1735c-341f-4d19-b959-673242ac2da7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1fe773a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification portGuardsCoverageRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b055-516273849803"; :>> title = "Sprint 55 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: M3a — port the three NO-GIT forward guards (actors, acceptance-events, sprint-coverage). All are text/decision scans, parity-checkable, de-risked by the M2.2 port pattern. Parity bar for a GUARD = same verdict + same violation SET (not byte-identical report text, which is human formatting). estimatedPoints=5 (actors 2 + acceptance-events 1 + sprint-coverage 2). #CharteredBy to d0074. DoD accepted."; }
    part portGuardsCoverageRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b055-516273849804"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1fe773a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCoverageStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b055-516273849805"; :>> title = "Sprint 55 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: new src/guards.rs with a GuardReport {name, scanned, warnings, violations} + a `sysmlv2 guard <name>` dispatcher (exit 1 on violation). actors: load known from .tracking/actors.sysml (part <id> : Person|Actor) + LEGACY_ACTORS set, scan .tracking for :>> authoredBy/createdBy/judgedBy values. acceptance-events: reuse view::compute_attestation via a new view::attestation_data. sprint-coverage: done tasks ({task}DoDR<n> pass in backlog) not in the delivery blob, minus GRANDFATHERED. Unit tests mirror the python --selftests; live-repo verdict parity vs the python guards. No blockers."; }
    part portGuardsCoverageStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b055-516273849806"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1fe773a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCoverageImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b055-516273849807"; :>> title = "Sprint 55 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented src/guards.rs (GuardReport + actors/acceptance-events/sprint-coverage) + view::attestation_data; `sysmlv2 guard <name>` wired in main.rs. PARITY: all three guards match the python guard's verdict + violation SET + counts on the live repo (actors 65 files/52 warn/0 err; acceptance-events 73/0; sprint-coverage 67/0; all PASS, exit 0==0). Unit tests mirror the python --selftests (actor_refs_extracted, dodr_task_stripped, sprint_coverage_selftest = orphan flagged). cargo test --workspace all pass (21 lib); clippy --workspace --all-targets -D warnings green; rust validate + validate_actors/ceremony/charter/sprint_coverage/acceptance_events/parity_check all PASS."; }
    part portGuardsCoverageImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b055-516273849808"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1fe773a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCoverageReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b055-516273849809"; :>> title = "Sprint 55 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. guards.rs introduces a clean GuardReport {name,scanned,warnings,violations} abstraction the M3c unified runner will aggregate. acceptance-events REUSES view::compute_attestation via the new attestation_data (single source of truth — no logic duplicated from the attestation-coverage view); actors/sprint-coverage are focused text scans reusing algo::{is_word,is_space}. Parity bar for a GUARD = verdict + violation SET (human report formatting is Rust-native, not byte-copied) — met, with scanned/warning counts ALSO matching python exactly. CLAUDE.md sec5 doc-sync deferred to M3c per the plan (the python guard path stays wired until the unified `sysmlv2 guard` runner replaces it). Next: M3b (ceremony + charter)."; }
    part portGuardsCoverageReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b055-51627384980a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1fe773a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCoverageCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b055-51627384980b"; :>> title = "Sprint 55 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyPortGuardsCoverage + backlog portGuardsCoverage DoDs recorded pass; cargo test + clippy + rust validate + all kernel-free validators green; the three Rust guards match the python guards' verdict + violation set + counts on the live repo. Self-evidencing (D0051: tested ports + verified parity)."; }
    part portGuardsCoverageCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b055-51627384980c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1fe773a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification portGuardsCoverageRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b055-51627384980d"; :>> title = "Sprint 55 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the GuardReport abstraction + reusing view::compute_attestation (acceptance-events) and algo helpers kept all three guards small; parity for guards (verdict + violation set, not byte-text) is the right, achievable bar and counts matched python exactly. The plan's 5pt estimate held. No avoidable issue arose — the guard-port pattern is now established for M3b/M3c, so no new tracked item or process improvement is warranted. Next: M3b (ceremony + charter — charter is the first git-aware guard)."; }
    part portGuardsCoverageRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b055-51627384980e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1fe773a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
