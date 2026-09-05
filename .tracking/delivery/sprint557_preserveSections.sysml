// ProjectDeliveryS557 — Sprint 557: a project's own unit-extras sections survive keel migrate - merged, reported, and a collision blocks by name. EXECUTE. 3 pts.
package ProjectDeliveryS557 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from preserveSectionsStory to d0317;

    part preserveSectionsStory : Story {
        :>> id = "85deda14-adc1-4621-b6c7-93ecd7cfaf10";
        :>> title = "Sprint 557: a project's own unit-extras sections survive keel migrate - merged, reported, and a collision blocks by name (3 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS557 {
        action storyPreserveSections;
        verification storyPreserveSectionsDoD : Test { :>> id = "cddf51d8-44c4-4a21-bbba-e2ba456e497b"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcMigratePreservesProjectSections (D0317; resolves issue349 / GH#44). DONE: step engine-resync merges sectioned contracts (unit-extras.toml) instead of overwriting - the engine's text leads, project-authored sections are appended under a header naming the rule, the run's detail says how many were PRESERVED, and a section both copies carry with different bodies is a Blocker naming it (refuse, write nothing). Unit tests for the merge and the collision; the realistic-fixture suite gains its fifth property (the section rides through and the run says so); project-migration skill and CLAUDE.md §4 name the sections among what must survive."; }
        // RAN: same suite receipt: 506/0 exit 0
        part storyPreserveSectionsDoDR1 : TestResult { :>> id = "36247fe3-acff-420e-a82d-0db24810bf78"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "88864053792a741b681bcee2fe202d2d1b14dffe"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification preserveSectionsRefineGate : Test { :>> id = "6e729a38-bc30-4ab5-b0ec-46a221223b5c"; :>> title = "Sprint 557 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the resync already had the never-overwrite list for project-owned contracts (issue314) and a Blocker mechanism that refuses a whole run; the design question was why not add unit-extras.toml to the list - because that would freeze the engine's own sections at scaffold vintage. Merge keeps both halves moving."; }
    verification preserveSectionsStandupGate : Test { :>> id = "a999d5a2-f8c8-4a5c-8e93-5ec815450b40"; :>> title = "Sprint 557 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE BOUNDARY STATED: the engine's own unit-extras.toml carries no section today, so a fixture cannot collide with one; the collision path is proven by unit test only, said so in the test's doc and in D0317."; }
    verification preserveSectionsImplementGate : Test { :>> id = "18b19aef-140d-4995-b1ec-8d7fd4e6e44b"; :>> title = "Sprint 557 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --lib section_merge 2 passed; cargo test --test migration_fixture_is_realistic extras 1 passed (the section intact after migrate, the run naming PRESERVED and issue349, the migrated project gating green); clippy -D warnings clean after the format_push_string fix; the full suite in the background to completion (receipt on the DoD result)."; }
    verification preserveSectionsReviewGate : Test { :>> id = "5f809731-3060-47ed-8173-daa22c7a5b5e"; :>> title = "Sprint 557 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) CAN THE MERGE LOSE DATA: a project section is appended verbatim; the only path that drops nothing and writes nothing is the collision block. (2) IS THE COLLISION RULE TOO STRICT: comments and blank lines are normalised away, so a re-commented engine section does not block; a body change does, which is the case nobody can decide mechanically. (3) DOES THE RUN SAY WHAT IT DID: the detail line counts preserved sections and the fixture asserts the words."; }
    verification preserveSectionsCloseOutGate : Test { :>> id = "22bb9a08-4890-4037-bc9d-4867ce6e103b"; :>> title = "Sprint 557 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0317 accepted under standing consent; issue349 resolved by computation."; }
    verification preserveSectionsRetroGate : Test { :>> id = "a47f6515-e0a0-4672-8411-15d118a29763"; :>> title = "Sprint 557 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the remaining migration exposure is the engine's inability to migrate itself (dcMigrationFixtureIsRealistic's standing note) and the fixture suite is where each new property lands. The lesson: a file that two parties write needs a merge, not an owner."; }
}
