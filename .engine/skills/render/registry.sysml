// The render skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryRender {
    private import EngineElement::*;
    private import EngineSkills::*;

    part renderSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a086-516273849615";
        :>> title = "render";
        :>> purpose = "Deploy the Modular Render + Review process (D0086): render ANY declared view as a self-contained interactive artifact via `keel render <view> --mode graph|table|review` (graph generalizes the D0085 diagram; table = sortable rows; review = accept/finding + rationale capture), and round-trip a human review back via `keel apply-review` as new linked critique items that attest state + induce suspicion. Never author or commit a rendered artifact (computed #View, §2.1/D0015 — git-ignored). The deploying skill for .engine/processes/render.sysml.";
        :>> location = ".engine/skills/render/SKILL.md";
        :>> triggerCondition = "Asked to render/tabulate/explore a specific view as an artifact, build a table from a view, review/disposition elements (accept/reject + rationale), capture critique findings, or export a review batch; or to apply a review batch back into the model.";
        :>> writePolicy = WritePolicy::direct;
    }
}
