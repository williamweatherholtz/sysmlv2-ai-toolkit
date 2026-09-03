// libraryStewardship — the discipline around `keel library` and `keel process publish` (D0259).
//
// The mechanics landed with D0250-D0252 and are probe-proven; the DISCIPLINE existed only in
// conversation - the issue245 shape the human named on the program-summary page (st036: define it
// now, one process for publish + consume). D0040: publishing recurred four times on its first day.
package ProcessLibraryStewardship {
    private import EngineElement::*;
    private import EngineProcess::*;

    action libraryStewardship : Process {
        :>> id = "d0259a01-0829-4b5c-8d1e-2f3a4b5c6d7e";
        :>> title = "Library stewardship";
        :>> purpose = "Keep the portable-content library trustworthy in both directions: publish deliberately (a unit changes what every other machine consumes), consume silently but knowingly (sync's staleness statements and no-op refusals are read, not scrolled past), and update per project as an explicit, gated act.";
        :>> cadence = Cadence::eventDriven;
        // APPLIES-WHEN: a machine holds a library clone, or a project imports from one - any keel
        // fleet past its first `keel library init`
    }

    action lsPublish : ProcessStep {
        :>> id = "d0259a01-0829-4b5c-8d1e-2f3a4b5c6d7f";
        :>> title = "Publish when the unit's CONTENT moved, and read the no-op refusal as information";
        :>> triggerCondition = "A unit's definition, skill, rules or extras changed in its home project, and the change is landed (gated, committed) there.";
        :>> actionText = "Publish FROM the landed tree, never from a dirty one - the library must never hold content no project's gate has seen. `keel process publish <name>` exports and commits; a 'nothing to publish' answer is INFORMATION (the content did not move even if prose around it did), never a step to force. Then push the library clone deliberately: the push is the fleet-visible act, and batching several publishes into one push is fine - the commits stay one-per-unit.";
        :>> producedArtifact = "One library commit per changed unit, pushed when the publisher chooses.";
        :>> owner = Owner::either;
    }

    action lsConsume : ProcessStep {
        :>> id = "d0259a01-0829-4b5c-8d1e-2f3a4b5c6d80";
        :>> title = "Sync silently, but READ what sync says";
        :>> triggerCondition = "Starting work on a machine with a library clone, or before importing.";
        :>> actionText = "`keel library sync` is safe to run any time - availability is never activation, held by the byte-identical-gate differential. Its two loud outcomes are OBLIGATIONS, not noise: a STALENESS statement means decisions about units are being made against a dated cache (say so when it matters); a DIVERGENCE refusal means something wrote to the clone outside publish - investigate or discard the cache, never merge it.";
        :>> producedArtifact = "A current (or stated-stale) cache, with any divergence escalated rather than absorbed.";
        :>> owner = Owner::either;
    }

    action lsUpdate : ProcessStep {
        :>> id = "d0259a01-0829-4b5c-8d1e-2f3a4b5c6d81";
        :>> title = "A project takes an update as an explicit, gated act";
        :>> triggerCondition = "`keel library list` shows a higher version than the project's install record for a unit the project uses.";
        :>> actionText = "`keel process import --from-library <name> --update` in the project, then its gate. The three-way merge keeps local adaptations; a CONFLICT is a real disagreement between the project's adaptation and upstream - resolve it in the project, and if the adaptation is generally right, publish it back rather than re-resolving forever. Commit the update under the keystone (a unit update is a process change in the receiving project, D0183/K9).";
        :>> producedArtifact = "An updated unit, gated in the receiving project, with the supersession committed under a marked Decision.";
        :>> owner = Owner::either;
    }

    action lsStock : ProcessStep {
        :>> id = "d0259a01-0829-4b5c-8d1e-2f3a4b5c6d82";
        :>> title = "A skill worth reusing becomes a UNIT before it becomes folklore";
        :>> triggerCondition = "A project grows a skill, template or working method a second project would want.";
        :>> actionText = "The issue245 test: if it has no process definition, it appears in no catalogue and travels nowhere. Author the process file, the engine-side skill + registry, and unit-extras naming what it needs to RUN (repo files only - never a machine environment); then publish. The control-defect kit is the worked example: skill + template + browser suite existed for weeks, transferable only after its process file made it a unit.";
        :>> producedArtifact = "A new unit in the library, importable whole.";
        :>> owner = Owner::either;
    }

    first lsPublish then lsConsume;
    first lsConsume then lsUpdate;
    first lsUpdate then lsStock;
}
