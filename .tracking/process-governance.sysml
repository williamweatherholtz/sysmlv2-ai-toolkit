// =====================================================================
// ProcessGovernance — the process-change edge registry (D0068/D0069, Inc 2 / pglProcessRef).
//
// Each edge links a process-change Decision -> the Process MODULE it changed, classed by
// RETROACTIVITY:
//   #ProspectiveChange : outputs chartered under the PRIOR version stay valid (accept what
//                        satisfied the then-process; D0062). The default.
//   #SafetyChange      : downstream items under the superseded version are MANDATORY reprocess
//                        candidates (the change closes a safety/correctness gap).
// The effective commit of a change = that Decision's acceptance-event judgedAgainst (no new
// authored field); the governing version of any work item is COMPUTED by git-traversal
// (D0069, pglViews) — these edges are the minimal authored input.
//
// HOME (issue017): these edges live HERE in .tracking, not inside the .engine Decision files,
// because validate_instances loads each .engine file in kernel isolation (schema/core only) and
// cannot resolve a cross-package target (Delivery / migration). .tracking is rust-validated and
// generic (name refs, no cross-resolution) — the same reason #CharteredBy edges live in tracking.
//
// SCOPE: only the clear, recent, unambiguous process-change Decisions are recorded here. Older
// process-change Decisions are NOT backfilled (D0067: never fabricate provenance) — they sit in
// the keystone-guard grandfather set (pglProcessChangeGuard, sprint40). From sprint40 on the
// guard makes a NEW process-def change carry its edge here, or the commit fails.
//
// Process MODULE names (work->process is by convention, D0069):
//   Delivery   = the sprint/ceremony workflow def (DeliveryWorkflow::Delivery)
//   migration  = the migration process part (EngineMigration::migration)
// =====================================================================
package ProcessGovernance {
    private import EngineRelationships::*;  // #ProspectiveChange / #SafetyChange markers

    // --- ceremony / Delivery workflow ---
    // D0039 created the per-sprint refine/standup/implement/review/closeOut/retro ceremony.
    #ProspectiveChange dependency from d0039 to Delivery;
    // D0049 made closeOut+retro autonomous and moved the human gate to a per-sitting review.
    #ProspectiveChange dependency from d0049 to Delivery;
    // D0064 ended the bootstrap exemption and mandated sprint ceremony for engine changes.
    #ProspectiveChange dependency from d0064 to Delivery;

    // --- migration process ---
    // D0067 created the migration process (expand/migrate/contract, 7 gates).
    #ProspectiveChange dependency from d0067 to migration;
}
