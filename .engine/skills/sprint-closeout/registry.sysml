// The sprint-closeout skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistrySprintCloseout {
    private import EngineElement::*;
    private import EngineSkills::*;

    part sprintCloseout : AISkill {
        :>> id = "a608di4d-1f56-4c8a-bc91-1f48f5i4h111";
        :>> title = "sprint-closeout";
        :>> purpose = "Autonomously close a sprint (D0049): verify DoD passes, record actualHours, record the closeOut gate (method=inspect, AI-judged) with NO human confirmation, hand off to the autonomous retro. Deploys the ceremony process .engine/processes/definition-of-done.sysml (DoD, D0059).";
        :>> location = ".engine/skills/sprint-closeout/SKILL.md";
        :>> triggerCondition = "Sprint closeOut gate; asked to close or accept a sprint.";
        :>> writePolicy = WritePolicy::direct;
    }
}
