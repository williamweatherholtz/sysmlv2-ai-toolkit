// =====================================================================
// EngineBacklog — the engine's own construction plan, as tracked SysML data
// (the self-build dogfood). Transitioned from RESUME's prose backlog.
//
// A project plan IS a behavior, so it's an `action def`: tasks are `action`s,
// dependencies are `succession` (first/then). whats-next resolves this exactly
// like a workflow (ordered, parallel-grouped waves).
//
// Each task's `doc` is its acceptance criterion ("the test"). NOTE: done-state
// is NOT yet here — per the engine principle, "done" must be COMPUTED from a
// passing test, not asserted by a flag. Attaching a real verify-linked Test +
// result (so whats-next shows OUTSTANDING = not-done) is the next increment,
// pending the test/verification-model decision.
// =====================================================================
package EngineBacklog {
    action def EngineBuild {
        // ---- done this session (acceptance = the validation/commit noted) ----
        action claudeMd { doc /* interaction discipline + §3 request-triage state machine; CLAUDE.md committed */ }
        action processSpec { doc /* process-model design spec committed (docs/superpowers/specs/) */ }
        action schemaRewrite { doc /* schema/core + safety -> flat Engine<Concern> packages; validate_schema 12/12 */ }
        action workflows { doc /* six workflows as native action defs; validate_workflows 7/7 */ }
        action validatorFix { doc /* validators exit cleanly: DEVNULL + os.kill(provisioner); no hung shell / JVM leak */ }
        action autoPush { doc /* post-commit hook auto-pushes; bootstrap.sh captures per-clone setup */ }
        action whatsNextV2 { doc /* resolver reads native action/succession DAG incl. parallelism; clean exit */ }

        // ---- outstanding ----
        action testModel { doc /* decide + validate the test/verification model: done computed from a passing test (git-SHA) */ }
        action instanceAware { doc /* whats-next reads .tracking work-items + done-state -> OUTSTANDING tasks, not just the plan */ }
        action schemaAudit { doc /* align schema types with natives: Need->requirement def (+satisfy/verify), UseCase->use case def, Test->verification, native elementId */ }
        action instanceMigration { doc /* migrate processes/*.sysml, decisions/*.sysml, skills-registry to the new packages; retire validate_sysml.py */ }
        action queryTools { doc /* the deliverable tools: parser, indexer, query CLI (whats-downstream/whats-stale-since), API, browser GUI */ }

        // ---- dependencies (succession: earlier then later) ----
        first claudeMd then processSpec;
        first processSpec then schemaRewrite;
        first schemaRewrite then workflows;
        first workflows then whatsNextV2;
        first whatsNextV2 then schemaAudit;
        first whatsNextV2 then testModel;
        first schemaAudit then testModel;
        first testModel then instanceAware;
        first whatsNextV2 then instanceAware;
        first schemaAudit then instanceMigration;
        first instanceAware then queryTools;
    }
}
