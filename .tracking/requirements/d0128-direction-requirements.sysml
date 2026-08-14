// =====================================================================
// ProjectD0128DirectionRequirements — the SystemRequirements decomposing the three Needs recorded by
// D0128 (accepted 2026-08-13): nAiGovernanceInLoop, nProcessPortability, nReleaseDiscipline.
//
// WHY THIS FILE EXISTS (human question, 2026-08-13: "is it in needs? are these items we've been
// discussing reflected in needs?"). It was not. The three Needs were recorded when D0128 was written
// and had ZERO satisfy edges — the "what/why" with no "how" — while six sprints of work (D0130-D0132:
// the response contract, four tracking-mechanism controls, the kernel-gate demotion) traced up to a
// DECISION and stopped there. `requirement-rootedness` permits that (unmarked decision-driven work is
// exempt, D0064), which is exactly why it stayed invisible; tier-satisfaction was reporting it all
// along as 83% need-decomposition with these three in the gap set.
//
// Requirements marked DELIVERED carry a #Verify edge to the DoD that actually verified them, so the
// trace runs Need -> SR -> verification rather than stopping at the Decision (issue082's lesson).
// =====================================================================
package ProjectD0128DirectionRequirements {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    // ── nAiGovernanceInLoop ───────────────────────────────────────────
    requirement srGovContractInSystemPrompt : SystemRequirement {
        :>> id = "0d6e0001-0813-4e01-9b3e-0000001e0001";
        :>> title = "The response contract lives at system-prompt level, with one canonical home";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "The AI's response contract — parse-first routing, no prose state, verify-don't-assert, never fabricate an attestation, correct-at-the-root — shall be carried in the SYSTEM PROMPT via a project-scoped output style, and shall have exactly one canonical home: other documents reference it and must not restate it.";
        :>> rationale = "Placement is the lever, not wording. CLAUDE.md is injected as ordinary user-turn context — the weakest layer that can hold a rule — and the contract was bypassed four times from there (issue082-085) while nothing detected it. Restating it more firmly in the same layer would have changed nothing. Committed at project scope so every contributor of the D0129 team gets the identical contract.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srGovTurnBoundaryGate : SystemRequirement {
        :>> id = "0d6e0002-0813-4e02-9b3e-0000001e0002";
        :>> title = "Honest-state gating runs at the TURN boundary, not only at commit";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "The engine shall gate honest state at the turn boundary: a turn shall not end while the model is invalid or a guard is violated, and the violations shall be fed back for correction. The gate shall be loop-safe (a second consecutive failure releases with a loud warning rather than trapping the agent), shall never block for a condition the agent cannot fix in-loop, and shall report VISIBLY when it cannot run rather than passing silently.";
        :>> rationale = "Guards ran only at COMMIT, so a turn could end dirty and the next actor inherited it. Moving the EXISTING gate earlier changes when it runs, not what it means — the cheapest possible intervention. Harness-executed, so unlike an instruction it actually enforces (D0128: there is no deterministic prevention for an LLM; the lever is moving gates earlier plus shaping affordances).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srGovSelfDetectingDiscipline : SystemRequirement {
        :>> id = "0d6e0003-0813-4e03-9b3e-0000001e0003";
        :>> title = "The engine detects when its own accepted discipline is being ignored";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "Where an accepted decision states a discipline whose inputs are recorded facts, the engine shall COMPUTE whether that discipline is being followed and surface the disagreement — rather than relying on the actor to comply and on a human to notice otherwise.";
        :>> rationale = "The defining finding of sprints 247-248: three of four process misses were BYPASSED EXISTING decisions (D0052 priority order, D0047 corrections-become-guards, D0018 no-prose-shadow-truth), not absent ones. Where both inputs are recorded — order and severity, work-done and requirement-verified, a retro's findings and the commit's tracked items, an acceptance and its attestation text — the bypass is computable, and computing it is the only thing that made it visible.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srGovTieredAffordances : SystemRequirement {
        :>> id = "0d6e0004-0813-4e04-9b3e-0000001e0004";
        :>> title = "In-loop gating escalates by tier, on measured evidence";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "Beyond the turn-boundary gate, the engine shall add per-edit (post-tool) and pre-action (deny) gating only as warranted by MEASURED evidence of the violations they would prevent, blocking clear violations and warning on the rest so governance does not thrash.";
        :>> rationale = "D0128 makes Tier-1/Tier-2 conditional on measurement precisely to avoid over-governance: a gate that fires on ambiguous cases trains the actor to route around it, which is the issue076/issue081 dynamic that cost eight bypassed commits. Deliberately UNDELIVERED until the turn-boundary gate has produced evidence.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }

    // ── nProcessPortability ───────────────────────────────────────────
    requirement srPortModularProcessUnit : SystemRequirement {
        :>> id = "0d6e0005-0813-4e05-9b3e-0000001e0005";
        :>> title = "A process is a self-contained, portable unit";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "A process shall be a self-contained unit carrying its definition, its deploying skill, its declared rules/guards, and its metadata — so it can be moved between projects whole, without carrying the rest of the engine and without leaving its enforcement behind.";
        :>> rationale = "The engine currently ships its process set monolithically: all-or-nothing, no selection. A downstream project wanting agile ceremony but not STPA has no way to take one without the other. Bundling the DECLARED rules with the definition is what makes portability real rather than cosmetic — a process whose enforcement stays behind is inert on arrival (D0059).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srPortActivationManifest : SystemRequirement {
        :>> id = "0d6e0006-0813-4e06-9b3e-0000001e0006";
        :>> title = "A per-project activation manifest decides which processes are enforced";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "Each project shall declare which processes are ACTIVE, and guards/orient shall enforce and report only the active set. Activating or disabling a process shall change what is enforced, not merely which skills exist.";
        :>> rationale = "Without this, importing a process either does nothing (a skill nobody invokes) or silently imposes gates the project never chose. It leans on the D0105/D0107 declarative-rules migration: only because controls are becoming DECLARED rules can activation toggle enforcement generically instead of requiring bespoke Rust per process.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srPortCatalogExchange : SystemRequirement {
        :>> id = "0d6e0007-0813-4e07-9b3e-0000001e0007";
        :>> title = "Processes can be listed, imported and exported across projects";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "The engine shall provide commands to list, search, import, export, activate and disable processes, backed by a curated palette plus bring-your-own sources, so a project can adopt a good process from elsewhere as a first-class operation.";
        :>> rationale = "This is the user-facing payoff of the two requirements above and the reason D0128 raised the Need: 'projects leverage GOOD processes a la carte across projects'. Sequenced last of the three because a catalog over non-portable, non-activatable units would move files around without moving discipline.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }

    // ── nReleaseDiscipline ────────────────────────────────────────────
    requirement srRelSemverReconciled : SystemRequirement {
        :>> id = "0d6e0008-0813-4e08-9b3e-0000001e0008";
        :>> title = "One reconciled version across crate, API contract and embedded engine";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "The crate version, the API/write contract version and the embedded engine version shall be reconciled to a single declared semver, and a contributor's toolchain shall be checkable against the version the project declares.";
        :>> rationale = "Guard semantics change with the engine, so contributors on different versions can legitimately disagree about whether the SAME model is valid — one lands what another's gate would block, and it presents as a mysterious CI failure rather than version skew. This is the dependency srDcEngineVersionParity (D0129) names: without published, pinnable versions there is nothing for a distributed team to pin TO.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srRelPublishedArtifact : SystemRequirement {
        :>> id = "0d6e0009-0813-4e09-9b3e-0000001e0009";
        :>> title = "A tagged version produces a CI-built, published artifact";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "Tagging a version shall cause CI to build and publish the release artifact (the binary plus the engine that `keel init` ships), so obtaining a specific engine version is a download rather than a local build.";
        :>> rationale = "A remote contributor cannot be asked to build from source to get a gate that matches everyone else's; the portable-gate work (issue076) is undermined if the binary is only obtainable by compiling. The engine currently has ONE stale GitHub release and a Deploy workflow that is defined but has never run — an honest dogfooding gap, since the engine governs Deploy for others while never running it itself.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srRelRecordedRelease : SystemRequirement {
        :>> id = "0d6e000a-0813-4e0a-9b3e-0000001e000a";
        :>> title = "Every release is a recorded item, not just a tag";
        :>> createdAt = "2026-08-13"; :>> createdBy = "claudeOpus5";
        :>> statement = "Each release shall be recorded as a `Release` item carrying its version, commit and membership rule, so what shipped is an authored fact and the release contents are a COMPUTED view over it rather than a hand-written note.";
        :>> rationale = "The schema already declares `Release` as a declaration plus a membership rule, with the baseline and release notes computed from it (§2.1). A git tag alone is not that: it cannot be traced to, queried, or asked as-of. Recording the release is what lets 'what was in v0.2.0' be computed instead of remembered.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::inspect;
    }

    // ── satisfy edges: Need <- SR ─────────────────────────────────────
    satisfy nAiGovernanceInLoop by srGovContractInSystemPrompt;
    satisfy nAiGovernanceInLoop by srGovTurnBoundaryGate;
    satisfy nAiGovernanceInLoop by srGovSelfDetectingDiscipline;
    satisfy nAiGovernanceInLoop by srGovTieredAffordances;
    satisfy nProcessPortability by srPortModularProcessUnit;
    satisfy nProcessPortability by srPortActivationManifest;
    satisfy nProcessPortability by srPortCatalogExchange;
    satisfy nReleaseDiscipline by srRelSemverReconciled;
    satisfy nReleaseDiscipline by srRelPublishedArtifact;
    satisfy nReleaseDiscipline by srRelRecordedRelease;

    // ── verify edges: what is ALREADY delivered, traced to the DoD that verified it ──
    // Only where the DoD's recorded evidence genuinely discharges the requirement (issue082).
    #Verify dependency from dcResponseContractGateDoD to srGovContractInSystemPrompt;
    #Verify dependency from dcResponseContractGateDoD to srGovTurnBoundaryGate;
    #Verify dependency from dcVerificationTraceDoD to srGovSelfDetectingDiscipline;
    #Verify dependency from dcPriorityInversionDoD to srGovSelfDetectingDiscipline;
    #Verify dependency from dcRetroToBacklogDoD to srGovSelfDetectingDiscipline;
    #Verify dependency from dcAttestationSubstanceDoD to srGovSelfDetectingDiscipline;

    // ── the governing Decisions derive from the Need they serve (cf. d0094/issue046) ──
    #DerivedFrom dependency from d0130 to nAiGovernanceInLoop;
    #DerivedFrom dependency from d0131 to nAiGovernanceInLoop;
    #DerivedFrom dependency from d0132 to nAiGovernanceInLoop;
}
