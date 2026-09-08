// D0383 (status: the `status` field below is the truth) — The STPA step-2 gate is a computed row set the self-run reads, not a sentence it asserts
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0383Accept event on sign-off.
package Decision0383 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0383 : Decision {
        :>> id = "676329f3-7a1f-475b-bdb0-9aba9c541457";
        :>> title = "The STPA step-2 gate is a computed row set the self-run reads, not a sentence it asserts";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0363 gave the schema the fifth element type (OtherInputOutput) and sprint 610 (issue394) made keel show control-structure decide the SOP's step-2 gate clause by clause: a stepTwoGate row per clause, holds true or false, with the evidence. Actuators and responsibilities are derived from the actions; process models and other inputs/outputs are authored and joined onto the roles; a role nothing wires sits in absentRoles. The stpa SOP and the stpa-self skill still told the run to argue step-2 completeness in prose.";
        :>> decision = "NOT A FORK: this records the process text that matches what the view now computes. In a keel self-run, STPA step 2 is complete when every stepTwoGate row of keel show control-structure holds; the stpa-self skill reads those rows and does not re-argue them, and when a row does not hold the run's first record is that clause. The stpa SOP names the computed gate for a keel self-run beside its prose gate for a hand-drawn structure.";
        :>> rationale = "A gate asserted in prose is a testimony; the same eight clauses computed from the model and the hook configuration are a receipt anyone can re-run. Leaving the SOP and skill text at the prose gate would have left two homes for one fact (D0105) and let the next run assert what it should read.";
        :>> consequences = "The stpa-self skill's stpa2 row and difference 1, and the stpa SOP's step-2 gate paragraph, name stepTwoGate, actuators, OtherInputOutput and absentRoles. A future clause added to the SOP's gate must be added to step_two_gate in control_structure.rs, or the two gates diverge - stpa-currency does not detect that.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0383Accept : Test { :>> id = "6dc0caa8-d0ba-45ed-ba85-dfb51b8a8cec"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Checked on the decision brief copy-out (seventh publish), pasted into chat 2026-09-08; no note added. - their words, verbatim: “Accept the gate rule: the step-2 gate is the computed stepTwoGate rows; the self-run reads them”"; }
    part d0383AcceptR1 : TestResult { :>> id = "218965e8-4474-4ec3-a7d2-75ccbeb90517"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b3ee47"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
