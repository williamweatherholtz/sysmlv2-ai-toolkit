// ProjectDeliveryS587 — Sprint 587: cut the release the field has been waiting seven days for, and make a failed update name what failed. EXECUTE. 2 pts.
package ProjectDeliveryS587 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from releaseV040Story to d0357;

    part releaseV040Story : Story {
        :>> id = "33b89b84-12af-4ba1-a5f4-7d353275d9b9";
        :>> title = "Sprint 587: cut the release the field has been waiting seven days for, and make a failed update name what failed (2 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS587 {
        action storyReleaseV040;
        verification storyReleaseV040DoD : Test { :>> id = "ac92ab0f-2677-4976-99a1-22d6a40b8a2a"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcMigrateReportsTheFailingLines (resolves issue384). DONE: the crate and the engine pin move to 0.4.0; the claude surface is regenerated so the plugin rendering carries the new version; migrate reports the lines of a failed gate that NAME a failure rather than the first forty, and records the same selection. VERIFIED: the three migration suites 20/20 after the fix; validate 666 clean; guard ALL PASS; clippy -D warnings clean; the full suite green on the tagged tree."; }
        // RAN: dcMigrateReportsTheFailingLines pass; 20/20 migration tests; validate 666; guard ALL PASS; clippy clean
        part storyReleaseV040DoDR1 : TestResult { :>> id = "63d73294-2db1-486a-b26d-c18e88de5e41"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dc7c7710ba4daf3dfa0a8633c14c36b6bf9d9de"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification releaseV040RefineGate : Test { :>> id = "3783342c-b9d6-4137-b2cf-59ee3a7b3953"; :>> title = "Sprint 587 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the owner answered 'Release now' on the published brief. The version is a minor bump - commands were added, none renamed - and the release renames nothing because they also chose to leave the command names alone."; }
    verification releaseV040StandupGate : Test { :>> id = "806fd69a-e134-4d6f-8ba3-f353d547afef"; :>> title = "Sprint 587 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THE BUMP BROKE TEN TESTS AND THE TOOL WOULD NOT SAY WHY. Every printed guard read PASS because migrate showed the first forty lines of a sixty-three-guard run; the real cause, a plugin rendering left stale by the version change, sat below the cut. Found by applying with --no-verify and running the gate by hand. The lesson is not the stale file - the gate caught that - it is that a diagnostic clipped by position hides exactly the line it exists to show."; }
    verification releaseV040ImplementGate : Test { :>> id = "0d85ff6c-8438-44d1-a1c9-ce284c378e6f"; :>> title = "Sprint 587 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: version bump; keel sync-claude (stamped 0.4.0); cargo test on migration_rolls_back, migration_fixture_is_realistic and federation = 20 passed; keel validate 666; keel guard ALL PASS; cargo clippy -D warnings clean; keel suite on the tagged tree."; }
    verification releaseV040ReviewGate : Test { :>> id = "c033062b-60df-4037-bdc3-0c4a2bf75bff"; :>> title = "Sprint 587 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) DID I RUN THE GATE THE PROCESS ASKS FOR: not at first - I ran validate and the suite and skipped guard, which is what would have named the stale rendering in seconds; the second attempt ran all of it. (2) IS THE FIX RELEVANCE-BASED: gate_failure_lines selects failing lines wherever they sit and falls back to the tail. (3) DOES THE RELEASE RENAME ANYTHING: no - the owner chose to leave the command names, so no adopter meets a rename."; }
    verification releaseV040CloseOutGate : Test { :>> id = "ab1f6cd2-7926-41dd-bc1d-46921fba307f"; :>> title = "Sprint 587 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. v0.4.0 is tagged and the Release is recorded as a fact carrying the tagged commit."; }
    verification releaseV040RetroGate : Test { :>> id = "aa6d9b94-689e-4104-bfac-8034843c39f1"; :>> title = "Sprint 587 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: issue384 is the truncation, fixed here. The process miss is mine and needs no new control - the deploy process already requires the full gate on the tree to be tagged, and running it is what found the defect."; }
}
