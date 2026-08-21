// =====================================================================
// ProductionReadinessBusiness — the Business layer for the production-readiness program
// (docs/proposals/2026-08-21-production-readiness-design.md, r4 FINAL, panel-unanimous 9/0).
// Five Needs from the proposal's section-1 goal, five UseCases from its testable
// section-7 criteria. The SystemRequirements are the panel's K1-K15 kernel invariants
// (production-readiness-requirements.sysml) - the proposal's own words: "the plan adopts
// them as its requirement skeleton." Sourced from st017 (the human's directive to ingest)
// and chartered through D0174-D0186.
// =====================================================================
package ProductionReadinessBusiness {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    // ── Needs: the section-1 goal, one need per production-blocking property ──────────

    requirement nEnforcementParity : Need {
        :>> id = "b65fa7f8-d62a-49af-aa37-4dbeab4f14ad";
        :>> title = "A downstream keel init project gets the self-build's enforcement";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::customer;
        :>> statement = "A fresh keel init project, on Windows and Linux with no manual steps, receives the same in-loop enforcement the self-build has - hooks, output style, permission rules, protected-path deny list, discoverable skills, a loud-fail commit gate - installable from the existing release channel, and the inheritance is proven behaviorally, never assumed. Today the in-loop half does not ship: init scaffolds none of .claude/, skills land where no harness discovers them, and the scaffolded pre-commit passes silently without the binary.";
        :>> priority = Priority::must;
    }

    requirement nPavedWritePath : Need {
        :>> id = "2ec436d6-fee9-4ef7-92f7-d88e5550f5a6";
        :>> title = "Model writes enter through validated channels; weakening a control leaves a record";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::customer;
        :>> statement = "No tracked-model state transition except through a validated channel or a recorded, surfaced override; records of human judgment originate only from channels the agent cannot invoke; control-plane writes (hooks config, hooksPath, actor identity, deactivation) are approval-gated and recorded; every refusal names an available sanctioned path; and every enforcement verdict is re-derivable from committed truth alone, hook-independent - the layer-3 guarantee the record shows working unconditionally.";
        :>> priority = Priority::must;
    }

    requirement nMeasuredEnforcement : Need {
        :>> id = "8e193210-5396-41eb-ac49-f66b12cf93fc";
        :>> title = "The effect of enforcement is measured, not presumed";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "Enforcement fires, blocks, overrides, and adherence defects are counted; every enforcement point is inventoried with its absent/error/timeout behavior so nothing silently passes; and promotion of any advisory control to blocking cites the measured evidence. This is D0128's recorded-but-undelivered 'prove the in-loop gate' step, and its absence is why over-strict gates historically trained their actor to disable them (issue076/081).";
        :>> priority = Priority::must;
    }

    requirement nBoundedLaunch : Need {
        :>> id = "d398867d-0344-4b66-8e00-15adc54cb652";
        :>> title = "Routine enforced work starts as a declared, approved process instance in bounded context";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::customer;
        :>> statement = "Routine EXECUTE and RECORD work enters through the console launch boundary: a declared, human-approved process instance in fresh bounded context, whose run is recorded, gated on exit against the run's own diff and stamped actor, never recorded complete while its declared obligations are unverified, and whose diff reaches human review unconditionally. Interactive sessions remain for CHANGE and design work; the ratio shift is a measured hypothesis, not a claim.";
        :>> priority = Priority::should;
    }

    requirement nPortableProcessUnits : Need {
        :>> id = "7b29c4c1-eac8-4750-80dc-8041c607984b";
        :>> title = "A process definition and its enforcement travel, update, and extend as one unit";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::customer;
        :>> statement = "A process definition and its enforcement are one distributable unit with versioned identity: import verifies the consumer can enforce it and refuses or loudly degrades; updates land by supersession without identity or history loss; a definition change propagates re-evaluation obligations to work done under the prior definition; and controls are declared data a consumer adopts, extends, or retires without modifying the engine. Today the teeth are compiled while the prose is modular - install-once, no update path, units export with empty rules.";
        :>> priority = Priority::must;
    }

    // ── UseCases: the section-7 testable meanings of production-ready ─────────────────

    use case ucDownstreamAdopt : UseCase {
        :>> id = "6af0b223-c6c5-4569-8fa4-313af3a083a6";
        :>> title = "Adopt keel in a fresh or existing project";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> description = "A downstream team installs the released binary via the documented path and runs keel init with a DECLARED adoption profile (strict for provably empty directories; any directory with existing content requires the explicit flag - the issue089/129 lockout class).";
        :>> mainSuccess = "The scaffold starts green; blocking in-loop gates provably fire (fire-ledger evidence in a spawned session); skills are discoverable; a commit without the binary blocks loudly with the install path named; the profile is a recorded, printed, model-visible fact.";
        :>> extensions = "Existing-content directory without --profile: init refuses with the two profiles explained. Binary absent at hook time: pure-shell protected-path test still denies with a loud message; nothing passes silently.";
    }

    use case ucLaunchedRun : UseCase {
        :>> id = "ca417649-460c-4fec-add8-7f17e5726d65";
        :>> title = "Launch a routine process run from the console";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> description = "The human picks a declared launchable process, fills its model-generated input form, approves, and the launcher spawns a bounded agent run (fresh context, absolute KEEL_BIN, explicit permission mode, wall-clock timeout) on a clean tree with a snapshot fingerprint.";
        :>> mainSuccess = "On exit the post-run gate runs validate + guards + rules over the run-start-to-working-tree diff under the run's stamped actor; the run record lands locally plus one tracked summary for non-empty diffs, written after the gate under the run's actor; the diff routes to human review unconditionally, green or red.";
        :>> extensions = "Red gate: run recorded failed, diff still reviewed. Headless ask: console proxy when up (ask-pending entry written before blocking, wait bounded), else auto-deny plus a recorded obligation. Dirty tree at launch: refused; the console offers a commit action so accept-then-launch does not detour through freeform.";
    }

    use case ucProcessExchange : UseCase {
        :>> id = "ef7e0dd0-e48a-4a5f-ba6d-4988fe7cb853";
        :>> title = "Export a process unit from project A; import and later update it in project B";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> description = "Project A exports a versioned process unit carrying its declared rules; project B imports it - the handshake diffs required guards against B's binary inventory and refuses or loudly degrades with a recorded Issue; the install record (unit id, version, per-file hashes) becomes the three-way merge base.";
        :>> mainSuccess = "A later upstream revision lands by import --update supersession: B's local assert-constraint additions are never silently clobbered; the superseded definition's supersede edge carries the keystone marker so reprocess-candidates fires; activation.toml rewrites atomically; the imported process appears as a launchable action in B's console.";
        :>> extensions = "Three-way divergence: refuse with a report. Pre-P4a install: bootstrap by treating local as base with explicit human confirmation.";
    }

    use case ucControlOverride : UseCase {
        :>> id = "175a29fe-53a6-450a-8bd8-d95e22366642";
        :>> title = "Override a write refusal, leaving the record";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> description = "An agent hits a hard deny on an API-owned fact surface (or must repair a corrupted API-owned file the 9 operations cannot reach) and takes the sanctioned unlock: single-use, target-path-bound, session-bound, reason required.";
        :>> mainSuccess = "The write proceeds once; the auto-recorded obligation fact names the path actually written and is orient-visible; one-file-per-fact recording cannot deadlock on the file being repaired (local ledger fallback with a sync obligation).";
        :>> extensions = "Control-plane target (.claude/settings.json, hooksPath, actor identity): ask-tier plus recorded fact - never a quiet edit. Human-judgment subcommand under an agent session: never exempt, routed to ask.";
    }

    use case ucEnforcementPromotion : UseCase {
        :>> id = "fa26ad3b-db43-4a3f-af3a-2115b5b4748f";
        :>> title = "Promote an advisory control to blocking, citing measurements";
        :>> createdAt = "2026-08-21"; :>> createdBy = "claudeOpus5";
        :>> description = "After at least N sprints (N fixed in D0180, default 3) of fire-ledger evidence - fires, blocks, overrides, red-yields, adherence trend from ProcessDefect issues - the human reviews a promotion (or non-promotion) decision for an advisory control.";
        :>> mainSuccess = "The promotion decision cites the measurements; guided-profile projects promote with one command; the launcher-fraction hypothesis and the dirty-tree-refusal rate are reported alongside.";
        :>> extensions = "Evidence shows the control trains bypass behavior (the issue081 class): the control is revised or retired instead of promoted, and the non-promotion is recorded with the same evidence.";
    }

    // ── provenance: every need derives from the human's directive over the panel's proposal ──
    #DerivedFrom dependency from nEnforcementParity to st017;
    #DerivedFrom dependency from nPavedWritePath to st017;
    #DerivedFrom dependency from nMeasuredEnforcement to st017;
    #DerivedFrom dependency from nBoundedLaunch to st017;
    #DerivedFrom dependency from nPortableProcessUnits to st017;
    #DerivedFrom dependency from ucDownstreamAdopt to nEnforcementParity;
    #DerivedFrom dependency from ucLaunchedRun to nBoundedLaunch;
    #DerivedFrom dependency from ucProcessExchange to nPortableProcessUnits;
    #DerivedFrom dependency from ucControlOverride to nPavedWritePath;
    #DerivedFrom dependency from ucEnforcementPromotion to nMeasuredEnforcement;
}
