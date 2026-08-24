// The status-report skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryStatusReport {
    private import EngineElement::*;
    private import EngineSkills::*;

    part statusReport : AISkill {
        :>> id = "c820fk6f-3b78-4e0c-de13-3b60b7k6j333";
        :>> title = "status-report";
        :>> purpose = "Generate a self-contained HTML project status dashboard: active sprint, velocity/efficiency, backlog, decisions, git activity. Materialized VIEW — not authored content.";
        :>> location = ".engine/skills/status-report/SKILL.md";
        :>> triggerCondition = "Asked for a status report, HTML dashboard, visual overview, or 'where does the project stand'.";
        :>> writePolicy = WritePolicy::readOnly;
    }
}
