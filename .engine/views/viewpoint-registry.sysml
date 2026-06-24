// =====================================================================
// EngineViewpoints — the formal VIEWPOINT registry (D0056/D0057; ISO 42010 / Arcadia
// / SysML v2). Each Viewpoint is a DECLARED lens: concern + audience + sources +
// renderer. Views are the computed application of a viewpoint (#View, regenerable) —
// the rendering stays in tooling (the Rust `keel` subcommands + TOML views, D0048/D0074),
// DRIVEN BY these declarations. Adding a view = declaring a Viewpoint here.
// Each renderer MUST name a real current command (no retired query.py/report.py refs) or be
// explicitly "(planned ...)" — the viewpointRendererGuard (issue034/D0056) enforces this.
// =====================================================================
package EngineViewpoints {
    private import EngineElement::*;     // Element identity
    private import EngineComputed::*;    // Viewpoint type

    part orientVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-1619-405162738410";
        :>> title = "orient";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Where do things stand and what is ready to work next?";
        :>> audience = "maintainer / lead";
        :>> sources = "backlog action defs + latest TestResults + succession DAG + delivery-file ceremony gates";
        :>> renderer = "keel orient [--html] (JSON authority; --html = the human dashboard #View, D0093)";
    }
    part whatsNextVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-2619-405162738411";
        :>> title = "whats-next";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which outstanding items are unblocked, in priority order?";
        :>> audience = "maintainer / AI executor";
        :>> sources = "ready frontier (outstanding tasks with all deps done), ranked by backlog declaration order (D0052)";
        :>> renderer = "keel whats-next";
    }
    part traceabilityVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-3619-405162738412";
        :>> title = "traceability";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Are needs->requirements->verification linked, and where are the gaps / subversions?";
        :>> audience = "reviewer / auditor";
        :>> sources = "satisfy + verify edges; sprint ceremony gate coverage; issue->decision links; process-evolution timeline";
        :>> renderer = "keel diagram";
    }
    part diagramVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-c085-405162738485";
        :>> title = "diagram";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "How does the WHOLE model connect — every element, typed edge, metadata, reliance, supersession, effects — as one navigable graph explored a part at a time?";
        :>> audience = "maintainer / reviewer / auditor";
        :>> sources = "all elements + all typed edges + authored metadata (the full Model)";
        :>> renderer = "keel diagram (interactive HTML #View, D0085; regenerate-don't-commit)";
    }
    part renderVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-c086-405162738486";
        :>> title = "render";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "How do I explore, tabulate, or REVIEW any declared view as an interactive artifact — and disposition elements (accept/finding + rationale) so the judgment lands back in the model as linked critiques that attest state and induce suspicion?";
        :>> audience = "maintainer / reviewer / auditor";
        :>> sources = "any declared view (D0056/D0057) over the Model — graph (elements + typed edges), table/review (selected rows + projected fields); review dispositions round-trip via apply-review into .tracking/critiques.sysml";
        :>> renderer = "keel render <view> --mode graph|table|review (interactive HTML #View, D0086; regenerate-don't-commit) + keel apply-review";
    }
    part reportVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-c087-405162738487";
        :>> title = "report";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "How are we doing, and where can we improve — aggregate HEALTH (verification coverage %, critique %, attestation %, findings, traceability completeness, readiness) and OPPORTUNITY (charter debt, requirements volatility, suspect+stale, flow/velocity) metrics rolled up from the per-element views, with git-derived trends?";
        :>> audience = "maintainer / reviewer / program lead";
        :>> sources = "aggregation of the per-element computed views (coverage 3-tier, critique-coverage, attestation, assured/readiness, decisions, orphans, open-issues, suspect) + delivery Stories (velocity) + git history (trends); grounded in the INCOSE DE Measurement Framework + SE Leading Indicators";
        :>> renderer = "keel report <assurance|traceability|quality-debt|flow|governance> [--html] [--trend] (computed scorecard #View, D0087; regenerate-don't-commit)";
    }
    part indicatorsVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-c089-405162738489";
        :>> title = "indicators";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which monitored measures (Indicators) inform our needs/decisions, and which way are they trending — improving, degrading, or flat — by their goal (D0089)?";
        :>> audience = "maintainer / lead / program";
        :>> sources = "Indicator items + their measurement method (computed series via the report/trend engine; pulled/manual series from recorded Measurement datapoints, #Measures edges); direction-aware status by goal (D0088 measure spectrum)";
        :>> renderer = "keel indicators [--trend] (+ keel record-measurement for pulled/manual datapoints, D0089)";
    }
    part safetyVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-4619-405162738413";
        :>> title = "safety";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Are safety-relevant losses/hazards/UCAs identified and constrained, and is each constraint verified?";
        :>> audience = "safety engineer";
        :>> sources = "STPA losses/hazards/UCAs/constraints (EngineSafety); safety-kind requirements; verify edges to their Tests";
        :>> renderer = "(planned) STPA safety view (losses/hazards/UCAs/constraints + verify coverage) — not yet rendered";
    }
    part requirementsVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-5619-405162738414";
        :>> title = "requirements";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "What does the system need to do, are requirements well-formed, and is each satisfied + verified?";
        :>> audience = "systems engineer / requirements owner";
        :>> sources = "Need / Requirement items; satisfy edges (to components) + verify edges (to Tests); requirement-quality (INCOSE/EARS)";
        :>> renderer = "(planned) dedicated requirements view — partially served today by keel coverage + keel critique-coverage";
    }
    part decisionsVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-6619-405162738415";
        :>> title = "decisions";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "What was decided, by whom, when, and why (incl. alternatives)?";
        :>> audience = "lead / auditor / future maintainer";
        :>> sources = ".engine/decisions/*.sysml Decision items (context/decisionText/consequences; alternatives via decisionAnalysis/issue016)";
        :>> renderer = "keel decisions (+ the decisions TOML view: keel view decisions)";
    }
    part baselinesVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-7619-405162738416";
        :>> title = "baselines";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "What was the state of the model at a given release/commit?";
        :>> audience = "release manager / auditor";
        :>> sources = "Release (commit SHA + member refs); baseline view computed from git + authored facts";
        :>> renderer = "(planned) baselines view computed from git + authored facts — not yet rendered";
    }
    part suspectVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-8619-405162738417";
        :>> title = "suspect";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which done items have stale evidence (model OR deliverable-source drift) and need re-verification?";
        :>> audience = "maintainer / QA";
        :>> sources = "git-ancestry of judgedAgainst vs upstream change over satisfy/:> edges; deliverable-manifest source drift (D0050); unresolved failing critiques (D0086)";
        :>> renderer = "keel suspect";
    }
    part orphansVP : Viewpoint {
        :>> id = "61728394-0a5b-4c6d-9619-405162738418";
        :>> title = "orphans";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which model elements are orphaned / dangling — a Need with no satisfying Requirement, a Requirement with no satisfy or verify edge, a task with no DoD, an Issue with no relatedTask, a Decision referenced by nothing — i.e. broken or missing required traceability?";
        :>> audience = "maintainer / auditor / reviewer";
        :>> sources = "all items + typed edges; flag any element missing a REQUIRED incoming/outgoing edge for its kind (Need->Requirement satisfy; Requirement->Test verify; task->DoD; Issue->relatedTask)";
        :>> renderer = "keel orphans";
    }

    part attestationCoverageVP : Viewpoint {
        :>> id = "67940516-2c7d-4e8f-a067-516273849620";
        :>> title = "attestation-coverage";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which items lack an attestation their governing process requires? Currently: every status=accepted Decision must carry a passing acceptance event (dNNNNAccept confirmation TestResult, D0066).";
        :>> audience = "maintainer / auditor / reviewer";
        :>> sources = "all decisions; flag any status=accepted Decision with no passing dNNNNAcceptR1 TestResult (extensible to other process-required attestations)";
        :>> renderer = "keel attestation-coverage";
    }

    part governingVersionVP : Viewpoint {
        :>> id = "63940516-2c7d-4e8f-a070-516273849630";
        :>> title = "governing-version";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which version of its governing process did a work item follow — i.e. what process was in force AS-OF the item's charter (charter-time freeze, D0068), so an output can be accepted against the process it actually satisfied (D0062) and audited / regenerated?";
        :>> audience = "maintainer / auditor / reviewer";
        :>> sources = "git ancestry (D0013): the process-def state at the latest change-commit that is an ancestor of the item's introduction commit; correlated with process-change Decisions (#ProspectiveChange/#SafetyChange markers, D0070) in force at vs. after the charter; work->process by convention (D0069)";
        :>> renderer = "keel governing-version <item>";
    }

    part reprocessVP : Viewpoint {
        :>> id = "63940516-2c7d-4e8f-a070-516273849631";
        :>> title = "reprocess-candidates";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which items were chartered under a process version later superseded by a SAFETY change, and so must be re-processed against the new process (mandatory iff retroactivity=safety; prospective changes never flag — D0062)?";
        :>> audience = "maintainer / safety reviewer";
        :>> sources = "governing-version per item x process-change Decisions whose retroactivity=safety and whose effective commit post-dates the item's charter (git ancestry, D0070)";
        :>> renderer = "keel reprocess-candidates";
    }

    part rootednessVP : Viewpoint {
        :>> id = "63940516-2c7d-4e8f-a099-516273849699";
        :>> title = "rootedness";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Is every chartered capability rooted in a driving Need — and what is the charter-source balance (need-rooted vs legitimate decision-driven evolution vs orphan)? (D0098/D0099/issue047)";
        :>> audience = "maintainer / reviewer / auditor";
        :>> sources = "delivery Story #CharteredBy chains -> Need (direct / via satisfy'd SystemRequirement / via a Decision #DerivedFrom a Need); the #Capability marker + its #DerivedFrom->Need link (the hard-gate set)";
        :>> renderer = "keel rootedness (+ the requirement-rootedness guard for the #Capability hard gate)";
    }
    part tierSatisfactionVP : Viewpoint {
        :>> id = "63940516-2c7d-4e8f-a098-516273849698";
        :>> title = "tier-satisfaction";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Is each tier cleanly + comprehensively satisfied by its downstream items — are Needs decomposed into SystemRequirements and SystemRequirements verified by Tests? Thin downstream satisfaction is a leading indicator of insufficient implementation (D0098/issue047).";
        :>> audience = "maintainer / reviewer / systems engineer";
        :>> sources = "satisfy edges (Need->SystemRequirement) + verify edges (Test->SystemRequirement); per-tier structural-satisfaction fraction + the gap set";
        :>> renderer = "keel tier-satisfaction (+ the orient burndown block)";
    }
    part critiquePolicyVP : Viewpoint {
        :>> id = "63940516-2c7d-4e8f-a097-516273849697";
        :>> title = "critique-policy";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which antagonistic critique lenses are REQUIRED per assurance-element type, and is that policy the built-in default or a project override (D0097)?";
        :>> audience = "maintainer / reviewer / auditor";
        :>> sources = "the declared .engine/contracts/critique-policy.toml ([lenses] type -> required CritiqueLens list); the built-in Core-3 default when the file is absent; the CritiqueLens canon (schema/core)";
        :>> renderer = "keel critique-policy";
    }
    part concernCoverageVP : Viewpoint {
        :>> id = "63940516-2c7d-4e8f-a071-516273849632";
        :>> title = "concern-coverage";
        :>> createdBy = "wweatherholtz";
        :>> concernText = "Which declared stakeholder concerns (Viewpoints) are SERVED by a working computed renderer, and which are still unserved (renderer planned/unbuilt) — i.e. the registry's own coverage of the concerns it declares (D0057/issue035)?";
        :>> audience = "maintainer / reviewer / auditor";
        :>> sources = "this viewpoint registry: each Viewpoint's renderer classified served (names a keel command) vs unserved (planned), with a coverage %";
        :>> renderer = "keel concern-coverage";
    }
}
