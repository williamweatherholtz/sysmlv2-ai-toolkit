// The migration skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryMigration {
    private import EngineElement::*;
    private import EngineSkills::*;

    part migration : AISkill {
        :>> id = "67940516-2c7d-4e8f-a067-516273849610";
        :>> title = "migration";
        :>> purpose = "Deploy the Safe Bulk Migration process (D0067): run any bulk migration of model data/schema (rename/split/drop/add a field across many sites) through the gated expand/migrate/contract lifecycle — committed transform script, dry-run+reconcile control totals, green at every step, backfill-before-tighten, grandfather/backfill-with-basis NEVER fabricate. The deploying skill for .engine/processes/migration.sysml.";
        :>> location = ".engine/skills/migration/SKILL.md";
        :>> triggerCondition = "Editing the same field/shape across many instances/files; a schema rename/split/drop/add; a representation refactor of authored model data.";
        :>> writePolicy = WritePolicy::direct;
    }
}
