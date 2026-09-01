// The project-migration skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryProjectMigration {
    private import EngineElement::*;
    private import EngineSkills::*;

    part projectMigration : AISkill {
        :>> id = "c7f1e820-4d63-4a95-b0e7-38f5142ca6d9";
        :>> title = "project-migration";
        :>> purpose = "Deploys .engine/processes/project-migration.sysml (D0275): move an EXISTING project onto an engine that changed underneath it - preflight to a recorded green SHA, check what the new engine REMOVES rather than adds, apply and let migrate roll back rather than hand-repairing, reconcile the project's own choices (pin comment, adoption, project-owned contracts) against the resync, prove it in the project's OWN CI rather than only locally, and report the cost upstream because the engine cannot migrate itself and is blind here. Carries the three traps that have already fired: the dirty-tree/skew deadlock (issue324), the half-update no refusal covers, and the CI that was never running (issue326/issue327).";
        :>> location = ".engine/skills/project-migration/SKILL.md";
        :>> triggerCondition = "A project's engine-version pin is behind the binary it is being asked to run - an upgrade, a unit needing a newer capability, or a breaking rename such as D0273.";
        :>> writePolicy = WritePolicy::direct;
    }
}
