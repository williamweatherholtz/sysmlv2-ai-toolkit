// The test-design skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryTestDesign {
    private import EngineElement::*;
    private import EngineSkills::*;

    part testDesign : AISkill {
        :>> id = "a042dce7-59f0-4d2c-b635-5e92e5f0e555";
        :>> title = "test-design";
        :>> purpose = "Design verification Tests for engine items: choose method (test/demo/inspect/analysis/confirmation), write BDD/Gherkin scenarios, map acceptance criteria to atomic Test instances.";
        :>> location = ".engine/skills/test-design/SKILL.md";
        :>> triggerCondition = "Authoring DoD criteria, acceptance tests, or BDD scenarios for engine items.";
        :>> writePolicy = WritePolicy::prOnly;
    }
}
