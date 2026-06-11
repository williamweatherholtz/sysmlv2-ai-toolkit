// =====================================================================
// EngineSkills — AI skills / agents as first-class workflow elements. The
// engine is AI-complemented; skills are task tools it invokes (CLAUDE.md §3).
// writePolicy is the enforced write boundary (decision 0003); a skill cannot
// self-promote it. (trigger -> triggerCondition.)
// =====================================================================
package EngineSkills {
    doc /* AI skills/agents as first-class elements. writePolicy is the intended write boundary - enforced by discipline until the write API exists (D0003). */

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
