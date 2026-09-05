// ProjectDeliveryS562 — Sprint 562: a fork in substance is held - record decision reads the words that weigh, and standing consent no longer accepts a disguised fork on the spot. EXECUTE. 3 pts.
package ProjectDeliveryS562 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from forkShapeStory to d0322;

    part forkShapeStory : Story {
        :>> id = "2d06f75a-5cfa-492c-b9d8-fbfddf3ebb55";
        :>> title = "Sprint 562: a fork in substance is held - record decision reads the words that weigh, and standing consent no longer accepts a disguised fork on the spot (3 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS562 {
        action storyForkShape;
        verification storyForkShapeDoD : Test { :>> id = "9343d1ba-8d26-445d-92db-3b078cf3aa3f"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcForkShapeIsDetected (D0322; resolves issue373, the first Issue stpa-self run 1 raised). DONE: deck::fork_signals reads the decision text for eight whole-word signals (alternative, either, or-we-could, option, versus, trade-off, (a)/(b) enumeration, recommend); deck::disguised_fork holds at two distinct signals unless the text states NOT A FORK; record decision prints HELD naming the words and the two ways forward and writes no acceptance; the OPTION-marker path and the human's quoted-word path are unchanged. Unit tests on the reader; keel-cli/tests/disguised_fork_stays_proposed.rs on a real scaffold with one decider. CLAUDE.md §4 says so."; }
        // RAN: same suite receipt: 521/0 exit 0
        part storyForkShapeDoDR1 : TestResult { :>> id = "e31090f5-6326-4177-9035-6aecb49f0015"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "50175b68d8fb95f333b6d66c7806c580ab3d42b2"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification forkShapeRefineGate : Test { :>> id = "7becc9c8-87b3-4f62-be22-eaa83ea8ccdc"; :>> title = "Sprint 562 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY once measured: over 109 auto-accepted Decisions, four carry two signals (D0207, D0269, D0288 meta; D0292 the real class) and none carries three - so two is the threshold that catches the D0292 shape at a 3-in-109 false-positive rate the author answers in five words. Threshold set after the reading, not before (D0102)."; }
    verification forkShapeStandupGate : Test { :>> id = "6db92cc3-520f-4f9f-9da5-be9b94f5c068"; :>> title = "Sprint 562 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE THING THE CONTROL DID ON ITS FIRST LIVE USE: D0322's own text, written to describe the marker, carried the marker shape twice and was read as a FORK - recorded proposed, not accepted. Rewritten without the shape and re-recorded under the same number; the NOT A FORK escape then carried it through auto-acceptance as designed. The control bit its author first, which is the arming evidence D0298 asks for."; }
    verification forkShapeImplementGate : Test { :>> id = "a778e6b1-08c4-4385-8215-de79272bb312"; :>> title = "Sprint 562 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: python measurement over .engine/decisions (109 auto, 202 human, 9 forks; signal counts per class); cargo test --lib fork_shape 3 passed; cargo test --test disguised_fork_stays_proposed 2 passed (held with no AcceptR1; genuine decision auto-accepts); clippy -D warnings clean; the full suite in the background to completion (receipt on the DoD result)."; }
    verification forkShapeReviewGate : Test { :>> id = "84c710c4-e8fe-47d2-b1e1-157438f5657f"; :>> title = "Sprint 562 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) CAN THE AUTHOR BYPASS: yes, by writing NOT A FORK - a visible claim in the record a reader can dispute, the D0192 shape; or by weighing in words the reader does not know, the stated residual. (2) DOES IT WEAKEN ANYTHING: no acceptance path is removed; one acceptance nobody gave is withheld. (3) IS THE MEASUREMENT IN THE RECORD: the four names and the split are in D0322's context and rationale."; }
    verification forkShapeCloseOutGate : Test { :>> id = "c7811235-8f7f-4d59-b8a0-486dda7ae3c6"; :>> title = "Sprint 562 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0322 accepted under standing consent (via its own escape); issue373 resolved by computation; CC-R1's binding moves from the Issue to the control at the next stpa-self run."; }
    verification forkShapeRetroGate : Test { :>> id = "64332fc8-d324-4da9-b00b-f318e542322b"; :>> title = "Sprint 562 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the twin gap the same run found is issue374 (a receipt proves a sentence was typed), resolver dcExternalFactGate, next on the frontier by class. The lesson: a control that reads words must be measured against the corpus before its threshold is chosen, and then it should be tried on its own Decision."; }
}
