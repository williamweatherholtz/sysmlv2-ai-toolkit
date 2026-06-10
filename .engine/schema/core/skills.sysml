// =====================================================================
// EngineSkills — AI skills / agents as first-class workflow elements. The
// engine is AI-complemented; skills are task tools it invokes (CLAUDE.md §3).
// writePolicy is the enforced write boundary (decision 0003); a skill cannot
// self-promote it. (trigger -> triggerCondition.)
// =====================================================================
package EngineSkills {
    private import ScalarValues::*;
    private import EngineElement::*;

    part def AISkill :> Element {
        attribute purpose : String;
        attribute location : String;           // path to SKILL.md / prompt
        attribute triggerCondition : String;   // when it fires
        attribute writePolicy : String;        // read-only | pr-only | direct
    }

    part def Agent :> AISkill {
        attribute subagents : String[*];        // names of child AISkills
    }
}
