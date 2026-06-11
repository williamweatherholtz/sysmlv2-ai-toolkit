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

    part def Process :> Element {
        attribute purpose : String;
        attribute cadence : Cadence;
    }

    part def ProcessStep :> Element {
        attribute order : Integer;
        attribute triggerCondition : String;   // when this step fires
        attribute actionText : String;         // what is done
        attribute producedArtifact : String;   // the tracked artifact change produced
        attribute owner : Owner;
    }
}
