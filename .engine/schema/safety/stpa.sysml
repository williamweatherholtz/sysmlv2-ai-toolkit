// =====================================================================
// Engine::Safety::Stpa
// STPA (System-Theoretic Process Analysis) element types.
// OPTIONAL MODULE — import only for safety-relevant projects.
// HARA and ASIL are intentionally OUT OF SCOPE (decision 0008).
// STPA/HARA artifacts are FIRST-CLASS (decision 0007): they have
// workflow state, participate in suspicion propagation, and their
// "verification" is computed analysis-completeness (see Gate rules).
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Safety {
    private import Engine::Core::*;

    // ---- Purpose layer ----------------------------------------------
    part def Loss {
        // @Tracked
        doc /* An unacceptable outcome: loss of life, injury, damage,
               mission failure. The top of the safety chain. */
        attribute statement : String;
    }

    part def Hazard {
        // @Tracked
        doc /* A system state/condition that, with worst-case
               environment, can lead to a Loss. Single shared concept
               (HARA dropped, so no STPA/HARA split needed). Links to
               Loss(es) via `dependency`; inverts to a
               SystemSafetyConstraint. */
        attribute statement : String;
    }

    part def SystemSafetyConstraint {
        // @Tracked
        doc /* The inverse of a Hazard — what the system must (not) do.
               Derives a SafetyRequirement (Core) via `:>`/`satisfy`. */
        attribute statement : String;
    }

    // ---- Control structure ------------------------------------------
    part def Controller {
        // @Tracked
        doc /* A decision-making entity issuing control actions from a
               process model + feedback. */
        attribute name : String;
    }

    part def ProcessModel {
        doc /* A Controller's internal belief about the controlled
               process state. An INADEQUATE process model is the most
               common UCA cause — hence first-class, not an attribute. */
        attribute beliefs : String;
    }

    part def ControlAction {
        // @Tracked
        doc /* A command a Controller issues (via an Actuator) to
               influence the ControlledProcess. */
        attribute name : String;
    }

    part def Feedback {
        doc /* Information returning to a Controller (via a Sensor)
               about process state. */
        attribute name : String;
    }

    part def ControlledProcess {
        // @Tracked
        attribute name : String;
    }

    part def Sensor {
        doc /* Conveys Feedback to a Controller. A distinct failure
               point (can report wrongly / late / not at all). */
        attribute name : String;
    }

    part def Actuator {
        doc /* Executes a ControlAction on the ControlledProcess. A
               distinct failure point. */
        attribute name : String;
    }

    // ---- Analytical outputs (the point of STPA) ---------------------
    part def UnsafeControlAction {
        // @Tracked
        doc /* A ControlAction unsafe in a given context. Four guideword
               classes via `ucaType`. Links to its ControlAction and to
               the Hazard(s) it can lead to. Derives a
               ControllerConstraint. */
        attribute ucaType : String;       // "provided" | "not-provided" | "wrong-timing-order" | "stopped-too-soon-applied-too-long"
        attribute context : String;
        attribute statement : String;
    }

    part def ControllerConstraint {
        // @Tracked
        doc /* A constraint on a Controller's behavior, derived from a
               UCA. Feeds a SafetyRequirement (Core). */
        attribute statement : String;
    }

    part def LossScenario {
        // @Tracked
        doc /* The causal story of HOW a UCA occurs (flawed process
               model, missing/delayed feedback, controller/actuator
               failure). Where mitigations & requirements come from. */
        attribute statement : String;
        attribute causalFactor : String;  // "process-model" | "feedback" | "control-path" | "execution" | ...
    }

    // -----------------------------------------------------------------
    // Completeness gate rules (evaluated by the query layer; this is
    // how an analysis artifact is "verified" — decision 0007). Examples:
    //   - every ControlAction has >=1 UCA assessment OR an
    //     explicit assessed-safe record (all four ucaTypes considered)
    //   - every UCA traces to >=1 Hazard
    //   - every Hazard traces to >=1 Loss
    //   - every UCA has >=1 LossScenario
    //   - every SystemSafetyConstraint/ControllerConstraint derives
    //     >=1 SafetyRequirement
    // These are authored as GateChecks in a safety Gate (processes/).
    // -----------------------------------------------------------------

}
}
