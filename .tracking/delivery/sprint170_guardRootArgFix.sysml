// ProjectDeliveryS170 — Sprint 170: guardRootArgFix (post-v0.1.0) — fix `keel guard <ROOT>` arg parsing
// (resolves issue053, found in the v0.1.0 release smoke). A known guard name runs that guard; 'all'/absent/
// a non-name path runs ALL guards on that ROOT (parity with `keel validate <ROOT>`). EXECUTE. 1 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS170 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from guardRootArgFixStory to guardRootArgFix;

    part guardRootArgFixStory : Story {
        :>> id = "63940516-2c7d-4e8f-c170-51627384c001";
        :>> title = "Sprint 170: guardRootArgFix (1 pt) - keel guard <ROOT> no longer misread as a guard name (issue053)";
        :>> createdAt = "2026-06-30"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p3; :>> owner = "claudeOpus"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS170 {
        action storyGuardRootArgFix;
        verification storyGuardRootArgFixDoD : Test { :>> id = "63940516-2c7d-4e8f-c170-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Resolves issue053 (v0.1.0 release-smoke find): cmd_guard treated any non-'all' first arg as a guard NAME, so `keel guard .`/`keel guard <ROOT>` failed 'unknown guard' (inconsistent with `keel validate <ROOT>`). FIX: classify_guard_args (main.rs) - a first arg that is a known guard name (GUARD_NAMES + runnable diagnostics) runs THAT guard; 'all'/no-arg/a non-name path runs ALL guards on that ROOT. Permanent control (D0047): unit test guard_args_distinguish_name_from_root. VERIFIED LIVE: `keel guard .` -> ALL PASS (was 'unknown guard .'); `keel guard ceremony` -> single guard; clippy -D warnings + cargo test (1 bin test + 69 lib) + keel validate + guard all (14) green."; }
        part storyGuardRootArgFixDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c170-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b048fa"; :>> judgedAt = "2026-06-30"; :>> judgedBy = "claudeOpus"; }
    }

    verification guardRootArgFixRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c170-51627384c004"; :>> title = "Sprint 170 refine gate (DoR)"; :>> createdAt = "2026-06-30"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: issue053 (Low, release-smoke). Small CLI arg-parsing fix + regression test. estimatedPoints=1. #CharteredBy guardRootArgFix. DoR met."; }
    part guardRootArgFixRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c170-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b048fa"; :>> judgedAt = "2026-06-30"; :>> judgedBy = "claudeOpus"; }
    verification guardRootArgFixStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c170-51627384c006"; :>> title = "Sprint 170 standup gate"; :>> createdAt = "2026-06-30"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: extract classify_guard_args (name-vs-root disambiguation) + is_guard_name; cmd_guard uses it; unit-test the classifier. No blockers."; }
    part guardRootArgFixStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c170-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b048fa"; :>> judgedAt = "2026-06-30"; :>> judgedBy = "claudeOpus"; }
    verification guardRootArgFixImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c170-51627384c008"; :>> title = "Sprint 170 implement gate"; :>> createdAt = "2026-06-30"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "classify_guard_args + is_guard_name added; cmd_guard rewired. LIVE: `keel guard .` ALL PASS; `keel guard ceremony` single guard. clippy -D warnings clean; cargo test guard_args_distinguish_name_from_root pass + 69 lib; keel validate + guard all (14) green."; }
    part guardRootArgFixImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c170-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b048fa"; :>> judgedAt = "2026-06-30"; :>> judgedBy = "claudeOpus"; }
    verification guardRootArgFixReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c170-51627384c00a"; :>> title = "Sprint 170 review gate"; :>> createdAt = "2026-06-30"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. The classifier is a clean, testable disambiguation (known-name -> guard; else -> ROOT), giving guard/validate CLI parity. The regression test locks all the cases (no-arg, all, bare ROOT, '.', name, name+root). D0047 satisfied (defect -> permanent test). Clean."; }
    part guardRootArgFixReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c170-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b048fa"; :>> judgedAt = "2026-06-30"; :>> judgedBy = "claudeOpus"; }
    verification guardRootArgFixCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c170-51627384c00c"; :>> title = "Sprint 170 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-30"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; issue053 resolved (the resolver is done). clippy + cargo test + keel validate + guard all (14) green. Self-evidencing (D0051). The post-v0.1.0 CLI follow-up is closed."; }
    part guardRootArgFixCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c170-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b048fa"; :>> judgedAt = "2026-06-30"; :>> judgedBy = "claudeOpus"; }
    verification guardRootArgFixRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c170-51627384c00e"; :>> title = "Sprint 170 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-30"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the v0.1.0 released-binary smoke EARNED its keep - it surfaced a real CLI ergonomics inconsistency that the internal cwd-relative hook usage had masked; fixing it gives guard/validate parity + a regression test (D0047). AVOIDABLE-ISSUE SCAN: the defect class (CLI arg ambiguity) is now locked by guard_args_distinguish_name_from_root; LESSON: smoke the SHIPPED artifact the way an external user would (positional ROOT), not just the way internal hooks call it. Estimate 1 pt held. The post-release follow-up is done; backlog outstanding remains 0."; }
    part guardRootArgFixRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c170-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b048fa"; :>> judgedAt = "2026-06-30"; :>> judgedBy = "claudeOpus"; }
}
