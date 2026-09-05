// ProjectDeliveryS558 — Sprint 558: adoption-check reads a foreign gate's verdict by the shape of the line, never by substring. EXECUTE. 2 pts.
package ProjectDeliveryS558 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from verdictLineStory to d0318;

    part verdictLineStory : Story {
        :>> id = "b353c16a-565e-4b73-930c-9a4d8a446653";
        :>> title = "Sprint 558: adoption-check reads a foreign gate's verdict by the shape of the line, never by substring (2 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS558 {
        action storyVerdictLine;
        verification storyVerdictLineDoD : Test { :>> id = "e3310f3d-b743-4514-86f7-a3010022c0d7"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcAdoptionCheckReportsTheRealVerdict (D0318; resolves issue350 / GH#45). DONE: adoption_check::failing_lines / line_is_failure - a `[guard:<name>]` line counts only when its verdict token is FAIL, a detail line only when its first token is ERROR, PASS and WARN never count whatever their text says, the `[guard] FAILED` total is left out, at most four in order, the non-empty tail only as fallback; the substring filter is gone; unit tests hold each rule."; }
        // RAN: same suite receipt: 506/0 exit 0
        part storyVerdictLineDoDR1 : TestResult { :>> id = "1ad746f9-b1ee-48be-91dc-418b23f30961"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "88864053792a741b681bcee2fe202d2d1b14dffe"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification verdictLineRefineGate : Test { :>> id = "8d2478a8-e0f5-4976-9b8f-e0c0f7810acd"; :>> title = "Sprint 558 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the exit code already decided pass/fail; only the explanation was wrong, and its wrongness had two generations - tail-of-output (the GH#45 report) and then contains(FAIL) (the issue329 class). The parse reads the grammar the printer writes (D0287 colours by the same position)."; }
    verification verdictLineStandupGate : Test { :>> id = "8b7e3b4c-61b2-44e2-8d7a-c499cab0f654"; :>> title = "Sprint 558 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE SCOPE LINE: the parse chooses which lines explain a non-zero exit; it never flips a verdict, so a wording change can only degrade the explanation. The --vintage lane reads the old binary's output through the same parser."; }
    verification verdictLineImplementGate : Test { :>> id = "6a5bbc2b-eaee-4fbf-ade3-ab1de8051c48"; :>> title = "Sprint 558 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --lib verdict_tests 2 passed (a PASS line mentioning FAIL out, a WARN out, the total out, a FAIL verdict in, an ERROR detail in, validate's ERROR: in, order and cap); clippy -D warnings clean; the full suite in the background to completion (receipt on the DoD result)."; }
    verification verdictLineReviewGate : Test { :>> id = "4bfcca4d-e987-412f-a2f7-bc0f8888e260"; :>> title = "Sprint 558 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS SHAPE ENOUGH: the verdict token sits in a fixed position the printer owns; the same position D0287 colours. (2) COULD A REAL FAILURE BE MISSED: a gate that fails with neither a FAIL verdict nor an ERROR line falls back to the tail, which is the pre-existing behaviour - degraded, never silent. (3) IS THE GH#45 CASE COVERED: the test's first case is that exact line shape."; }
    verification verdictLineCloseOutGate : Test { :>> id = "3c085f0f-dda0-4b82-b6d2-e4c3c74c124f"; :>> title = "Sprint 558 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0318 accepted under standing consent; issue350 resolved by computation."; }
    verification verdictLineRetroGate : Test { :>> id = "7457fd8d-7b06-45ff-914a-75d21ca5fb6c"; :>> title = "Sprint 558 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: this is the third issue329-class finding (a substring test on a control's text); dcRedYieldKeepsTheDiagnostic is the standing item about what a red run prints, and the lesson is the same - a control reads a grammar, not a word."; }
}
