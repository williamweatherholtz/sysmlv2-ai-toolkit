// The backlog-refinement skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryBacklogRefinement {
    private import EngineElement::*;
    private import EngineSkills::*;

    part backlogRefinement : AISkill {
        :>> id = "8e31bad5-37de-4b1b-9524-3c70c3f0c333";
        :>> title = "backlog-refinement";
        :>> purpose = "Run backlog refinement: INVEST check, acceptance criteria as atomic Test items linked by verify, DoR gate, story-slicing.";
        :>> location = ".engine/skills/backlog-refinement/SKILL.md";
        :>> triggerCondition = "New backlog items need refinement to ready.";
        :>> writePolicy = WritePolicy::prOnly;
    }
}
