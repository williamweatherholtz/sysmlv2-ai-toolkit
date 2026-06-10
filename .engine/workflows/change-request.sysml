// =====================================================================
// ChangeRequestWorkflow — the controlled, human-gated path for ANY change,
// including changes to the process-as-data itself. Triggerable from any
// phase, by a human or an AI. The human-accept gate is a subjective test.
// Spec §10. (During bootstrap the meta-process is frozen — CLAUDE.md §4 —
// so CR is not used to modify CR itself.)
// =====================================================================
package ChangeRequestWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    part crType : ArtifactType { :>> title = "Change Request"; :>> schemaType = "ChangeRequest"; :>> nature = "real"; }
    part decisionType : ArtifactType { :>> title = "Decision"; :>> schemaType = "Decision"; :>> nature = "real"; }

    part chkAccept : GateCheck {
        :>> title = "Human acceptance"; :>> criterion = "a human accepts the change (subjective test)";
    }
    part acceptGate : Gate { :>> title = "Human-accept gate"; :>> kind = "exit"; :>> checks = chkAccept; }

    part cDevilsAdvocate : CriticBinding { :>> critic = "devils-advocate"; :>> severityThreshold = "low"; }

    // ---- Phases: propose -> research alternatives -> accept/reject ----
    part propose : Phase { :>> title = "Propose"; :>> order = 1; :>> produces = crType; }
    part research : Phase { :>> title = "Research alternatives"; :>> order = 2; :>> critics = cDevilsAdvocate; }
    part decidePhase : Phase {
        :>> title = "Accept or reject"; :>> order = 3; :>> produces = decisionType; :>> exitGate = acceptGate;
    }

    part changeRequest : Workflow {
        :>> title = "Change Request";
        :>> purpose = "Controlled, human-gated path for any change, including to the process itself.";
        :>> cadence = "event-driven";
        :>> phases = (propose, research, decidePhase);
    }
}
