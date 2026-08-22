// =====================================================================
// ProductionReadinessRequirements — the panel's K1-K15 kernel invariants as SystemRequirements
// (docs/proposals/2026-08-21-production-readiness-design.md section 3: "what must be true, however
// implemented... the plan adopts them as its requirement skeleton. None is orphaned.").
// Statements are the proposal's, near-verbatim and implementation-free. satisfy edges root each in
// its Need (production-readiness.sysml); #Verify edges declare which chartered backlog item's DoD
// discharges each invariant (the section-3 WP map, made typed); #CharteredBy edges make the
// backlog items' "Chartered by dNNNN" prose typed.
// =====================================================================
package ProductionReadinessRequirements {
    private import EngineElement::*;
    private import EngineRequirements::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    requirement srK01ValidatedChannel : SystemRequirement {
        :>> id = "15b500f7-f7cd-4f18-82b7-f509d7c0f2d8";
        :>> title = "K1: no unvalidated state transition survives, overrides are recorded and surfaced";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "The system shall permit no tracked-model state transition except through a validated channel or a recorded, surfaced override - and no such transition shall survive gating unverified.";
        :>> rationale = "Kernel invariant K1: the override residue is K7's audit trail; layer 3 (verification over the tree) is the backstop.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK02FailLoud : SystemRequirement {
        :>> id = "445cb66f-fc55-4f5e-9540-63bd6567eb27";
        :>> title = "K2: every enforcement point fails loud and is inventoried";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Absent, erroring, or timed-out enforcers shall never silently pass; the full enforcement-point inventory with each point's absent/error/timeout behavior shall be enumerated and reported, with timeout-resolves-to-allow points recorded as residuals.";
        :>> rationale = "Kernel invariant K2: a silently open enforcement point is indistinguishable from a working one until the defect it missed lands.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK03EnforcementParity : SystemRequirement {
        :>> id = "37413066-7e2f-4889-8ebf-5ba05fc8e809";
        :>> title = "K3: downstream projects and spawned runs get the self-build's enforcement, proven";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Downstream projects and spawned runs shall receive the self-build's enforcement, and inheritance shall be proven by behavioral acceptance plus fire-ledger evidence, never assumed from file presence.";
        :>> rationale = "Kernel invariant K3: presence-checking a hook file proves nothing about whether it fires - the class of assumption issue183 recorded.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK04DeclaredBoundary : SystemRequirement {
        :>> id = "d887811b-74a2-4288-895c-859b4aff9d29";
        :>> title = "K4: enforced work enters as a declared, human-approved process instance";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Enforced work shall enter as a declared, human-approved process instance; undeclared actions shall not be offered at the launch boundary. Residual, stated: interactive sessions remain for CHANGE and design work.";
        :>> rationale = "Kernel invariant K4: the boundary layer shapes what starts; view::is_launchable already enforces the declared set.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::demo;
    }
    requirement srK05ComputedCompletion : SystemRequirement {
        :>> id = "418bce72-2e12-4f61-a961-732eaac6af65";
        :>> title = "K5: completion is computed from evidence, never recorded while obligations are unverified";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Work shall not be recorded complete while its declared obligations are unverified; completion shall be computed from evidence. Form-only - substance verification stays human (D0094/D0096).";
        :>> rationale = "Kernel invariant K5: the hollow-pass history (D0050, issue092, issue099) is exactly recorded-complete-without-evidence.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK06HumanChannel : SystemRequirement {
        :>> id = "ea0c8702-e339-470c-b229-ddd6d87a0cef";
        :>> title = "K6: human-judgment records originate only from channels the agent cannot invoke";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Records of human judgment (acceptance, confirmation) shall originate only from a channel the agent cannot invoke: the write layer refuses AI-kind actors, the harness layer treats human-judgment subcommands and actor-identity mutation as protected and never exempt, and acceptance channels are human-held.";
        :>> rationale = "Kernel invariant K6: the actor binding is agent-mutable state, so the check cannot rest on it alone - the seam issue197 demonstrated from the other direction.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK07RecordedWeakening : SystemRequirement {
        :>> id = "bac7d387-cdd1-42ed-9661-8daa6a0f62bd";
        :>> title = "K7: weakening a control is human-approved or leaves a durable, surfaced record - always visible";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Weakening a control (deactivation, exemption, hook or config edit, hooksPath, actor rebinding) shall be either human-approved at the moment of action or shall leave a durable, surfaced record - and always both a record and visibility. Override unlocks shall be single-use, target-path-bound, and session-bound, and the recorded fact shall name the path actually written.";
        :>> rationale = "Kernel invariant K7: the one recorded self-inflicted bypass (issue116, hooksPath=/dev/null) is the vector class.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK08UnitHandshake : SystemRequirement {
        :>> id = "6dd4ec6d-112b-4eac-b121-570471525b0c";
        :>> title = "K8: a process definition and its enforcement are one distributable unit, verified at import";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "A process definition and its enforcement shall be one distributable unit; import shall verify the consumer can enforce it and shall refuse or loudly degrade with a recorded Issue when it cannot.";
        :>> rationale = "Kernel invariant K8: prose that lands without its teeth is the modularity failure the proposal's section-1.3 names.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK09Supersession : SystemRequirement {
        :>> id = "6bb0036c-24c5-4b2e-b09f-64996af89153";
        :>> title = "K9: process definitions update by supersession without identity or history loss";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "A process definition shall be updateable by supersession, without identity or history loss, through the same governed channel as any other process change.";
        :>> rationale = "Kernel invariant K9: install-once-no-update is the current state (import refuses collisions).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK10Reprocess : SystemRequirement {
        :>> id = "2c43175d-35db-42c2-9b13-46675b27a2ec";
        :>> title = "K10: a definition change propagates re-evaluation obligations to prior work";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "A definition change shall propagate re-evaluation obligations to work done under the prior definition, for any process or workflow definition, resolved per item from its charter edges.";
        :>> rationale = "Kernel invariant K10: governing-version exists but is hardcoded to one path; the obligation must generalize.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK11DeclaredControls : SystemRequirement {
        :>> id = "3f1d6c21-f632-4ce9-8042-1ff797ae3012";
        :>> title = "K11: controls are declared data, adoptable and extensible without forking the engine";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Controls shall be declared data: a consumer adopts, extends, or retires them without modifying the engine, and adoption changes are governed.";
        :>> rationale = "Kernel invariant K11: a blocking control requiring a Rust fork means downstream projects have no controls of their own.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK12VerdictTraceability : SystemRequirement {
        :>> id = "3b96a79d-5ea9-4889-b985-edf51ddb5b79";
        :>> title = "K12: every enforcement verdict traces to the definition version and tree it judged";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every enforcement verdict shall be traceable to the definition version and the tree it was judged against; run records shall carry the run-start fingerprint and judgedAgainst.";
        :>> rationale = "Kernel invariant K12: existing doctrine (judgedAgainst) extended to run granularity.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK13DenyAndProvide : SystemRequirement {
        :>> id = "3b0a906b-5d05-4cce-9bfd-133bf89badcf";
        :>> title = "K13: every refusal names an available sanctioned path";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every refusal shall name an available sanctioned path (deny-and-provide) - or agents route around it. The CLI is the current typed binding; MCP is a recorded future binding of the same need (D0186 direction).";
        :>> rationale = "Kernel invariant K13: a refusal with no path trains the Bash bypass, the issue081 lesson.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srK14MeasuredEffect : SystemRequirement {
        :>> id = "6058ccef-2c17-4c4d-8e2c-76a875aaec70";
        :>> title = "K14: enforcement effect is measured; promotions cite the evidence";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Fires, blocks, overrides, and adherence defects shall be counted, and promotion of any advisory control to blocking shall cite that evidence over at least the N-sprint window fixed in D0180.";
        :>> rationale = "Kernel invariant K14: D0128's own 'prove the in-loop gate' step, recorded as undelivered by D0130 - delivered here.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::analyze;
    }
    requirement srK15TreeDerivable : SystemRequirement {
        :>> id = "11f7545d-11b1-4288-9e0b-90639f00874b";
        :>> title = "K15: every enforcement verdict is re-derivable from committed truth alone";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every enforcement verdict shall be re-derivable from committed truth alone, independent of any hook having run - validate and guards over the tree plus audit-history over the pushed range.";
        :>> rationale = "Kernel invariant K15: section 2's layer-3 guarantee stated as an invariant so no future cut can orphan its carrier.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }

    // ── satisfy: each invariant roots in its Need ─────────────────────────────────────
    satisfy nPavedWritePath by srK01ValidatedChannel;
    satisfy nMeasuredEnforcement by srK02FailLoud;
    satisfy nEnforcementParity by srK03EnforcementParity;
    satisfy nBoundedLaunch by srK04DeclaredBoundary;
    satisfy nBoundedLaunch by srK05ComputedCompletion;
    satisfy nPavedWritePath by srK06HumanChannel;
    satisfy nPavedWritePath by srK07RecordedWeakening;
    satisfy nPortableProcessUnits by srK08UnitHandshake;
    satisfy nPortableProcessUnits by srK09Supersession;
    satisfy nPortableProcessUnits by srK10Reprocess;
    satisfy nPortableProcessUnits by srK11DeclaredControls;
    satisfy nBoundedLaunch by srK12VerdictTraceability;
    satisfy nPavedWritePath by srK13DenyAndProvide;
    satisfy nMeasuredEnforcement by srK14MeasuredEffect;
    satisfy nPavedWritePath by srK15TreeDerivable;

    // ── verify: the section-3 WP map, typed - which chartered DoD discharges which invariant ──
    #Verify dependency from p1TieredDenyDoD to srK01ValidatedChannel;
    #Verify dependency from p5PostRunGateDoD to srK01ValidatedChannel;
    #Verify dependency from p0FailLoudDoD to srK02FailLoud;
    #Verify dependency from p0HardeningInventoryDoD to srK02FailLoud;
    #Verify dependency from p0ClaudeScaffoldDoD to srK03EnforcementParity;
    #Verify dependency from p0SyncClaudeDoD to srK03EnforcementParity;
    #Verify dependency from p0ProfilesDoD to srK03EnforcementParity;
    #Verify dependency from p0InstallDocsDoD to srK03EnforcementParity;
    #Verify dependency from p0SubagentStopDoD to srK03EnforcementParity;
    #Verify dependency from p5SpawnHardeningDoD to srK03EnforcementParity;
    #Verify dependency from p5FormsDoD to srK04DeclaredBoundary;
    #Verify dependency from p5PostRunGateDoD to srK05ComputedCompletion;
    #Verify dependency from p5CompletionCheckDoD to srK05ComputedCompletion;
    #Verify dependency from p1HumanChannelDoD to srK06HumanChannel;
    #Verify dependency from p1BashMatcherDoD to srK06HumanChannel;
    #Verify dependency from p1ControlPlaneDoD to srK07RecordedWeakening;
    #Verify dependency from p1TieredDenyDoD to srK07RecordedWeakening;
    #Verify dependency from p1RedYieldDoD to srK07RecordedWeakening;
    #Verify dependency from p4aHandshakeDoD to srK08UnitHandshake;
    #Verify dependency from p4bRulesTravelDoD to srK08UnitHandshake;
    #Verify dependency from p4aUnitIdentityDoD to srK09Supersession;
    #Verify dependency from p4aInstallRecordDoD to srK09Supersession;
    #Verify dependency from p4aImportUpdateDoD to srK09Supersession;
    #Verify dependency from p4aGoverningVersionDoD to srK10Reprocess;
    #Verify dependency from p1RulesWiredDoD to srK11DeclaredControls;
    #Verify dependency from p3aRuleAttributionDoD to srK11DeclaredControls;
    #Verify dependency from p3aKeystoneExtensionDoD to srK11DeclaredControls;
    #Verify dependency from p5RunRecordsDoD to srK12VerdictTraceability;
    #Verify dependency from p1TieredDenyDoD to srK13DenyAndProvide;
    #Verify dependency from pmMeasureDoD to srK14MeasuredEffect;
    #Verify dependency from p1AuditHistoryCiDoD to srK15TreeDerivable;

    // ── charter: the backlog items' "Chartered by dNNNN" prose, typed ─────────────────
    #CharteredBy dependency from p0ClaudeScaffold to d0174;
    #CharteredBy dependency from p0SyncClaude to d0174;
    #CharteredBy dependency from p0FailLoud to d0174;
    #CharteredBy dependency from p0Profiles to d0174;
    #CharteredBy dependency from p0SubagentStop to d0174;
    #CharteredBy dependency from p0HardeningInventory to d0174;
    #CharteredBy dependency from p0InstallDocs to d0175;
    #CharteredBy dependency from p1TieredDeny to d0176;
    #CharteredBy dependency from p1BashMatcher to d0176;
    #CharteredBy dependency from p1RedYield to d0176;
    #CharteredBy dependency from p1RulesWired to d0177;
    #CharteredBy dependency from p1HumanChannel to d0178;
    #CharteredBy dependency from p1ControlPlane to d0179;
    #CharteredBy dependency from p1AuditHistoryCi to d0179;
    #CharteredBy dependency from pmMeasure to d0180;
    #CharteredBy dependency from p5Forms to d0181;
    #CharteredBy dependency from p5RunRecords to d0182;
    #CharteredBy dependency from p5PostRunGate to d0182;
    #CharteredBy dependency from p5CompletionCheck to d0182;
    #CharteredBy dependency from p5SpawnHardening to d0182;
    #CharteredBy dependency from p4aUnitIdentity to d0183;
    #CharteredBy dependency from p4aInstallRecord to d0183;
    #CharteredBy dependency from p4aImportUpdate to d0183;
    #CharteredBy dependency from p4aHandshake to d0183;
    #CharteredBy dependency from p4aGoverningVersion to d0183;
    #CharteredBy dependency from p3aRuleAttribution to d0184;
    #CharteredBy dependency from p3aKeystoneExtension to d0184;
    #CharteredBy dependency from p4bRulesTravel to d0183;
    // D0195 clause 8 (panel R2, aerospace finding 1): EHZ9's controls traced to no standing
    // requirement - removing the release gate would have violated nothing recorded. Added by
    // claudeFable5 (a non-owner ADDS items, D0108).
    requirement srK16OutwardPublishGated : SystemRequirement {
        :>> id = "69aeb905-e9c3-44ca-81dc-2dfe9f6daa77";
        :>> title = "K16: no outward publish leaves the boundary ungated";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "No artifact shall leave the system boundary (a release publish, a tag-triggered build, a landing on the shared trunk) without the full enforced gate having passed over the exact tree being shipped; a gate failure shall build nothing and publish nothing, visibly.";
        :>> rationale = "Elevates the sprint-409 control layer (the release gate job the publish requires, the pre-push behind-check, land's gate-on-merged-tree) to a standing requirement, so the controls trace to something whose removal would be a recorded violation - the panel's finding was precisely that they did not. EHZ9 is the constrained hazard; issue211 is the incident lineage (the pipeline published with zero gate steps until sprint 409).";
        :>> kind = RequirementKind::functional;
    }

    // srK16's Need root (upstream fleshing, sprint 418): the release-discipline Need owns the outward boundary.
    satisfy nReleaseDiscipline by srK16OutwardPublishGated;


    // D0192 OPTION A (accepted 2026-08-22): srK06's letter and the sanctioned delegation practice
    // contradicted (issue204); the human chose to amend the requirement. srK06HumanChannel is
    // SUPERSEDED, not edited - it is claudeOpus5's item (D0108) and its history stays readable.
    requirement srK06aDelegatedRecording : SystemRequirement {
        :>> id = "3c5fe2bb-6f7b-44b9-8628-28ac0328aa0e";
        :>> title = "K6a: human-judgment records come from a human-held channel or a declared, quote-carrying delegation";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Records of human judgment (acceptance, confirmation) shall originate either from a channel the agent cannot invoke, or from an agent-recorded delegation that quotes the human's words verbatim, names them as judge, and cites the delegation decision; every recording delegation shall be declared per attestation class in the attestation policy, and a delegated record that carries no verbatim quote is a violation.";
        :>> rationale = "D0192 OPTION A: the human works by stating verdicts in conversation, and forcing every acceptance through a browser gesture taxes the workflow they chose (D0054). The protection this trades away is stated plainly in D0192: an agent that fabricates a quote defeats the check - the remaining protections are the human reading their own queue and the audit trail. The write layer still refuses AI-KIND judges; this clause governs the recording channel, not the judge.";
        :>> kind = RequirementKind::functional;
    }
    #Supersede dependency from srK06aDelegatedRecording to srK06HumanChannel;
    satisfy nPavedWritePath by srK06aDelegatedRecording;

}
