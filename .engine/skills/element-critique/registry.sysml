// The element-critique skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryElementCritique {
    private import EngineElement::*;
    private import EngineSkills::*;

    part elementCritiqueSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a080-516273849613";
        :>> title = "element-critique";
        :>> purpose = "Deploy the Antagonistic Element Critique process (D0080/D0079): adversarially verify each tracked Need/SystemRequirement/Decision/Architecture element through required adversarial lenses by an INDEPENDENT critic, recording each critique as a method=critique verification + result #Verify-linked to the target; raise findings as severity-carrying Issues (#Resolves loop); let critique coverage + staleness COMPUTE from the assurance substrate (keel show coverage, suspect). The deploying skill for .engine/processes/element-critique.sysml.";
        :>> location = ".engine/skills/element-critique/SKILL.md";
        :>> triggerCondition = "Asked to critique/red-team/adversarially review a requirement/need/decision; find weaknesses/gaps/ambiguities in the model; what hasn't been critiqued; or a critique going suspect after a change.";
        :>> writePolicy = WritePolicy::direct;
    }
}
