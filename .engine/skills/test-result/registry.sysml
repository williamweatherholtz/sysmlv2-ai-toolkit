// The test-result skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryTestResult {
    private import EngineElement::*;
    private import EngineSkills::*;

    part testResult : AISkill {
        :>> id = "b153edf8-6a01-4e3d-c746-6fa3f6f0f666";
        :>> title = "test-result";
        :>> purpose = "Record test execution outcomes as appended TestResult instances: outcome + judgedAgainst commit + judgedAt + judgedBy. Never mutate prior results.";
        :>> location = ".engine/skills/test-result/SKILL.md";
        :>> triggerCondition = "Executing tests, recording gate verdicts, or closing a DoD.";
        :>> writePolicy = WritePolicy::direct;
    }
}
