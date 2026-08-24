// The sprint-planning skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistrySprintPlanning {
    private import EngineElement::*;
    private import EngineSkills::*;

    part sprintPlanning : AISkill {
        :>> id = "d375af1a-8c23-4f5d-e968-8c15c2f1e888";
        :>> title = "sprint-planning";
        :>> purpose = "Orient on ready frontier, select scope, apply Fibonacci sizing to set estimatedPoints, pass DoR gate before sprint work begins. Deploys the ceremony process .engine/processes/definition-of-ready.sysml (DoR, D0059).";
        :>> location = ".engine/skills/sprint-planning/SKILL.md";
        :>> triggerCondition = "Sprint refine gate; asked to plan next sprint, size a story, or select scope.";
        :>> writePolicy = WritePolicy::direct;
    }
}
