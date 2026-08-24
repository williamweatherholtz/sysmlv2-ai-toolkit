// The knowledge-graph-memory skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryKnowledgeGraphMemory {
    private import EngineElement::*;
    private import EngineSkills::*;

    part knowledgeGraphMemory : AISkill {
        :>> id = "ec6f83bf-1a8e-438a-9f5c-79655c18bb49";
        :>> title = "knowledge-graph-memory";
        :>> purpose = "Deploy the Knowledge-Graph Memory process (D0153): derive a small closed ontology from the QUESTIONS a corpus must answer, extract a typed graph with computed identity, and INJECT the recalled facts before the model thinks — so retrieval is code rather than conversation and the model never does the traversal. The deploying skill for .engine/processes/knowledge-graph-memory.sysml.";
        :>> location = ".engine/skills/knowledge-graph-memory/SKILL.md";
        :>> triggerCondition = "An assistant must answer questions spanning documents that share no vocabulary; search returns wrong answers or right ones slowly; a request for memory / RAG / a knowledge graph over docs.";
        :>> writePolicy = WritePolicy::direct;
    }
}
