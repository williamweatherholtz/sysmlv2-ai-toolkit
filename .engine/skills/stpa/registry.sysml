// The stpa skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryStpa {
    private import EngineElement::*;
    private import EngineSkills::*;

    part stpaSkill : AISkill {
        :>> id = "7d20a9c4-26cd-4f0a-8413-2b6fb2f0b222";
        :>> title = "stpa";
        :>> purpose = "Run STPA (losses/hazards/UCAs/constraints) per the SOP for safety-relevant scope.";
        :>> location = ".engine/skills/stpa/SKILL.md";
        :>> triggerCondition = "Safety-relevant Need/UseCase/Component enters scope.";
        :>> writePolicy = WritePolicy::prOnly;
    }
}
