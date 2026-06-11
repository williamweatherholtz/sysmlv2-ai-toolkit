// =====================================================================
// EngineRelationships — the engine edge algebra. NATIVE where the pilot
// supports it (nativeTrace 2026-06-10, user-agreed rescope); metadata markers
// only where it does not. Spiked: _spike_trace.py / _spike_verify.py.
//
//   satisfy   -> NATIVE `satisfy <req> by <element>` (renders SatisfyRequirementUsage)
//   verify    -> NATIVE, STRUCTURAL: a requirement-verification must be in the
//                `objective` of a verification case — `verification def V {
//                subject s; objective r; }`. (`verify X by Y` is NOT valid.)
//   derive/   -> use `:>` (specialization). The v1 keywords derive/refine/trace
//   refine/      are ABSENT from the v2 pilot grammar; revisit on a newer toolchain.
//   trace
//   allocate / dependsOn / supersede -> metadata markers below (no native pilot form;
//                supersede's back-reference is COMPUTED).
//
// SUSPICION propagates DOWNSTREAM along satisfy, verify, :>, allocate, and
// semantic dependsOn — but NOT along an ordering-only dependency. Tag those
// with #OrderingOnly to exclude them (see contracts/computed-state.md).
// =====================================================================
package EngineRelationships {

    // #Satisfy / #Verify markers dropped — those edges use the native forms above.
    metadata def Allocate;
    metadata def DependsOn;
    metadata def Supersede;
    metadata def OrderingOnly;
}
