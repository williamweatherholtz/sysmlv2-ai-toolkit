// ProjectDeliveryS569 — Sprint 569: recall travels with the engine - verified on a fresh scaffold, and the knowledge-graph-memory skill now says which steps are the engine's. EXECUTE. 1 pts.
package ProjectDeliveryS569 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from kgTravelsStory to d0332;

    part kgTravelsStory : Story {
        :>> id = "45c160c2-cc83-45b0-bd31-b68ba0e46c05";
        :>> title = "Sprint 569: recall travels with the engine - verified on a fresh scaffold, and the knowledge-graph-memory skill now says which steps are the engine's (1 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS569 {
        action storyKgTravels;
        verification storyKgTravelsDoD : Test { :>> id = "51d7438c-aa0f-4e6d-a3a5-3c5c75738dc0"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: kgTravelsWithTheEngine (D0332). DONE: a fresh keel init scaffold has the UserPromptSubmit hook wired, ships no .knowledge/ (so none of this repository's questions travel), recalls over its own 326 items with an empty .knowledge/, and sync-claude --check is clean; the skill gains a keel-project section stating which of its eight steps the engine performs and which two (.knowledge/questions, .knowledge/lexicon) are optional."; }
        // RAN: same suite receipt: 526/0 exit 0; scaffold checks as listed
        part storyKgTravelsDoDR1 : TestResult { :>> id = "247c97f0-f979-41c4-ac4d-3d2f2962fa83"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "18270a81975f4de36c4a5f707b85babe8a4f0b9e"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification kgTravelsRefineGate : Test { :>> id = "12c72ee2-783d-4a39-a6b0-13776371b912"; :>> title = "Sprint 569 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the DoD's four VERIFY clauses were each a command on a scaffold, and all four held before any change - the mechanism was already the engine's since D0243; the missing half was the skill saying so."; }
    verification kgTravelsStandupGate : Test { :>> id = "7b759f51-735e-4f21-8059-30b7cb7960ff"; :>> title = "Sprint 569 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE THING NOT TO SHIP: .knowledge/ is instance data and stays out of init by construction; the check that none of this repository's questions reached the scaffold is what the issue243 class needs."; }
    verification kgTravelsImplementGate : Test { :>> id = "64453422-9417-4956-9220-dccf9fba100b"; :>> title = "Sprint 569 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: keel init C:/t/kgfresh; grep UserPromptSubmit settings.json (1); ls .knowledge (absent); git init + commit; keel recall on the scaffold (326 items, answers); keel sync-claude --check clean; skill section added; cargo test --release 526/0; clippy clean."; }
    verification kgTravelsReviewGate : Test { :>> id = "4aee57de-ac04-460b-ab41-a2864037546d"; :>> title = "Sprint 569 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS THE SKILL TABLE TRUE: each engine row names the Decision or file that does the work; each optional row names the command that reads it. (2) IS ANYTHING SHIPPED THAT SHOULD NOT BE: no .knowledge/ in the scaffold. (3) IS THERE NEW MECHANISM: none - stated in D0332."; }
    verification kgTravelsCloseOutGate : Test { :>> id = "815733ce-01d8-4c18-9957-cdcd169c36f0"; :>> title = "Sprint 569 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0332 accepted under standing consent; kgTravelsWithTheEngine done on scaffold evidence."; }
    verification kgTravelsRetroGate : Test { :>> id = "d38fe041-5163-4701-b29a-3eab6e0c2d6a"; :>> title = "Sprint 569 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the recall ranker's own gaps are recallRanksLinkedRecordsAsWellAsGrepDoes (issue367, open FAIL) and d0325 records the measurements; a downstream project's own bench would be the same recall_bench.py pointed at its model, which D0332 leaves to the project. The lesson: when a DoD's clauses are commands, run them first - the work may already be the engine's."; }
}
