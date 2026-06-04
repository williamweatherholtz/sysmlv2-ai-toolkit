// =====================================================================
// Engine::Core::Skills
// AI skills / agents as first-class workflow elements. The engine is
// "AI-complemented" — this is core, not optional.
// SYNTAX PENDING PILOT VALIDATION.
// =====================================================================
package Engine {
package Core {

    // -----------------------------------------------------------------
    // AISkill — a workflow-side pointer to an executable prompt/skill on
    // disk. The .sysml side carries WORKFLOW CONTEXT (trigger, write
    // policy); the actual logic lives at `location`.
    //
    // writePolicy is ENFORCED by the API (decision 0003). A skill
    // cannot self-promote its policy — it is set here and changing it
    // requires a human edit:
    //   "read-only"  may query, never write
    //   "pr-only"    writes go to a branch + PR for human review
    //   "direct"     may commit to main (reserve for mechanical
    //                bookkeeping like staleness flagging)
    // -----------------------------------------------------------------
    part def AISkill {
        // @Tracked
        attribute name : String;
        attribute purpose : String;
        attribute location : String;      // path to SKILL.md / prompt
        attribute trigger : String;       // when it fires
        attribute writePolicy : String;
    }

    part def Agent :> AISkill {
        doc /* A skill that may invoke sub-skills / sub-agents. */
        attribute subagents : String[*];  // qualified names of child AISkills
    }

}
}
