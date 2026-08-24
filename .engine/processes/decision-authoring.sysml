// =====================================================================
// EngineDecisionAuthoring — how a Decision is authored so each fact has ONE canonical home (D0058/issue058).
//
// Added 2026-07-05 (resolves issue058, human-adjudicated 'make it enforceable = a carried-out process').
// The dual-truth defect (a verdict restated as prose beside the discrete TestResult) recurred even after
// D0106, because the one-source rule was only guidance-in-a-skill. This makes it a DEFINED process, carried
// out by the decision-authoring skill (D0059) and backstopped by the decisionNoVerdictProseRule detecting
// rule. Deployed by the decision-authoring skill.
// =====================================================================
package EngineDecisionAuthoring {
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;
    private import EngineGuardConstraints::*;  // D0139(D): the guards this process brings

    action decisionAuthoring : Process {
        :>> id = "63940516-2c7d-4e8f-c108-516273849700";
        :>> title = "Decision Authoring (one source of truth)";
        :>> purpose = "Author every Decision so each fact has ONE canonical home — the why in its own fields, the verdict/acceptance in a discrete TestResult, status/state COMPUTED — never restated as prose in another field or a comment.";
        :>> cadence = Cadence::perItem;
        // NO `assert constraint` HERE, DELIBERATELY (D0223, human-directed).
        //
        // A guard is CORE — never deactivatable — precisely when NO unit claims it
        // (`activation::guard_state`: an unclaimed guard returns Core). Claiming a guard converts it
        // into that process's switchable property, which is the issue242 capture mechanism read
        // forwards. This process used to claim five:
        //
        //   decisionRationale · acceptanceEvents · decisionRequirementLink ·
        //   attestationSubstance · confirmationAuthenticity
        //
        // so `keel deactivate decision-authoring` switched all five off — including
        // confirmationAuthenticity, the guard that stops an AI recording a human acceptance THAT
        // NEVER HAPPENED. That put the monitor-modifiable-by-monitored gap (issue236/D0208) on the
        // most attestation-sensitive path in the engine: the one where the failure mode is a
        // fabricated human judgment, which §4 says must never be possible.
        //
        // Removing the claims makes all five CORE, and core is also more PORTABLE than switchable:
        // an adopting project gets them enforced with no activation step, because nothing there
        // claims them either. Defining and cementing a Decision is not an opt-in discipline — it is
        // how this engine records that a human agreed to something.
        //
        // Consequence, stated: a downstream project can no longer opt out of Decision-authoring
        // discipline even if it wants a lighter process. That is the intended trade.
    }

    action daFields : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849701";
        :>> title = "Author the Decision's own facts in their canonical fields";
        :>> triggerCondition = "A Decision is being recorded (CHANGE/RECORD, or `keel record decision`).";
        :>> actionText = "Fill context/decision/rationale/consequences with the decision's OWN what/why/impact only (Nygard ADR, D0058). Each is substantive (>=20 chars, guard:decision-rationale). Generate the UUID + next NNNN via `keel record decision` (never hand-number). Do NOT put another item's verdict, another decision's governance, or any computed/status fact in these prose fields.";
        :>> producedArtifact = "A Decision item whose prose fields carry only its own irreducible content.";
        :>> owner = Owner::ai;
    }
    action daVerdictIsAResult : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849702";
        :>> title = "Record acceptance/verdict as a discrete TestResult — never prose";
        :>> triggerCondition = "The Decision is accepted (or any verdict on it is recorded).";
        :>> actionText = "Acceptance is a method=confirmation verification + a passing TestResult carrying who/when/commit (D0016/D0066) — judged by a HUMAN Person (guard:confirmation-authenticity). NEVER restate the verdict/date/who as prose in a field or a '(ACCEPTED <date> by <who>)' comment (the issue058 dual-truth). The discrete TestResult is the SOLE source of the verdict.";
        :>> producedArtifact = "A method=confirmation acceptance Test + human-judged TestResult; no verdict prose anywhere else.";
        :>> owner = Owner::human;
    }
    action daStateIsComputed : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849703";
        :>> title = "Let state be computed — don't author derived facts";
        :>> triggerCondition = "Any status/coverage/suspicion/resolution fact about the Decision is wanted.";
        :>> actionText = "Read it, never store it: acceptance state, critique coverage, suspicion, and open/resolved are COMPUTED views (keel orient/decisions/coverage). The authored `status` field is the one materialized flag, guarded to match its acceptance event (proposed/accepted/superseded); everything else derivable is a #View, not a field or a prose note (§2.1).";
        :>> producedArtifact = "Computed state surfaced by the views; no dual-truth stored.";
        :>> owner = Owner::ai;
    }
    action daBackstop : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849704";
        :>> title = "Automated backstop — the detecting rule";
        :>> triggerCondition = "Before committing a Decision change.";
        :>> actionText = "Run `keel rules` — decisionNoVerdictProseRule flags a Decision that restates its acceptance verdict as prose (case-insensitive, all prose fields). Treat any flag as a defect to fix, not to ignore. (Warning-level per D0102 warning->hard precedent; promotable once proven low-noise.)";
        :>> producedArtifact = "A clean decisionNoVerdictProseRule (0 flags) confirming one-source authoring held.";
        :>> owner = Owner::ai;
    }

    // decisionAuthoring: step order, native succession (D0143 — was `order : Integer`)
    first daFields then daVerdictIsAResult;
    first daVerdictIsAResult then daStateIsComputed;
    first daStateIsComputed then daBackstop;
}
