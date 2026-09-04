// D0295 (status: the `status` field below is the truth) — The write API locates blocks with a string- and comment-aware scanner: a result lands after a multi-line DoD closes, and a brace inside a DoD's text never moves the next task
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0295Accept event on sign-off.
package Decision0295 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0295 : Decision {
        :>> id = "bd433f0d-17c5-4e78-b381-6b85e8efcc3a";
        :>> title = "The write API locates blocks with a string- and comment-aware scanner: a result lands after a multi-line DoD closes, and a brace inside a DoD's text never moves the next task";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#48 (issue353): append-result inserted a TestResult after the line that OPENS the DoD's verification block - right only for the one-liners add-task writes; a human who reflowed a DoD over several lines got the result inside the block and an error naming the wrong construct. issue354, found while routing GH#48's own Issue: add-task refused 'action def not found' after a DoD string containing 'Test {' was recorded. Established against write.rs find_action_def_close: it counted every brace character, so the DoD's brace pushed the depth off by one. Reproduced on two file shapes with two symptoms: this repository's backlog - the def's close never found, refusal; a small fixture - the scan overshot to the package close and the next task was written at PACKAGE LEVEL outside its def, silently, while whats-next still listed it. The real parser (keel_parser) was never at fault; keel check and every view saw the def throughout. The write path had its own, weaker reading of the file.";
        :>> decision = "One scanner, braces_in, that ignores braces inside string literals and after a // comment marker, and block_close_line built on it: the line where the block opened at a given line returns to depth zero, a one-line block closing on its own line. find_action_def_close uses it (add-task, append-gate-result); find_result_insertion uses it to place a result after the DoD block's CLOSING line rather than its opening line. Four tests: a multi-line DoD takes its result after the block; a one-line DoD still takes it on the next line; a DoD containing a brace does not misplace the next task; the scanner's own string/comment cases. The live reproduction on this repository's backlog is gone: with the brace text restored, add-task lands the probe inside EngineBuild at line 560 of 562.";
        :>> rationale = "WHY ONE SCANNER: two symptoms, two call sites, one cause; a fix at either site alone would have left the other. WHY STRUCTURE-AWARE AND NOT 'don't write braces in DoDs': the rule that broke retro-backlog this morning (split on a word) is the same rule - a scan over text must respect the text's own structure, or every string is a hazard to the file around it. The write path is the one place a wrong guess corrupts the record it writes (issue325 class), which is why it ranks with the consent-integrity items. WHAT IS STATED PLAINLY: issue354's first description named 'a brace in a string' as the cause and was right about the trigger, wrong about the mechanism, and silent about the worse symptom; it is corrected in place, as the owner may.";
        :>> consequences = "issue353 and issue354 resolve. A downstream human can reflow a DoD without breaking the next result; a DoD may quote a block header. The same scanner is available to the other block-locating code in write.rs, which should adopt it as each is touched. REMOVAL PATH (D0163): revert to the character count and the four tests fail.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0295Accept : Test { :>> id = "41ebd1be-03a3-4e47-96ec-8b5aef4bfdd6"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0295AcceptR1 : TestResult { :>> id = "40d99349-9d83-4d8e-9976-6cb35a1fb99b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7c34bb7"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "wweatherholtz"; }
}
