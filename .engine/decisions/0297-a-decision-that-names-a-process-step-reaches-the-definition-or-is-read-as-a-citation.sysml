// D0297 (status: the `status` field below is the truth) — A Decision that names a process step by identifier reaches the definition in the same commit, or the commit warns: guard decision-amends-process, warning-tier, with the unnamed shape stated as a residual
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0297Accept event on sign-off.
package Decision0297 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0297 : Decision {
        :>> id = "b8cdfd26-213d-41e3-863e-b79046193967";
        :>> title = "A Decision that names a process step by identifier reaches the definition in the same commit, or the commit warns: guard decision-amends-process, warning-tier, with the unnamed shape stated as a residual";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue298: D0243 changed what two steps of a process MEAN - the vocabulary directory optional, seeding corpus-derived - carried no marker and never touched the process file, so the keystone lock (which watches the FILE) never fired; the change was made around the control. D0244 wrote the rule - a design may amend a process, it may not quietly disagree with one - and carried D0243's amendment into the definition by hand (71b789b, marked). The backlog item asked for the forward control. MEASURED before designing it, over the 296 Decisions in this tree: 17 name a process step by exact identifier and 5 of those did not touch the step's file in their commit - D0242 among them, a real amendment - so an identifier anchor fires on about 2 per cent of Decisions with real amendments among the hits. 24 mention steps by NUMBER and two thirds of those are not amendments at all ('step 5 of the migration plan', 'STPA step 2', 'steps 4-7 fired'), so a number is not an anchor. D0243's own sentence - 'the source process's steps 1-2' - names nothing.";
        :>> decision = "Guard 59, decision-amends-process, WARNING-tier under D0102's promote-once-low-noise rule: for every staged Decision, every process it names by identifier - the Process action name, the file stem, or a ProcessStep name, matched as a whole identifier - whose definition file is not also staged yields one warning naming the Decision, the identifier and the file, and saying what to do in each case: if the Decision amends the step, edit the definition in the same commit; if it only cites the step, read past. The guard scans staged Decisions only, so it is legitimately empty between commits. The unnamed shape - a Decision that speaks of 'the process's steps' without an identifier - is NOT detected, and the guard's documentation and a test both say so.";
        :>> rationale = "WHY WARNING AND NOT VIOLATION: a violation would have blocked roughly half of the historical hits, which were citations, and a control that trains its actor to route around it is worse than none (the D0176 tier lesson). WHY IDENTIFIERS AND NOT NUMBERS: measured, above. WHY THE RESIDUAL IS STATED AND ARMED: D0243 is the motivating case and this guard would not have caught it; a guard whose documentation implies it closes the class that named it is the pass-at-zero hazard in prose form. The test that asserts the miss exists so that the day the residual is closed, the documentation is forced to change with it. WHAT CLOSES THE RESIDUAL: an #Amends edge a Decision carries to the process it changes, which the guard could then require the definition to acknowledge - a schema change, therefore a fork, therefore not this Decision.";
        :>> consequences = "processAmendmentsReachTheDefinition is delivered; issue298 resolves. guards.md gains the row with the measurement and the residual; the control map gains gDecisionAmendsProcess bound to the same hazard as process-change. The first commit carrying this guard is expected to warn on this very Decision, which names the process it is about without amending it - the receipt that the guard reads what it should. A fork for the #Amends edge may be raised when the unnamed shape recurs; it is not raised now on one case.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0297Accept : Test { :>> id = "887f93bb-ae28-404c-a304-0a586bce0984"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0297AcceptR1 : TestResult { :>> id = "6cce5800-ee30-4cfe-a01b-d65c1af10649"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "75941cc"; :>> judgedAt = "2026-09-04"; :>> judgedBy = "wweatherholtz"; }
}
