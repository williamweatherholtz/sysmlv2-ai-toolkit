// =====================================================================
// ChangeRequestWorkflow — the controlled, human-gated path for ANY change,
// including changes to the process-as-data itself. Triggerable from any phase,
// by a human or an AI. The human-accept "gate" is a verify-linked Test with
// method = "inspection" (a subjective human check), added as instance data.
// See spec §10. (During bootstrap the meta-process is frozen — CLAUDE.md §4.)
// =====================================================================
package ChangeRequestWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    part crType : ArtifactType { :>> title = "Change Request"; :>> schemaType = "ChangeRequest"; :>> nature = "real"; }
    part decisionType : ArtifactType { :>> title = "Decision"; :>> schemaType = "Decision"; :>> nature = "real"; }

    part propose : Phase {
        :>> title = "Propose";
        :>> produces = crType;
    }
    part research : Phase {
        :>> title = "Research alternatives";
        :>> consumes = crType;
    }
    part decidePhase : Phase {
        :>> title = "Accept or reject";
        :>> consumes = crType;
        :>> produces = decisionType;
    }

    part changeRequest : Workflow {
        :>> title = "Change Request";
        :>> purpose = "Controlled, human-gated path for any change, including to the process itself.";
        :>> cadence = "event-driven";
        :>> phases = (propose, research, decidePhase);
    }
}
