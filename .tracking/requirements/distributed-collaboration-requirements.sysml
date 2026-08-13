// =====================================================================
// ProjectDistributedCollaborationRequirements — the SystemRequirements that satisfy the
// distributed-collaboration Needs (ProjectDistributedCollaboration, .tracking/business/).
// Governed by D0129 (human direction 2026-08-12: operate keel as an async remote mostly-AI team).
//
// Each SR states a mechanism; its rationale names the VERIFIED defect it closes (file:line where the
// audit established it) so the requirement can be re-judged against evidence rather than opinion.
// The two-lane integration policy (facts direct-to-trunk via a gate-on-merged-tree loop; definitional
// change + deliverable code via reviewed proposal) is the human's recorded choice of 2026-08-12.
// =====================================================================
package ProjectDistributedCollaborationRequirements {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    // ── evidence preservation: the existential one ────────────────────
    requirement srDcMergeOnlyIntegration : SystemRequirement {
        :>> id = "0d5d0001-0812-4d01-9b3e-00000000d001";
        :>> title = "Integration preserves commit ancestry — merge only, never squash, rebase-merge, or force";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Integration of any contribution into a shared ref shall preserve the identity of the commits it carries: fast-forward or merge commit only. Squash-merge, rebase-merge, history rewriting, and force-push shall be disabled on the remote for shared refs and shall not be performed locally on commits that carry recorded evidence. The prohibition shall be enforced by remote configuration (branch protection + disabled merge methods), not by convention alone.";
        :>> rationale = "keel's evidence model DICTATES this; it is a consequence, not a style preference. A passing TestResult counts as done only while its `judgedAgainst` SHA still resolves (orient.rs:494-531, resolution via `git cat-file` at orient.rs:123). Squash and rebase mint new SHAs, so evidence recorded in a rewritten contribution is anchored to commits that are not ancestors of the trunk. The rewriting machine still finds them as loose objects and reports GREEN, while every other clone reports invalidEvidence — the same model yielding different answers per machine. Verified live: the remote currently permits BOTH squash and rebase merge and `main` has NO branch protection (gh api, 2026-08-12), so the hazard is live and ungated. Prior art confirms the shape: Gerrit introduced Change-Id precisely because SHAs die on rebase; git-notes needs notes.rewriteRef for the same reason.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    requirement srDcUnresolvedEvidenceClass : SystemRequirement {
        :>> id = "0d5d0002-0812-4d02-9b3e-00000000d002";
        :>> title = "Evidence that cannot be resolved locally is reported as unsynchronized, distinctly from invalid";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Computed views shall distinguish evidence whose anchor is ABSENT FROM THIS CLONE (not yet fetched) from evidence whose anchor is genuinely unresolvable, and shall not count the former as work-not-done. The distinction shall be visible in orient, and the unsynchronized class shall name what to fetch to resolve it.";
        :>> rationale = "Under partial synchronization — the normal state of a distributed team — a contributor routinely cannot see anchors for evidence another contributor has recorded but not yet pushed, or has pushed but this clone has not fetched. Today both collapse into `invalidEvidence`, which orient treats as not-done (orient.rs:38, 494-531), so finished work re-enters the ready frontier and is redone. Conflating 'I cannot see your evidence yet' with 'your evidence is broken' is precisely what makes the distributed view lie. Honest-state discipline (D0098) requires reporting what is actually true: unverifiable-from-here is not the same fact as unverified, and only one of them is a defect.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srDcSyncAwareOrient : SystemRequirement {
        :>> id = "0d5d0003-0812-4d03-9b3e-00000000d003";
        :>> title = "Orientation states its own freshness relative to the remote";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Orientation shall report how far the working copy diverges from the shared trunk (behind, ahead, diverged, and by how much) as part of its output, so every computed answer carries the synchronization state it was computed against. A contributor shall be able to synchronize and orient in one sanctioned step.";
        :>> rationale = "A computed answer is only as true as the tree it was computed from, and in a distributed team the tree is stale by default. Today orient reads the working copy with no reference to the remote, so a contributor cannot distinguish 'nothing is ready' from 'nothing is ready as far as I know, and I am forty commits behind'. Making freshness part of the answer is the compute-don't-store way to address this: the divergence is derived from git at query time, never stored.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    // ── concurrent authoring safety ───────────────────────────────────
    requirement srDcGateOnMergedTree : SystemRequirement {
        :>> id = "0d5d0004-0812-4d04-9b3e-00000000d004";
        :>> title = "The gate runs against the tree that will actually land";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Landing a contribution shall follow the loop: fetch, integrate the shared trunk, run the FULL enforced gate against the resulting tree, then push — retrying the loop when the push is rejected, with bounded retries and back-off. A contribution shall never land having been gated only in isolation.";
        :>> rationale = "Two contributions that each pass the gate alone can fail together — a semantic conflict — and this is the entire reason merge queues exist (the 'not-rocket-science rule'). Executing the loop client-side gives that guarantee with no server, which suits a git-only coordination medium. It also composes with git's own concurrency primitive: a non-fast-forward push rejection IS a compare-and-swap on the ref (the `--force-with-lease` flag was originally spelled `--cas`), so rejection is the correct, sufficient signal to re-run the loop rather than an error to work around. Today nothing fetches at all: post-commit pushes fire-and-forget and on rejection merely prints 'run git push when able' (.githooks/post-commit:8-9).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    requirement srDcUniqueIdentityGuard : SystemRequirement {
        :>> id = "0d5d0005-0812-4d05-9b3e-00000000d005";
        :>> title = "Duplicate identity is a blocked commit, not silent corruption";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "A hard-blocking guard shall detect, across the whole model, any repeated element `id`, any repeated declared item name within a package, any repeated package name across files, and any repeated allocated sequence identifier (decision number, issue number, sprint number). Detection shall not depend on the duplicates residing in the same file.";
        :>> rationale = "This is the failure class where the ABSENCE of a git conflict is the danger. Two contributors offline both mint the next decision number by scanning the directory (write.rs:929-942); different slugs give different filenames, so git reports no conflict, both land, and the resulting duplicate `package Decision0129` declarations are SILENTLY MERGED by the registry (keel-parser/src/registry.rs:52-53). The same shape applies to two `sprintNNN_*` files (both counted by in_progress_sprints, orient.rs:344) and to hand-appended issue numbers. Verified: no guard among the seventeen scans for duplicate ids or names. Git buys textual safety, not semantic safety; concurrent allocation of the same NAME needs its own control. Per D0047 a recurrable defect class requires a permanent automated guard, not vigilance.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srDcCollisionResistantIds : SystemRequirement {
        :>> id = "0d5d0006-0812-4d06-9b3e-00000000d006";
        :>> title = "Generated identifiers are collision-resistant across machines";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Generated element identifiers shall be drawn from a source with sufficient entropy to make collision across independent machines negligible — a cryptographically-seeded random UUID rather than a construction from local clock, process id, and in-process counter.";
        :>> rationale = "Identity is the engine's foundational invariant: items never collide on name because they are distinguished by `id` (CLAUDE.md 2.3), so an id collision corrupts identity itself. The current generator mixes only seconds, sub-second nanos, process id, and an in-process counter that is zero for the first record of every invocation (write.rs:63-99), and self-documents as 'not cryptographically random'. It contains NO host component, so two machines are not independent sources. Likelihood is low and consequence is severe and undetectable — which is exactly the profile that justifies fixing a one-function defect pre-emptively rather than waiting for evidence.";
        :>> kind = RequirementKind::nonFunctional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srDcPerActorWriteTargets : SystemRequirement {
        :>> id = "0d5d0007-0812-4d07-9b3e-00000000d007";
        :>> title = "Concurrent appends land at different anchors";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Write paths that append new records shall not all target the same anchor in the same file. New records shall be routed to per-actor or per-unit files (or otherwise distinct anchors) so that concurrent appends by different contributors do not produce a textual conflict at identical lines.";
        :>> rationale = "Verified: append_critique, append_disposition, append_measurement, append_marker_edge and create_item all insert immediately before the file's LAST closing brace (write.rs:458-461, 523, 613, 686, 912), and .tracking/authored.sysml is a single designated landing file for ALL serve-authored items and edges (write.rs:586, 909) — a purpose-built hot spot. Backlog and issues records are likewise hand-appended at the file tail (backlog.sysml, 1189 lines; issues.sysml, 68 issues; critiques.sysml, 1857 lines). Any two contributors appending anything in the same period conflict at the IDENTICAL anchor. This is safe to shard precisely because identity is a UUID and file position carries no meaning — the architecture already permits it, nothing exploits it. GitLab reduced changelog conflicts by roughly 90% with exactly this move to one-file-per-record.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }

    requirement srDcDeclaredMarkerVocabulary : SystemRequirement {
        :>> id = "0d5d0008-0812-4d08-9b3e-00000000d008";
        :>> title = "Every marker a guard depends on is declared, and an unknown marker is an error";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every metadata marker that any guard, rule, or computed view depends on shall be DECLARED in the schema, and validation shall reject an undeclared or misspelled marker rather than accepting it. The declared marker set shall be the single vocabulary that both authors and guards read.";
        :>> rationale = "Verified 2026-08-12: of the markers guards actually consume, only #View is declared. #Capability (15 files), #DerivedFrom (22 files), #Covers (6), #Dispositions (10) and #ProcessDefect (5) appear NOWHERE as a `metadata def`, yet #DerivedFrom is load-bearing for the hard-blocking requirement-rootedness guard (guards.rs:386-398, view.rs:3085-3116), #Covers for sitting-coverage, and #Dispositions for the disposition verdict path. Because markers are not type-checked, a misspelled marker validates CLEAN and silently removes that item from the guard's view — a blind guard reports PASS. This is the silent-gate-bypass class, and it scales with contributor count: any contributor's typo disables a gate for that item with no signal to anyone.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    // ── provenance + authority ────────────────────────────────────────
    requirement srDcActorIdentityBinding : SystemRequirement {
        :>> id = "0d5d0009-0812-4d09-9b3e-00000000d009";
        :>> title = "Acting identity is bound per machine and never defaulted";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "The acting actor shall be resolved from an explicit per-machine binding (configuration or environment) or from an explicit argument. No write path shall substitute a default actor when the acting identity is unstated: it shall refuse. In particular no write path shall ever default to a named human, and no default shall name an actor absent from the registry.";
        :>> rationale = "Verified: seven sites in serve.rs default `author`/`judged_by` to the human \"wweatherholtz\" (serve.rs:304, 331, 357, 384, 409, 483, 1313); `record decision` defaults --author to the same (main.rs:967); five paths default to \"keel-cli\", which is NOT a registered actor and would fail the actors guard (main.rs:910, 935, 1121, 1151); reverify defaults --by to claudeOpus (main.rs:429). The serious consequence is not untidy metadata: confirmation-authenticity exists specifically to prevent an AI fabricating HUMAN acceptance (D0106/issue059), and an AI-driven call that merely OMITS judged_by records a human attestation silently. The guard is therefore bypassable through a default parameter — live today, and multiplied by every added contributor. Capture already exists (capture_user.py prints an actor id) but nothing CONSUMES it; the gap is binding, not capture.";
        :>> kind = RequirementKind::security; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srDcActorKindClassification : SystemRequirement {
        :>> id = "0d5d000a-0812-4d0a-9b3e-00000000d00a";
        :>> title = "Actor kind is established explicitly at enrollment, never inferred";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Registering an actor shall require its kind (human or AI) to be stated explicitly, and shall not infer it from git identity or default to either. Registering an AI actor as a Person shall be impossible through the sanctioned path.";
        :>> rationale = "Verified: capture_user.py derives identity from the git committer and appends the actor to the registry hardcoded as `: Person` (append_id), with no kind question and no AI branch. Since git identity on an agent's machine is whatever that machine is configured with, an onboarding AI is registered as a HUMAN. confirmation-authenticity then reads the registry, sees a Person, and accepts that actor's acceptance attestations as human-judged — a second, independent bypass of the same guard, complementary to the defaulting one. Enrollment is the point where actor kind becomes true or false for every later authority check, so it is the one place inference is unacceptable.";
        :>> kind = RequirementKind::security; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srDcActorRoleModeled : SystemRequirement {
        :>> id = "0d5d000b-0812-4d0b-9b3e-00000000d00b";
        :>> title = "An actor's role is model data, linked to the actor";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "An actor's role shall be recorded as model data associated with that actor and queryable from the model, not held as prose, convention, or tribal knowledge. The role vocabulary shall be project-declarable so a downstream team can state its own roles.";
        :>> rationale = "The human asked for role capture at onboarding (2026-08-12). The schema already declares `part def Role { attribute name : String; }` (element.sysml:83) and it has ZERO instances anywhere in .engine or .tracking — declared vocabulary that nothing uses, the same dead-schema shape that issue060 removed for OrderingRule. Either roles become real model data or the type should go; leaving it declared-but-unused while roles are tracked in conversation is the prose-shadow-truth failure the engine exists to prevent. NOTE: linking an actor to a role needs either a typed edge or an attribute on Actor, and `schema/core` is FROZEN — so the mechanism requires a human-signed schema change (CLAUDE.md 2.5) and is deliberately NOT bundled with the day-one work.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::inspect;
    }

    requirement srDcAuthorityFromRegistry : SystemRequirement {
        :>> id = "0d5d000c-0812-4d0c-9b3e-00000000d00c";
        :>> title = "Attestation authority is checked against the registry, not assumed";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "For every attestation class that requires particular authority — accepting a Decision, dispositioning a finding at or above the human-disposition threshold, adjudicating a contributor conflict — the acting actor's recorded kind and role shall be checked against the declared policy for that class, and a mismatch shall be refused rather than warned.";
        :>> rationale = "Human-only authority is currently protected only by the registry happening to classify actors correctly and by every write path happening to pass the right actor. Both assumptions were just shown false (srDcActorIdentityBinding, srDcActorKindClassification). Deriving authority explicitly from recorded kind and role turns a convention that holds while everyone remembers it into a property checkable from the model, and it lets a distributed team hold differentiated authority without inventing a second, unmodeled permission system — which D0096 already rejected once for the right reasons.";
        :>> kind = RequirementKind::security; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srDcEnrollment : SystemRequirement {
        :>> id = "0d5d000d-0812-4d0d-9b3e-00000000d00d";
        :>> title = "One sanctioned enrollment step takes an uninitiated contributor to ready";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "A single sanctioned command shall enroll a contributor on a fresh machine: determine the acting identity, obtain its kind and role explicitly, register the actor if absent (without duplicating an existing entry), write the machine-local identity binding, verify that the local gate actually executes, and report readiness or the specific remedy. It shall require no other contributor to be present, and shall refuse rather than enrolling into a state where identity is indeterminate or the gate cannot run.";
        :>> rationale = "Every other guarantee presupposes an enrolled actor on a working machine: attribution (srDcActorIdentityBinding), acceptance authority (srDcActorKindClassification), and gate uniformity (srDcPortableGate). Every remote contributor begins in exactly the unenrolled state, so enrollment is the front door for the whole model, and it must be self-service because supervisors are asynchronous by definition. This is distinct from the existing `introduction` process, which onboards a fresh PROJECT from zero to first value (.engine/processes/introduction.sysml) — it does not establish WHO is acting; the two compose, enrollment first.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    // ── dividing the work ─────────────────────────────────────────────
    requirement srDcWorkClaim : SystemRequirement {
        :>> id = "0d5d000e-0812-4d0e-9b3e-00000000d00e";
        :>> title = "Work is claimed as an authored fact, and first-to-land wins";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "A contributor shall be able to claim a ready item by recording a claim fact (claiming actor, item, time, and the commit it was claimed against) and landing it. Mutual exclusion shall be provided by the remote accepting exactly one of two concurrent claims; the rejected contributor shall re-synchronize, observe the winning claim, and select other work. The ready frontier shall be filterable to exclude items claimed by others.";
        :>> rationale = "Verified: the ready frontier is one global list with no owner and no claim concept (orient.rs:33-45), so several contributors rationally select the same top item and the duplication surfaces only at integration. A claim is a legitimate AUTHORED fact — an irreducible observation of who took what, when, against which commit — while its liveness is COMPUTED, so this fits CLAUDE.md 2.6 rather than excepting it. No lock server is needed or wanted: git's atomic ref update already provides compare-and-swap, which is the one coordination primitive available when the remote is the only shared medium.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    requirement srDcStaleClaimExpiry : SystemRequirement {
        :>> id = "0d5d000f-0812-4d0f-9b3e-00000000d00f";
        :>> title = "An abandoned claim expires by computation, releasing the work but never the facts";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Claim liveness shall be COMPUTED from the claim fact and git-derived time, so that a claim held beyond its expiry without progress is reported as stale and its item returns to the ready frontier with no human intervention. Expiry shall release the CLAIM only — facts already recorded under it remain recorded.";
        :>> rationale = "A contributor that stops without releasing — session ended, machine offline, agent crashed — must not hold work indefinitely; this is the classic stale-lease problem and it is guaranteed to occur with autonomous agents across timezones. Computing staleness rather than storing a status keeps this inside the compute-don't-store invariant, and using git-derived time rather than wall-clock keeps it machine- and timezone-independent, consistent with D0013. Releasing the claim while preserving the facts is what makes abandonment safe: partial work stays evidence.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::test;
    }

    // ── contention, oversight, and the two lanes ──────────────────────
    requirement srDcContentionAdjudication : SystemRequirement {
        :>> id = "0d5d0010-0812-4d10-9b3e-00000000d010";
        :>> title = "Conflicting conclusions become a tracked issue for human adjudication";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "When contributors contend for the same work or record conclusions that cannot both hold, that condition shall be recordable in one sanctioned step as a tracked Issue naming both positions, shall appear among the items awaiting human authority, and shall be resolved by a recorded human adjudication. No contributor shall resolve such a conflict in favour of its own conclusion.";
        :>> rationale = "D0108 clause 5 already MANDATES exactly this ('conflicting conclusions across threads -> record an Issue; the HUMAN adjudicates, neither thread silently wins') and it is convention-only: verified absent from the seventeen guards (guards.rs:885-886) and from the eight declared rules (.engine/rules/rules.sysml:14-132), with enforcement resting on CLAUDE.md prose plus a reminder hook. There is also no command that records an Issue at all — `record decision` exists, `record issue` does not — so the sanctioned path D0108 requires has no implementation, which is the friction that guarantees non-compliance (D0054). Contention between autonomous contributors is usually information about upstream ambiguity; resolving it by precedence discards that information and leaves no trace a disagreement existed.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    requirement srDcHumanAuthorityQueue : SystemRequirement {
        :>> id = "0d5d0011-0812-4d11-9b3e-00000000d011";
        :>> title = "Everything awaiting human authority is visible with its age and its origin";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "A computed view shall present every item genuinely awaiting human authority — Decisions to accept, findings at or above the disposition threshold, conflicts to adjudicate, per-sitting reviews — with how long each has waited and which contributor produced it, so a supervisor can discharge them in one batched sitting. Items past an age threshold shall be visibly escalated. Contributors shall not block while items await authority.";
        :>> rationale = "Human judgment is the only resource in a mostly-AI team that cannot be scaled by adding contributors, so it must never sit on the critical path of each unit of work; the existing batched per-sitting review (D0049) is the right shape and needs the distributed dimensions of age and origin. The failure mode in both directions is documented: an unbounded queue makes the supervisor's availability the team's throughput ceiling, while asking for judgment an automated check could have made degrades review into rubber-stamping — which is worse than not asking, because it launders unreviewed work as reviewed. Confirm only what tests cannot (D0051) is the discriminator for what belongs in this queue.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    requirement srDcTwoLaneIntegration : SystemRequirement {
        :>> id = "0d5d0012-0812-4d12-9b3e-00000000d012";
        :>> title = "Integration lane is routed by change class — facts to trunk, definitional change through review";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "Contributions shall be routed to one of two integration lanes by the class of what they change. RECORD and EXECUTE contributions (verification results, claims, ceremony gates, new items and typed edges) shall land on the trunk through the gate-on-merged-tree loop, gated by the guards and the per-sitting review. CHANGE contributions (schema, process, workflow, gate, rule, or the meaning of a computed view) and deliverable source shall be proposed as a separately reviewable unit, gated by CI, reviewed by an actor OTHER than the author, and carry the recorded human acceptance the change path already requires. The routing shall follow the existing request taxonomy, and lane assignment shall not be at the contributor's discretion.";
        :>> rationale = "Human choice, 2026-08-12, over PR-for-everything and direct-to-trunk-for-everything. It cuts along the taxonomy the engine ALREADY routes by (CLAUDE.md 3), which is evidence the boundary is intrinsic rather than invented. Requiring review of every atomic fact would put hours of latency in front of contributors recording dozens of facts an hour for no assurance the guards and sitting review do not already provide, and under that load review degrades to rubber-stamping. Conversely, letting definitional change land unreviewed abandons independent review exactly where AI failure concentrates and where one error propagates to every contributor at once; LLM judges measurably favour their own output, so an author's self-review is not review, which is why the reviewer must differ from the author (consistent with the existing critic-independence principle, D0080). NOTE: this is a ROUTING policy over change classes; it does not revive the per-skill API-enforced writePolicy that D0096 superseded.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::inspect;
    }

    // ── uniform gate ──────────────────────────────────────────────────
    requirement srDcPortableGate : SystemRequirement {
        :>> id = "0d5d0013-0812-4d13-9b3e-00000000d013";
        :>> title = "The local gate runs on every contributor's machine, or says it cannot";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "The local gate shall contain no machine-specific absolute paths, shall locate the engine binary and any optional toolchain by probing, shall cover merge commits as well as ordinary commits, and shall FAIL LOUDLY when a check cannot run rather than passing silently. Text normalization for model files shall be declared in the repository so it does not depend on per-machine configuration.";
        :>> rationale = "Verified: the pre-commit hook hardcodes an absolute conda path and a Windows binary name (.githooks/pre-commit:17-21). On a non-Windows machine the binary is absent, so it prints 'guards skipped' and the local gate silently becomes nothing (pre-commit:66-68); on a machine lacking that conda path a commit touching .engine/*.sysml ABORTS, which teaches contributors to set SKIP_VALIDATE=1 and defeats the gate everywhere at once. Only pre-commit and post-commit are installed, so `git merge` — which fires pre-merge-commit — is UNGATED, meaning the conflict-resolution commits that this workflow produces, the very commits an AI is most likely to get semantically wrong, are precisely the ones with no local check. Separately .gitattributes normalizes only *.sh, leaving *.sysml dependent on each machine's core.autocrlf; a contributor configured differently turns every shared model file into a whole-file phantom diff and an unresolvable conflict on exactly the files everyone touches.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }

    requirement srDcEngineVersionParity : SystemRequirement {
        :>> id = "0d5d0014-0812-4d14-9b3e-00000000d014";
        :>> title = "Contributors are held to the engine version the project declares";
        :>> createdAt = "2026-08-12"; :>> createdBy = "claudeOpus5";
        :>> statement = "The project shall declare the engine version its checks are defined against, and a contributor's toolchain shall be compared against that declaration, reporting a mismatch before any authoring. A contributor shall be able to check parity — engine version and enforced check set — in one sanctioned step.";
        :>> rationale = "Guard semantics change with the engine, so contributors on different versions can legitimately disagree about whether the SAME model is valid — one lands what another's gate would have blocked, and the disagreement presents as a mysterious CI failure rather than as a version skew. This is the same class as the machine-dependent-orient defect (srDcUnresolvedEvidenceClass) with the engine binary as the varying input instead of the git object store. The engine already computes a governing-version view, so the notion of a governing version exists; what is missing is comparing the running toolchain against it. Note the dependency on release discipline: without published, pinnable versions (keelReleaseProcess, D0128) there is nothing for contributors to pin TO.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::test;
    }

    // ── satisfy edges: SR satisfies the upstream Need ─────────────────
    satisfy nIntegrationEvidencePreserving by srDcMergeOnlyIntegration;
    satisfy nComputedStateMachineIndependent by srDcUnresolvedEvidenceClass;
    satisfy nComputedStateMachineIndependent by srDcSyncAwareOrient;
    satisfy nConcurrentAuthoringSafe by srDcGateOnMergedTree;
    satisfy nConcurrentAuthoringSafe by srDcUniqueIdentityGuard;
    satisfy nConcurrentAuthoringSafe by srDcCollisionResistantIds;
    satisfy nConcurrentAuthoringSafe by srDcPerActorWriteTargets;
    satisfy nUniformGate by srDcDeclaredMarkerVocabulary;
    satisfy nProvenanceTrue by srDcActorIdentityBinding;
    satisfy nProvenanceTrue by srDcActorKindClassification;
    satisfy nAuthorityFollowsRecordedKind by srDcActorKindClassification;
    satisfy nAuthorityFollowsRecordedKind by srDcActorRoleModeled;
    satisfy nAuthorityFollowsRecordedKind by srDcAuthorityFromRegistry;
    satisfy nActorEnrollment by srDcEnrollment;
    satisfy nNoDuplicateWork by srDcWorkClaim;
    satisfy nNoDuplicateWork by srDcStaleClaimExpiry;
    satisfy nContentionVisible by srDcContentionAdjudication;
    satisfy nAsyncHumanOversight by srDcHumanAuthorityQueue;
    satisfy nAsyncHumanOversight by srDcTwoLaneIntegration;
    satisfy nUniformGate by srDcPortableGate;
    satisfy nUniformGate by srDcEngineVersionParity;

    // ── verify edges: the DELIVERED requirements' verification, traced UPWARD (issue082/D0130) ──
    // Sprint 247 recorded passing DoD TestResults for these six and CI green, but linked them to the
    // backlog ACTION only — so `tier-satisfaction` reported the REQUIREMENTS unverified and the AI
    // then narrated the number as functional verification. The new `verification-trace` guard flagged
    // exactly these pairs; the edges below close them. Scoped DELIBERATELY to the six this thread
    // delivered and can vouch for: the guard also surfaced ~60 pre-existing pairs across the viewer
    // programme (issue087), and asserting another thread's verification would be a judgment this
    // thread has no evidence for (D0108 — add, never speak for someone else's work).
    #Verify dependency from dcMergeOnlyIntegrationDoD to srDcMergeOnlyIntegration;
    #Verify dependency from dcDuplicateIdentityGuardDoD to srDcUniqueIdentityGuard;
    #Verify dependency from dcCollisionResistantIdsDoD to srDcCollisionResistantIds;
    #Verify dependency from dcActorIdentityBindingDoD to srDcActorIdentityBinding;
    #Verify dependency from dcActorIdentityBindingDoD to srDcActorKindClassification;
    #Verify dependency from dcPortableGateDoD to srDcPortableGate;
}
