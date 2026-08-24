// The build skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryBuild {
    private import EngineElement::*;
    private import EngineSkills::*;

    part build : AISkill {
        :>> id = "a3c8e1f0-5b9d-4e2a-8f6c-1d7b3e9a2c50";
        :>> title = "build";
        :>> purpose = "Compile the deliverable (Rust keel workspace) with warnings-as-errors and confirm the engine model still validates; report binary path + validation verdict. Prerequisite for test-verify; does not test or commit.";
        :>> location = ".engine/skills/build/SKILL.md";
        :>> triggerCondition = "Asked to build/compile, or before claiming the toolchain works.";
        :>> writePolicy = WritePolicy::readOnly;
    }
}
