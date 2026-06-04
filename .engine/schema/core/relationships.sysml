// =====================================================================
// Engine::Core::Relationships
// The relationship algebra + the one custom edge + the rule for which
// edges carry suspicion.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // The edge algebra. Five are NATIVE SysMLv2; one is ours.
    //
    //   :>           specialization  — type hierarchy AND requirement
    //                                   derivation (replaces v1
    //                                   derive/refine/trace)         NATIVE
    //   satisfy       design element fulfills a requirement          NATIVE
    //   verify        evidence checks an element. Validation is the
    //                 SAME edge pointing at a need / market-req /
    //                 use-case — distinguished by target type, by
    //                 query, NOT by a separate edge.                 NATIVE
    //   allocate      abstract cross-hierarchy association
    //                 (function -> component)                        NATIVE
    //   dependency    generic / ordering dependence                 NATIVE
    //   supersede     replacement / obsolescence                    CUSTOM (below)
    // -----------------------------------------------------------------

    // The one deliberate custom edge. Realized as a typed dependency.
    // Replaces the hand-set `supersededBy` string attribute pattern:
    // the back-reference is COMPUTED, not authored on both ends.
    dependency def Supersede {
        doc /* `supersede` from the new element to the one it replaces.
               e.g. dependency d2_supersedes_d1 : Supersede
                        { :>> source = decisionV2; :>> target = decisionV1; }
               The replaced element's "supersededBy" is computed by the
               query layer from the inverse of this edge. */
    }

    // -----------------------------------------------------------------
    // SUSPICION-CARRYING EDGES (see contracts/computed-state.md).
    //
    // When an upstream element materially changes, suspicion propagates
    // DOWNSTREAM along these edge kinds only:
    //     :>  (specialization/derivation), satisfy, verify, allocate
    //
    // It does NOT propagate along:
    //     dependency used purely for ORDERING (B-after-A), because
    //     ordering is not semantic dependence.
    //
    // `dependency` is therefore ambiguous: tag ordering-only
    // dependencies so the suspicion engine can exclude them.
    // -----------------------------------------------------------------
    metadata def OrderingOnly {
        doc /* Annotate a `dependency` with @OrderingOnly to exclude it
               from suspicion propagation. Untagged semantic
               dependencies DO carry suspicion. */
    }

}
}
