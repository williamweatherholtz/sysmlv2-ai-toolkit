// D0350 (status: the `status` field below is the truth) — redYieldKeepsTheDiagnostic: the red-yield obligation keeps the first problem whole, or cuts it on a line boundary and counts the lines it left out
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0350Accept event on sign-off.
package Decision0350 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0350 : Decision {
        :>> id = "2d14729b-0ddc-46eb-8bba-ed601c7f845f";
        :>> title = "redYieldKeepsTheDiagnostic: the red-yield obligation keeps the first problem whole, or cuts it on a line boundary and counts the lines it left out";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#50 (st079, routed under D0292; dcRedYieldKeepsTheDiagnostic). The Stop hook's second red pass yields to avoid a loop and records a tracked obligation whose description carries the first problem at yield - the one fact the record exists to preserve. It was built with chars().take(500), so three obligations each carried exactly 726 characters cut mid-token, and the diagnostic they existed to hold was lost at the cut.";
        :>> decision = "write::clip_at_line_boundary returns captured text whole when it fits the budget and otherwise the leading whole lines that fit followed by a (N more lines) marker; the first line is atomic and kept even when it alone exceeds the budget; the red-yield obligation uses it with a 2000-character budget in place of the character cut. NOT A FORK: the field's purpose fixes its shape.";
        :>> rationale = "Why a line and not a character: the gate's problems are line-structured - one violation per line - so a line boundary is the smallest cut that leaves every kept fact readable, and the marker says there was more. Why 2000: the three GH#50 obligations lost their first problem inside 500; the guard reports the hook captures run to a few hundred characters per violation, so 2000 holds several whole. Arming (D0298): a unit test - short text whole; a ten-line text cut to whole lines with (7 more lines); a 900-character first line kept whole with (1 more line).";
        :>> consequences = "A red-yield obligation reads as the gate printed it, up to a stated cut. dcRedYieldKeepsTheDiagnostic is done on the test above; sanitize_field still folds the recorded field to one line, so the boundary is preserved as whole clauses rather than line breaks.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0350Accept : Test { :>> id = "8bb06c35-5473-490f-a479-70ced9cf305e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0350AcceptR1 : TestResult { :>> id = "d0f98719-5050-4c1c-aa34-cd90da0413fe"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a66324c"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
