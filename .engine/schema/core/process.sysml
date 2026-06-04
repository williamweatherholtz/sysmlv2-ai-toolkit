// =====================================================================
// Engine::Core::Process
// The ability to MODEL a process. Specific processes (agile, DoD) are
// content under processes/ — this defines the vocabulary.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // Process — a named procedure made of ordered ProcessSteps.
    // `cadence` is intentionally a free string: "continuous",
    // "event-driven", "per-story", "per-sprint", "ad-hoc".
    // -----------------------------------------------------------------
    part def Process {
        // @Tracked
        attribute name : String;
        attribute purpose : String;
        attribute cadence : String;
    }

    part def ProcessStep {
        attribute name : String;
        attribute order : Integer;
        attribute trigger : String;       // when this step fires
        attribute action : String;        // what is done
        attribute output : String;        // the tracked artifact change produced
        attribute owner : String;         // "human" | "ai" | "either"
    }

}
}
