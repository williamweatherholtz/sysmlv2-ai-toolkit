// =====================================================================
// OperateWorkflow — field support; incidents feed back as new Needs or
// Hazards through Change Request. Continuous. Spec §9.
// =====================================================================
package OperateWorkflow {
    private import ScalarValues::*;
    private import EngineWorkflow::*;

    part issueType : ArtifactType { :>> title = "Field Issue"; :>> schemaType = "Issue"; :>> nature = "real"; }

    part support : Phase { :>> title = "Support"; :>> order = 1; :>> produces = issueType; }
    part triage : Phase {
        :>> title = "Triage to change"; :>> order = 2;
        :>> procedureRef = "ChangeRequestWorkflow::changeRequest";
    }

    part operate : Workflow {
        :>> title = "Operate";
        :>> purpose = "Field support; incidents feed back as new Needs/Hazards via Change Request.";
        :>> cadence = "continuous";
        :>> phases = (support, triage);
    }
}
