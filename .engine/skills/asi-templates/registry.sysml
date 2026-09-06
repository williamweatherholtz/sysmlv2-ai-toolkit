// The asi-templates skill's registry declaration, beside the skill it describes (D0222 pattern).
package SkillsRegistryAsiTemplates {
    private import EngineElement::*;
    private import EngineSkills::*;

    part asiTemplates : AISkill {
        :>> id = "3e580e63-3f04-41a4-8dcd-ab9bfde0f51e";
        :>> title = "asi-templates";
        :>> purpose = "Deploys .engine/processes/asi-templates.sysml: ASI-branded deliverables on one of two declared surfaces - an interactive exec-summary HTML decision page (recommendation-first tabs, steelmanned choices with honest costs, an enforced-budget reasoning chain, a Copy-for-AI digest that returns the reader's selections as a decision record) or a formal requirements-doc PDF (literal-verbatim statements, the CommonMark-subset content contract, field-map read-back and golden pixel diff). Carries the brand-drift check that must run before building against the tokens, and the contract checker that gates both surfaces on every commit. SUPERSEDES the standalone exec-summary unit, which carried its discipline and none of its machinery.";
        :>> location = ".engine/skills/asi-templates/SKILL.md";
        :>> triggerCondition = "A deliverable should leave as a DOCUMENT rather than terminal text - 'make me an html', 'exec summary', 'show me the options', 'report this graphically', any decision request terminal prose would bury; or a formal specification / requirements document / interface document is due; or brand currency is in question ('brand watch', 'has marketing changed anything').";
        :>> writePolicy = WritePolicy::direct;
    }
}
