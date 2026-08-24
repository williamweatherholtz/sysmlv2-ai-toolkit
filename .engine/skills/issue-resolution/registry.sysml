// The issue-resolution skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryIssueResolution {
    private import EngineElement::*;
    private import EngineSkills::*;

    part issueResolutionSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a078-516273849612";
        :>> title = "issue-resolution";
        :>> purpose = "Deploy the Issue Resolution process (D0078/D0077): link every Issue to the work or Decision that resolves it via a #Resolves edge, let open/resolved state COMPUTE from resolver-completeness (never a stored status), and record decision-moots-item as a typed edge (#Resolves for an Issue, supersede for a Need/Requirement) — never prose. The deploying skill for .engine/processes/issue-resolution.sysml.";
        :>> location = ".engine/skills/issue-resolution/SKILL.md";
        :>> triggerCondition = "Recording/triaging an Issue; asked to resolve an issue or list open issues; a Decision moots an item; the `issues` guard fails (untriaged).";
        :>> writePolicy = WritePolicy::direct;
    }
}
