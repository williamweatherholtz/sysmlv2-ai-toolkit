// The diagram skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryDiagram {
    private import EngineElement::*;
    private import EngineSkills::*;

    part diagramSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a085-516273849614";
        :>> title = "diagram";
        :>> purpose = "Deploy the Traceability Diagram process (D0085): generate the comprehensive traceability diagram as a regenerated, interactive, self-contained HTML view via `keel diagram` (whole model — typed nodes + metadata + every typed edge; in-page filter/search/focus/fit); never author or commit a diagram (it's a computed #View, §2.1/D0015 — git-ignored). The deploying skill for .engine/processes/diagram.sysml.";
        :>> location = ".engine/skills/diagram/SKILL.md";
        :>> triggerCondition = "Asked to see/draw/visualize/diagram the model, traceability, reliance, linkage, supersession, effects, or metadata; to map how decisions/needs/requirements/work/processes connect; or to explore the graph part at a time.";
        :>> writePolicy = WritePolicy::direct;
    }
}
