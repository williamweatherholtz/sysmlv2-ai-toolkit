// The requirement-quality skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryRequirementQuality {
    private import EngineElement::*;
    private import EngineSkills::*;

    part requirementQuality : AISkill {
        :>> id = "61c1f9c2-08e8-4f55-9b34-1f5fb1f0a111";
        :>> title = "requirement-quality";
        :>> purpose = "Audit requirements against INCOSE GtWR rules + EARS syntax; emit findings + rewrites.";
        :>> location = ".engine/skills/requirement-quality/SKILL.md";
        :>> triggerCondition = "A requirement is authored or changed; DoR gate.";
        :>> writePolicy = WritePolicy::prOnly;
    }
}
