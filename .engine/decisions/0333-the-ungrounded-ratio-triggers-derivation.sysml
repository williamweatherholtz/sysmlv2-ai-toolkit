// D0333 (status: the `status` field below is the truth) — ungroundedRatioTriggers: the rootedness ungrounded ratio is a declared indicator whose declared trigger surfaces latent-need derivation in orient and indicators when crossed
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0333Accept event on sign-off.
package Decision0333 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0333 : Decision {
        :>> id = "bc556ba1-4a6d-49fc-9a4d-e0aaf73107a4";
        :>> title = "ungroundedRatioTriggers: the rootedness ungrounded ratio is a declared indicator whose declared trigger surfaces latent-need derivation in orient and indicators when crossed";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0249's unbuilt clause (dcUngroundedRatioTriggers): sprint 483's retro named the failure - the Need set was inverted relative to the code for months and NO VIEW SAID SO, because tier-satisfaction reported 100 percent while rootedness held the honest number (415 of 482 ungrounded) and nothing ranked or raised it. A metric only readable by someone who already suspects the answer is not a control (D0047). Verified 2026-08-30 and again today before the change: the ratio was absent from keel show indicators entirely. Today it reads 85 percent (486 of 569 delivery Stories chartered by a Decision that reaches no Need).";
        :>> decision = "The ratio becomes a computed Indicator, ungroundedRatioIndicator (collectionRef ungrounded_ratio_pct, goal minimize, informing d0249), and a new contract .engine/contracts/indicator-triggers.toml declares per-indicator thresholds (above, below) and the work crossing one surfaces; keel show indicators reports each indicator's trigger and a top-level `triggered` list, and keel orient's burndown carries `ungrounded_ratio_pct` and the same `triggers` so the frontier's own screen shows the derivation work when the ratio is past 50 percent; below the threshold nothing is surfaced. The Indicator schema is unchanged: the threshold lives in a contract, not on the frozen type, and the indicator stays an indicator (D0088) - crossing a trigger surfaces work, it gates nothing.";
        :>> rationale = "Why a trigger and not a guard: the ratio has no defensible good-enough boundary to fail a commit on - a project's early Decisions legitimately outrun its Needs - but there is a defensible point at which the Need set has stopped describing the work, and at that point someone should be shown the derivation task rather than left to suspect it; that is the indicator-to-trigger step D0088 allows without promotion. Why 50 percent: above half, the Needs describe less than the Decisions do; the tree sat at 86 percent through sprint 483 with nothing raised and reads 85 today, so the trigger fires here now, which is the honest state. Why the contract and not a schema attribute: the core is frozen (D0002) and a contract already carries this kind of project-declared fact (process-enforcement, control-arming); the schema question of a threshold on Indicator is a fork nobody has asked for. Why orient carries it: the burndown is the screen the ranked frontier is read from, and a trigger the reader must open another lens to see is the sprint 483 failure again. Arming (D0298): a unit test on the parse and both directions of crossing; an integration test on the binary - a fresh scaffold (ratio 0) surfaces nothing, three decision-chartered Stories (ratio 100) surface the derivation work by name.";
        :>> consequences = "keel orient shows the trigger on every read until the Need set catches up; that work is the business-elicitation process (D0216) deriving the Needs the decision-chartered Stories imply, which this Decision surfaces and does not schedule. dcUngroundedRatioTriggers is done; d0249's remaining clause is closed. The contract ships with keel init (a fresh project inherits the threshold and, with no delivery Stories, triggers nothing). STATED: other indicators may declare triggers in the same contract; none does yet.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0333Accept : Test { :>> id = "29d34c6c-4304-44c7-814c-4558a65427c8"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0333AcceptR1 : TestResult { :>> id = "ac5c3119-eb41-4037-995c-98522b781e8f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c81f82c"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
