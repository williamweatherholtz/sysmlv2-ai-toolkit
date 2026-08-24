// The test-verify skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryTestVerify {
    private import EngineElement::*;
    private import EngineSkills::*;

    part testVerify : AISkill {
        :>> id = "b4d9f201-6c0e-4f3b-9a7d-2e8c4f0b3d61";
        :>> title = "test-verify";
        :>> purpose = "Run the deliverable's verification gate: cargo test + clippy -D warnings + BDD scenarios + Python<->Rust orient parity; report per-layer pass/fail as evidence for a method=test DoD. Hands off to test-result for recording; does not commit.";
        :>> location = ".engine/skills/test-verify/SKILL.md";
        :>> triggerCondition = "Asked to test/verify, check parity, or before recording a deliverable DoD TestResult.";
        :>> writePolicy = WritePolicy::readOnly;
    }
}
