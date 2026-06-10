// =====================================================================
// EngineRelationships — the engine edge algebra as metadata markers.
//
// Because the engine's tracked items are parts (:> Element), edges are
// realized as metadata markers applied to a `dependency` at instance time
// (there is no `dependency def`). Markers:
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
