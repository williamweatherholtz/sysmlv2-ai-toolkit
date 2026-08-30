// The library-stewardship skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryLibraryStewardship {
    private import EngineElement::*;
    private import EngineSkills::*;

    part libraryStewardship : AISkill {
        :>> id = "fd4de6d9-ef09-41a0-90dd-b51a4ebbd221";
        :>> title = "library-stewardship";
        :>> purpose = "Deploys .engine/processes/library-stewardship.sysml (D0259): publish deliberately (landed trees only, one commit per changed unit, the push is the fleet-visible act), consume knowingly (staleness statements and divergence refusals are obligations, not noise), update per project as an explicit gated act under the keystone, and turn any reusable skill into a UNIT before it becomes folklore (the issue245 test).";
        :>> location = ".engine/skills/library-stewardship/SKILL.md";
        :>> triggerCondition = "Publishing or updating a library unit, a sync statement that needs acting on, or a skill worth carrying to a second project.";
        :>> writePolicy = WritePolicy::direct;
    }
}
