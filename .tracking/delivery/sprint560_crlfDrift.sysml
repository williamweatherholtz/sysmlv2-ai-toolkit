// ProjectDeliveryS560 — Sprint 560: a Windows checkout of the claude surface is not drift - one comparison helper, line endings folded, nothing else. EXECUTE. 1 pts.
package ProjectDeliveryS560 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from crlfDriftStory to d0320;

    part crlfDriftStory : Story {
        :>> id = "5eeaca61-530b-4fb3-a0bd-ba454a5bb47b";
        :>> title = "Sprint 560: a Windows checkout of the claude surface is not drift - one comparison helper, line endings folded, nothing else (1 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS560 {
        action storyCrlfDrift;
        verification storyCrlfDriftDoD : Test { :>> id = "a25db51d-76df-46bd-8574-d4237be7473e"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcSurfaceDriftIgnoresLineEndings (D0320; resolves issue351 / GH#46). DONE: claude_surface::same_text folds CRLF to LF on both sides and nothing else; the output-style, per-skill and plugin comparisons in sync_claude all use it (the plugin one had normalised alone, which is how the defect existed). Unit test on same_text; a check-run test writes the surface, re-reads keel.md as CRLF (no drift), then changes one word (drift)."; }
        // RAN: same suite receipt: 512/0 exit 0
        part storyCrlfDriftDoDR1 : TestResult { :>> id = "397fc1af-6481-44da-8451-ec0f8735da22"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9e250e71ab615c2cac2723b71ffeabfad85ddb0"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification crlfDriftRefineGate : Test { :>> id = "c71e2b94-4890-4d05-9a72-634b2937305a"; :>> title = "Sprint 560 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the defect was three byte-equal comparisons of which one had already been fixed in isolation. One helper, three callers."; }
    verification crlfDriftStandupGate : Test { :>> id = "a162e4a1-3fcf-400d-9f0d-3597824b952a"; :>> title = "Sprint 560 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE BOUNDARY: only line endings are normalised. Whitespace or case folding would let real edits hide; a changed line count is drift and the test says so."; }
    verification crlfDriftImplementGate : Test { :>> id = "a18e8900-378c-4de1-94a5-a0ab704c6395"; :>> title = "Sprint 560 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --lib claude_surface 10 passed (incl. line_endings_are_not_drift_but_content_is and a_crlf_checkout_of_the_output_style_is_not_drift_but_a_changed_word_is); clippy -D warnings clean; the full suite in the background to completion (receipt on the DoD result)."; }
    verification crlfDriftReviewGate : Test { :>> id = "a99faf7f-4665-4340-8b95-ec28b85bf297"; :>> title = "Sprint 560 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) DOES THE WRITE PATH CHANGE: no - sync-claude still writes LF; only the check reads through the fold. (2) IS THE GUARD WEAKENED: a CRLF checkout was never a difference in content; every content difference still fails. (3) ARE ALL COMPARISONS COVERED: the three text comparisons route through same_text; settings.json is compared as parsed JSON and never saw the problem."; }
    verification crlfDriftCloseOutGate : Test { :>> id = "acf4123b-0250-4fb0-8426-c36be659be8e"; :>> title = "Sprint 560 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0320 accepted under standing consent; issue351 resolved by computation."; }
    verification crlfDriftRetroGate : Test { :>> id = "fa892b32-e942-4bde-9f5b-3a7a31bac1eb"; :>> title = "Sprint 560 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the three GH reports fixed today (GH#44, GH#46, GH#47) were all found by the human adopting keel on a second machine, which is the dcAdoptionCheckReportsTheRealVerdict / adoption-check --vintage lane's job to find first. The lesson: a check on a checkout property is a check on git, not on the content."; }
}
