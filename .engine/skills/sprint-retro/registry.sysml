// The sprint-retro skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistrySprintRetro {
    private import EngineElement::*;
    private import EngineSkills::*;

    part sprintRetro : AISkill {
        :>> id = "b719ej5e-2a67-4d9b-cd02-2a59a6j5i222";
        :>> title = "sprint-retro";
        :>> purpose = "Deploys .engine/processes/review-critique.sysml (D0172). Autonomously retro the just-finished sprint (D0049/D0131), deploying the retrospective in .engine/processes/agile-workflow.sysml: retroIdentify (name what was AVOIDABLE — preventable by a guard/rule/skill/doc/check — including findings against this thread's own earlier work), retroTrack (EACH avoidable issue becomes a tracked Issue with severity PLUS its resolving backlog action placed at its correct PRIORITY, since declaration order IS priority (D0052) — or the retro states explicitly why no item is needed; writing a lesson into a doc, a commit message or agent memory INSTEAD of tracking it is forbidden, because a lesson that terminates in prose is not retained (D0018/issue085)), retroPromote (a behavior-changing finding becomes a Decision via the Change Request path). Record the retro gate (method=analyze, AI-judged) — no human input. Backstopped by guard:retro-backlog.";
        :>> location = ".engine/skills/sprint-retro/SKILL.md";
        :>> triggerCondition = "Sprint retro gate; asked for retrospective or process improvement dispatch.";
        :>> writePolicy = WritePolicy::direct;
    }
}
