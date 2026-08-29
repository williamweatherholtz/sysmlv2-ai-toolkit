// The control-propriety skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryControlPropriety {
    private import EngineElement::*;
    private import EngineSkills::*;

    part controlPropriety : AISkill {
        :>> id = "0bdffcbf-a401-4f42-952f-2aa06312755a";
        :>> title = "control-propriety";
        :>> purpose = "Deploys .engine/processes/control-propriety.sysml (D0257): assess an enforcement mechanism - a control, guard, probe or load-bearing convention - under five adversarial lenses (overfit, class coverage, change robustness, circumventability, measurability), recording one ProprietyFinding per lens with a NAMED DEFEATER, routing every unsound verdict to an Issue with a resolver. Grounded in mutation testing (variants over specimens), eliminative induction (defeaters over affirmations), and control validation (attack over inventory).";
        :>> location = ".engine/skills/control-propriety/SKILL.md";
        :>> triggerCondition = "A new control, guard or probe lands; a control's soundness is asserted in a decision or gate; or the human calls a propriety round over the existing set.";
        :>> writePolicy = WritePolicy::direct;
    }
}
