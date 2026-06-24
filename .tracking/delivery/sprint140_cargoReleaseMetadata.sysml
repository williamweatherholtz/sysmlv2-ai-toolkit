// ProjectDeliveryS140 — Sprint 140: cargoReleaseMetadata (D0093) — add crate metadata
// (description/repository/readme/keywords/categories/rust-version) so the crates are release- and
// crates.io-ready (prereq for releaseWorkflow). 1 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS140 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from cargoReleaseMetadataStory to d0093;

    part cargoReleaseMetadataStory : Story {
        :>> id = "63940516-2c7d-4e8f-c140-51627384c001";
        :>> title = "Sprint 140: cargoReleaseMetadata (1 pt) — crate metadata for release/discoverability (D0093)";
        :>> createdAt = "2026-06-23"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p3; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS140 {
        action storyCargoReleaseMetadata;
        verification storyCargoReleaseMetadataDoD : Test { :>> id = "63940516-2c7d-4e8f-c140-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "keel-cli/keel-parser Cargo.toml lacked description/repository/readme/keywords/categories (not release- or crates.io-ready). FIX: added repository + rust-version=1.96 to [workspace.package] (shared), and per-crate description + keywords + categories (+ readme=../README.md for keel-cli; authors/license/repository/rust-version .workspace). VERIFY: `cargo package -p keel-parser --list` works (metadata accepted); cargo build --release + clippy --workspace --all-targets -D warnings green; keel validate 153 + guard all (13) green."; }
        part storyCargoReleaseMetadataDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c140-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "36f38c0"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification cargoReleaseMetadataRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c140-51627384c004"; :>> title = "Sprint 140 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: deployment-readiness #3 (prereq for releaseWorkflow). Crates lack release metadata. Resolver: cargoReleaseMetadata. estimatedPoints=1. #CharteredBy d0093. DoD accepted."; }
    part cargoReleaseMetadataRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c140-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "36f38c0"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoReleaseMetadataStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c140-51627384c006"; :>> title = "Sprint 140 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: shared fields (repository, rust-version) in [workspace.package]; per-crate description/keywords/categories (crates.io-valid categories). Verify cargo package --list + build + clippy. No .sysml/schema change. No blockers."; }
    part cargoReleaseMetadataStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c140-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "36f38c0"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoReleaseMetadataImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c140-51627384c008"; :>> title = "Sprint 140 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Added the metadata to Cargo.toml (workspace + both crates). VERIFY: cargo package --list works; build --release + clippy --all-targets -D warnings green; keel validate 153 + guard all (13) PASS."; }
    part cargoReleaseMetadataImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c140-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "36f38c0"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoReleaseMetadataReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c140-51627384c00a"; :>> title = "Sprint 140 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. Crates now carry description/repository/readme/keywords/categories + rust-version pin (1.96, matching the ciClippyGreen toolchain) — release-ready and crates.io-publishable if that path is ever taken. Shared fields hoisted to [workspace.package]. No behavior change. Clean."; }
    part cargoReleaseMetadataReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c140-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "36f38c0"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoReleaseMetadataCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c140-51627384c00c"; :>> title = "Sprint 140 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; crates release-ready; build + clippy + keel validate 153 + guard all (13) green. Self-evidencing (D0051). Deployment-readiness 3/8 done; consumerReadme next."; }
    part cargoReleaseMetadataCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c140-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "36f38c0"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    verification cargoReleaseMetadataRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c140-51627384c00e"; :>> title = "Sprint 140 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: pinning rust-version=1.96 in the metadata aligns the declared MSRV with the toolchain the CI + the ciClippyGreen fix established — one source of truth for the version bar. AVOIDABLE-ISSUE SCAN: none. (Side cleanup this turn: swept 25 orphaned ISysML JVM kernels left by the pre-commit kernel validators — env hygiene, not a code defect; a future option is reaping the kernel in the hook.) Estimate 1 pt held. 5 deployment tasks remain; consumerReadme next, then introductionDryRun + serveDownstreamDegrade (releaseWorkflow + releaseTag010 held for the human's ship decision)."; }
    part cargoReleaseMetadataRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c140-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "36f38c0"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
