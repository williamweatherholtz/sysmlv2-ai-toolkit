// =====================================================================
// ChangeRequestWorkflow — controlled, human-gated change, as a native action
// behavior: propose -> research alternatives -> resolve (accept/reject). The
// human-accept "gate" is a verify-linked Test (added as instance data). See
// spec §10. (action def named ChangeMgmt to avoid clashing with item ChangeRequest.)
// =====================================================================
package ChangeRequestWorkflow {
    private import EngineArtifacts::*;

    action def ChangeMgmt {
        action propose { out cr : ChangeRequest; }
        action research { in cr : ChangeRequest; }
        action resolve { in cr : ChangeRequest; out d : Decision; }

        first propose then research;
        first research then resolve;

        flow from propose.cr to research.cr;
        flow from research.cr to resolve.cr;
    }
}
