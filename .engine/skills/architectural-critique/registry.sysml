// The architectural-critique skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryArchitecturalCritique {
    private import EngineElement::*;
    private import EngineSkills::*;

    part architecturalCritique : AISkill {
        :>> id = "c5f0a2b1-6d3e-4f8a-9b1c-2e7d4a0c8e61";
        :>> title = "architectural-critique";
        :>> purpose = "Run the antagonistic architecture/process audit with the CAE+GQM+ATAM framework (D0046): attack each claim for refuting evidence, compute the GQM metric set, play ATAM scenarios; record findings as tracked Issue/Decision items (not prose). The deploying skill for .engine/processes/architectural-critique.sysml.";
        :>> location = ".engine/skills/architectural-critique/SKILL.md";
        :>> triggerCondition = "Asked to audit/critique the engine or processes; on the architectural-critique process; periodic stress-test.";
        :>> writePolicy = WritePolicy::direct;
    }
}
