// =====================================================================
// EngineRequirements — the technical requirement cascade (Architecture layer).
// System -> Subsystem -> Component. See the design spec §6.
//
// Modeled as parts :> Element (uniform identity + engine-owned edges) rather
// than native `requirement def`: a requirement def is a different metaclass
// that cannot specialize part def Element, and the engine owns satisfy/verify
// as its own typed edges (EngineRelationships) anyway.
//
// NOTE vs the old taxonomy: "MarketRequirement" is gone (collapsed into
// EngineNeeds::Need, distinguished by source). "Constraint" is gone (scope is
// a Decision that supersedes a Need; technical givens are architecture
// Decisions). `kind` carries functional/non-functional/safety/etc.
// =====================================================================
package EngineRequirements {
    private import ScalarValues::*;
    private import EngineElement::*;

    abstract part def Requirement :> Element {
        attribute statement : String;               // EARS-formatted
        attribute rationale : String;
        attribute kind : String[0..1];               // functional|non-functional|safety|security|interface
        attribute safetyLevel : String[0..1];        // PLr / SIL / ASIL when safety-related
        attribute verificationMethod : String[*];    // test | analysis | inspection | demonstration
        attribute priority : String;                // MoSCoW
    }

    part def SystemRequirement :> Requirement;
    part def SubsystemRequirement :> Requirement;
    part def ComponentRequirement :> Requirement;

    // Assumption — taken as true without proof. First-class because when an
    // assumption BREAKS, everything derived from it goes suspect (a high-value
    // input to the suspicion engine).
    part def Assumption :> Element {
        attribute statement : String;
        attribute confidence : String;              // high | medium | low
        attribute validUntil : String[0..1];         // optional review date
    }
}
