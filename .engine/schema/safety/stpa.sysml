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
    // A SENSOR IS A MEASURE, and it carries what a reader of its number cannot otherwise see (D0363).
    // Was `part def Sensor :> Element;` with no fields, so an instrument had no model identity and no
    // verify edge could reach it - the owner's correction, 2026-09-07: assessment is not metadata, it
    // is a verification case, and a verification case needs an endpoint.
    part def Sensor :> Element {
        attribute mechanism : String;              // what produces the feedback: a path, a command, a device
        attribute measures : String;               // the question it answers, in a reader's words
        attribute determinism : Determinism;       // whether ONE run of it is a measurement at all
        attribute reading : SensorReading;         // the form its answer takes
        ref producesFeedback : Feedback[0..1];
    }

    // Four kinds, because a single run of each means something different and nothing on the number
    // says which (issue392: two of four verdicts flipped between identical runs of a stochastic one).
    enum def Determinism {
        tree;          // a function of the tree: same tree, same answer
        seeded;        // random, but pinned - reproducible by anyone with the seed
        stochastic;    // a model in the loop; ONE RUN IS A SAMPLE, never a measurement
        external;      // an outside party answers, and can answer differently tomorrow
    }
    enum def SensorReading { number; verdict; figure; }

    part def Actuator :> Element;

    // THE FIFTH ELEMENT TYPE (STPA Handbook p.25): a control structure "contains at least five types
    // of elements: Controllers, Control Actions, Feedback, Other inputs to and outputs from components
    // (neither control nor feedback), Controlled processes". This schema had four of the five, and our
    // own SOP named four, so every run of the analysis produced four (issue394, D0362).
    //
    // It is not feedback - nobody controls it and it reports on nothing - and it is where "the
    // measurement was taken against the wrong thing" lives (scenario S-F9: an instrument reads the
    // working tree while the report cites a commit).
    part def OtherInputOutput :> Element {
        attribute direction : FlowDirection;
        attribute externalParty : String;          // what outside the boundary sends or receives it
        ref atController : Controller[0..1];
        ref atProcess : ControlledProcess[0..1];
    }
    enum def FlowDirection { inbound; outbound; }

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
