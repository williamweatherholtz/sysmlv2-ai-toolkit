// The intake skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryIntake {
    private import EngineElement::*;
    private import EngineSkills::*;

    part intake : AISkill {
        :>> id = "d5c81e30-4a76-4b29-9f05-63be7a12cd48";
        :>> title = "intake";
        :>> purpose = "Deploys .engine/processes/intake.sysml (D0166), the Intake process: record a human's directive statement VERBATIM, translate it into one or more UserStories, triage each by what it implicates from a closed vocabulary, create the downstream item and link it. Keeps their words and the AI's reading as separate auditable facts, because the failure this fixes is a paraphrase that drifted and was the only thing written down.";
        :>> location = ".engine/skills/intake/SKILL.md";
        :>> triggerCondition = "The human states direction - a request, correction, constraint, priority, verdict or terminology preference. Applies at the START of the turn, before the work, so what they asked for is recorded before it is acted on.";
        :>> writePolicy = WritePolicy::direct;
    }
}
