// =====================================================================
// EngineRelationships — the engine edge algebra as metadata markers.
//
// The engine's tracked items are now MIXED metaclasses (native requirement /
// use case / verification def + part def). Edges are realized as uniform
// metadata markers applied to a `dependency` at instance time (there is no
// `dependency def`), independent of the target's metaclass — so one edge
// algebra spans natives and parts alike (native satisfy/verify also exist for
// the native types, but the markers keep the engine layer uniform). Markers:
//   #Satisfy   design element fulfills a requirement / Need
//   #Verify    evidence checks an element (validation is the SAME marker
//              pointing at a Need/UseCase — distinguished by target type)
//   #Allocate  abstract cross-hierarchy association (function -> component)
//   #Supersede replacement / obsolescence (back-reference is COMPUTED)
//   #DependsOn semantic dependence
//
// Plus :> (specialization/derivation) which is native.
//
// SUSPICION propagates DOWNSTREAM along :>, Satisfy, Verify, Allocate, and
// semantic DependsOn — but NOT along an ordering-only dependency. Tag those
// with #OrderingOnly to exclude them (see contracts/computed-state.md).
// =====================================================================
package EngineRelationships {

    metadata def Satisfy;
    metadata def Verify;
    metadata def Allocate;
    metadata def Supersede;
    metadata def DependsOn;
    metadata def OrderingOnly;
}
