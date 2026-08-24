// The decision-channel skill's own registry declaration, beside the skill it describes (D0220).
//
// WHY IT LIVES HERE rather than in .engine/skills/skills-registry.sysml. Adopting this unit on
// another project (asirobots/penumbra) proved the central registry cannot travel: the bundle carried
// the process definition, the SKILL.md and the mechanism, but the registry ENTRY that binds
// skill -> process stayed home, so `process-skill` FAILED in the receiving project on the very first
// run — a new project's first experience of an adopted unit was a red gate it had no way to fix
// without hand-editing a shared file. The `process-skill` guard now reads every `.sysml` under
// `.engine/skills/`, so a unit ships its own registration as a FILE and nothing has to be
// text-merged into a registry that also holds thirty-six other skills. Same reasoning the rules
// layer already applies by carrying rules BY NAME instead of copying a shared file.
package SkillsRegistryDecisionChannel {
    private import EngineElement::*;

    part decisionChannelSkill : AISkill {
        :>> id = "2f82837a-bdd6-4c9b-9b2a-0e21af075f62";
        :>> title = "decision-channel";
        :>> purpose = "Deploys .engine/processes/decision-channel.sysml: decisions travel as GitHub issues - non-forks auto-accept under standing consent with receipts and a forever-open override thread; forks earn the ask via the judgment-request-quality bar. Portable as a UNIT since D0219: `keel process export` carries the workflows and scripts, and the receiving project declares its OWN deciders in .engine/contracts/github-actors.toml (D0205/D0207/D0219).";
        :>> location = ".engine/skills/decision-channel/SKILL.md";
        :>> triggerCondition = "A Decision is being proposed, published, recorded, or overridden through the GitHub channel; or a downstream project adopts the channel.";
        :>> writePolicy = WritePolicy::direct;
    }
}
