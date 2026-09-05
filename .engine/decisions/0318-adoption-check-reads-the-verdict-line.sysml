// D0318 (status: the `status` field below is the truth) — adoptionCheckReadsTheVerdictLine: a foreign gate's failure is read from the shape of its verdict lines, never from a substring
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0318Accept event on sign-off.
package Decision0318 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0318 : Decision {
        :>> id = "ad8314a0-dc0b-4183-acfe-ae1359c23e47";
        :>> title = "adoptionCheckReadsTheVerdictLine: a foreign gate's failure is read from the shape of its verdict lines, never from a substring";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#45 (st084, issue350): on se-templates every unit reported FAIL and the reason printed was a guard PASS line. The 0.3.1 adoption-check reported the last four lines of the gate's output as the reason, and guard all prints every guard's verdict, so the tail was passes from guards that ran after the failure. The interim fix selected lines containing ERROR or FAIL - the issue329 class, a substring test on guard text, which selects a passing guard's warning that mentions a FAILED sprint gate and would select nothing if the wording moved.";
        :>> decision = "adoption-check reads a gate's failure by the SHAPE of the line, in a function with unit tests (adoption_check::failing_lines / line_is_failure): a guard verdict line `[guard:<name>] <VERDICT> ...` counts only when its verdict token is FAIL; a detail line counts only when its first token is ERROR (a violation, or a validate/check-engine error); a PASS or WARN verdict is never a reason whatever its text says; the `[guard] FAILED` total is a count and is left out; at most four, in output order, with the non-empty tail as the fallback only when no such line exists.";
        :>> rationale = "Why shape and not substring: the gate's verdict vocabulary is a fixed grammar (PASS/FAIL/WARN/ERROR as the first token after the bracket, D0287 colours them by that position), so the parse reads what the printer wrote, and a warning whose prose mentions FAIL is a passing guard talking - substring cannot tell them apart and the first vintage run proved it. Why the exit code still decides pass/fail: the gate's exit is the verdict; the parse only chooses which lines explain a non-zero exit, so a wording change can degrade the explanation but never flip a verdict. Arming (D0298): unit tests hold a PASS line mentioning FAIL out, a FAIL verdict and an ERROR detail in, the total out, and the order and cap.";
        :>> consequences = "A unit that is green reports PASS; a red one prints the failing check. issue350 is resolved by dcAdoptionCheckReportsTheRealVerdict. The GH#45 report itself came from 0.3.1, whose tail-of-output reason this replaces; the adoption-check --vintage lane (D0302) runs the old binary's gate and reads its output through this same parser.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0318Accept : Test { :>> id = "b67bee78-9d1b-4175-8d2c-dd9ebab59df7"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0318AcceptR1 : TestResult { :>> id = "2f8c14f2-405d-42c6-a44f-20539cf453bb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8886405"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
