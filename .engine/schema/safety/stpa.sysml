// =====================================================================
// EngineSafety — STPA (System-Theoretic Process Analysis) element types.
// OPTIONAL MODULE — import only for safety-relevant projects.
// HARA/ASIL scoring intentionally lightweight (decision 0008). STPA artifacts
// are first-class :> Element (decision 0007): they carry workflow state and
// participate in suspicion. A Safety Goal is NOT here — it is an
// EngineNeeds::Need with source = "safety" (design spec §11).
// =====================================================================
package EngineSafety {
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
    part def Controller :> Element { attribute name : String; }
    part def ProcessModel :> Element { attribute beliefs : String; }
    part def ControlAction :> Element { attribute name : String; }
    part def Feedback :> Element { attribute name : String; }
    part def ControlledProcess :> Element { attribute name : String; }
    part def Sensor :> Element { attribute name : String; }
    part def Actuator :> Element { attribute name : String; }

    // ---- Analytical outputs (STPA Steps 3-4) ------------------------
    part def UnsafeControlAction :> Element {
        attribute ucaType : String;            // provided | not-provided | wrong-timing-order | stopped-too-soon-applied-too-long
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
