// =====================================================================
// ChangeRequestWorkflow — controlled, human-gated change, as a native action
// behavior: propose -> research alternatives -> resolve (accept/reject). The
// human-accept gate WILL be authored as verify-linked instance data when this
// workflow is first enacted (none exists yet). See
// spec §10. (action def named ChangeMgmt to avoid clashing with item ChangeRequest.)
// =====================================================================
package ChangeRequestWorkflow {
    private import EngineWork::*;           // ChangeRequest, Decision

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
