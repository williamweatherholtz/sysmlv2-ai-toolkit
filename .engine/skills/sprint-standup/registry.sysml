// The sprint-standup skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistrySprintStandup {
    private import EngineElement::*;
    private import EngineSkills::*;

    part sprintStandup : AISkill {
        :>> id = "e486bg2b-9d34-4a6e-fa79-9d26d3g2f999";
        :>> title = "sprint-standup";
        :>> purpose = "Orient on current status, surface blockers, confirm active phase alignment, state next action — lightweight 2-minute ceremony. Runs the overall sprint loop .engine/processes/agile-workflow.sysml (deployed by the sprint-* skills, D0059).";
        :>> location = ".engine/skills/sprint-standup/SKILL.md";
        :>> triggerCondition = "Sprint standup gate; start of work session; asked for status or blockers.";
        :>> writePolicy = WritePolicy::direct;
    }
}
