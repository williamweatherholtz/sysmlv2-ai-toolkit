// The stpa-diagram skill's registry declaration, beside the skill it describes (D0222).
package SkillsRegistryStpaDiagram {
    private import EngineElement::*;
    private import EngineSkills::*;

    part stpaDiagram : AISkill {
        :>> id = "f8b6126d-2d98-4f5f-b3fe-f2cd4711449a";
        :>> title = "stpa-diagram";
        :>> purpose = "Deploys .engine/processes/stpa-diagram.sysml (D0285): draw a computed control structure so it can be read - authority descending on the vertical axis, control actions down (solid, left) and feedback up (dashed, right), every edge labelled with WHAT PASSES along it, right-angle routing with no two segments sharing a line, a semicircular hop at every crossing. The tool .engine/tools/stpa_diagram.py enforces all of it by construction from the JSON of keel show control-structure; the skill's one judgment is the read-back before delivery.";
        :>> location = ".engine/skills/stpa-diagram/SKILL.md";
        :>> triggerCondition = "A control structure is to be shown to a human - this project's own (keel show control-structure) or any EngineSafety controller/action/feedback set - or an existing diagram of one is called unreadable.";
        :>> writePolicy = WritePolicy::readOnly;
    }
}
