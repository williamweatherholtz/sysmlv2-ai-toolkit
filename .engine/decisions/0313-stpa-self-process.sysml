// D0313 (status: the `status` field below is the truth) — stpaSelf: STPA on keel's own workflow is a process, run from the computed control structure, with the structure itself as the re-run trigger
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0313Accept event on sign-off.
package Decision0313 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0313 : Decision {
        :>> id = "6f44bdfb-1127-4966-8053-529d44e1e509";
        :>> title = "stpaSelf: STPA on keel's own workflow is a process, run from the computed control structure, with the structure itself as the re-run trigger";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human paired the verification revamp with a strong STPA process to find control gaps, given how much autonomy the AI carries (st065, 2026-09-02), and said keel itself had no defined system to analyse (st066). Since then the system exists: losses and hazards EL/EHZ1-9 (engine-safety.sysml), a COMPUTED control structure (D0284, 43 actions today) and its diagram (D0285). What did not exist was the analysis step that turns that structure into constraints - D0040 says a recurring task without a process leaks into conversation, and this one recurs every time the structure grows.";
        :>> decision = "The process `stpaSelf` (.engine/processes/stpa-self.sysml, skill stpa-self) is adopted: stpa1 reads the losses and hazards; stpa2 reads `keel show control-structure` back and never draws one; stpa3 walks every computed control action through the four UCA types and records each hazardous one as an UnsafeControlAction edged to its hazard; stpa4 gives every UCA a ControllerConstraint edged to the control that enforces it or to an Issue naming the gap; stpa5 records the run as a Test whose procedureText opens `ANALYSED: <names>`, and the warning-tier guard `stpa-currency` (guard 62) names every computed action no recorded run has analysed, so the structure growing a name is the trigger to run again.";
        :>> rationale = "Why a process rather than a one-off analysis: the human asked for it as a process, and the structure it analyses is computed and changes with every new hook event, workflow step or write command - an analysis with no re-run trigger is a document. Why the trigger is a guard on the ANALYSED list: it is the one currency check a text model can compute (a name appearing that no run walked); a changed body behind an unchanged name is the stated residual, and the run record keeps that visible rather than claiming completeness. Why warning-tier (D0098): an unanalysed action is queued work, not dishonest state. Why the union over runs counts, not the latest run: an analysis of cmdRecord does not lapse because a later run walked cmdLand; runs accumulate the way test results do. Why local-only: the remote's branch-protection row is fetched with `gh`, and a commit gate must not need the network. FIRST RUN, 2026-09-05, cmdRecord and cmdLand: six UCAs, six constraints - three bound to controls (gAcceptanceBindsToText, ctlPrePushBehind), three bound to Issues (issue373 new - a fork without OPTION markers auto-accepts under standing consent; issue342 - consent scope; issue360 - suite receipt). The guard reports 40 of 42 local actions unanalysed, which is the truthful state and the queue.";
        :>> consequences = "Every future growth of the control structure warns on commit until a run covers it; the runs live in .tracking/architecture/engine-ucas.sysml. issue373 is High and its resolver dcForkShapeIsDetected is ranked above the Medium frontier (D0311). GUARD_NAMES is 62; the constraint def, guards.md row and control-map anchor gStpaCurrency (ehz7) land with this Decision. The companion fork D0312 (what a passing verification means, by authority) stays with the human.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0313Accept : Test { :>> id = "775672ec-2988-4c19-8f38-f76a693d5eb3"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0313AcceptR1 : TestResult { :>> id = "66cc60f2-9ce4-4c02-8e26-1c47e083ee1f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "494d293"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
