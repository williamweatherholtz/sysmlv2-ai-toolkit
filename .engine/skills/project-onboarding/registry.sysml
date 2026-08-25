// The project-onboarding skill's own registry declaration, beside the skill it describes (D0220/D0222).
package SkillsRegistryProjectOnboarding {
    private import EngineElement::*;
    private import EngineSkills::*;

    part projectOnboardingSkill : AISkill {
        :>> id = "bd0f270b-a0bb-4f0a-836f-ee775cd22471";
        :>> title = "project-onboarding";
        :>> purpose = "Deploys .engine/processes/project-onboarding.sysml: establish what a project is building and what must not go wrong with it, match those facts against each process's declared APPLIES-WHEN condition, research the domain's practice where the engine has no process for it, recommend a set with per-process reasoning the author can reject, and charter the choice as a Decision (D0225).";
        :>> location = ".engine/skills/project-onboarding/SKILL.md";
        :>> triggerCondition = "`keel onboard` reports NOT CHARTERED; a project was just scaffolded by `keel init`; or the author asks which processes their project should run.";
        :>> writePolicy = WritePolicy::direct;
    }
}
