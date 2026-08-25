// D0226 (status: the `status` field below is the truth) — keelChartersItsOwnSet: this project adopts every declared process, because being its own test subject is what its evidence consists of
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0226Accept event on sign-off.
package Decision0226 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0226 : Decision {
        :>> id = "1fd1076a-3472-4c62-b900-cb4c00c5ff82";
        :>> title = "keelChartersItsOwnSet: this project adopts every declared process, because being its own test subject is what its evidence consists of";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0225 built onboarding, and `keel onboard` immediately reported this project NOT CHARTERED: all ten switchable processes are active, none is off, and no record anywhere says why that is the set. The first thing the new view found was that its own project had never made the choice it exists to elicit. Running the process on keel is also the only honest way to test it - a process whose first use is on someone else's repository has been tested by them, not by me.";
        :>> decision = "keel adopts EVERY declared process - all ten switchable ones active, and the fifteen that assert no guard applying unconditionally - and `.engine/contracts/activation.toml` names this Decision as `charteredBy`. The elicitation step was satisfied by the EXISTING intake corpus rather than a fresh dialogue: 18 recorded Statements in the author's own words, plus 225 Decisions, are a richer record of what this project is than any interview would produce today. That substitution is stated here rather than hidden, because the charter would otherwise imply a conversation that did not happen.";
        :>> rationale = "The condition is met for every process, and for one reason that is specific to this project rather than flattering to it: keel's evidence that a process WORKS is that keel runs it on itself. A process this project declines is a process nobody exercises, so it ships untested - which is the opposite of what a self-build is for. Checked per process rather than asserted in bulk: several conditions are met only because of what this project is - architectural-critique because the engine makes claims about itself that others will rely on; knowledge-graph-memory because the corpus is the model itself; distributed-collaboration because contributors are AI on separate machines. Others are met for ordinary reasons any project would share - issue-resolution, doc-sync, deploy. The honest caveat is that adopting everything is the DEFAULT, so this charter changes no switch; its value is that the set is now accounted for and a future removal has something to argue against.";
        :>> consequences = "This project can no longer treat any process as optional-by-neglect: every one of them is now a declared adoption with a stated reason, so dropping one requires a superseding Decision rather than an edit to a list. It also means keel is a poor demonstration of SELECTIVE adoption - everything is on, so the recommendation machinery is exercised here only in the direction of yes, and the first real test of a recommendation AGAINST will be the penumbra migration. That is a genuine gap in the evidence, named rather than papered over. Verified: `keel onboard` reports CHARTERED by d0226 across 25 processes with zero lacking a declared condition; `keel activation` reports the declared set unchanged; guards green.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0226Accept : Test { :>> id = "f9ae5b43-84bf-4525-96cf-06d61d084b55"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/24."; }
    part d0226AcceptR1 : TestResult { :>> id = "0b6a857c-50aa-4e88-91e3-eb340402d3f4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0440a49"; :>> judgedAt = "2026-08-25"; :>> judgedBy = "wweatherholtz"; }
}
