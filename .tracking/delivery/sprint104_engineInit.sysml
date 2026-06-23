// =====================================================================
// ProjectDeliveryS104 — Sprint 104: engineInit (D0093 cold-start milestone). `sysmlv2 init DIR`
// scaffolds a NEW project from a binary-EMBEDDED engine tree (include_dir!, the cytoscape-vendoring
// precedent): .engine/ + CLAUDE.md + a starter .tracking/ + a printed getting-started. RESOLVES the
// engine/instance boundary CONCRETELY (the d0093 critique's delegated concern): the engine's
// architecture decisions remap to read-only .engine/reference/decisions/ (NOT scanned as instance —
// the rust views scan .engine/decisions), .engine/decisions/ ships EMPTY for the new project's own,
// and the instance-specific deliverable-manifest.txt is reset to a starter. Verified live: a fresh
// `init` project validates clean + guards ALL PASS + shows 0 instance decisions.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS104 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from engineInitStory to d0093;

    part engineInitStory : Story {
        :>> id = "63940516-2c7d-4e8f-c104-51627384c001";
        :>> title = "Sprint 104: engineInit (5 pts) — `sysmlv2 init DIR` cold start + binary-embedded engine tree + engine/instance boundary resolution (D0093)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS104 {
        action storyEngineInit;
        verification storyEngineInitDoD : Test { :>> id = "63940516-2c7d-4e8f-c104-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0093 cold start: `sysmlv2 init DIR` scaffolds a new project. Embed the engine tree at compile time (include_dir crate; static ENGINE_DIR = include_dir!(.../.engine) + include_str! CLAUDE.md) — self-contained, the cytoscape precedent. Scaffold .engine/ (remap_engine_path: decisions/* -> reference/decisions/* read-only, so the engine's architecture decisions do NOT enter the new project's computed views which scan .engine/decisions; D0093 boundary), an EMPTY .engine/decisions/ for the project's own decisions, a starter .engine/deliverable-manifest.txt (the shipped one lists the engine's OWN tasks -> would fail manifest-coverage), CLAUDE.md, and a starter .tracking/README.md + a printed getting-started. Refuses to overwrite an existing .engine/. VERIFY (LIVE, ran during this sprint): `sysmlv2 init /tmp/p` scaffolded 210 files; the fresh project `sysmlv2 validate` clean + `guard` ALL PASS (13) + `decisions` shows 0 (reference not instance) + `orient` clean + reference/decisions has 93 + .engine/decisions empty; unit test engine_path_remap_isolates_decisions locks the boundary; main repo clippy -D warnings + cargo test (47 lib) + validate 117 + guard all (13) green."; }
        part storyEngineInitDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c104-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10fd40e"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification engineInitRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c104-51627384c004"; :>> title = "Sprint 104 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0093 accepted; engineInit is the cold-start foundation (and exercises the introduction process's target). Resolver: engineInit. estimatedPoints=5 (the meatiest spin-up milestone — embedding + the boundary resolution). #CharteredBy d0093. DoD accepted: init produces a fresh, guard-green project with engine decisions as reference."; }
    part engineInitRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c104-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10fd40e"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineInitStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c104-51627384c006"; :>> title = "Sprint 104 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add include_dir dep; static ENGINE_DIR + CLAUDE_MD + starters; cmd_init walks the embedded tree (scaffold_engine recursion; remap_engine_path pure + unit-tested), writes CLAUDE.md + starter .tracking; dispatch + help + doc. Verify by ACTUALLY running init to a temp dir + guarding the fresh project. No process-def/schema change -> no keystone/kernel. No blockers."; }
    part engineInitStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c104-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10fd40e"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineInitImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c104-51627384c008"; :>> title = "Sprint 104 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented cmd_init + embed + remap + starters. VERIFY (live): init scaffolded 210 files to a temp dir; fresh project validate clean + guard ALL PASS (13) + decisions 0 + reference/decisions 93 + .engine/decisions empty; engine_path_remap_isolates_decisions test pass; clippy -D warnings green; cargo test 47 lib pass; main repo validate 117 + guard all (13) PASS."; }
    part engineInitImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c104-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10fd40e"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineInitReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c104-51627384c00a"; :>> title = "Sprint 104 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. `init` is self-contained (embedded tree, no external fetch — the engine's offline ethos) and the engine/instance boundary is resolved AND PROVEN live: a fresh project is immediately valid + guard-green, the engine's 93 decisions are read-only reference (computed views show 0 instance decisions), and the instance-specific deliverable-manifest is reset. The remap is a pure, unit-tested fn. Cold start works end to end. Clean."; }
    part engineInitReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c104-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10fd40e"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineInitCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c104-51627384c00c"; :>> title = "Sprint 104 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyEngineInit DoD recorded pass; `sysmlv2 init` live + the fresh project guard-green; engine/instance boundary resolved (decisions->reference, manifest reset); clippy + cargo test (47) + validate 117 + guard all (13) green. Self-evidencing (D0051). Two of three D0093 milestones done (introductionProcess + engineInit); orientDashboard remains."; }
    part engineInitCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c104-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10fd40e"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineInitRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c104-51627384c00e"; :>> title = "Sprint 104 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: VERIFYING BY ACTUALLY RUNNING init (scaffold a temp project, then validate + guard it) caught the boundary defects a code-read would have missed; the d0093 correctness critique correctly PREDICTED the decisions-pollution risk, and resolving it (remap to reference/) made the fresh project's `decisions` view honestly show 0; reusing include_dir + a pure unit-tested remap kept it clean. AVOIDABLE-ISSUE SCAN: one real avoidable defect was CAUGHT + fixed in-sprint — the shipped deliverable-manifest.txt is INSTANCE-specific (lists the engine's own rust tasks) and failed manifest-coverage (34 violations) on the fresh project; the engine/instance boundary is BROADER than just decisions, and any future engine-specific artifact added to .engine (a manifest-like file listing this project's tasks/paths) must be reset by init too — a candidate forward guard: an init smoke-test in CI that scaffolds + guards a temp project so a new instance-specific file can't silently break cold start. Logged as the lesson; not yet a tracked Issue (the broader risk is low — most of .engine is genuinely reusable). Estimate 5 pts held (the boundary surprise was absorbed). engineInit done; orientDashboard is the last spin-up milestone."; }
    part engineInitRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c104-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "10fd40e"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
