// The decision-authoring skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryDecisionAuthoring {
    private import EngineElement::*;
    private import EngineSkills::*;

    part decisionAuthoringSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a108-516273849620";
        :>> title = "decision-authoring";
        :>> purpose = "Deploy the Decision Authoring process (issue058/D0058): author every Decision so each fact has ONE canonical home — the why in its own fields, the acceptance verdict in a discrete human-judged method=confirmation TestResult, status/state COMPUTED — never restated as prose in another field or a comment. Backstopped by decisionNoVerdictProseRule + guard:confirmation-authenticity. The deploying skill for .engine/processes/decision-authoring.sysml.";
        :>> location = ".engine/skills/decision-authoring/SKILL.md";
        :>> triggerCondition = "Recording, accepting, or amending a Decision (CHANGE/RECORD, or `keel record decision`).";
        :>> writePolicy = WritePolicy::direct;
    }
}
