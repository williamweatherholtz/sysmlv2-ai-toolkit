// D0382 (status: the `status` field below is the truth) — investigationLane: repeated sampling should be an option - as an investigation an agent may run on a result, which is a different process from verification and produces findings, never a verdict
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0382Accept event on sign-off.
package Decision0382 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0382 : Decision {
        :>> id = "32fafa20-46b9-4cf0-ac87-5b091885f14e";
        :>> title = "investigationLane: repeated sampling should be an option - as an investigation an agent may run on a result, which is a different process from verification and produces findings, never a verdict";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Accepting D0381 on 2026-09-08 the human added: 'repeated sampling should be an option though. sometimes i want AI agents to investigate a result, but it's a different process - ultimately they can't really verify' (st112, us104). D0381 says an objective check runs once and a self-disagreeing check is never re-sampled to break the tie. Read alone that would remove the re-run as a tool altogether. Their note keeps it, and names what it is: an investigation the human may order, distinct from the verification the check performs. NOT A FORK: this records the qualification in their own words; it depends on D0381 and narrows nothing in it.";
        :>> decision = "Re-running a check on identical inputs is an INVESTIGATION, a process apart from verification. It is started by the human or by an Issue that names the check under suspicion, it may run as many samples as the investigation needs, and its output is FINDINGS - an Issue against the check, a critique of its rubric, a recorded observation of how the verdicts moved - never a TestResult, a pass rate, or a tie-broken verdict. No sample from an investigation is recorded as evidence that the checked claim holds, because the agents running it cannot verify; the verification remains the single run D0381 describes, and the human's word where the verdict is subjective.";
        :>> rationale = "The human's words: repeated sampling stays an option because they sometimes want agents to investigate a result, and it is a different process because ultimately the agents cannot verify. Keeping the two lanes apart is what lets the re-run exist without becoming the averaging D0381 rules out - an investigation that could write a pass would be the panel by another name. This is a process-change: it adds a lane to how results are examined, so under D0337 it is recorded on their quoted words and stands or falls on them.";
        :>> consequences = "The routing rig (dcRoutingNumberIsStable, D0378 C) keeps its repeated-run mode and names it an investigation: it reports per-case sample count and disagreement as findings and can write no TestResult and no standing number. Any future repeated-sample tool follows the same shape - a findings writer, never an evidence writer. Nothing existing is re-run to satisfy this.";
    }
    #DerivedFrom dependency from d0382 to us104;
    // the qualification the human added while accepting D0381: the re-run survives as a different lane
    #DependsOn dependency from d0382 to d0381;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0382Accept : Test { :>> id = "923aedb3-5bc5-4bd8-95c6-61587f4d5183"; :>> method = VerificationMethod::confirmation; :>> procedureText = "recorded from chat - their words, verbatim: “repeated sampling should be an option though. sometimes i want AI agents to investigate a result, but it's a different process - ultimately they can't really verify”"; }
    part d0382AcceptR1 : TestResult { :>> id = "bd0773a5-f0fe-4cbe-aea1-15fd928a9451"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "eb4a14a"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
