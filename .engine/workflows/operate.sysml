// =====================================================================
// OperateWorkflow — field support; incidents feed back as new Needs/Hazards
// through Change Request. Continuous. See spec §9.
// =====================================================================
package OperateWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    part issueType : ArtifactType { :>> title = "Field Issue"; :>> schemaType = "Issue"; :>> nature = "real"; }

    part support : Phase {
        :>> title = "Support";
        :>> produces = issueType;
    }
    part triage : Phase {
        :>> title = "Triage to change";
        :>> consumes = issueType;
        :>> procedureRef = "ChangeRequestWorkflow::changeRequest";
    }

    part operate : Workflow {
        :>> title = "Operate";
        :>> purpose = "Field support; incidents feed back as new Needs/Hazards via Change Request.";
        :>> cadence = "continuous";
        :>> phases = (support, triage);
    }
}
