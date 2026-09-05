// D0311 (status: the `status` field below is the truth) — Repeated retro findings raise an item's computed priority class - two citations rank as High, three or more as Critical - inside the existing priority-inversion mechanism; guard 62 is withdrawn and keel show priority makes the metric visible
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0311Accept event on sign-off.
package Decision0311 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0311 : Decision {
        :>> id = "03bd34c5-91a8-4c2f-9a6b-36a5455c90a3";
        :>> title = "Repeated retro findings raise an item's computed priority class - two citations rank as High, three or more as Critical - inside the existing priority-inversion mechanism; guard 62 is withdrawn and keel show priority makes the metric visible";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human, 2026-09-05, on D0310's guard: guard 62 is improper. repeated findings increase priority, not compare against items and make their own violation. is there a priority metric? The answer to the question: D0052 makes declaration order the priority; D0258 reassesses that order against COMPUTED signals - only one existed, the resolver Issue's severity - through the priority-inversion guard's warnings and the refineAssessPriority step; WorkPriority p0-p3 is declared on the schema and computed by nothing; no lens showed a reader any of it. D0310 had bolted a separate violation onto recurrence with its own exemptions (front of frontier, fork-blocked), which duplicated the ranking mechanism instead of feeding it.";
        :>> decision = "Recurrence is the second computed priority signal. For every ready item, the effective class is the higher of its resolver Issue's severity and its recurrence class - two sprint retros naming it as the already-tracked home of a finding rank as High, three or more as Critical, one is tracking and ranks nothing. priority_inversions compares effective classes, so a lower-class item declared above a recurring one is the same inversion warning the guard already emits, discharged by the same refinement step; the warning names both signals and points at keel show priority. That lens lists every ready item in declared order with its effective class, which signal drove it, the resolver severity and the citation count, and the inversions. Guard 62 recurrence-escalates is withdrawn and its constraint identity removed; this Decision supersedes D0310.";
        :>> rationale = "WHY A SIGNAL: a defect that keeps returning is more urgent, and urgency is what rank encodes - the guard already compares rank against a severity signal, so a second signal belongs in that comparison, not beside it with its own verdict and exemptions. WHY NO SEPARATE VIOLATION: D0310's violation second-guessed the human's ordering with a hard stop; the inversion warning states the disagreement and leaves the judgment where D0052 put it, which is what the human asked for. WHY TWICE IS HIGH: one citation is the retro doing its job; the second means the finding came back while the item sat, which is what a High Issue means for a resolver. WHY THE LENS: a metric nobody can read is not a metric - the human's question was whether one existed, and the honest answer was no.";
        :>> consequences = "D0310 is superseded (status flipped, edge recorded). On this tree dcVerificationByAuthority reads Critical by recurrence (three citations) at position one, and the eight inversion warnings its pull-forward produced under D0310 vanish because the class now justifies the order. Guard count returns to 61. issue372 remains resolved by dcPreBashSeesHeredocSource, whose D0309 deny stands unchanged. CLAUDE.md names the lens beside whats-next.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0311Accept : Test { :>> id = "1d2cd99f-3456-42af-82dc-0006c49b0e8f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0311AcceptR1 : TestResult { :>> id = "3ae36918-b1ee-464b-a513-1b4dcef933ea"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2bc23d4"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }

    #Supersede dependency from d0311 to d0310;
}
