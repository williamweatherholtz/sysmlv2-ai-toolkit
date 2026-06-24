// =====================================================================
// EngineSkills — AI skills / agents as first-class workflow elements. The
// engine is AI-complemented; skills are task tools it invokes (CLAUDE.md §3).
// writePolicy is a DECLARED convention the agent follows (read-only|pr-only|direct);
// authorization itself is the human COMMIT-GATE — the agent never auto-commits (D0096
// supersedes d0003: not API-enforced; D0093 dual-surface / D0094). (trigger -> triggerCondition.)
// =====================================================================
package EngineSkills {
    doc /* AI skills/agents as first-class elements. writePolicy is a declared convention the agent follows; the authorization boundary is the human commit-gate (the agent never auto-commits), NOT API enforcement — D0096 supersedes d0003 (D0093 dual-surface + D0094). */

    private import ScalarValues::*;
    private import EngineElement::*;

    part def AISkill :> Element {
        attribute purpose : String;
        attribute location : String;           // path to SKILL.md / prompt
        attribute triggerCondition : String;   // when it fires
        attribute writePolicy : WritePolicy;
    }

    part def Agent :> AISkill {
        ref subagents : AISkill[*];             // typed refs, not name-strings (M4)
    }
}
