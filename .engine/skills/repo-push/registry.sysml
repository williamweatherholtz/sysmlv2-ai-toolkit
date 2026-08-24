// The repo-push skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryRepoPush {
    private import EngineElement::*;
    private import EngineSkills::*;

    part repoPush : AISkill {
        :>> id = "9f42cbe6-48ef-4c2c-a635-4d81d4f0d444";
        :>> title = "repo-push";
        :>> purpose = "Repository discipline: main-only direct commits, CR: prefix for schema/process changes, Co-Authored-By trailer, pre-commit validation + post-commit auto-push hooks (reconciled to D0014, issue009).";
        :>> location = ".engine/skills/repo-push/SKILL.md";
        :>> triggerCondition = "Committing engine/process work.";
        :>> writePolicy = WritePolicy::direct;
    }
}
