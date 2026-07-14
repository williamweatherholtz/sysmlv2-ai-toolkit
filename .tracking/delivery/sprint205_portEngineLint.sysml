// ProjectDeliveryS205 — Sprint 205: portEngineLint (D0112 phase 1) — the first kernel-free port of the
// .engine-instance lints. New Rust guard `engine-lint`: HARD (decisions import EngineWork) + WARN (tracked
// instances carry :>> id), replacing that slice of validate_instances.py. CHANGE (new guard) governed by
// the accepted D0112 (#ProspectiveChange). 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS205 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from portEngineLintStory to d0112;

    part portEngineLintStory : Story {
        :>> id = "63940516-2c7d-4e8f-c205-51627384c001";
        :>> title = "Sprint 205: portEngineLint (2 pt) - Rust engine-lint guard, first kernel-free port of the .engine lints (D0112 phase 1)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS205 {
        action storyPortEngineLint;
        verification storyPortEngineLintDoD : Test { :>> id = "63940516-2c7d-4e8f-c205-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Delivers D0112 phase 1 (accepted 2026-07-14): port the two MECHANICAL .engine-instance checks out of the JVM path (validate_instances.py) into a kernel-free Rust guard. guards.rs `engine_lint`: (1) HARD — every .engine/decisions/*.sysml must `import EngineWork` (the Decision type lives there); (2) WARN — every tracked instance (`part|verification|requirement <name> : <IdType>`, IdType per _ID_TYPES) carries an `:>> id` (§2.3). Wired into GUARD_NAMES (15->16) + run_one; D0047 unit test engine_lint_counts_tracked_instances (Decision/Test counted; SystemRequirement + non-instance lines not). doc-sync: CLAUDE.md §5 (15->16 guards, 14->15 hard-blocking). SCOPE HONESTY: this ports only the two PURE-PYTHON lints (already non-kernel in intent); the kernel is NOT removed — the parse + name/type-resolution deep semantics remain kernel-checked (D0112 phases 2-3). VERIFY: `keel guard engine-lint` PASS on the live repo (241 scanned, 0/0 — all decisions import EngineWork, no missing ids); clippy -D clean; cargo test (81) green; keel validate + guard all (16) green; kernel validate_instances still green (unchanged, backstop)."; }
        part storyPortEngineLintDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c205-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19c562d"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    }

    #Verify dependency from storyPortEngineLintDoD to d0112;

    verification pelRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c205-51627384c004"; :>> title = "Sprint 205 refine gate (DoR)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0112 accepted (human-judged 2026-07-14, wweatherholtz); phase 1 = the two mechanical lints -> a Rust guard; bounded, no kernel semantics touched. CHANGE (new guard) governed by D0112 #ProspectiveChange. estimatedPoints=2. #CharteredBy d0112. DoR met."; }
    part pelRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c205-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19c562d"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification pelStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c205-51627384c006"; :>> title = "Sprint 205 standup gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: engine_lint guard (import-EngineWork hard + missing-id warn) + count_tracked_instances helper; GUARD_NAMES 15->16 + run_one arm; unit test; doc-sync CLAUDE.md §5. No blockers."; }
    part pelStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c205-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19c562d"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification pelImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c205-51627384c008"; :>> title = "Sprint 205 implement gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "engine_lint guard + test + GUARD_NAMES 16 + doc-sync landed. `keel guard engine-lint` PASS (241 scanned, 0/0). clippy -D clean; cargo test (81) green; keel validate + guard all (16) green."; }
    part pelImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c205-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19c562d"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification pelReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c205-51627384c00a"; :>> title = "Sprint 205 review gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. Honest, bounded first step of the accepted D0112 direction: it moves ONLY the two mechanical lints (already pure-Python) off the kernel, keeps the kernel as the deep-semantics backstop (not removed), and doesn't overreach into name/type resolution (phase 2). The hard/warn split mirrors the Python (import = fail, missing-id = warn). New guard governed by D0112 #ProspectiveChange. Clean, kernel unchanged."; }
    part pelReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c205-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19c562d"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification pelCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c205-51627384c00c"; :>> title = "Sprint 205 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; engine-lint is the 16th enforced guard (kernel-free), delivering D0112 phase 1. clippy + cargo test + keel validate + guard (16) green. Self-evidencing (D0051). NEXT (D0112 phase 2, separately chartered): a Rust .engine reference resolver for the `unresolved` class; phase 3 demotes the kernel to CI-only."; }
    part pelCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c205-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19c562d"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification pelRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c205-51627384c00e"; :>> title = "Sprint 205 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the accepted D0112 direction turned into a concrete first increment within the same sitting — the two lints the spike identified as 'already pure-Python' are now a kernel-free Rust guard, a real (if small) dent in the JVM dependency. The spike's measure-first discipline paid off: we ported EXACTLY the mechanical checks and left the genuinely-kernel-hard semantics (phase 2/3) untouched, so no overreach. AVOIDABLE-ISSUE SCAN: none. LESSON: an accepted architecture Decision should yield a demonstrable first slice quickly (momentum + de-risking) rather than a big-bang. Estimate 2 pt held. NEXT: D0112 phase 2 (the .engine reference resolver) when chartered; open frontier otherwise only issue061."; }
    part pelRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c205-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "19c562d"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
}
