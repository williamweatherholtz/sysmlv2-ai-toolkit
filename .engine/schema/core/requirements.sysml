// =====================================================================
// EngineRequirements — the technical requirement cascade (Architecture layer).
// System -> Subsystem -> Component. See the design spec §6.
//
// Modeled as native `requirement def` (schemaAudit 2026-06-10): a requirement
// IS a requirement, so it uses the native metaclass for tool/visualizer interop.
// Because requirement def can't specialize part def Element, it specializes the
// per-metaclass TrackedRequirement base (EngineElement) for engine tracking.
// The engine still owns satisfy/verify/allocate as its own metadata edges
// (EngineRelationships), uniform across native + part types. Identity stays the
// authored `id` (native elementId is parse-ephemeral).
//
// NOTE vs the old taxonomy: "MarketRequirement" is gone (collapsed into
// EngineNeeds::Need, distinguished by source). "Constraint" is gone (scope is
// a Decision that supersedes a Need; technical givens are architecture
// Decisions). `kind` carries functional/non-functional/safety/etc.
// =====================================================================
package EngineRequirements {
    private import ScalarValues::*;
    private import EngineElement::*;

    abstract requirement def Requirement :> TrackedRequirement {
        attribute statement : String;               // EARS-formatted
        attribute rationale : String;
        attribute kind : String[0..1];               // functional|non-functional|safety|security|interface
        attribute safetyLevel : String[0..1];        // PLr / SIL / ASIL when safety-related
        attribute verificationMethod : String[*];    // inspect|analyze|demo|test (native) + confirmation (engine)
        attribute priority : String;                // MoSCoW
    }

    requirement def SystemRequirement :> Requirement;
    requirement def SubsystemRequirement :> Requirement;
    requirement def ComponentRequirement :> Requirement;

    // Assumption — taken as true without proof. First-class because when an
    // assumption BREAKS, everything derived from it goes suspect (a high-value
    // input to the suspicion engine).
    part def Assumption :> Element {
        attribute statement : String;
        attribute confidence : String;              // high | medium | low
        attribute validUntil : String[0..1];         // optional review date
    }
}
