// The control-defect skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryControlDefect {
    private import EngineElement::*;
    private import EngineSkills::*;

    part controlDefect : AISkill {
        :>> id = "e8b3f174-52c9-4a06-9d1e-70f4a26b8c53";
        :>> title = "control-defect";
        :>> purpose = "Deploys .engine/processes/control-defect.sysml (D0278): a wrong verdict from a guard, gate, hook or lens is its own defect class, because a broken control INVALIDATES ITS OWN HISTORY - every verdict it gave while broken is unverified and nobody knows which. Establish it against the SOURCE not the output, classify the direction (over-reporting is loud and costs trust; under-reporting is SILENT and is never below High), state what it PASSED while broken and whether that was re-checked, then record the Issue AND register it in control-defects.toml in the same act so keel guard prints the defect beside that control's own verdict. Detection cannot be gated - you notice by being surprised - so the process makes the RESPONSE reliable, not the noticing.";
        :>> location = ".engine/skills/control-defect/SKILL.md";
        :>> triggerCondition = "A control's verdict has to be explained away - 'the guard is wrong here', 'that's a false positive', 'it passed but it should not have', or 'that's the guard working correctly' about a verdict that took thought.";
        :>> writePolicy = WritePolicy::direct;
    }
}
