// =====================================================================
// EngineNeeds — the Business-layer item types (the "what / why").
// Brief, Persona, Need, UseCase. See the design spec §5.
//
// Need collapses "market requirement" into one type distinguished by
// `source` (customer|operator|internal|competitive|regulatory|safety).
// There is NO Constraint type: scope is expressed as Decisions that
// `supersede` Needs (see EngineWork::Decision + the supersede edge).
// A Safety Goal is a Need with source = "safety".
// Scenarios are sub-elements of a UseCase (main success + extensions),
// not a separate type.
// =====================================================================
package EngineNeeds {
    private import ScalarValues::*;
    private import EngineElement::*;

    part def Brief :> Element {
        attribute problem : String;
        attribute opportunity : String;
        attribute constraintsNote : String[0..1];   // free context; hard constraints are Needs(source) or Decisions
    }

    part def Persona :> Element {
        attribute description : String;
        attribute goals : String[*];                 // stable, solution-free goals (Cooper)
    }

    part def Need :> Element {
        attribute source : String;                   // customer|operator|internal|competitive|regulatory|safety
        attribute statement : String;                // solution-free desired outcome (JTBD)
        attribute marketJustification : String[0..1]; // opportunity / competitive rationale
        attribute priority : String;                 // MoSCoW
    }

    part def UseCase :> Element {
        attribute description : String;
        attribute mainSuccess : String;              // main success scenario
        attribute extensions : String[*];            // alternate / failure scenarios
    }
}
