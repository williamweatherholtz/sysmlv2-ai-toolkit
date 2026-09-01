// The exec-summary skill's registry declaration, beside the skill it describes (D0222 pattern).
package SkillsRegistryExecSummary {
    private import EngineElement::*;
    private import EngineSkills::*;

    part execSummary : AISkill {
        :>> id = "8f5d3ca2-69b4-4a7e-c3d8-0e1f2a3b4c5d";
        :>> title = "exec-summary";
        :>> purpose = "Deploys .engine/processes/exec-summary.sysml: findings, proposals and decision requests reach the author as an interactive HTML page - recommendation-first tabs, one decision per tab with steelmanned choices and honest costs, an enforced-budget reasoning chain, and a Copy-for-AI digest returning the reader's selections as a decision record. The template is a project asset (templates/exec-summary/); the browser suite (tests/exec_summary) verifies every behavioral claim before delivery.";
        :>> location = ".engine/skills/exec-summary/SKILL.md";
        :>> triggerCondition = "A turn's deliverable is feedback the author will react to - 'make me an html', 'show me the options', 'exec summary', or any decision request that terminal text would bury.";
        :>> writePolicy = WritePolicy::direct;
    }
}
