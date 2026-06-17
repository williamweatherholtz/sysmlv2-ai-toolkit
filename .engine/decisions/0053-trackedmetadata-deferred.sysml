// =====================================================================
// D0053 — trackedMetadata-deferred: on investigation (schema unfrozen with human
// sign-off), the @Tracked migration is a multi-system change, not one schema edit.
// Deferred behind a proper replan; schema/core stays frozen. Accepted 2026-06-16
// by wweatherholtz (signed off the unfreeze; this records the scope finding + defer).
// =====================================================================
package Decision0053 {
    private import EngineWork::*;

    part d0053 : Decision {
        :>> id = "2d3e4f50-6172-423b-376f-1324a5b6c7e4";
        :>> title = "D0053: defer trackedMetadata — @Tracked migration is multi-system, not one schema edit; replan first";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human signed off temporarily unfreezing schema/core to collapse the four duplicated tracking bases (Element + TrackedRequirement/TrackedUseCase/TrackedVerification, each declaring id/title/createdAt/authoredBy) into a single @Tracked metadata annotation (U2, 'SPIKED GREEN 2026-06-11'). Investigation before editing found the real scope is far larger than the 5-pt estimate: (1) the duplication is FORCED by SysML metaclass rules (requirement/use-case/verification def cannot specialize part def Element), so it cannot be removed by inheritance; (2) collapsing to @Tracked requires rewriting EVERY instance's `:>> id/title/createdAt/authoredBy` (hundreds across .tracking + .engine) to `@Tracked { ... }` annotation syntax; (3) the Rust parser/indexer — now the AUTHORITY (D0048) — does not parse metadata-application on instances and would need a rewrite, re-suspecting the whole deliverable (D0050); (4) validate_actors.py reads `:>> authoredBy` and would break. The 'spike green' only proved @Tracked PARSES in the kernel, not that the full migration is safe.";
        :>> decisionText = "DEFERRED. schema/core is RE-FROZEN — no change made (the temporary unfreeze is unused; lower risk than a half-migration). trackedMetadata is gated behind a new trackedMetadataReplan task that must first scope the migration as a multi-sprint effort INCLUDING: Rust parser support for @Tracked metadata-application, an instance-migration tool (bulk :>> id -> @Tracked), validate_actors + kernel-validator updates, and a deliverable re-verify pass. Only after that replan + a fresh human sign-off does the migration proceed. The duplication is 4 small attribute blocks; the migration cost (all instances + parser rewrite + re-suspect) may exceed the benefit — the replan decides do-vs-drop. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "No schema change now; schema/core stays frozen. trackedMetadata leaves the ready frontier (blocked on trackedMetadataReplan). The auto-followed next item becomes specVersionRuntimeCheck. issue003 (conflicting Release types, relatedTask trackedMetadata) rides with the replan. This is the engine's 'capture decisions even when they cause no action' discipline: a scope finding that converts a queued task into a planned effort rather than a reckless autonomous migration of the just-promoted toolchain.";
    }
}
