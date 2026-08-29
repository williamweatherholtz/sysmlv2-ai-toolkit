// =====================================================================
// EngineDerivedRequirements — Phase 2 of the Architecture workflow for the thirteen Needs accepted
// at the D0157 read-back gate (engineNeedsAccept, judged by wweatherholtz against 32149b5).
//
// WHY THIS FILE COULD NOT EXIST UNTIL NOW. The business-architecture workflow states the ordering as
// non-negotiable: a SystemRequirement may not be RECORDED until the Needs it serves exist AND are
// accepted. An earlier one-word "proceed." was refused as an attestation (D0248); the reply that came
// back when the page was actually read reclassified one item out of the Need tier entirely (D0249).
// Had requirements been derived on the first reply, they would have been derived under a Need that no
// longer exists.
//
// THE BAR FOR EVERY STATEMENT HERE: a threshold something can FAIL. A requirement whose truth cannot
// be checked is a restated Need wearing a requirement's clothes, and this project already carries one
// Need (nAuthoringIsLowFriction) whose widest clause was cut for exactly that reason. Where a number
// is not yet defensible it is stated as an OBSERVABLE rather than invented - the D0088 rule that a
// boundary nobody can justify stays an indicator.
//
// VERIFY EDGES ARE NOT AUTHORED HERE, deliberately. `backlog-refinement` turns a work item's
// acceptance criteria into atomic Gherkin Test items linked by `verify`; authoring the Tests before
// the sprint that owns them would put the verification layer ahead of the work it verifies and
// produce Tests nothing runs. Each requirement below names what its Gherkin must demonstrate, which
// is the input refinement consumes.
//
// Chartered by D0246 (keel's subject is AI work management) and D0249 (the baseline, as narrowed).
// =====================================================================
package EngineDerivedRequirements {
    private import EngineElement::*;
    private import EngineRequirements::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    // ══ Program A — portability: the working material travels ═══════════════════════════════════
    // The three architecture forks these depend on are NOT settled; these state what must be true
    // whichever way each fork goes, which is why they can be recorded before the forks are decided.

    requirement srLibraryAvailableWithoutInstall : SystemRequirement {
        :>> id = "86cb3baf-2a9f-412d-a7fe-91d598167bdf";
        :>> title = "Every library unit is listable and launchable from any project without an install step";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "From a project that has installed no units, the catalogue listing shall return every unit in the author's library, and any of them shall be launchable, with zero per-project install or approval actions performed first.";
        :>> rationale = "The measurable core of nWorkingMaterialTravels: the human's complaint was the extra step, so the threshold is the step COUNT, which is zero. Listing and launching are the two operations that must work without it; governing is explicitly not one of them.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srAvailabilityIsNotActivation : SystemRequirement {
        :>> id = "ceb227b7-2af2-4905-8047-5adcb36b1941";
        :>> title = "A library unit present but not activated contributes no guard, rule or gate outcome";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "For a project with library units available and none activated, the gate verdict and the set of guards that ran shall be identical to the same project with the library absent.";
        :>> rationale = "The property that makes silent sync safe, stated as a differential nobody can fudge: same tree, two library states, identical gate. Without it, ambient availability changes a project's gate without its tree changing, which is what makes a re-derived historical verdict disagree with the original.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srActivatedUnitVersionIsPinned : SystemRequirement {
        :>> id = "ecdc9fc7-4d16-4357-b6a2-969f6c8ecb04";
        :>> title = "An activated unit's version is recorded in the project tree and does not move on library update";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "After a unit is activated in a project, a library update to that unit shall leave the project's recorded unit version and every gate outcome unchanged until an explicit update action is taken in that project.";
        :>> rationale = "The other half of silent sync. The human asked for updates without per-update permission; this bounds that to content that governs NOTHING, so the permission they are freed from is never permission to change a gate.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srUnitDeclaresEngineRange : SystemRequirement {
        :>> id = "d5dde62a-e9a4-40a3-a888-a0fb5e73c22d";
        :>> title = "A unit declares its engine range, and an out-of-range install is refused at install time";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every portable unit shall carry a declared engine-version range; installing a unit whose range excludes the target project's engine shall fail at install with a message naming both versions, and shall leave no file written.";
        :>> rationale = "nPortableContentSurvivesEngineChange's checkable core. 'At install, naming both versions, writing nothing' is the difference between a refusal and a half-install - and a half-install is the failure mode that gets discovered later at a gate, which is the outcome the Need exists to prevent.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srUnitMigrationPath : SystemRequirement {
        :>> id = "342aaf40-238f-47a1-ad8c-49566a613632";
        :>> title = "A unit behind the engine has a migration path that does not require hand-rewriting it";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "For a unit whose declared range excludes the current engine, a migration command shall bring it into range or shall report, per file, exactly what it cannot migrate and why - never partially migrating and reporting success.";
        :>> rationale = "The existing migration tool brings a PROJECT to a vintage; nothing migrates a unit. The per-file refusal is the honest-state clause: a migration that silently skips what it cannot handle produces a unit that installs and then misbehaves.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srProjectPinsItsEngine : SystemRequirement {
        :>> id = "44d79285-3fb3-4bc1-bb83-8e4c67406244";
        :>> title = "The engine version a project declares is the one that runs, and a mismatch refuses";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "A gate invoked in a project shall run the engine version that project declares; where the resolved binary's version differs from the declaration, the invocation shall refuse and name both versions rather than proceeding with an advisory warning.";
        :>> rationale = "nEnginePinAndEvolution's threshold, and it is a STRENGTHENING of today's behaviour, stated so the change is visible: the current declaration is advisory and pins nothing, so a project's checks are whatever binary is on PATH. The escalation from warn to refuse is the substance of this requirement.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srWorkspacePinIsCoherent : SystemRequirement {
        :>> id = "74626a01-e2ee-4b76-91aa-a6c14c6b3960";
        :>> title = "Projects sharing a repository declare a coherent engine version, or the gate refuses";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Where one repository holds several projects, a gate spanning them shall refuse when their declared engine versions disagree, naming each project and its declaration; a single-project repository shall be unaffected.";
        :>> rationale = "One repository is one checkout and therefore one binary, so disagreeing declarations make a workspace gate incoherent - it would report a verdict computed by an engine that some project did not declare. Serves the engine pin; exercised by the use case the human reclassified out of the Need tier (D0249).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    // ══ Program B — assurance: what a pass means ════════════════════════════════════════════════

    requirement srResultRecordsItsGate : SystemRequirement {
        :>> id = "ac9ff3a8-1742-4170-8561-1103098bc8c5";
        :>> title = "A recorded result names the engine version and guard set that produced it";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every recorded verification result shall carry the engine version and guard-set identity that judged it, such that two results produced by different guard sets are distinguishable without reading anything outside the result.";
        :>> rationale = "The open half of nPassDisclosesItsSource. Results today carry the tree and the judge but not the GATE, so a pass from a 40-guard binary is indistinguishable from one from a 52-guard binary, and re-derivation over history uses rules that did not exist when the commit was made. Requires a frozen schema/core change, authorised verbatim by the human: 'implement all, including schema change'.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srArmingIsATypedOutcome : SystemRequirement {
        :>> id = "cbac015b-0c09-452b-8323-3ebc3329f836";
        :>> title = "A control probe returns a typed outcome distinguishing cannot-fire from nothing-to-catch";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every control probe shall return a typed result whose error case means the control could not run and whose success case means it ran; any coverage surface shall report those two cases separately and shall never render an unrunnable control as a clean finding.";
        :>> rationale = "nDeclaredControlDistinguishableFromArmed, carried by Rust's Result on the human's steer (us036): making cannot-fire an error variant puts the distinction where the compiler tracks it rather than in a declaration file that describes the code from outside. The named incident is a truthful all-clear reported while the commit gate could not run at all.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srSkippedConstructsAreCounted : SystemRequirement {
        :>> id = "ab522144-3a67-4578-93f3-d4bdc23d35e1";
        :>> title = "The parser reports what it skipped, and a rise above the recorded baseline fails";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "The parser shall report the count and kind of statement classes it did not read; a count exceeding the recorded baseline shall fail the gate, and a count at or below it shall remain a standing warning rather than being cleared.";
        :>> rationale = "nParserIsTheTrustedBase. The ratchet shape is deliberate and follows an existing lesson: an all-or-nothing gate on this number would be bypassed wholesale, so NEW invisibility fails while legacy invisibility stays visible.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srControlsProvenOnAForeignTree : SystemRequirement {
        :>> id = "c60864dc-5a89-4927-9859-55b51a688626";
        :>> title = "Every control is exercised against a project the engine did not author, in both directions";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "For each declared control, the gate shall demonstrate on a freshly scaffolded foreign project that the control passes when its unit is absent and fires when its unit is present and violated; a control demonstrated on neither shall be reported as unexercised.";
        :>> rationale = "nForeignTreeVerification. Both directions are required because they fail differently: firing on a project that never adopted the control is the lockout defect, and not firing where it should is the silent-control defect. The measurement that motivated the Need is that no guard read another project's tree while field defects clustered there.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    // ══ Program C — direction: what the model must keep telling the truth about ═════════════════

    requirement srNoDerivedItemWithoutASource : SystemRequirement {
        :>> id = "6818d3be-5ccc-40de-8ad0-e6da70777a34";
        :>> title = "A derived business item without a cited utterance is reported as unsourced, never as sourced";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every Need, Story or Brief shall either carry an edge to a recorded verbatim Statement or be reported as unsourced by a computed view; the write path shall refuse to record an interpretation citing a Statement that does not exist.";
        :>> rationale = "nIntakeBeforeInterpretation. The checkable core is DISTINGUISHABILITY rather than universal sourcing: an item may legitimately have no utterance behind it, and the failure is when that is indistinguishable from one that does. The unsourced count is a floor on the record, never a claim about the human.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srPromisedArtifactsAreChecked : SystemRequirement {
        :>> id = "f08fa44c-55a1-44c3-87e5-5780d6b9f495";
        :>> title = "An accepted decision's promised artifact is checked against the tree, not against the record";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "For every accepted decision declaring a promised artifact, a computed view shall report whether that artifact exists in the tree at the current commit, and an accepted decision whose artifact is absent shall appear as an open promise rather than as complete.";
        :>> rationale = "nDecisionPromisesTracked. 'Against the tree, not the record' is the whole requirement: the failure it prevents is a decision accepted while its artifact was never built, with the model reading as complete because acceptance was recorded.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srIndicatorsAreNeverGated : SystemRequirement {
        :>> id = "fa48b918-1acc-46f4-8327-b9f032c5010b";
        :>> title = "A declared indicator has no enforced threshold, and promoting one is a recorded act";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "No gate outcome shall depend on a declared indicator's value; promoting an indicator to an enforced contract shall require a recorded decision stating the boundary and its justification, and the gate shall refuse to enforce a threshold no such decision declares.";
        :>> rationale = "nMonitoredNotGated, priority should because it is a discipline rather than a blocker. Stated as a NEGATIVE property because that is what is checkable: the presence of a gate outcome depending on an indicator is detectable, whereas 'the boundary is defensible' is a judgment.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srInvalidatedDocClaimsFailTheSameCommit : SystemRequirement {
        :>> id = "cb5942fa-6b47-417b-a6ed-08de383272b6";
        :>> title = "A change invalidating a documented claim fails its own commit";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Where a commit changes a fact that documentation restates, the gate for that same commit shall fail while the documentation still asserts the superseded value, naming the file and the claim.";
        :>> rationale = "nDocSurfaceCannotOutliveTheChange, priority should. 'Its own commit' is the threshold - a later corrective commit still leaves a window in which the docs are read as current and cannot be distinguished from current. The mandate to DELETE duplicated prose was cut from the Need (D0248) and is deliberately absent here.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::test;
    }

    // ══ Program D — the agent loop ══════════════════════════════════════════════════════════════

    requirement srRecallIsSeededAndVisible : SystemRequirement {
        :>> id = "bb21e9b0-ab9e-4970-8616-3b68f6718ae0";
        :>> title = "Turn-start recall is seeded from the prompt, visible in the turn, and honest when it fails";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Recall shall seed from the turn's own prompt and shall emit, in the turn, what it retrieved and what it reached; a retrieval that cannot complete shall emit a statement that it pushed nothing rather than emitting a partial set indistinguishable from a complete one.";
        :>> rationale = "nTurnStartRecall as narrowed. The budget and latency cap were cut from the Need (D0248) because they are today's implementation numbers; what survives is seeding, visibility, and the honest-failure property - all three checkable without fixing any threshold.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srEngineSuppliesIdentityAndProvenance : SystemRequirement {
        :>> id = "65ca4077-0736-4588-b1a5-e22b1a285d17";
        :>> title = "No authored record requires a hand-typed identity, date or actor";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "For every record type the write API covers, identity shall be engine-minted, the date shall come from the clock, and the actor shall come from the bound identity or be refused - and no write path shall accept a hand-supplied identity in place of a minted one.";
        :>> rationale = "The checkable half of nAuthoringIsLowFriction after the universal comparative was cut (D0248). Refusal rather than defaulting is the load-bearing part: a defaulted actor is the mechanism by which an AI-driven call once recorded a human's name.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    requirement srRecurringFrictionBecomesTooling : SystemRequirement {
        :>> id = "445fcf97-36ca-4611-ad82-1b36107c3f41";
        :>> title = "A friction met more than once is answered by tooling, and the count is observable";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> statement = "Recurrence of a recorded friction shall be countable from the tracked record, and a friction recorded more than once without a tooling or control response shall be reported as an open friction rather than closed by a note or a reminder.";
        :>> rationale = "The second half of nAuthoringIsLowFriction. NO THRESHOLD IS SET on how many open frictions are acceptable, deliberately: no boundary is defensible yet, so this states the OBSERVABLE and the count is an indicator under srIndicatorsAreNeverGated rather than an invented limit. Manual vigilance is explicitly not a permitted response.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    // ══ satisfy: every requirement roots in an accepted Need ════════════════════════════════════

    satisfy nWorkingMaterialTravels by srLibraryAvailableWithoutInstall;
    satisfy nWorkingMaterialTravels by srAvailabilityIsNotActivation;
    satisfy nWorkingMaterialTravels by srActivatedUnitVersionIsPinned;
    satisfy nPortableContentSurvivesEngineChange by srUnitDeclaresEngineRange;
    satisfy nPortableContentSurvivesEngineChange by srUnitMigrationPath;
    satisfy nEnginePinAndEvolution by srProjectPinsItsEngine;
    satisfy nEnginePinAndEvolution by srWorkspacePinIsCoherent;

    satisfy nPassDisclosesItsSource by srResultRecordsItsGate;
    satisfy nDeclaredControlDistinguishableFromArmed by srArmingIsATypedOutcome;
    satisfy nParserIsTheTrustedBase by srSkippedConstructsAreCounted;
    satisfy nForeignTreeVerification by srControlsProvenOnAForeignTree;

    satisfy nIntakeBeforeInterpretation by srNoDerivedItemWithoutASource;
    satisfy nDecisionPromisesTracked by srPromisedArtifactsAreChecked;
    satisfy nMonitoredNotGated by srIndicatorsAreNeverGated;
    satisfy nDocSurfaceCannotOutliveTheChange by srInvalidatedDocClaimsFailTheSameCommit;

    satisfy nTurnStartRecall by srRecallIsSeededAndVisible;
    satisfy nAuthoringIsLowFriction by srEngineSuppliesIdentityAndProvenance;
    satisfy nAuthoringIsLowFriction by srRecurringFrictionBecomesTooling;
}
