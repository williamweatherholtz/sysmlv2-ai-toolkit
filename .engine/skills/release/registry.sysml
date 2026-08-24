// The release skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryRelease {
    private import EngineElement::*;
    private import EngineSkills::*;

    part releaseSkill : AISkill {
        :>> id = "0d740001-0814-5401-9b3e-000000240001";
        :>> title = "release";
        :>> purpose = "Deploy the Release process (D0135/nReleaseDiscipline): reconcile the version (the crate tracks the shipped binary; KEEL_API_VERSION is a SEPARATE contract a downstream viewer pins, bumped only on a breaking /api/* read shape) and name any caller-breaking change; run the FULL gate on the exact tree about to be tagged and confirm CI green at that commit, because a release is the one artifact that cannot be quietly re-verified later; push an annotated vX.Y.Z tag so CI builds and attaches linux-x86_64 / macOS-arm64 / windows-x86_64 binaries — a downstream contributor must obtain a matching gate by download, not by building from source; then author a `Release` item, leaving its CONTENTS computed from git ancestry rather than hand-listed. Per-milestone cadence, never per-sprint. The deploying skill for .engine/processes/deploy.sysml.";
        :>> location = ".engine/skills/release/SKILL.md";
        :>> triggerCondition = "A coherent capability step is complete on a green trunk and a release is wanted; or asked to cut/publish a release.";
        :>> writePolicy = WritePolicy::direct;
    }
}
