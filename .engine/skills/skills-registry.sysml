// =====================================================================
// EngineSkillRegistry — the AI skills that ACTUALLY EXIST on disk.
//
// CR-10 (2026-06-11): the four phantom registrations (triage, staleness-sweep,
// implementer, grill-me — none existed) are gone; only skills that ACTUALLY
// EXIST on disk are registered (critique P1). The set has since grown (ceremony
// skills D0039; build + test-verify D0046/issue008). writePolicy is the INTENDED
// write boundary — enforcement arrives with the write API (D0003); until then it
// binds by discipline (CLAUDE.md §4).
//
// CONVENTION (D0059/issue036, process-skill guard): a skill that DEPLOYS a process MUST name the
// `.engine/processes/<name>.sysml` it deploys in its `purpose` — that path reference is what the
// "every process has a deploying skill (no inert process)" audit checks. Ceremony processes
// (definition-of-ready / definition-of-done / agile-workflow) are deployed by the sprint-* skills
// (D0059); their purposes name those process files too.
// =====================================================================
package EngineSkillRegistry {
    private import EngineElement::*;     // WritePolicy enum
    private import EngineSkills::*;

    // decisionChannelSkill MOVED to .engine/skills/decision-channel/registry.sysml (D0220), so the
    // unit carries its own registration and an adopting project does not land with a red
    // `process-skill` gate it can only fix by hand-editing this shared file. Moved, never copied:
    // duplicate-identity caught the copy on the first edit, which is the guard doing its job.

}
