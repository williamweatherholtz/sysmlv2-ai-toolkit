// =====================================================================
// ProjectDeliveryS89 — Sprint 89: viewpointRendererGuard (resolves issue034; hardens D0056). The
// d0056 critique finding: the viewpoint registry is descriptive, not generative — renderers drift to
// dead tools. Backfill the 12 stale renderers (query.py/report.py -> real sysmlv2 commands; 3 unbuilt
// marked "(planned)") + add the viewpoint-renderer GUARD (enforced) so a renderer can never again
// name a retired/unknown command. ENGINE HARDENING (campaign batch-2 ACT disposition).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS89 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from viewpointRendererGuardStory to d0056;

    part viewpointRendererGuardStory : Story {
        :>> id = "63940516-2c7d-4e8f-b089-51627384ba01";
        :>> title = "Sprint 89: viewpointRendererGuard (3 pts) — backfill the stale viewpoint registry + add the enforced viewpoint-renderer guard (resolves issue034, hardens D0056)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS89 {
        action storyViewpointRendererGuard;
        verification storyViewpointRendererGuardDoD : Test { :>> id = "63940516-2c7d-4e8f-b089-51627384ba02"; :>> method = VerificationMethod::test; :>> procedureText = "Resolve issue034 (d0056 finding: viewpoint registry descriptive-not-generative, renderers drift) via the backlog action viewpointRendererGuard. (1) BACKFILL viewpoint-registry.sysml: every renderer referenced a RETIRED tool (12 of 15 — query.py [retired D0074 M4] / report.py [never existed in Rust]); rewrite to the real sysmlv2 command (orient/whats-next/diagram[traceability]/decisions/suspect/orphans/attestation-coverage/governing-version/reprocess-candidates) + mark the 3 unbuilt views (safety/requirements/baselines) '(planned)'. (2) GUARD guards.rs viewpoint_renderer + classify_renderer: a viewpoint renderer naming query.py/report.py (retired) or an unknown command is a VIOLATION; '(planned ...)' is a tolerated WARNING; a real sysmlv2 subcommand is ok. Add to GUARD_NAMES (10, ENFORCED) + run_one. VERIFY (live): `sysmlv2 guard viewpoint-renderer` = 15 scanned, 0 violations, 3 planned warnings; clippy --workspace --all-targets -D warnings + cargo test (40 lib, +viewpoint_renderer_classification) + rust validate (101) + guard all (10) green; doc-sync CLAUDE.md guard list (was stalely 'seven', now ten incl critique/assured/viewpoint-renderer). issue034 computes RESOLVED."; }
        part storyViewpointRendererGuardDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b089-51627384ba03"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ff4a54d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification viewpointRendererGuardRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b089-51627384ba04"; :>> title = "Sprint 89 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: engine hardening — work the campaign-batch-2 ACT disposition issue034 (resolver viewpointRendererGuard). The d0056 completeness finding: viewpoints are declared but renderers can drift to dead tools. Confirmed live: 12 of 15 renderers referenced retired query.py/report.py. Plan: backfill the registry + add an enforced guard. estimatedPoints=3. #CharteredBy d0056. DoD accepted."; }
    part viewpointRendererGuardRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b089-51627384ba05"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ff4a54d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification viewpointRendererGuardStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b089-51627384ba06"; :>> title = "Sprint 89 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: rewrite viewpoint-registry.sysml renderers to real sysmlv2 commands + '(planned)' for unbuilt; guards.rs classify_renderer (retired/planned/ok/unknown) + viewpoint_renderer guard + quoted_attr helper + VIEW_SUBCOMMANDS const; GUARD_NAMES 9->10 + run_one; regression test viewpoint_renderer_classification; doc-sync CLAUDE.md. Watch: clippy str-split/unwrap; the 3 planned warnings are expected (not violations). No blockers."; }
    part viewpointRendererGuardStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b089-51627384ba07"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ff4a54d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification viewpointRendererGuardImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b089-51627384ba08"; :>> title = "Sprint 89 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented: backfilled all 12 stale renderers + 3 '(planned)'; viewpoint_renderer guard + classify_renderer + GUARD_NAMES[10] + run_one; regression test. VERIFY (live): `sysmlv2 guard viewpoint-renderer` = 15 scanned / 0 violations / 3 planned warnings; append-result marked viewpointRendererGuard DONE -> open-issues 5->4 (issue034 RESOLVED); clippy --workspace --all-targets -D warnings green; cargo test 40 lib pass; rust validate 101 clean; guard all (10) PASS; CLAUDE.md guard list synced (seven->ten). "; }
    part viewpointRendererGuardImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b089-51627384ba09"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ff4a54d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification viewpointRendererGuardReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b089-51627384ba0a"; :>> title = "Sprint 89 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The guard is the permanent control the d0056 finding called for: a viewpoint renderer can never again silently reference a retired/unknown tool (the 12-renderer drift this fixed could not recur — it would now fail pre-commit). The planned-vs-retired distinction is honest (3 unbuilt views warn, not fail). The backfill ALSO surfaced + fixed a pre-existing doc drift (CLAUDE.md said 'seven forward guards' since D0081 added critique/assured — now correctly 'ten'). classify_renderer is a pure tested helper. issue034 resolves by computation (resolver done). No new decision/process-def (a Rust guard + an instance backfill under D0056); no keystone."; }
    part viewpointRendererGuardReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b089-51627384ba0b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ff4a54d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification viewpointRendererGuardCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b089-51627384ba0c"; :>> title = "Sprint 89 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyViewpointRendererGuard DoD recorded pass; registry backfilled; viewpoint-renderer guard enforced (GUARD_NAMES[10]); issue034 RESOLVED; clippy + cargo test (40) + rust validate (101) + guard all (10) green; doc-sync done. Self-evidencing (D0051). First engine-hardening finding (issue034) closed."; }
    part viewpointRendererGuardCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b089-51627384ba0d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ff4a54d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification viewpointRendererGuardRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b089-51627384ba0e"; :>> title = "Sprint 89 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the campaign->finding->ACT->guard loop closed end-to-end — a critique finding (d0056) became a tracked issue, then a permanent enforced guard that makes the flagged drift unrepeatable (exactly D0047). Building the guard immediately PROVED the finding: 12 of 15 renderers were stale (query.py/report.py), far more drift than expected. CAUGHT (avoidable-issue candidate): the backfill exposed that CLAUDE.md still said 'seven forward guards' though critique+assured joined at D0081 — an avoidable doc drift that the doc-sync discipline should have caught two milestones ago; fixed now (->ten). Lesson: when adding a guard, re-read the guard-count claim in CLAUDE.md — guard-set changes are exactly the doc surface most prone to silent staleness. Estimate 3 pts held. issue034 closed; 4 hardening findings remain (issue029/030/031/033)."; }
    part viewpointRendererGuardRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b089-51627384ba0f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ff4a54d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
