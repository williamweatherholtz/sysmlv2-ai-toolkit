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
    doc /* Technical requirement cascade (native requirement defs): System -> Subsystem -> Component, plus Assumption (suspicion input). Concrete requirements may carry native require/assume constraints. */

    private import ScalarValues::*;
    private import EngineElement::*;

    abstract requirement def Requirement :> TrackedRequirement {
        attribute statement : String;               // EARS-formatted
        attribute rationale : String;
        attribute kind : RequirementKind[0..1];
        attribute safetyLevel : String[0..1];        // PLr / SIL / ASIL when safety-related
        attribute verificationMethod : VerificationMethod[*];
        attribute priority : Priority;               // MoSCoW
    }

    requirement def SystemRequirement :> Requirement;
    requirement def SubsystemRequirement :> Requirement;
    requirement def ComponentRequirement :> Requirement;

    // CHECKABLE REQUIREMENTS (checkableReqs 2026-06-10): a concrete requirement MAY carry
    // native `require constraint { <predicate over its attributes/subject> }` and/or
    // `assume { ... }` to be machine-checkable (parses — see _spike_trace.py). Prefer this
    // over a prose `statement` when the condition is formalizable. EVALUATION is solver-
    // dependent (an external evaluator such as SysIDE walks `satisfy` edges and checks these);
    // the pilot kernel PARSES but does not evaluate. Authored at instance time, not on the
    // abstract type above.

    // Assumption — taken as true without proof. First-class because when an
    // assumption BREAKS, everything derived from it goes suspect (a high-value
    // input to the suspicion engine).
    enum def Confidence { high; medium; low; }

    part def Assumption :> Element {
        attribute statement : String;
        attribute confidence : Confidence;
        attribute validUntil : Timestamp[0..1];      // optional review date
    }
}
