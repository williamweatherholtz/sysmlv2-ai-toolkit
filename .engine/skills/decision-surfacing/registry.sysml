// The decision-surfacing skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryDecisionSurfacing {
    private import EngineElement::*;
    private import EngineSkills::*;

    part decisionSurfacing : AISkill {
        :>> id = "7cacbe28-64e7-4cd0-95b6-40bad7040596";
        :>> title = "decision-surfacing";
        :>> purpose = "Deploys .engine/processes/decision-surfacing.sysml (D0269): when a judgment is waiting on the human, render it into the exec-summary artifact, publish it to a durable URL, and LEAD the response with that link - never report the work first and the pending decisions last, which is indistinguishable from not asking. Re-surface every turn until answered or explicitly withdrawn.";
        :>> location = ".engine/skills/decision-surfacing/SKILL.md";
        :>> triggerCondition = "A turn ends with a proposed Need or Decision unaccepted, a fork where the other side is defensible, a risk the human has not seen, or an ordering I chose on their behalf.";
        :>> writePolicy = WritePolicy::direct;
    }
}
