// ProjectDeliveryS566 — Sprint 566: recall reads the spellings people type, and pushes only on a distinctive word - two recall items measured on the eight-case set. EXECUTE. 3 pts.
package ProjectDeliveryS566 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from recallSpellingsStory to d0327;

    part recallSpellingsStory : Story {
        :>> id = "642d3274-458d-4d7a-9fdf-a9208f2bd958";
        :>> title = "Sprint 566: recall reads the spellings people type, and pushes only on a distinctive word - two recall items measured on the eight-case set (3 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS566 {
        action storyRecallSpellings;
        verification storyRecallSpellingsDoD : Test { :>> id = "7b98bc9b-0afe-4057-90f9-379ccc690cb9"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: identifierMatchingAcceptsNaturalSpellings (D0328; resolves issue297) and shortDomainWordsCanSeed (D0327; resolves issue294). DONE: join_natural_identifiers folds `issue 294`, `D-0242`, `decision 242`, `sprint 480` into the compact identifiers (sprints to the delivery run DeliveryRunSNNN, which the compact form never reached); a bare number or a year is never joined; pinned by unit test. The push is gated on DISTINCTIVENESS (a word in under 10 percent of the model, or an identifier), so `that this with` pushes nothing and says why while gate, hook, land, push, view seed on frequency; the filler list stays at 55 words. Eight-case set 5/8 (the DoD's floor, unchanged); 50-case set unchanged at 48/50 in 13 rows."; }
        // RAN: same suite receipt: 524/0 exit 0
        part storyRecallSpellingsDoDR1 : TestResult { :>> id = "f521c8e8-52ca-48ab-af43-6624e5ec7890"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "73ae2a232a67cb411194c79354aa763e64bf9d76"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification recallSpellingsRefineGate : Test { :>> id = "3de71d14-36a4-44f0-97e1-5274f77d70d2"; :>> title = "Sprint 566 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: both DoDs named their instrument (the eight-case comparison) and their premise was checked on the binary first - one premise was stale (the length floor was already three), which turned the item from a floor change into a filler-versus-frequency question."; }
    verification recallSpellingsStandupGate : Test { :>> id = "ce473113-29b0-4b77-af41-70654628b604"; :>> title = "Sprint 566 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THE FILLER LIST WAS THE WRONG TOOL, AND THE INSTRUMENT SAID SO: naming the three- and four-letter function words as filler read 3/8 (who is what d0219 is about), 4/8 without the question words (the and and shift a marginal ranking), and the distinctiveness floor read 5/8. Frequency decides, as it already does for weighting."; }
    verification recallSpellingsImplementGate : Test { :>> id = "c08aba4f-3270-4ca6-81c5-5d84b354b013"; :>> title = "Sprint 566 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: keel recall on the six identifier spellings before (three pushed nothing) and after (all six seed the same element, named); cargo test --lib knowledge::tests 7 passed; recall_ab.py 3/8, 4/8, 5/8 across the three variants; recall_bench.py 48/50 in 13 rows after; clippy -D warnings clean; the full suite in the background to completion (receipts on both DoD results)."; }
    verification recallSpellingsReviewGate : Test { :>> id = "9cabe4ec-f077-4ff2-9827-8e90dfe465a0"; :>> title = "Sprint 566 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) FALSE POSITIVES ON NUMBERS: a bare number and a year stay words, pinned; `issue 2026` joins and finds nothing, stated in D0328. (2) DOES THE FLOOR SILENCE REAL QUESTIONS: the eight-case set is unchanged and the 50-case set unchanged; a prompt with one word under 10 percent still pushes. (3) IS THE REJECTED PATH RECORDED: the three list variants and their scores sit in the source comment and in D0327."; }
    verification recallSpellingsCloseOutGate : Test { :>> id = "e2377bcb-ada0-4ac4-9fd6-e00608f2db48"; :>> title = "Sprint 566 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0327 and D0328 accepted under standing consent; issue294 and issue297 resolved by computation."; }
    verification recallSpellingsRetroGate : Test { :>> id = "1437ae78-3183-48e2-a98a-b2f37138faa6"; :>> title = "Sprint 566 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the hand set's three misses (d0129 twice, d0176) are recallRanksLinkedRecordsAsWellAsGrepDoes's open FAIL and unchanged by either item. The lesson: check a DoD's premise on the binary before building to it - the length floor it described no longer existed."; }
}
