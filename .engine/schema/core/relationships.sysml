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
//   allocate  -> NATIVE `allocate <fn> to <comp>` (parses - _spike_remaining; the
//                old #Allocate marker was redundant and is gone).
//   dependsOn / supersede -> metadata markers below (no native pilot form;
//                supersede's back-reference is COMPUTED).
//
// SUSPICION propagates DOWNSTREAM along satisfy, verify, :>, allocate, and
// semantic dependsOn — but NOT along an ordering-only dependency. Tag those
// with #OrderingOnly to exclude them (see contracts/computed-state.md).
// =====================================================================
package EngineRelationships {
    doc /* The edge algebra: native satisfy/verify/allocate/:> where the pilot supports them; metadata markers (#DependsOn, #Supersede, #OrderingOnly) only where it does not. Suspicion travels semantic edges only. */


    // #Satisfy / #Verify / #Allocate markers dropped — those edges are native.
    metadata def DependsOn;
    metadata def Supersede;
    metadata def OrderingOnly;
    // #CharteredBy: work item -> the originating item (Decision / Need / Requirement) that
    // chartered it (D0068, charter-lineage governance). Authored as a marked dependency:
    // `#CharteredBy dependency from <workItem> to <charter>;`. Carries process IDENTITY by
    // lineage; the governing process VERSION is COMPUTED as-of the charter's creation commit.
    metadata def CharteredBy;

    // #ProspectiveChange / #SafetyChange: a process-change Decision -> the Process MODULE it
    // changed (D0068/D0069, process-governance). The class encodes RETROACTIVITY: prospective =
    // outputs chartered under the prior version stay valid (accept-what-satisfied-the-then-process,
    // D0062); safety = downstream items under the superseded version are mandatory reprocess
    // candidates. The effective commit = that Decision's acceptance-event judgedAgainst (no new
    // field); the governing version is COMPUTED by git-traversal (D0069). Authored as a marked
    // dependency: `#ProspectiveChange dependency from <dNNNN> to <ProcessModule>;`.
    // HOME: these edges live in .tracking (rust-validated, name-based) — NOT inside the kernel-
    // validated Decision files, because validate_instances loads each .engine file in isolation
    // (schema/core only) and cannot resolve a cross-package target like Delivery. See issue017 +
    // sysmlv2-syntax-notes (cross-package refs must be authored where validation is generic).
    metadata def ProspectiveChange;
    metadata def SafetyChange;
}
