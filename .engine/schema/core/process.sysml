// =====================================================================
// EngineProcess — the ability to MODEL a procedure (a Process of ordered
// ProcessSteps). This is the procedure INSIDE a workflow Phase (esp. the
// Delivery loop). Specific processes are content under processes/.
// (Reserved words renamed: action -> actionText, output -> producedArtifact,
//  trigger -> triggerCondition.)
// =====================================================================
package EngineProcess {
    doc /* Procedure modeling: Process + ordered ProcessSteps (the procedure inside a workflow phase). Step pass/fail is computed or human judgment per its evaluability label. */

    private import ScalarValues::*;
    private import EngineElement::*;

    // A procedure is BEHAVIOUR, so these are `action def` (D0143), specializing the per-metaclass
    // TrackedAction base for the same cross-metaclass reason the requirement/use-case/verification
    // bases exist.
    action def Process :> TrackedAction {
        attribute purpose : String;
        attribute cadence : Cadence;
    }

    // `order : Integer` is GONE: sequence is now native `first <step> then <step>` succession, which is
    // the construct SysML v2 provides for exactly this and which the engine already uses for sprint
    // runs and workflows. An integer sort key alongside a succession would be dual truth.
    action def ProcessStep :> TrackedAction {
        attribute triggerCondition : String;   // when this step fires
        attribute actionText : String;         // what is done
        attribute producedArtifact : String;   // the tracked artifact change produced
        attribute owner : Owner;
    }
}
