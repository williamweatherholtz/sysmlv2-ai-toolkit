// D0381 (PROPOSED — NOT YET ACCEPTED) — verdictEconomy: an objective check runs once and deterministically, a check that disagrees with itself is a measurement defect to critique, and a genuinely subjective verdict is sampled to the human - never re-run by a panel of agents
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0381Accept event on sign-off.
package Decision0381 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0381 : Decision {
        :>> id = "de75efdc-d8ea-4e8e-bfdb-99f287c6318c";
        :>> title = "verdictEconomy: an objective check runs once and deterministically, a check that disagrees with itself is a measurement defect to critique, and a genuinely subjective verdict is sampled to the human - never re-run by a panel of agents";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "On 2026-09-08, reading the brief on the routing probe (issue392: two of four verdicts flipped between identical runs), the human wrote: 'some of these items are just poorly measured, and that the measurement itself should be critiqued. We don't really need a panel of agents to perform the same test, especially if that test is objective. Where it's not objective, I should probably be included in validating the result at some point, if it's genuinely subjective. Otherwise, I don't see a necessity to constantly be running agents to double check a subjective assertion (e.g. interpreting my text).' (st111, us100, us101). The brief had offered three shapes that all spend agent runs to average a noisy instrument - a repeated sweep, a repeated subset, or a rig - and they chose the rig (D0378 C) while naming the premise wrong. Today several controls answer a flaky verdict by sampling it more: the routing probe's proposed --samples, panel critiques that re-judge one another. NOT A FORK: this records the human's stated course on how verdicts are obtained; the shapes it rules out are the ones they ruled out.";
        :>> decision = "A check whose inputs are fixed and whose verdict should not depend on who runs it is OBJECTIVE and runs once, by a deterministic procedure where one exists and by a single agent following a written rubric where it does not. When such a check returns different verdicts on identical inputs, the disagreement is recorded as an Issue against the CHECK - its rubric, its prompt, its inputs - and is never averaged into a rate, sampled repeatedly, or handed to a second agent to break the tie. A verdict that is genuinely subjective - interpreting the human's words, judging whether a description is wider than they meant - is recorded once by the agent with the text that decided it quoted beside it, and a sample of such verdicts is brought to the human for validation through the existing pull-audit surfaces, not re-run by agents.";
        :>> rationale = "The human's words. A panel that re-runs an objective test buys agreement on noise, and a second agent judging an interpretation of the human's text is another guess about them, not evidence about them; the only party who can validate a subjective reading is the person whose words are read. Recording the disagreement as a defect of the instrument turns a flaky number into a tracked correction (D0047) instead of a budget line. This is a process-change: it fixes how the critique, routing and quality processes obtain verdicts, so under D0337 it waits for the human's word.";
        :>> consequences = "dcRoutingNumberIsStable builds no --samples averaging: the probe stays a diagnostic rig (D0378 C) whose run-to-run disagreement files an Issue against the rubric. dcRoutingVerdictQuotesItsEvidence makes each routing verdict quote the span that decided it (us103). Panel-style re-judging of one agent's verdict by more agents is not started anywhere new; existing panels are not re-run to satisfy this. Sitting review and the critique dispositions remain the human's pull-audit for the subjective sample (D0204), never scheduled.";
    }
    #DerivedFrom dependency from d0381 to us101;
}
