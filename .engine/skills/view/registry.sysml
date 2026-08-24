// The view skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryView {
    private import EngineElement::*;
    private import EngineSkills::*;

    part viewSkill : AISkill {
        :>> id = "67940516-2c7d-4e8f-a075-516273849611";
        :>> title = "view";
        :>> purpose = "Deploy query-driven viewpoints (D0075): read/write/execute lightweight TOML view files (.engine/views/*.view.toml) — a select + optional traverse + project filter over the tracking model, run by `keel view <name>` (Rust authority, fail-loud). The deploying skill for D0075; resolves issue018.";
        :>> location = ".engine/skills/view/SKILL.md";
        :>> triggerCondition = "Asked to create/run a viewpoint, slice the model by a concern, or trace items along edges.";
        :>> writePolicy = WritePolicy::direct;
    }
}
