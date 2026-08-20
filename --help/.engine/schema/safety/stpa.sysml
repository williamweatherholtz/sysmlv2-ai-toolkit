// =====================================================================
// EngineSafety — STPA (System-Theoretic Process Analysis) element types.
// OPTIONAL MODULE — import only for safety-relevant projects.
// HARA/ASIL scoring intentionally lightweight (decision 0008). STPA artifacts
// are first-class :> Element (decision 0007): they carry workflow state and
// participate in suspicion. A Safety Goal is NOT here — it is an
// EngineNeeds::Need with source = "safety" (design spec §11).
// =====================================================================
package EngineSafety {
    doc /* STPA element types (optional module): losses/hazards/constraints, the typed control loop (Controller/ControlAction/Feedback/Process refs), UCAs, scenarios, SafetyRequirement. */

    private import ScalarValues::*;
    private import EngineElement::*;

    // ---- Purpose layer (STPA Step 1) --------------------------------
    part def Loss :> Element {
        attribute statement : String;          // an unacceptable outcome (top of the chain)
    }
    part def Hazard :> Element {
        attribute statement : String;          // system state that, worst-case, leads to a Loss
    }
    part def SystemSafetyConstraint :> Element {
        attribute statement : String;          // inverse of a Hazard; derives a SafetyRequirement
    }

    // ---- Control structure (STPA Step 2) ----------------------------
    // The control LOOP is structural (CR-12, critique M5): who issues what on
    // which process, and which feedback returns - typed refs, not a label inventory.
    // (Duplicate `name` attrs dropped: Element supplies `title`.)
    part def Controller :> Element;
    part def ProcessModel :> Element {
        attribute beliefs : String;
        ref heldBy : Controller[0..1];
    }
    part def ControlledProcess :> Element;
    part def ControlAction :> Element {
        ref issuedBy : Controller[0..1];
        ref actsOn : ControlledProcess[0..1];
    }
    part def Feedback :> Element {
        ref sensedFrom : ControlledProcess[0..1];
        ref reportsTo : Controller[0..1];
    }
    part def Sensor :> Element;
    part def Actuator :> Element;

    // ---- Analytical outputs (STPA Steps 3-4) ------------------------
    enum def UcaType { provided; notProvided; wrongTimingOrder; stoppedTooSoonAppliedTooLong; }

    part def UnsafeControlAction :> Element {
        attribute ucaType : UcaType;
        ref unsafeAction : ControlAction[0..1]; // the ControlAction made unsafe (`action` is reserved)
        attribute context : String;
        attribute statement : String;
    }
    part def ControllerConstraint :> Element {
        attribute statement : String;          // constraint on a Controller, derived from a UCA
    }
    part def LossScenario :> Element {
        attribute statement : String;
        attribute causalFactor : String;       // process-model | feedback | control-path | execution | ...
    }

    // ---- Safety requirement (allocated to the architecture) ---------
    part def SafetyRequirement :> Element {
        attribute statement : String;
        attribute safeState : String;
        attribute safetyFunction : String;
        attribute triggeringEvent : String;
        attribute responseTimeTarget : String[0..1];
        attribute safetyLevel : String[0..1];   // PLr / SIL / ASIL
    }
}
