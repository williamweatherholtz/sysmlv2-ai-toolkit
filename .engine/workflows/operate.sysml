// =====================================================================
// OperateWorkflow — field support, as a native action behavior. Incidents feed
// back as new work via Change Request. See spec §9.
// =====================================================================
package OperateWorkflow {
    private import EngineWork::*;           // Issue

    action def Operate {
        action support { out iss : Issue; }
        action triage { in iss : Issue; }

        first support then triage;

        flow from support.iss to triage.iss;
    }
}
