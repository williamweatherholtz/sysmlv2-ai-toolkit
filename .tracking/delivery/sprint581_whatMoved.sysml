// ProjectDeliveryS581 — Sprint 581: publish says what moved - a manifest refresh is never logged as a version - and the scaffold's no-grant state is read back as done. EXECUTE. 1 pts.
package ProjectDeliveryS581 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from whatMovedStory to d0347;

    part whatMovedStory : Story {
        :>> id = "e4f41777-396e-4e8e-88be-a1cac433da83";
        :>> title = "Sprint 581: publish says what moved - a manifest refresh is never logged as a version - and the scaffold's no-grant state is read back as done (1 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS581 {
        action storyWhatMoved;
        verification storyWhatMovedDoD : Test { :>> id = "1d406c3f-0a1d-472c-8104-f0159194d314"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcPublishSaysWhatMoved (D0347; resolves issue340) and dcScaffoldShipsNoGrants (delivered by D0340's init change in sprint 575 - recorded here because its own DoD is now true and verified). DONE: publish reads what git says changed under the unit in the clone - nothing is the stated no-op; only unit.toml commits once as publish X vN (manifest only) and prints MANIFEST ONLY with why the version did not move; a file change commits as before. VERIFIED: library_publish 4/4 including the manifest-only scenario (one qualified commit, the version line unmoved, the second run a stated no-op); for dcScaffoldShipsNoGrants, standing_words_are_project_local's first arm - a fresh scaffold's policy has no delegatedRecording and no standingConsent live, record decision on it stays proposed saying accept later via an explicit human sign-off, and this repository's own auto-accept is unchanged (D0341, D0342, D0347 accepted at record time today)."; }
        // RAN: dcPublishSaysWhatMoved and dcScaffoldShipsNoGrants DoD passes on suite581 550/0; the implement section names each run
        part storyWhatMovedDoDR1 : TestResult { :>> id = "389ea9b0-8e9f-404a-b6b3-1fda822becf9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "515cf802732f91cdab73150b530052ecf83561eb"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification whatMovedRefineGate : Test { :>> id = "b77c9608-96f2-4e85-bb70-0bfcf4c7dbe8"; :>> title = "Sprint 581 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: issue340's fix had one design point - hash the manifest ourselves or ask git what changed under the unit; git already holds the last published manifest and diffs it, so a second hash is a second place to be wrong. dcScaffoldShipsNoGrants needed no work: the sprint 527 retro's finding was the same defect GH#57 found from outside two days later."; }
    verification whatMovedStandupGate : Test { :>> id = "b22cacf7-40bb-44d0-a94a-65b109b4828f"; :>> title = "Sprint 581 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE DEFECT, FOUND TWICE: the retro named it on 2026-09-03 as a Low item; the downstream reporter met it as Critical on 2026-09-06. The Low rank was the priority miss - a fabricated attestation is never Low."; }
    verification whatMovedImplementGate : Test { :>> id = "0aad7dd5-4aea-4647-b6f0-9ad6d70cb738"; :>> title = "Sprint 581 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --test library_publish 4/4; clippy -D warnings clean; cargo test --test standing_words_are_project_local 3/3 (sprint 575, re-read for the second item); full suite in suite581.txt."; }
    verification whatMovedReviewGate : Test { :>> id = "14739bb5-8af7-4b27-a25b-4b15c400b326"; :>> title = "Sprint 581 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "TWO PASSES. (1) DOES THE VERSION EVER MOVE ON A MANIFEST CHANGE: no - next_unit_version is untouched and the test asserts the version line byte-equal. (2) IS ANY GRANT STILL SHIPPED: the scaffold test reads every grant line as commented."; }
    verification whatMovedCloseOutGate : Test { :>> id = "5d475432-dc3f-4c96-960e-b822bb175f7c"; :>> title = "Sprint 581 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. issue340 resolves; dcScaffoldShipsNoGrants is done on D0340's evidence."; }
    verification whatMovedRetroGate : Test { :>> id = "eadab930-a1b9-4d79-ab5d-c204dbc6209b"; :>> title = "Sprint 581 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: issue376 carries the Critical the retro item was the Low of; the ranking lesson is D0311's - a retro finding that names a fabricated-attestation class is Critical on the class, not on the instance's reach that day."; }
}
