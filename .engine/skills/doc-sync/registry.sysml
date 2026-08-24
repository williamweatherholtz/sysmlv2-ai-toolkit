// The doc-sync skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryDocSync {
    private import EngineElement::*;
    private import EngineSkills::*;

    part docSync : AISkill {
        :>> id = "d6e0a3c2-7e4f-4a9b-8c2d-3f8e5b1d9f72";
        :>> title = "doc-sync";
        :>> purpose = "Deploy the Documentation Sync process (D0059): on any change to a type/schema/workflow/process/skill/tool/template/superseding-decision/convention, grep the doc surface and fix every invalidated claim IN THE SAME COMMIT. The deploying skill for .engine/processes/doc-sync.sysml.";
        :>> location = ".engine/skills/doc-sync/SKILL.md";
        :>> triggerCondition = "Any CHANGE (§3a) or schema/process/skill/tool edit; before committing such a change.";
        :>> writePolicy = WritePolicy::direct;
    }
}
