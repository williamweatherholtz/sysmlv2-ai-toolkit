// The adversarial-panel-review skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryAdversarialPanelReview {
    private import EngineElement::*;
    private import EngineSkills::*;

    part adversarialPanelReviewSkill : AISkill {
        :>> id = "a9b4e9f0-492a-4aa3-983c-28402a4b8cff";
        :>> title = "adversarial-panel-review";
        :>> purpose = "Deploys .engine/processes/adversarial-panel-review.sysml (D0187), the Adversarial Panel Review process: convene a persona-diverse critic panel (pessimists, optimists, correctness/consistency/mechanism pedants, a scope reducer, a systems engineer deriving kernel invariants) whose ACCEPT verdicts are conditional on named, verifiable changes; dispatch rounds in parallel with cross-panelist digests from round 2; resolve every flip condition in an artifact revision carrying a finding-to-resolution map; iterate to uncompelled unanimity; record surviving outcomes as tracked facts. Carries the permanent independence honesty note: same-model subagent panelists are a rigor framing recording critiquedBy = aiModel - convergence is claimed, critic independence never is.";
        :>> location = ".engine/skills/adversarial-panel-review/SKILL.md";
        :>> triggerCondition = "A design artifact carries enough scope or risk that one author's judgment should not be the last word - a multi-work-package plan, a schema or process change, a direction Decision - and the human has asked for panel review or accepted its cost. Not for small mechanical changes: a full panel is expensive by design.";
        :>> writePolicy = WritePolicy::direct;
    }
}
