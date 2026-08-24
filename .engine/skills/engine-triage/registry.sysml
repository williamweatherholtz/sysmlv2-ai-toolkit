// The engine-triage skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryEngineTriage {
    private import EngineElement::*;
    private import EngineSkills::*;

    part engineTriage : AISkill {
        :>> id = "5b3f6f3e-91a4-4f47-9f57-1f2f3a4b5c6d";
        :>> title = "engine-triage";
        :>> purpose = "Break EVERY request into parts and route EACH to CHANGE/EXECUTE/RECORD/VIEW/ORIENT before acting; flag what doesn't cleanly map; state routes out loud (CLAUDE.md 3, D0064). Fired every turn by a UserPromptSubmit hook.";
        :>> location = ".engine/skills/engine-triage/SKILL.md";
        :>> triggerCondition = "Any substantive request arrives.";
        :>> writePolicy = WritePolicy::readOnly;
    }
}
