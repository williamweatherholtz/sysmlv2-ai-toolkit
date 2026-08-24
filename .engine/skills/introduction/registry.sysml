// The introduction skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryIntroduction {
    private import EngineElement::*;
    private import EngineSkills::*;

    part introductionSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a093-516273849618";
        :>> title = "introduction";
        :>> purpose = "Deploy the guided project-based Onboarding process (D0093): take a newcomer on a freshly `keel init`-ed project from zero to FIRST VALUE in one session — 1-minute mental model, capture the first business Need, derive a first requirement + work item, run the first sprint to a real artifact, show `orient`. Learn by DOING the first real item, not a doc dump; dual-surface (AI drives CLI, human supervises). The deploying skill for .engine/processes/introduction.sysml.";
        :>> location = ".engine/skills/introduction/SKILL.md";
        :>> triggerCondition = "A human asks to be introduced/onboarded, 'how do I start', 'set up / spin up a new project', or right after `keel init`.";
        :>> writePolicy = WritePolicy::direct;
    }
}
