// =====================================================================
// EngineDocSync — keep documentation true when the model changes.
//
// Added 2026-06-11 (user request, with critique CR-9): documentation drift was a
// recorded HIGH finding — docs taught invalid syntax and contradicted git reality.
// This process TIES INTO ITEM CREATION: it fires whenever an item type, schema
// package, workflow, process, tool, or template is created or changed, and its
// product lands in the SAME commit as the change (drift never becomes a backlog).
// The Definition of Done's "Docs in sync" step references this process.
// =====================================================================
package EngineDocSync {
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;

    part docSync : Process {
        :>> id = "77891011-63dd-4792-80f4-1516a414d5a9";
        :>> title = "Documentation Sync";
        :>> purpose = "On every model/tool change, sweep the documentation set for claims the change invalidates; fix them in the same commit.";
        :>> cadence = Cadence::perItem;
    }

    part dsTrigger : ProcessStep {
        :>> id = "97ab88cb-ead1-48bf-bbb2-e1177288dfbf";
        :>> title = "Detect a doc-bearing change";
        :>> order = 1;
        :>> triggerCondition = "An item type, schema package, workflow, process, skill, tool, or template is created or changed (any CHANGE/§3a or BOOTSTRAP/§3e work).";
        :>> actionText = "Identify what the change makes true or false that documentation might state.";
        :>> producedArtifact = "List of potentially-invalidated claims.";
        :>> owner = Owner::ai;
    }
    part dsSweep : ProcessStep {
        :>> id = "4d0d3399-9b73-4c92-b6e2-5a1b350a0ff6";
        :>> title = "Sweep the documentation set";
        :>> order = 2;
        :>> actionText = "Check each member of the DOC SET for invalidated claims: CLAUDE.md; "
                       + ".engine/README.md; .engine/docs/usage.md; .engine/docs/keel-syntax-notes.md; "
                       + ".engine/docs/tracking-template.sysml; .tracking/README.md; "
                       + "validator READMEs; and comments ADJACENT to the changed code/model.";
        :>> producedArtifact = "Set of stale claims to fix.";
        :>> owner = Owner::ai;
    }
    part dsFix : ProcessStep {
        :>> id = "2db7b7ec-ae00-469d-817e-c873d44f1c71";
        :>> title = "Fix in the SAME commit";
        :>> order = 3;
        :>> actionText = "Correct every stale claim in the same commit as the change that "
                       + "invalidated it. Never narrate computed state into comments; delete "
                       + "rather than update status-narrating comments.";
        :>> producedArtifact = "Docs consistent with the change, atomically.";
        :>> owner = Owner::ai;
    }
    part dsVerify : ProcessStep {
        :>> id = "783ec8e3-2650-4053-a2e4-79d59e2e8b0f";
        :>> title = "Verify claims match reality";
        :>> order = 4;
        :>> actionText = "Spot-check: do commands in docs run? Do quoted syntaxes parse "
                       + "(validators green)? Do policy claims match git reality (e.g. "
                       + "tracked-vs-ignored, branch policy)?";
        :>> producedArtifact = "Verified-consistent documentation set.";
        :>> owner = Owner::either;
    }
}
