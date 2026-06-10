// =====================================================================
// EngineProcess — the ability to MODEL a procedure (a Process of ordered
// ProcessSteps). This is the procedure INSIDE a workflow Phase (esp. the
// Delivery loop). Specific processes are content under processes/.
// (Reserved words renamed: action -> actionText, output -> producedArtifact,
//  trigger -> triggerCondition.)
// =====================================================================
package EngineProcess {
    private import ScalarValues::*;
    private import EngineElement::*;

    part def Process :> Element {
        attribute purpose : String;
        attribute cadence : String;        // continuous | event-driven | per-item | per-sprint | ad-hoc
    }

    part def ProcessStep :> Element {
        attribute order : Integer;
        attribute triggerCondition : String;   // when this step fires
        attribute actionText : String;         // what is done
        attribute producedArtifact : String;   // the tracked artifact change produced
        attribute owner : String;              // human | ai | either
    }
}
