// The github-intake skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryGithubIntake {
    private import EngineElement::*;
    private import EngineSkills::*;

    part githubIntake : AISkill {
        :>> id = "17864ae8-6a71-4392-b31d-e7a8bc1fedea";
        :>> title = "github-intake";
        :>> purpose = "Deploys .engine/processes/github-intake.sysml (D0263): ingest an issue VERBATIM with keel github-ingest (attributed to the GitHub login, idempotent on its URL), triage it through the ordinary intake vocabulary with a defensible note, route it to an item with a REAL resolver, and answer the reporter where they are standing.";
        :>> location = ".engine/skills/github-intake/SKILL.md";
        :>> triggerCondition = "An open issue on the repository that no Statement cites - a downstream project's field report, a collaborator's bug, or the human filing from a phone.";
        :>> writePolicy = WritePolicy::direct;
    }
}
