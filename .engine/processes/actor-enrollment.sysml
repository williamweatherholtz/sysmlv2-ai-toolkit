// =====================================================================
// EngineActorEnrollment — enroll an ACTOR (human or AI) onto an existing project, on a machine that
// has never run the engine (D0129). This is the FRONT DOOR of distributed operation: attribution,
// acceptance authority and gate uniformity all presuppose a correctly-registered actor on a working
// machine, and every remote contributor begins in exactly the unenrolled state.
//
// DISTINCT FROM `introduction` (.engine/processes/introduction.sysml), which onboards a fresh
// PROJECT from zero to first value and never establishes WHO is acting. The two COMPOSE, enrollment
// first: enroll (who am I, what may I attest, does my gate work) -> then either `introduction` on a
// new project, or `distributed-collaboration` on a populated one.
//
// Supersedes the capture_user.py capture path, which resolved identity from git only, asked no kind
// question, registered EVERY actor as `: Person` (issue073), and whose output nothing consumed
// (issue072 — the gap was binding, not capture). Deployed by the actor-enrollment skill.
// =====================================================================
package EngineActorEnrollment {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    part actorEnrollment : Process {
        :>> id = "0d5f0001-0812-4f01-9b3e-00000000f001";
        :>> title = "Actor enrollment (identity, kind, role, binding, gate)";
        :>> purpose = "Take an uninitiated contributor — human or AI, on a machine that has never run the engine — to the point of being able to author truthfully: acting identity established, kind and role recorded, the actor registered, the machine bound to that identity so later facts are attributed correctly, and the local gate proven to run. Self-service by design (supervisors are asynchronous, D0129), and REFUSING rather than enrolling into indeterminate identity or a silently-dead gate. This is the process that makes provenance true (nProvenanceTrue) and authority checkable (nAuthorityFollowsRecordedKind); every other guarantee depends on it.";
        :>> cadence = Cadence::adHoc;
    }

    part enIdentify : ProcessStep {
        :>> id = "0d5f0002-0812-4f02-9b3e-00000000f002";
        :>> title = "Establish the acting identity — explicitly, or refuse";
        :>> order = 1;
        :>> triggerCondition = "A human or AI is about to act on the project from a machine with no engine identity binding, or `keel` reports no acting actor.";
        :>> actionText = "Determine WHO is acting, from an explicit source: an existing machine-local binding, an explicit argument, or an interactive answer. Git committer identity may be OFFERED as a suggestion but is never authoritative on its own — on an agent's machine it is merely whatever that machine was configured with (the root of issue073). If the identity cannot be established explicitly, REFUSE and say so: do not fall back to any existing actor, and never to a named human. Unstated identity is the condition that makes confirmation-authenticity satisfiable by a false premise (issue072/issue073), so refusing is the correct outcome, not a failure.";
        :>> producedArtifact = "An explicitly-established acting identity (name + contact), or an honest refusal naming what is missing.";
        :>> owner = Owner::either;
    }
    part enClassify : ProcessStep {
        :>> id = "0d5f0003-0812-4f03-9b3e-00000000f003";
        :>> title = "Record actor KIND and ROLE explicitly — never infer";
        :>> order = 2;
        :>> triggerCondition = "The acting identity is established.";
        :>> actionText = "ASK, and record, two things that must never be inferred: (1) KIND — is this actor a human or an AI? This single field determines what the actor may attest: human-only authority (accepting a Decision, dispositioning a finding at or above the threshold, adjudicating a contributor conflict) is protected by nothing else. Registering an AI as a human Person must be IMPOSSIBLE here, not merely discouraged. (2) ROLE — what function does this actor fill while doing tracked work (contributor, supervisor, integrator, reviewer, or a project-declared role)? Role is model data, not tribal knowledge. Neither may be defaulted, and neither may be derived from git identity, the machine, or the actor's own unverified assertion of authority.";
        :>> producedArtifact = "A recorded actor kind (human | ai) and role, both explicitly stated rather than inferred.";
        :>> owner = Owner::either;
    }
    part enRegister : ProcessStep {
        :>> id = "0d5f0004-0812-4f04-9b3e-00000000f004";
        :>> title = "Register the actor in the project registry (without duplicating)";
        :>> order = 3;
        :>> triggerCondition = "Identity, kind and role are established.";
        :>> actionText = "Register the actor in the project's actor registry as the correctly-typed part — a Person for a human, an Actor with kind=ai for an AI — carrying name, contact where applicable, and role. If the actor is ALREADY registered, confirm and proceed without creating a second entry: duplicate identity is its own corruption class (issue074), and an enrollment step that duplicates is a defect. Registration is what makes the actor referenceable by authoredBy / createdBy / judgedBy, which the actors guard already enforces referentially.";
        :>> producedArtifact = "A registry entry for the actor, correctly typed by kind, with no duplicate.";
        :>> owner = Owner::ai;
    }
    part enBind : ProcessStep {
        :>> id = "0d5f0005-0812-4f05-9b3e-00000000f005";
        :>> title = "Bind the machine to the actor so writes are attributed truthfully";
        :>> order = 4;
        :>> triggerCondition = "The actor is registered.";
        :>> actionText = "Write the machine-local binding (configuration or environment) that makes this actor the acting identity for every subsequent write from this machine, so no write path needs a default and none is offered one. This is the step whose absence is issue072: capture already existed and printed an actor id, but nothing CONSUMED it, so the write paths kept defaulting — to a named human in seven places. After binding, verify by making the write path REPORT the actor it would attribute to, and confirm it is this actor.";
        :>> producedArtifact = "A machine-local identity binding, verified by reading back the actor the write path would attribute to.";
        :>> owner = Owner::ai;
    }
    part enVerifyGate : ProcessStep {
        :>> id = "0d5f0006-0812-4f06-9b3e-00000000f006";
        :>> title = "Prove the local gate actually runs, and matches what everyone is held to";
        :>> order = 5;
        :>> triggerCondition = "The machine is bound to a registered actor.";
        :>> actionText = "Run the gate and confirm it EXECUTES rather than skipping: the engine binary is present and locatable, the enforced check set matches what the project declares, the engine version matches the project's declared version, and any optional deep-semantic layer is either available or explicitly reported as unavailable. A check that cannot run must report NOT READY with the specific remedy — never pass silently, which is the issue076 failure where the local gate becomes nothing and only post-hoc CI remains. Do not proceed to authoring on an unverified gate: a contributor whose gate silently does nothing is held to a different standard than everyone else, which breaks nUniformGate for the whole team, not just for them.";
        :>> producedArtifact = "A verified gate-parity result: ready, or NOT ready with the specific remedy.";
        :>> owner = Owner::ai;
    }
    part enHandoff : ProcessStep {
        :>> id = "0d5f0007-0812-4f07-9b3e-00000000f007";
        :>> title = "Hand off to the working loop";
        :>> order = 6;
        :>> triggerCondition = "Identity, kind, role, binding and gate are all established and verified.";
        :>> actionText = "State plainly what the actor may and may not attest given its recorded kind and role (an AI actor cannot record human acceptance — say so explicitly rather than letting it be discovered at a refusal), then hand off: on a freshly-scaffolded project, to the `introduction` process (zero to first value); on a populated project, to `distributed-collaboration` (sync -> claim -> work -> land). Enrollment is COMPLETE when the actor can author a fact that is attributed to it truthfully and gated identically to everyone else's.";
        :>> producedArtifact = "An enrolled, gate-verified actor handed off to the appropriate working process, with its attestation limits stated.";
        :>> owner = Owner::either;
    }
}
