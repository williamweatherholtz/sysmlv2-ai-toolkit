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

    part decisionAuthoring : Process {
        :>> id = "63940516-2c7d-4e8f-c108-516273849700";
        :>> title = "Decision Authoring (one source of truth)";
        :>> purpose = "Author every Decision so each fact has ONE canonical home — the why in its own fields, the verdict/acceptance in a discrete TestResult, status/state COMPUTED — never restated as prose in another field or a comment.";
        :>> cadence = Cadence::perItem;
    }

    part daFields : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849701";
        :>> title = "Author the Decision's own facts in their canonical fields";
        :>> order = 1;
        :>> triggerCondition = "A Decision is being recorded (CHANGE/RECORD, or `keel record decision`).";
        :>> actionText = "Fill context/decision/rationale/consequences with the decision's OWN what/why/impact only (Nygard ADR, D0058). Each is substantive (>=20 chars, guard:decision-rationale). Generate the UUID + next NNNN via `keel record decision` (never hand-number). Do NOT put another item's verdict, another decision's governance, or any computed/status fact in these prose fields.";
        :>> producedArtifact = "A Decision item whose prose fields carry only its own irreducible content.";
        :>> owner = Owner::ai;
    }
    part daVerdictIsAResult : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849702";
        :>> title = "Record acceptance/verdict as a discrete TestResult — never prose";
        :>> order = 2;
        :>> triggerCondition = "The Decision is accepted (or any verdict on it is recorded).";
        :>> actionText = "Acceptance is a method=confirmation verification + a passing TestResult carrying who/when/commit (D0016/D0066) — judged by a HUMAN Person (guard:confirmation-authenticity). NEVER restate the verdict/date/who as prose in a field or a '(ACCEPTED <date> by <who>)' comment (the issue058 dual-truth). The discrete TestResult is the SOLE source of the verdict.";
        :>> producedArtifact = "A method=confirmation acceptance Test + human-judged TestResult; no verdict prose anywhere else.";
        :>> owner = Owner::human;
    }
    part daStateIsComputed : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849703";
        :>> title = "Let state be computed — don't author derived facts";
        :>> order = 3;
        :>> triggerCondition = "Any status/coverage/suspicion/resolution fact about the Decision is wanted.";
        :>> actionText = "Read it, never store it: acceptance state, critique coverage, suspicion, and open/resolved are COMPUTED views (keel orient/decisions/coverage). The authored `status` field is the one materialized flag, guarded to match its acceptance event (proposed/accepted/superseded); everything else derivable is a #View, not a field or a prose note (§2.1).";
        :>> producedArtifact = "Computed state surfaced by the views; no dual-truth stored.";
        :>> owner = Owner::ai;
    }
    part daBackstop : ProcessStep {
        :>> id = "63940516-2c7d-4e8f-c108-516273849704";
        :>> title = "Automated backstop — the detecting rule";
        :>> order = 4;
        :>> triggerCondition = "Before committing a Decision change.";
        :>> actionText = "Run `keel rules` — decisionNoVerdictProseRule flags a Decision that restates its acceptance verdict as prose (case-insensitive, all prose fields). Treat any flag as a defect to fix, not to ignore. (Warning-level per D0102 warning->hard precedent; promotable once proven low-noise.)";
        :>> producedArtifact = "A clean decisionNoVerdictProseRule (0 flags) confirming one-source authoring held.";
        :>> owner = Owner::ai;
    }
}
