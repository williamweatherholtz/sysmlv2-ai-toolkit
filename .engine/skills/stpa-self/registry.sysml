// The stpa-self skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryStpaSelf {
    private import EngineElement::*;
    private import EngineSkills::*;

    part stpaSelf : AISkill {
        :>> id = "7d2e9f4a-6b1c-4e8d-a3f5-9c0b2d4e6f81";
        :>> title = "stpa-self";
        :>> purpose = "Deploys .engine/processes/stpa-self.sysml (D0313): STPA on keel's own workflow - from the computed control structure, every control action's four unsafe-control-action types, each traced to a controller constraint bound to a control or to an Issue naming the gap; re-run when the structure grows (stpa-currency guard).";
        :>> location = ".engine/skills/stpa-self/SKILL.md";
        :>> triggerCondition = "The control structure has changed since the last recorded run (stpa-currency warns), a control defect asks what else its controller can do wrong, or someone asks to run STPA on keel / find the control gaps in the workflow.";
        :>> writePolicy = WritePolicy::direct;
    }
}
