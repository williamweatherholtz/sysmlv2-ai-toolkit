// ProjectDeliveryS549 — Sprint 549: the thing verified is the thing agreed - a done task's own criterion drift is suspect. EXECUTE. 2 pts.
package ProjectDeliveryS549 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from ownDoDDriftStory to d0307;

    part ownDoDDriftStory : Story {
        :>> id = "90811858-8f51-42b2-ab14-082085c81f98";
        :>> title = "Sprint 549: the thing verified is the thing agreed - a done task's own criterion drift is suspect (2 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS549 {
        action storyOwnDoDDrift;
        verification storyOwnDoDDriftDoD : Test { :>> id = "ad49f6c7-2eab-4210-b42f-bf5889ffb7ce"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcOwnDoDDriftIsSuspect (resolves issue343). DONE: criterion_suspects compares each done task's OWN DoD text at HEAD against its text at the result's judgedAgainst SHA, in the same batched blob fetch as the dependency comparison; the reason names the task, HEAD and the verified SHA and says OWN. Stated limit: a criterion recorded in the same commit as its result has no historical text and is not judged. VERIFY: the live experiment - widen dcVintageFixture's DoD, orient, revert - added exactly dcVintageFixture to the suspect set and removed it on revert; integration test own_dod_drift_is_suspect (git fixture: pass at c1, widen, suspect; restore, clean); orient timing 2.4s before, about 4s after, one batched cat-file; suite and clippy in the RAN line."; }
        // RAN: cargo test --release --no-fail-fast 484 passed 0 failed; clippy 0; live suspect-set diff +dcVintageFixture then -dcVintageFixture; orient 2.4s -> ~4s
        part storyOwnDoDDriftDoDR1 : TestResult { :>> id = "a97c875e-d1a3-4e30-8ad8-ab8551635ef3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b1a1f99"; :>> judgedAt = "2026-09-04"; :>> judgedBy = "claudeOpus5"; }
    }

    verification ownDoDDriftRefineGate : Test { :>> id = "f520df2f-30ea-4290-9f7a-f06ae0aea553"; :>> title = "Sprint 549 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY from the item's own MEASURED clause: the missing branch was named at the line, the mechanism to reuse was named, and the direction (trust-but-verify, DoD as gold standard) was the human's."; }
    verification ownDoDDriftStandupGate : Test { :>> id = "2f2bfc60-183e-437a-aa18-7df584f82d04"; :>> title = "Sprint 549 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE CHOICE: the own comparison rides the existing batch rather than a second git call per task - the DoD asked for the batch to stay batched and it does, at the cost of about five hundred more blobs in one process. Second: suspect, not a gate - the pass was true when judged; reverify is the gate that re-judges."; }
    verification ownDoDDriftImplementGate : Test { :>> id = "c9b5ace1-fe87-47eb-9347-f591b178a18f"; :>> title = "Sprint 549 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: the first live diff of suspect sets showed the whole pre-existing suspect population and looked like noise until diffed before/after - exactly one name moved; the reason text gained HEAD after the DoD was re-read (both SHAs); one clippy map_or_else; the fixture test needed the DoD committed BEFORE the result cites its SHA, which is the stated limit made concrete."; }
    verification ownDoDDriftReviewGate : Test { :>> id = "5ab344d8-4e8b-4a2f-9658-71fb817d31e9"; :>> title = "Sprint 549 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) FALSE SUSPICION: a task whose DoD was added with its result has no history and is silent - zero own-drift suspects on this tree today, which is consistent with that limit and with the fact that nobody edited a verified DoD; both readings are stated. (2) COST: 2.4 to about 4 seconds on orient; accepted in D0307 with the reason. (3) WHAT CLEARS IT: reverify re-running the declared test against the new words, or restoring them - same as dependency drift."; }
    verification ownDoDDriftCloseOutGate : Test { :>> id = "7ea6fb8f-4d1d-4e3f-af10-d873aef03176"; :>> title = "Sprint 549 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. issue343 resolves; D0307 records the meaning change of the suspect view."; }
    verification ownDoDDriftRetroGate : Test { :>> id = "f44033ec-6285-4a23-b1ea-5fd5aa5147b0"; :>> title = "Sprint 549 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the same-commit limit is the sibling issue341's too and is stated in d0307; the orient cost is stated and is dcOrientPerf's concern if it grows. The one lesson: a suspect-set diff is the only honest way to read a suspect experiment on a tree that already carries suspects - the first look at the raw list would have read as failure."; }
}
