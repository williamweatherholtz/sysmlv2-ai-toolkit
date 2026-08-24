// The business-architecture skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryBusinessArchitecture {
    private import EngineElement::*;
    private import EngineSkills::*;

    part businessArchitectureSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a105-516273849619";
        :>> title = "business-architecture";
        :>> purpose = "The keel-native design skill: run design/scoping by DEPLOYING the Business workflow (.engine/workflows/business.sysml: Brief->Personas->Needs->UseCases) then the Architecture workflow (.engine/workflows/architecture.sysml), recording each decision at the point of decision, keeping Needs strictly upstream of architecture, emitting authored facts (Brief/Needs/Decisions/SystemRequirements + typed edges) never a prose design doc. Replaces the keel-blind brainstorming skill (issue054 F1 root: the Business/Architecture workflows were inert — no deploying skill, D0059). Bound by D0106 (parse-first, point-of-decision recording) + D0105 (one source of truth). The deploying skill for .engine/processes/business-elicitation.sysml (issue245: the elicitation PROCEDURE, catalogued and transferable), and for .engine/workflows/business.sysml + .engine/workflows/architecture.sysml.";
        :>> location = ".engine/skills/business-architecture/SKILL.md";
        :>> triggerCondition = "A request means design/scope/figure-out X, 'what should we build', a new capability, requirements elicitation, or any architectural direction.";
        :>> writePolicy = WritePolicy::direct;
    }
}
