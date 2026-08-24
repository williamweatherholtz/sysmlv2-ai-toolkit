// The distributed-collaboration skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryDistributedCollaboration {
    private import EngineElement::*;
    private import EngineSkills::*;

    part distributedCollaborationSkill : AISkill {
        :>> id = "0d610002-0812-4102-9b3e-000000110002";
        :>> title = "distributed-collaboration";
        :>> purpose = "Deploy the Distributed Collaboration session loop (D0129) for an asynchronous remote mostly-AI team where the git remote is the ONLY coordination medium: SYNC (fetch, state divergence, integrate ancestry-preservingly — never rebase, since evidence is anchored to commit identity — check gate parity, then orient); CLAIM (take work visibly before starting; the remote's atomic ref update supplies mutual exclusion, so first-to-land wins and no lock server is needed); WORK (sprint ceremony under D0108's ownership contract, with true provenance); LAND by LANE (facts to the trunk via the gate-on-merged-tree loop; definitional change + deliverable source via a reviewed proposal with an independent reviewer and recorded human acceptance); ESCALATE (textual conflicts resolved then fully re-gated; semantic conflicts recorded as an Issue for HUMAN adjudication, never settled by precedence); CLOSE (release the claim, leave nothing in conversation — D0018). The deploying skill for .engine/processes/distributed-collaboration.sysml.";
        :>> location = ".engine/skills/distributed-collaboration/SKILL.md";
        :>> triggerCondition = "A work session begins on a project with more than one contributor; a push is rejected; a conflict or contention arises; or work is about to be landed.";
        :>> writePolicy = WritePolicy::direct;
    }
}
