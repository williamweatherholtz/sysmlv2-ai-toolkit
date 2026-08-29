// CONTROL MAP (D0195, sprint 417) - every standing control as a SystemSafetyConstraint instance
// (the constraint the control enforces; the enforcing mechanism named in the statement), with a
// plain dependency edge to each hazard it discharges. The `keel controls` view computes the
// two-way diff over exactly this file plus engine-safety.sysml: hazards no constraint reaches,
// and constraints anchored to no hazard (process-quality controls - reported, not warned away).
// Guard constraints carry the guard's kebab name in the statement so the mapping is greppable.
package ControlMap {
    private import EngineElement::*;
    private import EngineSafety::*;

    part gActors : SystemSafetyConstraint { :>> id = "e23727f4-74d9-43dd-af19-b80269a97912"; :>> title = "guard: actors"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every attesting actor resolves to the registry with a kind. ENFORCED BY guard `actors`."; }
    part gAcceptanceEvents : SystemSafetyConstraint { :>> id = "777c58e1-5e7c-49a7-b834-3fb1f6bbf63a"; :>> title = "guard: acceptance-events"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "An accepted Decision carries a recorded human acceptance event. ENFORCED BY guard `acceptance-events`."; }
    part gSprintCoverage : SystemSafetyConstraint { :>> id = "d6fa998e-e463-4a86-b9d2-0fee4ed771cf"; :>> title = "guard: sprint-coverage"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Delivered stories carry their ceremony coverage. ENFORCED BY guard `sprint-coverage`."; }
    part gCeremony : SystemSafetyConstraint { :>> id = "3fffae3b-b1e0-448c-8e8d-544a1b90b050"; :>> title = "guard: ceremony"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Ceremony gates pass in declared order with scan evidence. ENFORCED BY guard `ceremony`."; }
    part gCharter : SystemSafetyConstraint { :>> id = "1883b1b1-912d-4bff-8999-066672ea8c1e"; :>> title = "guard: charter"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A delivery story carries its charter lineage. ENFORCED BY guard `charter`."; }
    part gProcessChange : SystemSafetyConstraint { :>> id = "4bdaad57-f912-489c-82df-da7491889c31"; :>> title = "guard: process-change"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A process-definition change is co-committed with its governing marked Decision. ENFORCED BY guard `process-change`."; }
    part gIssues : SystemSafetyConstraint { :>> id = "410b8b83-6122-4360-b442-e074701cf072"; :>> title = "guard: issues"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every Issue is triaged with a resolving edge on arrival. ENFORCED BY guard `issues`."; }
    part gViewpointRenderer : SystemSafetyConstraint { :>> id = "e520e97d-9b43-4286-abcf-1f69a0325979"; :>> title = "guard: viewpoint-renderer"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A declared viewpoint names a renderer. ENFORCED BY guard `viewpoint-renderer`."; }
    part gManifestCoverage : SystemSafetyConstraint { :>> id = "ffff4202-7de4-48ca-bac5-e3348b4e342b"; :>> title = "guard: manifest-coverage"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Activation manifests cover what they claim to govern. ENFORCED BY guard `manifest-coverage`."; }
    part gCriticIndependence : SystemSafetyConstraint { :>> id = "e45c7a14-7f87-49d5-896f-cadce5a0d4e8"; :>> title = "guard: critic-independence"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A critique's judge differs from the element's author. ENFORCED BY guard `critic-independence`."; }
    part gProcessSkill : SystemSafetyConstraint { :>> id = "469d8484-0259-4e30-aafd-c7027b4928e0"; :>> title = "guard: process-skill"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every process has a deploying skill. ENFORCED BY guard `process-skill`."; }
    part gRequirementRootedness : SystemSafetyConstraint { :>> id = "693b8c3f-11a3-429e-8627-a37b73da3a4e"; :>> title = "guard: requirement-rootedness"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A #Capability item derives from a Need. ENFORCED BY guard `requirement-rootedness`."; }
    part gDecisionRationale : SystemSafetyConstraint { :>> id = "17bd632f-ec6f-476b-ac8b-ab998196fe46"; :>> title = "guard: decision-rationale"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A Decision carries substantive rationale fields. ENFORCED BY guard `decision-rationale`."; }
    part gAttestationSubstance : SystemSafetyConstraint { :>> id = "3d94ffde-1cf2-408c-86c9-b015167280d4"; :>> title = "guard: attestation-substance"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A confirmation's procedure text carries the attested claim, not a stub. ENFORCED BY guard `attestation-substance`."; }
    part gMarkerVocabulary : SystemSafetyConstraint { :>> id = "e295ab05-33c1-47b8-be41-66813e56a6f7"; :>> title = "guard: marker-vocabulary"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every marker used is declared - a typo cannot silently drop an edge from every view. ENFORCED BY guard `marker-vocabulary`."; }
    part gDuplicateIdentity : SystemSafetyConstraint { :>> id = "5a055c6e-171d-48a8-876a-6b99ad662db2"; :>> title = "guard: duplicate-identity"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "No repeated id, item name per package, package name, or allocated sequence number, cross-file. ENFORCED BY guard `duplicate-identity`."; }
    part gDecisionRequirementLink : SystemSafetyConstraint { :>> id = "8c5af3ee-6672-4748-b403-0cfae3169eaf"; :>> title = "guard: decision-requirement-link"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A requirement-affecting Decision links the requirements it touches. ENFORCED BY guard `decision-requirement-link`."; }
    part gVerificationTrace : SystemSafetyConstraint { :>> id = "4dea54e1-3f8d-4c3c-a69a-dc647f21504b"; :>> title = "guard: verification-trace"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Done work whose charter chain reaches an SR leaves no SR unverified. ENFORCED BY guard `verification-trace`."; }
    part gPriorityInversion : SystemSafetyConstraint { :>> id = "2203777d-7292-47c7-b187-41896e14c125"; :>> title = "guard: priority-inversion"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Ready-frontier ordering deviations from priority are visible. ENFORCED BY guard `priority-inversion`."; }
    part gRetroBacklog : SystemSafetyConstraint { :>> id = "0226b134-e598-43f7-b30d-d57f31f53fa8"; :>> title = "guard: retro-backlog"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A retro finding becomes a tracked backlog item, never prose. ENFORCED BY guard `retro-backlog`."; }
    part gConfirmationAuthenticity : SystemSafetyConstraint { :>> id = "180fef26-c87c-468c-bdca-67486d0ef38c"; :>> title = "guard: confirmation-authenticity"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A method=confirmation result is judged by a human. ENFORCED BY guard `confirmation-authenticity`."; }
    part gEngineLint : SystemSafetyConstraint { :>> id = "645aac90-0fcc-4ffd-a725-15895445349f"; :>> title = "guard: engine-lint"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Engine instance files satisfy structural lint. ENFORCED BY guard `engine-lint`."; }
    part gDocSync : SystemSafetyConstraint { :>> id = "40e20ebb-5a6f-44b9-93f5-15117d7382dc"; :>> title = "guard: doc-sync"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A model change sweeps the doc surface for invalidated claims in the same commit. ENFORCED BY guard `doc-sync`."; }
    part gHookConfigIntegrity : SystemSafetyConstraint { :>> id = "71fbd36b-a63d-4e3e-a940-8ec0a93268b1"; :>> title = "guard: hook-config-integrity"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "The hook wiring matches the generator - a mutated hook command is visible. ENFORCED BY guard `hook-config-integrity`."; }
    part gActivationManifest : SystemSafetyConstraint { :>> id = "da3ab162-1e54-4f7b-8568-749a768cd3ec"; :>> title = "guard: activation-manifest"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Activation state is declared data whose changes are governed. ENFORCED BY guard `activation-manifest`."; }
    part gSequenceMultiplicity : SystemSafetyConstraint { :>> id = "c25d7323-ddf8-407a-aa03-6ba1bf5c6059"; :>> title = "guard: sequence-multiplicity"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "No multi-valued assignment reaches a single-valued provenance field. ENFORCED BY guard `sequence-multiplicity`."; }
    part gParserCoverage : SystemSafetyConstraint { :>> id = "c8fb0cff-9940-47c6-8fd0-338c33ccaffa"; :>> title = "guard: parser-coverage"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Statements the parser cannot read are enumerated, never silently skipped. ENFORCED BY guard `parser-coverage`."; }
    part gBaseFirstJustification : SystemSafetyConstraint { :>> id = "1f204629-f51a-4ada-9665-8318af041b37"; :>> title = "guard: base-first-justification"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A project-declared marker carries a recorded justification. ENFORCED BY guard `base-first-justification`."; }
    part gEdgeEndpoints : SystemSafetyConstraint { :>> id = "fb8ad8ef-3d48-4bbc-af21-71fd5996e6ae"; :>> title = "guard: edge-endpoints"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every edge endpoint resolves to a declared item. ENFORCED BY guard `edge-endpoints`."; }
    part gOwnership : SystemSafetyConstraint { :>> id = "56a09dcb-4040-457c-88c0-8b6216473ab8"; :>> title = "guard: ownership"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A non-owner adds or supersedes, never overwrites another actor's fields. ENFORCED BY guard `ownership`."; }
    part gAttestationAuthority : SystemSafetyConstraint { :>> id = "8413819a-a05a-419a-a054-10d5e7aa27cd"; :>> title = "guard: attestation-authority"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "The attesting actor's kind and role satisfy the declared policy for the class, kind before role. ENFORCED BY guard `attestation-authority`."; }
    part gTypeCollision : SystemSafetyConstraint { :>> id = "1fac8117-2e76-4659-8de6-77b560b8621a"; :>> title = "guard: type-collision"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "No two types collide on a name. ENFORCED BY guard `type-collision`."; }
    part gAttributeVocabulary : SystemSafetyConstraint { :>> id = "33f2288f-7c10-4abd-bcef-5bb35d486516"; :>> title = "guard: attribute-vocabulary"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every attribute used is declared - a typo cannot silently vanish a fact. ENFORCED BY guard `attribute-vocabulary`."; }
    part gResolverKind : SystemSafetyConstraint { :>> id = "42da38da-c515-4649-9733-aa7f45320377"; :>> title = "guard: resolver-kind"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A #Resolves source is a kind that can actually resolve. ENFORCED BY guard `resolver-kind`."; }
    part gStaleGateProse : SystemSafetyConstraint { :>> id = "3b51eafb-15dc-44ac-8859-883e3360d3a6"; :>> title = "guard: stale-gate-prose"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Gate prose does not contradict the recorded results beside it. ENFORCED BY guard `stale-gate-prose`."; }
    part gImpossibleEvidenceDate : SystemSafetyConstraint { :>> id = "ae93f00d-0727-48b5-b59d-25642bd23072"; :>> title = "guard: impossible-evidence-date"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "No judgment predates the commit it claims to judge. ENFORCED BY guard `impossible-evidence-date`."; }
    part gIdentityPresent : SystemSafetyConstraint { :>> id = "adc75f2f-dba3-4e4b-ae3a-fffc82c5010c"; :>> title = "guard: identity-present"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every tracked item carries an id. ENFORCED BY guard `identity-present`."; }
    part gIdentityWellFormed : SystemSafetyConstraint { :>> id = "be225c0b-a843-4045-abe7-381c2ecd1631"; :>> title = "guard: identity-well-formed"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every id is a well-formed v4 UUID. ENFORCED BY guard `identity-well-formed`."; }
    part gToolReference : SystemSafetyConstraint { :>> id = "bdca2a6d-7645-4b00-9fce-f0564825c43c"; :>> title = "guard: tool-reference"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "No doc names a deleted tool. ENFORCED BY guard `tool-reference`."; }
    part gScaffoldPlaceholder : SystemSafetyConstraint { :>> id = "4ab75541-5e0a-4ecf-871b-cb4b235d9b28"; :>> title = "guard: scaffold-placeholder"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "No unfilled scaffold text passes as a recorded judgment. ENFORCED BY guard `scaffold-placeholder`."; }
    part gClaudeSurfaceDrift : SystemSafetyConstraint { :>> id = "565274ff-68c3-4543-9b7d-c169459f90f3"; :>> title = "guard: claude-surface-drift"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "The keel-owned enforcement surface matches this binary's generator. ENFORCED BY guard `claude-surface-drift`."; }
    part gDecisionScaffolding : SystemSafetyConstraint { :>> id = "3c07e7e6-ff5c-4400-8113-7236b7dcafba"; :>> title = "guard: decision-scaffolding"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "An accepted process-change Decision charters work. ENFORCED BY guard `decision-scaffolding`."; }
    part gReleaseRecorded : SystemSafetyConstraint { :>> id = "a36b8bf3-a3e3-4f4e-a265-9b8ba01eda05"; :>> title = "guard: release-recorded"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every version tag has a Release item whose commit matches the tag. ENFORCED BY guard `release-recorded`."; }
    part gEnrollmentBinding : SystemSafetyConstraint { :>> id = "f83f151f-1368-4fce-8d08-6d6e258452e9"; :>> title = "guard: enrollment-binding"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A machine binding resolves to a registered Person or kind-carrying Actor. ENFORCED BY guard `enrollment-binding`."; }
    part gControlEventCoverage : SystemSafetyConstraint { :>> id = "4edc0b88-e6c0-4516-8608-9e1dca90f367"; :>> title = "guard: control-event-coverage"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every declared control event has an emitter and every emitter is declared. ENFORCED BY guard `control-event-coverage`."; }
    part ctlAuditHistory : SystemSafetyConstraint { :>> id = "bb545d93-36b0-4a58-8ec1-8586c5ceeac1"; :>> title = "control: ctlAuditHistory"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Gate verdicts re-derive from committed truth over every pushed range (keel audit-history, CI layer 3)"; }
    part ctlBranchProtection : SystemSafetyConstraint { :>> id = "b0736aea-ea26-46a4-99fd-dbaadb32887b"; :>> title = "control: ctlBranchProtection"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "The remote refuses force-push and deletion on main; squash and rebase merges are disabled (GitHub configuration, probed sprint 401)"; }
    part ctlPrePushBehind : SystemSafetyConstraint { :>> id = "1e67ab1f-c1d3-43f7-9304-c5736a6cc15d"; :>> title = "control: ctlPrePushBehind"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "A main push that is not a fast-forward of the fetched trunk is refused at the pusher's terminal (.githooks/pre-push)"; }
    part ctlWriteLock : SystemSafetyConstraint { :>> id = "d804edcd-63ab-4d61-9be7-7ba35ac2d0b0"; :>> title = "control: ctlWriteLock"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "All same-machine writers serialise on .keel-write-lock; contention refuses loudly (write.rs)"; }
    part ctlPreWriteTiers : SystemSafetyConstraint { :>> id = "b6498cdd-879b-4c90-a544-9a0d2cd825f4"; :>> title = "control: ctlPreWriteTiers"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Protected fact surfaces deny, ask, or record-override at the harness boundary (hook pre-write, D0176)"; }
    part ctlCommitGate : SystemSafetyConstraint { :>> id = "086aafdc-a753-45f7-a328-a9c71087a6c2"; :>> title = "control: ctlCommitGate"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "No commit lands with an invalid model or a failing guard (.githooks/pre-commit + CI)"; }
    part ctlReleaseGateJob : SystemSafetyConstraint { :>> id = "d5dd1e78-69a8-46d6-9be0-4a786727d140"; :>> title = "control: ctlReleaseGateJob"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "The publish job requires the full gate over the tagged commit (release.yml, proven both ways sprint 409)"; }
    // D0254: IMPLEMENTED, load-bearing, PROVEN BY TEST - and until now absent from this map, which
    // is the third failure class. A control the map does not know about cannot appear as a gap,
    // and its omission makes the coverage view read CLEANER rather than worse.
    part ctlLaunchDirtyRefusal : SystemSafetyConstraint { :>> id = "8d5d5bca-0acd-4ba8-9945-419e1ab3f46a"; :>> title = "control: ctlLaunchDirtyRefusal"; :>> createdAt = "2026-08-29"; :>> createdBy = "claudeOpus5"; :>> statement = "A process launch onto a DIRTY tree is refused, so a run's diff is attributable to that run alone (launcher::prepare, D0182 single-writer). PROVEN BY TEST: keel-cli/tests/launcher_controls_actually_fire.rs."; }
    part ctlFireLedger : SystemSafetyConstraint { :>> id = "b8c675a6-c8f7-4227-b5dd-e88708d02235"; :>> title = "control: ctlFireLedger"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Every enforcement fire leaves a counted machine-local line; the report computes over it (D0180)"; }
    part ctlReverify : SystemSafetyConstraint { :>> id = "e54d5849-5927-46f5-ba9a-b14aebf60c9b"; :>> title = "control: ctlReverify"; :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5"; :>> statement = "Drift-suspect reproducible evidence re-runs at HEAD and re-stamps honestly (keel reverify, D0101)"; }

    // ---- constraint -> hazard discharge edges (plain base-SysML dependencies) ----
    dependency from gActors to ehz3;
    dependency from gAcceptanceEvents to ehz3;
    dependency from gSprintCoverage to ehz4;
    dependency from gCeremony to ehz4;
    dependency from gCharter to ehz4;
    dependency from gProcessChange to ehz7;
    dependency from gIssues to ehz4;
    dependency from gManifestCoverage to ehz5;
    dependency from gCriticIndependence to ehz3;
    dependency from gProcessSkill to ehz5;
    dependency from gAttestationSubstance to ehz3;
    dependency from gMarkerVocabulary to ehz8;
    dependency from gDuplicateIdentity to ehz6;
    dependency from gVerificationTrace to ehz4;
    dependency from gRetroBacklog to ehz4;
    dependency from gConfirmationAuthenticity to ehz3;
    dependency from gDocSync to ehz8;
    dependency from gHookConfigIntegrity to ehz5;
    dependency from gHookConfigIntegrity to ehz7;
    dependency from gActivationManifest to ehz7;
    dependency from gSequenceMultiplicity to ehz3;
    dependency from gParserCoverage to ehz8;
    dependency from gEdgeEndpoints to ehz8;
    dependency from gOwnership to ehz3;
    dependency from gAttestationAuthority to ehz3;
    dependency from gTypeCollision to ehz6;
    dependency from gAttributeVocabulary to ehz8;
    dependency from gResolverKind to ehz4;
    dependency from gStaleGateProse to ehz4;
    dependency from gStaleGateProse to ehz8;
    dependency from gImpossibleEvidenceDate to ehz4;
    dependency from gIdentityPresent to ehz6;
    dependency from gIdentityWellFormed to ehz6;
    dependency from gToolReference to ehz8;
    dependency from gScaffoldPlaceholder to ehz4;
    dependency from gClaudeSurfaceDrift to ehz7;
    dependency from gClaudeSurfaceDrift to ehz5;
    dependency from gDecisionScaffolding to ehz4;
    dependency from gReleaseRecorded to ehz9;
    dependency from gReleaseRecorded to ehz8;
    dependency from gEnrollmentBinding to ehz3;
    dependency from gControlEventCoverage to ehz5;
    dependency from ctlAuditHistory to ehz1;
    dependency from ctlBranchProtection to ehz1;
    dependency from ctlPrePushBehind to ehz1;
    dependency from ctlLaunchDirtyRefusal to ehz2;
    dependency from ctlWriteLock to ehz2;
    dependency from ctlPreWriteTiers to ehz2;
    dependency from ctlPreWriteTiers to ehz7;
    dependency from ctlCommitGate to ehz2;
    dependency from ctlCommitGate to ehz4;
    dependency from ctlReleaseGateJob to ehz9;
    dependency from ctlFireLedger to ehz5;
    dependency from ctlReverify to ehz8;
}
