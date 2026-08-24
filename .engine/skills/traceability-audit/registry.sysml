// The traceability-audit skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryTraceabilityAudit {
    private import EngineElement::*;
    private import EngineSkills::*;

    part traceabilityAudit : AISkill {
        :>> id = "c264fe09-7b12-4f4e-d857-7b047b01c777";
        :>> title = "traceability-audit";
        :>> purpose = "Audit satisfy/verify/allocate coverage: surface uncovered Needs, unverified Requirements, unallocated Components; trace-need paths; flag suspect items.";
        :>> location = ".engine/skills/traceability-audit/SKILL.md";
        :>> triggerCondition = "Architecture/Delivery review gates, DoD coverage checks, or asked to audit traceability.";
        :>> writePolicy = WritePolicy::readOnly;
    }
}
