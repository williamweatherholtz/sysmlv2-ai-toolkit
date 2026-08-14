// =====================================================================
// EngineDeploy — the release process (nReleaseDiscipline, D0128/D0135). Operationalises the Deploy
// workflow, which the engine had DEFINED but never run: one stale release, no recorded Release item,
// and version discipline left to memory. Deployed by the release skill.
// =====================================================================
package EngineDeploy {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    part deploy : Process {
        :>> id = "0d730001-0814-5301-9b3e-000000230001";
        :>> title = "Release (cut, publish, record)";
        :>> purpose = "Turn a green trunk into an obtainable, recorded release: a reconciled version, a full gate, a tag that CI builds and publishes for every platform, and a `Release` item so what shipped is an AUTHORED FACT whose contents are computed rather than remembered. Cadence is per-milestone, not per-sprint — a release marks a coherent capability step, and cutting one per sprint would make the tag meaningless.";
        :>> cadence = Cadence::adHoc;
    }

    part relReconcileVersion : ProcessStep {
        :>> id = "0d730002-0814-5302-9b3e-000000230002";
        :>> title = "Reconcile the versions";
        :>> order = 1;
        :>> triggerCondition = "A coherent capability step is complete on a green trunk.";
        :>> actionText = "Set the crate version to the new semver. RECONCILE means each version surface is meaningful and deliberate, NOT that they are equal: the crate version tracks the shipped binary, while the read-API version (`KEEL_API_VERSION`) is a SEPARATE contract that a downstream viewer pins, so it bumps only when an `/api/*` read shape breaks. Note in the release notes any change that breaks a caller — for 0.2.0, write paths now REFUSE when the acting actor is unstated (D0129), which breaks any script relying on a defaulted actor.";
        :>> producedArtifact = "A bumped, deliberate version; breaking changes identified.";
        :>> owner = Owner::ai;
    }
    part relGate : ProcessStep {
        :>> id = "0d730003-0814-5303-9b3e-000000230003";
        :>> title = "Gate the exact tree that will be tagged";
        :>> order = 2;
        :>> triggerCondition = "The version is set.";
        :>> actionText = "Run the FULL gate on the tree about to be tagged — `keel validate`, `keel guard` (all of them), `keel check-engine`, `cargo clippy -D warnings`, `cargo test` — and confirm CI is green on that same commit. A release is the one artifact that cannot be quietly re-verified later: downstream users have the binary. Never tag a tree whose gate you have not run at that commit, and never tag with a bypass in force.";
        :>> producedArtifact = "A green gate + green CI at the exact release commit.";
        :>> owner = Owner::ai;
    }
    part relTagPublish : ProcessStep {
        :>> id = "0d730004-0814-5304-9b3e-000000230004";
        :>> title = "Tag; CI builds and publishes";
        :>> order = 3;
        :>> triggerCondition = "The gate is green at the release commit.";
        :>> actionText = "Push an annotated `vX.Y.Z` tag. The release workflow builds the binary natively for linux-x86_64, macOS-arm64 and windows-x86_64 and attaches all three to a GitHub Release, so a downstream contributor obtains a matching gate by DOWNLOAD rather than by building from source — which is what makes the portable-gate guarantee real for a distributed team (D0129). Verify the workflow succeeded and the assets are attached; a tag whose build failed is worse than no tag, because the version now exists but is unobtainable.";
        :>> producedArtifact = "A published GitHub Release with per-platform binaries.";
        :>> owner = Owner::ai;
    }
    part relRecord : ProcessStep {
        :>> id = "0d730005-0814-5305-9b3e-000000230005";
        :>> title = "Record the Release as an authored fact";
        :>> order = 4;
        :>> triggerCondition = "The release is published.";
        :>> actionText = "Author a `Release` item carrying the version, the commit and its purpose. A git tag alone is not a tracked fact: it cannot be traced to, queried, or asked as-of. What the release CONTAINED stays COMPUTED from git ancestry plus the recorded facts (§2.1) — never hand-listed, because a hand-written contents list drifts from the commit the moment anything is cherry-picked or amended.";
        :>> producedArtifact = "A `Release` item; contents remain a computed view.";
        :>> owner = Owner::ai;
    }
}
