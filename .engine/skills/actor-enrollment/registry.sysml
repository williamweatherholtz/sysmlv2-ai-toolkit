// The actor-enrollment skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryActorEnrollment {
    private import EngineElement::*;
    private import EngineSkills::*;

    part actorEnrollmentSkill : AISkill {
        :>> id = "0d610001-0812-4101-9b3e-000000110001";
        :>> title = "actor-enrollment";
        :>> purpose = "Deploy the Actor Enrollment process (D0129): take an uninitiated contributor — human or AI, on a machine that has never run the engine — to truthful authoring. Establish the acting identity EXPLICITLY (refuse if indeterminate; git identity is a suggestion, never authority); ASK actor KIND (human/ai) and ROLE rather than inferring, since kind is the only thing protecting human-only attestation authority; register the correctly-typed actor without duplicating; BIND the machine so no write path needs a default; and PROVE the local gate runs. Closes issue072 (write paths defaulting the actor to a named human) + issue073 (every enrolled actor registered as Person). Distinct from and PRIOR to `introduction`, which onboards a fresh PROJECT rather than establishing who is acting. The deploying skill for .engine/processes/actor-enrollment.sysml.";
        :>> location = ".engine/skills/actor-enrollment/SKILL.md";
        :>> triggerCondition = "A contributor acts from a machine with no engine identity binding; a new human or AI joins the project; `keel` reports no acting actor; or provenance/authority is in question.";
        :>> writePolicy = WritePolicy::direct;
    }
}
