// =====================================================================
// ProjectDistributedCollaboration — the Business-workflow chain for operating keel as an
// ASYNCHRONOUS, REMOTE, MOSTLY-AI DISTRIBUTED team (human direction 2026-08-12): several AI agent
// sessions on different machines, supervised by different humans in different timezones, with NO
// shared runtime and NO shared database — the git remote is the only coordination medium.
//
// Authored in the DECLARED succession (BusinessWorkflow::Business): personas -> needs -> useCases.
// (The project Brief already exists — ProjectBusiness::projectBrief; this is a new stakeholder
// context over the same brief, not a new project.) SystemRequirements live in
// .tracking/requirements/distributed-collaboration-requirements.sysml and `satisfy` these Needs.
// Governed by D0129, which also supersedes D0014 (main-only/no-branches — a SOLO-scoped decision).
//
// Every Need below traces to a VERIFIED defect or gap found by the 2026-08-12 research pass
// (external prior-art lane + internal code audit); the evidence is recorded in D0129's context and
// in the issues it raises. Needs are deliberately SOLUTION-FREE — the mechanisms are the SRs.
// =====================================================================
package ProjectDistributedCollaboration {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    // ====================================================================
    // Phase 2: Personas  (the Brief is ProjectBusiness::projectBrief)
    // ====================================================================
    part remoteAiContributor : Persona {
        :>> id = "0d5a0001-0812-4a01-9b3e-00000000a001";
        :>> title = "Remote AI Contributor";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "An AI agent session running on its own machine and its own working copy, with no shared runtime, no shared filesystem and no shared database with any other contributor. It reads the model, computes orientation, claims work, authors facts through the write API, and integrates through the git remote — which is the ONLY channel through which it can learn what anyone else has done. It cannot be assumed to be online when another contributor acts, and it may stop mid-work without notice. Distinct from the existing aiAgentPersona (a single agent on the human's own machine, which could assume its working copy WAS the model).";
        :>> goals = "Do a full unit of work — orient, claim, execute, record, integrate — without duplicating another contributor's work, without silently overwriting their facts, and without needing anyone to be awake.";
    }

    part asyncSupervisor : Persona {
        :>> id = "0d5a0002-0812-4a02-9b3e-00000000a002";
        :>> title = "Asynchronous Supervising Human";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A human who supervises one or more remote AI contributors from a different timezone, typically hours after the work was done. They hold the authority that cannot be delegated to an agent: accepting Decisions, dispositioning findings, and adjudicating between contributors who reached conflicting conclusions. They review in BATCHES when they are next available, and they must never become the thing every agent waits on. Distinct from the existing teamLead persona, who READS live status; this persona EXERCISES authority over contested and non-test-verifiable judgment.";
        :>> goals = "Exercise real oversight — accept, disposition, adjudicate — in batched sessions on their own schedule, without stalling contributors and without being handed decisions an automated check could have made.";
    }

    part uninitiatedContributor : Persona {
        :>> id = "0d5a0003-0812-4a03-9b3e-00000000a003";
        :>> title = "Uninitiated Contributor (human or AI)";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A person or AI agent arriving at the project for the first time, on a machine that has never run the engine. They are not in the actor registry, their machine has no engine identity, their local gate may be absent or misconfigured, and they do not know the working discipline. Until they are enrolled and their gate verified, anything they author carries false or unattributable provenance and may bypass checks every other contributor is held to. Distinct from the newcomer served by the existing `introduction` process, which onboards a fresh PROJECT (zero to first value); this persona needs to be onboarded as an ACTOR onto an existing, populated project.";
        :>> goals = "Become a recognized, correctly-classified contributor with a working local gate, and reach the point of doing real work, without a human walking them through it and without silently acquiring authority they should not have.";
    }

    // ====================================================================
    // Phase 3: Needs  (solution-free; each traces to a verified finding)
    // ====================================================================
    requirement nComputedStateMachineIndependent : Need {
        :>> id = "0d5b0001-0812-4b01-9b3e-00000000b001";
        :>> title = "Computed state is the same for every contributor, whoever computes it";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "Given the same synchronized model, every contributor computing the same view obtains the same answer. What is done, ready, suspect, or covered is a property of the model — never a property of which machine or working copy ran the query.";
        :>> marketJustification = "This is the engine's foundational claim (state is COMPUTED, never authored — CLAUDE.md 2.1). If the computed answer varies by machine, the claim is void and no contributor can trust any view; the architecture would be strictly worse than the prose documents it replaces, because the divergence is invisible.";
        :>> priority = Priority::must;
    }

    requirement nIntegrationEvidencePreserving : Need {
        :>> id = "0d5b0002-0812-4b02-9b3e-00000000b002";
        :>> title = "Integrating a contribution never invalidates the evidence it carries";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "Bringing one contributor's work together with another's leaves every verification record it carries as valid, and as resolvable, as it was before integration. Evidence recorded once stays evidence.";
        :>> marketJustification = "Recorded evidence is anchored to the commit it was judged against (D0013), so an integration step that rewrites commit identity destroys the anchor. The work would have to be re-verified purely because someone else pushed first — a cost that grows with contributor count and silently converts finished work back into outstanding work.";
        :>> priority = Priority::must;
    }

    requirement nProvenanceTrue : Need {
        :>> id = "0d5b0003-0812-4b03-9b3e-00000000b003";
        :>> title = "Every recorded fact is attributable to the actor that actually produced it";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "The actor recorded against an authored fact or a judgment is the actor that produced it, and is classified correctly as human or AI. No contributor can record a fact as though another actor produced it, and no default or omission causes an attribution that was never made.";
        :>> marketJustification = "The engine's honesty guarantees are all built on attribution: independent critique requires knowing who critiqued (D0080), and confirmation authenticity requires knowing that an acceptance was judged by a HUMAN (D0106). If attribution can be wrong — especially wrong by default — those guards do not fail loudly, they simply stop meaning anything, and an AI can hold authority reserved for a human.";
        :>> priority = Priority::must;
    }

    requirement nConcurrentAuthoringSafe : Need {
        :>> id = "0d5b0004-0812-4b04-9b3e-00000000b004";
        :>> title = "Contributors authoring at the same time never silently corrupt each other";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "Two contributors recording facts concurrently either both succeed with both facts intact, or the conflict is surfaced and must be resolved. No concurrent authoring silently drops a fact, overwrites another actor's item, or produces a model that validates while being internally inconsistent.";
        :>> marketJustification = "Silent corruption is the worst failure class available to a system of record, because it is indistinguishable from correctness until something downstream depends on it. Concurrent allocation of the same identity is especially dangerous: it produces no conflict to resolve, so nothing warns anyone.";
        :>> priority = Priority::must;
    }

    requirement nNoDuplicateWork : Need {
        :>> id = "0d5b0005-0812-4b05-9b3e-00000000b005";
        :>> title = "Contributors divide the work without two of them doing the same thing";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A contributor choosing what to work on can tell what is already being worked on and by whom, and can take a piece of work in a way that is visible to everyone else before they begin. Work abandoned by a contributor who stops becomes available again without a human intervening.";
        :>> marketJustification = "With one contributor, the ready frontier is unambiguous. With several, an unowned frontier means every contributor rationally starts the same highest-value item, so throughput collapses toward that of a single contributor while consuming the cost of all of them — and the duplicated effort surfaces only at integration, as a conflict.";
        :>> priority = Priority::must;
    }

    requirement nUniformGate : Need {
        :>> id = "0d5b0006-0812-4b06-9b3e-00000000b006";
        :>> title = "Every contributor is held to the same checks, on every machine";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "The checks that gate a contribution are the same for every contributor regardless of operating system, toolchain layout, or local configuration, and a contributor can tell whether their gate is actually working. A gate that cannot run reports that fact rather than passing silently.";
        :>> marketJustification = "A gate that silently degrades to nothing on some machines is worse than no gate, because the discipline is believed to be enforced when it is not. A gate that blocks work on some machines is equally corrosive: contributors learn to bypass it, which defeats it everywhere at once.";
        :>> priority = Priority::must;
    }

    requirement nContentionVisible : Need {
        :>> id = "0d5b0007-0812-4b07-9b3e-00000000b007";
        :>> title = "Contention and conflicting conclusions surface for human adjudication";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "When contributors contend for the same work, or reach conclusions that cannot both be true, that condition becomes a visible tracked item addressed to a human. It is never resolved by whichever contributor happened to act last, and never resolved by a contributor judging its own claim against another's.";
        :>> marketJustification = "Contention between autonomous contributors is not an error to be suppressed but information about the work — usually an ambiguity upstream. Resolving it by precedence discards that information and lets one agent's reasoning silently override another's, with no record that a disagreement ever existed.";
        :>> priority = Priority::should;
    }

    requirement nAsyncHumanOversight : Need {
        :>> id = "0d5b0008-0812-4b08-9b3e-00000000b008";
        :>> title = "A supervisor exercises authority asynchronously and in batch, without blocking contributors";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A supervising human can see everything awaiting their authority, and how long it has waited, and can discharge it in a batch on their own schedule. Contributors continue working while items await that authority, and are presented for human judgment only what an automated check genuinely cannot decide.";
        :>> marketJustification = "Human judgment is the scarcest resource in a mostly-AI team and the only one that cannot be scaled by adding agents. If it sits on the critical path of each unit of work, the team's throughput becomes the supervisor's availability; if it is asked for indiscriminately, it degrades into rubber-stamping, which is worse than not asking.";
        :>> priority = Priority::must;
    }

    requirement nActorEnrollment : Need {
        :>> id = "0d5b0009-0812-4b09-9b3e-00000000b009";
        :>> title = "An uninitiated human or AI can become a correctly-classified contributor unaided";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A newcomer arriving on a machine that has never run the engine can establish who they are, what kind of actor they are, and what role they hold; have that recorded in the model; have their machine bound to that identity so their subsequent facts are attributed to them; and confirm their local gate works — before authoring anything. None of this requires another contributor to be present.";
        :>> marketJustification = "Enrollment is the gate through which every other guarantee is established: attribution, acceptance authority, and gate uniformity all presuppose a correctly-registered actor on a working machine. An unenrolled contributor does not merely lack privileges — it silently pollutes the record and can acquire authority it should not have, and every remote contributor starts in exactly that state.";
        :>> priority = Priority::must;
    }

    requirement nAuthorityFollowsRecordedKind : Need {
        :>> id = "0d5b000a-0812-4b0a-9b3e-00000000b00a";
        :>> title = "What an actor may attest follows from their recorded kind and role";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "Whether an actor may perform a given attestation — accepting a Decision, dispositioning a finding, adjudicating a conflict — is determined by the kind and role recorded for that actor in the model, and is checkable from the model alone. It is not a matter of convention, instruction, or the actor's own assertion.";
        :>> marketJustification = "Human-only authority is currently protected by the registry classifying actors correctly, which enrollment must therefore never get wrong. Making authority follow explicitly from recorded kind and role turns it from a convention that holds while everyone remembers it into a property that can be checked — and lets a distributed team hold differentiated roles without inventing a second, unmodeled permission system.";
        :>> priority = Priority::should;
    }

    // ====================================================================
    // Phase 4: Use Cases
    // ====================================================================
    use case ucDcEnroll : UseCase {
        :>> id = "0d5c0001-0812-4c01-9b3e-00000000c001";
        :>> title = "Enroll an uninitiated contributor (identity, kind, role, gate)";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A human or AI arrives on a fresh machine and becomes a recognized contributor: identity established, kind (human/AI) and role recorded, the machine bound to that identity, and the local gate proven to run.";
        :>> mainSuccess = "Run enrollment -> it determines the acting identity, ASKS whether the actor is a human or an AI and what role they hold rather than assuming, registers the actor in the registry if absent, writes the machine-local identity binding so later writes are attributed correctly, verifies the local gate actually executes, and reports readiness to work.";
        :>> extensions = "Already-registered actor: confirms and re-binds without duplicating the registry entry. Gate cannot run (missing binary, missing toolchain): reports it as NOT ready with the specific remedy, rather than enrolling into a state where checks silently do not run. Identity indeterminate: refuses rather than defaulting to any existing actor.";
    }
    use case ucDcSyncAtSessionStart : UseCase {
        :>> id = "0d5c0002-0812-4c02-9b3e-00000000c002";
        :>> title = "Synchronize and orient at session start";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A contributor starting a session brings its working copy up to date with everyone else's landed work BEFORE computing orientation, so it is not reasoning about a stale model.";
        :>> mainSuccess = "Sync -> fetch, report divergence from the remote, integrate remote work in a way that preserves evidence anchors, confirm the engine version and gate match, then compute orient on the synchronized tree.";
        :>> extensions = "Local unpushed work present: integrate remote work into it without rewriting local commit identity. Divergence too large or conflicted to integrate automatically: stop and surface it rather than orienting from a half-merged tree.";
    }
    use case ucDcClaimWork : UseCase {
        :>> id = "0d5c0003-0812-4c03-9b3e-00000000c003";
        :>> title = "Claim a ready item so others can see it is taken";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A contributor takes a piece of work from the ready frontier and makes that visible to every other contributor before starting, so nobody else begins the same item.";
        :>> mainSuccess = "Choose from the ready frontier minus items already claimed -> record the claim as an authored fact (who, what, when, at which commit) -> land it immediately so it is visible remotely -> begin work.";
        :>> extensions = "The claim fails to land because someone else's work arrived first: re-read the claims and choose again. Nothing unclaimed remains ready: report that rather than taking claimed work.";
    }
    use case ucDcRaceForSameItem : UseCase {
        :>> id = "0d5c0004-0812-4c04-9b3e-00000000c004";
        :>> title = "Two contributors claim the same item at the same time";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "Two contributors independently compute the same ready frontier and attempt to claim the same top item within the same moment. Exactly one must proceed, without a lock server and without a human arbitrating.";
        :>> mainSuccess = "Both attempt to land their claim; the remote accepts exactly one; the loser's attempt is rejected, so it re-synchronizes, observes the winning claim, and selects different work. No duplicated effort, no human involved.";
        :>> extensions = "Repeated rejection under heavy contention: back off rather than spinning, and report the contention. Both claims somehow land (distinct anchors): the condition is detected and surfaced as contention for adjudication, not silently tolerated.";
    }
    use case ucDcLandFacts : UseCase {
        :>> id = "0d5c0005-0812-4c05-9b3e-00000000c005";
        :>> title = "Land recorded facts while others are landing theirs";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A contributor integrates its recorded facts (verification results, ceremony gates, new items and edges) into the shared trunk while other contributors are doing the same, without a red trunk and without bypassing the gate.";
        :>> mainSuccess = "Fetch, integrate remote work, re-run the FULL gate against the resulting tree — the exact state that will land, not the state that was tested in isolation — and push. Rejected because someone landed first: repeat.";
        :>> extensions = "The gate fails only after integration (each contribution was green alone, together they are not): the push does not happen, and the semantic conflict is resolved or escalated. Retries exhausted: back off and report rather than forcing.";
    }
    use case ucDcResolveTextualConflict : UseCase {
        :>> id = "0d5c0006-0812-4c06-9b3e-00000000c006";
        :>> title = "Resolve a textual conflict in a shared model file";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "Two contributors appended records to the same region of the same shared file. The conflict must be resolved so that BOTH facts survive and the resulting model is semantically sound — not merely textually clean.";
        :>> mainSuccess = "Both records are kept; the resolution is then subjected to the full gate before it can land, because a textually plausible resolution can still produce duplicate identity or a broken reference.";
        :>> extensions = "The resolution drops or merges a record: caught by the gate as a lost or duplicated fact rather than landing. Conflicts recur constantly on the same file: that is a signal to separate the write targets, recorded as a tracked observation rather than absorbed as routine friction.";
    }
    use case ucDcConflictingConclusions : UseCase {
        :>> id = "0d5c0007-0812-4c07-9b3e-00000000c007";
        :>> title = "Two contributors reach conclusions that cannot both be true";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "Two contributors, working from the same model, record judgments or decisions that contradict each other. This is not a merge conflict — both may integrate cleanly — and it must not be settled by whichever landed second.";
        :>> mainSuccess = "The contradiction is recorded as a tracked issue addressed to a human, with both positions and their bases preserved; it appears in what awaits human authority; the supervisor adjudicates and the outcome is recorded as a decision. Neither contributor's conclusion silently wins.";
        :>> extensions = "The contradiction is discovered later, after both landed: it is still recorded and adjudicated, and anything derived from the losing conclusion is re-examined. A contributor notices its OWN conclusion is contradicted: it records the conflict rather than quietly conceding or overriding.";
    }
    use case ucDcStaleClaim : UseCase {
        :>> id = "0d5c0008-0812-4c08-9b3e-00000000c008";
        :>> title = "A contributor goes dark mid-work and its claim goes stale";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A contributor claims an item and then stops — session ended, machine offline, agent crashed — without releasing it. The work must become available again without a human noticing and intervening.";
        :>> mainSuccess = "The claim is computed as stale once it has been held beyond its expiry without progress; the item returns to the ready frontier; the staleness is visible so it is understood as an abandoned claim rather than an active one.";
        :>> extensions = "The original contributor returns and resumes: it re-synchronizes and discovers whether its claim expired and was retaken. Partial work was landed before going dark: it stays recorded — expiry releases the CLAIM, never the facts.";
    }
    use case ucDcChangeThroughReview : UseCase {
        :>> id = "0d5c0009-0812-4c09-9b3e-00000000c009";
        :>> title = "A definitional change goes through independent review before landing";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A contributor proposes a change to the schema, a process, a rule, or the meaning of a computed view — the class where an error propagates to every contributor at once. It is reviewed by someone other than its author before it becomes shared truth.";
        :>> mainSuccess = "The change is proposed as a separate, reviewable unit; the gate runs on it; a reviewer that is NOT its author examines it; the human acceptance the change path already requires is recorded against it; it integrates preserving evidence anchors.";
        :>> extensions = "No independent reviewer is available: the change waits rather than self-approving. The author attempts to record its own review or acceptance: refused, because author and reviewer must differ.";
    }
    use case ucDcBatchReview : UseCase {
        :>> id = "0d5c000a-0812-4c0a-9b3e-00000000c00a";
        :>> title = "A supervisor discharges accumulated authority in one batched sitting";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A supervisor, hours or days after the work, reviews everything genuinely requiring human authority across several contributors and discharges it in one session, in their own timezone.";
        :>> mainSuccess = "See everything awaiting human authority — decisions to accept, findings to disposition, conflicts to adjudicate — with how long each has waited and which contributor produced it; act on them in a batch; contributors observe the outcomes on their next synchronization.";
        :>> extensions = "The queue contains items that an automated check could have decided: that is itself a defect to correct, because asking for unnecessary human judgment causes rubber-stamping. An item has waited beyond its threshold: it is visibly escalated rather than aging silently.";
    }
    use case ucDcVerifyAfterOthersLanded : UseCase {
        :>> id = "0d5c000b-0812-4c0b-9b3e-00000000c00b";
        :>> title = "Re-establish what is still verified after other contributors' work lands";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "After integrating other contributors' work, a contributor determines what evidence is still current, what has genuinely drifted, and — distinctly — what it simply cannot see yet because it has not been synchronized.";
        :>> mainSuccess = "Compute suspicion and evidence validity on the synchronized tree; evidence still anchored is current; evidence whose subject changed is suspect and re-verifiable; evidence whose anchor is not present locally is reported as UNRESOLVED-HERE, distinct from invalid.";
        :>> extensions = "An anchor is unresolvable because it was never pushed: reported as unsynchronized, not as failed verification, and never counted as work-not-done. An anchor is unresolvable because history was rewritten: surfaced as a defect in integration, since that must not be possible.";
    }
    use case ucDcOnboardMidWeek : UseCase {
        :>> id = "0d5c000c-0812-4c0c-9b3e-00000000c00c";
        :>> title = "Add a contributor to work already in progress";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A new contributor joins while several sprints are already in flight, and must reach useful work without disturbing anyone else's claims or re-treading their reasoning.";
        :>> mainSuccess = "Enroll, synchronize, orient -> see what is in flight, who holds what, and what remains unclaimed -> claim unclaimed work and proceed. The reasoning behind in-flight work is readable from recorded decisions and items, with no verbal handoff.";
        :>> extensions = "Nothing is unclaimed: the newcomer takes up review, critique, or adjudication-support work rather than contending for a held item. Understanding a decision's basis requires context that was never recorded: that gap is recorded as an issue, because unrecorded reasoning is the defect.";
    }
    use case ucDcGateParity : UseCase {
        :>> id = "0d5c000d-0812-4c0d-9b3e-00000000c00d";
        :>> title = "Confirm the local gate matches the gate everyone else is held to";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> description = "A contributor confirms, before authoring, that the checks on its machine are the same ones enforced for everyone — rather than discovering after the fact that its gate silently did nothing or blocked everything.";
        :>> mainSuccess = "Check gate parity -> the engine version, the available checks, and the enforced set are compared against what the project declares; a match reports ready; a mismatch reports exactly what differs and how to correct it.";
        :>> extensions = "The gate cannot run at all: reported as NOT ready, never as passing. The engine version differs from the project's: reported, because contributors on different versions can disagree about whether the same model is valid.";
    }

    // ── use cases realize the needs (DerivedFrom: use case -> need) ───
    #DerivedFrom dependency from ucDcEnroll to nActorEnrollment;
    #DerivedFrom dependency from ucDcSyncAtSessionStart to nComputedStateMachineIndependent;
    #DerivedFrom dependency from ucDcClaimWork to nNoDuplicateWork;
    #DerivedFrom dependency from ucDcRaceForSameItem to nNoDuplicateWork;
    #DerivedFrom dependency from ucDcLandFacts to nConcurrentAuthoringSafe;
    #DerivedFrom dependency from ucDcResolveTextualConflict to nConcurrentAuthoringSafe;
    #DerivedFrom dependency from ucDcConflictingConclusions to nContentionVisible;
    #DerivedFrom dependency from ucDcStaleClaim to nNoDuplicateWork;
    #DerivedFrom dependency from ucDcChangeThroughReview to nProvenanceTrue;
    #DerivedFrom dependency from ucDcBatchReview to nAsyncHumanOversight;
    #DerivedFrom dependency from ucDcVerifyAfterOthersLanded to nIntegrationEvidencePreserving;
    #DerivedFrom dependency from ucDcOnboardMidWeek to nActorEnrollment;
    #DerivedFrom dependency from ucDcGateParity to nUniformGate;

    // ── root the governing Decision to the Needs it answers (cf. d0094/issue046) ──
    // d0129 (the decision that adopts the distributed operating model) #DerivedFrom these Needs.
    // Authored HERE, not in .engine/decisions/0129, so the engine-side Decision file carries no
    // reference into this project's instance data.
    #DerivedFrom dependency from d0129 to nComputedStateMachineIndependent;
    #DerivedFrom dependency from d0129 to nIntegrationEvidencePreserving;
    #DerivedFrom dependency from d0129 to nProvenanceTrue;
    #DerivedFrom dependency from d0129 to nConcurrentAuthoringSafe;
    #DerivedFrom dependency from d0129 to nNoDuplicateWork;
    #DerivedFrom dependency from d0129 to nUniformGate;
    #DerivedFrom dependency from d0129 to nContentionVisible;
    #DerivedFrom dependency from d0129 to nAsyncHumanOversight;
    #DerivedFrom dependency from d0129 to nActorEnrollment;
    #DerivedFrom dependency from d0129 to nAuthorityFollowsRecordedKind;
}
